use std::collections::{BTreeMap, HashMap, HashSet};

use uuid::Uuid;
use widgets::flow_editor::PortType;

use crate::action_system::flow::{AppFlow, FlowTrigger};
use crate::action_system::graph::{ActionGraph, ActionNodeData, ActionNodeId};
use crate::action_system::node_kinds::{
    ActionNodeKind, ActionValue, AuthoredCondition, AuthoredValueSource, CompareOp, CompareRhs,
    NavigateTarget, ValueSource,
};
use crate::action_system::state_ref::{ActionValueType, StateFieldRef};
use crate::data_structures::types::types::{AppView, Widget, WidgetId, WidgetType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SemanticDiagnosticCode {
    MissingTriggerNode,
    MultipleTriggerNodes,
    DuplicateNodeId,
    DanglingEdgeNode,
    DanglingEdgePort,
    InvalidEdgeKinds,
    DuplicateEdge,
    MultipleIncomingEdge,
    SelfLoopUnsupported,
    UnsupportedNodeKind,
    InvalidNodeConfiguration,
    MissingRequiredInput,
    UnresolvedReference,
    UnknownViewReference,
    InvalidTriggerTopology,
    UnsupportedExpressionShape,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticDiagnostic {
    pub flow_id: Uuid,
    pub flow_name: String,
    pub code: SemanticDiagnosticCode,
    pub message: String,
    pub node_id: Option<ActionNodeId>,
    pub edge_index: Option<usize>,
}

impl SemanticDiagnostic {
    fn new(
        flow: &AppFlow,
        code: SemanticDiagnosticCode,
        message: String,
        node_id: Option<ActionNodeId>,
        edge_index: Option<usize>,
    ) -> Self {
        Self {
            flow_id: flow.id,
            flow_name: flow.name.clone(),
            code,
            message,
            node_id,
            edge_index,
        }
    }
}

pub fn format_diagnostic(diag: &SemanticDiagnostic) -> String {
    let mut parts = vec![format!(
        "flow '{}' ({}) [{:?}]",
        diag.flow_name, diag.flow_id, diag.code
    )];
    if let Some(node_id) = diag.node_id {
        parts.push(format!("node {}", node_id));
    }
    if let Some(edge_idx) = diag.edge_index {
        parts.push(format!("edge {}", edge_idx));
    }
    parts.push(diag.message.clone());
    parts.join(": ")
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoweredDataSource {
    pub from_node_id: ActionNodeId,
    pub from_port_label: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoweredExpressionType {
    String,
    Number,
    Bool,
    Enum { type_name: String },
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LoweredExpression {
    Literal(ActionValue),
    StateField(StateFieldRef),
    TriggerInput {
        name: String,
        value_type: ActionValueType,
    },
    Compare {
        operator: CompareOp,
        lhs: Box<LoweredExpression>,
        rhs: Option<Box<LoweredExpression>>,
    },
    LogicAnd {
        lhs: Box<LoweredExpression>,
        rhs: Box<LoweredExpression>,
    },
    LogicOr {
        lhs: Box<LoweredExpression>,
        rhs: Box<LoweredExpression>,
    },
    LogicNot {
        value: Box<LoweredExpression>,
    },
    Formula(String),
}

impl LoweredExpression {
    pub fn value_type(&self) -> LoweredExpressionType {
        match self {
            Self::Literal(ActionValue::String(_)) => LoweredExpressionType::String,
            Self::Literal(ActionValue::Number(_)) => LoweredExpressionType::Number,
            Self::Literal(ActionValue::Bool(_)) => LoweredExpressionType::Bool,
            Self::Literal(ActionValue::EnumVariant { type_name, .. }) => {
                LoweredExpressionType::Enum {
                    type_name: type_name.clone(),
                }
            }
            Self::StateField(sf) => match &sf.field_type {
                ActionValueType::String => LoweredExpressionType::String,
                ActionValueType::Bool => LoweredExpressionType::Bool,
                ActionValueType::F32 | ActionValueType::F64 | ActionValueType::Usize => {
                    LoweredExpressionType::Number
                }
                ActionValueType::Enum { type_name, .. } => LoweredExpressionType::Enum {
                    type_name: type_name.clone(),
                },
            },
            Self::TriggerInput { value_type, .. } => match value_type {
                ActionValueType::String => LoweredExpressionType::String,
                ActionValueType::Bool => LoweredExpressionType::Bool,
                ActionValueType::F32 | ActionValueType::F64 | ActionValueType::Usize => {
                    LoweredExpressionType::Number
                }
                ActionValueType::Enum { type_name, .. } => LoweredExpressionType::Enum {
                    type_name: type_name.clone(),
                },
            },
            Self::Compare { .. }
            | Self::LogicAnd { .. }
            | Self::LogicOr { .. }
            | Self::LogicNot { .. } => LoweredExpressionType::Bool,
            Self::Formula(_) => LoweredExpressionType::Unknown,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LoweredActionGraph {
    trigger_node_id: ActionNodeId,
    nodes_by_id: HashMap<ActionNodeId, ActionNodeData>,
    flow_successors: HashMap<(ActionNodeId, usize), Vec<(ActionNodeId, usize)>>,
    data_sources_by_input: HashMap<(ActionNodeId, String), LoweredDataSource>,
    input_expressions: HashMap<(ActionNodeId, String), LoweredExpression>,
    navigation_targets_by_input: HashMap<(ActionNodeId, usize), NavigateTarget>,
}

impl LoweredActionGraph {
    pub fn trigger_node_id(&self) -> ActionNodeId {
        self.trigger_node_id
    }

    pub fn node(&self, node_id: ActionNodeId) -> Option<&ActionNodeData> {
        self.nodes_by_id.get(&node_id)
    }

    pub fn flow_successors_with_input_slots(
        &self,
        node_id: ActionNodeId,
        output_slot: usize,
    ) -> Vec<(ActionNodeId, usize)> {
        self.flow_successors
            .get(&(node_id, output_slot))
            .cloned()
            .unwrap_or_default()
    }

    pub fn data_source_for_input_label(
        &self,
        node_id: ActionNodeId,
        input_label: &str,
    ) -> Option<LoweredDataSource> {
        self.data_sources_by_input
            .get(&(node_id, input_label.to_string()))
            .cloned()
    }

    pub fn expression_for_input_label(
        &self,
        node_id: ActionNodeId,
        input_label: &str,
    ) -> Option<&LoweredExpression> {
        self.input_expressions
            .get(&(node_id, input_label.to_string()))
    }

    pub fn navigation_target_for_input_slot(
        &self,
        node_id: ActionNodeId,
        input_slot: usize,
    ) -> Option<&NavigateTarget> {
        self.navigation_targets_by_input.get(&(node_id, input_slot))
    }
}

pub struct SemanticValidationContext<'a> {
    pub callable_flow_ids: &'a HashSet<Uuid>,
    pub known_view_ids: Option<&'a HashSet<Uuid>>,
}

pub type ViewReferenceIndex = HashMap<(Uuid, WidgetId), HashSet<Uuid>>;

pub fn build_view_reference_index(all_views: &BTreeMap<Uuid, AppView>) -> ViewReferenceIndex {
    fn collect_widget_view_refs(
        owner_view_id: Uuid,
        widget: &Widget,
        out: &mut ViewReferenceIndex,
    ) {
        if widget.widget_type == WidgetType::ViewReference {
            if let Some(primary_view_id) = widget.properties.referenced_view_id {
                let mut allowed = HashSet::from([primary_view_id]);
                allowed.extend(widget.properties.extra_view_ids.iter().copied());
                out.insert((owner_view_id, widget.id), allowed);
            }
        }
        for child in &widget.children {
            collect_widget_view_refs(owner_view_id, child, out);
        }
    }

    let mut index = ViewReferenceIndex::new();
    for (owner_view_id, view) in all_views {
        collect_widget_view_refs(*owner_view_id, view.hierarchy.root(), &mut index);
    }
    index
}

#[derive(Debug, Clone)]
pub struct LoweredWidgetEventFlow {
    pub flow_id: Uuid,
    pub flow_name: String,
    pub trigger_slot: usize,
    pub graph: LoweredActionGraph,
}

#[derive(Debug, Clone, Default)]
pub struct LoweredWidgetEventResult {
    pub flows: Vec<LoweredWidgetEventFlow>,
    pub diagnostics: Vec<SemanticDiagnostic>,
}

#[derive(Debug, Clone)]
struct PortInfo {
    label: String,
    slot: usize,
    kind: PortType,
}

type PortMap = HashMap<(ActionNodeId, u64), PortInfo>;

pub fn callable_flow_ids(flows: &[&AppFlow]) -> HashSet<Uuid> {
    flows
        .iter()
        .filter(|f| f.enabled && matches!(f.trigger, FlowTrigger::Callable))
        .map(|f| f.id)
        .collect()
}

pub fn validate_and_lower_flow_graph(
    flow: &AppFlow,
    context: &SemanticValidationContext<'_>,
) -> Result<LoweredActionGraph, Vec<SemanticDiagnostic>> {
    validate_and_lower_flow_graph_with_view_refs(flow, context, None)
}

pub fn validate_and_lower_flow_graph_with_view_refs(
    flow: &AppFlow,
    context: &SemanticValidationContext<'_>,
    view_reference_index: Option<&ViewReferenceIndex>,
) -> Result<LoweredActionGraph, Vec<SemanticDiagnostic>> {
    let diagnostics = validate_flow_graph(flow, context, view_reference_index);
    if !diagnostics.is_empty() {
        return Err(diagnostics);
    }

    match lower_flow_graph(flow) {
        Ok(lowered) => Ok(lowered),
        Err(mut lowering_diagnostics) => {
            if lowering_diagnostics.is_empty() {
                lowering_diagnostics.push(SemanticDiagnostic::new(
                    flow,
                    SemanticDiagnosticCode::InvalidNodeConfiguration,
                    "graph lowering failed".to_string(),
                    None,
                    None,
                ));
            }
            Err(lowering_diagnostics)
        }
    }
}

pub fn lower_widget_event_flows(
    flows: &[&AppFlow],
    origin_view_id: Uuid,
    widget_id: WidgetId,
    event_name: &str,
    known_view_ids: Option<&HashSet<Uuid>>,
) -> LoweredWidgetEventResult {
    lower_widget_event_flows_with_view_refs(
        flows,
        origin_view_id,
        widget_id,
        event_name,
        known_view_ids,
        None,
    )
}

pub fn lower_widget_event_flows_with_view_refs(
    flows: &[&AppFlow],
    origin_view_id: Uuid,
    widget_id: WidgetId,
    event_name: &str,
    known_view_ids: Option<&HashSet<Uuid>>,
    view_reference_index: Option<&ViewReferenceIndex>,
) -> LoweredWidgetEventResult {
    let callable_ids = callable_flow_ids(flows);
    let context = SemanticValidationContext {
        callable_flow_ids: &callable_ids,
        known_view_ids,
    };

    let mut result = LoweredWidgetEventResult::default();

    for flow in flows {
        if !flow.enabled {
            continue;
        }
        let FlowTrigger::WidgetEvent { rows } = &flow.trigger else {
            continue;
        };

        let Some(row_idx) = rows.iter().position(|r| {
            r.event_type == event_name && r.target == Some((origin_view_id, widget_id.0))
        }) else {
            continue;
        };

        let trigger_slot = ActionNodeData::widget_event_row_slot(rows, row_idx);
        let has_trigger_slot = flow
            .graph
            .trigger_node_id()
            .and_then(|trigger_id| flow.graph.nodes.iter().find(|n| n.id == trigger_id))
            .map(|trigger| {
                trigger
                    .cached_ports_out
                    .iter()
                    .any(|p| matches!(p.kind, PortType::Flow) && p.slot == trigger_slot)
            })
            .unwrap_or(false);

        if !has_trigger_slot {
            result.diagnostics.push(SemanticDiagnostic::new(
                flow,
                SemanticDiagnosticCode::InvalidTriggerTopology,
                format!(
                    "trigger row '{}' has no matching flow output slot {}",
                    rows[row_idx].event_type, trigger_slot
                ),
                flow.graph.trigger_node_id(),
                None,
            ));
            continue;
        }

        match validate_and_lower_flow_graph_with_view_refs(flow, &context, view_reference_index) {
            Ok(graph) => result.flows.push(LoweredWidgetEventFlow {
                flow_id: flow.id,
                flow_name: flow.name.clone(),
                trigger_slot,
                graph,
            }),
            Err(diags) => result.diagnostics.extend(diags),
        }
    }

    result
}

fn validate_flow_graph(
    flow: &AppFlow,
    context: &SemanticValidationContext<'_>,
    view_reference_index: Option<&ViewReferenceIndex>,
) -> Vec<SemanticDiagnostic> {
    let mut diagnostics = Vec::new();
    let graph = &flow.graph;

    let mut seen_node_ids = HashSet::new();
    for node in &graph.nodes {
        if !seen_node_ids.insert(node.id) {
            diagnostics.push(SemanticDiagnostic::new(
                flow,
                SemanticDiagnosticCode::DuplicateNodeId,
                format!("duplicate node id {}", node.id),
                Some(node.id),
                None,
            ));
        }
    }

    let trigger_nodes: Vec<_> = graph.nodes.iter().filter(|n| n.is_trigger()).collect();
    if trigger_nodes.is_empty() {
        diagnostics.push(SemanticDiagnostic::new(
            flow,
            SemanticDiagnosticCode::MissingTriggerNode,
            "graph has no trigger node".to_string(),
            None,
            None,
        ));
    } else if trigger_nodes.len() > 1 {
        diagnostics.push(SemanticDiagnostic::new(
            flow,
            SemanticDiagnosticCode::MultipleTriggerNodes,
            format!("graph has {} trigger nodes", trigger_nodes.len()),
            None,
            None,
        ));
    }

    let nodes_by_id: HashMap<ActionNodeId, &ActionNodeData> =
        graph.nodes.iter().map(|n| (n.id, n)).collect();
    let (ports_out, ports_in) = build_port_maps(graph);
    let mut incoming_to_input: HashMap<(ActionNodeId, u64), usize> = HashMap::new();
    let mut incoming_data_labels: HashSet<(ActionNodeId, String)> = HashSet::new();
    let mut data_sources_by_input: HashMap<(ActionNodeId, String), LoweredDataSource> =
        HashMap::new();
    let mut seen_edges: HashSet<(ActionNodeId, u64, ActionNodeId, u64)> = HashSet::new();

    for (edge_idx, edge) in graph.edges.iter().enumerate() {
        if !seen_edges.insert((edge.from_node, edge.from_port, edge.to_node, edge.to_port)) {
            diagnostics.push(SemanticDiagnostic::new(
                flow,
                SemanticDiagnosticCode::DuplicateEdge,
                "duplicate edge".to_string(),
                None,
                Some(edge_idx),
            ));
        }

        let Some(_from_node) = nodes_by_id.get(&edge.from_node) else {
            diagnostics.push(SemanticDiagnostic::new(
                flow,
                SemanticDiagnosticCode::DanglingEdgeNode,
                format!("edge source node {} does not exist", edge.from_node),
                None,
                Some(edge_idx),
            ));
            continue;
        };
        let Some(_to_node) = nodes_by_id.get(&edge.to_node) else {
            diagnostics.push(SemanticDiagnostic::new(
                flow,
                SemanticDiagnosticCode::DanglingEdgeNode,
                format!("edge destination node {} does not exist", edge.to_node),
                None,
                Some(edge_idx),
            ));
            continue;
        };

        let Some(from_port) = ports_out.get(&(edge.from_node, edge.from_port)) else {
            diagnostics.push(SemanticDiagnostic::new(
                flow,
                SemanticDiagnosticCode::DanglingEdgePort,
                format!(
                    "edge source port {} does not exist on node {}",
                    edge.from_port, edge.from_node
                ),
                Some(edge.from_node),
                Some(edge_idx),
            ));
            continue;
        };
        let Some(to_port) = ports_in.get(&(edge.to_node, edge.to_port)) else {
            diagnostics.push(SemanticDiagnostic::new(
                flow,
                SemanticDiagnosticCode::DanglingEdgePort,
                format!(
                    "edge destination port {} does not exist on node {}",
                    edge.to_port, edge.to_node
                ),
                Some(edge.to_node),
                Some(edge_idx),
            ));
            continue;
        };

        if edge.from_node == edge.to_node {
            diagnostics.push(SemanticDiagnostic::new(
                flow,
                SemanticDiagnosticCode::SelfLoopUnsupported,
                "self-loop edges are not supported".to_string(),
                Some(edge.from_node),
                Some(edge_idx),
            ));
        }

        let kind_ok = matches!(
            (&from_port.kind, &to_port.kind),
            (PortType::Flow, PortType::Flow) | (PortType::Data(_), PortType::Data(_))
        );
        if !kind_ok {
            diagnostics.push(SemanticDiagnostic::new(
                flow,
                SemanticDiagnosticCode::InvalidEdgeKinds,
                format!(
                    "incompatible edge kinds from node {} port '{}' to node {} port '{}'",
                    edge.from_node, from_port.label, edge.to_node, to_port.label
                ),
                None,
                Some(edge_idx),
            ));
            continue;
        }

        *incoming_to_input
            .entry((edge.to_node, edge.to_port))
            .or_insert(0) += 1;
        if matches!(
            (&from_port.kind, &to_port.kind),
            (PortType::Data(_), PortType::Data(_))
        ) {
            incoming_data_labels.insert((edge.to_node, to_port.label.clone()));
            data_sources_by_input.insert(
                (edge.to_node, to_port.label.clone()),
                LoweredDataSource {
                    from_node_id: edge.from_node,
                    from_port_label: from_port.label.clone(),
                },
            );
        }
    }

    for ((to_node, to_port), count) in incoming_to_input {
        if count > 1 {
            diagnostics.push(SemanticDiagnostic::new(
                flow,
                SemanticDiagnosticCode::MultipleIncomingEdge,
                format!(
                    "input port {} on node {} has {} incoming edges",
                    to_port, to_node, count
                ),
                Some(to_node),
                None,
            ));
        }
    }

    let legacy_expression_source_nodes =
        collect_legacy_expression_source_node_ids(&nodes_by_id, &data_sources_by_input);

    for node in &graph.nodes {
        match &node.kind {
            ActionNodeKind::StateMutation { assignments } => {
                if assignments.is_empty() {
                    diagnostics.push(SemanticDiagnostic::new(
                        flow,
                        SemanticDiagnosticCode::InvalidNodeConfiguration,
                        "StateMutation has no assignments".to_string(),
                        Some(node.id),
                        None,
                    ));
                }
                for (idx, assignment) in assignments.iter().enumerate() {
                    let Some(target_ref) = &assignment.target else {
                        diagnostics.push(SemanticDiagnostic::new(
                            flow,
                            SemanticDiagnosticCode::InvalidNodeConfiguration,
                            format!("StateMutation assignment {} target is not configured", idx),
                            Some(node.id),
                            None,
                        ));
                        continue;
                    };
                    if let Some(known_views) = context.known_view_ids {
                        if !known_views.contains(&target_ref.view_id) {
                            diagnostics.push(SemanticDiagnostic::new(
                                flow,
                                SemanticDiagnosticCode::UnknownViewReference,
                                format!(
                                    "StateMutation target view {} does not exist",
                                    target_ref.view_id
                                ),
                                Some(node.id),
                                None,
                            ));
                        }
                    }
                    match &assignment.value_source {
                        ValueSource::FromPort => {
                            let input_label = format!("value_{idx}");
                            if !incoming_data_labels.contains(&(node.id, input_label.clone())) {
                                diagnostics.push(SemanticDiagnostic::new(
                                    flow,
                                    SemanticDiagnosticCode::MissingRequiredInput,
                                    format!(
                                        "StateMutation assignment {} is FromPort but input '{}' is unconnected",
                                        idx, input_label
                                    ),
                                    Some(node.id),
                                    None,
                                ));
                            }
                        }
                        ValueSource::StateField(src) => {
                            if let Some(known_views) = context.known_view_ids {
                                if !known_views.contains(&src.view_id) {
                                    diagnostics.push(SemanticDiagnostic::new(
                                        flow,
                                        SemanticDiagnosticCode::UnknownViewReference,
                                        format!(
                                            "StateMutation source view {} does not exist",
                                            src.view_id
                                        ),
                                        Some(node.id),
                                        None,
                                    ));
                                }
                            }
                        }
                        ValueSource::Literal(_) => {}
                    }
                }
            }
            ActionNodeKind::Conditional => {
                let authored_conditions = effective_authored_conditions(node);
                if !authored_conditions.is_empty() {
                    for authored in authored_conditions.iter() {
                        validate_authored_condition(
                            flow,
                            node.id,
                            authored,
                            context,
                            &mut diagnostics,
                        );
                    }
                } else if !incoming_data_labels.contains(&(node.id, "condition".to_string())) {
                    diagnostics.push(SemanticDiagnostic::new(
                        flow,
                        SemanticDiagnosticCode::MissingRequiredInput,
                        "If requires either in-node authored condition or connected 'condition' input".to_string(),
                        Some(node.id),
                        None,
                    ));
                }
            }
            ActionNodeKind::Match { enum_type, .. } => {
                if let Some(authored_subject) = &node.authored_match_subject {
                    validate_authored_value_source(
                        flow,
                        node.id,
                        authored_subject,
                        context,
                        &mut diagnostics,
                        "subject",
                    );
                    if let Some(enum_name) = enum_type {
                        match authored_subject.value_type() {
                            ActionValueType::Enum { type_name, .. } if &type_name == enum_name => {}
                            ActionValueType::Enum { type_name, .. } => diagnostics.push(
                                SemanticDiagnostic::new(
                                    flow,
                                    SemanticDiagnosticCode::InvalidNodeConfiguration,
                                    format!(
                                        "Match enum type '{}' does not match authored subject enum type '{}'",
                                        enum_name, type_name
                                    ),
                                    Some(node.id),
                                    None,
                                ),
                            ),
                            other => diagnostics.push(SemanticDiagnostic::new(
                                flow,
                                SemanticDiagnosticCode::InvalidNodeConfiguration,
                                format!(
                                    "Match enum type '{}' requires enum authored subject, got {:?}",
                                    enum_name, other
                                ),
                                Some(node.id),
                                None,
                            )),
                        }
                    }
                } else if !incoming_data_labels.contains(&(node.id, "value".to_string())) {
                    diagnostics.push(SemanticDiagnostic::new(
                        flow,
                        SemanticDiagnosticCode::MissingRequiredInput,
                        "Match requires either in-node authored subject or connected 'value' input"
                            .to_string(),
                        Some(node.id),
                        None,
                    ));
                }
            }
            ActionNodeKind::NavigateToView { targets } => {
                if targets.is_empty() {
                    diagnostics.push(SemanticDiagnostic::new(
                        flow,
                        SemanticDiagnosticCode::InvalidNodeConfiguration,
                        "NavigateToView has no targets".to_string(),
                        Some(node.id),
                        None,
                    ));
                }
                for (slot, target) in targets.iter().enumerate() {
                    let Some(target) = target else {
                        diagnostics.push(SemanticDiagnostic::new(
                            flow,
                            SemanticDiagnosticCode::InvalidNodeConfiguration,
                            format!("NavigateToView target row {} is not configured", slot),
                            Some(node.id),
                            None,
                        ));
                        continue;
                    };
                    match target {
                        NavigateTarget::AppView { view_id } => {
                            if let Some(known_views) = context.known_view_ids {
                                if !known_views.contains(view_id) {
                                    diagnostics.push(SemanticDiagnostic::new(
                                        flow,
                                        SemanticDiagnosticCode::UnknownViewReference,
                                        format!(
                                            "NavigateToView references unknown app view {}",
                                            view_id
                                        ),
                                        Some(node.id),
                                        None,
                                    ));
                                }
                            }
                        }
                        NavigateTarget::ViewReference {
                            owner_view_id,
                            widget_id,
                            target_view_id,
                        } => {
                            if let Some(known_views) = context.known_view_ids {
                                if !known_views.contains(owner_view_id) {
                                    diagnostics.push(SemanticDiagnostic::new(
                                        flow,
                                        SemanticDiagnosticCode::UnknownViewReference,
                                        format!(
                                            "NavigateToView references unknown owner view {}",
                                            owner_view_id
                                        ),
                                        Some(node.id),
                                        None,
                                    ));
                                }
                                if !known_views.contains(target_view_id) {
                                    diagnostics.push(SemanticDiagnostic::new(
                                        flow,
                                        SemanticDiagnosticCode::UnknownViewReference,
                                        format!(
                                            "NavigateToView references unknown target view {}",
                                            target_view_id
                                        ),
                                        Some(node.id),
                                        None,
                                    ));
                                }
                            }
                            if let Some(index) = view_reference_index {
                                let Some(allowed_targets) =
                                    index.get(&(*owner_view_id, *widget_id))
                                else {
                                    diagnostics.push(SemanticDiagnostic::new(
                                        flow,
                                        SemanticDiagnosticCode::UnknownViewReference,
                                        format!(
                                            "NavigateToView references missing ViewReference ({}, {})",
                                            owner_view_id, widget_id.0
                                        ),
                                        Some(node.id),
                                        None,
                                    ));
                                    continue;
                                };
                                if !allowed_targets.contains(target_view_id) {
                                    diagnostics.push(SemanticDiagnostic::new(
                                        flow,
                                        SemanticDiagnosticCode::InvalidNodeConfiguration,
                                        format!(
                                            "NavigateToView target view {} is not configured on ViewReference ({}, {})",
                                            target_view_id, owner_view_id, widget_id.0
                                        ),
                                        Some(node.id),
                                        None,
                                    ));
                                }
                            }
                        }
                    }
                }
            }
            ActionNodeKind::CallFlow { flow_id } => {
                let Some(target_id) = flow_id else {
                    diagnostics.push(SemanticDiagnostic::new(
                        flow,
                        SemanticDiagnosticCode::InvalidNodeConfiguration,
                        "CallFlow has no target flow".to_string(),
                        Some(node.id),
                        None,
                    ));
                    continue;
                };
                if !context.callable_flow_ids.contains(target_id) {
                    diagnostics.push(SemanticDiagnostic::new(
                        flow,
                        SemanticDiagnosticCode::UnresolvedReference,
                        format!("CallFlow references unknown callable flow {}", target_id),
                        Some(node.id),
                        None,
                    ));
                }
            }
            ActionNodeKind::SetState { .. } | ActionNodeKind::UpdateState { .. } => diagnostics
                .push(SemanticDiagnostic::new(
                    flow,
                    SemanticDiagnosticCode::UnsupportedNodeKind,
                    "legacy SetState/UpdateState node is unsupported; replace with StateMutation"
                        .to_string(),
                    Some(node.id),
                    None,
                )),
            ActionNodeKind::CallAction { .. } => diagnostics.push(SemanticDiagnostic::new(
                flow,
                SemanticDiagnosticCode::UnsupportedNodeKind,
                "legacy CallAction node is unsupported; replace with CallFlow".to_string(),
                Some(node.id),
                None,
            )),
            ActionNodeKind::LegacyGetState { .. } => diagnostics.push(SemanticDiagnostic::new(
                flow,
                SemanticDiagnosticCode::UnsupportedNodeKind,
                "LegacyGetState is unsupported".to_string(),
                Some(node.id),
                None,
            )),
            ActionNodeKind::StringLiteral { .. }
            | ActionNodeKind::NumberLiteral { .. }
            | ActionNodeKind::BoolLiteral { .. }
            | ActionNodeKind::EnumLiteral { .. }
            | ActionNodeKind::Compare { .. }
            | ActionNodeKind::LogicAnd
            | ActionNodeKind::LogicOr
            | ActionNodeKind::LogicNot
            | ActionNodeKind::Expression { .. } => {
                if !legacy_expression_source_nodes.contains(&node.id) {
                    diagnostics.push(SemanticDiagnostic::new(
                        flow,
                        SemanticDiagnosticCode::UnsupportedNodeKind,
                        "legacy value/operator node is retired from the primary model; remove it or connect it as an expression source"
                            .to_string(),
                        Some(node.id),
                        None,
                    ));
                }
            }
            ActionNodeKind::Trigger { .. } => {}
        }
    }

    diagnostics
}

fn is_numeric_type(value_type: &ActionValueType) -> bool {
    matches!(
        value_type,
        ActionValueType::F32 | ActionValueType::F64 | ActionValueType::Usize
    )
}

fn rhs_literal_matches_lhs_type(rhs: &ActionValue, lhs_type: &ActionValueType) -> bool {
    match lhs_type {
        ActionValueType::String => matches!(rhs, ActionValue::String(_)),
        ActionValueType::Bool => matches!(rhs, ActionValue::Bool(_)),
        ActionValueType::F32 | ActionValueType::F64 | ActionValueType::Usize => {
            matches!(rhs, ActionValue::Number(_))
        }
        ActionValueType::Enum { type_name, .. } => match rhs {
            ActionValue::EnumVariant {
                type_name: rhs_type,
                ..
            } => rhs_type == type_name,
            _ => false,
        },
    }
}

fn validate_authored_value_source(
    flow: &AppFlow,
    node_id: ActionNodeId,
    source: &AuthoredValueSource,
    context: &SemanticValidationContext<'_>,
    diagnostics: &mut Vec<SemanticDiagnostic>,
    source_label: &str,
) {
    if let AuthoredValueSource::StateField(field_ref) = source {
        if let Some(known_views) = context.known_view_ids {
            if !known_views.contains(&field_ref.view_id) {
                diagnostics.push(SemanticDiagnostic::new(
                    flow,
                    SemanticDiagnosticCode::UnknownViewReference,
                    format!(
                        "If/Match authored {} source references unknown view {}",
                        source_label, field_ref.view_id
                    ),
                    Some(node_id),
                    None,
                ));
            }
        }
    }
}

fn validate_authored_condition(
    flow: &AppFlow,
    node_id: ActionNodeId,
    condition: &AuthoredCondition,
    context: &SemanticValidationContext<'_>,
    diagnostics: &mut Vec<SemanticDiagnostic>,
) {
    validate_authored_value_source(flow, node_id, &condition.lhs, context, diagnostics, "lhs");
    let lhs_type = condition.lhs.value_type();

    let type_ok = match condition.operator {
        CompareOp::Contains
        | CompareOp::StartsWith
        | CompareOp::EndsWith
        | CompareOp::IsEmpty
        | CompareOp::IsNotEmpty
        | CompareOp::IsValidEmail => matches!(lhs_type, ActionValueType::String),
        CompareOp::Lt | CompareOp::Gt | CompareOp::LtEq | CompareOp::GtEq => {
            is_numeric_type(&lhs_type)
        }
        CompareOp::IsTrue | CompareOp::IsFalse => matches!(lhs_type, ActionValueType::Bool),
        CompareOp::Eq | CompareOp::NotEq => true,
    };

    if !type_ok {
        diagnostics.push(SemanticDiagnostic::new(
            flow,
            SemanticDiagnosticCode::InvalidNodeConfiguration,
            format!(
                "If authored condition operator '{}' is incompatible with lhs type {:?}",
                condition.operator, lhs_type
            ),
            Some(node_id),
            None,
        ));
    }

    if condition.operator.needs_rhs()
        && !rhs_literal_matches_lhs_type(&condition.rhs_literal, &lhs_type)
    {
        diagnostics.push(SemanticDiagnostic::new(
            flow,
            SemanticDiagnosticCode::InvalidNodeConfiguration,
            format!(
                "If authored condition rhs literal is incompatible with lhs type {:?}",
                lhs_type
            ),
            Some(node_id),
            None,
        ));
    }
}

fn effective_authored_conditions(node: &ActionNodeData) -> Vec<AuthoredCondition> {
    if !node.authored_conditions.is_empty() {
        node.authored_conditions.clone()
    } else if let Some(legacy) = &node.authored_condition {
        vec![legacy.clone()]
    } else {
        Vec::new()
    }
}

fn lower_authored_conditions(node: &ActionNodeData) -> Option<LoweredExpression> {
    let authored = effective_authored_conditions(node);
    let mut iter = authored.iter().map(lower_authored_condition);
    let mut expr = iter.next()?;
    for next in iter {
        expr = match node.authored_condition_join {
            crate::action_system::node_kinds::ConditionJoinMode::All => {
                LoweredExpression::LogicAnd {
                    lhs: Box::new(expr),
                    rhs: Box::new(next),
                }
            }
            crate::action_system::node_kinds::ConditionJoinMode::Any => {
                LoweredExpression::LogicOr {
                    lhs: Box::new(expr),
                    rhs: Box::new(next),
                }
            }
        };
    }
    Some(expr)
}

fn lower_authored_value_source(source: &AuthoredValueSource) -> LoweredExpression {
    match source {
        AuthoredValueSource::TriggerInput { name, value_type } => LoweredExpression::TriggerInput {
            name: name.clone(),
            value_type: value_type.clone(),
        },
        AuthoredValueSource::StateField(field_ref) => {
            LoweredExpression::StateField(field_ref.clone())
        }
        AuthoredValueSource::Literal(v) => LoweredExpression::Literal(v.clone()),
    }
}

fn lower_authored_condition(condition: &AuthoredCondition) -> LoweredExpression {
    let lhs = lower_authored_value_source(&condition.lhs);
    let rhs = if condition.operator.needs_rhs() {
        Some(Box::new(LoweredExpression::Literal(
            condition.rhs_literal.clone(),
        )))
    } else {
        None
    };
    LoweredExpression::Compare {
        operator: condition.operator.clone(),
        lhs: Box::new(lhs),
        rhs,
    }
}

fn collect_legacy_expression_source_node_ids(
    nodes_by_id: &HashMap<ActionNodeId, &ActionNodeData>,
    data_sources_by_input: &HashMap<(ActionNodeId, String), LoweredDataSource>,
) -> HashSet<ActionNodeId> {
    fn visit_input(
        node_id: ActionNodeId,
        input_label: &str,
        nodes_by_id: &HashMap<ActionNodeId, &ActionNodeData>,
        data_sources_by_input: &HashMap<(ActionNodeId, String), LoweredDataSource>,
        visited_inputs: &mut HashSet<(ActionNodeId, String)>,
        used_sources: &mut HashSet<ActionNodeId>,
    ) {
        let key = (node_id, input_label.to_string());
        if !visited_inputs.insert(key.clone()) {
            return;
        }
        let Some(source) = data_sources_by_input.get(&key) else {
            return;
        };
        visit_source_node(
            source.from_node_id,
            nodes_by_id,
            data_sources_by_input,
            visited_inputs,
            used_sources,
        );
    }

    fn visit_source_node(
        source_node_id: ActionNodeId,
        nodes_by_id: &HashMap<ActionNodeId, &ActionNodeData>,
        data_sources_by_input: &HashMap<(ActionNodeId, String), LoweredDataSource>,
        visited_inputs: &mut HashSet<(ActionNodeId, String)>,
        used_sources: &mut HashSet<ActionNodeId>,
    ) {
        if !used_sources.insert(source_node_id) {
            return;
        }
        let Some(node) = nodes_by_id.get(&source_node_id) else {
            return;
        };
        match &node.kind {
            ActionNodeKind::Compare { operator, rhs, .. } => {
                visit_input(
                    node.id,
                    "value",
                    nodes_by_id,
                    data_sources_by_input,
                    visited_inputs,
                    used_sources,
                );
                if *rhs == CompareRhs::FromPort && operator.needs_rhs() {
                    visit_input(
                        node.id,
                        "rhs",
                        nodes_by_id,
                        data_sources_by_input,
                        visited_inputs,
                        used_sources,
                    );
                }
            }
            ActionNodeKind::LogicAnd | ActionNodeKind::LogicOr => {
                visit_input(
                    node.id,
                    "a",
                    nodes_by_id,
                    data_sources_by_input,
                    visited_inputs,
                    used_sources,
                );
                visit_input(
                    node.id,
                    "b",
                    nodes_by_id,
                    data_sources_by_input,
                    visited_inputs,
                    used_sources,
                );
            }
            ActionNodeKind::LogicNot => {
                visit_input(
                    node.id,
                    "value",
                    nodes_by_id,
                    data_sources_by_input,
                    visited_inputs,
                    used_sources,
                );
            }
            _ => {}
        }
    }

    let mut used_sources = HashSet::new();
    let mut visited_inputs = HashSet::new();

    for node in nodes_by_id.values() {
        match &node.kind {
            ActionNodeKind::Conditional => {
                if effective_authored_conditions(node).is_empty() {
                    visit_input(
                        node.id,
                        "condition",
                        nodes_by_id,
                        data_sources_by_input,
                        &mut visited_inputs,
                        &mut used_sources,
                    );
                }
            }
            ActionNodeKind::Match { .. } => {
                if node.authored_match_subject.is_none() {
                    visit_input(
                        node.id,
                        "value",
                        nodes_by_id,
                        data_sources_by_input,
                        &mut visited_inputs,
                        &mut used_sources,
                    );
                }
            }
            ActionNodeKind::StateMutation { assignments } => {
                for (idx, assignment) in assignments.iter().enumerate() {
                    if matches!(assignment.value_source, ValueSource::FromPort) {
                        visit_input(
                            node.id,
                            &format!("value_{idx}"),
                            nodes_by_id,
                            data_sources_by_input,
                            &mut visited_inputs,
                            &mut used_sources,
                        );
                    }
                }
            }
            _ => {}
        }
    }

    used_sources
}

fn lower_flow_graph(flow: &AppFlow) -> Result<LoweredActionGraph, Vec<SemanticDiagnostic>> {
    let graph = &flow.graph;
    let mut diagnostics = Vec::new();

    let Some(trigger_node_id) = graph.trigger_node_id() else {
        diagnostics.push(SemanticDiagnostic::new(
            flow,
            SemanticDiagnosticCode::MissingTriggerNode,
            "graph has no trigger node".to_string(),
            None,
            None,
        ));
        return Err(diagnostics);
    };

    let nodes_by_id: HashMap<ActionNodeId, ActionNodeData> =
        graph.nodes.iter().map(|n| (n.id, n.clone())).collect();
    let (ports_out, ports_in) = build_port_maps(graph);

    let mut flow_successors: HashMap<(ActionNodeId, usize), Vec<(ActionNodeId, usize)>> =
        HashMap::new();
    let mut data_sources_by_input: HashMap<(ActionNodeId, String), LoweredDataSource> =
        HashMap::new();

    for edge in &graph.edges {
        let Some(from_port) = ports_out.get(&(edge.from_node, edge.from_port)) else {
            continue;
        };
        let Some(to_port) = ports_in.get(&(edge.to_node, edge.to_port)) else {
            continue;
        };

        match (&from_port.kind, &to_port.kind) {
            (PortType::Flow, PortType::Flow) => {
                flow_successors
                    .entry((edge.from_node, from_port.slot))
                    .or_default()
                    .push((edge.to_node, to_port.slot));
            }
            (PortType::Data(_), PortType::Data(_)) => {
                data_sources_by_input.insert(
                    (edge.to_node, to_port.label.clone()),
                    LoweredDataSource {
                        from_node_id: edge.from_node,
                        from_port_label: from_port.label.clone(),
                    },
                );
            }
            _ => {}
        }
    }

    let mut input_expressions: HashMap<(ActionNodeId, String), LoweredExpression> = HashMap::new();
    let mut navigation_targets_by_input: HashMap<(ActionNodeId, usize), NavigateTarget> =
        HashMap::new();
    for node in graph.nodes.iter() {
        match &node.kind {
            ActionNodeKind::Conditional => {
                let expr = if !effective_authored_conditions(node).is_empty() {
                    lower_authored_conditions(node)
                } else {
                    lower_expression_from_input(
                        flow,
                        node.id,
                        "condition",
                        &nodes_by_id,
                        &data_sources_by_input,
                        &mut HashSet::new(),
                        &mut diagnostics,
                    )
                };
                if let Some(expr) = expr {
                    if !matches!(expr.value_type(), LoweredExpressionType::Bool) {
                        diagnostics.push(SemanticDiagnostic::new(
                            flow,
                            SemanticDiagnosticCode::InvalidNodeConfiguration,
                            "If requires a boolean expression".to_string(),
                            Some(node.id),
                            None,
                        ));
                    }
                    input_expressions.insert((node.id, "condition".to_string()), expr);
                }
            }
            ActionNodeKind::Match { enum_type, .. } => {
                let expr = if let Some(authored_subject) = &node.authored_match_subject {
                    Some(lower_authored_value_source(authored_subject))
                } else {
                    lower_expression_from_input(
                        flow,
                        node.id,
                        "value",
                        &nodes_by_id,
                        &data_sources_by_input,
                        &mut HashSet::new(),
                        &mut diagnostics,
                    )
                };
                if let Some(expr) = expr {
                    if let Some(type_name) = enum_type {
                        match expr.value_type() {
                            LoweredExpressionType::Enum {
                                type_name: expr_enum,
                            } if expr_enum == *type_name => {}
                            other => diagnostics.push(SemanticDiagnostic::new(
                                flow,
                                SemanticDiagnosticCode::InvalidNodeConfiguration,
                                format!(
                                    "Match enum type '{}' requires enum expression; got {:?}",
                                    type_name, other
                                ),
                                Some(node.id),
                                None,
                            )),
                        }
                    }
                    input_expressions.insert((node.id, "value".to_string()), expr);
                }
            }
            ActionNodeKind::StateMutation { assignments } => {
                for (idx, assignment) in assignments.iter().enumerate() {
                    let key = format!("value_{idx}");
                    let expr = match &assignment.value_source {
                        ValueSource::Literal(v) => Some(LoweredExpression::Literal(v.clone())),
                        ValueSource::StateField(src) => {
                            Some(LoweredExpression::StateField(src.clone()))
                        }
                        ValueSource::FromPort => lower_expression_from_input(
                            flow,
                            node.id,
                            &key,
                            &nodes_by_id,
                            &data_sources_by_input,
                            &mut HashSet::new(),
                            &mut diagnostics,
                        ),
                    };
                    if let Some(expr) = expr {
                        input_expressions.insert((node.id, key), expr);
                    }
                }
            }
            ActionNodeKind::NavigateToView { targets } => {
                for (slot, target) in targets.iter().enumerate() {
                    if let Some(target) = target {
                        navigation_targets_by_input.insert((node.id, slot), target.clone());
                    }
                }
            }
            _ => {}
        }
    }

    if !diagnostics.is_empty() {
        return Err(diagnostics);
    }

    Ok(LoweredActionGraph {
        trigger_node_id,
        nodes_by_id,
        flow_successors,
        data_sources_by_input,
        input_expressions,
        navigation_targets_by_input,
    })
}

fn lower_expression_from_input(
    flow: &AppFlow,
    node_id: ActionNodeId,
    input_label: &str,
    nodes_by_id: &HashMap<ActionNodeId, ActionNodeData>,
    data_sources_by_input: &HashMap<(ActionNodeId, String), LoweredDataSource>,
    stack: &mut HashSet<(ActionNodeId, String)>,
    diagnostics: &mut Vec<SemanticDiagnostic>,
) -> Option<LoweredExpression> {
    let key = (node_id, input_label.to_string());
    if !stack.insert(key.clone()) {
        diagnostics.push(SemanticDiagnostic::new(
            flow,
            SemanticDiagnosticCode::UnsupportedExpressionShape,
            format!(
                "expression cycle detected at node {} input '{}'",
                node_id, input_label
            ),
            Some(node_id),
            None,
        ));
        return None;
    }

    let source = data_sources_by_input.get(&key).cloned();
    let expr = if let Some(source) = source {
        lower_expression_from_source_node(
            flow,
            source.from_node_id,
            &source.from_port_label,
            nodes_by_id,
            data_sources_by_input,
            stack,
            diagnostics,
        )
    } else {
        diagnostics.push(SemanticDiagnostic::new(
            flow,
            SemanticDiagnosticCode::MissingRequiredInput,
            format!(
                "input '{}' on node {} is not connected to an expression source",
                input_label, node_id
            ),
            Some(node_id),
            None,
        ));
        None
    };

    stack.remove(&key);
    expr
}

fn lower_expression_from_source_node(
    flow: &AppFlow,
    source_node_id: ActionNodeId,
    source_port_label: &str,
    nodes_by_id: &HashMap<ActionNodeId, ActionNodeData>,
    data_sources_by_input: &HashMap<(ActionNodeId, String), LoweredDataSource>,
    stack: &mut HashSet<(ActionNodeId, String)>,
    diagnostics: &mut Vec<SemanticDiagnostic>,
) -> Option<LoweredExpression> {
    let Some(node) = nodes_by_id.get(&source_node_id) else {
        diagnostics.push(SemanticDiagnostic::new(
            flow,
            SemanticDiagnosticCode::UnresolvedReference,
            format!(
                "expression references unknown source node {}",
                source_node_id
            ),
            Some(source_node_id),
            None,
        ));
        return None;
    };

    match &node.kind {
        ActionNodeKind::StringLiteral { value } => Some(LoweredExpression::Literal(
            ActionValue::String(value.clone()),
        )),
        ActionNodeKind::NumberLiteral { value } => {
            Some(LoweredExpression::Literal(ActionValue::Number(*value)))
        }
        ActionNodeKind::BoolLiteral { value } => {
            Some(LoweredExpression::Literal(ActionValue::Bool(*value)))
        }
        ActionNodeKind::EnumLiteral { enum_name, variant } => {
            match (enum_name.clone(), variant.clone()) {
                (Some(type_name), Some(variant)) => {
                    Some(LoweredExpression::Literal(ActionValue::EnumVariant {
                        type_name,
                        variant,
                    }))
                }
                _ => {
                    diagnostics.push(SemanticDiagnostic::new(
                        flow,
                        SemanticDiagnosticCode::InvalidNodeConfiguration,
                        "EnumLiteral used in expression but enum/variant is not configured"
                            .to_string(),
                        Some(node.id),
                        None,
                    ));
                    None
                }
            }
        }
        ActionNodeKind::Trigger { output_ports, .. } => {
            let value_type = output_ports
                .iter()
                .find(|p| p.name == source_port_label)
                .map(|p| p.value_type.clone())
                .unwrap_or(ActionValueType::String);
            Some(LoweredExpression::TriggerInput {
                name: source_port_label.to_string(),
                value_type,
            })
        }
        ActionNodeKind::Compare {
            operator,
            rhs,
            rhs_literal,
        } => {
            let lhs = lower_expression_from_input(
                flow,
                node.id,
                "value",
                nodes_by_id,
                data_sources_by_input,
                stack,
                diagnostics,
            )?;
            let rhs_expr = if operator.needs_rhs() {
                match rhs {
                    CompareRhs::Literal => Some(LoweredExpression::Literal(rhs_literal.clone())),
                    CompareRhs::FromPort => lower_expression_from_input(
                        flow,
                        node.id,
                        "rhs",
                        nodes_by_id,
                        data_sources_by_input,
                        stack,
                        diagnostics,
                    ),
                }
                .map(Box::new)
            } else {
                None
            };
            Some(LoweredExpression::Compare {
                operator: operator.clone(),
                lhs: Box::new(lhs),
                rhs: rhs_expr,
            })
        }
        ActionNodeKind::LogicAnd => {
            let lhs = lower_expression_from_input(
                flow,
                node.id,
                "a",
                nodes_by_id,
                data_sources_by_input,
                stack,
                diagnostics,
            )?;
            let rhs = lower_expression_from_input(
                flow,
                node.id,
                "b",
                nodes_by_id,
                data_sources_by_input,
                stack,
                diagnostics,
            )?;
            Some(LoweredExpression::LogicAnd {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            })
        }
        ActionNodeKind::LogicOr => {
            let lhs = lower_expression_from_input(
                flow,
                node.id,
                "a",
                nodes_by_id,
                data_sources_by_input,
                stack,
                diagnostics,
            )?;
            let rhs = lower_expression_from_input(
                flow,
                node.id,
                "b",
                nodes_by_id,
                data_sources_by_input,
                stack,
                diagnostics,
            )?;
            Some(LoweredExpression::LogicOr {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            })
        }
        ActionNodeKind::LogicNot => {
            let value = lower_expression_from_input(
                flow,
                node.id,
                "value",
                nodes_by_id,
                data_sources_by_input,
                stack,
                diagnostics,
            )?;
            Some(LoweredExpression::LogicNot {
                value: Box::new(value),
            })
        }
        ActionNodeKind::Expression { formula } => Some(LoweredExpression::Formula(formula.clone())),
        _ => {
            diagnostics.push(SemanticDiagnostic::new(
                flow,
                SemanticDiagnosticCode::UnsupportedExpressionShape,
                format!(
                    "node '{}' cannot be used as an expression source",
                    node.kind.display_name()
                ),
                Some(node.id),
                None,
            ));
            None
        }
    }
}

fn build_port_maps(graph: &ActionGraph) -> (PortMap, PortMap) {
    let mut out = HashMap::new();
    let mut input = HashMap::new();
    for node in &graph.nodes {
        for port in &node.cached_ports_out {
            out.insert(
                (node.id, port.id.0),
                PortInfo {
                    label: port.label.to_string(),
                    slot: port.slot,
                    kind: port.kind.clone(),
                },
            );
        }
        for port in &node.cached_ports_in {
            input.insert(
                (node.id, port.id.0),
                PortInfo {
                    label: port.label.to_string(),
                    slot: port.slot,
                    kind: port.kind.clone(),
                },
            );
        }
    }
    (out, input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::Point;

    use crate::action_system::graph::{ActionEdge, ActionNodeData};
    use crate::action_system::node_kinds::{
        ActionValue, AuthoredCondition, AuthoredValueSource, CompareOp, ConditionJoinMode,
        NavigateTarget,
    };
    use crate::action_system::state_ref::{ActionValueType, StateFieldRef, StateRefSource};
    use crate::data_structures::types::types::{AppView, WidgetType};

    fn custom_target(view_id: Uuid, field_id: Uuid, field_name: &str) -> StateFieldRef {
        StateFieldRef {
            view_id,
            source: StateRefSource::Custom {
                field_id,
                field_name: field_name.to_string(),
            },
            field_type: ActionValueType::String,
            display_name: field_name.to_string(),
        }
    }

    fn trigger_flow_out(flow: &AppFlow) -> u64 {
        flow.graph
            .nodes
            .iter()
            .find(|n| n.is_trigger())
            .and_then(|n| {
                n.cached_ports_out
                    .iter()
                    .find(|p| matches!(p.kind, PortType::Flow))
            })
            .expect("trigger flow_out")
            .id
            .0
    }

    fn port_in(node: &ActionNodeData, label: &str) -> u64 {
        node.cached_ports_in
            .iter()
            .find(|p| p.label == label)
            .unwrap_or_else(|| panic!("missing input port '{label}'"))
            .id
            .0
    }

    fn port_out(node: &ActionNodeData, label: &str) -> u64 {
        node.cached_ports_out
            .iter()
            .find(|p| p.label == label)
            .unwrap_or_else(|| panic!("missing output port '{label}'"))
            .id
            .0
    }

    fn owner_views_with_single_view_reference(
        owner_view_id: Uuid,
        primary_view_id: Uuid,
        secondary_view_id: Uuid,
    ) -> (BTreeMap<Uuid, AppView>, WidgetId) {
        let mut owner_view = AppView::with_id(owner_view_id, "Owner".to_string(), 0);
        let layout_id = owner_view
            .hierarchy
            .add_child(WidgetId(0), WidgetType::Column)
            .expect("add owner layout");
        let view_ref_id = owner_view
            .hierarchy
            .add_child(layout_id, WidgetType::ViewReference)
            .expect("add view reference");
        let view_ref = owner_view
            .hierarchy
            .get_widget_by_id_mut(view_ref_id)
            .expect("view reference widget");
        view_ref.properties.widget_name = "Main Pane".to_string();
        view_ref.properties.referenced_view_id = Some(primary_view_id);
        view_ref.properties.extra_view_ids = vec![secondary_view_id];

        let primary_view = AppView::with_id(primary_view_id, "Primary".to_string(), 1);
        let secondary_view = AppView::with_id(secondary_view_id, "Secondary".to_string(), 2);

        let views = BTreeMap::from([
            (owner_view_id, owner_view),
            (primary_view_id, primary_view),
            (secondary_view_id, secondary_view),
        ]);
        (views, view_ref_id)
    }

    #[test]
    fn validate_and_lower_accepts_valid_supported_flow() {
        let view_id = Uuid::new_v4();
        let field_id = Uuid::new_v4();
        let widget_id = WidgetId(1);

        let row = crate::action_system::flow::WidgetEventRow {
            id: Uuid::new_v4(),
            event_type: "on_press".to_string(),
            target: Some((view_id, widget_id.0)),
        };
        let mut flow = AppFlow::new(
            "flow".to_string(),
            FlowTrigger::WidgetEvent { rows: vec![row] },
        );
        flow.sync_trigger_topology();

        let node = ActionNodeData::new(
            2,
            ActionNodeKind::StateMutation {
                assignments: vec![crate::action_system::node_kinds::StateAssignment {
                    target: Some(custom_target(view_id, field_id, "value")),
                    value_source: ValueSource::Literal(ActionValue::String("ok".to_string())),
                }],
            },
            Point::new(260.0, 120.0),
        );
        let flow_in = node
            .cached_ports_in
            .iter()
            .find(|p| p.label == "flow_in")
            .expect("flow_in")
            .id
            .0;
        flow.graph.nodes.push(node);
        flow.graph.z_order.push(2);
        flow.graph.next_id = 3;
        assert!(
            flow.graph
                .connect_ports(1, trigger_flow_out(&flow), 2, flow_in)
        );

        let callable = HashSet::new();
        let known_views = HashSet::from([view_id]);
        let context = SemanticValidationContext {
            callable_flow_ids: &callable,
            known_view_ids: Some(&known_views),
        };
        let lowered = validate_and_lower_flow_graph(&flow, &context).expect("valid lowered flow");
        assert_eq!(lowered.trigger_node_id(), 1);
        assert_eq!(lowered.flow_successors_with_input_slots(1, 0), vec![(2, 0)]);
    }

    #[test]
    fn validate_diagnoses_dangling_edge_node() {
        let mut flow = AppFlow::new("flow".to_string(), FlowTrigger::Callable);
        flow.graph.edges.push(ActionEdge {
            from_node: 1,
            from_port: 1001,
            to_node: 999,
            to_port: 0,
        });
        let callable = HashSet::new();
        let context = SemanticValidationContext {
            callable_flow_ids: &callable,
            known_view_ids: None,
        };
        let err = validate_and_lower_flow_graph(&flow, &context).expect_err("invalid graph");
        assert!(
            err.iter()
                .any(|d| d.code == SemanticDiagnosticCode::DanglingEdgeNode)
        );
    }

    #[test]
    fn validate_diagnoses_nonexistent_port_reference() {
        let mut flow = AppFlow::new("flow".to_string(), FlowTrigger::Callable);
        let set_state = ActionNodeData::new(
            2,
            ActionNodeKind::StateMutation {
                assignments: vec![crate::action_system::node_kinds::StateAssignment {
                    target: None,
                    value_source: ValueSource::Literal(ActionValue::String(String::new())),
                }],
            },
            Point::new(260.0, 120.0),
        );
        let to_port = set_state
            .cached_ports_in
            .iter()
            .find(|p| p.label == "flow_in")
            .expect("flow_in")
            .id
            .0;
        flow.graph.nodes.push(set_state);
        flow.graph.z_order.push(2);
        flow.graph.next_id = 3;
        flow.graph.edges.push(ActionEdge {
            from_node: 1,
            from_port: 123456789,
            to_node: 2,
            to_port,
        });

        let callable = HashSet::new();
        let context = SemanticValidationContext {
            callable_flow_ids: &callable,
            known_view_ids: None,
        };
        let err = validate_and_lower_flow_graph(&flow, &context).expect_err("invalid graph");
        assert!(
            err.iter()
                .any(|d| d.code == SemanticDiagnosticCode::DanglingEdgePort)
        );
    }

    #[test]
    fn validate_diagnoses_invalid_node_configuration() {
        let mut flow = AppFlow::new("flow".to_string(), FlowTrigger::Callable);
        let set_state = ActionNodeData::new(
            2,
            ActionNodeKind::StateMutation {
                assignments: vec![crate::action_system::node_kinds::StateAssignment {
                    target: None,
                    value_source: ValueSource::Literal(ActionValue::String(String::new())),
                }],
            },
            Point::new(280.0, 120.0),
        );
        flow.graph.nodes.push(set_state);
        flow.graph.z_order.push(2);
        flow.graph.next_id = 3;
        let callable = HashSet::new();
        let context = SemanticValidationContext {
            callable_flow_ids: &callable,
            known_view_ids: None,
        };
        let err = validate_and_lower_flow_graph(&flow, &context).expect_err("invalid graph");
        assert!(err.iter().any(|d| {
            d.code == SemanticDiagnosticCode::InvalidNodeConfiguration && d.node_id == Some(2)
        }));
    }

    #[test]
    fn lowering_preserves_fanout_in_edge_insertion_order() {
        let first_id = Uuid::new_v4();
        let second_id = Uuid::new_v4();
        let callable = HashSet::from([first_id, second_id]);
        let context = SemanticValidationContext {
            callable_flow_ids: &callable,
            known_view_ids: None,
        };

        let mut flow = AppFlow::new("flow".to_string(), FlowTrigger::Callable);
        let first = ActionNodeData::new(
            2,
            ActionNodeKind::CallFlow {
                flow_id: Some(first_id),
            },
            Point::new(260.0, 120.0),
        );
        let second = ActionNodeData::new(
            3,
            ActionNodeKind::CallFlow {
                flow_id: Some(second_id),
            },
            Point::new(360.0, 120.0),
        );
        let first_in = first
            .cached_ports_in
            .iter()
            .find(|p| p.label == "flow_in")
            .expect("first flow_in")
            .id
            .0;
        let second_in = second
            .cached_ports_in
            .iter()
            .find(|p| p.label == "flow_in")
            .expect("second flow_in")
            .id
            .0;
        flow.graph.nodes.extend([first, second]);
        flow.graph.z_order.extend([2, 3]);
        flow.graph.next_id = 4;

        assert!(
            flow.graph
                .connect_ports(1, trigger_flow_out(&flow), 2, first_in)
        );
        assert!(
            flow.graph
                .connect_ports(1, trigger_flow_out(&flow), 3, second_in)
        );

        let lowered = validate_and_lower_flow_graph(&flow, &context).expect("valid lowered flow");
        assert_eq!(
            lowered.flow_successors_with_input_slots(1, 0),
            vec![(2, 0), (3, 0)]
        );
    }

    #[test]
    fn validate_diagnoses_unresolved_call_flow_reference() {
        let unresolved_id = Uuid::new_v4();
        let mut flow = AppFlow::new("flow".to_string(), FlowTrigger::Callable);
        let call = ActionNodeData::new(
            2,
            ActionNodeKind::CallFlow {
                flow_id: Some(unresolved_id),
            },
            Point::new(260.0, 120.0),
        );
        let call_in = call
            .cached_ports_in
            .iter()
            .find(|p| p.label == "flow_in")
            .expect("call flow_in")
            .id
            .0;
        flow.graph.nodes.push(call);
        flow.graph.z_order.push(2);
        flow.graph.next_id = 3;
        assert!(
            flow.graph
                .connect_ports(1, trigger_flow_out(&flow), 2, call_in)
        );

        let callable = HashSet::new();
        let context = SemanticValidationContext {
            callable_flow_ids: &callable,
            known_view_ids: None,
        };
        let err = validate_and_lower_flow_graph(&flow, &context).expect_err("invalid graph");
        assert!(err.iter().any(|d| {
            d.code == SemanticDiagnosticCode::UnresolvedReference && d.node_id == Some(2)
        }));
    }

    #[test]
    fn validate_diagnoses_retired_standalone_value_node() {
        let mut flow = AppFlow::new("flow".to_string(), FlowTrigger::Callable);
        let legacy_value = ActionNodeData::new(
            2,
            ActionNodeKind::StringLiteral {
                value: "orphan".to_string(),
            },
            Point::new(220.0, 220.0),
        );
        flow.graph.nodes.push(legacy_value);
        flow.graph.z_order.push(2);
        flow.graph.next_id = 3;

        let callable = HashSet::new();
        let context = SemanticValidationContext {
            callable_flow_ids: &callable,
            known_view_ids: None,
        };
        let err = validate_and_lower_flow_graph(&flow, &context).expect_err("invalid graph");
        assert!(err.iter().any(|d| {
            d.code == SemanticDiagnosticCode::UnsupportedNodeKind
                && d.node_id == Some(2)
                && d.message.contains("retired from the primary model")
        }));
    }

    #[test]
    fn validate_allows_legacy_value_node_when_used_as_expression_source() {
        let mut flow = AppFlow::new("flow".to_string(), FlowTrigger::Callable);
        let conditional =
            ActionNodeData::new(2, ActionNodeKind::Conditional, Point::new(320.0, 220.0));
        let legacy_bool = ActionNodeData::new(
            3,
            ActionNodeKind::BoolLiteral { value: true },
            Point::new(120.0, 220.0),
        );
        flow.graph
            .nodes
            .extend([conditional.clone(), legacy_bool.clone()]);
        flow.graph.z_order.extend([2, 3]);
        flow.graph.next_id = 4;
        assert!(flow.graph.connect_ports(
            1,
            trigger_flow_out(&flow),
            2,
            port_in(&conditional, "flow_in"),
        ));
        assert!(flow.graph.connect_ports(
            3,
            port_out(&legacy_bool, "value"),
            2,
            port_in(&conditional, "condition"),
        ));

        let callable = HashSet::new();
        let context = SemanticValidationContext {
            callable_flow_ids: &callable,
            known_view_ids: None,
        };
        validate_and_lower_flow_graph(&flow, &context).expect("valid lowered flow");
    }

    #[test]
    fn validate_diagnoses_legacy_state_and_call_nodes() {
        let mut flow = AppFlow::new("flow".to_string(), FlowTrigger::Callable);
        let legacy_set = ActionNodeData::new(
            2,
            ActionNodeKind::SetState {
                target: None,
                value_source: ValueSource::Literal(ActionValue::String(String::new())),
            },
            Point::new(260.0, 120.0),
        );
        let legacy_call = ActionNodeData::new(
            3,
            ActionNodeKind::CallAction {
                action_name: Some("old".to_string()),
            },
            Point::new(420.0, 120.0),
        );
        flow.graph
            .nodes
            .extend([legacy_set.clone(), legacy_call.clone()]);
        flow.graph.z_order.extend([2, 3]);
        flow.graph.next_id = 4;

        let callable = HashSet::new();
        let context = SemanticValidationContext {
            callable_flow_ids: &callable,
            known_view_ids: None,
        };
        let err = validate_and_lower_flow_graph(&flow, &context).expect_err("invalid graph");
        assert!(err.iter().any(|d| {
            d.code == SemanticDiagnosticCode::UnsupportedNodeKind
                && d.node_id == Some(2)
                && d.message.contains("StateMutation")
        }));
        assert!(err.iter().any(|d| {
            d.code == SemanticDiagnosticCode::UnsupportedNodeKind
                && d.node_id == Some(3)
                && d.message.contains("CallFlow")
        }));
    }

    #[test]
    fn validate_diagnoses_missing_from_port_assignment_input() {
        let view_id = Uuid::new_v4();
        let field_id = Uuid::new_v4();
        let mut flow = AppFlow::new("flow".to_string(), FlowTrigger::Callable);
        let node = ActionNodeData::new(
            2,
            ActionNodeKind::StateMutation {
                assignments: vec![crate::action_system::node_kinds::StateAssignment {
                    target: Some(custom_target(view_id, field_id, "value")),
                    value_source: ValueSource::FromPort,
                }],
            },
            Point::new(260.0, 120.0),
        );
        let flow_in = node
            .cached_ports_in
            .iter()
            .find(|p| p.label == "flow_in")
            .expect("flow_in")
            .id
            .0;
        flow.graph.nodes.push(node);
        flow.graph.z_order.push(2);
        flow.graph.next_id = 3;
        assert!(
            flow.graph
                .connect_ports(1, trigger_flow_out(&flow), 2, flow_in)
        );

        let callable = HashSet::new();
        let known_views = HashSet::from([view_id]);
        let context = SemanticValidationContext {
            callable_flow_ids: &callable,
            known_view_ids: Some(&known_views),
        };
        let err = validate_and_lower_flow_graph(&flow, &context).expect_err("invalid graph");
        assert!(err.iter().any(|d| {
            d.code == SemanticDiagnosticCode::MissingRequiredInput && d.node_id == Some(2)
        }));
    }

    #[test]
    fn lowering_preserves_state_mutation_assignments_and_call_flow_ids() {
        let view_id = Uuid::new_v4();
        let field_a = Uuid::new_v4();
        let field_b = Uuid::new_v4();
        let callee_id = Uuid::new_v4();
        let callable = HashSet::from([callee_id]);
        let known_views = HashSet::from([view_id]);
        let context = SemanticValidationContext {
            callable_flow_ids: &callable,
            known_view_ids: Some(&known_views),
        };

        let mut flow = AppFlow::new("flow".to_string(), FlowTrigger::Callable);
        let mutate = ActionNodeData::new(
            2,
            ActionNodeKind::StateMutation {
                assignments: vec![
                    crate::action_system::node_kinds::StateAssignment {
                        target: Some(custom_target(view_id, field_a, "first")),
                        value_source: ValueSource::Literal(ActionValue::String("a".to_string())),
                    },
                    crate::action_system::node_kinds::StateAssignment {
                        target: Some(custom_target(view_id, field_b, "second")),
                        value_source: ValueSource::Literal(ActionValue::String("b".to_string())),
                    },
                ],
            },
            Point::new(260.0, 120.0),
        );
        let call = ActionNodeData::new(
            3,
            ActionNodeKind::CallFlow {
                flow_id: Some(callee_id),
            },
            Point::new(380.0, 120.0),
        );
        let mutate_in = mutate
            .cached_ports_in
            .iter()
            .find(|p| p.label == "flow_in")
            .expect("mutate flow_in")
            .id
            .0;
        let mutate_out = mutate
            .cached_ports_out
            .iter()
            .find(|p| p.label == "flow_out")
            .expect("mutate flow_out")
            .id
            .0;
        let call_in = call
            .cached_ports_in
            .iter()
            .find(|p| p.label == "flow_in")
            .expect("call flow_in")
            .id
            .0;

        flow.graph.nodes.extend([mutate, call]);
        flow.graph.z_order.extend([2, 3]);
        flow.graph.next_id = 4;
        assert!(
            flow.graph
                .connect_ports(1, trigger_flow_out(&flow), 2, mutate_in)
        );
        assert!(flow.graph.connect_ports(2, mutate_out, 3, call_in));

        let lowered = validate_and_lower_flow_graph(&flow, &context).expect("valid lowered flow");
        let lowered_mutate = lowered.node(2).expect("mutate node");
        match &lowered_mutate.kind {
            ActionNodeKind::StateMutation { assignments } => {
                assert_eq!(assignments.len(), 2);
                assert_eq!(
                    assignments[0].value_source,
                    ValueSource::Literal(ActionValue::String("a".to_string()))
                );
                assert_eq!(
                    assignments[1].value_source,
                    ValueSource::Literal(ActionValue::String("b".to_string()))
                );
            }
            other => panic!("expected StateMutation, got {other:?}"),
        }

        let lowered_call = lowered.node(3).expect("call node");
        match &lowered_call.kind {
            ActionNodeKind::CallFlow { flow_id } => assert_eq!(*flow_id, Some(callee_id)),
            other => panic!("expected CallFlow, got {other:?}"),
        }
    }

    #[test]
    fn lowering_embeds_state_mutation_literal_expression() {
        let view_id = Uuid::new_v4();
        let field_id = Uuid::new_v4();
        let mut flow = AppFlow::new("flow".to_string(), FlowTrigger::Callable);
        let mutate = ActionNodeData::new(
            2,
            ActionNodeKind::StateMutation {
                assignments: vec![crate::action_system::node_kinds::StateAssignment {
                    target: Some(custom_target(view_id, field_id, "value")),
                    value_source: ValueSource::Literal(ActionValue::String("ok".to_string())),
                }],
            },
            Point::new(260.0, 120.0),
        );
        flow.graph.nodes.push(mutate.clone());
        flow.graph.z_order.push(2);
        flow.graph.next_id = 3;
        assert!(flow.graph.connect_ports(
            1,
            trigger_flow_out(&flow),
            2,
            port_in(&mutate, "flow_in")
        ));

        let callable = HashSet::new();
        let known_views = HashSet::from([view_id]);
        let context = SemanticValidationContext {
            callable_flow_ids: &callable,
            known_view_ids: Some(&known_views),
        };
        let lowered = validate_and_lower_flow_graph(&flow, &context).expect("lowered flow");
        let expr = lowered
            .expression_for_input_label(2, "value_0")
            .expect("assignment expression");
        assert_eq!(
            expr,
            &LoweredExpression::Literal(ActionValue::String("ok".to_string()))
        );
    }

    #[test]
    fn lowering_embeds_conditional_compare_logic_expression() {
        let mut flow = AppFlow::new("flow".to_string(), FlowTrigger::Callable);
        let lhs = ActionNodeData::new(
            2,
            ActionNodeKind::StringLiteral {
                value: "a".to_string(),
            },
            Point::new(60.0, 200.0),
        );
        let rhs = ActionNodeData::new(
            3,
            ActionNodeKind::StringLiteral {
                value: "a".to_string(),
            },
            Point::new(60.0, 260.0),
        );
        let compare = ActionNodeData::new(
            4,
            ActionNodeKind::Compare {
                operator: crate::action_system::node_kinds::CompareOp::Eq,
                rhs: crate::action_system::node_kinds::CompareRhs::FromPort,
                rhs_literal: ActionValue::String(String::new()),
            },
            Point::new(260.0, 220.0),
        );
        let not = ActionNodeData::new(5, ActionNodeKind::LogicNot, Point::new(420.0, 220.0));
        let conditional =
            ActionNodeData::new(6, ActionNodeKind::Conditional, Point::new(580.0, 220.0));

        flow.graph.nodes.extend([
            lhs.clone(),
            rhs.clone(),
            compare.clone(),
            not.clone(),
            conditional.clone(),
        ]);
        flow.graph.z_order.extend([2, 3, 4, 5, 6]);
        flow.graph.next_id = 7;

        assert!(flow.graph.connect_ports(
            1,
            trigger_flow_out(&flow),
            6,
            port_in(&conditional, "flow_in"),
        ));
        assert!(flow.graph.connect_ports(
            2,
            port_out(&lhs, "value"),
            4,
            port_in(&compare, "value")
        ));
        assert!(
            flow.graph
                .connect_ports(3, port_out(&rhs, "value"), 4, port_in(&compare, "rhs"))
        );
        assert!(flow.graph.connect_ports(
            4,
            port_out(&compare, "result"),
            5,
            port_in(&not, "value")
        ));
        assert!(flow.graph.connect_ports(
            5,
            port_out(&not, "result"),
            6,
            port_in(&conditional, "condition"),
        ));

        let callable = HashSet::new();
        let context = SemanticValidationContext {
            callable_flow_ids: &callable,
            known_view_ids: None,
        };
        let lowered = validate_and_lower_flow_graph(&flow, &context).expect("lowered flow");
        let expr = lowered
            .expression_for_input_label(6, "condition")
            .expect("condition expression");
        match expr {
            LoweredExpression::LogicNot { value } => match value.as_ref() {
                LoweredExpression::Compare { operator, .. } => {
                    assert_eq!(*operator, crate::action_system::node_kinds::CompareOp::Eq)
                }
                other => panic!("expected compare expression, got {other:?}"),
            },
            other => panic!("expected logic-not expression, got {other:?}"),
        }
    }

    #[test]
    fn lowering_embeds_authored_conditional_expression_without_input_edge() {
        let mut flow = AppFlow::new("flow".to_string(), FlowTrigger::Callable);
        let mut conditional =
            ActionNodeData::new(2, ActionNodeKind::Conditional, Point::new(320.0, 220.0));
        conditional.authored_conditions = vec![AuthoredCondition {
            lhs: AuthoredValueSource::Literal(ActionValue::String("admin@example.com".to_string())),
            operator: CompareOp::IsValidEmail,
            rhs_literal: ActionValue::String(String::new()),
        }];
        flow.graph.nodes.push(conditional.clone());
        flow.graph.z_order.push(2);
        flow.graph.next_id = 3;
        assert!(flow.graph.connect_ports(
            1,
            trigger_flow_out(&flow),
            2,
            port_in(&conditional, "flow_in"),
        ));

        let callable = HashSet::new();
        let context = SemanticValidationContext {
            callable_flow_ids: &callable,
            known_view_ids: None,
        };
        let lowered = validate_and_lower_flow_graph(&flow, &context).expect("lowered flow");
        let expr = lowered
            .expression_for_input_label(2, "condition")
            .expect("condition expression");
        assert_eq!(
            expr,
            &LoweredExpression::Compare {
                operator: CompareOp::IsValidEmail,
                lhs: Box::new(LoweredExpression::Literal(ActionValue::String(
                    "admin@example.com".to_string(),
                ))),
                rhs: None,
            }
        );
    }

    #[test]
    fn lowering_embeds_multi_authored_conditional_all_as_logic_and() {
        let mut flow = AppFlow::new("flow".to_string(), FlowTrigger::Callable);
        let mut conditional =
            ActionNodeData::new(2, ActionNodeKind::Conditional, Point::new(320.0, 220.0));
        conditional.authored_condition_join = ConditionJoinMode::All;
        conditional.authored_conditions = vec![
            AuthoredCondition {
                lhs: AuthoredValueSource::Literal(ActionValue::String("admin@example.com".into())),
                operator: CompareOp::IsValidEmail,
                rhs_literal: ActionValue::String(String::new()),
            },
            AuthoredCondition {
                lhs: AuthoredValueSource::Literal(ActionValue::String("admin@example.com".into())),
                operator: CompareOp::Contains,
                rhs_literal: ActionValue::String("@example.com".into()),
            },
        ];
        flow.graph.nodes.push(conditional.clone());
        flow.graph.z_order.push(2);
        flow.graph.next_id = 3;
        assert!(flow.graph.connect_ports(
            1,
            trigger_flow_out(&flow),
            2,
            port_in(&conditional, "flow_in"),
        ));

        let callable = HashSet::new();
        let context = SemanticValidationContext {
            callable_flow_ids: &callable,
            known_view_ids: None,
        };
        let lowered = validate_and_lower_flow_graph(&flow, &context).expect("lowered flow");
        let expr = lowered
            .expression_for_input_label(2, "condition")
            .expect("condition expression");
        match expr {
            LoweredExpression::LogicAnd { lhs, rhs } => {
                assert!(matches!(lhs.as_ref(), LoweredExpression::Compare { .. }));
                assert!(matches!(rhs.as_ref(), LoweredExpression::Compare { .. }));
            }
            other => panic!("expected logic-and expression, got {other:?}"),
        }
    }

    #[test]
    fn lowering_embeds_multi_authored_conditional_any_as_logic_or() {
        let mut flow = AppFlow::new("flow".to_string(), FlowTrigger::Callable);
        let mut conditional =
            ActionNodeData::new(2, ActionNodeKind::Conditional, Point::new(320.0, 220.0));
        conditional.authored_condition_join = ConditionJoinMode::Any;
        conditional.authored_conditions = vec![
            AuthoredCondition {
                lhs: AuthoredValueSource::Literal(ActionValue::String("admin@example.com".into())),
                operator: CompareOp::IsValidEmail,
                rhs_literal: ActionValue::String(String::new()),
            },
            AuthoredCondition {
                lhs: AuthoredValueSource::Literal(ActionValue::String("blocked".into())),
                operator: CompareOp::Eq,
                rhs_literal: ActionValue::String("allowed".into()),
            },
        ];
        flow.graph.nodes.push(conditional.clone());
        flow.graph.z_order.push(2);
        flow.graph.next_id = 3;
        assert!(flow.graph.connect_ports(
            1,
            trigger_flow_out(&flow),
            2,
            port_in(&conditional, "flow_in"),
        ));

        let callable = HashSet::new();
        let context = SemanticValidationContext {
            callable_flow_ids: &callable,
            known_view_ids: None,
        };
        let lowered = validate_and_lower_flow_graph(&flow, &context).expect("lowered flow");
        let expr = lowered
            .expression_for_input_label(2, "condition")
            .expect("condition expression");
        match expr {
            LoweredExpression::LogicOr { lhs, rhs } => {
                assert!(matches!(lhs.as_ref(), LoweredExpression::Compare { .. }));
                assert!(matches!(rhs.as_ref(), LoweredExpression::Compare { .. }));
            }
            other => panic!("expected logic-or expression, got {other:?}"),
        }
    }

    #[test]
    fn lowering_diagnoses_invalid_authored_conditional_operator_for_lhs_type() {
        let mut flow = AppFlow::new("flow".to_string(), FlowTrigger::Callable);
        let mut conditional =
            ActionNodeData::new(2, ActionNodeKind::Conditional, Point::new(320.0, 220.0));
        conditional.authored_conditions = vec![AuthoredCondition {
            lhs: AuthoredValueSource::Literal(ActionValue::Number(42.0)),
            operator: CompareOp::Contains,
            rhs_literal: ActionValue::String("4".to_string()),
        }];
        flow.graph.nodes.push(conditional.clone());
        flow.graph.z_order.push(2);
        flow.graph.next_id = 3;
        assert!(flow.graph.connect_ports(
            1,
            trigger_flow_out(&flow),
            2,
            port_in(&conditional, "flow_in"),
        ));

        let callable = HashSet::new();
        let context = SemanticValidationContext {
            callable_flow_ids: &callable,
            known_view_ids: None,
        };
        let err = validate_and_lower_flow_graph(&flow, &context).expect_err("invalid graph");
        assert!(err.iter().any(|d| {
            d.code == SemanticDiagnosticCode::InvalidNodeConfiguration
                && d.node_id == Some(2)
                && d.message.contains("operator")
        }));
    }

    #[test]
    fn lowering_diagnoses_non_boolean_conditional_expression() {
        let mut flow = AppFlow::new("flow".to_string(), FlowTrigger::Callable);
        let condition_src = ActionNodeData::new(
            2,
            ActionNodeKind::StringLiteral {
                value: "not_bool".to_string(),
            },
            Point::new(120.0, 220.0),
        );
        let conditional =
            ActionNodeData::new(3, ActionNodeKind::Conditional, Point::new(320.0, 220.0));

        flow.graph
            .nodes
            .extend([condition_src.clone(), conditional.clone()]);
        flow.graph.z_order.extend([2, 3]);
        flow.graph.next_id = 4;
        assert!(flow.graph.connect_ports(
            1,
            trigger_flow_out(&flow),
            3,
            port_in(&conditional, "flow_in"),
        ));
        assert!(flow.graph.connect_ports(
            2,
            port_out(&condition_src, "value"),
            3,
            port_in(&conditional, "condition"),
        ));

        let callable = HashSet::new();
        let context = SemanticValidationContext {
            callable_flow_ids: &callable,
            known_view_ids: None,
        };
        let err = validate_and_lower_flow_graph(&flow, &context).expect_err("invalid graph");
        assert!(err.iter().any(|d| {
            d.code == SemanticDiagnosticCode::InvalidNodeConfiguration && d.node_id == Some(3)
        }));
    }

    #[test]
    fn lowering_embeds_match_discriminant_expression() {
        let mut flow = AppFlow::new("flow".to_string(), FlowTrigger::Callable);
        let match_node = ActionNodeData::new(
            2,
            ActionNodeKind::Match {
                arms: vec!["A".to_string()],
                enum_type: None,
            },
            Point::new(280.0, 120.0),
        );
        let literal = ActionNodeData::new(
            3,
            ActionNodeKind::StringLiteral {
                value: "A".to_string(),
            },
            Point::new(120.0, 220.0),
        );

        flow.graph
            .nodes
            .extend([match_node.clone(), literal.clone()]);
        flow.graph.z_order.extend([2, 3]);
        flow.graph.next_id = 4;
        assert!(flow.graph.connect_ports(
            1,
            trigger_flow_out(&flow),
            2,
            port_in(&match_node, "flow_in"),
        ));
        assert!(flow.graph.connect_ports(
            3,
            port_out(&literal, "value"),
            2,
            port_in(&match_node, "value")
        ));

        let callable = HashSet::new();
        let context = SemanticValidationContext {
            callable_flow_ids: &callable,
            known_view_ids: None,
        };
        let lowered = validate_and_lower_flow_graph(&flow, &context).expect("lowered flow");
        let expr = lowered
            .expression_for_input_label(2, "value")
            .expect("match value expression");
        assert_eq!(
            expr,
            &LoweredExpression::Literal(ActionValue::String("A".to_string()))
        );
    }

    #[test]
    fn lowering_embeds_authored_match_subject_without_input_edge() {
        let mut flow = AppFlow::new("flow".to_string(), FlowTrigger::Callable);
        let mut match_node = ActionNodeData::new(
            2,
            ActionNodeKind::Match {
                arms: vec!["A".to_string()],
                enum_type: None,
            },
            Point::new(280.0, 120.0),
        );
        match_node.authored_match_subject = Some(AuthoredValueSource::Literal(
            ActionValue::String("A".to_string()),
        ));
        flow.graph.nodes.push(match_node.clone());
        flow.graph.z_order.push(2);
        flow.graph.next_id = 3;
        assert!(flow.graph.connect_ports(
            1,
            trigger_flow_out(&flow),
            2,
            port_in(&match_node, "flow_in"),
        ));

        let callable = HashSet::new();
        let context = SemanticValidationContext {
            callable_flow_ids: &callable,
            known_view_ids: None,
        };
        let lowered = validate_and_lower_flow_graph(&flow, &context).expect("lowered flow");
        let expr = lowered
            .expression_for_input_label(2, "value")
            .expect("match value expression");
        assert_eq!(
            expr,
            &LoweredExpression::Literal(ActionValue::String("A".to_string()))
        );
    }

    #[test]
    fn lowering_diagnoses_invalid_authored_match_enum_subject_type() {
        let mut flow = AppFlow::new("flow".to_string(), FlowTrigger::Callable);
        let mut match_node = ActionNodeData::new(
            2,
            ActionNodeKind::Match {
                arms: vec!["Admin".to_string()],
                enum_type: Some("Role".to_string()),
            },
            Point::new(280.0, 120.0),
        );
        match_node.authored_match_subject = Some(AuthoredValueSource::Literal(
            ActionValue::String("Admin".to_string()),
        ));
        flow.graph.nodes.push(match_node.clone());
        flow.graph.z_order.push(2);
        flow.graph.next_id = 3;
        assert!(flow.graph.connect_ports(
            1,
            trigger_flow_out(&flow),
            2,
            port_in(&match_node, "flow_in"),
        ));

        let callable = HashSet::new();
        let context = SemanticValidationContext {
            callable_flow_ids: &callable,
            known_view_ids: None,
        };
        let err = validate_and_lower_flow_graph(&flow, &context).expect_err("invalid graph");
        assert!(err.iter().any(|d| {
            d.code == SemanticDiagnosticCode::InvalidNodeConfiguration
                && d.node_id == Some(2)
                && d.message.contains("requires enum authored subject")
        }));
    }

    #[test]
    fn lowering_diagnoses_invalid_enum_literal_expression_source() {
        let mut flow = AppFlow::new("flow".to_string(), FlowTrigger::Callable);
        let match_node = ActionNodeData::new(
            2,
            ActionNodeKind::Match {
                arms: vec!["A".to_string()],
                enum_type: None,
            },
            Point::new(280.0, 120.0),
        );
        let enum_lit = ActionNodeData::new(
            3,
            ActionNodeKind::EnumLiteral {
                enum_name: None,
                variant: None,
            },
            Point::new(120.0, 220.0),
        );
        flow.graph
            .nodes
            .extend([match_node.clone(), enum_lit.clone()]);
        flow.graph.z_order.extend([2, 3]);
        flow.graph.next_id = 4;
        assert!(flow.graph.connect_ports(
            1,
            trigger_flow_out(&flow),
            2,
            port_in(&match_node, "flow_in"),
        ));
        assert!(flow.graph.connect_ports(
            3,
            port_out(&enum_lit, "value"),
            2,
            port_in(&match_node, "value")
        ));

        let callable = HashSet::new();
        let context = SemanticValidationContext {
            callable_flow_ids: &callable,
            known_view_ids: None,
        };
        let err = validate_and_lower_flow_graph(&flow, &context).expect_err("invalid graph");
        assert!(err.iter().any(|d| {
            d.code == SemanticDiagnosticCode::InvalidNodeConfiguration && d.node_id == Some(3)
        }));
    }

    #[test]
    fn validate_and_lower_accepts_canonical_view_reference_navigation_target() {
        let owner_view_id = Uuid::new_v4();
        let primary_view_id = Uuid::new_v4();
        let secondary_view_id = Uuid::new_v4();
        let (views, view_ref_id) = owner_views_with_single_view_reference(
            owner_view_id,
            primary_view_id,
            secondary_view_id,
        );
        let view_reference_index = build_view_reference_index(&views);

        let mut flow = AppFlow::new("flow".to_string(), FlowTrigger::Callable);
        let navigate = ActionNodeData::new(
            2,
            ActionNodeKind::NavigateToView {
                targets: vec![Some(NavigateTarget::ViewReference {
                    owner_view_id,
                    widget_id: view_ref_id,
                    target_view_id: secondary_view_id,
                })],
            },
            Point::new(260.0, 120.0),
        );
        flow.graph.nodes.push(navigate.clone());
        flow.graph.z_order.push(2);
        flow.graph.next_id = 3;
        assert!(flow.graph.connect_ports(
            1,
            trigger_flow_out(&flow),
            2,
            port_in(&navigate, "flow_in"),
        ));

        let callable = HashSet::new();
        let known_views = HashSet::from([owner_view_id, primary_view_id, secondary_view_id]);
        let context = SemanticValidationContext {
            callable_flow_ids: &callable,
            known_view_ids: Some(&known_views),
        };
        let lowered = validate_and_lower_flow_graph_with_view_refs(
            &flow,
            &context,
            Some(&view_reference_index),
        )
        .expect("valid lowered flow");
        assert_eq!(
            lowered.navigation_target_for_input_slot(2, 0),
            Some(&NavigateTarget::ViewReference {
                owner_view_id,
                widget_id: view_ref_id,
                target_view_id: secondary_view_id,
            })
        );
    }

    #[test]
    fn validate_diagnoses_view_reference_navigation_to_non_member_target() {
        let owner_view_id = Uuid::new_v4();
        let primary_view_id = Uuid::new_v4();
        let secondary_view_id = Uuid::new_v4();
        let non_member_target_view_id = Uuid::new_v4();
        let (mut views, view_ref_id) = owner_views_with_single_view_reference(
            owner_view_id,
            primary_view_id,
            secondary_view_id,
        );
        views.insert(
            non_member_target_view_id,
            AppView::with_id(non_member_target_view_id, "Other".to_string(), 3),
        );
        let view_reference_index = build_view_reference_index(&views);

        let mut flow = AppFlow::new("flow".to_string(), FlowTrigger::Callable);
        let navigate = ActionNodeData::new(
            2,
            ActionNodeKind::NavigateToView {
                targets: vec![Some(NavigateTarget::ViewReference {
                    owner_view_id,
                    widget_id: view_ref_id,
                    target_view_id: non_member_target_view_id,
                })],
            },
            Point::new(260.0, 120.0),
        );
        flow.graph.nodes.push(navigate.clone());
        flow.graph.z_order.push(2);
        flow.graph.next_id = 3;
        assert!(flow.graph.connect_ports(
            1,
            trigger_flow_out(&flow),
            2,
            port_in(&navigate, "flow_in"),
        ));

        let callable = HashSet::new();
        let known_views = HashSet::from([
            owner_view_id,
            primary_view_id,
            secondary_view_id,
            non_member_target_view_id,
        ]);
        let context = SemanticValidationContext {
            callable_flow_ids: &callable,
            known_view_ids: Some(&known_views),
        };
        let err = validate_and_lower_flow_graph_with_view_refs(
            &flow,
            &context,
            Some(&view_reference_index),
        )
        .expect_err("invalid graph");
        assert!(err.iter().any(|d| {
            d.code == SemanticDiagnosticCode::InvalidNodeConfiguration
                && d.message.contains("not configured on ViewReference")
        }));
    }

    #[test]
    fn validate_diagnoses_missing_view_reference_identity() {
        let owner_view_id = Uuid::new_v4();
        let primary_view_id = Uuid::new_v4();
        let secondary_view_id = Uuid::new_v4();
        let (views, _) = owner_views_with_single_view_reference(
            owner_view_id,
            primary_view_id,
            secondary_view_id,
        );
        let view_reference_index = build_view_reference_index(&views);
        let stale_widget_id = WidgetId(9999);

        let mut flow = AppFlow::new("flow".to_string(), FlowTrigger::Callable);
        let navigate = ActionNodeData::new(
            2,
            ActionNodeKind::NavigateToView {
                targets: vec![Some(NavigateTarget::ViewReference {
                    owner_view_id,
                    widget_id: stale_widget_id,
                    target_view_id: secondary_view_id,
                })],
            },
            Point::new(260.0, 120.0),
        );
        flow.graph.nodes.push(navigate.clone());
        flow.graph.z_order.push(2);
        flow.graph.next_id = 3;
        assert!(flow.graph.connect_ports(
            1,
            trigger_flow_out(&flow),
            2,
            port_in(&navigate, "flow_in"),
        ));

        let callable = HashSet::new();
        let known_views = HashSet::from([owner_view_id, primary_view_id, secondary_view_id]);
        let context = SemanticValidationContext {
            callable_flow_ids: &callable,
            known_view_ids: Some(&known_views),
        };
        let err = validate_and_lower_flow_graph_with_view_refs(
            &flow,
            &context,
            Some(&view_reference_index),
        )
        .expect_err("invalid graph");
        assert!(err.iter().any(|d| {
            d.code == SemanticDiagnosticCode::UnknownViewReference
                && d.message.contains("missing ViewReference")
        }));
    }
}

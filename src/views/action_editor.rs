use iced::mouse::Interaction;
use iced::widget::{
    Space, button, checkbox, column, container, mouse_area, pick_list, row, rule, scrollable, text,
    text_input, toggler,
};
use iced::{Alignment, Border, Color, Element, Length, Point, Task, Vector, border, keyboard};
use std::collections::BTreeMap;
use uuid::Uuid;

use widgets::flow_editor::{
    ConnectionPreview, ContextPalette, DragState, Edge, FlowEditor, FlowEditorAction, FlowNode,
    NodeContent, NodeId, PaletteEntry, PanState, PendingConnection, PortSide, SelectionRect,
};

use crate::action_system::events::actionable_events;
use crate::action_system::node_kinds::{
    ActionNodeKind, ActionValue, AuthoredCondition, AuthoredValueSource, CompareOp,
    ConditionJoinMode, NavigateTarget, TriggerPort, ValueSource,
};
use crate::action_system::state_ref::{ActionValueType, StateFieldRef, StateRefSource};
use crate::action_system::{
    ActionGraph, ActionNodeData, ActionNodeId, AppFlow, FlowTrigger, WidgetEventRow,
    action_node_from_palette_id, action_palette_entries,
};
use crate::data_structures::types::types::{AppView, Widget, WidgetId, WidgetType};
use crate::enum_builder::TypeSystem;

use crate::icon_lucide;
use crate::styles;

// ─── View option wrapper for NavigateToView pick_list ─────────────────────────

#[derive(Debug, Clone, PartialEq)]
struct ViewOption {
    pub id: Uuid,
    pub name: String,
}

impl std::fmt::Display for ViewOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NavigateModeOption {
    AppView,
    ViewReference,
}

impl std::fmt::Display for NavigateModeOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AppView => write!(f, "App View"),
            Self::ViewReference => write!(f, "View Reference"),
        }
    }
}

fn infer_navigate_mode(targets: &[Option<NavigateTarget>]) -> NavigateModeOption {
    targets
        .iter()
        .flatten()
        .find_map(|target| match target {
            NavigateTarget::ViewReference { .. } => Some(NavigateModeOption::ViewReference),
            NavigateTarget::AppView { .. } => Some(NavigateModeOption::AppView),
        })
        .unwrap_or(NavigateModeOption::AppView)
}

fn default_navigate_target_for_mode(
    mode: NavigateModeOption,
    all_views: &BTreeMap<Uuid, AppView>,
) -> Option<NavigateTarget> {
    match mode {
        NavigateModeOption::AppView => all_views
            .keys()
            .next()
            .copied()
            .map(|view_id| NavigateTarget::AppView { view_id }),
        NavigateModeOption::ViewReference => collect_view_reference_nav_options(all_views)
            .first()
            .and_then(|opt| {
                opt.targets
                    .first()
                    .map(|target| NavigateTarget::ViewReference {
                        owner_view_id: opt.owner_view_id,
                        widget_id: opt.widget_id,
                        target_view_id: target.id,
                    })
            }),
    }
}

#[derive(Debug, Clone, PartialEq)]
struct ViewReferenceNavOption {
    owner_view_id: Uuid,
    widget_id: WidgetId,
    label: String,
    targets: Vec<ViewOption>,
}

#[derive(Debug, Clone, PartialEq)]
struct ViewReferenceChoice {
    owner_view_id: Uuid,
    widget_id: WidgetId,
    label: String,
}

impl std::fmt::Display for ViewReferenceChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label)
    }
}

/// Widget option for WidgetEvent row target pick_list.
#[derive(Debug, Clone, PartialEq)]
struct WidgetOption {
    view_id: Uuid,
    widget_id: WidgetId,
    label: String,
}

impl std::fmt::Display for WidgetOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label)
    }
}

/// Rich widget info used for per-row event filtering.
#[derive(Clone)]
struct WidgetInfo {
    view_id: Uuid,
    widget_id: WidgetId,
    /// Pre-computed list of event names this widget supports (from actionable_events).
    supported_events: Vec<&'static str>,
    label: String,
}

#[derive(Debug, Clone, PartialEq)]
struct CallableFlowOption {
    id: Uuid,
    label: String,
}

impl std::fmt::Display for CallableFlowOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ExpressionSourceOption {
    TriggerInput {
        name: String,
        value_type: ActionValueType,
    },
    StateField(StateFieldRef),
    LiteralValue,
}

impl ExpressionSourceOption {
    fn to_source(&self, existing_literal: Option<ActionValue>) -> AuthoredValueSource {
        match self {
            Self::TriggerInput { name, value_type } => AuthoredValueSource::TriggerInput {
                name: name.clone(),
                value_type: value_type.clone(),
            },
            Self::StateField(field) => AuthoredValueSource::StateField(field.clone()),
            Self::LiteralValue => AuthoredValueSource::Literal(
                existing_literal.unwrap_or(ActionValue::String(String::new())),
            ),
        }
    }

    fn matches_source(&self, source: &AuthoredValueSource) -> bool {
        match (self, source) {
            (
                Self::TriggerInput {
                    name: a_name,
                    value_type: a_type,
                },
                AuthoredValueSource::TriggerInput {
                    name: b_name,
                    value_type: b_type,
                },
            ) => a_name == b_name && a_type == b_type,
            (Self::StateField(a), AuthoredValueSource::StateField(b)) => a == b,
            (Self::LiteralValue, AuthoredValueSource::Literal(_)) => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for ExpressionSourceOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TriggerInput { name, .. } => write!(f, "Trigger: {}", name),
            Self::StateField(field) => write!(f, "State: {}", field.display_name),
            Self::LiteralValue => write!(f, "Literal value"),
        }
    }
}

// ─── State ───────────────────────────────────────────────────────────────────

/// Interaction + cache state for the action editor.
/// Stored in `AdUiBuilder` and passed into every view/update call.
pub struct ActionEditorState {
    /// When Some, the action editor is editing this flow's graph.
    pub selected_flow_id: Option<Uuid>,
    /// Filter string for the flows browser.
    pub flow_search: String,
    // Interaction state passed to FlowEditor
    pub drag: Option<DragState>,
    pub pan: Option<PanState>,
    pub selection_rect: Option<SelectionRect>,
    pub preview_conn: Option<ConnectionPreview>,
    pub context_palette: Option<ContextPalette>,
    pub modifiers: keyboard::Modifiers,
    // Cached graph data (rebuilt after every mutation)
    pub cached_nodes: Vec<ActionNodeData>,
    pub cached_flow_nodes: Vec<FlowNode>,
    /// Like `cached_flow_nodes` but with Trigger node heights corrected for body content.
    pub corrected_flow_nodes: Vec<FlowNode>,
    pub cached_z_order: Vec<NodeId>,
    pub cached_flow_edges: Vec<Edge>,
    /// Static palette entries — stored here so FlowEditor can borrow them with 'a.
    pub palette: Vec<PaletteEntry>,
    /// Row being drag-reordered in WidgetEvent trigger: (flow_id, row_id).
    pub widget_event_row_dragging: Option<(Uuid, Uuid)>,
    /// Current row hovered as a drop target while reordering WidgetEvent rows.
    pub widget_event_row_drop_target: Option<(Uuid, Uuid)>,
}

impl Default for ActionEditorState {
    fn default() -> Self {
        Self {
            selected_flow_id: None,
            flow_search: String::new(),
            drag: None,
            pan: None,
            selection_rect: None,
            preview_conn: None,
            context_palette: None,
            modifiers: keyboard::Modifiers::default(),
            cached_nodes: Vec::new(),
            cached_flow_nodes: Vec::new(),
            corrected_flow_nodes: Vec::new(),
            cached_z_order: Vec::new(),
            cached_flow_edges: Vec::new(),
            palette: action_palette_entries(),
            widget_event_row_dragging: None,
            widget_event_row_drop_target: None,
        }
    }
}

impl ActionEditorState {
    /// Rebuilds the cached flow data from the given graph (or clears it if None).
    pub fn sync_cache(&mut self, graph: Option<&ActionGraph>) {
        if let Some(graph) = graph {
            self.cached_nodes = graph.nodes.clone();
            self.cached_flow_nodes = graph.flow_nodes();
            self.cached_z_order = graph.z_order.iter().map(|&id| NodeId(id)).collect();
            self.cached_flow_edges = graph.flow_edges();
        } else {
            self.cached_nodes.clear();
            self.cached_flow_nodes.clear();
            self.cached_z_order.clear();
            self.cached_flow_edges.clear();
        }
        // Reset corrected nodes to match (caller should call apply_trigger_heights after)
        self.corrected_flow_nodes = self.cached_flow_nodes.clone();
    }

    /// Updates `corrected_flow_nodes` with heights based on the active trigger's content.
    pub fn apply_trigger_heights(&mut self, trigger: &Option<(Uuid, FlowTrigger)>) {
        self.corrected_flow_nodes = self
            .cached_flow_nodes
            .iter()
            .cloned()
            .map(|mut fn_node| {
                if self.cached_nodes.iter().any(|n| {
                    n.id == fn_node.id.0 && matches!(n.kind, ActionNodeKind::Trigger { .. })
                }) {
                    fn_node.cached_height = trigger_node_body_height(trigger, 0);
                }
                fn_node
            })
            .collect();
    }
}

// ─── Message ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum Message {
    EditorAction(FlowEditorAction),
    // NavigateToView
    NavigateModeSelected(ActionNodeId, NavigateModeOption), // node_id, mode
    NavigateAppViewSelected(ActionNodeId, usize, Uuid),     // node_id, slot, view_id
    NavigateViewReferenceSelected(ActionNodeId, usize, Uuid, WidgetId), // node_id, slot, owner_view_id, widget_id
    NavigateViewReferenceTargetSelected(ActionNodeId, usize, Uuid), // node_id, slot, target_view_id
    AddNavigateTarget(ActionNodeId),
    RemoveNavigateTarget(ActionNodeId, usize),
    // StateMutation
    StateMutationAddAssignment(ActionNodeId),
    StateMutationRemoveAssignment(ActionNodeId, usize),
    StateMutationSetTarget(ActionNodeId, usize, StateFieldRef),
    StateMutationSetLiteralString(ActionNodeId, usize, String),
    StateMutationSetLiteralBool(ActionNodeId, usize, bool),
    StateMutationSetLiteralEnum(ActionNodeId, usize, String, String), // node_id, assignment_idx, type_name, variant
    // If authored conditions
    AddConditionalRow(ActionNodeId),
    RemoveConditionalRow(ActionNodeId, usize),
    SetConditionalJoinMode(ActionNodeId, ConditionJoinMode),
    SetConditionalSource(ActionNodeId, usize, ExpressionSourceOption),
    SetConditionalOperator(ActionNodeId, usize, CompareOp),
    SetConditionalRhsText(ActionNodeId, usize, String),
    SetConditionalRhsBool(ActionNodeId, usize, bool),
    // Match arms
    AddMatchArm(ActionNodeId),
    RemoveMatchArm(ActionNodeId, usize),
    SetMatchArm(ActionNodeId, usize, String),
    // Match enum type
    SetMatchEnumType(ActionNodeId, String),
    ClearMatchEnumType(ActionNodeId),
    // Match authored subject
    SetMatchSubjectSource(ActionNodeId, ExpressionSourceOption),
    SetMatchSubjectLiteral(ActionNodeId, String),
    // CallFlow
    SetCallFlowTarget(ActionNodeId, Uuid),
    // Flow management
    SelectFlow(Uuid),
    AddFlow,
    DeleteFlow(Uuid),
    SetFlowSearch(String),
    SetFlowName(Uuid, String),
    SetFlowTriggerKind(Uuid, &'static str),
    SetFlowTimerInterval(Uuid, String),
    SetFlowKeyComboKey(Uuid, String),
    SetFlowKeyComboMods(Uuid, bool, bool, bool),
    // WidgetEvent row management
    AddWidgetEventRow(Uuid),                            // flow_id
    RemoveWidgetEventRow(Uuid, Uuid),                   // flow_id, row_id
    SetRowEventType(Uuid, Uuid, String),                // flow_id, row_id, event_type
    SetRowTarget(Uuid, Uuid, Option<(Uuid, WidgetId)>), // flow_id, row_id, target
    // Row drag-to-reorder (hover-based)
    StartWidgetEventRowDrag(Uuid, Uuid), // flow_id, row_id
    HoverWidgetEventRow(Uuid, Uuid),     // flow_id, target_row_id
    EndWidgetEventRowDrag,
}

// ─── Update ──────────────────────────────────────────────────────────────────

pub fn update_with_type_system(
    all_views: &mut BTreeMap<Uuid, AppView>,
    flows: &mut Vec<AppFlow>,
    state: &mut ActionEditorState,
    msg: Message,
    type_system: &TypeSystem,
) -> Task<Message> {
    if let Message::SetMatchEnumType(node_id, ref enum_name) = msg {
        if let Some(graph) = get_active_graph_mut(flows, state.selected_flow_id) {
            if let Some(node) = graph.nodes.iter_mut().find(|n| n.id == node_id) {
                if let ActionNodeKind::Match { arms, enum_type } = &mut node.kind {
                    // Auto-populate arms from enum variants
                    if let Some(enum_def) = type_system.get_enum_by_name(enum_name) {
                        *arms = enum_def.variants.iter().map(|v| v.name.clone()).collect();
                    }
                    *enum_type = Some(enum_name.clone());
                }
                node.rebuild_ports();
            }
        }
        state.sync_cache(get_active_graph_ref(flows, state.selected_flow_id));
        let trigger_info = get_flow_trigger_info(flows, state.selected_flow_id);
        state.apply_trigger_heights(&trigger_info);
        return Task::none();
    }
    update(all_views, flows, state, msg)
}

pub fn update(
    all_views: &mut BTreeMap<Uuid, AppView>,
    flows: &mut Vec<AppFlow>,
    state: &mut ActionEditorState,
    msg: Message,
) -> Task<Message> {
    match msg {
        Message::EditorAction(action) => {
            let flow_id = state.selected_flow_id;
            if flow_id.is_none() {
                return Task::none();
            }
            handle_editor_action(flows, state, action, flow_id);
        }
        Message::NavigateModeSelected(node_id, mode) => {
            if let Some(graph) = get_active_graph_mut(flows, state.selected_flow_id) {
                if let Some(node) = graph.nodes.iter_mut().find(|n| n.id == node_id) {
                    if let ActionNodeKind::NavigateToView { targets } = &mut node.kind {
                        let default = default_navigate_target_for_mode(mode, all_views);
                        for target in targets.iter_mut() {
                            match (mode, target.as_ref()) {
                                (
                                    NavigateModeOption::AppView,
                                    Some(NavigateTarget::AppView { .. }),
                                )
                                | (
                                    NavigateModeOption::ViewReference,
                                    Some(NavigateTarget::ViewReference { .. }),
                                ) => {}
                                _ => *target = default.clone(),
                            }
                        }
                    }
                    node.rebuild_ports();
                }
            }
        }
        Message::NavigateAppViewSelected(node_id, slot, view_id) => {
            if let Some(graph) = get_active_graph_mut(flows, state.selected_flow_id) {
                if let Some(node) = graph.nodes.iter_mut().find(|n| n.id == node_id) {
                    if let ActionNodeKind::NavigateToView { targets } = &mut node.kind {
                        if let Some(t) = targets.get_mut(slot) {
                            *t = Some(NavigateTarget::AppView { view_id });
                        }
                    }
                    node.rebuild_ports();
                }
            }
        }
        Message::NavigateViewReferenceSelected(node_id, slot, owner_view_id, widget_id) => {
            let target_view_id = collect_view_reference_nav_options(all_views)
                .into_iter()
                .find(|opt| opt.owner_view_id == owner_view_id && opt.widget_id == widget_id)
                .and_then(|opt| opt.targets.first().map(|target| target.id));
            if let Some(target_view_id) = target_view_id {
                if let Some(graph) = get_active_graph_mut(flows, state.selected_flow_id) {
                    if let Some(node) = graph.nodes.iter_mut().find(|n| n.id == node_id) {
                        if let ActionNodeKind::NavigateToView { targets } = &mut node.kind {
                            if let Some(t) = targets.get_mut(slot) {
                                *t = Some(NavigateTarget::ViewReference {
                                    owner_view_id,
                                    widget_id,
                                    target_view_id,
                                });
                            }
                        }
                        node.rebuild_ports();
                    }
                }
            }
        }
        Message::NavigateViewReferenceTargetSelected(node_id, slot, target_view_id) => {
            if let Some(graph) = get_active_graph_mut(flows, state.selected_flow_id) {
                if let Some(node) = graph.nodes.iter_mut().find(|n| n.id == node_id) {
                    if let ActionNodeKind::NavigateToView { targets } = &mut node.kind {
                        let existing = targets.get(slot).and_then(|target| match target {
                            Some(NavigateTarget::ViewReference {
                                owner_view_id,
                                widget_id,
                                ..
                            }) => Some((*owner_view_id, *widget_id)),
                            _ => None,
                        });
                        if let Some((owner_view_id, widget_id)) = existing {
                            if let Some(t) = targets.get_mut(slot) {
                                *t = Some(NavigateTarget::ViewReference {
                                    owner_view_id,
                                    widget_id,
                                    target_view_id,
                                });
                            }
                        }
                    }
                    node.rebuild_ports();
                }
            }
        }
        Message::AddNavigateTarget(node_id) => {
            if let Some(graph) = get_active_graph_mut(flows, state.selected_flow_id) {
                if let Some(node) = graph.nodes.iter_mut().find(|n| n.id == node_id) {
                    if let ActionNodeKind::NavigateToView { targets } = &mut node.kind {
                        let mode = infer_navigate_mode(targets);
                        targets.push(default_navigate_target_for_mode(mode, all_views));
                    }
                    node.rebuild_ports();
                }
            }
        }
        Message::RemoveNavigateTarget(node_id, slot) => {
            if let Some(graph) = get_active_graph_mut(flows, state.selected_flow_id) {
                if let Some(node) = graph.nodes.iter_mut().find(|n| n.id == node_id) {
                    if let ActionNodeKind::NavigateToView { targets } = &mut node.kind {
                        if targets.len() > 1 {
                            targets.remove(slot);
                        }
                    }
                    node.rebuild_ports();
                }
            }
        }
        Message::StateMutationAddAssignment(node_id) => {
            if let Some(graph) = get_active_graph_mut(flows, state.selected_flow_id) {
                if let Some(node) = graph.nodes.iter_mut().find(|n| n.id == node_id) {
                    if let ActionNodeKind::StateMutation { assignments } = &mut node.kind {
                        assignments.push(crate::action_system::node_kinds::StateAssignment {
                            target: None,
                            value_source: ValueSource::Literal(ActionValue::String(String::new())),
                        });
                    }
                    node.rebuild_ports();
                }
            }
        }
        Message::StateMutationRemoveAssignment(node_id, idx) => {
            if let Some(graph) = get_active_graph_mut(flows, state.selected_flow_id) {
                if let Some(node) = graph.nodes.iter_mut().find(|n| n.id == node_id) {
                    if let ActionNodeKind::StateMutation { assignments } = &mut node.kind {
                        if assignments.len() > 1 && idx < assignments.len() {
                            assignments.remove(idx);
                        }
                    }
                    node.rebuild_ports();
                }
            }
        }
        Message::StateMutationSetTarget(node_id, idx, sf) => {
            if let Some(graph) = get_active_graph_mut(flows, state.selected_flow_id) {
                if let Some(node) = graph.nodes.iter_mut().find(|n| n.id == node_id) {
                    if let ActionNodeKind::StateMutation { assignments } = &mut node.kind {
                        if let Some(assignment) = assignments.get_mut(idx) {
                            assignment.target = Some(sf.clone());
                            assignment.value_source =
                                ValueSource::Literal(default_value_for_type(&sf.field_type));
                        }
                    }
                }
            }
        }
        Message::StateMutationSetLiteralString(node_id, idx, raw) => {
            if let Some(graph) = get_active_graph_mut(flows, state.selected_flow_id) {
                if let Some(node) = graph.nodes.iter_mut().find(|n| n.id == node_id) {
                    if let ActionNodeKind::StateMutation { assignments } = &mut node.kind {
                        if let Some(assignment) = assignments.get_mut(idx) {
                            if let ValueSource::Literal(current) = &assignment.value_source {
                                assignment.value_source =
                                    ValueSource::Literal(coerce_literal(current.clone(), &raw));
                            }
                        }
                    }
                }
            }
        }
        Message::StateMutationSetLiteralBool(node_id, idx, b) => {
            if let Some(graph) = get_active_graph_mut(flows, state.selected_flow_id) {
                if let Some(node) = graph.nodes.iter_mut().find(|n| n.id == node_id) {
                    if let ActionNodeKind::StateMutation { assignments } = &mut node.kind {
                        if let Some(assignment) = assignments.get_mut(idx) {
                            assignment.value_source = ValueSource::Literal(ActionValue::Bool(b));
                        }
                    }
                }
            }
        }
        Message::StateMutationSetLiteralEnum(node_id, idx, type_name, variant) => {
            if let Some(graph) = get_active_graph_mut(flows, state.selected_flow_id) {
                if let Some(node) = graph.nodes.iter_mut().find(|n| n.id == node_id) {
                    if let ActionNodeKind::StateMutation { assignments } = &mut node.kind {
                        if let Some(assignment) = assignments.get_mut(idx) {
                            assignment.value_source =
                                ValueSource::Literal(ActionValue::EnumVariant {
                                    type_name,
                                    variant,
                                });
                        }
                    }
                }
            }
        }
        Message::AddConditionalRow(node_id) => {
            if let Some(graph) = get_active_graph_mut(flows, state.selected_flow_id) {
                if let Some(node) = graph.nodes.iter_mut().find(|n| n.id == node_id) {
                    let conditions = ensure_authored_conditions_mut(node);
                    conditions.push(default_authored_condition());
                    node.rebuild_ports();
                }
            }
        }
        Message::RemoveConditionalRow(node_id, idx) => {
            if let Some(graph) = get_active_graph_mut(flows, state.selected_flow_id) {
                if let Some(node) = graph.nodes.iter_mut().find(|n| n.id == node_id) {
                    let conditions = ensure_authored_conditions_mut(node);
                    if conditions.len() > 1 && idx < conditions.len() {
                        conditions.remove(idx);
                    }
                    node.rebuild_ports();
                }
            }
        }
        Message::SetConditionalJoinMode(node_id, join_mode) => {
            if let Some(graph) = get_active_graph_mut(flows, state.selected_flow_id) {
                if let Some(node) = graph.nodes.iter_mut().find(|n| n.id == node_id) {
                    node.authored_condition_join = join_mode;
                }
            }
        }
        Message::SetConditionalSource(node_id, idx, source_option) => {
            if let Some(graph) = get_active_graph_mut(flows, state.selected_flow_id) {
                if let Some(node) = graph.nodes.iter_mut().find(|n| n.id == node_id) {
                    let conditions = ensure_authored_conditions_mut(node);
                    if let Some(condition) = conditions.get_mut(idx) {
                        let existing_literal = match &condition.lhs {
                            AuthoredValueSource::Literal(v) => Some(v.clone()),
                            _ => None,
                        };
                        condition.lhs = source_option.to_source(existing_literal);
                        normalize_authored_condition_rhs(condition);
                    }
                }
            }
        }
        Message::SetConditionalOperator(node_id, idx, operator) => {
            if let Some(graph) = get_active_graph_mut(flows, state.selected_flow_id) {
                if let Some(node) = graph.nodes.iter_mut().find(|n| n.id == node_id) {
                    let conditions = ensure_authored_conditions_mut(node);
                    if let Some(condition) = conditions.get_mut(idx) {
                        condition.operator = operator;
                        normalize_authored_condition_rhs(condition);
                    }
                }
            }
        }
        Message::SetConditionalRhsText(node_id, idx, raw) => {
            if let Some(graph) = get_active_graph_mut(flows, state.selected_flow_id) {
                if let Some(node) = graph.nodes.iter_mut().find(|n| n.id == node_id) {
                    let conditions = ensure_authored_conditions_mut(node);
                    if let Some(condition) = conditions.get_mut(idx) {
                        condition.rhs_literal =
                            parse_authored_rhs_text(&condition.lhs.value_type(), raw.as_str());
                    }
                }
            }
        }
        Message::SetConditionalRhsBool(node_id, idx, value) => {
            if let Some(graph) = get_active_graph_mut(flows, state.selected_flow_id) {
                if let Some(node) = graph.nodes.iter_mut().find(|n| n.id == node_id) {
                    let conditions = ensure_authored_conditions_mut(node);
                    if let Some(condition) = conditions.get_mut(idx) {
                        condition.rhs_literal = ActionValue::Bool(value);
                    }
                }
            }
        }
        Message::AddMatchArm(node_id) => {
            if let Some(graph) = get_active_graph_mut(flows, state.selected_flow_id) {
                if let Some(node) = graph.nodes.iter_mut().find(|n| n.id == node_id) {
                    if let ActionNodeKind::Match { arms, .. } = &mut node.kind {
                        let n = arms.len() + 1;
                        arms.push(format!("arm {n}"));
                    }
                    node.rebuild_ports();
                }
            }
        }
        Message::RemoveMatchArm(node_id, idx) => {
            if let Some(graph) = get_active_graph_mut(flows, state.selected_flow_id) {
                if let Some(node) = graph.nodes.iter_mut().find(|n| n.id == node_id) {
                    if let ActionNodeKind::Match { arms, .. } = &mut node.kind {
                        if idx < arms.len() {
                            arms.remove(idx);
                        }
                    }
                    node.rebuild_ports();
                }
            }
        }
        Message::SetMatchArm(node_id, idx, value) => {
            if let Some(graph) = get_active_graph_mut(flows, state.selected_flow_id) {
                if let Some(node) = graph.nodes.iter_mut().find(|n| n.id == node_id) {
                    if let ActionNodeKind::Match { arms, .. } = &mut node.kind {
                        if idx < arms.len() {
                            arms[idx] = value;
                        }
                    }
                    node.rebuild_ports();
                }
            }
        }
        Message::SetMatchEnumType(node_id, enum_name) => {
            if let Some(graph) = get_active_graph_mut(flows, state.selected_flow_id) {
                if let Some(node) = graph.nodes.iter_mut().find(|n| n.id == node_id) {
                    if let ActionNodeKind::Match { arms, enum_type } = &mut node.kind {
                        *enum_type = Some(enum_name);
                        let variant = arms.first().cloned().unwrap_or_default();
                        let type_name = enum_type.clone().unwrap_or_default();
                        match &mut node.authored_match_subject {
                            Some(AuthoredValueSource::Literal(value)) => {
                                *value = ActionValue::EnumVariant { type_name, variant };
                            }
                            Some(_) => {}
                            None => {
                                node.authored_match_subject =
                                    Some(AuthoredValueSource::Literal(ActionValue::EnumVariant {
                                        type_name,
                                        variant,
                                    }));
                            }
                        }
                    }
                    node.rebuild_ports();
                }
            }
        }
        Message::ClearMatchEnumType(node_id) => {
            if let Some(graph) = get_active_graph_mut(flows, state.selected_flow_id) {
                if let Some(node) = graph.nodes.iter_mut().find(|n| n.id == node_id) {
                    if let ActionNodeKind::Match { enum_type, .. } = &mut node.kind {
                        *enum_type = None;
                    }
                }
            }
        }
        Message::SetMatchSubjectSource(node_id, source_option) => {
            if let Some(graph) = get_active_graph_mut(flows, state.selected_flow_id) {
                if let Some(node) = graph.nodes.iter_mut().find(|n| n.id == node_id) {
                    let existing_literal = match &node.authored_match_subject {
                        Some(AuthoredValueSource::Literal(v)) => Some(v.clone()),
                        _ => None,
                    };
                    let fallback_literal = existing_literal.or_else(|| {
                        if matches!(source_option, ExpressionSourceOption::LiteralValue) {
                            match &node.kind {
                                ActionNodeKind::Match {
                                    arms,
                                    enum_type: Some(type_name),
                                } => Some(ActionValue::EnumVariant {
                                    type_name: type_name.clone(),
                                    variant: arms.first().cloned().unwrap_or_default(),
                                }),
                                _ => None,
                            }
                        } else {
                            None
                        }
                    });
                    node.authored_match_subject = Some(source_option.to_source(fallback_literal));
                    node.rebuild_ports();
                }
            }
        }
        Message::SetMatchSubjectLiteral(node_id, raw) => {
            if let Some(graph) = get_active_graph_mut(flows, state.selected_flow_id) {
                if let Some(node) = graph.nodes.iter_mut().find(|n| n.id == node_id) {
                    let enum_type = match &node.kind {
                        ActionNodeKind::Match { enum_type, .. } => enum_type.clone(),
                        _ => None,
                    };
                    if let Some(AuthoredValueSource::Literal(value)) =
                        &mut node.authored_match_subject
                    {
                        if let Some(type_name) = enum_type {
                            *value = ActionValue::EnumVariant {
                                type_name,
                                variant: raw,
                            };
                        } else {
                            *value = match value {
                                ActionValue::Number(_) => raw
                                    .trim()
                                    .parse::<f64>()
                                    .map(ActionValue::Number)
                                    .unwrap_or_else(|_| ActionValue::String(raw)),
                                ActionValue::Bool(_) => {
                                    match raw.trim().to_ascii_lowercase().as_str() {
                                        "true" => ActionValue::Bool(true),
                                        "false" => ActionValue::Bool(false),
                                        _ => ActionValue::String(raw),
                                    }
                                }
                                ActionValue::EnumVariant { type_name, .. } => {
                                    ActionValue::EnumVariant {
                                        type_name: type_name.clone(),
                                        variant: raw,
                                    }
                                }
                                ActionValue::String(_) => ActionValue::String(raw),
                            };
                        }
                    }
                }
            }
        }
        Message::SetCallFlowTarget(node_id, flow_id) => {
            if let Some(graph) = get_active_graph_mut(flows, state.selected_flow_id) {
                if let Some(node) = graph.nodes.iter_mut().find(|n| n.id == node_id) {
                    if let ActionNodeKind::CallFlow { flow_id: target } = &mut node.kind {
                        *target = Some(flow_id);
                    }
                }
            }
        }

        // ── Flow management ────────────────────────────────────────────────────
        Message::SelectFlow(flow_id) => {
            if flows.iter().any(|f| f.id == flow_id) {
                state.selected_flow_id = Some(flow_id);
            }
            clear_interaction(state);
        }
        Message::AddFlow => {
            let n = flows.len() + 1;
            let flow = AppFlow::new(format!("flow_{n}"), FlowTrigger::Callable);
            let flow_id = flow.id;
            flows.push(flow);
            state.selected_flow_id = Some(flow_id);
            clear_interaction(state);
        }
        Message::DeleteFlow(flow_id) => {
            flows.retain(|f| f.id != flow_id);
            if state.selected_flow_id == Some(flow_id) {
                state.selected_flow_id = None;
            }
        }
        Message::SetFlowSearch(s) => {
            state.flow_search = s;
        }
        Message::SetFlowName(flow_id, name) => {
            if let Some(flow) = flows.iter_mut().find(|f| f.id == flow_id) {
                flow.name = name;
            }
        }
        Message::SetFlowTriggerKind(flow_id, kind_label) => {
            if let Some(flow) = flows.iter_mut().find(|f| f.id == flow_id) {
                let new_trigger = FlowTrigger::all_kinds()
                    .into_iter()
                    .find(|t| t.kind_label() == kind_label)
                    .unwrap_or(FlowTrigger::Callable);
                flow.trigger = new_trigger;
                flow.sync_trigger_topology();
            }
        }
        Message::SetFlowTimerInterval(flow_id, raw) => {
            if let Some(flow) = flows.iter_mut().find(|f| f.id == flow_id) {
                if let FlowTrigger::Timer { interval_ms } = &mut flow.trigger {
                    if let Ok(v) = raw.parse::<u64>() {
                        *interval_ms = v;
                    }
                }
            }
        }
        Message::SetFlowKeyComboKey(flow_id, key) => {
            if let Some(flow) = flows.iter_mut().find(|f| f.id == flow_id) {
                if let FlowTrigger::KeyCombo { key: k, .. } = &mut flow.trigger {
                    *k = key;
                }
            }
        }
        Message::SetFlowKeyComboMods(flow_id, ctrl, shift, alt) => {
            if let Some(flow) = flows.iter_mut().find(|f| f.id == flow_id) {
                if let FlowTrigger::KeyCombo {
                    ctrl: c,
                    shift: sh,
                    alt: a,
                    ..
                } = &mut flow.trigger
                {
                    *c = ctrl;
                    *sh = shift;
                    *a = alt;
                }
            }
        }
        Message::AddWidgetEventRow(flow_id) => {
            if let Some(flow) = find_flow_mut(flows, flow_id) {
                if let FlowTrigger::WidgetEvent { rows } = &mut flow.trigger {
                    rows.push(WidgetEventRow::new());
                }
            }
        }
        Message::RemoveWidgetEventRow(flow_id, row_id) => {
            if let Some(flow) = find_flow_mut(flows, flow_id) {
                if let FlowTrigger::WidgetEvent { rows } = &mut flow.trigger {
                    rows.retain(|r| r.id != row_id);
                }
            }
            if state.widget_event_row_dragging == Some((flow_id, row_id)) {
                state.widget_event_row_dragging = None;
                state.widget_event_row_drop_target = None;
            }
        }
        Message::SetRowEventType(flow_id, row_id, event_type) => {
            if let Some(flow) = find_flow_mut(flows, flow_id) {
                if let FlowTrigger::WidgetEvent { rows } = &mut flow.trigger {
                    if let Some(row) = rows.iter_mut().find(|r| r.id == row_id) {
                        let event_changed = row.event_type != event_type;
                        row.event_type = event_type.clone();
                        if event_changed {
                            if let Some((view_id, widget_raw)) = row.target {
                                let widget_id = WidgetId(widget_raw);
                                if !widget_supports_event(
                                    all_views,
                                    view_id,
                                    widget_id,
                                    &event_type,
                                ) {
                                    row.target = None;
                                }
                            }
                        }
                    }
                }
            }
        }
        Message::SetRowTarget(flow_id, row_id, target) => {
            if let Some(flow) = find_flow_mut(flows, flow_id) {
                if let FlowTrigger::WidgetEvent { rows } = &mut flow.trigger {
                    if let Some(row) = rows.iter_mut().find(|r| r.id == row_id) {
                        row.target = target.map(|(vid, wid)| (vid, wid.0));
                    }
                }
            }
        }
        Message::StartWidgetEventRowDrag(flow_id, row_id) => {
            state.widget_event_row_dragging = Some((flow_id, row_id));
            state.widget_event_row_drop_target = Some((flow_id, row_id));
        }
        Message::HoverWidgetEventRow(flow_id, target_row_id) => {
            if let Some((drag_fid, drag_rid)) = state.widget_event_row_dragging {
                if drag_fid == flow_id {
                    state.widget_event_row_drop_target = Some((flow_id, target_row_id));
                    if let Some(flow) = find_flow_mut(flows, flow_id) {
                        if let FlowTrigger::WidgetEvent { rows } = &mut flow.trigger {
                            move_widget_event_row(rows, drag_rid, target_row_id);
                        }
                    }
                }
            }
        }
        Message::EndWidgetEventRowDrag => {
            state.widget_event_row_dragging = None;
            state.widget_event_row_drop_target = None;
        }
    }

    sync_selected_flow_trigger_topology(flows, state.selected_flow_id);

    // Always sync the cache so view() sees the latest data.
    state.sync_cache(get_active_graph_ref(flows, state.selected_flow_id));
    // Update corrected heights for Trigger nodes.
    let trigger_info = get_flow_trigger_info(flows, state.selected_flow_id);
    state.apply_trigger_heights(&trigger_info);
    Task::none()
}

// ─── View ────────────────────────────────────────────────────────────────────

pub fn view<'a>(
    all_views: &'a BTreeMap<Uuid, AppView>,
    flows: &'a Vec<AppFlow>,
    state: &'a ActionEditorState,
    type_system: &'a TypeSystem,
) -> Element<'a, Message> {
    // Collect state fields once for all node body pickers
    let state_fields = collect_state_fields(all_views);

    // ─── Left panel: flows browser ────────────────────────────────────────
    // Search + Add row (no per-view picker — all flows shown in one flat list)
    let search_add: Element<'a, Message> = row![
        text_input("Search flows...", &state.flow_search)
            .on_input(Message::SetFlowSearch)
            .size(12)
            .width(Length::Fill),
        button(icon_lucide::plus().size(12).center())
            .style(button::text)
            .on_press(Message::AddFlow),
    ]
    .spacing(4)
    .into();

    // Flow list rows — all flows in flat list, filterable by search
    let flow_rows: Vec<Element<'a, Message>> = {
        let search = state.flow_search.to_lowercase();
        let mut rows: Vec<Element<'a, Message>> = Vec::new();
        for flow in flows {
            if !search.is_empty() && !flow.name.to_lowercase().contains(&search) {
                continue;
            }
            let is_sel = state.selected_flow_id == Some(flow.id);
            let fid = flow.id;
            let fid2 = flow.id;
            let kind = flow.trigger.kind_label();
            let name = flow.name.clone();
            let row_elem: Element<'a, Message> = row![
                column![
                    // Inline-editable flow name
                    text_input("Flow name...", &name)
                        .on_input(move |n| Message::SetFlowName(fid, n))
                        .size(12)
                        .style(if is_sel {
                            text_input::default
                        } else {
                            text_input::default
                        })
                        .width(Length::Fill),
                    text(kind).size(10),
                ]
                .spacing(2)
                .width(Length::Fill),
                button(text("→").size(10))
                    .style(if is_sel {
                        button::primary
                    } else {
                        button::text
                    })
                    .on_press(Message::SelectFlow(fid)),
                button(icon_lucide::trash_2().size(10.0))
                    .style(styles::button::cancel)
                    .on_press(Message::DeleteFlow(fid2)),
            ]
            .spacing(2)
            .align_y(iced::Alignment::Center)
            .into();
            rows.push(row_elem);
        }
        rows
    };

    let left_panel: Element<'a, Message> = container(
        column![
            text("Flows").size(13),
            search_add,
            scrollable(column(flow_rows).spacing(2)).height(Length::Fill),
        ]
        .spacing(6),
    )
    .width(Length::Fixed(220.0))
    .height(Length::Fill)
    .padding(8)
    .clip(true)
    .style(|theme: &iced::Theme| {
        let background = theme
            .extended_palette()
            .background
            .weakest
            .color
            .scale_alpha(0.5);

        iced::widget::container::Style {
            background: Some(iced::Background::Color(background)),
            ..iced::widget::container::Style::default()
        }
    })
    .into();

    // ─── Center: FlowEditor ──────────────────────────────────────────────
    let has_active = state.selected_flow_id.is_some();
    let center: Element<'a, Message> = if has_active && !state.cached_flow_nodes.is_empty() {
        let viewport = get_current_viewport(flows, state.selected_flow_id);

        // Build view options once for NavigateToView pickers
        let view_options: Vec<ViewOption> = all_views
            .iter()
            .map(|(id, v)| ViewOption {
                id: *id,
                name: v.name.clone(),
            })
            .collect();
        let view_reference_nav_options = collect_view_reference_nav_options(all_views);

        // Build one body element per node in z_order
        // view_options, state_fields, and flow option lists are cloned per node (small vecs)
        let enum_names: Vec<String> = type_system.enum_names();
        let callable_flow_options: Vec<CallableFlowOption> = flows
            .iter()
            .filter(|f| f.enabled && matches!(f.trigger, FlowTrigger::Callable))
            .map(|f| CallableFlowOption {
                id: f.id,
                label: f.name.clone(),
            })
            .collect();

        // For flow graphs, get the current flow's trigger for the Trigger node body
        let flow_trigger_info: Option<(Uuid, FlowTrigger)> =
            get_flow_trigger_info(flows, state.selected_flow_id);

        // Collect all widgets with type/properties info for per-row event filtering.
        let all_widget_infos: Vec<WidgetInfo> = collect_all_widget_infos(all_views);
        let trigger_output_ports: Vec<TriggerPort> =
            get_active_graph_ref(flows, state.selected_flow_id)
                .and_then(|graph| {
                    graph
                        .nodes
                        .iter()
                        .find(|n| n.is_trigger())
                        .and_then(|trigger| match &trigger.kind {
                            ActionNodeKind::Trigger { output_ports, .. } => {
                                Some(output_ports.clone())
                            }
                            _ => None,
                        })
                })
                .unwrap_or_default();

        let row_dragging = state.widget_event_row_dragging;
        let row_drag_target = state.widget_event_row_drop_target;
        let content: Vec<NodeContent<'a, Message>> = state
            .cached_z_order
            .iter()
            .filter_map(|nid| state.cached_nodes.iter().find(|n| n.id == nid.0))
            .map(|node| {
                let body = build_node_body(
                    node,
                    view_options.clone(),
                    view_reference_nav_options.clone(),
                    state_fields.clone(),
                    enum_names.clone(),
                    callable_flow_options.clone(),
                    type_system,
                    flow_trigger_info.clone(),
                    all_widget_infos.clone(),
                    trigger_output_ports.clone(),
                    row_dragging,
                    row_drag_target,
                );
                let mut content = NodeContent::body(body);
                if let Some(header) =
                    build_node_header(node, enum_names.clone(), flow_trigger_info.clone())
                {
                    content = content.with_header(header);
                }
                content
            })
            .collect();

        FlowEditor::new(
            &state.corrected_flow_nodes,
            &state.cached_z_order,
            &state.cached_flow_edges,
            viewport,
            state.drag,
            state.pan,
            state.selection_rect,
            state.preview_conn,
            state.context_palette,
            state.modifiers,
            &state.palette,
            content,
        )
        .style(flow_editor_style_tokens)
        .on_action(Message::EditorAction)
        .into()
    } else if has_active {
        container(text("Loading..."))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    } else {
        container(
            text("Select a widget event or callable flow on the left to open the action editor")
                .size(14),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
    };

    let center: Element<'a, Message> = container(center)
        .width(Length::Fill)
        .height(Length::Fill)
        .clip(true)
        .into();

    row![
        left_panel,
        rule::vertical(1).style(styles::rule::rule_strong),
        center
    ]
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

// ─── Editor action handler ────────────────────────────────────────────────────

fn handle_editor_action(
    flows: &mut Vec<AppFlow>,
    state: &mut ActionEditorState,
    action: FlowEditorAction,
    flow_id: Option<Uuid>,
) {
    match action {
        FlowEditorAction::UpdateModifiers { modifiers } => {
            state.modifiers = modifiers;
        }

        FlowEditorAction::SelectSingle(node_id) => {
            if let Some(graph) = get_active_graph_mut(flows, flow_id) {
                let additive = state.modifiers.shift();
                for node in &mut graph.nodes {
                    if additive {
                        if node.id == node_id.0 {
                            node.selected = true;
                        }
                    } else {
                        node.selected = node.id == node_id.0;
                    }
                }
            }
        }

        FlowEditorAction::ClearSelection => {
            if let Some(graph) = get_active_graph_mut(flows, flow_id) {
                for node in &mut graph.nodes {
                    node.selected = false;
                }
            }
        }

        FlowEditorAction::StartNodeDrag { id, cursor_scene } => {
            if let Some(graph) = get_active_graph_ref(flows, flow_id) {
                let flow_nodes = graph.flow_nodes();
                state.drag = Some(DragState::from_nodes(id, cursor_scene, &flow_nodes));
            }
        }

        FlowEditorAction::DragNodeTo { cursor_scene } => {
            if let Some(drag) = state.drag {
                let delta_x = cursor_scene.x - drag.cursor_start_scene.x;
                let delta_y = cursor_scene.y - drag.cursor_start_scene.y;
                let origins = drag.selected_origins[..drag.selected_count].to_vec();
                if let Some(graph) = get_active_graph_mut(flows, flow_id) {
                    for (nid, origin) in &origins {
                        if let Some(node) = graph.nodes.iter_mut().find(|n| n.id == nid.0) {
                            node.position = Point::new(origin.x + delta_x, origin.y + delta_y);
                        }
                    }
                }
            }
        }

        FlowEditorAction::EndNodeDrag => {
            state.drag = None;
        }

        FlowEditorAction::StartPan { cursor_screen } => {
            if let Some(graph) = get_active_graph_ref(flows, flow_id) {
                state.pan = Some(PanState {
                    cursor_start_screen: cursor_screen,
                    pan_start: Vector::new(graph.pan[0], graph.pan[1]),
                });
            }
        }

        FlowEditorAction::PanTo { cursor_screen } => {
            if let Some(pan) = state.pan {
                if let Some(graph) = get_active_graph_mut(flows, flow_id) {
                    let delta_x = cursor_screen.x - pan.cursor_start_screen.x;
                    let delta_y = cursor_screen.y - pan.cursor_start_screen.y;
                    graph.pan = [pan.pan_start.x + delta_x, pan.pan_start.y + delta_y];
                }
            }
        }

        FlowEditorAction::EndPan => {
            state.pan = None;
        }

        FlowEditorAction::StartSelection {
            cursor_scene,
            additive,
        } => {
            state.selection_rect = Some(SelectionRect {
                start: cursor_scene,
                current: cursor_scene,
                additive,
            });
        }

        FlowEditorAction::UpdateSelection { cursor_scene } => {
            if let Some(sel) = &mut state.selection_rect {
                sel.current = cursor_scene;
            }
        }

        FlowEditorAction::FinishSelection => {
            if let Some(sel) = state.selection_rect.take() {
                if let Some(graph) = get_active_graph_mut(flows, flow_id) {
                    let min_x = sel.start.x.min(sel.current.x);
                    let max_x = sel.start.x.max(sel.current.x);
                    let min_y = sel.start.y.min(sel.current.y);
                    let max_y = sel.start.y.max(sel.current.y);
                    for node in &mut graph.nodes {
                        let in_rect = node.position.x >= min_x
                            && node.position.x + node_width_for(&node.kind) <= max_x
                            && node.position.y >= min_y
                            && node.position.y + node.cached_height <= max_y;
                        if sel.additive {
                            if in_rect {
                                node.selected = true;
                            }
                        } else {
                            node.selected = in_rect;
                        }
                    }
                }
            }
        }

        FlowEditorAction::StartConnection {
            node,
            port,
            side,
            cursor_scene,
        } => {
            state.preview_conn = Some(ConnectionPreview {
                from_node: node,
                from_port: port,
                side,
                cursor_scene,
            });
        }

        FlowEditorAction::UpdateConnectionPreview { cursor_scene } => {
            if let Some(conn) = &mut state.preview_conn {
                conn.cursor_scene = cursor_scene;
            }
        }

        FlowEditorAction::FinishConnection {
            from_node,
            from_port,
            to_node,
            to_port,
        } => {
            state.preview_conn = None;
            if let Some(graph) = get_active_graph_mut(flows, flow_id) {
                let _ = graph.connect_ports(from_node.0, from_port.0, to_node.0, to_port.0);
            }
        }

        FlowEditorAction::CancelConnection => {
            state.preview_conn = None;
        }

        FlowEditorAction::ZoomAt {
            cursor_screen,
            root_bounds,
            delta,
        } => {
            if let Some(graph) = get_active_graph_mut(flows, flow_id) {
                let old_zoom = graph.zoom;
                let factor = 1.0 + delta * 0.12;
                let new_zoom = (old_zoom * factor).clamp(0.2, 3.5);
                let rel_x = cursor_screen.x - root_bounds.x;
                let rel_y = cursor_screen.y - root_bounds.y;
                let scene_x = (rel_x - graph.pan[0]) / old_zoom;
                let scene_y = (rel_y - graph.pan[1]) / old_zoom;
                graph.pan[0] = rel_x - scene_x * new_zoom;
                graph.pan[1] = rel_y - scene_y * new_zoom;
                graph.zoom = new_zoom;
            }
        }

        FlowEditorAction::DeleteSelected => {
            if let Some(graph) = get_active_graph_mut(flows, flow_id) {
                let to_delete: Vec<u64> = graph
                    .nodes
                    .iter()
                    .filter(|n| n.selected && !n.is_trigger())
                    .map(|n| n.id)
                    .collect();
                graph.nodes.retain(|n| !to_delete.contains(&n.id));
                graph.z_order.retain(|id| !to_delete.contains(id));
                graph.edges.retain(|e| {
                    !to_delete.contains(&e.from_node) && !to_delete.contains(&e.to_node)
                });
            }
        }

        FlowEditorAction::DeleteEdge(edge) => {
            if let Some(graph) = get_active_graph_mut(flows, flow_id) {
                let _ = graph.remove_edge(edge);
            }
        }

        FlowEditorAction::ToggleSelectedEnabled => {
            if let Some(graph) = get_active_graph_mut(flows, flow_id) {
                for node in graph
                    .nodes
                    .iter_mut()
                    .filter(|n| n.selected && !n.is_trigger())
                {
                    node.enabled = !node.enabled;
                }
            }
        }

        FlowEditorAction::OpenContextPalette { cursor_scene } => {
            state.context_palette = Some(ContextPalette {
                position: cursor_scene,
                pending_connection: None,
            });
            state.preview_conn = None;
        }

        FlowEditorAction::OpenContextPaletteFromConnection {
            cursor_scene,
            from_node,
            from_port,
            side,
        } => {
            state.context_palette = Some(ContextPalette {
                position: cursor_scene,
                pending_connection: Some(PendingConnection {
                    source_node: from_node,
                    source_port: from_port,
                    source_side: side,
                    drop_point: cursor_scene,
                }),
            });
            state.preview_conn = None;
        }

        FlowEditorAction::CloseContextPalette => {
            state.context_palette = None;
            state.preview_conn = None;
        }

        FlowEditorAction::CreateNodeFromTemplate {
            template_id,
            position,
        } => {
            // Save pending connection before clearing state
            let pending = state
                .context_palette
                .as_ref()
                .and_then(|cp| cp.pending_connection);
            state.context_palette = None;
            state.preview_conn = None;

            if let Some(graph) = get_active_graph_mut(flows, flow_id) {
                let new_id = graph.next_id;
                if let Some(node) = action_node_from_palette_id(template_id, position, new_id) {
                    graph.next_id += 1;
                    let node_id = node.id;

                    // Deselect others, auto-select the new node
                    for n in &mut graph.nodes {
                        n.selected = false;
                    }
                    let mut node = node;
                    node.selected = true;

                    graph.z_order.push(node_id);
                    graph.nodes.push(node);

                    // Auto-connect if dropped from a dragged connection
                    if let Some(pending) = pending {
                        let new_node = graph.nodes.iter().find(|n| n.id == node_id);
                        if let Some(new_node) = new_node {
                            match pending.source_side {
                                PortSide::Output => {
                                    // Source is an output → connect to new node's first input
                                    if let Some(in_port) = new_node.cached_ports_in.first().cloned()
                                    {
                                        let _ = graph.connect_ports(
                                            pending.source_node.0,
                                            pending.source_port.0,
                                            node_id,
                                            in_port.id.0,
                                        );
                                    }
                                }
                                PortSide::Input => {
                                    // Source is an input → connect from new node's first output
                                    if let Some(out_port) =
                                        new_node.cached_ports_out.first().cloned()
                                    {
                                        let _ = graph.connect_ports(
                                            node_id,
                                            out_port.id.0,
                                            pending.source_node.0,
                                            pending.source_port.0,
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Not yet implemented
        FlowEditorAction::DuplicateSelected
        | FlowEditorAction::CopySelected
        | FlowEditorAction::CutSelected
        | FlowEditorAction::Paste
        | FlowEditorAction::CenterView => {}
    }
}

// ─── Node body builder ────────────────────────────────────────────────────────

const NODE_ROW_GAP: f32 = 8.0;
const NODE_TOP_PAD: f32 = 8.0;
const NODE_SIDE_PAD: f32 = 10.0;
const NODE_BOTTOM_PAD: f32 = 8.0;
const NODE_ADD_BTN_H: f32 = 28.0;
const TRIGGER_EVENT_ROW_H: f32 = 38.0;
const STATE_MUTATION_HEADER_H: f32 = 32.0;
const STATE_MUTATION_ROW_H: f32 = 44.0;
const NAVIGATE_ROW_H: f32 = 72.0;
const MATCH_VALUE_ROW_H: f32 = 36.0;
const MATCH_ARM_ROW_H: f32 = 36.0;
const MATCH_DEFAULT_ROW_H: f32 = 34.0;
const IF_CONDITION_ROW_H: f32 = 36.0;
const IF_BRANCH_ROW_H: f32 = 34.0;
const CALL_FLOW_ROW_H: f32 = 38.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RowTone {
    Neutral,
    Connected,
    Branch,
    Dragged,
    DropTarget,
}

fn color_alpha(color: Color, alpha: f32) -> Color {
    Color { a: alpha, ..color }
}

fn mix_color(a: Color, b: Color, amount: f32) -> Color {
    let t = amount.clamp(0.0, 1.0);

    Color {
        r: a.r + (b.r - a.r) * t,
        g: a.g + (b.g - a.g) * t,
        b: a.b + (b.b - a.b) * t,
        a: a.a + (b.a - a.a) * t,
    }
}

fn flow_editor_style_tokens(theme: &iced::Theme) -> widgets::flow_editor::Style {
    let palette = theme.extended_palette();
    let mut style = widgets::flow_editor::Style::from_theme(theme);
    style.canvas_bg = palette.background.base.color;
    style.grid_line = color_alpha(palette.background.strong.color, 0.18);
    style.node_bg = palette.background.weak.color;
    style.node_border = color_alpha(palette.background.strong.color, 0.72);
    style.node_border_selected = palette.primary.base.color;
    style.header_bg = mix_color(
        palette.background.strong.color,
        palette.background.base.color,
        0.25,
    );
    style.header_bg_selected = mix_color(style.header_bg, palette.primary.weak.color, 0.35);
    style.title_text = palette.background.base.text;
    style.kind_label_text = palette.background.weak.text;
    style.port_input = palette.success.base.color;
    style.port_output = palette.primary.base.color;
    style.edge = style.port_output;
    style.edge_preview = palette.warning.base.color;
    style
}

fn semantic_row_style(theme: &iced::Theme, tone: RowTone) -> iced::widget::container::Style {
    let style = flow_editor_style_tokens(theme);
    let warning = theme.extended_palette().warning.base.color;
    let (bg, border_color) = match tone {
        RowTone::Neutral => (
            color_alpha(style.node_border, 0.10),
            color_alpha(style.node_border, 0.32),
        ),
        RowTone::Connected => (
            color_alpha(style.port_output, 0.13),
            color_alpha(style.port_output, 0.42),
        ),
        RowTone::Branch => (color_alpha(warning, 0.13), color_alpha(warning, 0.42)),
        RowTone::Dragged => (
            color_alpha(style.port_output, 0.20),
            color_alpha(style.port_output, 0.52),
        ),
        RowTone::DropTarget => (
            color_alpha(style.port_input, 0.20),
            color_alpha(style.port_input, 0.55),
        ),
    };

    iced::widget::container::Style::default()
        .background(bg)
        .color(style.title_text)
        .border(Border {
            color: border_color,
            width: 1.0,
            radius: border::radius(7.0),
        })
}

fn add_button_style(
    theme: &iced::Theme,
    status: iced::widget::button::Status,
) -> iced::widget::button::Style {
    widgets::flow_editor::styles::add_icon_button_style(
        theme,
        status,
        &flow_editor_style_tokens(theme),
    )
}

fn danger_button_style(
    theme: &iced::Theme,
    status: iced::widget::button::Status,
) -> iced::widget::button::Style {
    widgets::flow_editor::styles::danger_icon_button_style(theme, status)
}

fn header_picker_style(
    theme: &iced::Theme,
    status: iced::widget::pick_list::Status,
) -> iced::widget::pick_list::Style {
    widgets::flow_editor::styles::header_pick_list_style(
        theme,
        status,
        &flow_editor_style_tokens(theme),
    )
}

fn default_action_value_for_type(value_type: &ActionValueType) -> ActionValue {
    match value_type {
        ActionValueType::Bool => ActionValue::Bool(false),
        ActionValueType::F32 | ActionValueType::F64 | ActionValueType::Usize => {
            ActionValue::Number(0.0)
        }
        ActionValueType::Enum {
            type_name,
            variants,
        } => ActionValue::EnumVariant {
            type_name: type_name.clone(),
            variant: variants.first().cloned().unwrap_or_default(),
        },
        ActionValueType::String => ActionValue::String(String::new()),
    }
}

fn default_authored_condition() -> AuthoredCondition {
    AuthoredCondition::default()
}

fn authored_conditions_for_display(node: &ActionNodeData) -> Vec<AuthoredCondition> {
    if !node.authored_conditions.is_empty() {
        node.authored_conditions.clone()
    } else if let Some(legacy) = &node.authored_condition {
        vec![legacy.clone()]
    } else {
        vec![default_authored_condition()]
    }
}

fn ensure_authored_conditions_mut(node: &mut ActionNodeData) -> &mut Vec<AuthoredCondition> {
    if node.authored_conditions.is_empty() {
        if let Some(legacy) = node.authored_condition.take() {
            node.authored_conditions.push(legacy);
        } else {
            node.authored_conditions.push(default_authored_condition());
        }
    } else {
        node.authored_condition = None;
    }
    &mut node.authored_conditions
}

fn expression_source_options(
    state_fields: &[StateFieldRef],
    trigger_output_ports: &[TriggerPort],
) -> Vec<ExpressionSourceOption> {
    let mut options = Vec::with_capacity(1 + state_fields.len() + trigger_output_ports.len());
    options.push(ExpressionSourceOption::LiteralValue);
    options.extend(
        state_fields
            .iter()
            .cloned()
            .map(ExpressionSourceOption::StateField),
    );
    options.extend(trigger_output_ports.iter().cloned().map(|p| {
        ExpressionSourceOption::TriggerInput {
            name: p.name,
            value_type: p.value_type,
        }
    }));
    options
}

fn selected_expression_source_option(
    options: &[ExpressionSourceOption],
    source: &AuthoredValueSource,
) -> Option<ExpressionSourceOption> {
    options
        .iter()
        .find(|opt| opt.matches_source(source))
        .cloned()
}

fn condition_operator_options(lhs_type: &ActionValueType) -> Vec<CompareOp> {
    match lhs_type {
        ActionValueType::String => vec![
            CompareOp::Eq,
            CompareOp::NotEq,
            CompareOp::Contains,
            CompareOp::StartsWith,
            CompareOp::EndsWith,
            CompareOp::IsEmpty,
            CompareOp::IsNotEmpty,
            CompareOp::IsValidEmail,
        ],
        ActionValueType::Bool => vec![
            CompareOp::IsTrue,
            CompareOp::IsFalse,
            CompareOp::Eq,
            CompareOp::NotEq,
        ],
        ActionValueType::F32 | ActionValueType::F64 | ActionValueType::Usize => vec![
            CompareOp::Eq,
            CompareOp::NotEq,
            CompareOp::Lt,
            CompareOp::Gt,
            CompareOp::LtEq,
            CompareOp::GtEq,
        ],
        ActionValueType::Enum { .. } => vec![CompareOp::Eq, CompareOp::NotEq],
    }
}

fn normalize_authored_condition_rhs(condition: &mut AuthoredCondition) {
    let lhs_type = condition.lhs.value_type();
    let valid_ops = condition_operator_options(&lhs_type);
    if !valid_ops.contains(&condition.operator) {
        condition.operator = valid_ops.first().cloned().unwrap_or(CompareOp::Eq);
    }
    if !condition.operator.needs_rhs() {
        return;
    }
    if !rhs_literal_matches_type_for_editor(&condition.rhs_literal, &lhs_type) {
        condition.rhs_literal = default_action_value_for_type(&lhs_type);
    }
}

fn rhs_literal_matches_type_for_editor(rhs: &ActionValue, lhs_type: &ActionValueType) -> bool {
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

fn parse_authored_rhs_text(lhs_type: &ActionValueType, raw: &str) -> ActionValue {
    match lhs_type {
        ActionValueType::Bool => match raw.trim().to_ascii_lowercase().as_str() {
            "true" => ActionValue::Bool(true),
            "false" => ActionValue::Bool(false),
            _ => ActionValue::String(raw.to_string()),
        },
        ActionValueType::F32 | ActionValueType::F64 | ActionValueType::Usize => raw
            .trim()
            .parse::<f64>()
            .map(ActionValue::Number)
            .unwrap_or_else(|_| ActionValue::String(raw.to_string())),
        ActionValueType::Enum { type_name, .. } => ActionValue::EnumVariant {
            type_name: type_name.clone(),
            variant: raw.to_string(),
        },
        _ => ActionValue::String(raw.to_string()),
    }
}

fn build_node_header<'a>(
    node: &'a ActionNodeData,
    enum_names: Vec<String>,
    flow_trigger: Option<(Uuid, FlowTrigger)>,
) -> Option<Element<'a, Message>> {
    let nid = node.id;
    match &node.kind {
        ActionNodeKind::Trigger { .. } => flow_trigger.map(|(fid, trigger)| {
            let kind_labels: Vec<&'static str> = FlowTrigger::all_kinds()
                .iter()
                .map(|t| t.kind_label())
                .collect();
            pick_list(
                kind_labels,
                Some(trigger.kind_label()),
                move |label: &'static str| Message::SetFlowTriggerKind(fid, label),
            )
            .width(Length::Fixed(146.0))
            .text_size(11.0)
            .style(header_picker_style)
            .into()
        }),
        ActionNodeKind::NavigateToView { targets } => {
            let mode_options = vec![
                NavigateModeOption::AppView,
                NavigateModeOption::ViewReference,
            ];
            let selected_mode = infer_navigate_mode(targets);
            Some(
                pick_list(
                    mode_options,
                    Some(selected_mode),
                    move |mode: NavigateModeOption| Message::NavigateModeSelected(nid, mode),
                )
                .width(Length::Fixed(162.0))
                .text_size(11.0)
                .style(header_picker_style)
                .into(),
            )
        }
        ActionNodeKind::Match { enum_type, .. } => Some(
            pick_list(enum_names, enum_type.clone(), move |name: String| {
                Message::SetMatchEnumType(nid, name)
            })
            .placeholder("String arms")
            .width(Length::Fixed(182.0))
            .text_size(11.0)
            .style(header_picker_style)
            .into(),
        ),
        _ => None,
    }
}

fn build_node_body<'a>(
    node: &'a ActionNodeData,
    view_options: Vec<ViewOption>,
    view_reference_nav_options: Vec<ViewReferenceNavOption>,
    state_fields: Vec<StateFieldRef>,
    _enum_names: Vec<String>,
    callable_flow_options: Vec<CallableFlowOption>,
    _type_system: &TypeSystem,
    flow_trigger: Option<(Uuid, FlowTrigger)>,
    all_widget_infos: Vec<WidgetInfo>,
    trigger_output_ports: Vec<TriggerPort>,
    row_dragging: Option<(Uuid, Uuid)>,
    row_drag_target: Option<(Uuid, Uuid)>,
) -> Element<'a, Message> {
    let txt = |base: f32| -> f32 { base };
    let sp = |base: f32| -> f32 { base };
    let pd = |base: f32| -> f32 { base };
    let pxf = |base: f32| -> Length { Length::Fixed(base) };
    let nid = node.id;
    let inner: Element<'a, Message> = match &node.kind {
        ActionNodeKind::Trigger { .. } => {
            if let Some((fid, trigger)) = flow_trigger {
                let mut col = column![]
                    .spacing(sp(NODE_ROW_GAP))
                    .padding([pd(NODE_TOP_PAD), pd(NODE_SIDE_PAD)]);

                match &trigger {
                    FlowTrigger::WidgetEvent { rows } => {
                        let mut rows_col = column![].spacing(sp(NODE_ROW_GAP));

                        for row_data in rows.iter() {
                            let rid = row_data.id;
                            let is_dragged = row_dragging.map(|(_, r)| r == rid).unwrap_or(false);
                            let is_drop_target = row_drag_target
                                .map(|(drag_fid, target_rid)| drag_fid == fid && target_rid == rid)
                                .unwrap_or(false);

                            let event_picker = pick_list(
                                all_event_types(),
                                Some(row_data.event_type.clone()),
                                move |e: String| Message::SetRowEventType(fid, rid, e),
                            )
                            .width(pxf(108.0))
                            .text_size(txt(10.5));

                            let event_str = row_data.event_type.clone();
                            let widget_opts: Vec<WidgetOption> = all_widget_infos
                                .iter()
                                .filter(|w| w.supported_events.contains(&event_str.as_str()))
                                .map(|w| WidgetOption {
                                    view_id: w.view_id,
                                    widget_id: w.widget_id,
                                    label: w.label.clone(),
                                })
                                .collect();

                            let selected_widget: Option<WidgetOption> =
                                row_data.target.and_then(|(vid, wraw)| {
                                    widget_opts
                                        .iter()
                                        .find(|o| o.view_id == vid && o.widget_id.0 == wraw)
                                        .cloned()
                                });

                            let widget_picker = pick_list(
                                widget_opts,
                                selected_widget,
                                move |opt: WidgetOption| {
                                    Message::SetRowTarget(
                                        fid,
                                        rid,
                                        Some((opt.view_id, opt.widget_id)),
                                    )
                                },
                            )
                            .placeholder("Widget target...")
                            .width(Length::Fill)
                            .text_size(txt(10.5));

                            let delete_btn: Element<'a, Message> = container(
                                button(icon_lucide::trash_2().size(txt(11.0)))
                                    .style(styles::button::cancel)
                                    .on_press(Message::RemoveWidgetEventRow(fid, rid))
                                    .padding([pd(3.0), pd(6.0)]),
                            )
                            .into();

                            let handle = mouse_area(
                                container(text("⋮").size(txt(9.0)))
                                    .center_x(pxf(14.0))
                                    .center_y(pxf(TRIGGER_EVENT_ROW_H)),
                            )
                            .on_press(Message::StartWidgetEventRowDrag(fid, rid))
                            .interaction(if is_dragged {
                                Interaction::Grabbing
                            } else {
                                Interaction::Grab
                            });

                            let row_inner = row![handle, event_picker, widget_picker, delete_btn]
                                .spacing(sp(3.0))
                                .align_y(Alignment::Center)
                                .width(Length::Fill);

                            let row_tone = if is_dragged {
                                RowTone::Dragged
                            } else if is_drop_target {
                                RowTone::DropTarget
                            } else if row_data.target.is_some() {
                                RowTone::Connected
                            } else {
                                RowTone::Neutral
                            };
                            let row_el: Element<'a, Message> = mouse_area(
                                container(row_inner)
                                    .height(pxf(TRIGGER_EVENT_ROW_H))
                                    .align_y(Alignment::Center)
                                    .padding([0.0, pd(4.0)])
                                    .style(move |theme| semantic_row_style(theme, row_tone)),
                            )
                            .on_enter(Message::HoverWidgetEventRow(fid, rid))
                            .on_move(move |_| Message::HoverWidgetEventRow(fid, rid))
                            .into();

                            rows_col = rows_col.push(row_el);
                        }

                        let rows_wrapped: Element<'a, Message> = mouse_area(rows_col)
                            .on_release(Message::EndWidgetEventRowDrag)
                            .on_exit(Message::EndWidgetEventRowDrag)
                            .into();

                        let add_btn: Element<'a, Message> = container(
                            button(text("+ Add event row").size(txt(10.5)))
                                .on_press(Message::AddWidgetEventRow(fid))
                                .style(add_button_style)
                                .padding([pd(4.0), pd(9.0)]),
                        )
                        .height(pxf(NODE_ADD_BTN_H))
                        .align_x(iced::alignment::Horizontal::Left)
                        .into();

                        col = col.push(rows_wrapped).push(add_btn);
                    }
                    FlowTrigger::Timer { interval_ms } => {
                        let ms_str = interval_ms.to_string();
                        col = col.push(
                            container(
                                text_input("interval ms", &ms_str)
                                    .on_input(move |v| Message::SetFlowTimerInterval(fid, v))
                                    .size(txt(11.0))
                                    .width(Length::Fill),
                            )
                            .height(pxf(TRIGGER_EVENT_ROW_H))
                            .align_y(Alignment::Center)
                            .padding([0.0, pd(6.0)])
                            .style(|theme| semantic_row_style(theme, RowTone::Neutral)),
                        );
                    }
                    FlowTrigger::KeyCombo {
                        ctrl,
                        shift,
                        alt,
                        key,
                    } => {
                        let (c, s, a) = (*ctrl, *shift, *alt);
                        let key_str = key.clone();
                        col = col.push(
                            container(
                                text_input("key", &key_str)
                                    .on_input(move |v| Message::SetFlowKeyComboKey(fid, v))
                                    .size(txt(11.0))
                                    .width(Length::Fill),
                            )
                            .height(pxf(TRIGGER_EVENT_ROW_H))
                            .align_y(Alignment::Center)
                            .padding([0.0, pd(6.0)])
                            .style(|theme| semantic_row_style(theme, RowTone::Neutral)),
                        );
                        col = col.push(
                            container(
                                row![
                                    checkbox(c)
                                        .label("Ctrl")
                                        .on_toggle(move |v| Message::SetFlowKeyComboMods(
                                            fid, v, s, a
                                        ))
                                        .size(txt(12.0)),
                                    checkbox(s)
                                        .label("Shift")
                                        .on_toggle(move |v| Message::SetFlowKeyComboMods(
                                            fid, c, v, a
                                        ))
                                        .size(txt(12.0)),
                                    checkbox(a)
                                        .label("Alt")
                                        .on_toggle(move |v| Message::SetFlowKeyComboMods(
                                            fid, c, s, v
                                        ))
                                        .size(txt(12.0)),
                                ]
                                .spacing(sp(8.0))
                                .align_y(Alignment::Center),
                            )
                            .height(pxf(TRIGGER_EVENT_ROW_H))
                            .align_y(Alignment::Center)
                            .padding([0.0, pd(6.0)])
                            .style(|theme| semantic_row_style(theme, RowTone::Neutral)),
                        );
                    }
                    FlowTrigger::AppStartup => {
                        col = col.push(
                            container(text("Runs once on startup").size(txt(11.0)))
                                .height(pxf(TRIGGER_EVENT_ROW_H))
                                .align_y(Alignment::Center)
                                .padding([0.0, pd(8.0)])
                                .style(|theme| semantic_row_style(theme, RowTone::Neutral)),
                        );
                    }
                    FlowTrigger::Callable => {
                        col = col.push(
                            container(text("Callable entry flow").size(txt(11.0)))
                                .height(pxf(TRIGGER_EVENT_ROW_H))
                                .align_y(Alignment::Center)
                                .padding([0.0, pd(8.0)])
                                .style(|theme| semantic_row_style(theme, RowTone::Neutral)),
                        );
                    }
                }
                col.into()
            } else {
                text("Trigger").size(txt(11.0)).into()
            }
        }

        ActionNodeKind::StateMutation { assignments } => {
            let mut col = column![]
                .spacing(sp(NODE_ROW_GAP))
                .padding([pd(NODE_TOP_PAD), pd(NODE_SIDE_PAD)]);
            col = col.push(
                container(text("Apply assignments in order").size(txt(10.0)))
                    .height(pxf(STATE_MUTATION_HEADER_H))
                    .align_y(Alignment::Center)
                    .padding([0.0, pd(8.0)])
                    .style(|theme| semantic_row_style(theme, RowTone::Neutral)),
            );
            for (idx, assignment) in assignments.iter().enumerate() {
                let sf_opts: Vec<StateFieldRef> = state_fields.to_vec();
                let selected_sf = assignment.target.clone();
                let field_picker = pick_list(sf_opts, selected_sf, move |sf: StateFieldRef| {
                    Message::StateMutationSetTarget(nid, idx, sf)
                })
                .placeholder("Select state field...")
                .width(Length::FillPortion(3))
                .text_size(txt(10.5));

                let value_ui: Element<'_, Message> = match &assignment.value_source {
                    ValueSource::FromPort => {
                        container(text(format!("Input value_{idx}")).size(txt(10.5)))
                            .padding([pd(5.0), pd(8.0)])
                            .style(|theme| semantic_row_style(theme, RowTone::Connected))
                            .width(Length::FillPortion(4))
                            .into()
                    }
                    ValueSource::StateField(src) => {
                        container(text(format!("<- {}", src.display_name)).size(txt(10.5)))
                            .padding([pd(4.0), pd(8.0)])
                            .style(|theme| semantic_row_style(theme, RowTone::Connected))
                            .width(Length::FillPortion(4))
                            .into()
                    }
                    ValueSource::Literal(v) => {
                        let target_type = assignment.target.as_ref().map(|sf| &sf.field_type);
                        match target_type {
                            Some(ActionValueType::Bool) => {
                                let checked = matches!(v, ActionValue::Bool(true));
                                container(toggler(checked).on_toggle(move |b| {
                                    Message::StateMutationSetLiteralBool(nid, idx, b)
                                }))
                                .width(Length::FillPortion(4))
                                .align_x(iced::alignment::Horizontal::Left)
                                .into()
                            }
                            Some(ActionValueType::Enum {
                                type_name,
                                variants,
                            }) => {
                                let type_name = type_name.clone();
                                let current_variant =
                                    if let ActionValue::EnumVariant { variant, .. } = v {
                                        Some(variant.clone())
                                    } else {
                                        None
                                    };
                                let opts: Vec<String> = variants.clone();
                                pick_list(opts, current_variant, move |variant: String| {
                                    Message::StateMutationSetLiteralEnum(
                                        nid,
                                        idx,
                                        type_name.clone(),
                                        variant,
                                    )
                                })
                                .width(Length::FillPortion(4))
                                .text_size(txt(10.5))
                                .into()
                            }
                            _ => {
                                let display = match v {
                                    ActionValue::String(s) => s.clone(),
                                    ActionValue::Number(n) => n.to_string(),
                                    ActionValue::Bool(b) => b.to_string(),
                                    ActionValue::EnumVariant { variant, .. } => variant.clone(),
                                };
                                text_input("value...", &display)
                                    .on_input(move |s| {
                                        Message::StateMutationSetLiteralString(nid, idx, s)
                                    })
                                    .size(txt(10.5))
                                    .width(Length::FillPortion(4))
                                    .into()
                            }
                        }
                    }
                };
                let rm_btn = button(icon_lucide::trash_2().size(txt(11.0)))
                    .on_press(Message::StateMutationRemoveAssignment(nid, idx))
                    .style(styles::button::cancel)
                    .padding([pd(3.0), pd(6.0)]);
                let row_tone = if assignment.target.is_some() {
                    RowTone::Connected
                } else {
                    RowTone::Neutral
                };
                col = col.push(
                    container(
                        row![field_picker, value_ui, rm_btn]
                            .spacing(sp(6.0))
                            .align_y(Alignment::Center),
                    )
                    .height(pxf(STATE_MUTATION_ROW_H))
                    .align_y(Alignment::Center)
                    .padding([0.0, pd(4.0)])
                    .style(move |theme| semantic_row_style(theme, row_tone)),
                );
            }
            let add_btn = button(text("+ Add assignment").size(txt(10.5)))
                .on_press(Message::StateMutationAddAssignment(nid))
                .style(add_button_style)
                .padding([pd(4.0), pd(9.0)]);
            col = col.push(
                container(add_btn)
                    .height(pxf(NODE_ADD_BTN_H))
                    .align_x(iced::alignment::Horizontal::Left)
                    .align_y(Alignment::Center),
            );
            col.into()
        }

        ActionNodeKind::NavigateToView { targets } => {
            let mut col = column![]
                .spacing(sp(NODE_ROW_GAP))
                .padding([pd(NODE_TOP_PAD), pd(NODE_SIDE_PAD)]);
            let selected_mode = infer_navigate_mode(targets);
            let view_reference_choices: Vec<ViewReferenceChoice> = view_reference_nav_options
                .iter()
                .map(|opt| ViewReferenceChoice {
                    owner_view_id: opt.owner_view_id,
                    widget_id: opt.widget_id,
                    label: opt.label.clone(),
                })
                .collect();
            for (slot, target) in targets.iter().enumerate() {
                let target_picker: Element<'_, Message> = match (selected_mode, target) {
                    (NavigateModeOption::AppView, Some(NavigateTarget::AppView { view_id })) => {
                        let selected = view_options.iter().find(|v| v.id == *view_id).cloned();
                        pick_list(view_options.clone(), selected, move |opt: ViewOption| {
                            Message::NavigateAppViewSelected(nid, slot, opt.id)
                        })
                        .placeholder("Select app view...")
                        .width(Length::Fill)
                        .text_size(txt(10.5))
                        .into()
                    }
                    (NavigateModeOption::AppView, _) => pick_list(
                        view_options.clone(),
                        None::<ViewOption>,
                        move |opt: ViewOption| Message::NavigateAppViewSelected(nid, slot, opt.id),
                    )
                    .placeholder("Select app view...")
                    .width(Length::Fill)
                    .text_size(txt(10.5))
                    .into(),
                    (
                        NavigateModeOption::ViewReference,
                        Some(NavigateTarget::ViewReference {
                            owner_view_id,
                            widget_id,
                            target_view_id,
                        }),
                    ) => {
                        let selected_ref = view_reference_choices.iter().find(|choice| {
                            choice.owner_view_id == *owner_view_id && choice.widget_id == *widget_id
                        });
                        let target_options = view_reference_nav_options
                            .iter()
                            .find(|opt| {
                                opt.owner_view_id == *owner_view_id && opt.widget_id == *widget_id
                            })
                            .map(|opt| opt.targets.clone())
                            .unwrap_or_default();
                        let selected_target = target_options
                            .iter()
                            .find(|opt| opt.id == *target_view_id)
                            .cloned();

                        column![
                            pick_list(
                                view_reference_choices.clone(),
                                selected_ref.cloned(),
                                move |choice: ViewReferenceChoice| {
                                    Message::NavigateViewReferenceSelected(
                                        nid,
                                        slot,
                                        choice.owner_view_id,
                                        choice.widget_id,
                                    )
                                }
                            )
                            .placeholder("Select View Reference...")
                            .width(Length::Fill)
                            .text_size(txt(10.5)),
                            pick_list(target_options, selected_target, move |opt: ViewOption| {
                                Message::NavigateViewReferenceTargetSelected(nid, slot, opt.id)
                            })
                            .placeholder("Select target view...")
                            .width(Length::Fill)
                            .text_size(txt(10.5)),
                        ]
                        .spacing(sp(6.0))
                        .into()
                    }
                    (NavigateModeOption::ViewReference, _) => {
                        if view_reference_choices.is_empty() {
                            container(text("No eligible View References found").size(txt(10.5)))
                                .padding([pd(6.0), pd(8.0)])
                                .style(|theme| semantic_row_style(theme, RowTone::Neutral))
                                .width(Length::Fill)
                                .into()
                        } else {
                            column![
                                pick_list(
                                    view_reference_choices.clone(),
                                    None::<ViewReferenceChoice>,
                                    move |choice: ViewReferenceChoice| {
                                        Message::NavigateViewReferenceSelected(
                                            nid,
                                            slot,
                                            choice.owner_view_id,
                                            choice.widget_id,
                                        )
                                    }
                                )
                                .placeholder("Select View Reference...")
                                .width(Length::Fill)
                                .text_size(txt(10.5)),
                                container(text("Select target view...").size(txt(10.5)))
                                    .padding([pd(6.0), pd(8.0)])
                                    .style(|theme| semantic_row_style(theme, RowTone::Neutral))
                                    .width(Length::Fill),
                            ]
                            .spacing(sp(6.0))
                            .into()
                        }
                    }
                };
                let is_complete = matches!(
                    (selected_mode, target),
                    (
                        NavigateModeOption::AppView,
                        Some(NavigateTarget::AppView { .. })
                    ) | (
                        NavigateModeOption::ViewReference,
                        Some(NavigateTarget::ViewReference { .. })
                    )
                );
                let row_tone = if is_complete {
                    RowTone::Connected
                } else {
                    RowTone::Neutral
                };
                let remove_btn = button(icon_lucide::trash_2().size(txt(11.0)))
                    .on_press(Message::RemoveNavigateTarget(nid, slot))
                    .style(styles::button::cancel)
                    .padding([pd(3.0), pd(6.0)]);
                let row_el: Element<'_, Message> = container(
                    row![
                        container(target_picker).width(Length::Fill),
                        container(remove_btn).width(pxf(32.0))
                    ]
                    .spacing(sp(6.0))
                    .align_y(iced::Alignment::Center),
                )
                .height(pxf(NAVIGATE_ROW_H))
                .align_y(Alignment::Center)
                .padding([0.0, pd(4.0)])
                .style(move |theme| semantic_row_style(theme, row_tone))
                .into();
                col = col.push(row_el);
            }
            let add_btn: Element<'_, Message> = container(
                button(text("+ Add target").size(txt(10.5)))
                    .on_press(Message::AddNavigateTarget(nid))
                    .style(add_button_style)
                    .padding([pd(4.0), pd(9.0)]),
            )
            .height(pxf(NODE_ADD_BTN_H))
            .align_x(iced::alignment::Horizontal::Left)
            .into();
            col.push(add_btn).into()
        }

        ActionNodeKind::Conditional => {
            let source_options = expression_source_options(&state_fields, &trigger_output_ports);
            let mut authored_conditions = authored_conditions_for_display(node);
            for condition in authored_conditions.iter_mut() {
                normalize_authored_condition_rhs(condition);
            }
            if authored_conditions.is_empty() {
                authored_conditions.push(default_authored_condition());
            }

            let join_row: Element<'a, Message> = container(
                row![
                    text("Conditions").size(txt(11.0)),
                    Space::new().width(Length::Fill),
                    pick_list(
                        vec![ConditionJoinMode::All, ConditionJoinMode::Any],
                        Some(node.authored_condition_join),
                        move |mode| Message::SetConditionalJoinMode(nid, mode),
                    )
                    .width(pxf(118.0))
                    .text_size(txt(10.5))
                    .style(header_picker_style)
                ]
                .align_y(Alignment::Center),
            )
            .height(pxf(IF_CONDITION_ROW_H))
            .align_y(Alignment::Center)
            .padding([0.0, pd(8.0)])
            .style(|theme| semantic_row_style(theme, RowTone::Neutral))
            .into();

            let mut col = column![join_row]
                .spacing(sp(NODE_ROW_GAP))
                .padding([pd(NODE_TOP_PAD), pd(NODE_SIDE_PAD)]);

            for (idx, authored) in authored_conditions.iter().cloned().enumerate() {
                let lhs_type = authored.lhs.value_type();
                let selected_source =
                    selected_expression_source_option(&source_options, &authored.lhs);
                let operator_options = condition_operator_options(&lhs_type);
                let selected_operator = if operator_options.contains(&authored.operator) {
                    Some(authored.operator.clone())
                } else {
                    operator_options.first().cloned()
                };
                let needs_rhs = selected_operator
                    .as_ref()
                    .map(|op| op.needs_rhs())
                    .unwrap_or(false);

                let source_picker = pick_list(
                    source_options.clone(),
                    selected_source.clone(),
                    move |source| Message::SetConditionalSource(nid, idx, source),
                )
                .placeholder("Left value")
                .width(if needs_rhs {
                    Length::FillPortion(4)
                } else {
                    Length::FillPortion(6)
                })
                .text_size(txt(10.5));
                let op_picker = pick_list(
                    operator_options.clone(),
                    selected_operator.clone(),
                    move |op| Message::SetConditionalOperator(nid, idx, op),
                )
                .placeholder("Operator")
                .width(if needs_rhs {
                    Length::FillPortion(3)
                } else {
                    Length::FillPortion(4)
                })
                .text_size(txt(10.5));

                let mut row_controls = row![source_picker, op_picker]
                    .spacing(sp(6.0))
                    .align_y(Alignment::Center);

                if needs_rhs {
                    let rhs_control: Element<'a, Message> = match &lhs_type {
                        ActionValueType::Bool => {
                            let checked = matches!(authored.rhs_literal, ActionValue::Bool(true));
                            container(
                                toggler(checked)
                                    .label("true")
                                    .on_toggle(move |value| {
                                        Message::SetConditionalRhsBool(nid, idx, value)
                                    })
                                    .size(txt(11.0)),
                            )
                            .width(Length::FillPortion(2))
                            .align_x(iced::alignment::Horizontal::Left)
                            .into()
                        }
                        ActionValueType::Enum { variants, .. } if !variants.is_empty() => {
                            let selected_variant = match &authored.rhs_literal {
                                ActionValue::EnumVariant { variant, .. } => Some(variant.clone()),
                                _ => variants.first().cloned(),
                            };
                            pick_list(variants.clone(), selected_variant, move |variant| {
                                Message::SetConditionalRhsText(nid, idx, variant)
                            })
                            .placeholder("Variant")
                            .width(Length::FillPortion(3))
                            .text_size(txt(10.5))
                            .into()
                        }
                        _ => {
                            let rhs_display = match &authored.rhs_literal {
                                ActionValue::String(s) => s.clone(),
                                ActionValue::Number(n) => n.to_string(),
                                ActionValue::Bool(b) => b.to_string(),
                                ActionValue::EnumVariant { variant, .. } => variant.clone(),
                            };
                            text_input("Right value", rhs_display.as_str())
                                .on_input(move |value| {
                                    Message::SetConditionalRhsText(nid, idx, value)
                                })
                                .size(txt(10.5))
                                .width(Length::FillPortion(3))
                                .into()
                        }
                    };
                    row_controls = row_controls.push(rhs_control);
                }

                let can_remove = authored_conditions.len() > 1;
                let remove_btn = {
                    let base = button(icon_lucide::trash_2().size(txt(11.0)))
                        .style(styles::button::cancel)
                        .padding([pd(3.0), pd(6.0)]);
                    if can_remove {
                        base.on_press(Message::RemoveConditionalRow(nid, idx))
                    } else {
                        base
                    }
                };
                row_controls = row_controls.push(container(remove_btn).width(pxf(32.0)));

                let row_tone = if selected_source.is_some() {
                    RowTone::Connected
                } else {
                    RowTone::Neutral
                };
                col = col.push(
                    container(row_controls)
                        .height(pxf(IF_CONDITION_ROW_H))
                        .align_y(Alignment::Center)
                        .padding([0.0, pd(6.0)])
                        .style(move |theme| semantic_row_style(theme, row_tone)),
                );
            }

            col = col.push(
                container(
                    button(text("+ Add condition").size(txt(10.5)))
                        .on_press(Message::AddConditionalRow(nid))
                        .style(add_button_style)
                        .padding([pd(4.0), pd(9.0)]),
                )
                .height(pxf(NODE_ADD_BTN_H))
                .align_x(iced::alignment::Horizontal::Left),
            );

            col.push(
                container(row![
                    text("Then (true)").size(txt(11.0)),
                    Space::new().width(Length::Fill),
                    text("flow").size(txt(10.0)),
                ])
                .height(pxf(IF_BRANCH_ROW_H))
                .align_y(Alignment::Center)
                .padding([0.0, pd(8.0)])
                .style(|theme| semantic_row_style(theme, RowTone::Branch)),
            )
            .push(
                container(row![
                    text("Else (false)").size(txt(11.0)),
                    Space::new().width(Length::Fill),
                    text("flow").size(txt(10.0)),
                ])
                .height(pxf(IF_BRANCH_ROW_H))
                .align_y(Alignment::Center)
                .padding([0.0, pd(8.0)])
                .style(|theme| semantic_row_style(theme, RowTone::Branch)),
            )
            .into()
        }

        ActionNodeKind::Match { arms, enum_type } => {
            let subject = node.authored_match_subject.clone();
            let source_options = expression_source_options(&state_fields, &trigger_output_ports);
            let selected_source = subject
                .as_ref()
                .and_then(|s| selected_expression_source_option(&source_options, s));
            let subject_literal = match subject.as_ref() {
                Some(AuthoredValueSource::Literal(ActionValue::String(s))) => s.clone(),
                Some(AuthoredValueSource::Literal(ActionValue::Number(n))) => n.to_string(),
                Some(AuthoredValueSource::Literal(ActionValue::Bool(b))) => b.to_string(),
                Some(AuthoredValueSource::Literal(ActionValue::EnumVariant {
                    variant, ..
                })) => variant.clone(),
                _ => String::new(),
            };

            let mut col = column![]
                .spacing(sp(NODE_ROW_GAP))
                .padding([pd(NODE_TOP_PAD), pd(NODE_SIDE_PAD)]);
            col = col.push(
                container(
                    row![
                        text("Match subject").size(txt(11.0)),
                        Space::new().width(Length::Fill)
                    ]
                    .align_y(Alignment::Center),
                )
                .height(pxf(MATCH_VALUE_ROW_H))
                .align_y(Alignment::Center)
                .padding([0.0, pd(8.0)])
                .style(|theme| semantic_row_style(theme, RowTone::Neutral)),
            );

            col = col.push(
                container(
                    pick_list(
                        source_options.clone(),
                        selected_source.clone(),
                        move |source| Message::SetMatchSubjectSource(nid, source),
                    )
                    .placeholder("Subject source")
                    .width(Length::Fill)
                    .text_size(txt(10.5)),
                )
                .height(pxf(MATCH_VALUE_ROW_H))
                .align_y(Alignment::Center)
                .padding([0.0, pd(6.0)])
                .style(|theme| semantic_row_style(theme, RowTone::Connected)),
            );
            if matches!(subject, Some(AuthoredValueSource::Literal(_))) {
                let literal_row: Element<'a, Message> = if enum_type.is_some() && !arms.is_empty() {
                    let selected_variant = match subject.as_ref() {
                        Some(AuthoredValueSource::Literal(ActionValue::EnumVariant {
                            variant,
                            ..
                        })) => Some(variant.clone()),
                        _ => arms.first().cloned(),
                    };
                    container(
                        pick_list(arms.clone(), selected_variant, move |variant| {
                            Message::SetMatchSubjectLiteral(nid, variant)
                        })
                        .placeholder("Literal subject variant")
                        .width(Length::Fill)
                        .text_size(txt(10.5)),
                    )
                    .height(pxf(MATCH_VALUE_ROW_H))
                    .align_y(Alignment::Center)
                    .padding([0.0, pd(6.0)])
                    .style(|theme| semantic_row_style(theme, RowTone::Connected))
                    .into()
                } else {
                    container(
                        text_input("Literal subject", subject_literal.as_str())
                            .on_input(move |value| Message::SetMatchSubjectLiteral(nid, value))
                            .size(txt(10.5))
                            .width(Length::Fill),
                    )
                    .height(pxf(MATCH_VALUE_ROW_H))
                    .align_y(Alignment::Center)
                    .padding([0.0, pd(6.0)])
                    .style(|theme| semantic_row_style(theme, RowTone::Connected))
                    .into()
                };
                col = col.push(literal_row);
            }

            if enum_type.is_some() {
                for arm in arms.iter() {
                    col = col.push(
                        container(text(arm).size(txt(11.0)))
                            .height(pxf(MATCH_ARM_ROW_H))
                            .align_y(Alignment::Center)
                            .padding([0.0, pd(8.0)])
                            .style(|theme| semantic_row_style(theme, RowTone::Branch)),
                    );
                }
                col = col.push(
                    container(
                        row![
                            text("Default").size(txt(11.0)),
                            Space::new().width(Length::Fill),
                            button(text("Clear enum").size(txt(10.5)))
                                .on_press(Message::ClearMatchEnumType(nid))
                                .style(danger_button_style)
                                .padding([pd(2.0), pd(6.0)])
                        ]
                        .align_y(Alignment::Center),
                    )
                    .height(pxf(MATCH_DEFAULT_ROW_H))
                    .align_y(Alignment::Center)
                    .padding([0.0, pd(8.0)])
                    .style(|theme| semantic_row_style(theme, RowTone::Branch)),
                );
            } else {
                for (i, arm) in arms.iter().enumerate() {
                    let idx = i;
                    col = col.push(
                        container(
                            row![
                                text_input("arm value", arm)
                                    .on_input(move |v| Message::SetMatchArm(nid, idx, v))
                                    .size(txt(10.8))
                                    .width(Length::Fill),
                                button(icon_lucide::trash_2().size(txt(11.0)))
                                    .style(styles::button::cancel)
                                    .padding([pd(2.0), pd(6.0)])
                                    .on_press(Message::RemoveMatchArm(nid, idx))
                            ]
                            .spacing(sp(6.0))
                            .align_y(Alignment::Center),
                        )
                        .height(pxf(MATCH_ARM_ROW_H))
                        .align_y(Alignment::Center)
                        .padding([0.0, pd(6.0)])
                        .style(|theme| semantic_row_style(theme, RowTone::Branch)),
                    );
                }
                col = col.push(
                    container(
                        row![
                            text("Default").size(txt(11.0)),
                            Space::new().width(Length::Fill),
                            button(text("+ Arm").size(txt(10.5)))
                                .on_press(Message::AddMatchArm(nid))
                                .style(add_button_style)
                                .padding([pd(2.0), pd(7.0)])
                        ]
                        .align_y(Alignment::Center),
                    )
                    .height(pxf(MATCH_DEFAULT_ROW_H))
                    .align_y(Alignment::Center)
                    .padding([0.0, pd(8.0)])
                    .style(|theme| semantic_row_style(theme, RowTone::Branch)),
                );
            }
            col.into()
        }

        ActionNodeKind::CallFlow { flow_id } => {
            let selected: Option<CallableFlowOption> = flow_id.and_then(|id| {
                callable_flow_options
                    .iter()
                    .find(|opt| opt.id == id)
                    .cloned()
            });
            let row: Element<'a, Message> = if callable_flow_options.is_empty() {
                container(text("No callable flows available").size(txt(10.5)))
                    .padding([pd(6.0), pd(8.0)])
                    .style(|theme| semantic_row_style(theme, RowTone::Neutral))
                    .height(pxf(CALL_FLOW_ROW_H))
                    .align_y(Alignment::Center)
                    .into()
            } else {
                container(
                    pick_list(
                        callable_flow_options,
                        selected,
                        move |opt: CallableFlowOption| Message::SetCallFlowTarget(nid, opt.id),
                    )
                    .placeholder("Select callable flow...")
                    .width(Length::Fill)
                    .text_size(txt(10.8)),
                )
                .height(pxf(CALL_FLOW_ROW_H))
                .align_y(Alignment::Center)
                .padding([0.0, pd(8.0)])
                .style(|theme| semantic_row_style(theme, RowTone::Neutral))
                .into()
            };
            column![row]
                .padding([pd(NODE_TOP_PAD), pd(NODE_SIDE_PAD)])
                .into()
        }

        ActionNodeKind::StringLiteral { .. }
        | ActionNodeKind::NumberLiteral { .. }
        | ActionNodeKind::BoolLiteral { .. }
        | ActionNodeKind::EnumLiteral { .. }
        | ActionNodeKind::Compare { .. }
        | ActionNodeKind::LogicAnd
        | ActionNodeKind::LogicOr
        | ActionNodeKind::LogicNot
        | ActionNodeKind::Expression { .. } => {
            text("(legacy value/operator node — retired from palette; use embedded expressions)")
                .size(txt(10.0))
                .into()
        }
        ActionNodeKind::SetState { .. } | ActionNodeKind::UpdateState { .. } => {
            text("(legacy state node — replace with State Mutation)")
                .size(txt(10.0))
                .into()
        }

        ActionNodeKind::CallAction { .. } => text("(legacy call node — replace with Call Flow)")
            .size(txt(10.0))
            .into(),

        ActionNodeKind::LegacyGetState { .. } => text("(legacy node — will be removed on save)")
            .size(txt(10.0))
            .into(),
    };
    container(inner)
        .padding([pd(4.0), pd(6.0)])
        .style(|theme| {
            widgets::flow_editor::styles::node_body_style(theme, &flow_editor_style_tokens(theme))
        })
        .into()
}

// ─── Value helpers ────────────────────────────────────────────────────────────

fn default_value_for_type(t: &ActionValueType) -> ActionValue {
    match t {
        ActionValueType::Bool => ActionValue::Bool(false),
        ActionValueType::F32 | ActionValueType::F64 => ActionValue::Number(0.0),
        ActionValueType::Enum {
            type_name,
            variants,
        } => ActionValue::EnumVariant {
            type_name: type_name.clone(),
            variant: variants.first().cloned().unwrap_or_default(),
        },
        _ => ActionValue::String(String::new()),
    }
}

fn coerce_literal(current: ActionValue, raw: &str) -> ActionValue {
    match current {
        ActionValue::Number(_) => raw
            .parse::<f64>()
            .map(ActionValue::Number)
            .unwrap_or(ActionValue::Number(0.0)),
        ActionValue::Bool(_) => ActionValue::Bool(raw == "true"),
        ActionValue::EnumVariant { type_name, .. } => ActionValue::EnumVariant {
            type_name,
            variant: raw.to_string(),
        },
        _ => ActionValue::String(raw.to_string()),
    }
}

// ─── State field collection ───────────────────────────────────────────────────

fn collect_view_reference_nav_options(
    all_views: &BTreeMap<Uuid, AppView>,
) -> Vec<ViewReferenceNavOption> {
    let mut options = Vec::new();
    for (owner_view_id, owner_view) in all_views {
        collect_view_reference_nav_options_recursive(
            owner_view.hierarchy.root(),
            *owner_view_id,
            &owner_view.name,
            all_views,
            &mut options,
        );
    }
    options.sort_by(|a, b| a.label.cmp(&b.label));
    options
}

fn collect_view_reference_nav_options_recursive(
    widget: &Widget,
    owner_view_id: Uuid,
    owner_view_name: &str,
    all_views: &BTreeMap<Uuid, AppView>,
    out: &mut Vec<ViewReferenceNavOption>,
) {
    if widget.widget_type == WidgetType::ViewReference {
        if let Some(primary_view_id) = widget.properties.referenced_view_id {
            let mut seen = std::collections::HashSet::new();
            let mut targets = Vec::new();
            if seen.insert(primary_view_id) {
                if let Some(view) = all_views.get(&primary_view_id) {
                    targets.push(ViewOption {
                        id: primary_view_id,
                        name: view.name.clone(),
                    });
                }
            }
            for target_view_id in &widget.properties.extra_view_ids {
                if seen.insert(*target_view_id) {
                    if let Some(view) = all_views.get(target_view_id) {
                        targets.push(ViewOption {
                            id: *target_view_id,
                            name: view.name.clone(),
                        });
                    }
                }
            }
            if targets.len() > 1 {
                let widget_name = if widget.properties.widget_name.trim().is_empty() {
                    format!("widget_{}", widget.id.0)
                } else {
                    widget.properties.widget_name.trim().to_string()
                };
                out.push(ViewReferenceNavOption {
                    owner_view_id,
                    widget_id: widget.id,
                    label: format!("{owner_view_name}/{widget_name}"),
                    targets,
                });
            }
        }
    }

    for child in &widget.children {
        collect_view_reference_nav_options_recursive(
            child,
            owner_view_id,
            owner_view_name,
            all_views,
            out,
        );
    }
}

fn collect_state_fields(all_views: &BTreeMap<Uuid, AppView>) -> Vec<StateFieldRef> {
    let mut fields = Vec::new();
    for view in all_views.values() {
        collect_fields_from_widget(view.hierarchy.root(), view.id, &view.name, &mut fields);
        collect_view_selection_fields(
            view.hierarchy.root(),
            view.id,
            &view.name,
            all_views,
            &mut fields,
        );
        // Add custom state fields
        for cf in &view.custom_state {
            fields.push(StateFieldRef {
                view_id: view.id,
                source: StateRefSource::Custom {
                    field_id: cf.id,
                    field_name: cf.name.clone(),
                },
                field_type: cf.field_type.to_action_value_type(),
                display_name: format!("{} / {} (custom)", view.name, cf.display_name),
            });
        }
    }
    fields
}

fn collect_view_selection_fields(
    widget: &Widget,
    view_id: Uuid,
    view_name: &str,
    all_views: &BTreeMap<Uuid, AppView>,
    out: &mut Vec<StateFieldRef>,
) {
    if widget.widget_type == WidgetType::ViewReference {
        if let Some(primary_id) = widget.properties.referenced_view_id {
            if !widget.properties.extra_view_ids.is_empty() {
                if let Some(primary_view) = all_views.get(&primary_id) {
                    // Use the same naming logic as collect_view_refs_recursive in events.rs:
                    // builder::to_snake_case = s.to_lowercase().replace(' ', "_")
                    let primary_field = if !widget.properties.widget_name.trim().is_empty() {
                        widget
                            .properties
                            .widget_name
                            .trim()
                            .to_lowercase()
                            .replace(' ', "_")
                    } else {
                        primary_view.name.trim().to_lowercase().replace(' ', "_")
                    };
                    let type_name = format!("{}Selection", pascal_case(&primary_field));
                    let field_name = format!("{}_selection", primary_field);
                    // Primary variant uses actual view name, not widget alias
                    let mut variants = vec![pascal_case(&snake_case(&primary_view.name))];
                    for eid in &widget.properties.extra_view_ids {
                        if let Some(ev) = all_views.get(eid) {
                            variants.push(pascal_case(&snake_case(&ev.name)));
                        }
                    }
                    out.push(StateFieldRef {
                        view_id,
                        source: StateRefSource::ViewSelection {
                            widget_id: widget.id,
                            field_name,
                        },
                        field_type: ActionValueType::Enum {
                            type_name,
                            variants,
                        },
                        display_name: format!("{} / {} (view selection)", view_name, primary_field),
                    });
                }
            }
        }
    }
    for child in &widget.children {
        collect_view_selection_fields(child, view_id, view_name, all_views, out);
    }
}

fn snake_case(s: &str) -> String {
    s.chars()
        .enumerate()
        .flat_map(|(i, c)| {
            if c.is_uppercase() && i > 0 {
                vec!['_', c.to_lowercase().next().unwrap_or(c)]
            } else {
                vec![c.to_lowercase().next().unwrap_or(c)]
            }
        })
        .collect::<String>()
        .replace(' ', "_")
}

fn pascal_case(s: &str) -> String {
    s.split(|c: char| c == '_' || c == ' ')
        .filter(|p| !p.is_empty())
        .map(|p| {
            let mut chars = p.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => c.to_uppercase().to_string() + chars.as_str(),
            }
        })
        .collect()
}

fn collect_fields_from_widget(
    widget: &Widget,
    view_id: Uuid,
    view_name: &str,
    out: &mut Vec<StateFieldRef>,
) {
    let widget_label = if widget.properties.widget_name.is_empty() {
        format!("{:?}", widget.widget_type)
    } else {
        widget.properties.widget_name.clone()
    };
    let prefix = format!("{view_name} / {widget_label}");

    let make_widget_ref = |suffix: &str, ftype: ActionValueType, label: &str| StateFieldRef {
        view_id,
        source: StateRefSource::Widget {
            widget_id: widget.id,
            field_suffix: suffix.to_string(),
        },
        field_type: ftype,
        display_name: format!("{prefix} / {label}"),
    };

    match widget.widget_type {
        WidgetType::TextInput => {
            out.push(make_widget_ref("_value", ActionValueType::String, "value"));
        }
        WidgetType::Slider | WidgetType::VerticalSlider => {
            out.push(make_widget_ref("_value", ActionValueType::F32, "value"));
        }
        WidgetType::Checkbox => {
            out.push(make_widget_ref(
                "_checked",
                ActionValueType::Bool,
                "checked",
            ));
        }
        WidgetType::Toggler => {
            out.push(make_widget_ref("_active", ActionValueType::Bool, "active"));
        }
        WidgetType::GenericOverlay => {
            out.push(make_widget_ref("_open", ActionValueType::Bool, "open"));
        }
        WidgetType::DatePicker => {
            out.push(make_widget_ref("_open", ActionValueType::Bool, "open"));
        }
        WidgetType::PickList => {
            out.push(make_widget_ref(
                "_selected",
                ActionValueType::String,
                "selected",
            ));
        }
        WidgetType::Radio => {
            out.push(make_widget_ref(
                "_selected",
                ActionValueType::Usize,
                "selected",
            ));
        }
        _ => {}
    }

    for child in &widget.children {
        collect_fields_from_widget(child, view_id, view_name, out);
    }
}

// ─── Graph helpers ────────────────────────────────────────────────────────────

fn get_active_graph_mut<'a>(
    flows: &'a mut Vec<AppFlow>,
    flow_id: Option<Uuid>,
) -> Option<&'a mut ActionGraph> {
    let fid = flow_id?;
    flows.iter_mut().find(|f| f.id == fid).map(|f| &mut f.graph)
}

fn get_active_graph_ref<'a>(
    flows: &'a Vec<AppFlow>,
    flow_id: Option<Uuid>,
) -> Option<&'a ActionGraph> {
    let fid = flow_id?;
    flows.iter().find(|f| f.id == fid).map(|f| &f.graph)
}

fn get_current_viewport(
    flows: &Vec<AppFlow>,
    flow_id: Option<Uuid>,
) -> widgets::flow_editor::Viewport2D {
    get_active_graph_ref(flows, flow_id)
        .map(|g| g.viewport())
        .unwrap_or_default()
}

fn node_width_for(kind: &ActionNodeKind) -> f32 {
    match kind {
        ActionNodeKind::Trigger { .. } => 380.0,
        ActionNodeKind::NavigateToView { .. } => 430.0,
        ActionNodeKind::StateMutation { .. } | ActionNodeKind::SetState { .. } => 460.0,
        ActionNodeKind::Conditional => 420.0,
        ActionNodeKind::Match { .. } => 430.0,
        ActionNodeKind::CallFlow { .. } | ActionNodeKind::CallAction { .. } => 320.0,
        ActionNodeKind::StringLiteral { .. }
        | ActionNodeKind::NumberLiteral { .. }
        | ActionNodeKind::BoolLiteral { .. } => 220.0,
        _ => 280.0,
    }
}

fn clear_interaction(state: &mut ActionEditorState) {
    state.drag = None;
    state.pan = None;
    state.selection_rect = None;
    state.preview_conn = None;
    state.context_palette = None;
    state.widget_event_row_dragging = None;
    state.widget_event_row_drop_target = None;
}

/// All widget event type names available as flow targets.
fn all_event_types() -> Vec<String> {
    vec![
        "on_press",
        "on_release",
        "on_double_click",
        "on_input",
        "on_submit",
        "on_paste",
        "on_toggle",
        "on_change",
        "on_select",
        "on_option_hovered",
        "on_open",
        "on_close",
        "on_right_press",
        "on_right_release",
        "on_middle_press",
        "on_middle_release",
        "on_scroll",
        "on_enter",
        "on_exit",
        "on_move",
    ]
    .into_iter()
    .map(|s| s.to_string())
    .collect()
}

fn get_flow_trigger_info(
    flows: &Vec<AppFlow>,
    selected_flow_id: Option<Uuid>,
) -> Option<(Uuid, FlowTrigger)> {
    selected_flow_id.and_then(|fid| {
        flows
            .iter()
            .find(|f| f.id == fid)
            .map(|f| (fid, f.trigger.clone()))
    })
}

/// Compute the scene-space height for a Trigger node based on its body content.
fn trigger_node_body_height(trigger: &Option<(Uuid, FlowTrigger)>, _n_widgets: usize) -> f32 {
    const HEADER: f32 = 42.0;
    const MIN_H: f32 = 138.0;
    match trigger.as_ref().map(|(_, t)| t) {
        Some(FlowTrigger::WidgetEvent { rows }) => {
            let n = rows.len().max(1) as f32;
            (HEADER
                + NODE_TOP_PAD
                + n * (TRIGGER_EVENT_ROW_H + NODE_ROW_GAP)
                + NODE_ADD_BTN_H
                + NODE_BOTTOM_PAD)
                .max(MIN_H)
        }
        Some(FlowTrigger::Timer { .. }) => {
            (HEADER + NODE_TOP_PAD + TRIGGER_EVENT_ROW_H + NODE_BOTTOM_PAD).max(MIN_H)
        }
        Some(FlowTrigger::KeyCombo { .. }) => (HEADER
            + NODE_TOP_PAD
            + TRIGGER_EVENT_ROW_H
            + NODE_ROW_GAP
            + TRIGGER_EVENT_ROW_H
            + NODE_BOTTOM_PAD)
            .max(MIN_H),
        _ => (HEADER + NODE_TOP_PAD + TRIGGER_EVENT_ROW_H + NODE_BOTTOM_PAD).max(MIN_H),
    }
}

/// Collects all widgets with full type/properties info for per-row event filtering.
fn collect_all_widget_infos(all_views: &BTreeMap<Uuid, AppView>) -> Vec<WidgetInfo> {
    let mut out = Vec::new();
    for (vid, view) in all_views.iter() {
        collect_widget_infos_recursive(view.hierarchy.root(), *vid, &view.name, &mut out);
    }
    out
}

fn widget_supports_event(
    all_views: &BTreeMap<Uuid, AppView>,
    view_id: Uuid,
    widget_id: WidgetId,
    event: &str,
) -> bool {
    all_views
        .get(&view_id)
        .and_then(|view| view.hierarchy.get_widget_by_id(widget_id))
        .map(|widget| {
            actionable_events(widget.widget_type, &widget.properties)
                .iter()
                .any(|name| *name == event)
        })
        .unwrap_or(false)
}

fn collect_widget_infos_recursive(
    widget: &Widget,
    view_id: Uuid,
    view_name: &str,
    out: &mut Vec<WidgetInfo>,
) {
    let label = if widget.properties.widget_name.is_empty() {
        format!("{}/{:?}", view_name, widget.widget_type)
    } else {
        format!("{}/{}", view_name, widget.properties.widget_name)
    };
    out.push(WidgetInfo {
        view_id,
        widget_id: widget.id,
        supported_events: actionable_events(widget.widget_type, &widget.properties),
        label,
    });
    for child in &widget.children {
        collect_widget_infos_recursive(child, view_id, view_name, out);
    }
}

/// Old flat widget list — kept for `count_all_widgets` and places still using it.
fn collect_all_widgets(
    widget: &Widget,
    view_id: Uuid,
    view_name: &str,
    out: &mut Vec<(Uuid, WidgetId, String)>,
) {
    let label = if widget.properties.widget_name.is_empty() {
        format!("{}/{:?}", view_name, widget.widget_type)
    } else {
        format!("{}/{}", view_name, widget.properties.widget_name)
    };
    out.push((view_id, widget.id, label));
    for child in &widget.children {
        collect_all_widgets(child, view_id, view_name, out);
    }
}

fn count_all_widgets(all_views: &BTreeMap<Uuid, AppView>) -> usize {
    collect_all_widget_infos(all_views).len()
}

// ─── WidgetEvent row helpers ──────────────────────────────────────────────────

fn find_flow_mut<'a>(flows: &'a mut Vec<AppFlow>, flow_id: Uuid) -> Option<&'a mut AppFlow> {
    flows.iter_mut().find(|f| f.id == flow_id)
}

fn move_widget_event_row(rows: &mut Vec<WidgetEventRow>, drag_id: Uuid, target_id: Uuid) {
    let from = rows.iter().position(|r| r.id == drag_id);
    let to = rows.iter().position(|r| r.id == target_id);
    if let (Some(from), Some(to)) = (from, to) {
        if from != to {
            let row = rows.remove(from);
            let insert = to.min(rows.len());
            rows.insert(insert, row);
        }
    }
}

/// Before sync_cache: apply canonical trigger topology sync for the selected flow.
fn sync_selected_flow_trigger_topology(flows: &mut Vec<AppFlow>, selected_flow_id: Option<Uuid>) {
    let Some(fid) = selected_flow_id else { return };
    let Some(flow) = flows.iter_mut().find(|f| f.id == fid) else {
        return;
    };
    flow.sync_trigger_topology();
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::Point;

    use crate::action_system::node_kinds::{
        ActionNodeKind, ActionValue, AuthoredValueSource, CompareOp, ConditionJoinMode,
        NavigateTarget, ValueSource,
    };

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

    fn single_view_with_widget(
        widget_type: WidgetType,
        configure: impl FnOnce(&mut Widget),
    ) -> (BTreeMap<Uuid, AppView>, Uuid, WidgetId) {
        let view_id = Uuid::new_v4();
        let mut view = AppView::with_id(view_id, "Main".to_string(), 0);
        let layout_id = view
            .hierarchy
            .add_child(WidgetId(0), WidgetType::Column)
            .expect("add layout");
        let widget_id = view
            .hierarchy
            .add_child(layout_id, widget_type)
            .expect("add widget");
        let widget = view
            .hierarchy
            .get_widget_by_id_mut(widget_id)
            .expect("widget exists");
        configure(widget);
        (BTreeMap::from([(view_id, view)]), view_id, widget_id)
    }

    #[test]
    fn set_flow_trigger_kind_rebuilds_trigger_ports_immediately() {
        let mut all_views: BTreeMap<Uuid, AppView> = BTreeMap::new();
        let mut flows = vec![AppFlow::new("flow".to_string(), FlowTrigger::Callable)];
        let flow_id = flows[0].id;
        let mut state = ActionEditorState::default();
        // Keep unselected to verify SetFlowTriggerKind does not rely on selected-flow sync.
        state.selected_flow_id = None;

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
        let set_state_flow_in = set_state
            .cached_ports_in
            .iter()
            .find(|p| p.label == "flow_in")
            .expect("setstate flow_in")
            .id
            .0;
        flows[0].graph.nodes.push(set_state);
        flows[0].graph.z_order.push(2);
        flows[0].graph.next_id = 3;

        let trigger_flow_out = flows[0]
            .graph
            .nodes
            .iter()
            .find(|n| n.is_trigger())
            .and_then(|n| n.cached_ports_out.first())
            .expect("trigger flow_out")
            .id
            .0;
        assert!(
            flows[0]
                .graph
                .connect_ports(1, trigger_flow_out, 2, set_state_flow_in)
        );
        assert_eq!(flows[0].graph.edges.len(), 1);

        let _ = update(
            &mut all_views,
            &mut flows,
            &mut state,
            Message::SetFlowTriggerKind(flow_id, "Widget Event"),
        );

        let trigger = flows[0]
            .graph
            .nodes
            .iter()
            .find(|n| n.is_trigger())
            .expect("trigger node");
        // Default WidgetEvent trigger row has no target, so no flow output ports.
        assert!(trigger.cached_ports_out.is_empty());
        // Old edge was pruned because its trigger source port no longer exists.
        assert!(flows[0].graph.edges.is_empty());
    }

    #[test]
    fn set_row_event_type_preserves_target_when_widget_supports_new_event() {
        let (mut all_views, view_id, widget_id) =
            single_view_with_widget(WidgetType::MouseArea, |widget| {
                widget.properties.mousearea_on_press = true;
                widget.properties.mousearea_on_release = true;
            });

        let mut row = WidgetEventRow::new();
        row.event_type = "on_press".to_string();
        row.target = Some((view_id, widget_id.0));
        let mut flows = vec![AppFlow::new(
            "flow".to_string(),
            FlowTrigger::WidgetEvent {
                rows: vec![row.clone()],
            },
        )];
        let flow_id = flows[0].id;

        let mut state = ActionEditorState::default();
        state.selected_flow_id = Some(flow_id);

        let _ = update(
            &mut all_views,
            &mut flows,
            &mut state,
            Message::SetRowEventType(flow_id, row.id, "on_release".to_string()),
        );

        let FlowTrigger::WidgetEvent { rows } = &flows[0].trigger else {
            panic!("expected widget event trigger");
        };
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].event_type, "on_release");
        assert_eq!(rows[0].target, Some((view_id, widget_id.0)));
    }

    #[test]
    fn set_row_event_type_clears_target_when_widget_lacks_new_event() {
        let (mut all_views, view_id, widget_id) =
            single_view_with_widget(WidgetType::Button, |widget| {
                widget.properties.button_on_press_enabled = true;
            });

        let mut row = WidgetEventRow::new();
        row.event_type = "on_press".to_string();
        row.target = Some((view_id, widget_id.0));
        let mut flows = vec![AppFlow::new(
            "flow".to_string(),
            FlowTrigger::WidgetEvent {
                rows: vec![row.clone()],
            },
        )];
        let flow_id = flows[0].id;

        let mut state = ActionEditorState::default();
        state.selected_flow_id = Some(flow_id);

        let _ = update(
            &mut all_views,
            &mut flows,
            &mut state,
            Message::SetRowEventType(flow_id, row.id, "on_toggle".to_string()),
        );

        let FlowTrigger::WidgetEvent { rows } = &flows[0].trigger else {
            panic!("expected widget event trigger");
        };
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].event_type, "on_toggle");
        assert_eq!(rows[0].target, None);
    }

    #[test]
    fn widget_event_drag_updates_drop_target_and_clears_on_end() {
        let (mut all_views, view_id, widget_id) =
            single_view_with_widget(WidgetType::Button, |widget| {
                widget.properties.button_on_press_enabled = true;
            });

        let mut row_a = WidgetEventRow::new();
        row_a.target = Some((view_id, widget_id.0));
        let mut row_b = WidgetEventRow::new();
        row_b.target = Some((view_id, widget_id.0));

        let mut flows = vec![AppFlow::new(
            "flow".to_string(),
            FlowTrigger::WidgetEvent {
                rows: vec![row_a.clone(), row_b.clone()],
            },
        )];
        let flow_id = flows[0].id;
        let mut state = ActionEditorState::default();
        state.selected_flow_id = Some(flow_id);

        let _ = update(
            &mut all_views,
            &mut flows,
            &mut state,
            Message::StartWidgetEventRowDrag(flow_id, row_a.id),
        );
        assert_eq!(state.widget_event_row_dragging, Some((flow_id, row_a.id)));
        assert_eq!(
            state.widget_event_row_drop_target,
            Some((flow_id, row_a.id))
        );

        let _ = update(
            &mut all_views,
            &mut flows,
            &mut state,
            Message::HoverWidgetEventRow(flow_id, row_b.id),
        );
        assert_eq!(
            state.widget_event_row_drop_target,
            Some((flow_id, row_b.id))
        );

        let _ = update(
            &mut all_views,
            &mut flows,
            &mut state,
            Message::EndWidgetEventRowDrag,
        );
        assert_eq!(state.widget_event_row_dragging, None);
        assert_eq!(state.widget_event_row_drop_target, None);
    }

    #[test]
    fn default_palette_excludes_retired_value_and_legacy_state_call_entries() {
        let state = ActionEditorState::default();
        let ids: Vec<u64> = state.palette.iter().map(|p| p.id).collect();
        assert_eq!(ids, vec![1, 3, 4, 8, 14]);
        let labels: Vec<&str> = state.palette.iter().map(|p| p.label).collect();
        assert_eq!(
            labels,
            vec![
                "State Mutation",
                "Navigate to View",
                "If",
                "Match",
                "Call Flow"
            ]
        );

        for retired in [5_u64, 6, 7, 9, 10, 11, 12, 13, 16] {
            assert!(
                !ids.contains(&retired),
                "retired palette entry {} must be hidden",
                retired
            );
        }
    }

    #[test]
    fn collect_state_fields_includes_generic_overlay_open_field() {
        let (all_views, view_id, widget_id) =
            single_view_with_widget(WidgetType::GenericOverlay, |_| {});

        let fields = collect_state_fields(&all_views);
        let overlay_field = fields.iter().find(|field| {
            field.view_id == view_id
                && matches!(
                    &field.source,
                    StateRefSource::Widget {
                        widget_id: source_widget_id,
                        field_suffix,
                    } if *source_widget_id == widget_id && field_suffix == "_open"
                )
        });

        assert!(
            overlay_field.is_some(),
            "expected generic overlay open state field"
        );
        assert_eq!(
            overlay_field.map(|field| field.field_type.clone()),
            Some(ActionValueType::Bool)
        );
    }

    #[test]
    fn collect_state_fields_includes_date_picker_open_field() {
        let (all_views, view_id, widget_id) =
            single_view_with_widget(WidgetType::DatePicker, |_| {});

        let fields = collect_state_fields(&all_views);
        let date_picker_field = fields.iter().find(|field| {
            field.view_id == view_id
                && matches!(
                    &field.source,
                    StateRefSource::Widget {
                        widget_id: source_widget_id,
                        field_suffix,
                    } if *source_widget_id == widget_id && field_suffix == "_open"
                )
        });

        assert!(
            date_picker_field.is_some(),
            "expected date picker open state field"
        );
        assert_eq!(
            date_picker_field.map(|field| field.field_type.clone()),
            Some(ActionValueType::Bool)
        );
    }

    #[test]
    fn set_call_flow_target_updates_node_with_stable_flow_id() {
        let mut all_views: BTreeMap<Uuid, AppView> = BTreeMap::new();
        let mut flows = vec![AppFlow::new("flow".to_string(), FlowTrigger::Callable)];
        let flow_id = flows[0].id;
        let target_id = Uuid::new_v4();
        let mut state = ActionEditorState::default();
        state.selected_flow_id = Some(flow_id);

        let call_node = ActionNodeData::new(
            2,
            ActionNodeKind::CallFlow { flow_id: None },
            Point::new(280.0, 120.0),
        );
        flows[0].graph.nodes.push(call_node);
        flows[0].graph.z_order.push(2);
        flows[0].graph.next_id = 3;

        let _ = update(
            &mut all_views,
            &mut flows,
            &mut state,
            Message::SetCallFlowTarget(2, target_id),
        );

        let updated = flows[0]
            .graph
            .nodes
            .iter()
            .find(|n| n.id == 2)
            .expect("call node");
        match &updated.kind {
            ActionNodeKind::CallFlow { flow_id } => assert_eq!(*flow_id, Some(target_id)),
            other => panic!("expected CallFlow, got {other:?}"),
        }
    }

    #[test]
    fn build_node_header_exposes_primary_selector_nodes() {
        let trigger_node = ActionNodeData::new(
            1,
            ActionNodeKind::Trigger {
                event_name: "entry".to_string(),
                output_ports: Vec::new(),
            },
            Point::new(0.0, 0.0),
        );
        let navigate_node = ActionNodeData::new(
            2,
            ActionNodeKind::NavigateToView {
                targets: vec![None],
            },
            Point::new(0.0, 0.0),
        );
        let match_node = ActionNodeData::new(
            3,
            ActionNodeKind::Match {
                arms: vec!["arm".to_string()],
                enum_type: None,
            },
            Point::new(0.0, 0.0),
        );
        let state_node = ActionNodeData::new(
            4,
            ActionNodeKind::StateMutation {
                assignments: vec![crate::action_system::node_kinds::StateAssignment {
                    target: None,
                    value_source: ValueSource::Literal(ActionValue::String(String::new())),
                }],
            },
            Point::new(0.0, 0.0),
        );

        assert!(
            build_node_header(
                &trigger_node,
                Vec::new(),
                Some((Uuid::new_v4(), FlowTrigger::Callable))
            )
            .is_some()
        );
        assert!(build_node_header(&navigate_node, Vec::new(), None).is_some());
        assert!(build_node_header(&match_node, vec!["MyEnum".to_string()], None).is_some());
        assert!(build_node_header(&state_node, Vec::new(), None).is_none());
    }

    #[test]
    fn navigate_mode_selected_updates_all_rows_to_app_view_mode() {
        let owner_id = Uuid::new_v4();
        let primary_id = Uuid::new_v4();
        let secondary_id = Uuid::new_v4();
        let (mut all_views, view_ref_id) =
            owner_views_with_single_view_reference(owner_id, primary_id, secondary_id);

        let mut flows = vec![AppFlow::new("flow".to_string(), FlowTrigger::Callable)];
        let flow_id = flows[0].id;
        let mut state = ActionEditorState::default();
        state.selected_flow_id = Some(flow_id);

        let node = ActionNodeData::new(
            2,
            ActionNodeKind::NavigateToView {
                targets: vec![
                    Some(NavigateTarget::ViewReference {
                        owner_view_id: owner_id,
                        widget_id: view_ref_id,
                        target_view_id: primary_id,
                    }),
                    None,
                ],
            },
            Point::new(200.0, 100.0),
        );
        flows[0].graph.nodes.push(node);
        flows[0].graph.z_order.push(2);
        flows[0].graph.next_id = 3;

        let _ = update(
            &mut all_views,
            &mut flows,
            &mut state,
            Message::NavigateModeSelected(2, NavigateModeOption::AppView),
        );

        let updated = flows[0]
            .graph
            .nodes
            .iter()
            .find(|n| n.id == 2)
            .expect("navigate node");
        let ActionNodeKind::NavigateToView { targets } = &updated.kind else {
            panic!("expected NavigateToView");
        };
        assert_eq!(targets.len(), 2);
        assert!(matches!(
            targets[0],
            Some(NavigateTarget::AppView { view_id }) if all_views.contains_key(&view_id)
        ));
        assert!(matches!(
            targets[1],
            Some(NavigateTarget::AppView { view_id }) if all_views.contains_key(&view_id)
        ));
    }

    #[test]
    fn add_navigate_target_uses_current_node_mode_default() {
        let owner_id = Uuid::new_v4();
        let primary_id = Uuid::new_v4();
        let secondary_id = Uuid::new_v4();
        let (mut all_views, view_ref_id) =
            owner_views_with_single_view_reference(owner_id, primary_id, secondary_id);

        let mut flows = vec![AppFlow::new("flow".to_string(), FlowTrigger::Callable)];
        let flow_id = flows[0].id;
        let mut state = ActionEditorState::default();
        state.selected_flow_id = Some(flow_id);

        let node = ActionNodeData::new(
            2,
            ActionNodeKind::NavigateToView {
                targets: vec![Some(NavigateTarget::ViewReference {
                    owner_view_id: owner_id,
                    widget_id: view_ref_id,
                    target_view_id: primary_id,
                })],
            },
            Point::new(200.0, 100.0),
        );
        flows[0].graph.nodes.push(node);
        flows[0].graph.z_order.push(2);
        flows[0].graph.next_id = 3;

        let _ = update(
            &mut all_views,
            &mut flows,
            &mut state,
            Message::AddNavigateTarget(2),
        );

        let updated = flows[0]
            .graph
            .nodes
            .iter()
            .find(|n| n.id == 2)
            .expect("navigate node");
        let ActionNodeKind::NavigateToView { targets } = &updated.kind else {
            panic!("expected NavigateToView");
        };
        assert_eq!(targets.len(), 2);
        assert!(matches!(
            targets[1],
            Some(NavigateTarget::ViewReference { .. })
        ));
    }

    #[test]
    fn conditional_authoring_messages_support_multiple_rows_and_join_mode() {
        let mut all_views: BTreeMap<Uuid, AppView> = BTreeMap::new();
        let mut flows = vec![AppFlow::new("flow".to_string(), FlowTrigger::Callable)];
        let flow_id = flows[0].id;
        let mut state = ActionEditorState::default();
        state.selected_flow_id = Some(flow_id);

        let node = ActionNodeData::new(2, ActionNodeKind::Conditional, Point::new(220.0, 100.0));
        flows[0].graph.nodes.push(node);
        flows[0].graph.z_order.push(2);
        flows[0].graph.next_id = 3;

        let _ = update(
            &mut all_views,
            &mut flows,
            &mut state,
            Message::AddConditionalRow(2),
        );
        let _ = update(
            &mut all_views,
            &mut flows,
            &mut state,
            Message::SetConditionalJoinMode(2, ConditionJoinMode::Any),
        );
        let _ = update(
            &mut all_views,
            &mut flows,
            &mut state,
            Message::SetConditionalSource(2, 0, ExpressionSourceOption::LiteralValue),
        );
        let _ = update(
            &mut all_views,
            &mut flows,
            &mut state,
            Message::SetConditionalOperator(2, 0, CompareOp::Eq),
        );
        let _ = update(
            &mut all_views,
            &mut flows,
            &mut state,
            Message::SetConditionalRhsText(2, 0, "admin@example.com".to_string()),
        );
        let _ = update(
            &mut all_views,
            &mut flows,
            &mut state,
            Message::SetConditionalSource(2, 1, ExpressionSourceOption::LiteralValue),
        );
        let _ = update(
            &mut all_views,
            &mut flows,
            &mut state,
            Message::SetConditionalOperator(2, 1, CompareOp::IsNotEmpty),
        );
        let _ = update(
            &mut all_views,
            &mut flows,
            &mut state,
            Message::RemoveConditionalRow(2, 1),
        );

        let node = flows[0]
            .graph
            .nodes
            .iter()
            .find(|n| n.id == 2)
            .expect("conditional node");
        assert_eq!(node.authored_condition_join, ConditionJoinMode::Any);
        assert_eq!(node.authored_conditions.len(), 1);
        let condition = node.authored_conditions.first().expect("condition row");
        assert!(matches!(condition.lhs, AuthoredValueSource::Literal(_)));
        assert_eq!(condition.operator, CompareOp::Eq);
        assert_eq!(
            condition.rhs_literal,
            ActionValue::String("admin@example.com".to_string())
        );
    }

    #[test]
    fn match_subject_authoring_messages_support_enum_literal_subjects() {
        let mut all_views: BTreeMap<Uuid, AppView> = BTreeMap::new();
        let mut flows = vec![AppFlow::new("flow".to_string(), FlowTrigger::Callable)];
        let flow_id = flows[0].id;
        let mut state = ActionEditorState::default();
        state.selected_flow_id = Some(flow_id);

        let mut node = ActionNodeData::new(
            2,
            ActionNodeKind::Match {
                arms: vec!["Admin".to_string(), "User".to_string()],
                enum_type: None,
            },
            Point::new(220.0, 100.0),
        );
        node.authored_match_subject = Some(AuthoredValueSource::Literal(ActionValue::String(
            String::new(),
        )));
        flows[0].graph.nodes.push(node);
        flows[0].graph.z_order.push(2);
        flows[0].graph.next_id = 3;

        let _ = update(
            &mut all_views,
            &mut flows,
            &mut state,
            Message::SetMatchEnumType(2, "Role".to_string()),
        );
        let _ = update(
            &mut all_views,
            &mut flows,
            &mut state,
            Message::SetMatchSubjectSource(2, ExpressionSourceOption::LiteralValue),
        );
        let _ = update(
            &mut all_views,
            &mut flows,
            &mut state,
            Message::SetMatchSubjectLiteral(2, "User".to_string()),
        );

        let node = flows[0]
            .graph
            .nodes
            .iter()
            .find(|n| n.id == 2)
            .expect("match node");
        assert!(matches!(
            node.authored_match_subject,
            Some(AuthoredValueSource::Literal(ActionValue::EnumVariant {
                ref type_name,
                ref variant,
            })) if type_name == "Role" && variant == "User"
        ));
    }
}

//! The inside of a node: the controls that configure it, and the pins its
//! edges attach to.
//!
//! `flow_editor` kept these apart — a body of fixed-height rows, and a separate
//! table of port offsets that had to agree with it. They are one thing here: a
//! row carries its own pins, and where the row lands is where the edge meets
//! it.
//!
//! Every kind gets a body even when it has no settings, because the pins are
//! part of the body now.

use iced::widget::{button, checkbox, column, container, pick_list, row, text, text_input};
use iced::{Alignment, Element, Length};
use iced_nodegraph::prelude::*;
use uuid::Uuid;

use super::flow::AppFlow;
use super::flow_model::{PortDef, PortSide, PortType};
use super::graph::ActionNodeData;
use super::node_kinds::{
    ActionNodeKind, ActionValue, CompareOp, CompareRhs, NavigateTarget, StateAssignment,
    ValueSource,
};
use super::state_ref::StateFieldRef;

/// Width a control inside a node gets: what is left of the node once the pin
/// gutters and padding are taken out.
const CONTROL_WIDTH: f32 = 150.0;
const LABEL_SIZE: f32 = 11.0;
/// Room for a pin on each side of a row, so a column of rows has its dots in a
/// line whether or not every row has one.
const PIN_GUTTER: f32 = 14.0;

/// A view a `NavigateToView` node can target.
#[derive(Debug, Clone, PartialEq)]
pub struct ViewOption {
    pub id: Uuid,
    pub name: String,
}

impl std::fmt::Display for ViewOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)
    }
}

/// A flow a `CallFlow` node can call.
#[derive(Debug, Clone, PartialEq)]
pub struct FlowOption {
    pub id: Uuid,
    pub name: String,
}

impl std::fmt::Display for FlowOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)
    }
}

/// What the node bodies need to offer choices.
///
/// Gathered once per frame rather than per node — every picker draws from the
/// same lists.
#[derive(Debug, Clone, Default)]
pub struct NodeContext {
    pub state_fields: Vec<StateFieldRef>,
    pub views: Vec<ViewOption>,
    pub flows: Vec<FlowOption>,
}

impl NodeContext {
    pub fn gather(
        views: &std::collections::BTreeMap<Uuid, crate::data_structures::types::types::AppView>,
        flows: &[AppFlow],
        state_fields: Vec<StateFieldRef>,
    ) -> Self {
        Self {
            state_fields,
            views: views
                .values()
                .map(|view| ViewOption {
                    id: view.id,
                    name: view.name.clone(),
                })
                .collect(),
            flows: flows
                .iter()
                .map(|flow| FlowOption {
                    id: flow.id,
                    name: flow.name.clone(),
                })
                .collect(),
        }
    }
}

/// An edit to one node.
///
/// Every variant names the node it applies to: bodies are built per node, but
/// the messages all come back through one channel.
#[derive(Debug, Clone)]
pub enum NodeMessage {
    SetString(u64, String),
    SetNumber(u64, String),
    SetBool(u64, bool),
    SetFormula(u64, String),

    SetCompareOp(u64, CompareOp),
    SetCompareRhsMode(u64, CompareRhs),
    SetCompareLiteral(u64, String),

    /// Adds an arm to a `Match`, or a target to a `NavigateToView`.
    AddRow(u64),
    RemoveRow(u64, usize),
    SetMatchArm(u64, usize, String),
    SetNavigateTarget(u64, usize, Uuid),

    AddAssignment(u64),
    RemoveAssignment(u64, usize),
    SetAssignmentTarget(u64, usize, StateFieldRef),
    SetAssignmentSource(u64, usize, SourceKind),
    SetAssignmentLiteral(u64, usize, String),

    SetCallFlow(u64, Uuid),
}

/// Where an assignment's value comes from.
///
/// Mirrors [`ValueSource`] without its payload, so it can drive a pick list.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceKind {
    Literal,
    FromPort,
}

impl SourceKind {
    const ALL: &'static [Self] = &[Self::Literal, Self::FromPort];

    fn of(source: &ValueSource) -> Self {
        match source {
            ValueSource::Literal(_) | ValueSource::StateField(_) => Self::Literal,
            ValueSource::FromPort => Self::FromPort,
        }
    }
}

impl std::fmt::Display for SourceKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal => f.write_str("Value"),
            Self::FromPort => f.write_str("From port"),
        }
    }
}

/// Renders a value as the text its editor shows.
fn literal_text(value: &ActionValue) -> String {
    match value {
        ActionValue::String(value) => value.clone(),
        ActionValue::Number(value) => value.to_string(),
        ActionValue::Bool(value) => value.to_string(),
        ActionValue::EnumVariant { type_name, variant } => format!("{type_name}::{variant}"),
    }
}

/// A pin, drawn as the dot an edge meets.
fn pin_for<'a>(port: &PortDef) -> Element<'a, NodeMessage> {
    // Flow and data read differently at a glance, which matters on a node that
    // has both on the same side.
    let glyph = match port.kind {
        PortType::Flow => "\u{25B6}",
        PortType::Data(_) => "\u{25CF}",
    };

    let dot = text(glyph).size(10);

    match port.side {
        PortSide::Input => pin!(Left, port.id.0, dot, Input, port.kind.clone()).into(),
        PortSide::Output => pin!(Right, port.id.0, dot, Output, port.kind.clone()).into(),
    }
}

/// One row of a node: whatever it shows, with the pins that belong to it.
fn body_row<'a>(
    left: Option<&PortDef>,
    content: impl Into<Element<'a, NodeMessage>>,
    right: Option<&PortDef>,
) -> Element<'a, NodeMessage> {
    let gutter = |port: Option<&PortDef>| -> Element<'a, NodeMessage> {
        match port {
            Some(port) => container(pin_for(port))
                .width(Length::Fixed(PIN_GUTTER))
                .into(),
            None => container(text(""))
                .width(Length::Fixed(PIN_GUTTER))
                .height(Length::Fixed(14.0))
                .into(),
        }
    };

    row![gutter(left), content.into(), gutter(right)]
        .spacing(6)
        .align_y(Alignment::Center)
        .into()
}

/// A row that is only a named pin — for ports that need labelling and nothing
/// else, like a logic node's inputs.
fn labelled_pin<'a>(port: &PortDef) -> Element<'a, NodeMessage> {
    let label = text(port.label.clone()).size(LABEL_SIZE).width(Length::Fill);

    match port.side {
        PortSide::Input => body_row(Some(port), label, None),
        PortSide::Output => body_row(None, label, Some(port)),
    }
}

/// Ports from `skip_in` / `skip_out` onward, one per line.
fn remaining_pins<'a>(
    node: &'a ActionNodeData,
    skip_in: usize,
    skip_out: usize,
) -> Vec<Element<'a, NodeMessage>> {
    node.cached_ports_in
        .iter()
        .skip(skip_in)
        .chain(node.cached_ports_out.iter().skip(skip_out))
        .map(labelled_pin)
        .collect()
}

/// How many ports a kind's own rows account for, as `(inputs, outputs)`.
///
/// Whatever is left over is drawn underneath by [`body`], so a port can never
/// end up with no dot for its edge to meet.
fn covered_ports(node: &ActionNodeData) -> Option<(usize, usize)> {
    Some(match &node.kind {
        ActionNodeKind::StringLiteral { .. }
        | ActionNodeKind::NumberLiteral { .. }
        | ActionNodeKind::BoolLiteral { .. }
        | ActionNodeKind::Expression { .. } => (0, 1),

        ActionNodeKind::Compare { rhs, .. } => (
            match rhs {
                CompareRhs::FromPort => 2,
                CompareRhs::Literal => 1,
            },
            1,
        ),

        ActionNodeKind::Match { arms, .. } => (1, arms.len() + 1),

        ActionNodeKind::NavigateToView { targets } => (targets.len(), targets.len()),

        ActionNodeKind::StateMutation { assignments }
        | ActionNodeKind::UpdateState { assignments } => {
            // One flow input, and a data input for every assignment --
            // `rebuild_ports` makes one per assignment whether or not it is
            // wired, so the pin is always there to attach to.
            (1 + assignments.len(), 1)
        }

        ActionNodeKind::CallFlow { .. } => (1, 1),

        // Everything else is drawn as its pins and nothing more, which covers
        // them all by definition.
        _ => return None,
    })
}

/// The body of one node: its controls, with its pins on them.
pub fn body<'a>(node: &'a ActionNodeData, ctx: &'a NodeContext) -> Element<'a, NodeMessage> {
    let id = node.id;
    let input = |slot: usize| node.cached_ports_in.get(slot);
    let output = |slot: usize| node.cached_ports_out.get(slot);

    let mut rows: Vec<Element<'a, NodeMessage>> = match &node.kind {
        ActionNodeKind::StringLiteral { value } => vec![body_row(
            None,
            text_input("text", value)
                .on_input(move |value| NodeMessage::SetString(id, value))
                .size(LABEL_SIZE)
                .width(Length::Fixed(CONTROL_WIDTH)),
            output(0),
        )],

        ActionNodeKind::NumberLiteral { value } => vec![body_row(
            None,
            text_input("0", value.to_string())
                .on_input(move |value| NodeMessage::SetNumber(id, value))
                .size(LABEL_SIZE)
                .width(Length::Fixed(CONTROL_WIDTH)),
            output(0),
        )],

        ActionNodeKind::BoolLiteral { value } => vec![body_row(
            None,
            checkbox(*value)
                .label("true")
                .text_size(LABEL_SIZE)
                .on_toggle(move |value| NodeMessage::SetBool(id, value)),
            output(0),
        )],

        ActionNodeKind::Expression { formula } => vec![body_row(
            None,
            text_input("a + b", formula)
                .on_input(move |value| NodeMessage::SetFormula(id, value))
                .size(LABEL_SIZE)
                .width(Length::Fixed(CONTROL_WIDTH)),
            output(0),
        )],

        ActionNodeKind::Compare {
            operator,
            rhs,
            rhs_literal,
        } => {
            let mut rows = vec![
                body_row(
                    input(0),
                    pick_list(Some(operator.clone()), CompareOp::all(), ToString::to_string)
                        .on_select(move |op| NodeMessage::SetCompareOp(id, op))
                        .text_size(LABEL_SIZE)
                        .width(Length::Fixed(CONTROL_WIDTH)),
                    output(0),
                ),
                body_row(
                    None,
                    pick_list(
                        Some(rhs.clone()),
                        vec![CompareRhs::Literal, CompareRhs::FromPort],
                        ToString::to_string,
                    )
                    .on_select(move |mode| NodeMessage::SetCompareRhsMode(id, mode))
                    .text_size(LABEL_SIZE)
                    .width(Length::Fixed(CONTROL_WIDTH)),
                    None,
                ),
            ];

            // The second input exists only when comparing against a wired
            // value, so the literal editor takes its place otherwise.
            rows.push(match rhs {
                CompareRhs::FromPort => body_row(
                    input(1),
                    text("compare to").size(LABEL_SIZE).width(Length::Fill),
                    None,
                ),
                CompareRhs::Literal => body_row(
                    None,
                    text_input("value", literal_text(rhs_literal))
                        .on_input(move |value| NodeMessage::SetCompareLiteral(id, value))
                        .size(LABEL_SIZE)
                        .width(Length::Fixed(CONTROL_WIDTH)),
                    None,
                ),
            });

            rows
        }

        ActionNodeKind::Match { arms, enum_type } => {
            let mut rows = vec![body_row(
                input(0),
                text(enum_type.clone().unwrap_or_else(|| "match on".to_string()))
                    .size(LABEL_SIZE)
                    .width(Length::Fill),
                None,
            )];

            // Arm i's flow output is slot i and the default is the slot after
            // the last arm, which is the order `rebuild_ports` lays them out.
            for (index, arm) in arms.iter().enumerate() {
                rows.push(body_row(
                    None,
                    row![
                        text_input("arm", arm)
                            .on_input(move |value| NodeMessage::SetMatchArm(id, index, value))
                            .size(LABEL_SIZE)
                            .width(Length::Fixed(CONTROL_WIDTH - 24.0)),
                        button(text("\u{2715}").size(9))
                            .on_press(NodeMessage::RemoveRow(id, index))
                            .style(button::text),
                    ]
                    .spacing(2)
                    .align_y(Alignment::Center),
                    output(index),
                ));
            }

            rows.push(body_row(
                None,
                button(text("+ arm").size(LABEL_SIZE))
                    .on_press(NodeMessage::AddRow(id))
                    .style(button::text),
                None,
            ));

            rows.push(body_row(
                None,
                text("default").size(LABEL_SIZE).width(Length::Fill),
                output(arms.len()),
            ));

            rows
        }

        ActionNodeKind::NavigateToView { targets } => {
            let mut rows: Vec<Element<'a, NodeMessage>> = targets
                .iter()
                .enumerate()
                .map(|(index, target)| {
                    // Only an app-view target is offered here. A ViewReference
                    // target needs the widget picked as well, which is the
                    // next piece.
                    let selected = target.as_ref().and_then(|target| match target {
                        NavigateTarget::AppView { view_id } => ctx
                            .views
                            .iter()
                            .find(|view| view.id == *view_id)
                            .cloned(),
                        NavigateTarget::ViewReference { .. } => None,
                    });

                    body_row(
                        input(index),
                        pick_list(selected, ctx.views.clone(), ToString::to_string)
                            .on_select(move |view: ViewOption| {
                                NodeMessage::SetNavigateTarget(id, index, view.id)
                            })
                            .placeholder("view")
                            .text_size(LABEL_SIZE)
                            .width(Length::Fixed(CONTROL_WIDTH)),
                        output(index),
                    )
                })
                .collect();

            rows.push(body_row(
                None,
                button(text("+ target").size(LABEL_SIZE))
                    .on_press(NodeMessage::AddRow(id))
                    .style(button::text),
                None,
            ));

            rows
        }

        ActionNodeKind::StateMutation { assignments }
        | ActionNodeKind::UpdateState { assignments } => {
            let mut rows = vec![body_row(input(0), text("").width(Length::Fill), output(0))];

            for (index, assignment) in assignments.iter().enumerate() {
                rows.push(assignment_row(id, index, assignment, ctx, node));
            }

            rows.push(body_row(
                None,
                button(text("+ assignment").size(LABEL_SIZE))
                    .on_press(NodeMessage::AddAssignment(id))
                    .style(button::text),
                None,
            ));

            rows
        }

        ActionNodeKind::CallFlow { flow_id } => {
            let selected = flow_id
                .and_then(|flow_id| ctx.flows.iter().find(|flow| flow.id == flow_id))
                .cloned();

            vec![body_row(
                input(0),
                pick_list(selected, ctx.flows.clone(), ToString::to_string)
                    .on_select(move |flow: FlowOption| NodeMessage::SetCallFlow(id, flow.id))
                    .placeholder("flow")
                    .text_size(LABEL_SIZE)
                    .width(Length::Fixed(CONTROL_WIDTH)),
                output(0),
            )]
        }

        // Everything else is its pins and nothing more.
        _ => remaining_pins(node, 0, 0),
    };

    if let Some((inputs, outputs)) = covered_ports(node) {
        rows.extend(remaining_pins(node, inputs, outputs));
    }

    column(rows).spacing(4).padding([4, 6]).into()
}

/// One assignment: what it writes, where the value comes from, and the value.
fn assignment_row<'a>(
    id: u64,
    index: usize,
    assignment: &'a StateAssignment,
    ctx: &'a NodeContext,
    node: &'a ActionNodeData,
) -> Element<'a, NodeMessage> {
    let source = SourceKind::of(&assignment.value_source);

    let target = pick_list(
        assignment.target.clone(),
        ctx.state_fields.clone(),
        ToString::to_string,
    )
    .on_select(move |field| NodeMessage::SetAssignmentTarget(id, index, field))
    .placeholder("field")
    .text_size(LABEL_SIZE)
    .width(Length::Fixed(CONTROL_WIDTH));

    let kind = pick_list(Some(source), SourceKind::ALL, ToString::to_string)
        .on_select(move |kind| NodeMessage::SetAssignmentSource(id, index, kind))
        .text_size(LABEL_SIZE)
        .width(Length::Fixed(CONTROL_WIDTH));

    let value: Element<'a, NodeMessage> = match &assignment.value_source {
        ValueSource::Literal(value) => text_input("value", literal_text(value))
            .on_input(move |value| NodeMessage::SetAssignmentLiteral(id, index, value))
            .size(LABEL_SIZE)
            .width(Length::Fixed(CONTROL_WIDTH))
            .into(),
        ValueSource::StateField(field) => {
            text(field.display_name.clone()).size(LABEL_SIZE).into()
        }
        ValueSource::FromPort => text("wired").size(LABEL_SIZE).into(),
    };

    // Every assignment gets a data input from `rebuild_ports`, after the flow
    // input -- so the pin is shown whatever the source is, and the selector
    // above says whether the wire is what gets used.
    let data_in = node.cached_ports_in.get(1 + index);

    container(
        column![
            body_row(None, target, None),
            body_row(None, kind, None),
            body_row(data_in, value, None),
            body_row(
                None,
                button(text("remove").size(LABEL_SIZE))
                    .on_press(NodeMessage::RemoveAssignment(id, index))
                    .style(button::text),
                None,
            ),
        ]
        .spacing(2),
    )
    .padding([2, 0])
    .into()
}


/// Applies an edit to the node it names.
///
/// Returns whether anything changed, so the caller knows when to regenerate.
/// Ports are rebuilt whenever an edit can change how many there are — adding a
/// match arm, or switching an assignment to read from a port.
pub fn update(graph: &mut super::graph::ActionGraph, message: NodeMessage) -> bool {
    let id = match &message {
        NodeMessage::SetString(id, _)
        | NodeMessage::SetNumber(id, _)
        | NodeMessage::SetBool(id, _)
        | NodeMessage::SetFormula(id, _)
        | NodeMessage::SetCompareOp(id, _)
        | NodeMessage::SetCompareRhsMode(id, _)
        | NodeMessage::SetCompareLiteral(id, _)
        | NodeMessage::AddRow(id)
        | NodeMessage::RemoveRow(id, _)
        | NodeMessage::SetMatchArm(id, _, _)
        | NodeMessage::SetNavigateTarget(id, _, _)
        | NodeMessage::AddAssignment(id)
        | NodeMessage::RemoveAssignment(id, _)
        | NodeMessage::SetAssignmentTarget(id, _, _)
        | NodeMessage::SetAssignmentSource(id, _, _)
        | NodeMessage::SetAssignmentLiteral(id, _, _)
        | NodeMessage::SetCallFlow(id, _) => *id,
    };

    let Some(node) = graph.nodes.iter_mut().find(|node| node.id == id) else {
        return false;
    };

    // Set when the edit can change the port list, which is also when an edge
    // to a port that no longer exists has to go.
    let mut ports_changed = false;

    match (message, &mut node.kind) {
        (NodeMessage::SetString(_, value), ActionNodeKind::StringLiteral { value: slot }) => {
            *slot = value;
        }

        (NodeMessage::SetNumber(_, raw), ActionNodeKind::NumberLiteral { value }) => {
            // An unparseable entry leaves the value alone rather than snapping
            // to zero halfway through typing "1.5".
            if let Ok(parsed) = raw.trim().parse::<f64>() {
                *value = parsed;
            } else if raw.trim().is_empty() {
                *value = 0.0;
            }
        }

        (NodeMessage::SetBool(_, value), ActionNodeKind::BoolLiteral { value: slot }) => {
            *slot = value;
        }

        (NodeMessage::SetFormula(_, value), ActionNodeKind::Expression { formula }) => {
            *formula = value;
        }

        (NodeMessage::SetCompareOp(_, op), ActionNodeKind::Compare { operator, .. }) => {
            *operator = op;
        }

        (NodeMessage::SetCompareRhsMode(_, mode), ActionNodeKind::Compare { rhs, .. }) => {
            *rhs = mode;
            // Comparing against a wired value adds an input; a literal takes it
            // away again.
            ports_changed = true;
        }

        (NodeMessage::SetCompareLiteral(_, raw), ActionNodeKind::Compare { rhs_literal, .. }) => {
            *rhs_literal = retype_literal(rhs_literal, raw);
        }

        (NodeMessage::AddRow(_), ActionNodeKind::Match { arms, .. }) => {
            arms.push(format!("arm {}", arms.len() + 1));
            ports_changed = true;
        }

        (NodeMessage::RemoveRow(_, index), ActionNodeKind::Match { arms, .. }) => {
            if index >= arms.len() {
                return false;
            }

            arms.remove(index);
            ports_changed = true;
        }

        (NodeMessage::SetMatchArm(_, index, value), ActionNodeKind::Match { arms, .. }) => {
            let Some(arm) = arms.get_mut(index) else {
                return false;
            };

            *arm = value;
        }

        (NodeMessage::AddRow(_), ActionNodeKind::NavigateToView { targets }) => {
            targets.push(None);
            ports_changed = true;
        }

        (
            NodeMessage::SetNavigateTarget(_, index, view_id),
            ActionNodeKind::NavigateToView { targets },
        ) => {
            let Some(target) = targets.get_mut(index) else {
                return false;
            };

            *target = Some(NavigateTarget::AppView { view_id });
        }

        (
            NodeMessage::AddAssignment(_),
            ActionNodeKind::StateMutation { assignments }
            | ActionNodeKind::UpdateState { assignments },
        ) => {
            assignments.push(StateAssignment {
                target: None,
                value_source: ValueSource::Literal(ActionValue::String(String::new())),
            });

            // Every assignment carries a data input.
            ports_changed = true;
        }

        (
            NodeMessage::RemoveAssignment(_, index),
            ActionNodeKind::StateMutation { assignments }
            | ActionNodeKind::UpdateState { assignments },
        ) => {
            if index >= assignments.len() {
                return false;
            }

            assignments.remove(index);
            ports_changed = true;
        }

        (
            NodeMessage::SetAssignmentTarget(_, index, field),
            ActionNodeKind::StateMutation { assignments }
            | ActionNodeKind::UpdateState { assignments },
        ) => {
            let Some(assignment) = assignments.get_mut(index) else {
                return false;
            };

            assignment.target = Some(field);
        }

        (
            NodeMessage::SetAssignmentSource(_, index, kind),
            ActionNodeKind::StateMutation { assignments }
            | ActionNodeKind::UpdateState { assignments },
        ) => {
            let Some(assignment) = assignments.get_mut(index) else {
                return false;
            };

            assignment.value_source = match kind {
                SourceKind::FromPort => ValueSource::FromPort,
                SourceKind::Literal => ValueSource::Literal(ActionValue::String(String::new())),
            };

            // No port change: the data input exists either way, and the source
            // only decides whether what is wired to it gets used.
        }

        (
            NodeMessage::SetAssignmentLiteral(_, index, raw),
            ActionNodeKind::StateMutation { assignments }
            | ActionNodeKind::UpdateState { assignments },
        ) => {
            let Some(assignment) = assignments.get_mut(index) else {
                return false;
            };

            let ValueSource::Literal(value) = &assignment.value_source else {
                return false;
            };

            assignment.value_source = ValueSource::Literal(retype_literal(value, raw));
        }

        (NodeMessage::SetCallFlow(_, flow), ActionNodeKind::CallFlow { flow_id }) => {
            *flow_id = Some(flow);
        }

        // A message for a kind that does not carry it: a stale frame, or a node
        // whose kind changed underneath. Nothing to do.
        _ => return false,
    }

    if ports_changed {
        node.rebuild_ports();
        prune_dangling_edges(graph);
    }

    true
}

/// Keeps a literal's type while replacing its text.
///
/// Editing a number should leave it a number, not turn it into a string the
/// moment a character lands.
fn retype_literal(current: &ActionValue, raw: String) -> ActionValue {
    match current {
        ActionValue::Number(previous) => {
            ActionValue::Number(raw.trim().parse().unwrap_or(*previous))
        }
        ActionValue::Bool(previous) => ActionValue::Bool(raw.trim().parse().unwrap_or(*previous)),
        ActionValue::String(_) | ActionValue::EnumVariant { .. } => ActionValue::String(raw),
    }
}

/// Drops edges whose port no longer exists.
///
/// Removing a match arm, or unwiring an assignment, takes its port with it —
/// and an edge to a port that is gone would be carried into code generation.
fn prune_dangling_edges(graph: &mut super::graph::ActionGraph) {
    let keep: Vec<bool> = graph
        .edges
        .iter()
        .map(|edge| {
            port_exists(graph, edge.from_node, edge.from_port)
                && port_exists(graph, edge.to_node, edge.to_port)
        })
        .collect();

    let mut keep = keep.into_iter();
    graph.edges.retain(|_| keep.next().unwrap_or(true));
}

fn port_exists(graph: &super::graph::ActionGraph, node_id: u64, port: u64) -> bool {
    graph
        .nodes
        .iter()
        .find(|node| node.id == node_id)
        .is_some_and(|node| {
            node.cached_ports_in
                .iter()
                .chain(node.cached_ports_out.iter())
                .any(|candidate| candidate.id.0 == port)
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::action_system::graph::ActionGraph;

    /// A graph holding one node of the given kind, and that node's id.
    fn with_node(kind: ActionNodeKind) -> (ActionGraph, u64) {
        let mut graph = ActionGraph::new_named_action();
        let id = graph.next_id;

        // `new` rebuilds the ports, which is what the body reads.
        let node = super::super::graph::ActionNodeData::new(id, kind, iced::Point::ORIGIN);

        graph.next_id += 1;
        graph.z_order.push(id);
        graph.nodes.push(node);

        (graph, id)
    }

    fn kind_of(graph: &ActionGraph, id: u64) -> &ActionNodeKind {
        &graph.nodes.iter().find(|node| node.id == id).unwrap().kind
    }

    fn ports(graph: &ActionGraph, id: u64) -> (usize, usize) {
        let node = graph.nodes.iter().find(|node| node.id == id).unwrap();
        (node.cached_ports_in.len(), node.cached_ports_out.len())
    }

    #[test]
    fn a_number_keeps_its_last_value_while_being_typed_into() {
        let (mut graph, id) = with_node(ActionNodeKind::NumberLiteral { value: 1.5 });

        // "-" is the first character of "-5" and parses as nothing.
        assert!(update(&mut graph, NodeMessage::SetNumber(id, "-".into())));
        assert!(matches!(
            kind_of(&graph, id),
            ActionNodeKind::NumberLiteral { value } if *value == 1.5
        ));

        assert!(update(&mut graph, NodeMessage::SetNumber(id, "1.25".into())));
        assert!(matches!(
            kind_of(&graph, id),
            ActionNodeKind::NumberLiteral { value } if *value == 1.25
        ));
    }

    #[test]
    fn adding_a_match_arm_adds_its_output() {
        let (mut graph, id) = with_node(ActionNodeKind::Match {
            arms: vec!["one".into()],
            enum_type: None,
        });

        let (_, outputs) = ports(&graph, id);

        assert!(update(&mut graph, NodeMessage::AddRow(id)));

        let (_, after) = ports(&graph, id);
        assert_eq!(after, outputs + 1, "an arm should bring its own output");
    }

    #[test]
    fn removing_a_match_arm_takes_its_edges_with_it() {
        let (mut graph, id) = with_node(ActionNodeKind::Match {
            arms: vec!["one".into(), "two".into()],
            enum_type: None,
        });

        // Wire something to the last arm's output, then take the arm away.
        let doomed = graph
            .nodes
            .iter()
            .find(|node| node.id == id)
            .unwrap()
            .cached_ports_out
            .last()
            .unwrap()
            .id
            .0;

        graph.edges.push(super::super::graph::ActionEdge {
            from_node: id,
            from_port: doomed,
            to_node: graph.nodes[0].id,
            to_port: 0,
        });

        assert!(update(&mut graph, NodeMessage::RemoveRow(id, 1)));

        assert!(
            !graph.edges.iter().any(|edge| edge.from_port == doomed),
            "an edge outlived the port it was attached to",
        );
    }

    #[test]
    fn an_assignment_brings_a_data_input_with_it() {
        let (mut graph, id) = with_node(ActionNodeKind::StateMutation {
            assignments: vec![StateAssignment {
                target: None,
                value_source: ValueSource::Literal(ActionValue::String(String::new())),
            }],
        });

        let (inputs, _) = ports(&graph, id);

        assert!(update(&mut graph, NodeMessage::AddAssignment(id)));
        assert_eq!(
            ports(&graph, id).0,
            inputs + 1,
            "an assignment needs somewhere to wire a value from",
        );

        assert!(update(&mut graph, NodeMessage::RemoveAssignment(id, 1)));
        assert_eq!(ports(&graph, id).0, inputs);

        // The source only decides whether the wire is used, so it moves no
        // ports either way.
        assert!(update(
            &mut graph,
            NodeMessage::SetAssignmentSource(id, 0, SourceKind::FromPort),
        ));
        assert_eq!(ports(&graph, id).0, inputs);
    }

    #[test]
    fn editing_a_literal_keeps_its_type() {
        let (mut graph, id) = with_node(ActionNodeKind::Compare {
            operator: CompareOp::Eq,
            rhs: CompareRhs::Literal,
            rhs_literal: ActionValue::Number(1.0),
        });

        assert!(update(&mut graph, NodeMessage::SetCompareLiteral(id, "7".into())));

        assert!(
            matches!(
                kind_of(&graph, id),
                ActionNodeKind::Compare { rhs_literal: ActionValue::Number(value), .. }
                    if *value == 7.0
            ),
            "a number's editor should not turn it into a string",
        );
    }

    #[test]
    fn an_edit_meant_for_another_kind_is_ignored() {
        let (mut graph, id) = with_node(ActionNodeKind::Conditional);

        assert!(!update(&mut graph, NodeMessage::SetString(id, "x".into())));
        assert!(!update(&mut graph, NodeMessage::SetCallFlow(id, uuid::Uuid::new_v4())));
    }

    #[test]
    fn an_edit_for_a_node_that_is_gone_is_ignored() {
        let (mut graph, _) = with_node(ActionNodeKind::Conditional);

        assert!(!update(&mut graph, NodeMessage::AddRow(9_999)));
    }
}

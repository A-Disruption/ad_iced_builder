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
    ActionNodeKind, ActionValue, AuthoredCondition, AuthoredValueSource, CompareOp, CompareRhs,
    ConditionJoinMode, NavigateTarget, StateAssignment, ValueSource,
};
use super::state_ref::{ActionValueType, StateFieldRef};
use crate::data_structures::types::types::WidgetId;
use std::collections::BTreeMap;

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
    /// Every widget, with the events it emits — for the trigger rows.
    pub widgets: Vec<WidgetOption>,
    /// The `ViewReference` widgets a navigate target can switch.
    pub view_references: Vec<ViewReferenceOption>,
    /// The project's enums, for `EnumLiteral` and enum-driven `Match`.
    pub enums: Vec<EnumOption>,
    /// What an authored `If` condition can read: any state field, or one of
    /// the trigger's outputs.
    ///
    /// A `Match` subject can also be a literal, which it adds itself — an
    /// `If` comparing one constant against another would be no condition at
    /// all.
    pub condition_sources: Vec<ConditionSource>,
    /// The rows of the selected flow's widget-event trigger, when it has one.
    ///
    /// The trigger node draws these, but they belong to the flow — so they
    /// arrive here rather than through the node.
    pub trigger_rows: Option<Vec<crate::action_system::flow::WidgetEventRow>>,
}

impl NodeContext {
    pub fn gather(
        views: &BTreeMap<Uuid, crate::data_structures::types::types::AppView>,
        flows: &[AppFlow],
        state_fields: Vec<StateFieldRef>,
        type_system: &crate::enum_builder::TypeSystem,
        selected_flow: Option<&AppFlow>,
    ) -> Self {
        // An authored condition can read the trigger's outputs, which are
        // whatever the flow it belongs to fires with.
        let trigger_ports = selected_flow
            .and_then(|flow| {
                flow.graph.nodes.iter().find_map(|node| match &node.kind {
                    ActionNodeKind::Trigger { output_ports, .. } => Some(output_ports.clone()),
                    _ => None,
                })
            })
            .unwrap_or_default();

        let condition_sources = state_fields
            .iter()
            .cloned()
            .map(ConditionSource::StateField)
            .chain(
                trigger_ports
                    .into_iter()
                    .map(|port| ConditionSource::TriggerInput {
                        name: port.name,
                        value_type: port.value_type,
                    }),
            )
            .collect();

        Self {
            condition_sources,
            state_fields,
            trigger_rows: selected_flow.and_then(|flow| match &flow.trigger {
                crate::action_system::flow::FlowTrigger::WidgetEvent { rows } => {
                    Some(rows.clone())
                }
                _ => None,
            }),
            widgets: collect_widget_options(views),
            view_references: collect_view_reference_options(views),
            enums: type_system
                .all_enums()
                .into_iter()
                .map(|def| EnumOption {
                    name: def.name.clone(),
                    variants: def.variants.iter().map(|v| v.name.clone()).collect(),
                })
                .collect(),
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

    SetEnum(u64, String),
    SetEnumVariant(u64, String),

    /// Points a navigate target at a `ViewReference` widget instead of the
    /// app view.
    SetNavigateViewReference(u64, usize, Uuid, WidgetId, Uuid),

    // An `If`'s conditions, authored in the node instead of wired in.
    AddCondition(u64),
    RemoveCondition(u64, usize),
    SetConditionJoin(u64, ConditionJoinMode),
    SetConditionSource(u64, usize, ConditionSource),
    SetConditionOp(u64, usize, CompareOp),
    /// The right-hand side as typed or picked; read back as the left-hand
    /// side's type.
    SetConditionRhs(u64, usize, String),

    /// Switches a `Match` onto an enum, whose variants become its arms.
    SetMatchEnum(u64, EnumOption),
    SetMatchSubject(u64, ConditionSource),
    SetMatchSubjectLiteral(u64, String),

    /// An edit to the flow's trigger, which the trigger node shows but does
    /// not own.
    Trigger(TriggerMessage),
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

        // Flow in and the subject value in; an output per arm and a default.
        ActionNodeKind::Match { arms, .. } => (2, arms.len() + 1),

        // Flow in and the wired condition in; the two branches out.
        ActionNodeKind::Conditional => (2, 2),

        ActionNodeKind::NavigateToView { targets } => (targets.len(), targets.len()),

        ActionNodeKind::StateMutation { assignments }
        | ActionNodeKind::UpdateState { assignments } => {
            // One flow input, and a data input for every assignment --
            // `rebuild_ports` makes one per assignment whether or not it is
            // wired, so the pin is always there to attach to.
            (1 + assignments.len(), 1)
        }

        ActionNodeKind::CallFlow { .. } => (1, 1),

        ActionNodeKind::EnumLiteral { .. } => (0, 1),


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
            let mut rows = match_subject_rows(node, enum_type, arms, ctx);

            // Arm i's flow output is slot i and the default is the slot after
            // the last arm, which is the order `rebuild_ports` lays them out.
            for (index, arm) in arms.iter().enumerate() {
                // An enum match's arms are its variants: naming them here
                // would only let them drift out of step with the enum.
                let content: Element<'a, NodeMessage> = if enum_type.is_some() {
                    text(arm).size(LABEL_SIZE).width(Length::Fill).into()
                } else {
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
                    .align_y(Alignment::Center)
                    .into()
                };

                rows.push(body_row(None, content, output(index)));
            }

            if enum_type.is_none() {
                rows.push(body_row(
                    None,
                    button(text("+ arm").size(LABEL_SIZE))
                        .on_press(NodeMessage::AddRow(id))
                        .style(button::text),
                    None,
                ));
            }

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
                .map(|(index, target)| navigate_target_row(id, index, target, ctx, node))
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

        ActionNodeKind::EnumLiteral {
            enum_name,
            variant,
        } => enum_literal_rows(id, enum_name, variant, ctx, node),

        ActionNodeKind::Conditional => conditional_rows(node, ctx),

        // The rows belong to the flow's trigger; the node only shows them.
        ActionNodeKind::Trigger { .. } => match &ctx.trigger_rows {
            Some(rows) => trigger_rows(rows, node, ctx),
            None => remaining_pins(node, 0, 0),
        },

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
        | NodeMessage::SetCallFlow(id, _)
        | NodeMessage::SetEnum(id, _)
        | NodeMessage::SetEnumVariant(id, _)
        | NodeMessage::SetNavigateViewReference(id, _, _, _, _)
        | NodeMessage::AddCondition(id)
        | NodeMessage::RemoveCondition(id, _)
        | NodeMessage::SetConditionJoin(id, _)
        | NodeMessage::SetConditionSource(id, _, _)
        | NodeMessage::SetConditionOp(id, _, _)
        | NodeMessage::SetConditionRhs(id, _, _)
        | NodeMessage::SetMatchEnum(id, _)
        | NodeMessage::SetMatchSubject(id, _)
        | NodeMessage::SetMatchSubjectLiteral(id, _) => *id,

        // The trigger belongs to the flow, and is applied by
        // `update_trigger` rather than here.
        NodeMessage::Trigger(_) => return false,
    };

    let Some(node) = graph.nodes.iter_mut().find(|node| node.id == id) else {
        return false;
    };

    // Set when the edit can change the port list, which is also when an edge
    // to a port that no longer exists has to go.
    let mut ports_changed = false;

    // The authored If/Match fields hang off the node rather than its kind, so
    // they cannot be matched alongside the rest.
    let message = match update_authored(node, message) {
        Authored::Applied { ports_changed } => {
            if ports_changed {
                node.rebuild_ports();
                prune_dangling_edges(graph);
            }

            return true;
        }
        Authored::Ignored => return false,
        Authored::NotMine(message) => message,
    };

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

        (NodeMessage::SetEnum(_, name), ActionNodeKind::EnumLiteral { enum_name, variant }) => {
            // A variant of the old enum means nothing under the new one.
            if enum_name.as_deref() != Some(name.as_str()) {
                *variant = None;
            }

            *enum_name = Some(name);
        }

        (NodeMessage::SetEnumVariant(_, name), ActionNodeKind::EnumLiteral { variant, .. }) => {
            *variant = Some(name);
        }

        (
            NodeMessage::SetNavigateViewReference(_, index, owner, widget, view),
            ActionNodeKind::NavigateToView { targets },
        ) => {
            let Some(target) = targets.get_mut(index) else {
                return false;
            };

            *target = Some(NavigateTarget::ViewReference {
                owner_view_id: owner,
                widget_id: widget,
                target_view_id: view,
            });
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

    /// A custom state field of the given type, for a condition to read.
    fn state_field(name: &str, field_type: ActionValueType) -> StateFieldRef {
        use super::super::state_ref::StateRefSource;

        StateFieldRef {
            view_id: Uuid::nil(),
            source: StateRefSource::Custom {
                field_id: Uuid::nil(),
                field_name: name.to_string(),
            },
            field_type,
            display_name: name.to_string(),
        }
    }

    fn node_of(graph: &ActionGraph, id: u64) -> &super::super::graph::ActionNodeData {
        graph.nodes.iter().find(|node| node.id == id).unwrap()
    }

    #[test]
    fn an_if_reads_its_wired_input_until_a_condition_is_authored() {
        let (mut graph, id) = with_node(ActionNodeKind::Conditional);

        // A new node shows a row to author into, but has not claimed it.
        assert!(
            node_of(&graph, id).authored_conditions.is_empty(),
            "an untouched node should still be running off its wired input",
        );

        assert!(update(
            &mut graph,
            NodeMessage::SetConditionRhs(id, 0, "ready".into()),
        ));

        assert_eq!(
            node_of(&graph, id).authored_conditions.len(),
            1,
            "editing the shown row should be what claims it",
        );
    }

    #[test]
    fn a_conditions_operator_follows_the_type_it_reads() {
        let (mut graph, id) = with_node(ActionNodeKind::Conditional);

        // The default row compares a string, so it starts on a string
        // operator.
        assert!(update(
            &mut graph,
            NodeMessage::SetConditionOp(id, 0, CompareOp::Contains),
        ));

        // Pointing it at a bool leaves `contains` meaningless.
        assert!(update(
            &mut graph,
            NodeMessage::SetConditionSource(
                id,
                0,
                ConditionSource::StateField(state_field("flag", ActionValueType::Bool)),
            ),
        ));

        let condition = node_of(&graph, id).authored_conditions[0].clone();

        assert!(
            operators_for(&ActionValueType::Bool).contains(&condition.operator),
            "a bool condition kept `{:?}`, which it cannot lower",
            condition.operator,
        );
    }

    #[test]
    fn the_last_condition_cannot_be_removed() {
        let (mut graph, id) = with_node(ActionNodeKind::Conditional);

        assert!(update(&mut graph, NodeMessage::AddCondition(id)));
        assert!(update(&mut graph, NodeMessage::RemoveCondition(id, 1)));
        assert_eq!(node_of(&graph, id).authored_conditions.len(), 1);

        assert!(
            !update(&mut graph, NodeMessage::RemoveCondition(id, 0)),
            "removing the last row would hand the node back to its wired \
             input without saying so",
        );
    }

    #[test]
    fn taking_an_enum_makes_a_matchs_arms_its_variants() {
        let (mut graph, id) = with_node(ActionNodeKind::Match {
            arms: vec!["one".into(), "two".into(), "three".into()],
            enum_type: None,
        });

        // Wire the last arm's output, which the enum's two variants leave
        // behind.
        let doomed = node_of(&graph, id).cached_ports_out.last().unwrap().id.0;

        graph.edges.push(super::super::graph::ActionEdge {
            from_node: id,
            from_port: doomed,
            to_node: graph.nodes[0].id,
            to_port: 0,
        });

        assert!(update(
            &mut graph,
            NodeMessage::SetMatchEnum(
                id,
                EnumOption {
                    name: "Page".into(),
                    variants: vec!["Home".into(), "Away".into()],
                },
            ),
        ));

        assert!(matches!(
            kind_of(&graph, id),
            ActionNodeKind::Match { arms, enum_type }
                if arms == &["Home".to_string(), "Away".to_string()]
                    && enum_type.as_deref() == Some("Page")
        ));

        // Two arms and a default.
        assert_eq!(ports(&graph, id).1, 3);

        assert!(
            !graph.edges.iter().any(|edge| edge.from_port == doomed),
            "an edge outlived the arm the enum replaced",
        );
    }

    #[test]
    fn an_enum_matchs_literal_subject_carries_the_enums_name() {
        let (mut graph, id) = with_node(ActionNodeKind::Match {
            arms: Vec::new(),
            enum_type: None,
        });

        assert!(update(
            &mut graph,
            NodeMessage::SetMatchEnum(
                id,
                EnumOption {
                    name: "Page".into(),
                    variants: vec!["Home".into()],
                },
            ),
        ));

        assert!(update(
            &mut graph,
            NodeMessage::SetMatchSubjectLiteral(id, "Home".into()),
        ));

        assert!(matches!(
            &node_of(&graph, id).authored_match_subject,
            Some(AuthoredValueSource::Literal(ActionValue::EnumVariant {
                type_name,
                variant,
            })) if type_name == "Page" && variant == "Home"
        ));
    }

    #[test]
    fn repointing_a_condition_at_a_same_typed_field_keeps_its_comparison() {
        let (mut graph, id) = with_node(ActionNodeKind::Conditional);

        assert!(update(
            &mut graph,
            NodeMessage::SetConditionOp(id, 0, CompareOp::Eq),
        ));
        assert!(update(
            &mut graph,
            NodeMessage::SetConditionRhs(id, 0, "keep me".into()),
        ));

        // Both strings, so nothing about the comparison has to change.
        for name in ["first", "second"] {
            assert!(update(
                &mut graph,
                NodeMessage::SetConditionSource(
                    id,
                    0,
                    ConditionSource::StateField(state_field(name, ActionValueType::String)),
                ),
            ));
        }

        assert!(
            matches!(
                &node_of(&graph, id).authored_conditions[0].rhs_literal,
                ActionValue::String(value) if value == "keep me"
            ),
            "the comparison value should survive a trip through the picker",
        );
    }
}

// ─── The choices the node bodies offer ───────────────────────────────────────
// Moved out of `views::action_editor` when that module was parked: these walk
// the project, not the editor.

/// A widget a trigger row can listen to.
#[derive(Debug, Clone, PartialEq)]
pub struct WidgetOption {
    pub view_id: Uuid,
    pub widget_id: WidgetId,
    /// The events this widget actually emits, so a row cannot pick one it
    /// would never fire.
    pub supported_events: Vec<&'static str>,
    pub label: String,
}

impl std::fmt::Display for WidgetOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.label)
    }
}

/// A `ViewReference` widget a navigate target can switch, and the views it can
/// be switched to.
#[derive(Debug, Clone, PartialEq)]
pub struct ViewReferenceOption {
    pub owner_view_id: Uuid,
    pub widget_id: WidgetId,
    pub label: String,
    pub targets: Vec<ViewOption>,
}

impl std::fmt::Display for ViewReferenceOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.label)
    }
}

/// Every widget in the project, with the events it emits.
pub fn collect_widget_options(
    all_views: &BTreeMap<Uuid, crate::data_structures::types::types::AppView>,
) -> Vec<WidgetOption> {
    let mut out = Vec::new();

    for view in all_views.values() {
        collect_widget_options_recursive(view.hierarchy.root(), view.id, &view.name, &mut out);
    }

    out.sort_by(|a, b| a.label.cmp(&b.label));
    out
}

fn collect_widget_options_recursive(
    widget: &crate::data_structures::types::types::Widget,
    view_id: Uuid,
    view_name: &str,
    out: &mut Vec<WidgetOption>,
) {
    let label = if widget.properties.widget_name.is_empty() {
        format!("{}/{:?}", view_name, widget.widget_type)
    } else {
        format!("{}/{}", view_name, widget.properties.widget_name)
    };

    out.push(WidgetOption {
        view_id,
        widget_id: widget.id,
        supported_events: crate::action_system::events::actionable_events(
            widget.widget_type,
            &widget.properties,
        ),
        label,
    });

    for child in &widget.children {
        collect_widget_options_recursive(child, view_id, view_name, out);
    }
}

/// Every `ViewReference` widget that has more than one view to switch between.
///
/// One with a single view has nothing to navigate, so it is not offered.
pub fn collect_view_reference_options(
    all_views: &BTreeMap<Uuid, crate::data_structures::types::types::AppView>,
) -> Vec<ViewReferenceOption> {
    let mut out = Vec::new();

    for (owner_view_id, owner_view) in all_views {
        collect_view_reference_options_recursive(
            owner_view.hierarchy.root(),
            *owner_view_id,
            &owner_view.name,
            all_views,
            &mut out,
        );
    }

    out.sort_by(|a, b| a.label.cmp(&b.label));
    out
}

fn collect_view_reference_options_recursive(
    widget: &crate::data_structures::types::types::Widget,
    owner_view_id: Uuid,
    owner_view_name: &str,
    all_views: &BTreeMap<Uuid, crate::data_structures::types::types::AppView>,
    out: &mut Vec<ViewReferenceOption>,
) {
    use crate::data_structures::types::types::WidgetType;

    if widget.widget_type == WidgetType::ViewReference
        && let Some(primary) = widget.properties.referenced_view_id
        && !widget.properties.extra_view_ids.is_empty()
    {
        let named = |id: &Uuid| -> Option<ViewOption> {
            all_views.get(id).map(|view| ViewOption {
                id: view.id,
                name: view.name.clone(),
            })
        };

        let targets: Vec<ViewOption> = std::iter::once(&primary)
            .chain(widget.properties.extra_view_ids.iter())
            .filter_map(named)
            .collect();

        if targets.len() > 1 {
            let name = if widget.properties.widget_name.trim().is_empty() {
                format!("{:?}", widget.widget_type)
            } else {
                widget.properties.widget_name.trim().to_string()
            };

            out.push(ViewReferenceOption {
                owner_view_id,
                widget_id: widget.id,
                label: format!("{owner_view_name}/{name}"),
                targets,
            });
        }
    }

    for child in &widget.children {
        collect_view_reference_options_recursive(
            child,
            owner_view_id,
            owner_view_name,
            all_views,
            out,
        );
    }
}

/// The events a trigger row can listen for.
pub fn all_event_types() -> Vec<String> {
    [
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
    .map(str::to_string)
    .collect()
}

/// An enum an `EnumLiteral` can emit a variant of.
#[derive(Debug, Clone, PartialEq)]
pub struct EnumOption {
    pub name: String,
    pub variants: Vec<String>,
}

impl std::fmt::Display for EnumOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)
    }
}

// ─── Trigger rows ────────────────────────────────────────────────────────────
// A widget-event trigger belongs to the *flow*, not to the node that draws it,
// so these messages carry the row's id rather than a node's and are applied
// against the flow. The node is only where they are shown.

/// An edit to a flow's widget-event trigger.
#[derive(Debug, Clone)]
pub enum TriggerMessage {
    AddRow,
    RemoveRow(Uuid),
    SetEvent(Uuid, String),
    SetTarget(Uuid, Uuid, WidgetId),
}

/// The rows of a widget-event trigger: which event, on which widget.
///
/// The widget picker is narrowed to widgets that actually emit the chosen
/// event, so a row cannot be pointed at something that would never fire.
fn trigger_rows<'a>(
    rows: &'a [crate::action_system::flow::WidgetEventRow],
    node: &'a ActionNodeData,
    ctx: &'a NodeContext,
) -> Vec<Element<'a, NodeMessage>> {
    let mut out: Vec<Element<'a, NodeMessage>> = Vec::new();

    for (index, row_data) in rows.iter().enumerate() {
        let row_id = row_data.id;
        let event = row_data.event_type.clone();

        let targets: Vec<WidgetOption> = ctx
            .widgets
            .iter()
            .filter(|widget| widget.supported_events.iter().any(|name| *name == event))
            .cloned()
            .collect();

        let selected = row_data.target.and_then(|(view_id, widget)| {
            targets
                .iter()
                .find(|option| option.view_id == view_id && option.widget_id.0 == widget)
                .cloned()
        });

        out.push(body_row(
            None,
            pick_list(
                Some(row_data.event_type.clone()),
                all_event_types(),
                ToString::to_string,
            )
            .on_select(move |event| {
                NodeMessage::Trigger(TriggerMessage::SetEvent(row_id, event))
            })
            .text_size(LABEL_SIZE)
            .width(Length::Fixed(CONTROL_WIDTH)),
            None,
        ));

        out.push(body_row(
            None,
            row![
                pick_list(selected, targets, ToString::to_string)
                    .on_select(move |widget: WidgetOption| {
                        NodeMessage::Trigger(TriggerMessage::SetTarget(
                            row_id,
                            widget.view_id,
                            widget.widget_id,
                        ))
                    })
                    .placeholder("widget")
                    .text_size(LABEL_SIZE)
                    .width(Length::Fixed(CONTROL_WIDTH - 24.0)),
                button(text("\u{2715}").size(9))
                    .on_press(NodeMessage::Trigger(TriggerMessage::RemoveRow(row_id)))
                    .style(button::text),
            ]
            .spacing(2)
            .align_y(Alignment::Center),
            // Each row has its own flow output, in row order.
            node.cached_ports_out.get(index),
        ));
    }

    out.push(body_row(
        None,
        button(text("+ event").size(LABEL_SIZE))
            .on_press(NodeMessage::Trigger(TriggerMessage::AddRow))
            .style(button::text),
        None,
    ));

    out
}

/// Applies a trigger edit to the flow that owns it.
///
/// Returns whether anything changed. The node's ports follow the row count, so
/// adding or removing one rebuilds them.
pub fn update_trigger(
    flow: &mut crate::action_system::flow::AppFlow,
    message: TriggerMessage,
) -> bool {
    use crate::action_system::flow::{FlowTrigger, WidgetEventRow};

    let FlowTrigger::WidgetEvent { rows } = &mut flow.trigger else {
        return false;
    };

    match message {
        TriggerMessage::AddRow => rows.push(WidgetEventRow::new()),

        TriggerMessage::RemoveRow(id) => {
            let before = rows.len();
            rows.retain(|row| row.id != id);

            if rows.len() == before {
                return false;
            }
        }

        TriggerMessage::SetEvent(id, event) => {
            let Some(row) = rows.iter_mut().find(|row| row.id == id) else {
                return false;
            };

            // The target may not emit the new event, and a row pointing at a
            // widget that cannot fire it would generate a handler that never
            // runs. Clearing it makes the picker ask again.
            row.event_type = event;
            row.target = None;
        }

        TriggerMessage::SetTarget(id, view_id, widget_id) => {
            let Some(row) = rows.iter_mut().find(|row| row.id == id) else {
                return false;
            };

            row.target = Some((view_id, widget_id.0));
        }
    }

    // Only a *complete* row (event and target both set) gets an output, so
    // every edit here can move the port list -- including one that only
    // cleared a target.
    let rows = rows.clone();

    if let Some(trigger) = flow
        .graph
        .nodes
        .iter_mut()
        .find(|node| matches!(node.kind, ActionNodeKind::Trigger { .. }))
    {
        trigger.rebuild_ports_for_widget_event(&rows);
    }

    // A row that lost its output takes its edge with it.
    prune_dangling_edges(&mut flow.graph);

    true
}

/// One navigate target: an app view, or a `ViewReference` widget switched to
/// one of its own views.
///
/// The two are separate pickers because they are separate destinations — a
/// widget target needs the widget named as well as the view.
fn navigate_target_row<'a>(
    id: u64,
    index: usize,
    target: &'a Option<NavigateTarget>,
    ctx: &'a NodeContext,
    node: &'a ActionNodeData,
) -> Element<'a, NodeMessage> {
    let app_view = match target {
        Some(NavigateTarget::AppView { view_id }) => {
            ctx.views.iter().find(|view| view.id == *view_id).cloned()
        }
        _ => None,
    };

    let reference = match target {
        Some(NavigateTarget::ViewReference {
            owner_view_id,
            widget_id,
            ..
        }) => ctx
            .view_references
            .iter()
            .find(|option| {
                option.owner_view_id == *owner_view_id && option.widget_id == *widget_id
            })
            .cloned(),
        _ => None,
    };

    let mut rows = vec![body_row(
        node.cached_ports_in.get(index),
        pick_list(app_view, ctx.views.clone(), ToString::to_string)
            .on_select(move |view: ViewOption| NodeMessage::SetNavigateTarget(id, index, view.id))
            .placeholder("app view")
            .text_size(LABEL_SIZE)
            .width(Length::Fixed(CONTROL_WIDTH)),
        node.cached_ports_out.get(index),
    )];

    // Only offered when the project has a switchable ViewReference; otherwise
    // it is a picker that can never be used.
    if !ctx.view_references.is_empty() {
        rows.push(body_row(
            None,
            pick_list(
                reference.clone(),
                ctx.view_references.clone(),
                ToString::to_string,
            )
            .on_select(move |option: ViewReferenceOption| {
                // Default to the reference's first view; the picker below
                // changes which.
                let first = option.targets.first().map(|view| view.id).unwrap_or_default();

                NodeMessage::SetNavigateViewReference(
                    id,
                    index,
                    option.owner_view_id,
                    option.widget_id,
                    first,
                )
            })
            .placeholder("or a view widget")
            .text_size(LABEL_SIZE)
            .width(Length::Fixed(CONTROL_WIDTH)),
            None,
        ));
    }

    // Which of that widget's views to switch it to.
    if let Some(option) = reference {
        let selected = match target {
            Some(NavigateTarget::ViewReference { target_view_id, .. }) => option
                .targets
                .iter()
                .find(|view| view.id == *target_view_id)
                .cloned(),
            _ => None,
        };

        let owner = option.owner_view_id;
        let widget = option.widget_id;

        rows.push(body_row(
            None,
            pick_list(selected, option.targets.clone(), ToString::to_string)
                .on_select(move |view: ViewOption| {
                    NodeMessage::SetNavigateViewReference(id, index, owner, widget, view.id)
                })
                .placeholder("show")
                .text_size(LABEL_SIZE)
                .width(Length::Fixed(CONTROL_WIDTH)),
            None,
        ));
    }

    column(rows).spacing(2).into()
}

/// An enum and one of its variants, emitted as a value.
fn enum_literal_rows<'a>(
    id: u64,
    enum_name: &'a Option<String>,
    variant: &'a Option<String>,
    ctx: &'a NodeContext,
    node: &'a ActionNodeData,
) -> Vec<Element<'a, NodeMessage>> {
    let selected = enum_name
        .as_ref()
        .and_then(|name| ctx.enums.iter().find(|option| option.name == *name))
        .cloned();

    let variants = selected
        .as_ref()
        .map(|option| option.variants.clone())
        .unwrap_or_default();

    vec![
        body_row(
            None,
            pick_list(selected, ctx.enums.clone(), ToString::to_string)
                .on_select(move |option: EnumOption| NodeMessage::SetEnum(id, option.name))
                .placeholder("enum")
                .text_size(LABEL_SIZE)
                .width(Length::Fixed(CONTROL_WIDTH)),
            None,
        ),
        body_row(
            None,
            pick_list(variant.clone(), variants, ToString::to_string)
                .on_select(move |variant| NodeMessage::SetEnumVariant(id, variant))
                .placeholder("variant")
                .text_size(LABEL_SIZE)
                .width(Length::Fixed(CONTROL_WIDTH)),
            node.cached_ports_out.first(),
        ),
    ]
}

// ─── Authored If / Match conditions ──────────────────────────────────────────
// An `If` can take its condition either from a wired `condition` input or from
// rows authored in the node. The authored rows win when there are any, which
// is what `semantic` lowers -- so the wired pin stays on the node, and the
// header says which of the two is in force.
//
// These fields live on `ActionNodeData` rather than in its `kind`, so their
// edits are applied before the kind match in [`update`].

/// Where an authored condition or match subject reads from.
///
/// Mirrors [`AuthoredValueSource`] with the literal's payload dropped, so it
/// can drive a pick list without the value churning as it is typed.
#[derive(Debug, Clone, PartialEq)]
pub enum ConditionSource {
    TriggerInput {
        name: String,
        value_type: ActionValueType,
    },
    StateField(StateFieldRef),
    Literal,
}

impl ConditionSource {
    /// Keeps the literal already entered when the picker lands back on
    /// `Literal`, so switching away and back does not clear it.
    fn to_source(&self, existing: Option<ActionValue>) -> AuthoredValueSource {
        match self {
            Self::TriggerInput { name, value_type } => AuthoredValueSource::TriggerInput {
                name: name.clone(),
                value_type: value_type.clone(),
            },
            Self::StateField(field) => AuthoredValueSource::StateField(field.clone()),
            Self::Literal => AuthoredValueSource::Literal(
                existing.unwrap_or(ActionValue::String(String::new())),
            ),
        }
    }

    fn matches(&self, source: &AuthoredValueSource) -> bool {
        match (self, source) {
            (
                Self::TriggerInput { name, value_type },
                AuthoredValueSource::TriggerInput {
                    name: other_name,
                    value_type: other_type,
                },
            ) => name == other_name && value_type == other_type,
            (Self::StateField(field), AuthoredValueSource::StateField(other)) => field == other,
            (Self::Literal, AuthoredValueSource::Literal(_)) => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for ConditionSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TriggerInput { name, .. } => write!(f, "trigger: {name}"),
            Self::StateField(field) => write!(f, "state: {}", field.display_name),
            Self::Literal => f.write_str("literal"),
        }
    }
}

/// The condition rows to draw.
///
/// An `If` with nothing authored still shows one row to author into; it is
/// only written back to the node once edited, so until then the wired
/// `condition` input is still what runs.
fn conditions_for_display(node: &ActionNodeData) -> Vec<AuthoredCondition> {
    let mut rows = if !node.authored_conditions.is_empty() {
        node.authored_conditions.clone()
    } else if let Some(legacy) = &node.authored_condition {
        vec![legacy.clone()]
    } else {
        vec![AuthoredCondition::default()]
    };

    for row in rows.iter_mut() {
        normalize_condition(row);
    }

    rows
}

/// The condition rows to edit, materialising the display default and folding
/// in a single-condition save from before there were rows.
fn conditions_mut(node: &mut ActionNodeData) -> &mut Vec<AuthoredCondition> {
    if node.authored_conditions.is_empty() {
        match node.authored_condition.take() {
            Some(legacy) => node.authored_conditions.push(legacy),
            None => node.authored_conditions.push(AuthoredCondition::default()),
        }
    } else {
        // The rows are the authority once there are any.
        node.authored_condition = None;
    }

    &mut node.authored_conditions
}

/// The operators that mean anything for a left-hand side of this type.
fn operators_for(value_type: &ActionValueType) -> Vec<CompareOp> {
    match value_type {
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

/// Brings a row's operator and right-hand side back in line with the type of
/// its left-hand side.
///
/// Changing the source can leave `> ` on a string or a number against an enum,
/// neither of which lowers to anything -- so the row is corrected rather than
/// allowed to go invalid.
fn normalize_condition(condition: &mut AuthoredCondition) {
    let value_type = condition.lhs.value_type();
    let operators = operators_for(&value_type);

    if !operators.contains(&condition.operator) {
        condition.operator = operators.first().cloned().unwrap_or(CompareOp::Eq);
    }

    if condition.operator.needs_rhs() && !rhs_matches(&condition.rhs_literal, &value_type) {
        condition.rhs_literal = default_value_for(&value_type);
    }
}

fn rhs_matches(rhs: &ActionValue, value_type: &ActionValueType) -> bool {
    match value_type {
        ActionValueType::String => matches!(rhs, ActionValue::String(_)),
        ActionValueType::Bool => matches!(rhs, ActionValue::Bool(_)),
        ActionValueType::F32 | ActionValueType::F64 | ActionValueType::Usize => {
            matches!(rhs, ActionValue::Number(_))
        }
        ActionValueType::Enum { type_name, .. } => matches!(
            rhs,
            ActionValue::EnumVariant {
                type_name: rhs_type,
                ..
            } if rhs_type == type_name
        ),
    }
}

fn default_value_for(value_type: &ActionValueType) -> ActionValue {
    match value_type {
        ActionValueType::String => ActionValue::String(String::new()),
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
    }
}

/// Reads typed text back into a value of the left-hand side's type.
///
/// Text that is not yet a number stays a string rather than snapping to zero
/// halfway through being typed; `normalize_condition` is what puts it right if
/// it never becomes one.
fn parse_rhs(value_type: &ActionValueType, raw: &str) -> ActionValue {
    match value_type {
        ActionValueType::Bool => match raw.trim().to_ascii_lowercase().as_str() {
            "true" => ActionValue::Bool(true),
            "false" => ActionValue::Bool(false),
            _ => ActionValue::String(raw.to_string()),
        },
        ActionValueType::F32 | ActionValueType::F64 | ActionValueType::Usize => raw
            .trim()
            .parse()
            .map(ActionValue::Number)
            .unwrap_or_else(|_| ActionValue::String(raw.to_string())),
        ActionValueType::Enum { type_name, .. } => ActionValue::EnumVariant {
            type_name: type_name.clone(),
            variant: raw.to_string(),
        },
        ActionValueType::String => ActionValue::String(raw.to_string()),
    }
}

/// A value as it is edited: an enum variant on its own, not `Type::Variant`.
fn editable_text(value: &ActionValue) -> String {
    match value {
        ActionValue::EnumVariant { variant, .. } => variant.clone(),
        other => literal_text(other),
    }
}

/// An `If`: its conditions, and the two branches they pick between.
fn conditional_rows<'a>(
    node: &'a ActionNodeData,
    ctx: &'a NodeContext,
) -> Vec<Element<'a, NodeMessage>> {
    let id = node.id;
    let authored = conditions_for_display(node);

    // A row is only authoritative once it is on the node; until then the wired
    // input is what runs, and the header says so.
    let authored_wins = !node.authored_conditions.is_empty() || node.authored_condition.is_some();

    let header: Element<'a, NodeMessage> = if authored_wins && authored.len() > 1 {
        row![
            text("if").size(LABEL_SIZE),
            pick_list(
                Some(node.authored_condition_join),
                vec![ConditionJoinMode::All, ConditionJoinMode::Any],
                ToString::to_string,
            )
            .on_select(move |mode| NodeMessage::SetConditionJoin(id, mode))
            .text_size(LABEL_SIZE)
            .width(Length::Fixed(CONTROL_WIDTH - 24.0)),
        ]
        .spacing(4)
        .align_y(Alignment::Center)
        .into()
    } else if authored_wins {
        text("if").size(LABEL_SIZE).width(Length::Fill).into()
    } else {
        text("if wired").size(LABEL_SIZE).width(Length::Fill).into()
    };

    let mut rows = vec![
        body_row(node.cached_ports_in.first(), header, None),
        // The wired condition stays offered even while rows are authored: it
        // is what runs again the moment they are all removed.
        body_row(
            node.cached_ports_in.get(1),
            text("condition").size(LABEL_SIZE).width(Length::Fill),
            None,
        ),
    ];

    for (index, condition) in authored.iter().enumerate() {
        let value_type = condition.lhs.value_type();
        let selected = ctx
            .condition_sources
            .iter()
            .find(|source| source.matches(&condition.lhs))
            .cloned();

        rows.push(body_row(
            None,
            pick_list(
                selected,
                ctx.condition_sources.clone(),
                ToString::to_string,
            )
            .on_select(move |source| NodeMessage::SetConditionSource(id, index, source))
            .placeholder("value")
            .text_size(LABEL_SIZE)
            .width(Length::Fixed(CONTROL_WIDTH)),
            None,
        ));

        let removable = authored.len() > 1;
        let remove = button(text("\u{2715}").size(9)).style(button::text);

        rows.push(body_row(
            None,
            row![
                pick_list(
                    Some(condition.operator.clone()),
                    operators_for(&value_type),
                    ToString::to_string,
                )
                .on_select(move |op| NodeMessage::SetConditionOp(id, index, op))
                .text_size(LABEL_SIZE)
                .width(Length::Fixed(CONTROL_WIDTH - 24.0)),
                if removable {
                    remove.on_press(NodeMessage::RemoveCondition(id, index))
                } else {
                    remove
                },
            ]
            .spacing(2)
            .align_y(Alignment::Center),
            None,
        ));

        if condition.operator.needs_rhs() {
            rows.push(body_row(None, rhs_editor(id, index, condition), None));
        }
    }

    rows.push(body_row(
        None,
        button(text("+ condition").size(LABEL_SIZE))
            .on_press(NodeMessage::AddCondition(id))
            .style(button::text),
        None,
    ));

    rows.push(body_row(
        None,
        text("then").size(LABEL_SIZE).width(Length::Fill),
        node.cached_ports_out.first(),
    ));

    rows.push(body_row(
        None,
        text("else").size(LABEL_SIZE).width(Length::Fill),
        node.cached_ports_out.get(1),
    ));

    rows
}

/// The right-hand side of one condition, shaped by the left-hand side's type:
/// a checkbox for a bool, the variants for an enum, text otherwise.
/// Takes nothing by reference, so the rows can be built from a normalised copy
/// of the conditions rather than from the node's own.
fn rhs_editor(
    id: u64,
    index: usize,
    condition: &AuthoredCondition,
) -> Element<'static, NodeMessage> {
    match condition.lhs.value_type() {
        ActionValueType::Bool => checkbox(matches!(condition.rhs_literal, ActionValue::Bool(true)))
            .label("true")
            .text_size(LABEL_SIZE)
            .on_toggle(move |value| {
                NodeMessage::SetConditionRhs(id, index, value.to_string())
            })
            .into(),

        ActionValueType::Enum { variants, .. } if !variants.is_empty() => {
            let selected = match &condition.rhs_literal {
                ActionValue::EnumVariant { variant, .. } => Some(variant.clone()),
                _ => None,
            };

            pick_list(selected, variants, ToString::to_string)
                .on_select(move |variant| NodeMessage::SetConditionRhs(id, index, variant))
                .placeholder("variant")
                .text_size(LABEL_SIZE)
                .width(Length::Fixed(CONTROL_WIDTH))
                .into()
        }

        _ => text_input("value", editable_text(&condition.rhs_literal))
            .on_input(move |value| NodeMessage::SetConditionRhs(id, index, value))
            .size(LABEL_SIZE)
            .width(Length::Fixed(CONTROL_WIDTH))
            .into(),
    }
}

/// The subject rows of a `Match`: which enum it switches on, if any, and where
/// the value being matched comes from.
fn match_subject_rows<'a>(
    node: &'a ActionNodeData,
    enum_type: &'a Option<String>,
    arms: &'a [String],
    ctx: &'a NodeContext,
) -> Vec<Element<'a, NodeMessage>> {
    let id = node.id;

    let selected_enum = enum_type
        .as_ref()
        .and_then(|name| ctx.enums.iter().find(|option| option.name == *name))
        .cloned();

    // Unlike a condition, a match subject can be a constant -- switching on a
    // fixed value is how an enum match is usually authored.
    let sources: Vec<ConditionSource> = std::iter::once(ConditionSource::Literal)
        .chain(ctx.condition_sources.iter().cloned())
        .collect();

    let selected_source = node
        .authored_match_subject
        .as_ref()
        .and_then(|subject| sources.iter().find(|source| source.matches(subject)))
        .cloned();

    let mut rows = vec![
        // An enum subject turns the arms into its variants, which is why it
        // sits above the source picker rather than beside the arms.
        body_row(
            node.cached_ports_in.first(),
            pick_list(selected_enum, ctx.enums.clone(), ToString::to_string)
                .on_select(move |option: EnumOption| NodeMessage::SetMatchEnum(id, option))
                .placeholder("on enum")
                .text_size(LABEL_SIZE)
                .width(Length::Fixed(CONTROL_WIDTH)),
            None,
        ),
        body_row(
            node.cached_ports_in.get(1),
            pick_list(selected_source, sources, ToString::to_string)
                .on_select(move |source| NodeMessage::SetMatchSubject(id, source))
                .placeholder("or wire a value")
                .text_size(LABEL_SIZE)
                .width(Length::Fixed(CONTROL_WIDTH)),
            None,
        ),
    ];

    // A literal subject needs the value itself, which for an enum match is one
    // of the arms.
    if let Some(AuthoredValueSource::Literal(value)) = &node.authored_match_subject {
        let editor: Element<'a, NodeMessage> = if enum_type.is_some() && !arms.is_empty() {
            let selected = match value {
                ActionValue::EnumVariant { variant, .. } => Some(variant.clone()),
                _ => None,
            };

            pick_list(selected, arms.to_vec(), ToString::to_string)
                .on_select(move |variant| NodeMessage::SetMatchSubjectLiteral(id, variant))
                .placeholder("variant")
                .text_size(LABEL_SIZE)
                .width(Length::Fixed(CONTROL_WIDTH))
                .into()
        } else {
            text_input("value", editable_text(value))
                .on_input(move |raw| NodeMessage::SetMatchSubjectLiteral(id, raw))
                .size(LABEL_SIZE)
                .width(Length::Fixed(CONTROL_WIDTH))
                .into()
        };

        rows.push(body_row(None, editor, None));
    }

    rows
}

/// What an edit to the node's authored fields did.
///
/// Those fields sit on [`ActionNodeData`] rather than in its `kind`, so they
/// are applied before the kind match in [`update`] -- and a message that turns
/// out not to be one of theirs is handed straight back.
enum Authored {
    /// Applied. The port list moves when a `Match` takes an enum, because the
    /// arms become its variants.
    Applied { ports_changed: bool },
    /// One of these edits, but nothing changed by it.
    Ignored,
    /// Not one of these edits.
    NotMine(NodeMessage),
}

fn update_authored(node: &mut ActionNodeData, message: NodeMessage) -> Authored {
    match message {
        NodeMessage::AddCondition(_) => {
            conditions_mut(node).push(AuthoredCondition::default());
            Authored::Applied {
                ports_changed: false,
            }
        }

        NodeMessage::RemoveCondition(_, index) => {
            let conditions = conditions_mut(node);

            // The last row is what the node is authored against; removing it
            // would silently hand the node back to its wired input.
            if index >= conditions.len() || conditions.len() == 1 {
                return Authored::Ignored;
            }

            conditions.remove(index);
            Authored::Applied {
                ports_changed: false,
            }
        }

        NodeMessage::SetConditionJoin(_, mode) => {
            if node.authored_condition_join == mode {
                return Authored::Ignored;
            }

            node.authored_condition_join = mode;
            Authored::Applied {
                ports_changed: false,
            }
        }

        NodeMessage::SetConditionSource(_, index, source) => {
            let conditions = conditions_mut(node);

            let Some(condition) = conditions.get_mut(index) else {
                return Authored::Ignored;
            };

            let existing = match &condition.lhs {
                AuthoredValueSource::Literal(value) => Some(value.clone()),
                _ => None,
            };

            condition.lhs = source.to_source(existing);
            // The new source may not take the operator or value the old one
            // did.
            normalize_condition(condition);

            Authored::Applied {
                ports_changed: false,
            }
        }

        NodeMessage::SetConditionOp(_, index, operator) => {
            let conditions = conditions_mut(node);

            let Some(condition) = conditions.get_mut(index) else {
                return Authored::Ignored;
            };

            condition.operator = operator;
            // An operator that wants no right-hand side leaves the old one
            // alone; one that does may need it retyped.
            normalize_condition(condition);

            Authored::Applied {
                ports_changed: false,
            }
        }

        NodeMessage::SetConditionRhs(_, index, raw) => {
            let conditions = conditions_mut(node);

            let Some(condition) = conditions.get_mut(index) else {
                return Authored::Ignored;
            };

            condition.rhs_literal = parse_rhs(&condition.lhs.value_type(), &raw);

            Authored::Applied {
                ports_changed: false,
            }
        }

        NodeMessage::SetMatchEnum(_, option) => {
            let ActionNodeKind::Match { arms, enum_type } = &mut node.kind else {
                return Authored::Ignored;
            };

            // The arms *are* the variants for an enum match, so taking an enum
            // replaces them -- and with them the node's outputs.
            let ports_changed = *arms != option.variants;

            *arms = option.variants;
            *enum_type = Some(option.name.clone());

            // A literal subject was typed against the old type.
            if let Some(AuthoredValueSource::Literal(_)) = &node.authored_match_subject {
                node.authored_match_subject =
                    Some(AuthoredValueSource::Literal(ActionValue::EnumVariant {
                        type_name: option.name,
                        variant: String::new(),
                    }));
            }

            Authored::Applied { ports_changed }
        }

        NodeMessage::SetMatchSubject(_, source) => {
            let existing = match &node.authored_match_subject {
                Some(AuthoredValueSource::Literal(value)) => Some(value.clone()),
                _ => None,
            };

            node.authored_match_subject = Some(source.to_source(existing));

            Authored::Applied {
                ports_changed: false,
            }
        }

        NodeMessage::SetMatchSubjectLiteral(_, raw) => {
            let enum_type = match &node.kind {
                ActionNodeKind::Match { enum_type, .. } => enum_type.clone(),
                _ => None,
            };

            let value = match enum_type {
                Some(type_name) => ActionValue::EnumVariant {
                    type_name,
                    variant: raw,
                },
                None => ActionValue::String(raw),
            };

            node.authored_match_subject = Some(AuthoredValueSource::Literal(value));

            Authored::Applied {
                ports_changed: false,
            }
        }

        other => Authored::NotMine(other),
    }
}

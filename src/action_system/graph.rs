use crate::action_system::flow::WidgetEventRow;
use crate::action_system::node_kinds::{
    ActionNodeKind, ActionValue, AuthoredCondition, AuthoredValueSource, CompareRhs,
    ConditionJoinMode,
};
use crate::action_system::state_ref::ActionValueType;
use iced::Color;
use iced::Point;
use iced::Vector;
use serde::{Deserialize, Serialize};
use widgets::flow_editor::{
    Edge, FlowNode, NodeId, PaletteEntry, PortDef, PortId, PortSide, PortType, VariableType,
    Viewport2D,
};

// ─── Constants (must match widgets/src/flow_editor/hit_test.rs) ──────────────
const HEADER_HEIGHT: f32 = 42.0;
const PORT_TOP_PADDING: f32 = 18.0;
const ROW_HALF_HEIGHT: f32 = 20.0;
const PORT_ROW_HEIGHT: f32 = 46.0;
const BOTTOM_PADDING: f32 = 24.0;

// ─── WidgetEvent row layout constants (scene space, calibrated at zoom=1.0) ──
// These must match the fixed container heights set in action_editor.rs build_node_body.
const NODE_ROW_GAP: f32 = 8.0;
const NODE_TOP_PAD: f32 = 8.0;
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

fn port_y_offset(slot: usize) -> f32 {
    HEADER_HEIGHT + PORT_TOP_PADDING + ROW_HALF_HEIGHT + slot as f32 * PORT_ROW_HEIGHT
}

fn node_height(max_slots: usize) -> f32 {
    HEADER_HEIGHT + PORT_TOP_PADDING + max_slots as f32 * PORT_ROW_HEIGHT + BOTTOM_PADDING
}

fn map_value_type(t: &ActionValueType) -> VariableType {
    match t {
        ActionValueType::String => VariableType::String,
        ActionValueType::Bool => VariableType::Boolean,
        ActionValueType::F32 | ActionValueType::F64 => VariableType::Number,
        ActionValueType::Usize => VariableType::Number,
        ActionValueType::Enum { .. } => VariableType::String,
    }
}

// ─── Opaque per-node ID ───────────────────────────────────────────────────────

pub type ActionNodeId = u64;

// ─── Edge (serialisable, mirrors flow_editor::Edge) ──────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionEdge {
    pub from_node: ActionNodeId,
    pub from_port: u64,
    pub to_node: ActionNodeId,
    pub to_port: u64,
}

impl ActionEdge {
    pub fn to_flow_edge(&self) -> Edge {
        Edge::new(
            NodeId(self.from_node),
            PortId(self.from_port),
            NodeId(self.to_node),
            PortId(self.to_port),
        )
    }
}

// ─── ActionNodeData ───────────────────────────────────────────────────────────

/// Combines semantic data (`kind`) with geometry/display data
/// needed by the `FlowEditor` widget.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionNodeData {
    pub id: ActionNodeId,
    #[serde(with = "crate::persistence::serde_iced::point")]
    pub position: Point,
    #[serde(default)]
    pub selected: bool,
    pub enabled: bool,
    pub kind: ActionNodeKind,

    // Cached geometry — recomputed by rebuild_ports() on load; not persisted.
    #[serde(skip, default)]
    pub cached_ports_in: Vec<PortDef>,
    #[serde(skip, default)]
    pub cached_ports_out: Vec<PortDef>,
    #[serde(skip, default = "default_cached_height")]
    pub cached_height: f32,

    /// Overrides the drawn kind_label for Trigger nodes (shows the actual trigger variant,
    /// e.g. "Widget Event" instead of the default empty string).
    #[serde(default)]
    pub kind_label_override: Option<String>,

    /// Optional in-node authored condition for If nodes.
    /// Legacy single-condition field kept for compatibility with older saves.
    #[serde(default)]
    pub authored_condition: Option<AuthoredCondition>,

    /// In-node authored conditions for If nodes (primary authoring path).
    #[serde(default)]
    pub authored_conditions: Vec<AuthoredCondition>,

    /// Join mode for authored If conditions.
    #[serde(default)]
    pub authored_condition_join: ConditionJoinMode,

    /// Optional in-node authored match subject for Match nodes.
    #[serde(default)]
    pub authored_match_subject: Option<AuthoredValueSource>,
}

fn default_cached_height() -> f32 {
    HEADER_HEIGHT + BOTTOM_PADDING
}

impl ActionNodeData {
    pub fn new(id: ActionNodeId, kind: ActionNodeKind, position: Point) -> Self {
        let mut node = Self {
            id,
            position,
            selected: false,
            enabled: true,
            kind,
            cached_ports_in: Vec::new(),
            cached_ports_out: Vec::new(),
            cached_height: HEADER_HEIGHT + BOTTOM_PADDING,
            kind_label_override: None,
            authored_condition: None,
            authored_conditions: Vec::new(),
            authored_condition_join: ConditionJoinMode::All,
            authored_match_subject: None,
        };
        node.rebuild_ports();
        node
    }

    fn effective_authored_condition_count(&self) -> usize {
        if !self.authored_conditions.is_empty() {
            self.authored_conditions.len()
        } else if self.authored_condition.is_some() {
            1
        } else {
            // Keep one visible authoring row in new UI even when compatibility input-port
            // fallback is still active under the hood.
            1
        }
    }

    /// Recomputes `cached_ports_in`, `cached_ports_out`, and `cached_height`
    /// from the current `kind`.
    pub fn rebuild_ports(&mut self) {
        let base = self.id * 1_000;
        self.cached_ports_in.clear();
        self.cached_ports_out.clear();

        match &self.kind {
            ActionNodeKind::Trigger { output_ports, .. } => {
                // Flow output (slot 0)
                self.cached_ports_out.push(PortDef::new(
                    base + 1,
                    "flow_out",
                    PortSide::Output,
                    0,
                    PortType::Flow,
                    port_y_offset(0),
                ));
                // Data outputs (slots 1..)
                for (i, tp) in output_ports.iter().enumerate() {
                    self.cached_ports_out.push(PortDef::new(
                        base + 10 + i as u64,
                        tp.name.clone(),
                        PortSide::Output,
                        i + 1,
                        PortType::Data(map_value_type(&tp.value_type)),
                        port_y_offset(i + 1),
                    ));
                }
                let max_slots = 1 + output_ports.len();
                self.cached_height = node_height(max_slots);
            }

            ActionNodeKind::SetState { .. } => {
                // Inputs: flow (slot 0), value data (slot 1)
                self.cached_ports_in.push(PortDef::new(
                    base + 0,
                    "flow_in",
                    PortSide::Input,
                    0,
                    PortType::Flow,
                    port_y_offset(0),
                ));
                self.cached_ports_in.push(PortDef::new(
                    base + 10,
                    "value",
                    PortSide::Input,
                    1,
                    PortType::Data(VariableType::String),
                    port_y_offset(1),
                ));
                // Output: flow (slot 0)
                self.cached_ports_out.push(PortDef::new(
                    base + 1,
                    "flow_out",
                    PortSide::Output,
                    0,
                    PortType::Flow,
                    port_y_offset(0),
                ));
                self.cached_height = node_height(2);
            }

            ActionNodeKind::StateMutation { assignments } => {
                let flow_y = HEADER_HEIGHT + NODE_TOP_PAD + STATE_MUTATION_HEADER_H * 0.5;
                self.cached_ports_in.push(PortDef::new(
                    base + 0,
                    "flow_in",
                    PortSide::Input,
                    0,
                    PortType::Flow,
                    flow_y,
                ));
                for (idx, _) in assignments.iter().enumerate() {
                    let y = HEADER_HEIGHT
                        + NODE_TOP_PAD
                        + STATE_MUTATION_HEADER_H
                        + NODE_ROW_GAP
                        + idx as f32 * (STATE_MUTATION_ROW_H + NODE_ROW_GAP)
                        + STATE_MUTATION_ROW_H * 0.5;
                    self.cached_ports_in.push(PortDef::new(
                        base + 10 + idx as u64,
                        format!("value_{idx}"),
                        PortSide::Input,
                        idx + 1,
                        PortType::Data(VariableType::String),
                        y,
                    ));
                }
                self.cached_ports_out.push(PortDef::new(
                    base + 1,
                    "flow_out",
                    PortSide::Output,
                    0,
                    PortType::Flow,
                    flow_y,
                ));
                let n = assignments.len().max(1) as f32;
                self.cached_height = HEADER_HEIGHT
                    + NODE_TOP_PAD
                    + STATE_MUTATION_HEADER_H
                    + NODE_ROW_GAP
                    + n * STATE_MUTATION_ROW_H
                    + n * NODE_ROW_GAP
                    + NODE_ADD_BTN_H
                    + NODE_BOTTOM_PAD;
            }

            ActionNodeKind::NavigateToView { targets } => {
                let n = targets.len().max(1);
                for i in 0..n {
                    let y = HEADER_HEIGHT
                        + NODE_TOP_PAD
                        + i as f32 * (NAVIGATE_ROW_H + NODE_ROW_GAP)
                        + NAVIGATE_ROW_H * 0.5;
                    self.cached_ports_in.push(PortDef::new(
                        base + i as u64 * 2,
                        "flow_in",
                        PortSide::Input,
                        i,
                        PortType::Flow,
                        y,
                    ));
                    self.cached_ports_out.push(PortDef::new(
                        base + i as u64 * 2 + 1,
                        "flow_out",
                        PortSide::Output,
                        i,
                        PortType::Flow,
                        y,
                    ));
                }
                self.cached_height = HEADER_HEIGHT
                    + NODE_TOP_PAD
                    + n as f32 * NAVIGATE_ROW_H
                    + n as f32 * NODE_ROW_GAP
                    + NODE_ADD_BTN_H
                    + NODE_BOTTOM_PAD;
            }

            ActionNodeKind::Conditional => {
                let flow_y = HEADER_HEIGHT * 0.5;
                let condition_count = self.effective_authored_condition_count().max(1);
                let conditions_block_h = condition_count as f32 * IF_CONDITION_ROW_H
                    + (condition_count as f32 - 1.0) * NODE_ROW_GAP;
                let condition_y = HEADER_HEIGHT
                    + NODE_TOP_PAD
                    + IF_CONDITION_ROW_H
                    + NODE_ROW_GAP
                    + IF_CONDITION_ROW_H * 0.5;
                let true_y = HEADER_HEIGHT
                    + NODE_TOP_PAD
                    + IF_CONDITION_ROW_H
                    + NODE_ROW_GAP
                    + conditions_block_h
                    + NODE_ROW_GAP
                    + NODE_ADD_BTN_H
                    + NODE_ROW_GAP
                    + IF_BRANCH_ROW_H * 0.5;
                let false_y = HEADER_HEIGHT
                    + NODE_TOP_PAD
                    + IF_CONDITION_ROW_H
                    + NODE_ROW_GAP
                    + conditions_block_h
                    + NODE_ROW_GAP
                    + NODE_ADD_BTN_H
                    + NODE_ROW_GAP
                    + IF_BRANCH_ROW_H
                    + NODE_ROW_GAP
                    + IF_BRANCH_ROW_H * 0.5;
                self.cached_ports_in.push(PortDef::new(
                    base + 0,
                    "flow_in",
                    PortSide::Input,
                    0,
                    PortType::Flow,
                    flow_y,
                ));
                self.cached_ports_in.push(PortDef::new(
                    base + 10,
                    "condition",
                    PortSide::Input,
                    1,
                    PortType::Data(VariableType::Boolean),
                    condition_y,
                ));
                self.cached_ports_out.push(PortDef::new(
                    base + 1,
                    "true",
                    PortSide::Output,
                    0,
                    PortType::Flow,
                    true_y,
                ));
                self.cached_ports_out.push(PortDef::new(
                    base + 2,
                    "false",
                    PortSide::Output,
                    1,
                    PortType::Flow,
                    false_y,
                ));
                self.cached_height = HEADER_HEIGHT
                    + NODE_TOP_PAD
                    + IF_CONDITION_ROW_H
                    + NODE_ROW_GAP
                    + conditions_block_h
                    + NODE_ROW_GAP
                    + NODE_ADD_BTN_H
                    + NODE_ROW_GAP
                    + IF_BRANCH_ROW_H
                    + NODE_ROW_GAP
                    + IF_BRANCH_ROW_H
                    + NODE_BOTTOM_PAD;
            }

            ActionNodeKind::Match { arms, .. } => {
                let flow_y = HEADER_HEIGHT * 0.5;
                let value_y = HEADER_HEIGHT
                    + NODE_TOP_PAD
                    + MATCH_VALUE_ROW_H
                    + NODE_ROW_GAP
                    + MATCH_VALUE_ROW_H * 0.5;
                let literal_row_block = if matches!(
                    self.authored_match_subject,
                    Some(AuthoredValueSource::Literal(_))
                ) {
                    MATCH_VALUE_ROW_H + NODE_ROW_GAP
                } else {
                    0.0
                };
                let arms_base_y = HEADER_HEIGHT
                    + NODE_TOP_PAD
                    + MATCH_VALUE_ROW_H
                    + NODE_ROW_GAP
                    + MATCH_VALUE_ROW_H
                    + NODE_ROW_GAP
                    + literal_row_block;
                let arms_start_y = arms_base_y + MATCH_ARM_ROW_H * 0.5;
                self.cached_ports_in.push(PortDef::new(
                    base + 0,
                    "flow_in",
                    PortSide::Input,
                    0,
                    PortType::Flow,
                    flow_y,
                ));
                self.cached_ports_in.push(PortDef::new(
                    base + 10,
                    "value",
                    PortSide::Input,
                    1,
                    PortType::Data(VariableType::String),
                    value_y,
                ));
                for (i, arm) in arms.iter().enumerate() {
                    let y = arms_start_y + i as f32 * (MATCH_ARM_ROW_H + NODE_ROW_GAP);
                    self.cached_ports_out.push(PortDef::new(
                        base + 1 + i as u64,
                        arm.clone(),
                        PortSide::Output,
                        i,
                        PortType::Flow,
                        y,
                    ));
                }
                let default_slot = arms.len();
                let default_y = arms_base_y
                    + arms.len() as f32 * MATCH_ARM_ROW_H
                    + arms.len() as f32 * NODE_ROW_GAP
                    + MATCH_DEFAULT_ROW_H * 0.5;
                self.cached_ports_out.push(PortDef::new(
                    base + 1 + arms.len() as u64,
                    "default",
                    PortSide::Output,
                    default_slot,
                    PortType::Flow,
                    default_y,
                ));
                self.cached_height = HEADER_HEIGHT
                    + NODE_TOP_PAD
                    + MATCH_VALUE_ROW_H
                    + NODE_ROW_GAP
                    + MATCH_VALUE_ROW_H
                    + NODE_ROW_GAP
                    + literal_row_block
                    + arms.len() as f32 * MATCH_ARM_ROW_H
                    + arms.len() as f32 * NODE_ROW_GAP
                    + MATCH_DEFAULT_ROW_H
                    + NODE_BOTTOM_PAD;
            }

            ActionNodeKind::StringLiteral { .. } => {
                self.cached_ports_out.push(PortDef::new(
                    base + 20,
                    "value",
                    PortSide::Output,
                    0,
                    PortType::Data(VariableType::String),
                    port_y_offset(0),
                ));
                self.cached_height = node_height(1);
            }

            ActionNodeKind::NumberLiteral { .. } => {
                self.cached_ports_out.push(PortDef::new(
                    base + 20,
                    "value",
                    PortSide::Output,
                    0,
                    PortType::Data(VariableType::Number),
                    port_y_offset(0),
                ));
                self.cached_height = node_height(1);
            }

            ActionNodeKind::BoolLiteral { .. } => {
                self.cached_ports_out.push(PortDef::new(
                    base + 20,
                    "value",
                    PortSide::Output,
                    0,
                    PortType::Data(VariableType::Boolean),
                    port_y_offset(0),
                ));
                self.cached_height = node_height(1);
            }

            ActionNodeKind::EnumLiteral { .. } => {
                self.cached_ports_out.push(PortDef::new(
                    base + 20,
                    "value",
                    PortSide::Output,
                    0,
                    PortType::Data(VariableType::String),
                    port_y_offset(0),
                ));
                self.cached_height = node_height(1);
            }

            ActionNodeKind::Compare { rhs, operator, .. } => {
                // Input: `value` (slot 0)
                self.cached_ports_in.push(PortDef::new(
                    base + 10,
                    "value",
                    PortSide::Input,
                    0,
                    PortType::Data(VariableType::String),
                    port_y_offset(0),
                ));
                let mut max_slots = 1;
                // Input: `rhs` (slot 1) only if rhs mode is FromPort and operator needs rhs
                if *rhs == CompareRhs::FromPort && operator.needs_rhs() {
                    self.cached_ports_in.push(PortDef::new(
                        base + 11,
                        "rhs",
                        PortSide::Input,
                        1,
                        PortType::Data(VariableType::String),
                        port_y_offset(1),
                    ));
                    max_slots = 2;
                }
                // Output: `result` Bool (slot 0)
                self.cached_ports_out.push(PortDef::new(
                    base + 20,
                    "result",
                    PortSide::Output,
                    0,
                    PortType::Data(VariableType::Boolean),
                    port_y_offset(0),
                ));
                self.cached_height = node_height(max_slots);
            }

            ActionNodeKind::LogicAnd | ActionNodeKind::LogicOr => {
                self.cached_ports_in.push(PortDef::new(
                    base + 10,
                    "a",
                    PortSide::Input,
                    0,
                    PortType::Data(VariableType::Boolean),
                    port_y_offset(0),
                ));
                self.cached_ports_in.push(PortDef::new(
                    base + 11,
                    "b",
                    PortSide::Input,
                    1,
                    PortType::Data(VariableType::Boolean),
                    port_y_offset(1),
                ));
                self.cached_ports_out.push(PortDef::new(
                    base + 20,
                    "result",
                    PortSide::Output,
                    0,
                    PortType::Data(VariableType::Boolean),
                    port_y_offset(0),
                ));
                self.cached_height = node_height(2);
            }

            ActionNodeKind::LogicNot => {
                self.cached_ports_in.push(PortDef::new(
                    base + 10,
                    "value",
                    PortSide::Input,
                    0,
                    PortType::Data(VariableType::Boolean),
                    port_y_offset(0),
                ));
                self.cached_ports_out.push(PortDef::new(
                    base + 20,
                    "result",
                    PortSide::Output,
                    0,
                    PortType::Data(VariableType::Boolean),
                    port_y_offset(0),
                ));
                self.cached_height = node_height(1);
            }

            ActionNodeKind::CallAction { .. } => {
                self.cached_ports_in.push(PortDef::new(
                    base + 0,
                    "flow_in",
                    PortSide::Input,
                    0,
                    PortType::Flow,
                    port_y_offset(0),
                ));
                self.cached_ports_out.push(PortDef::new(
                    base + 1,
                    "flow_out",
                    PortSide::Output,
                    0,
                    PortType::Flow,
                    port_y_offset(0),
                ));
                self.cached_height = node_height(1);
            }
            ActionNodeKind::CallFlow { .. } => {
                let y = HEADER_HEIGHT + NODE_TOP_PAD + CALL_FLOW_ROW_H * 0.5;
                self.cached_ports_in.push(PortDef::new(
                    base + 0,
                    "flow_in",
                    PortSide::Input,
                    0,
                    PortType::Flow,
                    y,
                ));
                self.cached_ports_out.push(PortDef::new(
                    base + 1,
                    "flow_out",
                    PortSide::Output,
                    0,
                    PortType::Flow,
                    y,
                ));
                self.cached_height =
                    HEADER_HEIGHT + NODE_TOP_PAD + CALL_FLOW_ROW_H + NODE_BOTTOM_PAD;
            }
            ActionNodeKind::UpdateState { assignments } => {
                self.cached_ports_in.push(PortDef::new(
                    base + 0,
                    "flow_in",
                    PortSide::Input,
                    0,
                    PortType::Flow,
                    port_y_offset(0),
                ));
                self.cached_ports_out.push(PortDef::new(
                    base + 1,
                    "flow_out",
                    PortSide::Output,
                    0,
                    PortType::Flow,
                    port_y_offset(0),
                ));
                self.cached_height = node_height((assignments.len() + 1).max(1));
            }
            ActionNodeKind::Expression { .. } => {
                // Data-only: 1 output "value" port, no flow ports
                self.cached_ports_out.push(PortDef::new(
                    base + 20,
                    "value",
                    PortSide::Output,
                    0,
                    PortType::Data(VariableType::String),
                    port_y_offset(0),
                ));
                self.cached_height = node_height(2);
            }
            ActionNodeKind::LegacyGetState { .. } => {
                // No ports — this node is dropped on migration load
                self.cached_height = node_height(1);
            }
        }
    }

    /// Converts this node to a `FlowNode` suitable for passing to `FlowEditor`.
    pub fn to_flow_node(&self) -> FlowNode {
        let mut all_ports = self.cached_ports_in.clone();
        all_ports.extend(self.cached_ports_out.clone());

        let (inputs, outputs): (Vec<_>, Vec<_>) = all_ports
            .into_iter()
            .partition(|p| p.side == PortSide::Input);

        FlowNode {
            id: NodeId(self.id),
            position: self.position,
            width: node_width(&self.kind),
            cached_height: self.cached_height,
            inputs,
            outputs,
            selected: self.selected,
            enabled: self.enabled,
            title: self.kind.display_name().to_string(),
            kind_label: self
                .kind_label_override
                .as_deref()
                .unwrap_or_else(|| self.kind.kind_label())
                .to_string(),
            accent_color: self.accent_color_for_kind(),
        }
    }

    pub fn is_trigger(&self) -> bool {
        matches!(self.kind, ActionNodeKind::Trigger { .. })
    }

    /// Returns the flow output slot for the trigger row at `row_idx` in a WidgetEvent trigger.
    /// Slot = number of complete rows (target.is_some()) before row_idx.
    /// Matches the slot assignment in `rebuild_ports_for_widget_event`.
    pub fn widget_event_row_slot(rows: &[WidgetEventRow], row_idx: usize) -> usize {
        rows[..row_idx]
            .iter()
            .filter(|r| r.target.is_some())
            .count()
    }

    /// Called instead of `rebuild_ports()` for Trigger nodes when the trigger is `WidgetEvent`.
    /// Generates one flow output port per complete row (both event_type and target set).
    pub fn rebuild_ports_for_widget_event(&mut self, rows: &[WidgetEventRow]) {
        let base = self.id * 1_000;
        self.cached_ports_in.clear();
        self.cached_ports_out.clear();

        let mut slot = 0usize;
        for (i, row) in rows.iter().enumerate() {
            if row.target.is_some() {
                let y = HEADER_HEIGHT
                    + NODE_TOP_PAD
                    + i as f32 * (TRIGGER_EVENT_ROW_H + NODE_ROW_GAP)
                    + TRIGGER_EVENT_ROW_H * 0.5;
                // Stable port ID derived from the row's UUID lower bits.
                let port_id = base + 500 + (row.id.as_u128() as u64 & 0xFFF);
                self.cached_ports_out.push(PortDef::new(
                    port_id,
                    row.event_type.as_str(),
                    PortSide::Output,
                    slot,
                    PortType::Flow,
                    y,
                ));
                slot += 1;
            }
        }

        let n = rows.len().max(1) as f32;
        self.cached_height = (HEADER_HEIGHT
            + NODE_TOP_PAD
            + n * (TRIGGER_EVENT_ROW_H + NODE_ROW_GAP)
            + NODE_ADD_BTN_H
            + NODE_BOTTOM_PAD)
            .max(130.0);
    }

    /// Returns the accent colour for the node's header stripe based on its kind.
    fn accent_color_for_kind(&self) -> Color {
        match &self.kind {
            ActionNodeKind::Trigger { .. } => Color::from_rgb8(255, 116, 61),
            ActionNodeKind::StateMutation { .. }
            | ActionNodeKind::SetState { .. }
            | ActionNodeKind::UpdateState { .. }
            | ActionNodeKind::NavigateToView { .. }
            | ActionNodeKind::CallFlow { .. }
            | ActionNodeKind::CallAction { .. } => Color::from_rgb8(99, 179, 255),
            ActionNodeKind::Conditional
            | ActionNodeKind::Match { .. }
            | ActionNodeKind::Compare { .. }
            | ActionNodeKind::LogicAnd
            | ActionNodeKind::LogicOr
            | ActionNodeKind::LogicNot => Color::from_rgb8(255, 190, 80),
            ActionNodeKind::StringLiteral { .. }
            | ActionNodeKind::NumberLiteral { .. }
            | ActionNodeKind::BoolLiteral { .. }
            | ActionNodeKind::EnumLiteral { .. }
            | ActionNodeKind::Expression { .. } => Color::from_rgb8(112, 197, 146),
            _ => Color::TRANSPARENT,
        }
    }
}

fn node_width(kind: &ActionNodeKind) -> f32 {
    match kind {
        ActionNodeKind::Trigger { .. } => 380.0,
        ActionNodeKind::NavigateToView { .. } => 430.0,
        ActionNodeKind::StateMutation { .. } | ActionNodeKind::SetState { .. } => 460.0,
        ActionNodeKind::Conditional => 420.0,
        ActionNodeKind::Match { .. } => 430.0,
        ActionNodeKind::StringLiteral { .. }
        | ActionNodeKind::NumberLiteral { .. }
        | ActionNodeKind::BoolLiteral { .. } => 220.0,
        ActionNodeKind::Compare { .. } => 260.0,
        ActionNodeKind::LogicAnd | ActionNodeKind::LogicOr | ActionNodeKind::LogicNot => 200.0,
        ActionNodeKind::EnumLiteral { .. } => 220.0,
        ActionNodeKind::CallAction { .. } | ActionNodeKind::CallFlow { .. } => 320.0,
        _ => 280.0,
    }
}

// ─── ActionGraph ──────────────────────────────────────────────────────────────

/// The complete action graph for a single widget event.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActionGraph {
    pub nodes: Vec<ActionNodeData>,
    pub edges: Vec<ActionEdge>,
    pub z_order: Vec<ActionNodeId>,
    pub next_id: u64,
    /// Persisted viewport (pan + zoom) so the user's view is remembered.
    pub pan: [f32; 2],
    pub zoom: f32,
}

impl ActionGraph {
    /// Creates a new graph for a callable-flow entry trigger.
    pub fn new_named_action() -> Self {
        Self::new_with_trigger("entry")
    }

    pub fn new_with_trigger(event_name: &str) -> Self {
        let mut graph = Self {
            zoom: 1.0,
            pan: [40.0, 40.0],
            ..Default::default()
        };
        let trigger = ActionNodeData::new(
            1,
            ActionNodeKind::Trigger {
                event_name: event_name.to_string(),
                output_ports: crate::action_system::events::trigger_ports_for_event(event_name),
            },
            Point::new(60.0, 80.0),
        );
        graph.z_order.push(1);
        graph.nodes.push(trigger);
        graph.next_id = 2;
        graph
    }

    pub fn viewport(&self) -> Viewport2D {
        Viewport2D {
            pan: Vector::new(self.pan[0], self.pan[1]),
            zoom: self.zoom,
        }
    }

    pub fn set_viewport(&mut self, vp: Viewport2D) {
        self.pan = [vp.pan.x, vp.pan.y];
        self.zoom = vp.zoom;
    }

    /// Returns the trigger node id, if one exists.
    pub fn trigger_node_id(&self) -> Option<ActionNodeId> {
        self.nodes.iter().find(|n| n.is_trigger()).map(|n| n.id)
    }

    /// Returns the ID of the next flow node reached via `output_slot` of `node_id`.
    pub fn flow_successors(
        &self,
        node_id: ActionNodeId,
        output_slot: usize,
    ) -> Option<ActionNodeId> {
        self.flow_successors_with_input_slot(node_id, output_slot)
            .map(|(next_id, _)| next_id)
    }

    /// Returns all `(next_node_id, input_slot_of_next_node)` reached via `output_slot` of `node_id`
    /// in edge insertion order.
    pub fn flow_successors_with_input_slots(
        &self,
        node_id: ActionNodeId,
        output_slot: usize,
    ) -> Vec<(ActionNodeId, usize)> {
        let Some(node) = self.nodes.iter().find(|n| n.id == node_id) else {
            return Vec::new();
        };
        let Some(flow_port) = node
            .cached_ports_out
            .iter()
            .find(|p| matches!(p.kind, PortType::Flow) && p.slot == output_slot)
        else {
            return Vec::new();
        };

        self.edges
            .iter()
            .filter_map(|edge| {
                if edge.from_node != node_id || edge.from_port != flow_port.id.0 {
                    return None;
                }
                let next_node = self.nodes.iter().find(|n| n.id == edge.to_node)?;
                let input = next_node
                    .cached_ports_in
                    .iter()
                    .find(|p| p.id.0 == edge.to_port)?;
                if !matches!(input.kind, PortType::Flow) {
                    return None;
                }
                Some((edge.to_node, input.slot))
            })
            .collect()
    }

    /// Returns `(next_node_id, input_slot_of_next_node)` reached via `output_slot` of `node_id`.
    pub fn flow_successors_with_input_slot(
        &self,
        node_id: ActionNodeId,
        output_slot: usize,
    ) -> Option<(ActionNodeId, usize)> {
        self.flow_successors_with_input_slots(node_id, output_slot)
            .into_iter()
            .next()
    }

    /// Validates whether a connection is semantically valid for the current graph topology.
    pub fn can_connect_ports(
        &self,
        from_node: ActionNodeId,
        from_port: u64,
        to_node: ActionNodeId,
        to_port: u64,
    ) -> bool {
        if from_node == to_node {
            return false;
        }

        let Some(source_node) = self.nodes.iter().find(|n| n.id == from_node) else {
            return false;
        };
        let Some(dest_node) = self.nodes.iter().find(|n| n.id == to_node) else {
            return false;
        };

        let Some(source_port) = source_node
            .cached_ports_out
            .iter()
            .find(|p| p.id.0 == from_port)
        else {
            return false;
        };
        let Some(dest_port) = dest_node.cached_ports_in.iter().find(|p| p.id.0 == to_port) else {
            return false;
        };

        matches!(
            (&source_port.kind, &dest_port.kind),
            (PortType::Flow, PortType::Flow) | (PortType::Data(_), PortType::Data(_))
        )
    }

    /// Connects source output -> destination input if valid.
    /// - Rejects invalid/stale/self-loop connections
    /// - Rejects exact duplicate edges
    /// - Preserves single incoming edge per destination input port
    pub fn connect_ports(
        &mut self,
        from_node: ActionNodeId,
        from_port: u64,
        to_node: ActionNodeId,
        to_port: u64,
    ) -> bool {
        if !self.can_connect_ports(from_node, from_port, to_node, to_port) {
            return false;
        }
        if self.edges.iter().any(|e| {
            e.from_node == from_node
                && e.from_port == from_port
                && e.to_node == to_node
                && e.to_port == to_port
        }) {
            return false;
        }

        self.edges
            .retain(|e| !(e.to_node == to_node && e.to_port == to_port));
        self.edges.push(ActionEdge {
            from_node,
            from_port,
            to_node,
            to_port,
        });
        true
    }

    pub fn remove_edge(&mut self, edge: Edge) -> bool {
        let before = self.edges.len();
        self.edges.retain(|existing| {
            !(existing.from_node == edge.from_node.0
                && existing.from_port == edge.from_port.0
                && existing.to_node == edge.to_node.0
                && existing.to_port == edge.to_port.0)
        });
        self.edges.len() != before
    }

    /// Collects the `FlowNode`s in z_order for passing to `FlowEditor`.
    pub fn flow_nodes(&self) -> Vec<FlowNode> {
        self.nodes.iter().map(|n| n.to_flow_node()).collect()
    }

    /// Collects the `Edge`s for passing to `FlowEditor`.
    pub fn flow_edges(&self) -> Vec<Edge> {
        self.edges.iter().map(|e| e.to_flow_edge()).collect()
    }
}

// ─── Palette entries for the action editor ───────────────────────────────────

pub fn action_palette_entries() -> Vec<PaletteEntry> {
    vec![
        PaletteEntry {
            id: 1,
            label: "State Mutation",
            kind_label: "Action",
            accent_color: Color::from_rgb8(99, 179, 255),
        },
        PaletteEntry {
            id: 3,
            label: "Navigate to View",
            kind_label: "Action",
            accent_color: Color::from_rgb8(161, 120, 255),
        },
        PaletteEntry {
            id: 4,
            label: "If",
            kind_label: "Control",
            accent_color: Color::from_rgb8(255, 190, 80),
        },
        PaletteEntry {
            id: 8,
            label: "Match",
            kind_label: "Control",
            accent_color: Color::from_rgb8(255, 190, 80),
        },
        PaletteEntry {
            id: 14,
            label: "Call Flow",
            kind_label: "Action",
            accent_color: Color::from_rgb8(99, 179, 255),
        },
    ]
}

pub fn action_node_from_palette_id(
    id: u64,
    position: Point,
    next_id: ActionNodeId,
) -> Option<ActionNodeData> {
    let kind = match id {
        1 => ActionNodeKind::StateMutation {
            assignments: vec![crate::action_system::node_kinds::StateAssignment {
                target: None,
                value_source: crate::action_system::node_kinds::ValueSource::Literal(
                    crate::action_system::node_kinds::ActionValue::String(String::new()),
                ),
            }],
        },
        3 => ActionNodeKind::NavigateToView {
            targets: vec![None],
        },
        4 => ActionNodeKind::Conditional,
        8 => ActionNodeKind::Match {
            arms: vec!["arm 1".to_string()],
            enum_type: None,
        },
        14 => ActionNodeKind::CallFlow { flow_id: None },
        _ => return None,
    };
    let mut node = ActionNodeData::new(next_id, kind, position);
    match id {
        4 => {
            node.authored_condition = None;
            node.authored_conditions = vec![AuthoredCondition::default()];
            node.authored_condition_join = ConditionJoinMode::All;
        }
        8 => {
            node.authored_match_subject = Some(AuthoredValueSource::Literal(ActionValue::String(
                String::new(),
            )));
        }
        _ => {}
    }
    Some(node)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::action_system::node_kinds::{ActionValue, ValueSource};
    use std::collections::HashMap;

    fn set_state_node(id: ActionNodeId) -> ActionNodeData {
        ActionNodeData::new(
            id,
            ActionNodeKind::StateMutation {
                assignments: vec![crate::action_system::node_kinds::StateAssignment {
                    target: None,
                    value_source: ValueSource::Literal(ActionValue::String(String::new())),
                }],
            },
            Point::new(120.0 + id as f32 * 20.0, 120.0),
        )
    }

    fn call_flow_node(id: ActionNodeId, flow_id: uuid::Uuid) -> ActionNodeData {
        ActionNodeData::new(
            id,
            ActionNodeKind::CallFlow {
                flow_id: Some(flow_id),
            },
            Point::new(120.0 + id as f32 * 20.0, 120.0),
        )
    }

    fn flow_out(node: &ActionNodeData) -> u64 {
        node.cached_ports_out
            .iter()
            .find(|p| matches!(p.kind, PortType::Flow))
            .expect("flow out")
            .id
            .0
    }

    fn flow_in(node: &ActionNodeData) -> u64 {
        node.cached_ports_in
            .iter()
            .find(|p| matches!(p.kind, PortType::Flow))
            .expect("flow in")
            .id
            .0
    }

    fn approx_eq(a: f32, b: f32) {
        assert!(
            (a - b).abs() < 0.01,
            "expected {a} ~= {b} (delta={})",
            (a - b).abs()
        );
    }

    #[test]
    fn state_mutation_ports_align_with_header_and_assignment_rows() {
        let node = ActionNodeData::new(
            7,
            ActionNodeKind::StateMutation {
                assignments: vec![
                    crate::action_system::node_kinds::StateAssignment {
                        target: None,
                        value_source: ValueSource::Literal(ActionValue::String(String::new())),
                    },
                    crate::action_system::node_kinds::StateAssignment {
                        target: None,
                        value_source: ValueSource::Literal(ActionValue::String(String::new())),
                    },
                ],
            },
            Point::new(0.0, 0.0),
        );
        assert_eq!(node.cached_ports_in.len(), 3);
        assert_eq!(node.cached_ports_out.len(), 1);
        approx_eq(
            node.cached_ports_in[0].y_offset,
            HEADER_HEIGHT + NODE_TOP_PAD + STATE_MUTATION_HEADER_H * 0.5,
        );
        approx_eq(
            node.cached_ports_in[1].y_offset,
            HEADER_HEIGHT
                + NODE_TOP_PAD
                + STATE_MUTATION_HEADER_H
                + NODE_ROW_GAP
                + STATE_MUTATION_ROW_H * 0.5,
        );
        approx_eq(
            node.cached_ports_in[2].y_offset,
            HEADER_HEIGHT
                + NODE_TOP_PAD
                + STATE_MUTATION_HEADER_H
                + NODE_ROW_GAP
                + (STATE_MUTATION_ROW_H + NODE_ROW_GAP)
                + STATE_MUTATION_ROW_H * 0.5,
        );
        approx_eq(
            node.cached_ports_out[0].y_offset,
            node.cached_ports_in[0].y_offset,
        );
    }

    #[test]
    fn navigate_to_view_ports_align_with_each_navigation_row() {
        let node = ActionNodeData::new(
            8,
            ActionNodeKind::NavigateToView {
                targets: vec![None, None],
            },
            Point::new(0.0, 0.0),
        );
        assert_eq!(node.cached_ports_in.len(), 2);
        assert_eq!(node.cached_ports_out.len(), 2);
        let step = NAVIGATE_ROW_H + NODE_ROW_GAP;
        approx_eq(
            node.cached_ports_in[0].y_offset,
            HEADER_HEIGHT + NODE_TOP_PAD + NAVIGATE_ROW_H * 0.5,
        );
        approx_eq(
            node.cached_ports_in[1].y_offset,
            HEADER_HEIGHT + NODE_TOP_PAD + NAVIGATE_ROW_H * 0.5 + step,
        );
        approx_eq(
            node.cached_ports_out[0].y_offset,
            node.cached_ports_in[0].y_offset,
        );
        approx_eq(
            node.cached_ports_out[1].y_offset,
            node.cached_ports_in[1].y_offset,
        );
    }

    #[test]
    fn conditional_ports_align_to_condition_then_else_rows() {
        let node = ActionNodeData::new(9, ActionNodeKind::Conditional, Point::new(0.0, 0.0));
        assert_eq!(node.cached_ports_in.len(), 2);
        assert_eq!(node.cached_ports_out.len(), 2);
        let flow_in = node
            .cached_ports_in
            .iter()
            .find(|p| p.label == "flow_in")
            .expect("flow_in");
        let condition = node
            .cached_ports_in
            .iter()
            .find(|p| p.label == "condition")
            .expect("condition");
        let then_out = node
            .cached_ports_out
            .iter()
            .find(|p| p.label == "true")
            .expect("true");
        let else_out = node
            .cached_ports_out
            .iter()
            .find(|p| p.label == "false")
            .expect("false");

        approx_eq(flow_in.y_offset, HEADER_HEIGHT * 0.5);
        approx_eq(
            condition.y_offset,
            HEADER_HEIGHT
                + NODE_TOP_PAD
                + IF_CONDITION_ROW_H
                + NODE_ROW_GAP
                + IF_CONDITION_ROW_H * 0.5,
        );
        approx_eq(
            then_out.y_offset,
            HEADER_HEIGHT
                + NODE_TOP_PAD
                + IF_CONDITION_ROW_H
                + NODE_ROW_GAP
                + IF_CONDITION_ROW_H
                + NODE_ROW_GAP
                + NODE_ADD_BTN_H
                + NODE_ROW_GAP
                + IF_BRANCH_ROW_H * 0.5,
        );
        approx_eq(
            else_out.y_offset,
            HEADER_HEIGHT
                + NODE_TOP_PAD
                + IF_CONDITION_ROW_H
                + NODE_ROW_GAP
                + IF_CONDITION_ROW_H
                + NODE_ROW_GAP
                + NODE_ADD_BTN_H
                + NODE_ROW_GAP
                + IF_BRANCH_ROW_H
                + NODE_ROW_GAP
                + IF_BRANCH_ROW_H * 0.5,
        );
    }

    #[test]
    fn conditional_multiple_authored_rows_shift_branch_ports_and_height() {
        let mut node = ActionNodeData::new(52, ActionNodeKind::Conditional, Point::new(0.0, 0.0));
        node.authored_conditions = vec![
            AuthoredCondition::default(),
            AuthoredCondition::default(),
            AuthoredCondition::default(),
        ];
        node.rebuild_ports();

        let then_out = node
            .cached_ports_out
            .iter()
            .find(|p| p.label == "true")
            .expect("true");
        let else_out = node
            .cached_ports_out
            .iter()
            .find(|p| p.label == "false")
            .expect("false");
        let conditions_block_h = 3.0 * IF_CONDITION_ROW_H + 2.0 * NODE_ROW_GAP;
        approx_eq(
            then_out.y_offset,
            HEADER_HEIGHT
                + NODE_TOP_PAD
                + IF_CONDITION_ROW_H
                + NODE_ROW_GAP
                + conditions_block_h
                + NODE_ROW_GAP
                + NODE_ADD_BTN_H
                + NODE_ROW_GAP
                + IF_BRANCH_ROW_H * 0.5,
        );
        approx_eq(
            else_out.y_offset,
            HEADER_HEIGHT
                + NODE_TOP_PAD
                + IF_CONDITION_ROW_H
                + NODE_ROW_GAP
                + conditions_block_h
                + NODE_ROW_GAP
                + NODE_ADD_BTN_H
                + NODE_ROW_GAP
                + IF_BRANCH_ROW_H
                + NODE_ROW_GAP
                + IF_BRANCH_ROW_H * 0.5,
        );
    }

    #[test]
    fn match_ports_have_one_output_per_arm_plus_default_aligned_to_rows() {
        let node = ActionNodeData::new(
            10,
            ActionNodeKind::Match {
                arms: vec!["One".to_string(), "Two".to_string()],
                enum_type: None,
            },
            Point::new(0.0, 0.0),
        );
        assert_eq!(node.cached_ports_in.len(), 2);
        assert_eq!(node.cached_ports_out.len(), 3);
        assert_eq!(node.cached_ports_out[0].label, "One");
        assert_eq!(node.cached_ports_out[1].label, "Two");
        assert_eq!(node.cached_ports_out[2].label, "default");
        approx_eq(
            node.cached_ports_out[0].y_offset,
            HEADER_HEIGHT
                + NODE_TOP_PAD
                + MATCH_VALUE_ROW_H
                + NODE_ROW_GAP
                + MATCH_VALUE_ROW_H
                + NODE_ROW_GAP
                + MATCH_ARM_ROW_H * 0.5,
        );
        approx_eq(
            node.cached_ports_out[1].y_offset,
            HEADER_HEIGHT
                + NODE_TOP_PAD
                + MATCH_VALUE_ROW_H
                + NODE_ROW_GAP
                + MATCH_VALUE_ROW_H
                + NODE_ROW_GAP
                + MATCH_ARM_ROW_H * 0.5
                + MATCH_ARM_ROW_H
                + NODE_ROW_GAP,
        );
        approx_eq(
            node.cached_ports_out[2].y_offset,
            HEADER_HEIGHT
                + NODE_TOP_PAD
                + MATCH_VALUE_ROW_H
                + NODE_ROW_GAP
                + MATCH_VALUE_ROW_H
                + NODE_ROW_GAP
                + 2.0 * MATCH_ARM_ROW_H
                + 2.0 * NODE_ROW_GAP
                + MATCH_DEFAULT_ROW_H * 0.5,
        );
    }

    #[test]
    fn match_literal_subject_adds_extra_row_before_arms_and_shifts_branch_ports() {
        let mut node = ActionNodeData::new(
            51,
            ActionNodeKind::Match {
                arms: vec!["One".to_string()],
                enum_type: None,
            },
            Point::new(0.0, 0.0),
        );
        node.authored_match_subject = Some(AuthoredValueSource::Literal(ActionValue::String(
            "One".to_string(),
        )));
        node.rebuild_ports();
        let arm = node
            .cached_ports_out
            .iter()
            .find(|p| p.label == "One")
            .expect("arm port");
        approx_eq(
            arm.y_offset,
            HEADER_HEIGHT
                + NODE_TOP_PAD
                + MATCH_VALUE_ROW_H
                + NODE_ROW_GAP
                + MATCH_VALUE_ROW_H
                + NODE_ROW_GAP
                + MATCH_VALUE_ROW_H
                + NODE_ROW_GAP
                + MATCH_ARM_ROW_H * 0.5,
        );
    }

    #[test]
    fn call_flow_ports_align_to_single_picker_row() {
        let node = ActionNodeData::new(
            11,
            ActionNodeKind::CallFlow { flow_id: None },
            Point::new(0.0, 0.0),
        );
        assert_eq!(node.cached_ports_in.len(), 1);
        assert_eq!(node.cached_ports_out.len(), 1);
        let expected_y = HEADER_HEIGHT + NODE_TOP_PAD + CALL_FLOW_ROW_H * 0.5;
        approx_eq(node.cached_ports_in[0].y_offset, expected_y);
        approx_eq(node.cached_ports_out[0].y_offset, expected_y);
    }

    #[test]
    fn widget_event_trigger_ports_align_with_event_rows_without_kind_row_offset() {
        let mut trigger = ActionNodeData::new(
            12,
            ActionNodeKind::Trigger {
                event_name: "widget_event".to_string(),
                output_ports: Vec::new(),
            },
            Point::new(0.0, 0.0),
        );
        let rows = vec![
            WidgetEventRow {
                id: uuid::Uuid::new_v4(),
                event_type: "on_press".to_string(),
                target: Some((uuid::Uuid::new_v4(), 1)),
            },
            WidgetEventRow {
                id: uuid::Uuid::new_v4(),
                event_type: "on_press".to_string(),
                target: Some((uuid::Uuid::new_v4(), 2)),
            },
        ];
        trigger.rebuild_ports_for_widget_event(&rows);
        assert_eq!(trigger.cached_ports_out.len(), 2);
        let y0 = HEADER_HEIGHT + NODE_TOP_PAD + TRIGGER_EVENT_ROW_H * 0.5;
        let y1 = y0 + TRIGGER_EVENT_ROW_H + NODE_ROW_GAP;
        approx_eq(trigger.cached_ports_out[0].y_offset, y0);
        approx_eq(trigger.cached_ports_out[1].y_offset, y1);
    }

    #[test]
    fn match_slots_are_zero_based_with_default_at_arms_len() {
        let node = ActionNodeData::new(
            42,
            ActionNodeKind::Match {
                arms: vec!["One".to_string(), "Two".to_string()],
                enum_type: None,
            },
            Point::new(0.0, 0.0),
        );
        assert_eq!(node.cached_ports_out.len(), 3);
        assert_eq!(node.cached_ports_out[0].label, "One");
        assert_eq!(node.cached_ports_out[0].slot, 0);
        assert_eq!(node.cached_ports_out[1].label, "Two");
        assert_eq!(node.cached_ports_out[1].slot, 1);
        assert_eq!(node.cached_ports_out[2].label, "default");
        assert_eq!(node.cached_ports_out[2].slot, 2);
    }

    #[test]
    fn can_connect_ports_rejects_invalid_direction_kind_and_stale_ports() {
        let mut graph = ActionGraph::new_with_trigger("on_press");
        let set_state = set_state_node(2);
        let literal = ActionNodeData::new(
            3,
            ActionNodeKind::StringLiteral {
                value: "x".to_string(),
            },
            Point::new(320.0, 120.0),
        );
        graph.nodes.push(set_state.clone());
        graph.nodes.push(literal.clone());
        graph.z_order.extend([2, 3]);
        graph.next_id = 4;

        let trigger = graph.nodes.iter().find(|n| n.id == 1).expect("trigger");
        let trigger_flow_out = flow_out(trigger);
        let set_state_flow_in = flow_in(&set_state);
        let set_state_value_in = set_state
            .cached_ports_in
            .iter()
            .find(|p| p.label == "value_0")
            .expect("setstate value in")
            .id
            .0;
        let literal_data_out = literal.cached_ports_out[0].id.0;

        assert!(graph.can_connect_ports(1, trigger_flow_out, 2, set_state_flow_in));
        assert!(!graph.can_connect_ports(1, trigger_flow_out, 2, set_state_value_in));
        assert!(!graph.can_connect_ports(3, literal_data_out, 2, set_state_flow_in));
        assert!(!graph.can_connect_ports(999, trigger_flow_out, 2, set_state_flow_in));
        assert!(!graph.can_connect_ports(1, 123_456_789, 2, set_state_flow_in));
        assert!(!graph.can_connect_ports(1, trigger_flow_out, 2, 987_654_321));
    }

    #[test]
    fn connect_ports_rejects_self_loop_and_duplicates() {
        let mut graph = ActionGraph::new_with_trigger("on_press");
        let set_state = set_state_node(2);
        let set_state_flow_out = flow_out(&set_state);
        let set_state_flow_in = flow_in(&set_state);
        graph.nodes.push(set_state);
        graph.z_order.push(2);
        graph.next_id = 3;

        assert!(!graph.connect_ports(2, set_state_flow_out, 2, set_state_flow_in));

        let trigger_flow_out = {
            let trigger = graph.nodes.iter().find(|n| n.id == 1).expect("trigger");
            flow_out(trigger)
        };
        assert!(graph.connect_ports(1, trigger_flow_out, 2, set_state_flow_in));
        assert!(!graph.connect_ports(1, trigger_flow_out, 2, set_state_flow_in));
        assert_eq!(graph.edges.len(), 1);
    }

    #[test]
    fn connect_ports_enforces_single_incoming_edge_per_input() {
        let mut graph = ActionGraph::new_with_trigger("on_press");
        let set_state = set_state_node(2);
        let call_action = call_flow_node(3, uuid::Uuid::new_v4());
        graph.nodes.push(set_state.clone());
        graph.nodes.push(call_action.clone());
        graph.z_order.extend([2, 3]);
        graph.next_id = 4;

        let trigger_flow_out = {
            let trigger = graph.nodes.iter().find(|n| n.id == 1).expect("trigger");
            flow_out(trigger)
        };
        let set_state_flow_in = flow_in(&set_state);
        let call_action_flow_out = flow_out(&call_action);

        assert!(graph.connect_ports(1, trigger_flow_out, 2, set_state_flow_in));
        assert_eq!(graph.edges.len(), 1);
        assert!(graph.connect_ports(3, call_action_flow_out, 2, set_state_flow_in));
        assert_eq!(graph.edges.len(), 1);
        assert_eq!(graph.edges[0].from_node, 3);
        assert_eq!(graph.edges[0].from_port, call_action_flow_out);
    }

    #[test]
    fn flow_successors_with_input_slots_returns_fanout_in_edge_insertion_order() {
        let mut graph = ActionGraph::new_with_trigger("on_press");
        let a = set_state_node(2);
        let b = set_state_node(3);
        graph.nodes.push(a.clone());
        graph.nodes.push(b.clone());
        graph.z_order.extend([2, 3]);
        graph.next_id = 4;

        let trigger_flow_out = {
            let trigger = graph.nodes.iter().find(|n| n.id == 1).expect("trigger");
            flow_out(trigger)
        };
        let a_in = flow_in(&a);
        let b_in = flow_in(&b);

        assert!(graph.connect_ports(1, trigger_flow_out, 2, a_in));
        graph.edges.push(ActionEdge {
            from_node: 1,
            from_port: trigger_flow_out,
            to_node: 2,
            to_port: 999_999,
        });
        assert!(graph.connect_ports(1, trigger_flow_out, 3, b_in));

        let successors = graph.flow_successors_with_input_slots(1, 0);
        assert_eq!(successors, vec![(2, 0), (3, 0)]);
    }

    #[test]
    fn action_palette_hides_retired_value_operator_nodes() {
        let entries = action_palette_entries();
        let ids: Vec<u64> = entries.iter().map(|e| e.id).collect();
        let labels: HashMap<u64, &str> = entries.iter().map(|e| (e.id, e.label)).collect();

        assert!(ids.contains(&1));
        assert!(ids.contains(&3));
        assert!(ids.contains(&4));
        assert!(ids.contains(&8));
        assert!(ids.contains(&14));
        assert_eq!(labels.get(&1), Some(&"State Mutation"));
        assert_eq!(labels.get(&4), Some(&"If"));
        assert_eq!(labels.get(&8), Some(&"Match"));
        assert_eq!(labels.get(&14), Some(&"Call Flow"));

        for retired in [5, 6, 7, 9, 10, 11, 12, 13, 16] {
            assert!(
                !ids.contains(&retired),
                "retired palette id {} should not be exposed",
                retired
            );
            assert!(
                action_node_from_palette_id(retired, Point::new(0.0, 0.0), 99).is_none(),
                "retired palette id {} should not create a node",
                retired
            );
        }
    }

    #[test]
    fn action_palette_if_and_match_nodes_start_with_in_node_authoring_defaults() {
        let if_node = action_node_from_palette_id(4, Point::new(0.0, 0.0), 100).expect("if node");
        assert!(if_node.authored_condition.is_none());
        assert_eq!(if_node.authored_conditions.len(), 1);
        assert_eq!(if_node.authored_condition_join, ConditionJoinMode::All);
        assert!(if_node.authored_match_subject.is_none());

        let match_node =
            action_node_from_palette_id(8, Point::new(0.0, 0.0), 101).expect("match node");
        assert!(match_node.authored_condition.is_none());
        assert!(match_node.authored_conditions.is_empty());
        assert!(matches!(
            match_node.authored_match_subject,
            Some(AuthoredValueSource::Literal(ActionValue::String(_)))
        ));
    }
}

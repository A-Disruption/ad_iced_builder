use crate::action_system::state_ref::{ActionValueType, StateFieldRef};
use crate::data_structures::types::types::WidgetId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A scalar value used in action expressions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ActionValue {
    String(String),
    Number(f64),
    Bool(bool),
    /// A named enum variant, e.g. `NavigationBarSelection::SidePanel`.
    EnumVariant {
        type_name: String,
        variant: String,
    },
}

impl ActionValue {
    /// Returns the Rust literal expression for this value.
    pub fn rust_literal(&self) -> String {
        match self {
            Self::String(s) => format!("String::from({:?})", s),
            Self::Number(n) => {
                if n.fract() == 0.0 && n.abs() < 1e15 {
                    format!("{:.1}", n)
                } else {
                    format!("{}", n)
                }
            }
            Self::Bool(b) => b.to_string(),
            Self::EnumVariant { type_name, variant } => format!("{}::{}", type_name, variant),
        }
    }

    pub fn value_type(&self) -> ActionValueType {
        match self {
            Self::String(_) => ActionValueType::String,
            Self::Number(_) => ActionValueType::F64,
            Self::Bool(_) => ActionValueType::Bool,
            Self::EnumVariant { type_name, .. } => ActionValueType::Enum {
                type_name: type_name.clone(),
                variants: Vec::new(),
            },
        }
    }
}

/// One field assignment in a state mutation node.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StateAssignment {
    pub target: Option<StateFieldRef>,
    pub value_source: ValueSource,
}

/// How a state assignment gets its value — from a literal, a connected port, or a state field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ValueSource {
    Literal(ActionValue),
    /// Value comes from a data port connected from another node.
    FromPort,
    /// Value comes from reading a state field directly (replaces the old GetState node).
    StateField(StateFieldRef),
}

/// Canonical navigation targets for `NavigateToView`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NavigateTarget {
    /// Switch the app's top-level active view.
    AppView { view_id: Uuid },
    /// Switch a specific ViewReference widget to one of its configured views.
    ViewReference {
        owner_view_id: Uuid,
        widget_id: WidgetId,
        target_view_id: Uuid,
    },
}

/// In-node source selector for authored If/Match expressions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AuthoredValueSource {
    TriggerInput {
        name: String,
        value_type: ActionValueType,
    },
    StateField(StateFieldRef),
    Literal(ActionValue),
}

impl AuthoredValueSource {
    pub fn value_type(&self) -> ActionValueType {
        match self {
            Self::TriggerInput { value_type, .. } => value_type.clone(),
            Self::StateField(field) => field.field_type.clone(),
            Self::Literal(v) => v.value_type(),
        }
    }
}

/// Compact in-node condition authoring model used by If.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthoredCondition {
    pub lhs: AuthoredValueSource,
    pub operator: CompareOp,
    pub rhs_literal: ActionValue,
}

impl Default for AuthoredCondition {
    fn default() -> Self {
        Self {
            lhs: AuthoredValueSource::Literal(ActionValue::String(String::new())),
            operator: CompareOp::IsNotEmpty,
            rhs_literal: ActionValue::String(String::new()),
        }
    }
}

/// Join mode for multi-row If condition authoring.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ConditionJoinMode {
    #[default]
    All,
    Any,
}

impl std::fmt::Display for ConditionJoinMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::All => write!(f, "All"),
            Self::Any => write!(f, "Any"),
        }
    }
}

/// A single output port declaration on a Trigger node, describing one piece
/// of event data that flows out.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TriggerPort {
    pub name: String,
    pub value_type: ActionValueType,
}

/// The semantic type of an action node.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ActionNodeKind {
    /// Start node. Always present and locked (cannot be deleted).
    /// Carries event data as data outputs.
    Trigger {
        event_name: String,
        output_ports: Vec<TriggerPort>,
    },

    /// Unified state mutation node. Applies assignments in order.
    StateMutation {
        assignments: Vec<StateAssignment>,
    },

    /// Legacy single-assignment node kept for load-time compatibility.
    SetState {
        target: Option<StateFieldRef>,
        value_source: ValueSource,
    },

    /// Legacy multi-assignment node kept for load-time compatibility.
    UpdateState {
        assignments: Vec<StateAssignment>,
    },

    /// Changes the currently active view.
    /// Each entry in `targets` corresponds to one (flow_in, flow_out) port pair.
    /// Wiring a flow into slot i executes targets[i].
    NavigateToView {
        targets: Vec<Option<NavigateTarget>>,
    },

    /// Branches execution based on a boolean input.
    /// Has two flow outputs: slot 0 = true branch, slot 1 = false branch.
    Conditional,

    /// Branches on a string/number value (or enum variant).
    /// Flow outputs: one per arm (in order) + a final "default" output.
    Match {
        arms: Vec<String>,
        /// If set, arms are enum variant names and codegen emits `EnumType::Variant` patterns.
        enum_type: Option<String>,
    },

    // ── Literal value nodes ─────────────────────────────────────────────────
    StringLiteral {
        value: String,
    },
    NumberLiteral {
        value: f64,
    },
    BoolLiteral {
        value: bool,
    },

    /// Emits an enum variant as a String value. Data-only (no flow ports).
    EnumLiteral {
        enum_name: Option<String>,
        variant: Option<String>,
    },

    // ── Compare / Logic nodes ────────────────────────────────────────────────
    /// Compares two values. Data-only (no flow ports). Outputs Bool.
    Compare {
        operator: CompareOp,
        rhs: CompareRhs,
        rhs_literal: ActionValue,
    },
    /// Logical AND of two Bool inputs. Data-only.
    LogicAnd,
    /// Logical OR of two Bool inputs. Data-only.
    LogicOr,
    /// Logical NOT of one Bool input. Data-only.
    LogicNot,

    /// Calls a callable flow by stable flow identity. Has flow_in and flow_out.
    CallFlow {
        flow_id: Option<Uuid>,
    },

    /// Legacy name-based call node kept for load-time compatibility.
    CallAction {
        action_name: Option<String>,
    },

    /// Emits an expression formula verbatim as a data value (data-only, no flow ports).
    /// In code gen, the formula is emitted as-is. In the interpreter, it is evaluated.
    Expression {
        formula: String,
    },

    /// Legacy node removed from the UI. Loaded from old saves only; dropped on migration.
    #[serde(rename = "GetState")]
    LegacyGetState {
        #[serde(default)]
        source: Option<()>,
    },
}

impl ActionNodeKind {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Trigger { .. } => "Trigger",
            Self::StateMutation { .. } => "State Mutation",
            Self::SetState { .. } => "Set State (legacy)",
            Self::UpdateState { .. } => "Update State (legacy)",
            Self::NavigateToView { .. } => "Navigate to View",
            Self::Conditional => "If",
            Self::Match { .. } => "Match",
            Self::StringLiteral { .. } => "String",
            Self::NumberLiteral { .. } => "Number",
            Self::BoolLiteral { .. } => "Bool",
            Self::EnumLiteral { .. } => "Enum Value",
            Self::Compare { .. } => "Compare",
            Self::LogicAnd => "AND",
            Self::LogicOr => "OR",
            Self::LogicNot => "NOT",
            Self::CallFlow { .. } => "Call Flow",
            Self::CallAction { .. } => "Call Action (legacy)",
            Self::Expression { .. } => "Expression",
            Self::LegacyGetState { .. } => "Get State (legacy)",
        }
    }

    pub fn kind_label(&self) -> &'static str {
        match self {
            Self::Trigger { .. } => "", // overridden per-instance via kind_label_override
            Self::StateMutation { .. }
            | Self::SetState { .. }
            | Self::UpdateState { .. }
            | Self::NavigateToView { .. }
            | Self::CallFlow { .. }
            | Self::CallAction { .. }
            | Self::LegacyGetState { .. } => "Action",
            Self::Conditional | Self::Match { .. } => "Control",
            Self::Compare { .. } | Self::LogicAnd | Self::LogicOr | Self::LogicNot => "Logic",
            Self::StringLiteral { .. }
            | Self::NumberLiteral { .. }
            | Self::BoolLiteral { .. }
            | Self::EnumLiteral { .. }
            | Self::Expression { .. } => "Value",
        }
    }
}

// ── CompareOp / CompareRhs ────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompareOp {
    Eq,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    Contains,
    StartsWith,
    EndsWith,
    IsEmpty,
    IsNotEmpty,
    IsTrue,
    IsFalse,
    IsValidEmail,
}

impl CompareOp {
    pub fn all() -> Vec<CompareOp> {
        vec![
            Self::Eq,
            Self::NotEq,
            Self::Lt,
            Self::Gt,
            Self::LtEq,
            Self::GtEq,
            Self::Contains,
            Self::StartsWith,
            Self::EndsWith,
            Self::IsEmpty,
            Self::IsNotEmpty,
            Self::IsTrue,
            Self::IsFalse,
            Self::IsValidEmail,
        ]
    }

    /// Whether this operator needs a right-hand side value.
    pub fn needs_rhs(&self) -> bool {
        !matches!(
            self,
            Self::IsEmpty | Self::IsNotEmpty | Self::IsTrue | Self::IsFalse | Self::IsValidEmail
        )
    }

    /// Generates an inline Rust expression for this comparison.
    pub fn rust_expr(&self, lhs: &str, rhs: &str) -> String {
        match self {
            Self::Eq => format!("{lhs} == {rhs}"),
            Self::NotEq => format!("{lhs} != {rhs}"),
            Self::Lt => format!("{lhs} < {rhs}"),
            Self::Gt => format!("{lhs} > {rhs}"),
            Self::LtEq => format!("{lhs} <= {rhs}"),
            Self::GtEq => format!("{lhs} >= {rhs}"),
            Self::Contains => format!("{lhs}.contains({rhs})"),
            Self::StartsWith => format!("{lhs}.starts_with({rhs})"),
            Self::EndsWith => format!("{lhs}.ends_with({rhs})"),
            Self::IsEmpty => format!("{lhs}.is_empty()"),
            Self::IsNotEmpty => format!("!{lhs}.is_empty()"),
            Self::IsTrue => format!("matches!({lhs}, true)"),
            Self::IsFalse => format!("matches!({lhs}, false)"),
            Self::IsValidEmail => format!(
                "{{ let __email = format!(\"{{}}\", {lhs}); let __at = __email.find('@'); __at.map(|idx| idx > 0 && idx + 1 < __email.len() && __email[idx + 1..].contains('.')).unwrap_or(false) }}"
            ),
        }
    }
}

impl std::fmt::Display for CompareOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Eq => "==",
            Self::NotEq => "≠",
            Self::Lt => "<",
            Self::Gt => ">",
            Self::LtEq => "≤",
            Self::GtEq => "≥",
            Self::Contains => "contains",
            Self::StartsWith => "starts with",
            Self::EndsWith => "ends with",
            Self::IsEmpty => "is empty",
            Self::IsNotEmpty => "is not empty",
            Self::IsTrue => "is true",
            Self::IsFalse => "is false",
            Self::IsValidEmail => "is valid email",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompareRhs {
    Literal,
    FromPort,
}

impl std::fmt::Display for CompareRhs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal => write!(f, "Literal"),
            Self::FromPort => write!(f, "From port"),
        }
    }
}

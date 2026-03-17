use crate::action_system::state_ref::ActionValueType;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A user-defined state field that is not auto-generated from widget properties.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomStateField {
    pub id: Uuid,
    /// snake_case Rust field name, e.g. "user_list"
    pub name: String,
    /// UI label, e.g. "User List"
    pub display_name: String,
    pub field_type: CustomFieldType,
    /// Rust expression for the initial value, e.g. "Vec::new()" or "0"
    pub default_expr: String,
}

impl CustomStateField {
    pub fn new(name: String) -> Self {
        let display_name = name.replace('_', " ");
        Self {
            id: Uuid::new_v4(),
            name,
            display_name,
            field_type: CustomFieldType::String,
            default_expr: "String::new()".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CustomFieldType {
    Bool,
    String,
    I32,
    I64,
    F32,
    F64,
    Usize,
    /// A user-defined enum by name.
    Enum(String),
    /// A user-defined struct by name.
    Struct(String),
    Vec(Box<CustomFieldType>),
    Option(Box<CustomFieldType>),
}

impl CustomFieldType {
    /// All primitive options for the type picker.
    pub fn primitives() -> Vec<CustomFieldType> {
        vec![
            Self::Bool,
            Self::String,
            Self::I32,
            Self::I64,
            Self::F32,
            Self::F64,
            Self::Usize,
        ]
    }

    /// The Rust type string for code generation.
    pub fn rust_type(&self) -> String {
        match self {
            Self::Bool => "bool".to_string(),
            Self::String => "String".to_string(),
            Self::I32 => "i32".to_string(),
            Self::I64 => "i64".to_string(),
            Self::F32 => "f32".to_string(),
            Self::F64 => "f64".to_string(),
            Self::Usize => "usize".to_string(),
            Self::Enum(name) => name.clone(),
            Self::Struct(name) => name.clone(),
            Self::Vec(inner) => format!("Vec<{}>", inner.rust_type()),
            Self::Option(inner) => format!("Option<{}>", inner.rust_type()),
        }
    }

    /// The default Rust expression for code generation.
    pub fn default_expr(&self) -> String {
        match self {
            Self::Bool => "false".to_string(),
            Self::String => "String::new()".to_string(),
            Self::I32 | Self::I64 | Self::Usize => "0".to_string(),
            Self::F32 | Self::F64 => "0.0".to_string(),
            Self::Enum(name) => format!("{name}::new()"),
            Self::Struct(name) => format!("{name}::new()"),
            Self::Vec(_) => "Vec::new()".to_string(),
            Self::Option(_) => "None".to_string(),
        }
    }

    /// Maps to an `ActionValueType` for use in the interpreter (best-effort).
    pub fn to_action_value_type(&self) -> ActionValueType {
        match self {
            Self::Bool => ActionValueType::Bool,
            Self::String | Self::Enum(_) | Self::Struct(_) => ActionValueType::String,
            Self::I32 | Self::I64 | Self::Usize => ActionValueType::Usize,
            Self::F32 => ActionValueType::F32,
            Self::F64 => ActionValueType::F64,
            Self::Vec(_) | Self::Option(_) => ActionValueType::String,
        }
    }
}

impl std::fmt::Display for CustomFieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.rust_type())
    }
}

#[cfg(test)]
mod tests {
    use super::CustomFieldType;

    #[test]
    fn named_types_default_to_new_constructors() {
        assert_eq!(
            CustomFieldType::Enum("Status".to_string()).default_expr(),
            "Status::new()"
        );
        assert_eq!(
            CustomFieldType::Struct("Payload".to_string()).default_expr(),
            "Payload::new()"
        );
    }
}

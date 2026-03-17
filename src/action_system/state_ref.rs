use crate::data_structures::types::types::WidgetId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// The type of a state field value.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionValueType {
    String,
    Bool,
    F32,
    F64,
    Usize,
    /// A named enum type (e.g. a ViewSelection enum).
    /// `variants` lists the PascalCase variant names.
    Enum {
        type_name: String,
        variants: Vec<String>,
    },
}

impl ActionValueType {
    pub fn display_name(&self) -> String {
        match self {
            Self::String => "String".to_string(),
            Self::Bool => "Bool".to_string(),
            Self::F32 => "f32".to_string(),
            Self::F64 => "f64".to_string(),
            Self::Usize => "usize".to_string(),
            Self::Enum { type_name, .. } => type_name.clone(),
        }
    }

    /// Rust type as it appears in generated code.
    pub fn rust_type(&self) -> String {
        match self {
            Self::String => "String".to_string(),
            Self::Bool => "bool".to_string(),
            Self::F32 => "f32".to_string(),
            Self::F64 => "f64".to_string(),
            Self::Usize => "usize".to_string(),
            Self::Enum { type_name, .. } => type_name.clone(),
        }
    }
}

/// Discriminates between widget-derived and custom state fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StateRefSource {
    /// An auto-generated field derived from a widget property.
    Widget {
        widget_id: WidgetId,
        /// Suffix appended to the widget name, e.g. "_value", "_checked".
        field_suffix: String,
    },
    /// A user-defined custom state field on the view.
    Custom {
        field_id: Uuid,
        /// snake_case field name as it appears in the struct, e.g. "user_list".
        field_name: String,
    },
    /// The `{primary_field}_selection` field generated for a multi-view ViewReference.
    ViewSelection {
        widget_id: WidgetId,
        /// Full snake_case field name, e.g. "navigation_bar_selection".
        field_name: String,
    },
}

/// A reference to a specific state field (widget-derived or custom).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StateFieldRef {
    pub view_id: Uuid,
    pub source: StateRefSource,
    pub field_type: ActionValueType,
    /// Human-readable label shown in the UI picker.
    pub display_name: String,
}

impl std::fmt::Display for StateFieldRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name)
    }
}

impl StateFieldRef {
    /// Returns the Rust expression used in generated code to access this field.
    ///
    /// `current_view_id` is the view whose `update()` arm is being generated.
    /// `all_names` maps (view_id, widget_id) → widget snake_case name.
    /// `view_field_names` maps sub-view UUID → field name in main's struct.
    pub fn rust_path(
        &self,
        current_view_id: Uuid,
        all_names: &HashMap<(Uuid, WidgetId), String>,
        view_field_names: &HashMap<Uuid, String>,
    ) -> String {
        let field_name = match &self.source {
            StateRefSource::Widget {
                widget_id,
                field_suffix,
            } => {
                let widget_name = all_names
                    .get(&(self.view_id, *widget_id))
                    .cloned()
                    .unwrap_or_else(|| format!("widget_{}", widget_id.0));
                format!("{}{}", to_snake_case(&widget_name), field_suffix)
            }
            StateRefSource::Custom { field_name, .. } => field_name.clone(),
            StateRefSource::ViewSelection { field_name, .. } => field_name.clone(),
        };

        if self.view_id == current_view_id {
            format!("self.{}", field_name)
        } else {
            let sub_field = view_field_names
                .get(&self.view_id)
                .cloned()
                .unwrap_or_else(|| format!("view_{}", uuid_short(&self.view_id)));
            format!("self.{}.{}", sub_field, field_name)
        }
    }
}

/// Stable preview storage key for a ViewReference selection field.
pub fn view_selection_state_key(view_id: Uuid, field_name: &str) -> Uuid {
    fn fnv64(seed: u64, data: &[u8]) -> u64 {
        let mut hash = seed;
        for byte in data {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash
    }

    let key = format!("view-selection:{view_id}:{field_name}");
    let hi = fnv64(0xcbf29ce484222325, key.as_bytes());
    let lo = fnv64(0x84222325cbf29ce4, key.as_bytes());
    Uuid::from_u128((u128::from(hi) << 64) | u128::from(lo))
}

/// Stable preview storage key for canonical ViewReference selection state.
pub fn view_reference_selection_state_key(owner_view_id: Uuid, widget_id: WidgetId) -> Uuid {
    fn fnv64(seed: u64, data: &[u8]) -> u64 {
        let mut hash = seed;
        for byte in data {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash
    }

    let key = format!("view-reference-selection:{owner_view_id}:{}", widget_id.0);
    let hi = fnv64(0xcbf29ce484222325, key.as_bytes());
    let lo = fnv64(0x84222325cbf29ce4, key.as_bytes());
    Uuid::from_u128((u128::from(hi) << 64) | u128::from(lo))
}

/// Stable preview storage key for a Generic Overlay open/closed state.
pub fn generic_overlay_open_state_key(owner_view_id: Uuid, widget_id: WidgetId) -> Uuid {
    fn fnv64(seed: u64, data: &[u8]) -> u64 {
        let mut hash = seed;
        for byte in data {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash
    }

    let key = format!("generic-overlay-open:{owner_view_id}:{}", widget_id.0);
    let hi = fnv64(0xcbf29ce484222325, key.as_bytes());
    let lo = fnv64(0x84222325cbf29ce4, key.as_bytes());
    Uuid::from_u128((u128::from(hi) << 64) | u128::from(lo))
}

/// Stable preview storage key for a Date Picker open/closed state.
pub fn date_picker_open_state_key(owner_view_id: Uuid, widget_id: WidgetId) -> Uuid {
    fn fnv64(seed: u64, data: &[u8]) -> u64 {
        let mut hash = seed;
        for byte in data {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash
    }

    let key = format!("date-picker-open:{owner_view_id}:{}", widget_id.0);
    let hi = fnv64(0xcbf29ce484222325, key.as_bytes());
    let lo = fnv64(0x84222325cbf29ce4, key.as_bytes());
    Uuid::from_u128((u128::from(hi) << 64) | u128::from(lo))
}

fn to_snake_case(s: &str) -> String {
    s.chars()
        .enumerate()
        .flat_map(|(i, c)| {
            if c.is_uppercase() && i > 0 {
                vec!['_', c.to_lowercase().next().unwrap_or(c)]
            } else {
                vec![c.to_lowercase().next().unwrap_or(c)]
            }
        })
        .collect()
}

fn uuid_short(id: &Uuid) -> String {
    id.to_string().replace('-', "")[..8].to_string()
}

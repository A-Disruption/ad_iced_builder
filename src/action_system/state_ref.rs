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
pub fn overlay_open_state_key(owner_view_id: Uuid, widget_id: WidgetId) -> Uuid {
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

use std::collections::BTreeMap;
use uuid::Uuid as CollectUuid;

use crate::data_structures::types::types::{AppView, Widget, WidgetType};

// ─── Collecting the state fields a project exposes ───────────────────────────
// Moved out of `views::action_editor` when that module was parked: these walk
// the widget tree, not the editor, and the node bodies need them.

pub(crate) fn collect_state_fields(all_views: &BTreeMap<Uuid, AppView>) -> Vec<StateFieldRef> {
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
        WidgetType::TextEditor => {
            out.push(make_widget_ref("_text", ActionValueType::String, "text"));
        }
        WidgetType::TextInput => {
            out.push(make_widget_ref("_value", ActionValueType::String, "value"));
        }
        WidgetType::Slider | WidgetType::VerticalSlider => {
            out.push(make_widget_ref("_value", ActionValueType::F32, "value"));
        }
        WidgetType::Split => {
            out.push(make_widget_ref(
                "_split_at",
                ActionValueType::F32,
                "split position",
            ));
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
        WidgetType::Popover | WidgetType::Menu | WidgetType::Layer => {
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

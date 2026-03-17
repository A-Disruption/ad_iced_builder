pub mod serde_iced;

use std::collections::BTreeMap;
use std::path::Path;
use uuid::Uuid;

use crate::action_system::flow::AppFlow;
use crate::data_structures::types::types::AppView;
use crate::enum_builder::TypeSystem;
use crate::styles::style_enum::SavedStyleDefinition;
use crate::views::theme_and_stylefn_builder::ThemePaneEnum;

pub const FORMAT_VERSION: u32 = 1;

/// The serializable subset of CustomThemes — only the parts we persist.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct SerializableThemes {
    #[serde(default)]
    pub theme_name: String,
    #[serde(default)]
    pub styles: BTreeMap<ThemePaneEnum, BTreeMap<String, SavedStyleDefinition>>,
}

/// The root of the save file. All fields use `#[serde(default)]` for forward compatibility.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ProjectData {
    #[serde(default = "default_format_version")]
    pub format_version: u32,

    #[serde(default)]
    pub app_name: String,

    /// App-level flows (not per-view).
    #[serde(default)]
    pub flows: Vec<AppFlow>,

    /// All views, keyed by their UUID.
    #[serde(default)]
    pub views: BTreeMap<Uuid, AppView>,

    /// The view that was selected when saved.
    #[serde(default)]
    pub selected_view_id: Uuid,

    /// Custom style definitions (CustomThemes minus transient UI state).
    #[serde(default)]
    pub custom_styles: SerializableThemes,

    /// Enum and struct type definitions.
    #[serde(default)]
    pub type_system: TypeSystem,

    /// The active app theme name, e.g. "Dark" or "Light".
    #[serde(default = "default_theme_name")]
    pub theme_name: String,
}

fn default_format_version() -> u32 {
    FORMAT_VERSION
}

fn default_theme_name() -> String {
    "Dark".to_string()
}

impl ProjectData {
    pub fn new(app_name: impl Into<String>) -> Self {
        Self {
            format_version: FORMAT_VERSION,
            app_name: app_name.into(),
            flows: Vec::new(),
            views: BTreeMap::new(),
            selected_view_id: Uuid::nil(),
            custom_styles: SerializableThemes::default(),
            type_system: TypeSystem::default(),
            theme_name: "Dark".to_string(),
        }
    }
}

// ─── Save (reference-based, no Clone required on AppView) ────────────────────

/// Borrow-only view of SerializableThemes for zero-copy serialization.
#[derive(serde::Serialize)]
pub struct SerializableThemesRef<'a> {
    pub theme_name: &'a str,
    pub styles: &'a BTreeMap<ThemePaneEnum, BTreeMap<String, SavedStyleDefinition>>,
}

/// Borrow-only view of ProjectData for zero-copy serialization.
#[derive(serde::Serialize)]
pub struct ProjectDataRef<'a> {
    pub format_version: u32,
    pub app_name: &'a str,
    pub flows: &'a [AppFlow],
    pub views: &'a BTreeMap<Uuid, AppView>,
    pub selected_view_id: Uuid,
    pub custom_styles: SerializableThemesRef<'a>,
    pub type_system: &'a TypeSystem,
    pub theme_name: &'a str,
}

pub fn save_project(data: &ProjectData, path: &Path) -> Result<(), String> {
    let json = serde_json::to_string_pretty(data).map_err(|e| e.to_string())?;
    std::fs::write(path, json).map_err(|e| e.to_string())
}

/// Serialize and write without requiring Clone on any field.
pub fn save_project_ref(data: &ProjectDataRef<'_>, path: &Path) -> Result<(), String> {
    let json = serde_json::to_string_pretty(data).map_err(|e| e.to_string())?;
    std::fs::write(path, json).map_err(|e| e.to_string())
}

// ─── Load ────────────────────────────────────────────────────────────────────

pub fn load_project(path: &Path) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|e| e.to_string())
}

pub fn deserialize_project(json: &str) -> Result<ProjectData, String> {
    let raw: serde_json::Value = serde_json::from_str(json).map_err(|e| e.to_string())?;
    let version = raw
        .get("format_version")
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as u32;
    migrate_and_deserialize(raw, version)
}

fn migrate_and_deserialize(
    val: serde_json::Value,
    from_version: u32,
) -> Result<ProjectData, String> {
    // Future migrations: if from_version < 2 { ... }
    let _ = from_version; // no migrations yet beyond serde(default)
    serde_json::from_value(val).map_err(|e| e.to_string())
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_roundtrip_empty_project() {
        let original = ProjectData::new("Test App");
        let json = serde_json::to_string(&original).unwrap();
        let loaded: ProjectData = serde_json::from_str(&json).unwrap();
        assert_eq!(original.app_name, loaded.app_name);
        assert_eq!(original.format_version, loaded.format_version);
        assert!(loaded.views.is_empty());
    }

    #[test]
    fn test_roundtrip_with_view() {
        use crate::data_structures::types::types::AppView;
        let mut data = ProjectData::new("View Test");
        let view = AppView::new("Main".to_string(), 0);
        let vid = view.id;
        data.views.insert(vid, view);
        data.selected_view_id = vid;

        let json = serde_json::to_string(&data).unwrap();
        let loaded: ProjectData = serde_json::from_str(&json).unwrap();
        assert_eq!(loaded.views.len(), 1);
        assert_eq!(loaded.selected_view_id, vid);
        assert!(loaded.views.contains_key(&vid));
    }

    #[test]
    fn test_missing_fields_use_defaults() {
        let json = r#"{"format_version": 1, "app_name": "OldApp"}"#;
        let loaded: ProjectData = serde_json::from_str(json).unwrap();
        assert_eq!(loaded.app_name, "OldApp");
        assert_eq!(loaded.format_version, 1);
        assert!(loaded.views.is_empty());
    }

    #[test]
    fn test_version_field_present_in_output() {
        let data = ProjectData::new("Test");
        let json = serde_json::to_string(&data).unwrap();
        let raw: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(raw["format_version"], FORMAT_VERSION);
    }

    #[test]
    fn test_roundtrip_action_graph() {
        use crate::action_system::graph::{ActionEdge, ActionGraph, ActionNodeData};
        use crate::action_system::node_kinds::{ActionNodeKind, ActionValue, ValueSource};
        use iced::Point;

        let mut graph = ActionGraph::new_with_trigger("on_press");
        let node = ActionNodeData::new(
            2,
            ActionNodeKind::SetState {
                target: None,
                value_source: ValueSource::Literal(ActionValue::Bool(true)),
            },
            Point::new(300.0, 100.0),
        );
        graph.nodes.push(node);
        graph.edges.push(ActionEdge {
            from_node: 1,
            from_port: 1000 + 1,
            to_node: 2,
            to_port: 2000,
        });
        graph.next_id = 3;

        let json = serde_json::to_string(&graph).unwrap();
        let mut loaded: ActionGraph = serde_json::from_str(&json).unwrap();
        for n in &mut loaded.nodes {
            n.rebuild_ports();
        }
        assert_eq!(graph.nodes.len(), loaded.nodes.len());
        assert_eq!(graph.edges.len(), loaded.edges.len());
        assert_eq!(loaded.nodes[0].kind, graph.nodes[0].kind);
    }

    #[test]
    fn test_roundtrip_properties_iced_types() {
        use crate::data_structures::properties::properties::Properties;
        use iced::{Color, Length, Padding, Vector};

        let mut props = Properties::default();
        props.width = Length::Fixed(200.0);
        props.height = Length::Shrink;
        props.padding = Padding {
            top: 5.0,
            bottom: 10.0,
            left: 8.0,
            right: 8.0,
        };
        props.background_color = Color {
            r: 0.2,
            g: 0.3,
            b: 0.8,
            a: 1.0,
        };
        props.shadow_offset = Vector { x: 2.0, y: 4.0 };

        let json = serde_json::to_string(&props).unwrap();
        let loaded: Properties = serde_json::from_str(&json).unwrap();

        assert_eq!(props.width, loaded.width);
        assert_eq!(props.height, loaded.height);
        // Compare padding fields individually (Padding doesn't impl PartialEq in older iced)
        assert_eq!(loaded.padding.top, 5.0);
        assert_eq!(loaded.padding.bottom, 10.0);
        assert_eq!(props.background_color.r, loaded.background_color.r);
        assert_eq!(props.shadow_offset.x, loaded.shadow_offset.x);
    }

    #[test]
    fn test_roundtrip_custom_state() {
        use crate::action_system::custom_state::{CustomFieldType, CustomStateField};

        let mut field = CustomStateField::new("user_count".to_string());
        field.field_type = CustomFieldType::I32;
        field.default_expr = "0".to_string();

        let json = serde_json::to_string(&field).unwrap();
        let loaded: CustomStateField = serde_json::from_str(&json).unwrap();
        assert_eq!(field.name, loaded.name);
        assert_eq!(field.field_type, loaded.field_type);
        assert_eq!(field.default_expr, loaded.default_expr);
    }
}

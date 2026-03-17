use crate::action_system::custom_state::CustomStateField;
use crate::action_system::flow::AppFlow;
use crate::action_system::node_kinds::ActionValue;
use crate::data_structures::{
    properties::properties::Properties, widget_hierarchy::WidgetHierarchy,
};
use iced::window::Settings;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Widget {
    pub id: WidgetId,
    pub widget_type: WidgetType,
    pub name: String,
    pub properties: Box<Properties>,
    pub children: Vec<Widget>,
}

impl Widget {
    pub fn new(widget_type: WidgetType, id: WidgetId) -> Self {
        Self {
            id,
            widget_type,
            name: format!("{:?}", widget_type),
            properties: Box::new(Properties::for_widget_type(widget_type)),
            children: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WidgetId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WidgetType {
    Container,
    Scrollable,
    Row,
    Column,
    Button,
    Text,
    TextInput,
    Checkbox,
    Radio,
    Slider,
    VerticalSlider,
    ProgressBar,
    Toggler,
    PickList,
    Space,
    Rule,
    Image,
    Svg,
    Tooltip,
    ComboBox,
    Markdown,
    MouseArea,
    QRCode,
    Stack,
    Collapsible,
    CollapsibleGroup,
    GenericOverlay,
    DatePicker,
    Themer,
    Grid,
    Pin,
    Table,
    ViewReference,
    Icon,
}

#[derive(Serialize, Deserialize)]
pub struct AppView {
    pub id: Uuid,
    pub name: String,
    pub show_widget_bounds: bool,
    pub hierarchy: WidgetHierarchy,
    pub order: usize,
    pub is_main: bool,
    pub _is_component: bool,
    /// User-defined state fields for this view (not auto-generated from widgets).
    pub custom_state: Vec<CustomStateField>,
    /// Live-preview runtime values keyed by stable field key
    /// (custom field IDs plus generated view-selection field keys).
    pub custom_state_values: HashMap<Uuid, ActionValue>,
    /// Flows stored on the view for legacy migration only (loaded from old saves, then moved to app level).
    #[serde(default, skip_serializing)]
    pub flows: Vec<AppFlow>,
    /// Override the generated state field name prefix for auto-state widgets.
    /// Maps widget_id.0 (usize cast to u64) → custom snake_case name.
    pub widget_state_names: HashMap<u64, String>,
}

impl AppView {
    pub fn new(name: String, order: usize) -> AppView {
        AppView {
            id: Uuid::new_v4(),
            name: name,
            show_widget_bounds: false,
            hierarchy: WidgetHierarchy::new(WidgetType::Container),
            order: order,
            is_main: order == 0,
            _is_component: false,
            custom_state: Vec::new(),
            custom_state_values: HashMap::new(),
            flows: Vec::new(),
            widget_state_names: HashMap::new(),
        }
    }

    pub fn with_id(id: Uuid, name: String, order: usize) -> Self {
        Self {
            id,
            name,
            show_widget_bounds: false,
            hierarchy: WidgetHierarchy::new(WidgetType::Container),
            order,
            is_main: order == 0,
            _is_component: false,
            custom_state: Vec::new(),
            custom_state_values: HashMap::new(),
            flows: Vec::new(),
            widget_state_names: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfig {
    pub title: String,
    #[serde(default)]
    pub assigned_view_id: Option<Uuid>,
    // iced::window::Settings is not serializable — reconstructed from draft strings on load.
    #[serde(skip, default = "Settings::default")]
    pub settings: Settings,

    #[serde(default)]
    pub draft_width: String,
    #[serde(default)]
    pub draft_height: String,
    #[serde(default)]
    pub draft_min_width: String,
    #[serde(default)]
    pub draft_min_height: String,
    #[serde(default)]
    pub draft_max_width: String,
    #[serde(default)]
    pub draft_max_height: String,
    #[serde(default)]
    pub draft_pos_x: String,
    #[serde(default)]
    pub draft_pos_y: String,
}

impl WindowConfig {
    pub fn new(title: String, settings: Settings) -> Self {
        Self {
            title,
            assigned_view_id: None,
            // Initialize drafts from actual values
            draft_width: settings.size.width.to_string(),
            draft_height: settings.size.height.to_string(),
            draft_min_width: settings
                .min_size
                .map(|s| s.width.to_string())
                .unwrap_or_default(),
            draft_min_height: settings
                .min_size
                .map(|s| s.height.to_string())
                .unwrap_or_default(),
            draft_max_width: settings
                .max_size
                .map(|s| s.width.to_string())
                .unwrap_or_default(),
            draft_max_height: settings
                .max_size
                .map(|s| s.height.to_string())
                .unwrap_or_default(),
            draft_pos_x: String::new(),
            draft_pos_y: String::new(),
            settings,
        }
    }
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self::new(String::from("New Window"), Settings::default())
    }
}

impl WindowConfig {
    /// Create config for main Editor window
    pub fn editor() -> Self {
        Self::new(
            "UI Builder".to_string(),
            Settings {
                size: iced::Size::new(1920.0, 1080.0),
                min_size: Some(iced::Size::new(700.0, 975.0)),
                position: iced::window::Position::Centered,
                exit_on_close_request: true,
                ..Settings::default()
            },
        )
    }

    /// Create config for a Visualizer window
    pub fn visualizer() -> Self {
        Self::new("Visualizer".to_string(), Settings::default())
    }
}

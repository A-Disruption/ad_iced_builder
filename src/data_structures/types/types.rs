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
    TextEditor,
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
    // The widgets crate replaced its one `generic_overlay` with three separate
    // widgets, so the builder does too. A project saved before that split
    // loads as a `Popover`: it is the one of the three that keeps both of the
    // old widget's children (trigger and content) and its default anchored,
    // trigger-driven behaviour.
    #[serde(alias = "Overlay")]
    Popover,
    Menu,
    Layer,
    Toast,
    DatePicker,
    Split,
    Float,
    Responsive,
    Sensor,
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

    /// Whether the generated application draws its own title bar with
    /// `iced-native-frame` rather than asking the platform for one.
    ///
    /// Off by default: it adds a dependency, and a plain window is what most
    /// projects want.
    #[serde(default)]
    pub use_native_frame: bool,

    /// How that title bar looks, when it is drawn.
    #[serde(default)]
    pub native_frame: NativeFrameSettings,

    /// A view rendered into the title bar's application-owned row.
    ///
    /// The frame lays out `[icon][title][this][caption controls]`, with this
    /// row taking all the width between, so a menu bar or a row of tool
    /// buttons goes here. `None` leaves the space empty.
    #[serde(default)]
    pub title_bar_view_id: Option<Uuid>,
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

/// A serializable mirror of `iced_native_frame::NativeFrameConfig`.
///
/// The real config is `Copy` but not `Serialize`, and carries a
/// `DecorationMode` that duplicates [`WindowConfig::use_native_frame`]. This
/// holds the rest of it, so a project's title bar survives a save and the
/// builder can preview it.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct NativeFrameSettings {
    pub title_bar_height: f32,
    pub caption_button_width: f32,
    /// Whether the frame draws its own minimize / maximize / close controls.
    pub caption_buttons: bool,
    pub resize_border: f32,
    pub resizable: bool,
    pub corner_radius: f32,
    pub frame_border: bool,
    /// Needs `outer_padding` above zero and a transparent window.
    pub client_shadow: bool,
    pub outer_padding: f32,
    /// Windows 11 only, through the window's corner preference.
    pub native_rounding: bool,
    /// Windows only, through winit's undecorated shadow.
    pub native_shadow: bool,
    pub window_icon_size: f32,
    pub title_spacing: f32,
    pub title_padding: f32,
    /// Room before the leading region — on macOS, for the traffic lights.
    pub leading_inset: f32,
    pub show_title: bool,
}

impl Default for NativeFrameSettings {
    /// The crate's own platform defaults, read off a real config so the two
    /// cannot drift apart.
    fn default() -> Self {
        Self::from_config(iced_native_frame::NativeFrameConfig::platform_default())
    }
}

impl NativeFrameSettings {
    pub fn from_config(config: iced_native_frame::NativeFrameConfig) -> Self {
        Self {
            title_bar_height: config.title_bar_height,
            caption_button_width: config.caption_button_width,
            caption_buttons: config.caption_buttons,
            resize_border: config.resize_border,
            resizable: config.resizable,
            corner_radius: config.corner_radius,
            frame_border: config.frame_border,
            client_shadow: config.client_shadow,
            outer_padding: config.outer_padding,
            native_rounding: config.native_rounding,
            native_shadow: config.native_shadow,
            window_icon_size: config.window_icon_size,
            title_spacing: config.title_spacing,
            title_padding: config.title_padding,
            leading_inset: config.leading_inset,
            show_title: config.show_title,
        }
    }

    /// The real config, with `decoration_mode` set from whether the custom
    /// frame is wanted at all.
    pub fn to_config(self, custom: bool) -> iced_native_frame::NativeFrameConfig {
        iced_native_frame::NativeFrameConfig {
            decoration_mode: if custom {
                iced_native_frame::DecorationMode::Custom
            } else {
                iced_native_frame::DecorationMode::System
            },
            title_bar_height: self.title_bar_height,
            caption_button_width: self.caption_button_width,
            caption_buttons: self.caption_buttons,
            resize_border: self.resize_border,
            resizable: self.resizable,
            corner_radius: self.corner_radius,
            frame_border: self.frame_border,
            client_shadow: self.client_shadow,
            outer_padding: self.outer_padding,
            native_rounding: self.native_rounding,
            native_shadow: self.native_shadow,
            window_icon_size: self.window_icon_size,
            title_spacing: self.title_spacing,
            title_padding: self.title_padding,
            leading_inset: self.leading_inset,
            show_title: self.show_title,
        }
    }
}

impl WindowConfig {
    pub fn new(title: String, settings: Settings) -> Self {
        Self {
            title,
            assigned_view_id: None,
            use_native_frame: false,
            native_frame: NativeFrameSettings::default(),
            title_bar_view_id: None,
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

use iced::widget::{
    combo_box, markdown, progress_bar, qr_code, radio, slider, text, text_editor, toggler,
    vertical_slider,
};
use iced::{Alignment, Color, Length, Padding, Point, Theme, Vector};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::action_system::StateFieldRef;
use crate::data_structures::types::type_implementations::*;
use crate::data_structures::types::types::*;
use crate::persistence::serde_iced;

#[derive(Debug, Serialize, Deserialize)]
pub struct Properties {
    #[serde(with = "serde_iced::length")]
    pub width: Length,
    #[serde(with = "serde_iced::length")]
    pub height: Length,
    pub max_width: Option<f32>,
    pub max_height: Option<f32>,
    pub clip: bool,
    #[serde(with = "serde_iced::padding")]
    pub padding: Padding,
    pub widget_id: Option<String>,
    pub custom_style_name: Option<String>,
    pub menu_style_name: Option<String>,
    /// An alternate style applied when the condition matches (or always if no condition set).
    pub active_style_name: Option<String>,
    /// State field path to check for conditional style, e.g. `nav_selection`.
    pub style_condition_field: Option<String>,
    /// Expected value for the condition, e.g. `NavSelection::Home`.
    pub style_condition_value: Option<String>,

    //draft state for text_inputs
    pub draft_fixed_width: String,
    pub draft_fixed_height: String,
    pub draft_fill_portion_width: String,
    pub draft_fill_portion_height: String,
    pub draft_text_color: String,
    pub padding_mode: PaddingMode,

    // Container properties
    pub align_x: ContainerAlignX,
    pub align_y: ContainerAlignY,
    pub border_width: f32,
    pub border_radius: f32,
    #[serde(with = "serde_iced::color")]
    pub border_color: Color,
    #[serde(with = "serde_iced::color")]
    pub background_color: Color,
    pub has_shadow: bool,
    #[serde(with = "serde_iced::vector")]
    pub shadow_offset: Vector,
    pub shadow_blur: f32,
    #[serde(with = "serde_iced::color")]
    pub shadow_color: Color,
    pub container_sizing_mode: ContainerSizingMode,
    #[serde(with = "serde_iced::length")]
    pub container_center_length: Length, // Used when center_x/y/both is active

    // Row wrapping
    pub is_wrapping_row: bool,
    pub wrapping_vertical_spacing: f32,
    pub match_horizontal_spacing: bool,
    pub wrapping_align_x: ContainerAlignX,

    // Layout properties (Row/Column)
    pub spacing: f32,
    #[serde(with = "serde_iced::alignment")]
    pub align_items: Alignment,

    // Text properties
    pub text_content: String,
    pub text_size: f32,
    #[serde(with = "serde_iced::color")]
    pub text_color: Color,
    pub font: FontType,
    #[serde(with = "serde_iced::text_line_height")]
    pub line_height: text::LineHeight,
    #[serde(with = "serde_iced::text_wrapping")]
    pub wrap: text::Wrapping,
    #[serde(with = "serde_iced::text_shaping")]
    pub shaping: text::Shaping,
    #[serde(with = "serde_iced::text_alignment")]
    pub text_align_x: text::Alignment,
    #[serde(with = "serde_iced::vertical_alignment")]
    pub text_align_y: iced::alignment::Vertical,

    // Button properties
    pub button_on_press_maybe_enabled: bool,
    pub button_on_press_with_enabled: bool,
    pub button_on_press_enabled: bool,

    // TextEditor properties
    #[serde(
        default = "default_text_editor_content",
        with = "serde_iced::text_editor_content"
    )]
    pub text_editor_content: text_editor::Content,
    #[serde(default = "default_text_editor_placeholder")]
    pub text_editor_placeholder: String,
    #[serde(default = "default_text_editor_size")]
    pub text_editor_size: f32,
    #[serde(default = "default_text_editor_padding")]
    pub text_editor_padding: f32,
    #[serde(default = "default_text_editor_font")]
    pub text_editor_font: FontType,
    #[serde(
        default = "default_text_editor_line_height",
        with = "serde_iced::text_line_height"
    )]
    pub text_editor_line_height: text::LineHeight,
    #[serde(
        default = "default_text_editor_wrapping",
        with = "serde_iced::text_wrapping"
    )]
    pub text_editor_wrapping: text::Wrapping,

    // TextInput properties
    pub text_input_value: String,
    pub text_input_placeholder: String,
    pub text_input_size: f32,
    pub text_input_padding: f32,
    pub is_secure: bool,
    pub text_input_on_submit: bool,
    pub text_input_on_paste: bool,
    pub text_input_font: FontType,
    #[serde(with = "serde_iced::text_line_height")]
    pub text_input_line_height: text::LineHeight,
    pub text_input_alignment: ContainerAlignX,
    // TextInput icon
    pub text_input_icon_enabled: bool,
    #[serde(default = "default_icon_family")]
    pub text_input_icon_family: String,
    pub text_input_icon_name: String,
    pub text_input_icon_codepoint: u32,
    pub text_input_icon_size: f32, // 0.0 = use font default (None in iced)
    pub text_input_icon_spacing: f32,
    pub text_input_icon_side: TextInputIconSide,
    pub text_input_icon_picker_filter: String,

    // Checkbox properties
    pub checkbox_checked: bool,
    pub checkbox_label: String,
    pub checkbox_size: f32,
    pub checkbox_spacing: f32,

    // Radio properties
    pub radio_selected_index: usize,
    pub radio_options: Vec<String>,
    pub radio_label: String,
    pub radio_size: f32,
    pub radio_spacing: f32,

    // Slider properties
    pub slider_value: f32,
    pub slider_min: f32,
    pub slider_max: f32,
    pub slider_step: f32,
    pub slider_width: f32,
    pub slider_height: f32,

    // Progress properties
    pub progress_value: f32,
    pub progress_min: f32,
    pub progress_max: f32,
    #[serde(with = "serde_iced::length")]
    pub progress_length: Length,
    pub progress_girth: f32,
    pub progress_vertical: bool,

    // Toggler properties
    pub toggler_active: bool,
    pub toggler_label: String,
    pub toggler_size: f32,
    pub toggler_spacing: f32,

    // Collapsible properties
    #[serde(default = "default_collapsible_title")]
    pub collapsible_title: String,
    #[serde(default = "default_collapsible_header_height")]
    pub collapsible_header_height: f32,
    #[serde(default = "default_collapsible_header_clickable")]
    pub collapsible_header_clickable: bool,
    #[serde(default)]
    pub collapsible_expanded: bool,

    // Generic Overlay properties
    #[serde(default = "default_overlay_title")]
    #[serde(alias = "generic_overlay_title")]
    pub overlay_title: String,
    #[serde(
        default = "default_overlay_width",
        with = "serde_iced::length"
    )]
    #[serde(alias = "generic_overlay_overlay_width")]
    pub overlay_width: Length,
    #[serde(
        default = "default_overlay_height",
        with = "serde_iced::length"
    )]
    #[serde(alias = "generic_overlay_overlay_height")]
    pub overlay_height: Length,
    #[serde(default)]
    #[serde(alias = "generic_overlay_overlay_width_dynamic")]
    pub overlay_width_dynamic: bool,
    #[serde(default)]
    #[serde(alias = "generic_overlay_overlay_height_dynamic")]
    pub overlay_height_dynamic: bool,
    #[serde(default = "default_overlay_dynamic_factor")]
    #[serde(alias = "generic_overlay_overlay_width_dynamic_factor")]
    pub overlay_width_dynamic_factor: f32,
    #[serde(default = "default_overlay_dynamic_factor")]
    #[serde(alias = "generic_overlay_overlay_height_dynamic_factor")]
    pub overlay_height_dynamic_factor: f32,
    #[serde(default = "default_overlay_padding")]
    #[serde(alias = "generic_overlay_overlay_padding")]
    pub overlay_padding: f32,
    #[serde(default = "default_overlay_radius")]
    #[serde(alias = "generic_overlay_overlay_radius")]
    pub overlay_radius: f32,
    #[serde(default)]
    #[serde(alias = "generic_overlay_overlay_style_name")]
    pub overlay_style_name: Option<String>,
    #[serde(default)]
    #[serde(alias = "generic_overlay_on_hover")]
    pub overlay_on_hover: bool,
    #[serde(default)]
    #[serde(alias = "generic_overlay_hover_positions_on_click")]
    pub overlay_hover_positions_on_click: bool,
    #[serde(default)]
    #[serde(alias = "generic_overlay_initially_open")]
    pub overlay_initially_open: bool,
    #[serde(default)]
    #[serde(alias = "generic_overlay_hover_position")]
    pub overlay_hover_position: OverlayPosition,
    #[serde(default = "default_overlay_hover_gap")]
    #[serde(alias = "generic_overlay_hover_gap")]
    pub overlay_hover_gap: f32,
    #[serde(default = "default_overlay_hover_alignment")]
    #[serde(alias = "generic_overlay_hover_alignment")]
    pub overlay_hover_alignment: ContainerAlignX,
    #[serde(default)]
    #[serde(alias = "generic_overlay_hover_mode")]
    pub overlay_hover_mode: OverlayPositionMode,
    #[serde(default = "default_overlay_hover_snap")]
    #[serde(alias = "generic_overlay_hover_snap")]
    pub overlay_hover_snap: bool,
    #[serde(default)]
    #[serde(alias = "generic_overlay_close_on_click_outside")]
    pub overlay_close_on_click_outside: bool,
    #[serde(default)]
    #[serde(alias = "generic_overlay_opaque")]
    pub overlay_opaque: bool,
    #[serde(default = "default_overlay_opaque_alpha")]
    #[serde(alias = "generic_overlay_opaque_alpha")]
    pub overlay_opaque_alpha: f32,
    #[serde(default)]
    #[serde(alias = "generic_overlay_hide_header")]
    pub overlay_hide_header: bool,
    #[serde(default)]
    #[serde(alias = "generic_overlay_hide_close_button")]
    pub overlay_hide_close_button: bool,
    #[serde(default)]
    #[serde(alias = "generic_overlay_block_dragging")]
    pub overlay_block_dragging: bool,
    #[serde(default)]
    #[serde(alias = "generic_overlay_resizable")]
    pub overlay_resizable: OverlayResizeMode,
    #[serde(default)]
    #[serde(alias = "generic_overlay_reset_on_close")]
    pub overlay_reset_on_close: bool,
    #[serde(default)]
    #[serde(alias = "generic_overlay_interactive_base")]
    pub overlay_interactive_base: bool,
    #[serde(default)]
    #[serde(alias = "generic_overlay_animate")]
    pub overlay_animate: bool,
    #[serde(default)]
    #[serde(alias = "generic_overlay_animation_preset")]
    pub overlay_animation_preset: OverlayAnimationPreset,
    #[serde(default = "default_overlay_safe_triangle")]
    #[serde(alias = "generic_overlay_safe_triangle")]
    pub overlay_safe_triangle: bool,

    // ── The rest of the overlay widgets' builder surface ──────────────────────
    //
    // Everything above is shared by Popover, Menu, Layer and Toast, with each
    // widget's own defaults applied in `apply_defaults_for_type`. What follows
    // is per-widget: the methods only one of them has.

    /// Popover, Menu, Layer, Toast: lower bound on the surface width.
    #[serde(default)]
    pub overlay_min_width: f32,
    /// Popover, Menu, Layer, Toast: upper bound on the surface width.
    ///
    /// `None` leaves the surface to its contents, which is the widget default.
    #[serde(default)]
    pub overlay_max_width: Option<f32>,

    // ── Menu ─────────────────────────────────────────────────────────────────
    /// Gap between a submenu and the panel that opened it.
    ///
    /// Negative by default, which is what sits a submenu flush against its
    /// parent's border.
    #[serde(default = "default_menu_submenu_gap")]
    pub menu_submenu_gap: f32,
    #[serde(default = "default_menu_item_padding", with = "serde_iced::padding")]
    pub menu_item_padding: Padding,
    #[serde(default = "default_menu_trigger_padding", with = "serde_iced::padding")]
    pub menu_trigger_padding: Padding,
    /// Draws the trigger as bare content, with no surface of its own.
    #[serde(default)]
    pub menu_plain_trigger: bool,
    /// Which side of a row its checkmark gutter sits on.
    #[serde(default)]
    pub menu_check_side: MenuCheckSide,
    /// Space between the gutter and a row's content.
    #[serde(default = "default_menu_gutter_spacing")]
    pub menu_gutter_spacing: f32,

    // ── Menu rows ────────────────────────────────────────────────────────────
    // Set on the *children* of a Menu, one per row.
    /// What kind of row this child is, when its parent is a Menu.
    #[serde(default)]
    pub menu_item_kind: MenuItemKind,
    /// Whether a checkable row draws its checkmark.
    #[serde(default)]
    pub menu_item_checked: bool,
    /// A row that is drawn but does not respond.
    #[serde(default = "default_true")]
    pub menu_item_enabled: bool,

    // ── Layer ────────────────────────────────────────────────────────────────
    /// Lower bound on the surface height.
    #[serde(default)]
    pub layer_min_height: f32,
    /// Stretches the surface along the edge it is anchored to.
    #[serde(default)]
    pub layer_stretch: bool,
    /// Which part of the viewport the surface is anchored to.
    ///
    /// `Layer::new` starts at `Anchor::Center`, which the four-sided
    /// `OverlayPosition` cannot express.
    #[serde(default)]
    pub layer_anchor: OverlayAnchor,
    /// Places the surface at an explicit point instead of by anchor.
    #[serde(default)]
    pub layer_position_enabled: bool,
    #[serde(default)]
    pub layer_position_x: f32,
    #[serde(default)]
    pub layer_position_y: f32,
    /// Closes the surface when Escape is pressed.
    #[serde(default = "default_true")]
    pub layer_dismiss_on_escape: bool,

    // ── Toast ────────────────────────────────────────────────────────────────
    /// Which corner of the viewport the stack grows from.
    #[serde(default = "default_toast_anchor")]
    pub toast_anchor: OverlayAnchor,
    /// Distance from the viewport edges.
    #[serde(default = "default_toast_margin")]
    pub toast_margin: f32,
    /// Space between stacked toasts.
    #[serde(default = "default_toast_spacing")]
    pub toast_spacing: f32,
    /// Fixed width of every toast in the stack.
    #[serde(default = "default_toast_width")]
    pub toast_width: f32,

    // ── Toast items ──────────────────────────────────────────────────────────
    // Set on the *children* of a Toast, one per notification.
    /// The accent a single toast is drawn with.
    #[serde(default)]
    pub toast_variant: ToastVariant,
    /// Drops the leading status icon from a single toast.
    #[serde(default)]
    pub toast_without_icon: bool,
    /// Seconds before a toast dismisses itself. Zero means it stays.
    #[serde(default)]
    pub toast_timeout_secs: f32,

    // ── Date picker ──────────────────────────────────────────────────────────
    /// Places the picker at an explicit point instead of over its trigger.
    #[serde(default)]
    pub date_picker_position_enabled: bool,
    #[serde(default)]
    pub date_picker_position_x: f32,
    #[serde(default)]
    pub date_picker_position_y: f32,
    #[serde(default)]
    #[serde(alias = "draft_generic_overlay_overlay_width_fixed")]
    pub draft_overlay_width_fixed: String,
    #[serde(default)]
    #[serde(alias = "draft_generic_overlay_overlay_width_fill_portion")]
    pub draft_overlay_width_fill_portion: String,
    #[serde(default)]
    #[serde(alias = "draft_generic_overlay_overlay_width_dynamic")]
    pub draft_overlay_width_dynamic: String,
    #[serde(default)]
    #[serde(alias = "draft_generic_overlay_overlay_height_fixed")]
    pub draft_overlay_height_fixed: String,
    #[serde(default)]
    #[serde(alias = "draft_generic_overlay_overlay_height_fill_portion")]
    pub draft_overlay_height_fill_portion: String,
    #[serde(default)]
    #[serde(alias = "draft_generic_overlay_overlay_height_dynamic")]
    pub draft_overlay_height_dynamic: String,

    // Date Picker properties
    #[serde(default)]
    pub date_picker_mode: DatePickerSelectionMode,
    #[serde(default)]
    pub date_picker_show_time: bool,
    #[serde(default)]
    pub date_picker_initially_open: bool,
    #[serde(default)]
    pub date_picker_initial_single_date: String,
    #[serde(default)]
    pub date_picker_initial_range_start: String,
    #[serde(default)]
    pub date_picker_initial_range_end: String,
    #[serde(default)]
    pub date_picker_initial_hour: u32,
    #[serde(default)]
    pub date_picker_initial_minute: u32,

    // Split properties
    #[serde(default = "default_split_at")]
    pub split_at: f32,
    #[serde(default)]
    pub split_strategy: SplitStrategy,
    #[serde(default = "default_split_handle_width")]
    pub split_handle_width: f32,

    // Float properties
    #[serde(default = "default_float_scale")]
    pub float_scale: f32,
    #[serde(default)]
    pub float_translate_x: f32,
    #[serde(default)]
    pub float_translate_y: f32,

    // Responsive properties
    #[serde(default = "default_responsive_breakpoint")]
    pub responsive_breakpoint: f32,

    // Sensor properties
    #[serde(default)]
    pub sensor_on_show: bool,
    #[serde(default)]
    pub sensor_on_resize: bool,
    #[serde(default)]
    pub sensor_on_hide: bool,
    #[serde(default)]
    pub sensor_anticipate: f32,
    #[serde(default)]
    pub sensor_delay_ms: u64,

    // PickList properties
    pub picklist_selected: Option<String>,
    pub picklist_placeholder: String,
    pub picklist_options: Vec<String>,

    // Scrollable properties
    #[serde(with = "serde_iced::scrollable_direction")]
    pub scroll_dir: iced::widget::scrollable::Direction,
    #[serde(with = "serde_iced::scrollable_anchor")]
    pub anchor_x: iced::widget::scrollable::Anchor,
    #[serde(with = "serde_iced::scrollable_anchor")]
    pub anchor_y: iced::widget::scrollable::Anchor,

    // Rule properties
    pub rule_thickness: f32,

    // Rule + Space properties
    pub orientation: Orientation,

    // Image properties
    pub image_path: String,
    pub image_fit: ContentFitChoice,

    // Svg prroperties
    pub svg_path: String,
    pub svg_fit: ContentFitChoice,

    // Tooltip properties
    pub tooltip_text: String,
    pub tooltip_position: TooltipPosition,

    // ComboBox properties — state is transient, recreated from combobox_options on load.
    #[serde(skip, default = "default_combobox_state")]
    pub combobox_state: combo_box::State<String>,
    pub combobox_placeholder: String,
    pub combobox_selected: Option<String>,
    pub combobox_options: Vec<String>,
    pub combobox_size: f32,
    pub combobox_use_on_input: bool,
    pub combobox_use_on_option_hovered: bool,
    pub combobox_use_on_open: bool,
    pub combobox_use_on_close: bool,
    pub referenced_enum: Option<Uuid>,
    // ComboBox icon
    pub combobox_icon_enabled: bool,
    #[serde(default = "default_icon_family")]
    pub combobox_icon_family: String,
    pub combobox_icon_name: String,
    pub combobox_icon_codepoint: u32,
    pub combobox_icon_size: f32,
    pub combobox_icon_spacing: f32,
    pub combobox_icon_side: TextInputIconSide,
    pub combobox_icon_picker_filter: String,

    // Markdown properties — parsed items are transient (re-parsed from source on load).
    #[serde(skip, default)]
    pub markdown_content: Vec<markdown::Item>,
    #[serde(with = "serde_iced::text_editor_content")]
    pub markdown_source: text_editor::Content,
    pub markdown_text_size: f32,

    // QR Code properties — Data is transient (re-created from qrcode_link on load).
    pub qrcode_link: String,
    #[serde(skip, default)]
    pub qrcode_data: Option<qr_code::Data>,
    pub qrcode_cell_size: f32,

    // Themer properties — serialized as theme name string.
    #[serde(with = "serde_iced::iced_theme_opt")]
    pub themer_theme: Option<Theme>,

    // Table properties
    pub table_referenced_struct: Option<Uuid>,
    pub table_padding_x: f32,
    pub table_padding_y: f32,
    pub table_separator_x: f32,
    pub table_separator_y: f32,
    pub table_bold_headers: bool,

    // Pin properties
    #[serde(with = "serde_iced::point")]
    pub pin_point: Point,
    pub draft_pin_x: String,
    pub draft_pin_y: String,

    //Mouse_Area properties
    pub mousearea_on_press: bool,
    pub mousearea_on_release: bool,
    pub mousearea_on_double_click: bool,
    pub mousearea_on_right_press: bool,
    pub mousearea_on_right_release: bool,
    pub mousearea_on_middle_press: bool,
    pub mousearea_on_middle_release: bool,
    pub mousearea_on_scroll: bool,
    pub mousearea_on_enter: bool,
    pub mousearea_on_move: bool,
    pub mousearea_on_exit: bool,
    pub mousearea_interaction: Option<MouseInteraction>,

    pub show_widget_bounds: bool,
    pub widget_name: String,
    #[serde(with = "serde_iced::length_opt")]
    pub saved_height_before_scrollable: Option<Length>,
    #[serde(with = "serde_iced::length_opt")]
    pub saved_width_before_scrollable: Option<Length>,
    pub referenced_view_id: Option<Uuid>,
    /// Additional view IDs for multi-view selection (generates a `{Field}Selection` enum).
    pub extra_view_ids: Vec<Uuid>,

    // Icon widget properties
    #[serde(default = "default_icon_family")]
    pub icon_family: String,
    pub icon_name: String,   // Upstream icon name, e.g. "house"
    pub icon_codepoint: u32, // Unicode codepoint for rendering
    pub icon_size: f32,
    pub icon_picker_filter: String, // Live search filter for the icon picker

    // Grid properties
    pub grid_columns: usize,           // Number of columns (default 3)
    pub grid_spacing: f32,             // Spacing between cells (default 0.0)
    pub grid_fixed_width: Option<f32>, // Fixed pixel width for the grid (None = not set)
    pub grid_use_fluid: bool,          // Use fluid (auto-column) mode instead of fixed columns
    pub grid_fluid_max_width: f32,     // Max column width in fluid mode (default 200.0)

    // Action system
    pub state_field_override: Option<StateFieldRef>,
}

fn default_combobox_state() -> combo_box::State<String> {
    combo_box::State::new(Vec::new())
}

fn default_collapsible_title() -> String {
    "Collapsible".to_string()
}

fn default_collapsible_header_height() -> f32 {
    32.0
}

fn default_collapsible_header_clickable() -> bool {
    true
}

fn default_overlay_title() -> String {
    "Overlay".to_string()
}

fn default_overlay_width() -> Length {
    Length::Fixed(400.0)
}

fn default_overlay_height() -> Length {
    Length::Shrink
}

fn default_overlay_dynamic_factor() -> f32 {
    1.0
}

fn default_overlay_padding() -> f32 {
    15.0
}

fn default_overlay_radius() -> f32 {
    12.0
}

fn default_overlay_hover_gap() -> f32 {
    5.0
}

fn default_overlay_hover_alignment() -> ContainerAlignX {
    ContainerAlignX::Center
}

fn default_overlay_hover_snap() -> bool {
    true
}

fn default_overlay_opaque_alpha() -> f32 {
    0.3
}

fn default_overlay_safe_triangle() -> bool {
    true
}

fn default_split_at() -> f32 {
    0.5
}

fn default_split_handle_width() -> f32 {
    11.0
}

fn default_float_scale() -> f32 {
    1.0
}

fn default_responsive_breakpoint() -> f32 {
    720.0
}

fn default_text_editor_content() -> text_editor::Content {
    text_editor::Content::new()
}

fn default_text_editor_placeholder() -> String {
    "Start typing...".to_string()
}

fn default_text_editor_size() -> f32 {
    16.0
}

fn default_text_editor_padding() -> f32 {
    5.0
}

fn default_text_editor_font() -> FontType {
    FontType::Default
}

fn default_text_editor_line_height() -> text::LineHeight {
    text::LineHeight::default()
}

fn default_text_editor_wrapping() -> text::Wrapping {
    text::Wrapping::default()
}

impl Default for Properties {
    fn default() -> Self {
        Self {
            // Common defaults
            width: Length::Fill,
            height: Length::Fill,
            padding: Padding::new(0.0),
            max_width: None,
            max_height: None,
            clip: false,
            widget_id: None,
            custom_style_name: None,
            menu_style_name: None,
            active_style_name: None,
            style_condition_field: None,
            style_condition_value: None,

            // Draft properties
            draft_fixed_width: String::new(),
            draft_fixed_height: String::new(),
            draft_fill_portion_width: String::new(),
            draft_fill_portion_height: String::new(),
            draft_text_color: String::new(),
            padding_mode: PaddingMode::Uniform,

            // Container defaults
            border_width: 1.0,
            border_radius: 5.0,
            border_color: Color::from_rgb(0.5, 0.5, 0.5),
            background_color: Color::from_rgba(0.0, 0.0, 0.0, 0.0),
            has_shadow: false,
            shadow_offset: Vector::new(0.0, 2.0),
            shadow_blur: 5.0,
            shadow_color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
            container_sizing_mode: ContainerSizingMode::Manual,
            container_center_length: Length::Fill,

            // Row wrapping
            is_wrapping_row: false,
            wrapping_vertical_spacing: 0.0,
            match_horizontal_spacing: false,
            wrapping_align_x: ContainerAlignX::Left,

            // Layout defaults
            spacing: 0.0,
            align_items: Alignment::Start,
            align_x: ContainerAlignX::Left,
            align_y: ContainerAlignY::Top,

            // Text defaults
            text_size: 16.0, // should be None
            text_color: Color::from_rgba(0.0, 0.0, 0.0, 0.0),
            font: FontType::Default,
            line_height: text::LineHeight::default(),
            wrap: text::Wrapping::default(),
            shaping: text::Shaping::default(),
            text_align_x: text::Alignment::default(),
            text_align_y: iced::alignment::Vertical::Top,

            // Button defaults
            button_on_press_maybe_enabled: false,
            button_on_press_with_enabled: false,
            button_on_press_enabled: true,

            // TextEditor defaults
            text_editor_content: default_text_editor_content(),
            text_editor_placeholder: default_text_editor_placeholder(),
            text_editor_size: default_text_editor_size(),
            text_editor_padding: default_text_editor_padding(),
            text_editor_font: default_text_editor_font(),
            text_editor_line_height: default_text_editor_line_height(),
            text_editor_wrapping: default_text_editor_wrapping(),

            // TextInput defaults
            text_content: "Sample Text".to_string(),
            text_input_value: String::new(),
            text_input_placeholder: "Enter text...".to_string(),
            text_input_size: 16.0, // should be None
            text_input_padding: 5.0,
            is_secure: false,
            text_input_on_submit: false,
            text_input_on_paste: false,
            text_input_font: FontType::Default,
            text_input_line_height: text::LineHeight::default(),
            text_input_alignment: ContainerAlignX::Left,
            text_input_icon_enabled: false,
            text_input_icon_family: default_icon_family(),
            text_input_icon_name: "house".to_string(),
            text_input_icon_codepoint: 0xE1D7,
            text_input_icon_size: 0.0, // 0.0 = use font default
            text_input_icon_spacing: 5.0,
            text_input_icon_side: TextInputIconSide::Left,
            text_input_icon_picker_filter: String::new(),

            // Checkbox defaults
            checkbox_checked: false,
            checkbox_label: "Check me".to_string(),
            checkbox_size: 16.0,
            checkbox_spacing: 8.0,

            // Radio defaults
            radio_selected_index: 0,
            radio_options: vec![
                "Option 1".to_string(),
                "Option 2".to_string(),
                "Option 3".to_string(),
            ],
            radio_label: "Radio Option".to_string(),
            radio_size: radio::Radio::<Theme>::DEFAULT_SIZE,
            radio_spacing: radio::Radio::<Theme>::DEFAULT_SPACING,

            // Slider defaults
            slider_value: 50.0,
            slider_min: 0.0,
            slider_max: 100.0,
            slider_step: 1.0,
            slider_height: slider::Slider::<f32, Theme>::DEFAULT_HEIGHT,
            slider_width: vertical_slider::VerticalSlider::<f32, Theme>::DEFAULT_WIDTH,

            // Progress defaults
            progress_min: 0.0,
            progress_max: 1.0,
            progress_value: 0.5,
            progress_length: Length::Fill,
            progress_girth: progress_bar::ProgressBar::<Theme>::DEFAULT_GIRTH,
            progress_vertical: false,

            // Toggler defaults
            toggler_active: false,
            toggler_label: "Toggle me".to_string(),
            toggler_size: toggler::Toggler::<Theme>::DEFAULT_SIZE,
            toggler_spacing: toggler::Toggler::<Theme>::DEFAULT_SIZE / 2.0,

            // Collapsible defaults
            collapsible_title: default_collapsible_title(),
            collapsible_header_height: default_collapsible_header_height(),
            collapsible_header_clickable: default_collapsible_header_clickable(),
            collapsible_expanded: false,

            // Generic Overlay defaults
            overlay_title: default_overlay_title(),
            overlay_width: default_overlay_width(),
            overlay_height: default_overlay_height(),
            overlay_width_dynamic: false,
            overlay_height_dynamic: false,
            overlay_width_dynamic_factor: default_overlay_dynamic_factor(),
            overlay_height_dynamic_factor: default_overlay_dynamic_factor(),
            overlay_padding: default_overlay_padding(),
            overlay_radius: default_overlay_radius(),
            overlay_style_name: None,
            overlay_on_hover: false,
            overlay_hover_positions_on_click: false,
            overlay_initially_open: false,
            overlay_hover_position: OverlayPosition::default(),
            overlay_hover_gap: default_overlay_hover_gap(),
            overlay_hover_alignment: default_overlay_hover_alignment(),
            overlay_hover_mode: OverlayPositionMode::default(),
            overlay_hover_snap: default_overlay_hover_snap(),
            overlay_close_on_click_outside: false,
            overlay_opaque: false,
            overlay_opaque_alpha: default_overlay_opaque_alpha(),
            overlay_hide_header: false,
            overlay_hide_close_button: false,
            overlay_block_dragging: false,
            overlay_resizable: OverlayResizeMode::default(),
            overlay_reset_on_close: false,
            overlay_interactive_base: false,
            overlay_animate: false,
            overlay_animation_preset: OverlayAnimationPreset::default(),
            overlay_safe_triangle: default_overlay_safe_triangle(),

            // The rest of the overlay surface. These start from the widgets'
            // own defaults; `apply_defaults_for_type` then narrows them to the
            // particular widget being placed.
            overlay_min_width: 0.0,
            overlay_max_width: None,

            menu_submenu_gap: default_menu_submenu_gap(),
            menu_item_padding: default_menu_item_padding(),
            menu_trigger_padding: default_menu_trigger_padding(),
            menu_plain_trigger: false,
            menu_check_side: MenuCheckSide::default(),
            menu_gutter_spacing: default_menu_gutter_spacing(),
            menu_item_kind: MenuItemKind::default(),
            menu_item_checked: false,
            menu_item_enabled: true,

            layer_anchor: OverlayAnchor::Center,
            layer_min_height: 0.0,
            layer_stretch: false,
            layer_position_enabled: false,
            layer_position_x: 0.0,
            layer_position_y: 0.0,
            layer_dismiss_on_escape: true,

            toast_anchor: default_toast_anchor(),
            toast_margin: default_toast_margin(),
            toast_spacing: default_toast_spacing(),
            toast_width: default_toast_width(),
            toast_variant: ToastVariant::default(),
            toast_without_icon: false,
            toast_timeout_secs: 0.0,

            date_picker_position_enabled: false,
            date_picker_position_x: 0.0,
            date_picker_position_y: 0.0,
            draft_overlay_width_fixed: String::new(),
            draft_overlay_width_fill_portion: String::new(),
            draft_overlay_width_dynamic: String::new(),
            draft_overlay_height_fixed: String::new(),
            draft_overlay_height_fill_portion: String::new(),
            draft_overlay_height_dynamic: String::new(),

            // Date Picker defaults
            date_picker_mode: DatePickerSelectionMode::default(),
            date_picker_show_time: false,
            date_picker_initially_open: false,
            date_picker_initial_single_date: String::new(),
            date_picker_initial_range_start: String::new(),
            date_picker_initial_range_end: String::new(),
            date_picker_initial_hour: 0,
            date_picker_initial_minute: 0,

            // Split defaults
            split_at: default_split_at(),
            split_strategy: SplitStrategy::default(),
            split_handle_width: default_split_handle_width(),

            // Float defaults
            float_scale: default_float_scale(),
            float_translate_x: 0.0,
            float_translate_y: 0.0,

            // Responsive defaults
            responsive_breakpoint: default_responsive_breakpoint(),

            // Sensor defaults
            sensor_on_show: false,
            sensor_on_resize: false,
            sensor_on_hide: false,
            sensor_anticipate: 0.0,
            sensor_delay_ms: 0,

            // PickList defaults
            picklist_selected: None,
            picklist_placeholder: String::new(),
            picklist_options: vec![
                "Option 1".to_string(),
                "Option 2".to_string(),
                "Option 3".to_string(),
            ],

            // Scrollable defaults
            scroll_dir: iced::widget::scrollable::Direction::default(),
            anchor_x: iced::widget::scrollable::Anchor::default(),
            anchor_y: iced::widget::scrollable::Anchor::default(),

            // Rule defaults
            rule_thickness: 5.0,

            // Rule + Space Orientation
            orientation: Orientation::Horizontal,

            // Image defaults
            image_path: String::new(),
            image_fit: ContentFitChoice::Contain,

            // Svg defaults
            svg_path: String::new(),
            svg_fit: ContentFitChoice::Contain,

            // Tooltip defaults
            tooltip_text: "Tooltip".to_string(),
            tooltip_position: TooltipPosition::Top,

            // ComboBox defaults
            combobox_state: combo_box::State::new(vec![
                "Option 1".to_string(),
                "Option 2".to_string(),
                "Option 3".to_string(),
            ]),

            combobox_selected: None,
            combobox_placeholder: "Type to search...".to_string(),
            combobox_options: vec![
                "Option 1".to_string(),
                "Option 2".to_string(),
                "Option 3".to_string(),
            ],
            combobox_use_on_input: false,
            combobox_use_on_option_hovered: false,
            combobox_use_on_open: false,
            combobox_use_on_close: false,
            combobox_size: 16.0,
            referenced_enum: None,
            combobox_icon_enabled: false,
            combobox_icon_family: default_icon_family(),
            combobox_icon_name: "house".to_string(),
            combobox_icon_codepoint: 0xE1D7,
            combobox_icon_size: 0.0,
            combobox_icon_spacing: 5.0,
            combobox_icon_side: TextInputIconSide::Left,
            combobox_icon_picker_filter: String::new(),

            // Markdown defaults
            markdown_content: Vec::new(),
            markdown_source: text_editor::Content::with_text(""),
            markdown_text_size: 16.0,

            // QR Code defaults
            qrcode_link: "https://example.com".to_string(),
            qrcode_data: Some(qr_code::Data::new("https://example.com").unwrap()),
            qrcode_cell_size: 4.0,

            // Themer defaults
            themer_theme: None,

            // Table defaults
            table_referenced_struct: None,
            table_padding_x: 10.0,
            table_padding_y: 5.0,
            table_separator_x: 1.0,
            table_separator_y: 1.0,
            table_bold_headers: false,

            // Pin defaults
            pin_point: Point::ORIGIN,
            draft_pin_x: String::new(),
            draft_pin_y: String::new(),

            // Mouse_area defaults
            mousearea_on_press: false,
            mousearea_on_release: false,
            mousearea_on_double_click: false,
            mousearea_on_right_press: false,
            mousearea_on_right_release: false,
            mousearea_on_middle_press: false,
            mousearea_on_middle_release: false,
            mousearea_on_scroll: false,
            mousearea_on_enter: false,
            mousearea_on_move: false,
            mousearea_on_exit: false,
            mousearea_interaction: None,

            show_widget_bounds: false,
            widget_name: String::new(),
            saved_height_before_scrollable: None,
            saved_width_before_scrollable: None,
            referenced_view_id: None,
            extra_view_ids: Vec::new(),

            // Icon defaults — "house" icon from Lucide
            icon_family: default_icon_family(),
            icon_name: "house".to_string(),
            icon_codepoint: 0xE0F5,
            icon_size: 16.0,
            icon_picker_filter: String::new(),

            // Grid defaults
            grid_columns: 3,
            grid_spacing: 0.0,
            grid_fixed_width: None,
            grid_use_fluid: false,
            grid_fluid_max_width: 200.0,

            // Action system defaults
            state_field_override: None,
        }
    }
}

impl Properties {
    pub fn for_widget_type(widget_type: WidgetType) -> Self {
        let mut props = Self::default();

        // Customize defaults based on widget type [ Match actual iced defaults ]
        match widget_type {
            // `container` and `scrollable` both start at `Length::Fit`.
            WidgetType::Container | WidgetType::Scrollable => {
                props.width = Length::Fit;
                props.height = Length::Fit;
            }
            // `column` and `row` both start at `Length::Fit`.
            WidgetType::Column | WidgetType::Row => {
                props.width = Length::Fit;
                props.height = Length::Fit;
            }
            // `button::DEFAULT_PADDING` with `Length::Fit` on both axes.
            WidgetType::Button => {
                props.text_content = "Click Me!".to_string();
                props.width = Length::Fit;
                props.height = Length::Fit;
                props.padding_mode = PaddingMode::Symmetric;
                props.padding = Padding {
                    top: 5.0,
                    bottom: 5.0,
                    right: 10.0,
                    left: 10.0,
                };
            }
            WidgetType::Text => {
                props.text_content = "Sample Text".to_string();
                props.width = Length::Shrink;
                props.height = Length::Shrink;
            }
            WidgetType::TextEditor => {
                props.width = Length::Fill;
                props.height = Length::Fixed(180.0);
                props.text_editor_placeholder = default_text_editor_placeholder();
                props.text_editor_size = default_text_editor_size();
                props.text_editor_padding = default_text_editor_padding();
                props.text_editor_font = default_text_editor_font();
                props.text_editor_line_height = default_text_editor_line_height();
                props.text_editor_wrapping = default_text_editor_wrapping();
            }
            // `text_input` fills its row and fits its line, with 5px padding.
            WidgetType::TextInput => {
                props.text_input_placeholder = "Enter text...".to_string();
                props.width = Length::Fill;
                props.height = Length::Fit;
                props.padding_mode = PaddingMode::Uniform;
                props.padding = Padding::new(5.0);
            }
            WidgetType::Checkbox => {
                props.checkbox_label = "Check me".to_string();
                props.width = Length::Shrink;
            }
            WidgetType::Radio => {
                props.radio_options =
                    vec!["Radio Option 1".to_string(), "Radio Option 2".to_string()];
                props.width = Length::Shrink;
            }
            WidgetType::Toggler => {
                props.toggler_label = "Toggle me".to_string();
                props.width = Length::Shrink;
            }
            WidgetType::Collapsible => {
                props.collapsible_title = "Collapsible".to_string();
                props.width = Length::Fill;
                props.height = Length::Shrink;
                props.padding_mode = PaddingMode::Symmetric;
                props.padding = Padding {
                    top: 4.0,
                    right: 8.0,
                    bottom: 4.0,
                    left: 8.0,
                };
            }
            WidgetType::CollapsibleGroup => {
                props.width = Length::Fill;
                props.height = Length::Shrink;
                props.spacing = 0.0;
            }
            WidgetType::Popover | WidgetType::Menu | WidgetType::Layer | WidgetType::Toast => {
                props.text_content = match widget_type {
                    WidgetType::Menu => "Menu".to_string(),
                    WidgetType::Toast => "Notifications".to_string(),
                    _ => "Open".to_string(),
                };
                props.width = Length::Shrink;
                props.height = Length::Shrink;
                props.padding_mode = PaddingMode::Symmetric;
                props.padding = Padding {
                    top: 5.0,
                    bottom: 5.0,
                    right: 10.0,
                    left: 10.0,
                };

                // The surface sizes itself to its contents unless bounded, the
                // way each of these widgets does when constructed in code.
                props.overlay_width = Length::Shrink;
                props.overlay_height = Length::Shrink;
                props.overlay_min_width = 0.0;
                props.overlay_max_width = None;
                props.overlay_animate = true;

                // Each widget's own `new()`, so a widget placed here starts out
                // looking like one written by hand.
                match widget_type {
                    // `Placement::new(Side::Bottom).align(Align::Center).gap(6)`,
                    // padding 12, radius 8, `Motion::QUICK`.
                    WidgetType::Popover => {
                        props.overlay_hover_position = OverlayPosition::Bottom;
                        props.overlay_hover_alignment = ContainerAlignX::Center;
                        props.overlay_hover_gap = 6.0;
                        props.overlay_padding = 12.0;
                        props.overlay_radius = 8.0;
                        props.overlay_animation_preset = OverlayAnimationPreset::Quick;
                        props.overlay_hover_snap = true;
                        props.overlay_safe_triangle = true;
                        props.overlay_close_on_click_outside = true;
                    }
                    // `Placement::new(Side::Bottom).gap(4)`, panel padding 4,
                    // radius 6, `Motion::QUICK`.
                    WidgetType::Menu => {
                        props.overlay_hover_position = OverlayPosition::Bottom;
                        props.overlay_hover_alignment = ContainerAlignX::Left;
                        props.overlay_hover_gap = 4.0;
                        props.overlay_padding = 4.0;
                        props.overlay_radius = 6.0;
                        props.overlay_animation_preset = OverlayAnimationPreset::Quick;
                    }
                    // `Anchor::Center`, margin 16, padding 16, radius 10,
                    // `Motion::SMOOTH`, no backdrop, and neither draggable nor
                    // resizable. `dialog()` and `modal()` are the presets that
                    // add those; a bare `layer()` has none of them.
                    WidgetType::Layer => {
                        props.layer_anchor = OverlayAnchor::Center;
                        props.overlay_hover_gap = 16.0;
                        props.overlay_padding = 16.0;
                        props.overlay_radius = 10.0;
                        props.overlay_animation_preset = OverlayAnimationPreset::Default;
                        props.overlay_opaque = false;
                        props.overlay_block_dragging = true;
                        props.overlay_resizable = OverlayResizeMode::None;
                        props.overlay_hide_close_button = false;
                        props.overlay_close_on_click_outside = true;
                        props.layer_dismiss_on_escape = true;
                    }
                    // `Anchor::BottomRight`, margin 16, spacing 10, padding 14,
                    // radius 8, width 360, `Motion::SMOOTH`.
                    _ => {
                        props.overlay_padding = 14.0;
                        props.overlay_radius = 8.0;
                        props.overlay_animation_preset = OverlayAnimationPreset::Default;
                    }
                }
            }
            WidgetType::DatePicker => {
                props.text_content = "Select a date".to_string();
                props.width = Length::Shrink;
                props.height = Length::Shrink;
                props.padding_mode = PaddingMode::Symmetric;
                props.padding = Padding {
                    top: 5.0,
                    bottom: 5.0,
                    right: 10.0,
                    left: 10.0,
                };
            }
            WidgetType::Split => {
                props.width = Length::Fill;
                props.height = Length::Fill;
                props.orientation = Orientation::Vertical;
                props.split_at = default_split_at();
                props.split_strategy = SplitStrategy::default();
                props.split_handle_width = default_split_handle_width();
            }
            WidgetType::Float => {
                props.width = Length::Shrink;
                props.height = Length::Shrink;
                props.float_scale = default_float_scale();
            }
            WidgetType::Responsive => {
                props.width = Length::Fill;
                props.height = Length::Fill;
                props.responsive_breakpoint = default_responsive_breakpoint();
            }
            WidgetType::Sensor => {
                props.width = Length::Shrink;
                props.height = Length::Shrink;
                props.sensor_anticipate = 0.0;
                props.sensor_delay_ms = 0;
            }
            WidgetType::PickList => {
                props.padding_mode = PaddingMode::Symmetric;
                props.padding = Padding {
                    top: 5.0,
                    bottom: 5.0,
                    right: 10.0,
                    left: 10.0,
                }; // Same as button's padding
                props.width = Length::Shrink;
            }
            WidgetType::Space => {
                props.show_widget_bounds = true;
                props.height = Length::Shrink;
            }
            WidgetType::Image => {
                props.width = Length::Shrink;
                props.height = Length::Shrink;
                props.show_widget_bounds = true;
            }
            // `svg` fills its row and takes its natural height.
            WidgetType::Svg => {
                props.width = Length::Fill;
                props.height = Length::Shrink;
                props.show_widget_bounds = true;
            }
            WidgetType::Tooltip => {
                props.width = Length::Shrink;
                props.height = Length::Shrink;
            }

            WidgetType::Markdown => {
                props.height = Length::Shrink;
            }
            WidgetType::QRCode => {
                props.width = Length::Shrink;
                props.height = Length::Shrink;
            }
            WidgetType::ComboBox => {
                props.combobox_state = combo_box::State::new(props.combobox_options.clone());
            }
            WidgetType::Table => {
                props.width = Length::Fill;
                props.height = Length::Fill;
            }
            WidgetType::Icon => {
                props.width = Length::Shrink;
                props.height = Length::Shrink;
            }
            WidgetType::Stack => {
                // iced Stack defaults to Shrink (NOT Fill like most widgets)
                props.width = Length::Shrink;
                props.height = Length::Shrink;
            }
            _ => {} // Use defaults for other types
        }

        props
    }
}

#[derive(Debug, Clone, Default)]
pub struct CommonProperties {
    // Track which categories of properties are common
    pub has_width_height: bool,
    pub has_padding: bool,
    pub has_spacing: bool,
    pub has_alignment: bool,
    pub has_text_properties: bool,
    pub has_border: bool,
    pub has_background: bool,

    // Store the actual values (if all widgets have same value)
    pub uniform_width: Option<Length>,
    pub uniform_height: Option<Length>,
    pub uniform_padding_mode: Option<PaddingMode>,
    pub uniform_padding: Option<Padding>,
    pub uniform_spacing: Option<f32>,
    pub uniform_text_size: Option<f32>,

    pub draft_fixed_width: String,
    pub draft_fixed_height: String,
    pub draft_fill_portion_width: String,
    pub draft_fill_portion_height: String,
}

impl CommonProperties {
    /// Check widgets to find common properties
    pub fn from_widgets(widgets: &[&Widget]) -> Self {
        if widgets.is_empty() {
            return Self::default();
        }

        // All widgets have width/height
        let has_width_height = true;

        // Check if all widgets have padding (containers do, text doesn't)
        let has_padding = widgets.iter().all(|w| {
            matches!(
                w.widget_type,
                WidgetType::Container
                    | WidgetType::Button
                    | WidgetType::Row
                    | WidgetType::Column
                    | WidgetType::Scrollable
            )
        });

        // Check if all widgets have spacing (only Row/Column)
        let has_spacing = widgets
            .iter()
            .all(|w| matches!(w.widget_type, WidgetType::Row | WidgetType::Column));

        // Check if all widgets have text properties
        let has_text_properties = widgets.iter().all(|w| {
            matches!(
                w.widget_type,
                WidgetType::Text | WidgetType::Button | WidgetType::TextInput
            )
        });

        // Check for uniform values
        let uniform_width = Self::get_uniform_property(widgets, |w| w.properties.width);
        let uniform_height = Self::get_uniform_property(widgets, |w| w.properties.height);
        let uniform_padding_mode =
            Self::get_uniform_property(widgets, |w| w.properties.padding_mode);
        let uniform_padding = Self::get_uniform_property(widgets, |w| w.properties.padding);
        let uniform_spacing = if has_spacing {
            Self::get_uniform_property(widgets, |w| w.properties.spacing)
        } else {
            None
        };
        let uniform_text_size = if has_text_properties {
            Self::get_uniform_property(widgets, |w| w.properties.text_size)
        } else {
            None
        };

        Self {
            has_width_height,
            has_padding,
            has_spacing,
            has_alignment: false, // todo
            has_text_properties,
            has_border: false,     // todo
            has_background: false, // todo
            uniform_width,
            uniform_height,
            uniform_padding_mode,
            uniform_padding,
            uniform_spacing,
            uniform_text_size,
            draft_fixed_width: String::new(),
            draft_fixed_height: String::new(),
            draft_fill_portion_width: String::new(),
            draft_fill_portion_height: String::new(),
        }
    }

    /// Helper to check if all widgets have the same value for a property
    fn get_uniform_property<T, F>(widgets: &[&Widget], getter: F) -> Option<T>
    where
        T: PartialEq + Clone,
        F: Fn(&Widget) -> T,
    {
        if widgets.is_empty() {
            return None;
        }

        let first_value = getter(widgets[0]);

        if widgets.iter().all(|w| getter(w) == first_value) {
            Some(first_value)
        } else {
            None
        }
    }
}

// ── Defaults matching each widget's own ──────────────────────────────────────
// The values the widgets crate itself starts from, so a freshly placed widget
// in the builder looks like one constructed in code.

fn default_true() -> bool {
    true
}

/// `widgets::menu::DEFAULT_SUBMENU_GAP`
fn default_menu_submenu_gap() -> f32 {
    -4.0
}

/// `widgets::menu::Menu::new`
fn default_menu_item_padding() -> Padding {
    Padding::from([4.0, 8.0])
}

/// `widgets::menu::DEFAULT_TRIGGER_PADDING`
fn default_menu_trigger_padding() -> Padding {
    Padding::from([4.0, 10.0])
}

/// `widgets::menu::DEFAULT_GUTTER_SPACING`
fn default_menu_gutter_spacing() -> f32 {
    8.0
}

/// `widgets::toast::Toasts::new`
fn default_toast_anchor() -> OverlayAnchor {
    OverlayAnchor::BottomRight
}

/// `widgets::toast::DEFAULT_MARGIN`
fn default_toast_margin() -> f32 {
    16.0
}

/// `widgets::toast::DEFAULT_SPACING`
fn default_toast_spacing() -> f32 {
    10.0
}

/// `widgets::toast::DEFAULT_WIDTH`
fn default_toast_width() -> f32 {
    360.0
}

/// The width bound offered when a max-width is first switched on.
///
/// The widgets themselves default to no upper bound at all, so this is only a
/// starting point for the control, not a widget default.
pub const DEFAULT_OVERLAY_MAX_WIDTH: f32 = 320.0;

/// The icon family a project starts from, and the one a save written before
/// families existed is assumed to have used.
///
/// Lucide was the only set `iced_lucide` offered at the time, so every icon
/// name in an older `.adui` is a Lucide name.
pub fn default_icon_family() -> String {
    "lucide".to_string()
}

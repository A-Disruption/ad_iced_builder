use iced::{Length, Padding, Color, Vector, Alignment, Point, Theme};
use iced::widget::{combo_box, markdown, progress_bar, qr_code, radio, slider, text, text_editor, toggler, vertical_slider};
use uuid::Uuid;

use crate::data_structures::types::type_implementations::*;
use crate::data_structures::types::types::*;


#[derive(Debug, )]
pub struct Properties {
    pub width: Length,
    pub height: Length,
    pub max_width: Option<f32>,
    pub max_height: Option<f32>,
    pub clip: bool, 
    pub padding: Padding,
    pub widget_id: Option<String>,
    pub custom_style_name: Option<String>,


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
    pub border_color: Color,
    pub background_color: Color,
    pub has_shadow: bool,
    pub shadow_offset: Vector,
    pub shadow_blur: f32,
    pub shadow_color: Color,
    pub container_sizing_mode: ContainerSizingMode,
    pub container_center_length: Length,  // Used when center_x/y/both is active

    // Row wrapping
    pub is_wrapping_row: bool,
    pub wrapping_vertical_spacing: f32,
    pub match_horizontal_spacing: bool,
    pub wrapping_align_x: ContainerAlignX,
    
    // Layout properties (Row/Column)
    pub spacing: f32,
    pub align_items: Alignment,
    
    // Text properties
    pub text_content: String,
    pub text_size: f32,
    pub text_color: Color,
    pub font: FontType,
    pub line_height: text::LineHeight,
    pub wrap: text::Wrapping,
    pub shaping: text::Shaping,
    pub text_align_x: text::Alignment,
    pub text_align_y: iced::alignment::Vertical,
    
    // Button properties
//    pub button_style: ButtonStyleType,
    pub button_on_press_maybe_enabled: bool,
    pub button_on_press_with_enabled: bool,
    pub button_on_press_enabled: bool,
    
    // TextInput properties
    pub text_input_value: String,
    pub text_input_placeholder: String,
    pub text_input_size: f32,
    pub text_input_padding: f32,
    pub is_secure: bool,
    pub text_input_on_submit: bool,
    pub text_input_on_paste: bool,
    pub text_input_font: FontType,
    pub text_input_line_height: text::LineHeight,
    pub text_input_alignment: ContainerAlignX,
    // TextInput icon (Lucide)
    pub text_input_icon_enabled: bool,
    pub text_input_icon_name: String,
    pub text_input_icon_codepoint: u32,
    pub text_input_icon_size: f32,     // 0.0 = use font default (None in iced)
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
    pub progress_length: Length,
    pub progress_girth: f32,
    pub progress_vertical: bool,
    
    // Toggler properties
    pub toggler_active: bool,
    pub toggler_label: String,
    pub toggler_size: f32,
    pub toggler_spacing: f32,
    
    // PickList properties
    pub picklist_selected: Option<String>,
    pub picklist_placeholder: String,
    pub picklist_options: Vec<String>,
    
    // Scrollable properties
    pub scroll_dir: iced::widget::scrollable::Direction,
    pub anchor_x: iced::widget::scrollable::Anchor,
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

    // ComboBox properties
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
    // ComboBox icon (Lucide)
    pub combobox_icon_enabled: bool,
    pub combobox_icon_name: String,
    pub combobox_icon_codepoint: u32,
    pub combobox_icon_size: f32,
    pub combobox_icon_spacing: f32,
    pub combobox_icon_side: TextInputIconSide,
    pub combobox_icon_picker_filter: String,

    // Markdown properties
    pub markdown_content: Vec<markdown::Item>,
    pub markdown_source: text_editor::Content,
    pub markdown_text_size: f32,

    // QR Code properties
    pub qrcode_link: String,
    pub qrcode_data: Option<qr_code::Data>,
    pub qrcode_cell_size: f32,

    // Themer properties
    pub themer_theme: Option<Theme>,

    // Table properties
    pub table_referenced_struct: Option<Uuid>,
    pub table_padding_x: f32,
    pub table_padding_y: f32,
    pub table_separator_x: f32,
    pub table_separator_y: f32,
    pub table_bold_headers: bool,

    // Pin properties
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
    pub saved_height_before_scrollable: Option<Length>,
    pub saved_width_before_scrollable: Option<Length>,
    pub referenced_view_id: Option<Uuid>,

    // Icon (Lucide) properties
    pub icon_name: String,          // Human-readable name, e.g. "house"
    pub icon_codepoint: u32,        // Unicode codepoint for rendering
    pub icon_size: f32,
    pub icon_picker_filter: String, // Live search filter for the icon picker

    // Grid properties
    pub grid_columns: usize,        // Number of columns (default 3)
    pub grid_spacing: f32,          // Spacing between cells (default 0.0)
    pub grid_fixed_width: Option<f32>, // Fixed pixel width for the grid (None = not set)
    pub grid_use_fluid: bool,       // Use fluid (auto-column) mode instead of fixed columns
    pub grid_fluid_max_width: f32,  // Max column width in fluid mode (default 200.0)
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
            text_color:  Color::from_rgba(0.0, 0.0, 0.0, 0.0),
            font: FontType::Default,
            line_height: text::LineHeight::default(),
            wrap: text::Wrapping::default(),
            shaping: text::Shaping::default(),
            text_align_x: text::Alignment::default(),
            text_align_y: iced::alignment::Vertical::Top,
            
            // Button defaults
//            button_style: ButtonStyleType::Primary,
            button_on_press_maybe_enabled: false,
            button_on_press_with_enabled: false,
            button_on_press_enabled: true,
            
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
            text_input_icon_name: "house".to_string(),
            text_input_icon_codepoint: 0xE1D7,
            text_input_icon_size: 0.0,     // 0.0 = use font default
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

            // Icon defaults — "house" icon from Lucide
            icon_name: "house".to_string(),
            icon_codepoint: 0xE1D7,
            icon_size: 24.0,
            icon_picker_filter: String::new(),

            // Grid defaults
            grid_columns: 3,
            grid_spacing: 0.0,
            grid_fixed_width: None,
            grid_use_fluid: false,
            grid_fluid_max_width: 200.0,
        }
    }
}

impl Properties {
    pub fn for_widget_type(widget_type: WidgetType) -> Self {
        let mut props = Self::default();
        
        // Customize defaults based on widget type [ Match actual iced defaults ]
        match widget_type {
            WidgetType::Container => {
                // Shrink = "let iced decide" (fluid/child-based default).
                // Explicit Fill can be set by the user when needed.
                props.width = Length::Shrink;
                props.height = Length::Shrink;
            }
            WidgetType::Scrollable => {
                props.width = Length::Shrink;
                props.height = Length::Shrink;
            }
            WidgetType::Column => {
                props.width = Length::Shrink;
                props.height = Length::Shrink;
            }
            WidgetType::Row => {
                props.width = Length::Shrink;
                props.height = Length::Shrink;
            }
            WidgetType::Button => {
                props.text_content = "Click Me!".to_string();
                props.width = Length::Shrink;
                props.height = Length::Shrink;
                props.padding_mode = PaddingMode::Symmetric;
                props.padding = Padding { top: 5.0, bottom: 5.0, right: 10.0, left: 10.0 };
            }
            WidgetType::Text => {
                props.text_content = "Sample Text".to_string();
                props.width = Length::Shrink;
                props.height = Length::Shrink;
            }
            WidgetType::TextInput => {
                props.text_input_placeholder = "Enter text...".to_string();
            }
            WidgetType::Checkbox => {
                props.checkbox_label = "Check me".to_string();
                props.width = Length::Shrink;
            }
            WidgetType::Radio => {
                props.radio_options = vec![
                    "Radio Option 1".to_string(),
                    "Radio Option 2".to_string(),
                ];
                props.width = Length::Shrink;
            }
            WidgetType::Toggler => {
                props.toggler_label = "Toggle me".to_string();
                props.width = Length::Shrink;
            }
            WidgetType::PickList => {
                props.padding_mode = PaddingMode::Symmetric;
                props.padding = Padding { top: 5.0, bottom: 5.0, right: 10.0, left: 10.0 }; // Same as button's padding
                props.width = Length::Shrink;
            }
            WidgetType::Space => {
                props.show_widget_bounds = true;
                props.height = Length::Shrink;
            }
            WidgetType::Image => {
                props.width  = Length::Shrink;
                props.height = Length::Shrink;
                props.show_widget_bounds = true;
            }
            WidgetType::Svg => {
                props.height = Length::Shrink;
                props.show_widget_bounds = true;
            }
            WidgetType::Tooltip => {
                props.width  = Length::Shrink;
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
                WidgetType::Container | WidgetType::Button | WidgetType::Row | 
                WidgetType::Column | WidgetType::Scrollable
            )
        });
        
        // Check if all widgets have spacing (only Row/Column)
        let has_spacing = widgets.iter().all(|w| {
            matches!(w.widget_type, WidgetType::Row | WidgetType::Column)
        });
        
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
        let uniform_padding_mode = Self::get_uniform_property(widgets, |w| w.properties.padding_mode);
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
            has_border: false,    // todo
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
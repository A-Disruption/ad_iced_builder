use std::collections::HashSet;
use iced::advanced::text;
use iced::Alignment;
use crate::data_structures::types::{type_implementations::{ContainerAlignX, ContainerAlignY, FontType, PaddingMode}, types::{WidgetType, Widget}};

pub struct ImportTracker {
    pub used_widgets: HashSet<&'static str>,
    
    pub uses_length: bool,
    pub uses_alignment: bool,
    pub uses_padding: bool,
    pub uses_color: bool,
    
    // Text properties
    pub uses_text_line_height: bool,
    pub uses_text_wrapping: bool,
    pub uses_text_shaping: bool,
    pub uses_text_alignment: bool,
    
    // Mouse
    pub uses_mouse: bool,
    pub uses_mouse_interaction: bool,
    pub uses_mouse_scroll_delta: bool,
    
    // Other
    pub uses_point: bool,
    pub uses_font: bool,
    pub uses_border: bool,
    pub uses_shadow: bool,
    pub uses_background: bool,
    pub uses_vector: bool,
}

impl ImportTracker {
    pub fn new() -> Self {
        Self {
            used_widgets: HashSet::new(),
            uses_length: false,
            uses_alignment: false,
            uses_padding: false,
            uses_color: false,
            uses_text_line_height: false,
            uses_text_wrapping: false,
            uses_text_shaping: false,
            uses_text_alignment: false,
            uses_mouse: false,
            uses_mouse_interaction: false,
            uses_mouse_scroll_delta: false,
            uses_point: false,
            uses_font: false,
            uses_border: false,
            uses_shadow: false,
            uses_background: false,
            uses_vector: false,
        }
    }
    
    pub fn scan_widget(&mut self, widget: &Widget) {
        let props = &widget.properties;
        
        // Track widget type
        match widget.widget_type {
            WidgetType::Container => { self.used_widgets.insert("container"); }
            WidgetType::Row => { self.used_widgets.insert("row"); }
            WidgetType::Column => { self.used_widgets.insert("column"); }
            WidgetType::Button => { self.used_widgets.insert("button"); }
            WidgetType::Text => { self.used_widgets.insert("text"); }
            WidgetType::TextInput => { self.used_widgets.insert("text_input"); }
            WidgetType::Checkbox => { self.used_widgets.insert("checkbox"); }
            WidgetType::Radio => { self.used_widgets.insert("radio"); }
            WidgetType::Slider => { self.used_widgets.insert("slider"); }
            WidgetType::VerticalSlider => { self.used_widgets.insert("vertical_slider"); }
            WidgetType::ProgressBar => { self.used_widgets.insert("progress_bar"); }
            WidgetType::Toggler => { self.used_widgets.insert("toggler"); }
            WidgetType::PickList => { self.used_widgets.insert("pick_list"); }
            WidgetType::Scrollable => { self.used_widgets.insert("scrollable"); }
            WidgetType::Space => { self.used_widgets.insert("space"); }
            WidgetType::Rule => { self.used_widgets.insert("rule"); }
            WidgetType::Image => { self.used_widgets.insert("image"); }
            WidgetType::Svg => { self.used_widgets.insert("svg"); }
            WidgetType::Tooltip => { self.used_widgets.insert("tooltip"); }
            WidgetType::ComboBox => { self.used_widgets.insert("combo_box"); }
            WidgetType::Markdown => { self.used_widgets.insert("markdown"); }
            WidgetType::MouseArea => { 
                self.used_widgets.insert("mouse_area");
                self.uses_mouse = true;
            }
            WidgetType::QRCode => { self.used_widgets.insert("qr_code"); }
            WidgetType::Stack => { self.used_widgets.insert("stack"); }
            WidgetType::Themer => { self.used_widgets.insert("themer"); }
            WidgetType::Pin => { self.used_widgets.insert("pin"); }
            WidgetType::ViewReference => {}
        }
        
        // Track if any Length is used (always true if widget exists)
        self.uses_length = true;
        
        // Track Alignment if used in Row/Column
        if matches!(widget.widget_type, WidgetType::Row | WidgetType::Column) {
            if props.align_items != Alignment::Start {
                self.uses_alignment = true;
            }
        }
        
        // Track Padding
        if props.padding_mode == PaddingMode::Individual {
            self.uses_padding = true;
        }
        
        // Track Container-specific features
        if widget.widget_type == WidgetType::Container {
            if props.border_width > 0.0 {
                self.uses_border = true;
            }
            if props.background_color.a > 0.0 {
                self.uses_background = true;
                self.uses_color = true;
            }
            if props.has_shadow {
                self.uses_shadow = true;
                self.uses_vector = true;
            }
            if props.align_x != ContainerAlignX::Left || props.align_y != ContainerAlignY::Top {
                self.uses_alignment = true;
            }
        }
        
        // Track Text properties
        if widget.widget_type == WidgetType::Text {
            if props.text_color.a > 0.0 {
                self.uses_color = true;
            }
            if props.font != FontType::Default {
                self.uses_font = true;
            }
            if props.line_height != text::LineHeight::default() {
                self.uses_text_line_height = true;
            }
            if props.wrap != text::Wrapping::default() {
                self.uses_text_wrapping = true;
            }
            if props.shaping != text::Shaping::default() {
                self.uses_text_shaping = true;
            }
            if props.text_align_x != text::Alignment::default() || 
               props.text_align_y != iced::alignment::Vertical::Top {
                self.uses_text_alignment = true;
                self.uses_alignment = true;
            }
        }
        
        // Track TextInput properties
        if widget.widget_type == WidgetType::TextInput {
            if props.text_input_font != FontType::Default {
                self.uses_font = true;
            }
            if props.text_input_line_height != text::LineHeight::default() {
                self.uses_text_line_height = true;
            }
            if props.text_input_alignment != ContainerAlignX::Left {
                self.uses_alignment = true;
            }
        }
        
        // Track MouseArea event handlers
        if widget.widget_type == WidgetType::MouseArea {
            if props.mousearea_on_scroll {
                self.uses_mouse_scroll_delta = true;
            }
            if props.mousearea_on_move {
                self.uses_point = true;
            }
            if props.mousearea_interaction.is_some() {
                self.uses_mouse_interaction = true;
            }
        }
        
        // Recursively scan children
        for child in &widget.children {
            self.scan_widget(child);
        }
    }
}
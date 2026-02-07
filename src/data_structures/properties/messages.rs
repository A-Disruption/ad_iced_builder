use iced::{Alignment, Length, Color, Padding, Theme, };
use iced::widget::{combo_box, markdown, text, text_editor, qr_code};
use uuid::Uuid;

use crate::data_structures::types::type_implementations::*;
use crate::enum_builder::TypeSystem;
use super::properties::*;


/// Unique identifier for widgets in the hierarchy
#[derive(Debug, Clone)]
pub enum PropertyChange {
    // Common properties
    WidgetName(String),
    Width(Length),
    Height(Length),
    MaxWidth(Option<f32>),
    MaxHeight(Option<f32>),
    Clip(bool),
    WidgetId(Option<String>),
    CustomStyle(Option<String>),

    // Draft Properties
    DraftFixedWidth(String),
    DraftFixedHeight(String),
    DraftFillPortionWidth(String),
    DraftFillPortionHeight(String),
    DraftTextColor(String),

    // Padding mode and convenience setters
    PaddingMode(PaddingMode),
    PaddingUniform(f32),       // Sets all sides to same value
    PaddingVertical(f32),      // Sets top and bottom
    PaddingHorizontal(f32),    // Sets left and right
    PaddingTop(f32),
    PaddingRight(f32),
    PaddingBottom(f32),
    PaddingLeft(f32),
    
    // Container properties
    AlignX(ContainerAlignX),
    AlignY(ContainerAlignY),
    BorderWidth(f32),
    BorderRadius(f32),
    BorderColor(Color),
    BackgroundColor(Color),
    HasShadow(bool),
    ShadowOffsetX(f32),
    ShadowOffsetY(f32),
    ShadowBlur(f32),
    ShadowColor(Color),
    ContainerSizingMode(ContainerSizingMode),
    ContainerCenterLength(Length),

    // Wrapping Row
    IsWrappingRow(bool),
    WrappingVerticalSpacing(f32),
    WrappingSpacingMatchToggle(bool),
    WrappingAlignX(ContainerAlignX),
    
    // Layout properties
    Spacing(f32),
    AlignItems(Alignment),
    
    // Text properties
    TextContent(String),
    TextSize(f32),
    TextColor(Color),
    Font(FontType),
    TextLineHeight(text::LineHeight),
    TextWrap(TextWrapping),
    TextShaping(TextShaping),
    TextAlignX(AlignText),
    TextAlignY(AlignmentYOption),
    
    // Button properties
//    ButtonStyle(ButtonStyleType),
    ButtonPressHandler(OnHandler),

    // TextInput properties
    TextInputValue(String),
    TextInputPlaceholder(String),
    TextInputSize(f32),
    TextInputPadding(f32),
    IsSecure(bool),
    TextInputOnSubmit(bool),
    TextInputOnPaste(bool),
    TextInputFont(FontType),
    TextInputLineHeight(text::LineHeight),
    TextInputAlignment(ContainerAlignX),
    
    // Checkbox properties
    CheckboxChecked(bool),
    CheckboxLabel(String),
    CheckboxSize(f32),
    CheckboxSpacing(f32),
    
    // Radio properties
    RadioSelectedIndex(usize),
    RadioOptions(Vec<String>),
    RadioLabel(String),
    RadioSize(f32),
    RadioSpacing(f32),

    // Slider properties
    SliderValue(f32),
    SliderMin(f32),
    SliderMax(f32),
    SliderStep(f32),
    SliderHeight(f32),
    SliderWidth(f32),

    // Progress properties
    ProgressValue(f32),
    ProgressMin(f32),
    ProgressMax(f32),
    ProgressLength(Length),     // main axis (width if horizontal, height if vertical)
    ProgressGirth(f32),      // thickness (height if horizontal, width if vertical)
    ProgressVertical(bool),     // orientation
    
    // Toggler properties
    TogglerActive(bool),
    TogglerLabel(String),
    TogglerSize(f32),
    TogglerSpacing(f32),
    
    // PickList properties
    PickListSelected(Option<String>),
    PickListPlaceholder(String),
    PickListOptions(Vec<String>),

    // Rule properties
    Orientation(Orientation),
    RuleThickness(f32),

    // Scrollable properties
    ScrollableDirection(iced::widget::scrollable::Direction),
    ScrollableAnchorX(iced::widget::scrollable::Anchor),
    ScrollableAnchorY(iced::widget::scrollable::Anchor),

    // Image
    ImagePath(String),
    ImageFit(ContentFitChoice),
    // Svg
    SvgPath(String),
    SvgFit(ContentFitChoice),
    // Tooltip
    TooltipText(String),
    TooltipPosition(TooltipPosition),
    TooltipGap(f32),

    // ComboBox
    ComboBoxPlaceholder(String),
    ComboBoxSelected(Option<String>),
    ComboBoxState(Vec<String>),
    ComboBoxUseOnInput(bool),
    ComboBoxUseOnOptionHovered(bool),
    ComboBoxUseOnOpen(bool),
    ComboBoxUseOnClose(bool),
    ComboBoxSize(f32),
    ComboBoxPadding(f32),
    ComboBoxEnumId(Option<Uuid>),
    
    // Markdown
    MarkdownContent(text_editor::Action),
    MarkdownTextSize(f32),
    
    // QR Code
    QRCodeData(String),
    QRCodeCellSize(f32),
    
    // Themer
    ThemerTheme(Option<Theme>),

    // Mouse events
    MouseAreaOnPress(bool),
    MouseAreaOnRelease(bool),
    MouseAreaOnDoubleClick(bool),
    MouseAreaOnRightPress(bool),
    MouseAreaOnRightRelease(bool),
    MouseAreaOnMiddlePress(bool),
    MouseAreaOnMiddleRelease(bool),
    MouseAreaOnScroll(bool),
    MouseAreaOnEnter(bool),
    MouseAreaOnMove(bool),
    MouseAreaOnExit(bool),
    MouseAreaInteraction(Option<MouseInteraction>),

    // View references
    ViewReferenceId(Option<Uuid>, String),

    //Do Nothing
    Noop
}

// Helper function to apply property changes
pub fn apply_property_change(properties: &mut Properties, change: PropertyChange, type_system: &TypeSystem) {
    match change {
        PropertyChange::Width(value) => {
            properties.width = value;
            properties.draft_fixed_width.clear();
            properties.draft_fill_portion_width.clear();
        }
        
        PropertyChange::Height(value) => {
            properties.height = value;
            properties.draft_fixed_height.clear();
            properties.draft_fill_portion_height.clear();
        }
        PropertyChange::AlignItems(value) => properties.align_items = value,

        PropertyChange::MaxWidth(v) => properties.max_width = v,
        PropertyChange::MaxHeight(v) => properties.max_height = v,
        PropertyChange::Clip(v) => properties.clip = v,
        PropertyChange::WidgetId(v) => properties.widget_id = v,
        PropertyChange::CustomStyle(style_name) => properties.custom_style_name = style_name,
        PropertyChange::IsWrappingRow(v) => properties.is_wrapping_row = v,
        PropertyChange::WrappingVerticalSpacing(v) => properties.wrapping_vertical_spacing = v,
        PropertyChange::WrappingSpacingMatchToggle(toggle) => {
            properties.match_horizontal_spacing = toggle;
        },
        PropertyChange::WrappingAlignX(v) => properties.wrapping_align_x = v,

        PropertyChange::DraftFixedWidth(text) => {
            properties.draft_fixed_width = text.clone();
            if let Ok(px) = text.trim().parse::<f32>() {
                if px >= 0.0 {
                    properties.width = Length::Fixed(px);
                }
            }
        }
        PropertyChange::DraftFixedHeight(text) => {
            properties.draft_fixed_height = text.clone();
            if let Ok(px) = text.trim().parse::<f32>() {
                if px >= 0.0 {
                    properties.height = Length::Fixed(px);
                }
            }
        }
        PropertyChange::DraftFillPortionWidth(text) => {
            properties.draft_fill_portion_width = text.clone();
            if let Ok(p) = text.trim().parse::<u16>() {
                if p >= 1 {
                    properties.width = Length::FillPortion(p);
                }
            }
        }
        PropertyChange::DraftFillPortionHeight(text) => {
            properties.draft_fill_portion_height = text.clone();
            if let Ok(p) = text.trim().parse::<u16>() {
                if p >= 1 {
                    properties.height = Length::FillPortion(p);
                }
            }
        }
        PropertyChange::DraftTextColor(text) => {
            properties.draft_text_color = text.clone();
            match parse_color_hex(&text) {
                Some(color) => { properties.text_color = color;}
                None => {}
            }
        }
        PropertyChange::PaddingMode(mode) => {
            let current = properties.padding;
            properties.padding_mode = mode;

            match mode {
                PaddingMode::Uniform => {
                    properties.padding = Padding::new(current.top);
                }
                PaddingMode::Symmetric => {
                    properties.padding = Padding {
                        top: current.top,
                        right: current.left,
                        bottom: current.top,
                        left: current.left,
                    };
                }
                PaddingMode::Individual => {}
            }
        }
        
        PropertyChange::PaddingUniform(value) => {
            properties.padding_mode = PaddingMode::Uniform;
            properties.padding = Padding::new(value);
        }
        
        PropertyChange::PaddingVertical(value) => {
            properties.padding_mode = PaddingMode::Symmetric;
            properties.padding.top = value;
            properties.padding.bottom = value;
        }
        
        PropertyChange::PaddingHorizontal(value) => {
            properties.padding_mode = PaddingMode::Symmetric;
            properties.padding.left = value;
            properties.padding.right = value;
        }

        PropertyChange::PaddingRight(value) => {
            properties.padding.right = value;
            match properties.padding_mode {
                PaddingMode::Uniform => {
                    properties.padding = Padding::new(value);
                }
                PaddingMode::Symmetric => {
                    properties.padding.left = value;
                }
                PaddingMode::Individual => {}
            }
        }
        
        PropertyChange::PaddingBottom(value) => {
            properties.padding.bottom = value;
            match properties.padding_mode {
                PaddingMode::Uniform => {
                    properties.padding = Padding::new(value);
                }
                PaddingMode::Symmetric => {
                    properties.padding.top = value;
                }
                PaddingMode::Individual => {}
            }
        }
        
        PropertyChange::PaddingLeft(value) => {
            properties.padding.left = value;
            match properties.padding_mode {
                PaddingMode::Uniform => {
                    properties.padding = Padding::new(value);
                }
                PaddingMode::Symmetric => {
                    properties.padding.right = value;
                }
                PaddingMode::Individual => {}
            }
        }

        PropertyChange::PaddingTop(value) => {
            properties.padding.top = value;
            match properties.padding_mode {
                PaddingMode::Uniform => {
                    properties.padding = Padding::new(value);
                }
                PaddingMode::Symmetric => {
                    properties.padding.bottom = value;
                }
                PaddingMode::Individual => {}
            }
        }

        PropertyChange::Spacing(value)          => properties.spacing = value,

        PropertyChange::WidgetName(value) => properties.widget_name = value,

        PropertyChange::BorderWidth(value)  => properties.border_width = value,
        PropertyChange::BorderRadius(value) => properties.border_radius = value,
        PropertyChange::BorderColor(value)  => properties.border_color = value,
        PropertyChange::ContainerSizingMode(v) => {
            properties.container_sizing_mode = v;
            // When switching to center modes, copy current width/height as starting point
            match v {
                ContainerSizingMode::CenterX | ContainerSizingMode::Center => {
                    properties.container_center_length = properties.width;
                }
                ContainerSizingMode::CenterY => {
                    properties.container_center_length = properties.height;
                }
                _ => {}
            }
        }
        PropertyChange::ContainerCenterLength(v) => properties.container_center_length = v,
        PropertyChange::AlignX(v) => properties.align_x = v,
        PropertyChange::AlignY(v) => properties.align_y = v,

        PropertyChange::BackgroundColor(value) => properties.background_color = value,

        PropertyChange::TextContent(value)          => properties.text_content = value,
        PropertyChange::TextSize(value)             => properties.text_size = value,
        PropertyChange::TextColor(value)            => properties.text_color = value,
        PropertyChange::Font(value)                 => properties.font = value,        
        PropertyChange::TextLineHeight(line_height) => properties.line_height = line_height,
        PropertyChange::TextWrap(wrapping)          => properties.wrap = wrapping.to_wrap(),
        PropertyChange::TextShaping(shapping)       => properties.shaping = shapping.to_shaping(),
        PropertyChange::TextAlignX(alignment)       => properties.text_align_x = alignment.to_alignment().into(),
        PropertyChange::TextAlignY(alignment)       => properties.text_align_y = alignment.to_alignment(),

//        PropertyChange::ButtonStyle(value) => properties.button_style = value,
        PropertyChange::ButtonPressHandler(handler) => {
            // Reset all to false first
            properties.button_on_press_enabled = false;
            properties.button_on_press_with_enabled = false;
            properties.button_on_press_maybe_enabled = false;
            
            // Set the selected one
            match handler {
                OnHandler::None => {}, // All stay false
                OnHandler::OnAction => properties.button_on_press_enabled = true,
                OnHandler::OnActionWith => properties.button_on_press_with_enabled = true,
                OnHandler::OnActionMaybe => properties.button_on_press_maybe_enabled = true,
            }
        },
        
        // TextInput properties
        PropertyChange::TextInputValue(value)       => properties.text_input_value = value,
        PropertyChange::TextInputPlaceholder(value) => properties.text_input_placeholder = value,
        PropertyChange::TextInputSize(value)        => properties.text_input_size = value,
        PropertyChange::TextInputPadding(value)     => properties.text_input_padding = value,
        PropertyChange::IsSecure(value)             => properties.is_secure = value,
        PropertyChange::TextInputOnSubmit(b) => properties.text_input_on_submit = b,
        PropertyChange::TextInputOnPaste(b) => properties.text_input_on_paste = b,
        PropertyChange::TextInputFont(font) => properties.text_input_font = font,
        PropertyChange::TextInputLineHeight(line_height) => properties.text_input_line_height = line_height,
        PropertyChange::TextInputAlignment(align_x) => properties.text_input_alignment = align_x,
        
        // Checkbox properties
        PropertyChange::CheckboxChecked(value)  => properties.checkbox_checked = value,
        PropertyChange::CheckboxLabel(value)    => properties.checkbox_label = value,
        PropertyChange::CheckboxSize(value)     => properties.checkbox_size = value,
        PropertyChange::CheckboxSpacing(value)  => properties.checkbox_spacing = value,

        // Slider properties
        PropertyChange::SliderValue(value)  => properties.slider_value = value,
        PropertyChange::SliderMin(value)    => properties.slider_min = value,
        PropertyChange::SliderMax(value)    => properties.slider_max = value,
        PropertyChange::SliderStep(value)   => properties.slider_step = value,
        PropertyChange::SliderHeight(value) => properties.slider_height = value,
        PropertyChange::SliderWidth(value)  => properties.slider_width = value,
        
        // Radio properties
        PropertyChange::RadioSelectedIndex(value) => {
            if value < properties.radio_options.len() {
                properties.radio_selected_index = value;
            }
        },
        PropertyChange::RadioOptions(value) => {
            properties.radio_options = value;
            // Reset selection if it's out of bounds
            if properties.radio_selected_index >= properties.radio_options.len() {
                properties.radio_selected_index = 0;
            }
        },
        PropertyChange::RadioLabel(value)   => properties.radio_label = value,
        PropertyChange::RadioSize(value)    => properties.radio_size = value,
        PropertyChange::RadioSpacing(value) => properties.radio_spacing = value,

        // Progress properties
        PropertyChange::ProgressValue(v) => {
            let lo = properties.progress_min.min(properties.progress_max);
            let hi = properties.progress_min.max(properties.progress_max);
            properties.progress_value = v.clamp(lo, hi);
        }
        PropertyChange::ProgressMin(v) => {
            properties.progress_min = v;
            let lo = properties.progress_min.min(properties.progress_max);
            let hi = properties.progress_min.max(properties.progress_max);
            properties.progress_value = properties.progress_value.clamp(lo, hi);
        }
        PropertyChange::ProgressMax(v) => {
            properties.progress_max = v;
            let lo = properties.progress_min.min(properties.progress_max);
            let hi = properties.progress_min.max(properties.progress_max);
            properties.progress_value = properties.progress_value.clamp(lo, hi);
        }
        PropertyChange::ProgressLength(len) => properties.progress_length = len,
        PropertyChange::ProgressGirth(len)  => properties.progress_girth = len,
        PropertyChange::ProgressVertical(v) => properties.progress_vertical = v,
        
        // Toggler properties
        PropertyChange::TogglerActive(value)    => properties.toggler_active = value,
        PropertyChange::TogglerLabel(value)     => properties.toggler_label = value,
        PropertyChange::TogglerSize(value)      => properties.toggler_size = value,
        PropertyChange::TogglerSpacing(value)   => properties.toggler_spacing = value,
        
        // PickList properties
        PropertyChange::PickListSelected(value)     => properties.picklist_selected = value,
        PropertyChange::PickListPlaceholder(value)  => properties.picklist_placeholder = value,
        PropertyChange::PickListOptions(value)      => properties.picklist_options = value,

        // Rule properties
        PropertyChange::RuleThickness(v)   => properties.rule_thickness  = v,

        //Rule + Space properties
        PropertyChange::Orientation(v) => properties.orientation = v,

        // Scrollable properties
        PropertyChange::ScrollableDirection(value)  => properties.scroll_dir = value,
        PropertyChange::ScrollableAnchorX(value)    => properties.anchor_x = value,
        PropertyChange::ScrollableAnchorY(value)    => properties.anchor_y = value,

        // Image properties
        PropertyChange::ImagePath(v)        => properties.image_path = v,
        PropertyChange::ImageFit(v)         => properties.image_fit = v,

        // Svg properties
        PropertyChange::SvgPath(v)          => properties.svg_path = v,
        PropertyChange::SvgFit(v)           => properties.svg_fit = v,

        // Tooltip properties
        PropertyChange::TooltipText(v)      => properties.tooltip_text = v,
        PropertyChange::TooltipPosition(v)  => properties.tooltip_position = v,
        PropertyChange::TooltipGap(v)       => properties.tooltip_gap = v,

        PropertyChange::ComboBoxSelected(v) => properties.combobox_selected = v,
        PropertyChange::ComboBoxPlaceholder(v) => properties.combobox_placeholder = v,
        PropertyChange::ComboBoxState(v) => {
            properties.combobox_options = v.clone();
            // Recreate state with new options
            properties.combobox_state = combo_box::State::new(v);
        }
        PropertyChange::ComboBoxUseOnInput(v) => properties.combobox_use_on_input = v,
        PropertyChange::ComboBoxUseOnOptionHovered(v) => properties.combobox_use_on_option_hovered = v,
        PropertyChange::ComboBoxUseOnOpen(v) => properties.combobox_use_on_open = v,
        PropertyChange::ComboBoxUseOnClose(v) => properties.combobox_use_on_close = v,
        PropertyChange::ComboBoxSize(v) => properties.combobox_size = v,
        PropertyChange::ComboBoxPadding(v) => properties.combobox_padding = v,
        PropertyChange::ComboBoxEnumId(id) => {
            //Set referenced_enum Id
            properties.referenced_enum = id;

            //Update combo_box state from Enum
            let state = if let Some(ref enum_id) = properties.referenced_enum {
                if let Some(enum_def) = type_system.get_enum(enum_id.clone()) {
                    let variants: Vec<String> = enum_def.variants.iter()
                        .map(|v| v.name.clone())
                        .collect();

                    combo_box::State::new(variants)                  
                } else { combo_box::State::new(vec![])}
            } else { combo_box::State::new(vec![])};

            properties.combobox_state = state;
        }
        
        PropertyChange::MarkdownContent(action) => {
                let is_edit = action.is_edit();

                properties.markdown_source.perform(action);

                if is_edit {
                    properties.markdown_content = markdown::Content::parse(&properties.markdown_source.text()).items().to_vec();
                }
        },
        PropertyChange::MarkdownTextSize(v) => properties.markdown_text_size = v,
        
        PropertyChange::QRCodeData(url) => {
            properties.qrcode_link = url;
            properties.qrcode_data = Some(qr_code::Data::new(&properties.qrcode_link).unwrap(),)
        },
        PropertyChange::QRCodeCellSize(v) => properties.qrcode_cell_size = v,
        
        PropertyChange::ThemerTheme(v) => properties.themer_theme = v,

        PropertyChange::Noop => {},

        PropertyChange::MouseAreaOnPress(b) => properties.mousearea_on_press = b,
        PropertyChange::MouseAreaOnRelease(b) => properties.mousearea_on_release = b,
        PropertyChange::MouseAreaOnDoubleClick(b) => properties.mousearea_on_double_click = b,
        PropertyChange::MouseAreaOnRightPress(b) => properties.mousearea_on_right_press = b,
        PropertyChange::MouseAreaOnRightRelease(b) => properties.mousearea_on_right_release = b,
        PropertyChange::MouseAreaOnMiddlePress(b) => properties.mousearea_on_middle_press = b,
        PropertyChange::MouseAreaOnMiddleRelease(b) => properties.mousearea_on_middle_release = b,
        PropertyChange::MouseAreaOnScroll(b) => properties.mousearea_on_scroll = b,
        PropertyChange::MouseAreaOnEnter(b) => properties.mousearea_on_enter = b,
        PropertyChange::MouseAreaOnMove(b) => properties.mousearea_on_move = b,
        PropertyChange::MouseAreaOnExit(b) => properties.mousearea_on_exit = b,
        PropertyChange::MouseAreaInteraction(interaction) => properties.mousearea_interaction = interaction,

        PropertyChange::ViewReferenceId(view_id, name) => { 
            properties.referenced_view_id = view_id;
            properties.widget_name = name;
        }
        
        _ => {} // Placeholder for properties not implemented
    }
}

fn parse_color_hex(s: &str) -> Option<iced::Color> {
    let t = s.trim().trim_start_matches('#');
    let hex = |i: usize| u8::from_str_radix(&t[i..i+2], 16).ok();
    match t.len() {
        6 => {
            if let (Some(r), Some(g), Some(b)) = (hex(0), hex(2), hex(4)) {
                return Some(Color::from_rgba8(r, g, b, 255.0));
            } else { None }
        }
        8 => {
            if let (Some(r), Some(g), Some(b), Some(a)) = (hex(0), hex(2), hex(4), hex(6)) {
                return Some(Color::from_rgba8(r, g, b, a.into()));
            } else { None }
        }
        _ => None,
    }
}
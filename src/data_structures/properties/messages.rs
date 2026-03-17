use iced::widget::{combo_box, markdown, qr_code, text, text_editor};
use iced::{Alignment, Color, Length, Padding, Theme};
use uuid::Uuid;

use super::properties::*;
use crate::action_system::StateFieldRef;
use crate::data_structures::types::type_implementations::*;
use crate::enum_builder::TypeSystem;

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
    MenuStyle(Option<String>),
    ActiveStyle(Option<String>),
    StyleConditionField(Option<String>),
    StyleConditionValue(Option<String>),

    // Draft Properties
    DraftFixedWidth(String),
    DraftFixedHeight(String),
    DraftFillPortionWidth(String),
    DraftFillPortionHeight(String),
    DraftTextColor(String),

    // Padding mode and convenience setters
    PaddingMode(PaddingMode),
    PaddingUniform(f32),    // Sets all sides to same value
    PaddingVertical(f32),   // Sets top and bottom
    PaddingHorizontal(f32), // Sets left and right
    PaddingTop(f32),
    PaddingRight(f32),
    PaddingBottom(f32),
    PaddingLeft(f32),

    // Container properties
    AlignX(ContainerAlignX),
    AlignY(ContainerAlignY),
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
    Font(FontType),
    TextLineHeight(text::LineHeight),
    TextWrap(TextWrapping),
    TextShaping(TextShaping),
    TextAlignX(AlignText),
    TextAlignY(AlignmentYOption),

    // Button properties
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
    TextInputAlignment(ContainerAlignX),
    TextInputIconEnabled(bool),
    TextInputIconSelected(String, u32),
    TextInputIconSize(f32),
    TextInputIconSpacing(f32),
    TextInputIconSide(TextInputIconSide),
    TextInputIconPickerFilter(String),

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
    ProgressLength(Length), // main axis (width if horizontal, height if vertical)
    ProgressGirth(f32),     // thickness (height if horizontal, width if vertical)
    ProgressVertical(bool), // orientation

    // Toggler properties
    TogglerActive(bool),
    TogglerLabel(String),
    TogglerSize(f32),
    TogglerSpacing(f32),

    // Collapsible properties
    CollapsibleTitle(String),
    CollapsibleHeaderHeight(f32),
    CollapsibleHeaderClickable(bool),
    CollapsibleExpanded(bool),

    // Generic Overlay properties
    GenericOverlayTitle(String),
    GenericOverlayOverlayWidthChoice(GenericOverlayLengthChoice),
    GenericOverlayOverlayHeightChoice(GenericOverlayLengthChoice),
    DraftGenericOverlayOverlayWidthFixed(String),
    DraftGenericOverlayOverlayWidthFillPortion(String),
    DraftGenericOverlayOverlayWidthDynamic(String),
    DraftGenericOverlayOverlayHeightFixed(String),
    DraftGenericOverlayOverlayHeightFillPortion(String),
    DraftGenericOverlayOverlayHeightDynamic(String),
    GenericOverlayOverlayPadding(f32),
    GenericOverlayOverlayRadius(f32),
    GenericOverlayOverlayStyle(Option<String>),
    GenericOverlayOnHover(bool),
    GenericOverlayHoverPositionsOnClick(bool),
    GenericOverlayInitiallyOpen(bool),
    GenericOverlayHoverPosition(GenericOverlayPosition),
    GenericOverlayHoverGap(f32),
    GenericOverlayHoverAlignment(ContainerAlignX),
    GenericOverlayHoverMode(GenericOverlayPositionMode),
    GenericOverlayHoverSnap(bool),
    GenericOverlayCloseOnClickOutside(bool),
    GenericOverlayOpaque(bool),
    GenericOverlayOpaqueAlpha(f32),
    GenericOverlayHideHeader(bool),
    GenericOverlayHideCloseButton(bool),
    GenericOverlayBlockDragging(bool),
    GenericOverlayResizable(GenericOverlayResizeMode),
    GenericOverlayResetOnClose(bool),
    GenericOverlayAnimate(bool),
    GenericOverlayAnimationPreset(GenericOverlayAnimationPreset),
    GenericOverlaySafeTriangle(bool),

    // Date Picker properties
    DatePickerSelectionMode(DatePickerSelectionMode),
    DatePickerShowTime(bool),
    DatePickerInitiallyOpen(bool),
    DatePickerInitialSingleDate(String),
    DatePickerInitialRangeStart(String),
    DatePickerInitialRangeEnd(String),
    DatePickerInitialHour(u32),
    DatePickerInitialMinute(u32),

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

    // ComboBox
    ComboBoxPlaceholder(String),
    ComboBoxSelected(Option<String>),
    ComboBoxState(Vec<String>),
    ComboBoxUseOnInput(bool),
    ComboBoxUseOnOptionHovered(bool),
    ComboBoxUseOnOpen(bool),
    ComboBoxUseOnClose(bool),
    ComboBoxSize(f32),
    ComboBoxEnumId(Option<Uuid>),
    ComboBoxIconEnabled(bool),
    ComboBoxIconSelected(String, u32),
    ComboBoxIconSize(f32),
    ComboBoxIconSpacing(f32),
    ComboBoxIconSide(TextInputIconSide),
    ComboBoxIconPickerFilter(String),

    // Markdown
    MarkdownContent(text_editor::Action),
    MarkdownTextSize(f32),

    // QR Code
    QRCodeData(String),
    QRCodeCellSize(f32),

    // Table
    TableReferencedStruct(Option<Uuid>),
    TablePaddingX(f32),
    TablePaddingY(f32),
    TableSeparatorX(f32),
    TableSeparatorY(f32),
    TableBoldHeaders(bool),

    // Pin
    PinX(f32),
    PinY(f32),

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
    AddExtraViewRef,
    SetExtraViewRef(usize, Uuid),
    RemoveExtraViewRef(usize),

    // Icon (Lucide) properties
    IconName(String),
    IconCodepoint(u32),
    IconSize(f32),
    IconSelected(String, u32), // Sets both name and codepoint atomically
    IconPickerFilter(String),

    // Grid properties
    GridColumns(usize),
    GridSpacing(f32),
    GridFixedWidth(Option<f32>),
    GridUseFluid(bool),
    GridFluidMaxWidth(f32),

    // Action system
    StateFieldOverride(Option<StateFieldRef>),

    //Do Nothing
    Noop,
}

// Helper function to apply property changes
pub fn apply_property_change(
    properties: &mut Properties,
    change: PropertyChange,
    type_system: &TypeSystem,
) {
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
        PropertyChange::MenuStyle(style_name) => properties.menu_style_name = style_name,
        PropertyChange::ActiveStyle(v) => properties.active_style_name = v,
        PropertyChange::StyleConditionField(v) => properties.style_condition_field = v,
        PropertyChange::StyleConditionValue(v) => properties.style_condition_value = v,
        PropertyChange::IsWrappingRow(v) => properties.is_wrapping_row = v,
        PropertyChange::WrappingVerticalSpacing(v) => properties.wrapping_vertical_spacing = v,
        PropertyChange::WrappingSpacingMatchToggle(toggle) => {
            properties.match_horizontal_spacing = toggle;
        }
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
                Some(color) => {
                    properties.text_color = color;
                }
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

        PropertyChange::Spacing(value) => properties.spacing = value,

        PropertyChange::WidgetName(value) => properties.widget_name = value,

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

        PropertyChange::TextContent(value) => properties.text_content = value,
        PropertyChange::TextSize(value) => properties.text_size = value,
        PropertyChange::Font(value) => properties.font = value,
        PropertyChange::TextLineHeight(line_height) => properties.line_height = line_height,
        PropertyChange::TextWrap(wrapping) => properties.wrap = wrapping.to_wrap(),
        PropertyChange::TextShaping(shapping) => properties.shaping = shapping.to_shaping(),
        PropertyChange::TextAlignX(alignment) => {
            properties.text_align_x = alignment.to_alignment().into()
        }
        PropertyChange::TextAlignY(alignment) => properties.text_align_y = alignment.to_alignment(),

        PropertyChange::ButtonPressHandler(handler) => {
            // Reset all to false first
            properties.button_on_press_enabled = false;
            properties.button_on_press_with_enabled = false;
            properties.button_on_press_maybe_enabled = false;

            // Set the selected one
            match handler {
                OnHandler::None => {} // All stay false
                OnHandler::OnAction => properties.button_on_press_enabled = true,
                OnHandler::OnActionWith => properties.button_on_press_with_enabled = true,
                OnHandler::OnActionMaybe => properties.button_on_press_maybe_enabled = true,
            }
        }

        // TextInput properties
        PropertyChange::TextInputValue(value) => properties.text_input_value = value,
        PropertyChange::TextInputPlaceholder(value) => properties.text_input_placeholder = value,
        PropertyChange::TextInputSize(value) => properties.text_input_size = value,
        PropertyChange::TextInputPadding(value) => properties.text_input_padding = value,
        PropertyChange::IsSecure(value) => properties.is_secure = value,
        PropertyChange::TextInputOnSubmit(b) => properties.text_input_on_submit = b,
        PropertyChange::TextInputOnPaste(b) => properties.text_input_on_paste = b,
        PropertyChange::TextInputFont(font) => properties.text_input_font = font,
        PropertyChange::TextInputAlignment(align_x) => properties.text_input_alignment = align_x,
        PropertyChange::TextInputIconEnabled(v) => properties.text_input_icon_enabled = v,
        PropertyChange::TextInputIconSelected(name, cp) => {
            properties.text_input_icon_name = name;
            properties.text_input_icon_codepoint = cp;
        }
        PropertyChange::TextInputIconSize(v) => properties.text_input_icon_size = v,
        PropertyChange::TextInputIconSpacing(v) => properties.text_input_icon_spacing = v,
        PropertyChange::TextInputIconSide(v) => properties.text_input_icon_side = v,
        PropertyChange::TextInputIconPickerFilter(v) => {
            properties.text_input_icon_picker_filter = v
        }

        // Checkbox properties
        PropertyChange::CheckboxChecked(value) => properties.checkbox_checked = value,
        PropertyChange::CheckboxLabel(value) => properties.checkbox_label = value,
        PropertyChange::CheckboxSize(value) => properties.checkbox_size = value,
        PropertyChange::CheckboxSpacing(value) => properties.checkbox_spacing = value,

        // Slider properties
        PropertyChange::SliderValue(value) => properties.slider_value = value,
        PropertyChange::SliderMin(value) => properties.slider_min = value,
        PropertyChange::SliderMax(value) => properties.slider_max = value,
        PropertyChange::SliderStep(value) => properties.slider_step = value,
        PropertyChange::SliderHeight(value) => properties.slider_height = value,
        PropertyChange::SliderWidth(value) => properties.slider_width = value,

        // Radio properties
        PropertyChange::RadioSelectedIndex(value) => {
            if value < properties.radio_options.len() {
                properties.radio_selected_index = value;
            }
        }
        PropertyChange::RadioOptions(value) => {
            properties.radio_options = value;
            // Reset selection if it's out of bounds
            if properties.radio_selected_index >= properties.radio_options.len() {
                properties.radio_selected_index = 0;
            }
        }
        PropertyChange::RadioLabel(value) => properties.radio_label = value,
        PropertyChange::RadioSize(value) => properties.radio_size = value,
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
        PropertyChange::ProgressGirth(len) => properties.progress_girth = len,
        PropertyChange::ProgressVertical(v) => properties.progress_vertical = v,

        // Toggler properties
        PropertyChange::TogglerActive(value) => properties.toggler_active = value,
        PropertyChange::TogglerLabel(value) => properties.toggler_label = value,
        PropertyChange::TogglerSize(value) => properties.toggler_size = value,
        PropertyChange::TogglerSpacing(value) => properties.toggler_spacing = value,

        // Collapsible properties
        PropertyChange::CollapsibleTitle(value) => properties.collapsible_title = value,
        PropertyChange::CollapsibleHeaderHeight(value) => {
            properties.collapsible_header_height = value.max(0.0)
        }
        PropertyChange::CollapsibleHeaderClickable(value) => {
            properties.collapsible_header_clickable = value
        }
        PropertyChange::CollapsibleExpanded(value) => properties.collapsible_expanded = value,

        // Generic Overlay properties
        PropertyChange::GenericOverlayTitle(value) => properties.generic_overlay_title = value,
        PropertyChange::GenericOverlayOverlayWidthChoice(choice) => match choice {
            GenericOverlayLengthChoice::Fill => {
                properties.generic_overlay_overlay_width_dynamic = false;
                properties.generic_overlay_overlay_width = Length::Fill;
            }
            GenericOverlayLengthChoice::FillPortion => {
                properties.generic_overlay_overlay_width_dynamic = false;
                if !matches!(
                    properties.generic_overlay_overlay_width,
                    Length::FillPortion(_)
                ) {
                    properties.generic_overlay_overlay_width = Length::FillPortion(1);
                }
            }
            GenericOverlayLengthChoice::Shrink => {
                properties.generic_overlay_overlay_width_dynamic = false;
                properties.generic_overlay_overlay_width = Length::Shrink;
            }
            GenericOverlayLengthChoice::Fixed => {
                properties.generic_overlay_overlay_width_dynamic = false;
                if !matches!(properties.generic_overlay_overlay_width, Length::Fixed(_)) {
                    properties.generic_overlay_overlay_width = Length::Fixed(120.0);
                }
            }
            GenericOverlayLengthChoice::Dynamic => {
                properties.generic_overlay_overlay_width_dynamic = true;
                if properties.generic_overlay_overlay_width_dynamic_factor <= 0.0 {
                    properties.generic_overlay_overlay_width_dynamic_factor = 1.0;
                }
            }
        },
        PropertyChange::GenericOverlayOverlayHeightChoice(choice) => match choice {
            GenericOverlayLengthChoice::Fill => {
                properties.generic_overlay_overlay_height_dynamic = false;
                properties.generic_overlay_overlay_height = Length::Fill;
            }
            GenericOverlayLengthChoice::FillPortion => {
                properties.generic_overlay_overlay_height_dynamic = false;
                if !matches!(
                    properties.generic_overlay_overlay_height,
                    Length::FillPortion(_)
                ) {
                    properties.generic_overlay_overlay_height = Length::FillPortion(1);
                }
            }
            GenericOverlayLengthChoice::Shrink => {
                properties.generic_overlay_overlay_height_dynamic = false;
                properties.generic_overlay_overlay_height = Length::Shrink;
            }
            GenericOverlayLengthChoice::Fixed => {
                properties.generic_overlay_overlay_height_dynamic = false;
                if !matches!(properties.generic_overlay_overlay_height, Length::Fixed(_)) {
                    properties.generic_overlay_overlay_height = Length::Fixed(120.0);
                }
            }
            GenericOverlayLengthChoice::Dynamic => {
                properties.generic_overlay_overlay_height_dynamic = true;
                if properties.generic_overlay_overlay_height_dynamic_factor <= 0.0 {
                    properties.generic_overlay_overlay_height_dynamic_factor = 1.0;
                }
            }
        },
        PropertyChange::DraftGenericOverlayOverlayWidthFixed(text) => {
            properties.draft_generic_overlay_overlay_width_fixed = text.clone();
            if let Ok(px) = text.trim().parse::<f32>() {
                if px >= 0.0 {
                    properties.generic_overlay_overlay_width = Length::Fixed(px);
                }
            }
        }
        PropertyChange::DraftGenericOverlayOverlayWidthFillPortion(text) => {
            properties.draft_generic_overlay_overlay_width_fill_portion = text.clone();
            if let Ok(portion) = text.trim().parse::<u16>() {
                if portion >= 1 {
                    properties.generic_overlay_overlay_width = Length::FillPortion(portion);
                }
            }
        }
        PropertyChange::DraftGenericOverlayOverlayWidthDynamic(text) => {
            properties.draft_generic_overlay_overlay_width_dynamic = text.clone();
            if let Ok(factor) = text.trim().parse::<f32>() {
                if factor > 0.0 {
                    properties.generic_overlay_overlay_width_dynamic_factor = factor;
                }
            }
        }
        PropertyChange::DraftGenericOverlayOverlayHeightFixed(text) => {
            properties.draft_generic_overlay_overlay_height_fixed = text.clone();
            if let Ok(px) = text.trim().parse::<f32>() {
                if px >= 0.0 {
                    properties.generic_overlay_overlay_height = Length::Fixed(px);
                }
            }
        }
        PropertyChange::DraftGenericOverlayOverlayHeightFillPortion(text) => {
            properties.draft_generic_overlay_overlay_height_fill_portion = text.clone();
            if let Ok(portion) = text.trim().parse::<u16>() {
                if portion >= 1 {
                    properties.generic_overlay_overlay_height = Length::FillPortion(portion);
                }
            }
        }
        PropertyChange::DraftGenericOverlayOverlayHeightDynamic(text) => {
            properties.draft_generic_overlay_overlay_height_dynamic = text.clone();
            if let Ok(factor) = text.trim().parse::<f32>() {
                if factor > 0.0 {
                    properties.generic_overlay_overlay_height_dynamic_factor = factor;
                }
            }
        }
        PropertyChange::GenericOverlayOverlayPadding(value) => {
            properties.generic_overlay_overlay_padding = value.max(0.0)
        }
        PropertyChange::GenericOverlayOverlayRadius(value) => {
            properties.generic_overlay_overlay_radius = value.max(0.0)
        }
        PropertyChange::GenericOverlayOverlayStyle(value) => {
            properties.generic_overlay_overlay_style_name = value
        }
        PropertyChange::GenericOverlayOnHover(value) => properties.generic_overlay_on_hover = value,
        PropertyChange::GenericOverlayHoverPositionsOnClick(value) => {
            properties.generic_overlay_hover_positions_on_click = value
        }
        PropertyChange::GenericOverlayInitiallyOpen(value) => {
            properties.generic_overlay_initially_open = value
        }
        PropertyChange::GenericOverlayHoverPosition(value) => {
            properties.generic_overlay_hover_position = value
        }
        PropertyChange::GenericOverlayHoverGap(value) => {
            properties.generic_overlay_hover_gap = value.max(0.0)
        }
        PropertyChange::GenericOverlayHoverAlignment(value) => {
            properties.generic_overlay_hover_alignment = value
        }
        PropertyChange::GenericOverlayHoverMode(value) => {
            properties.generic_overlay_hover_mode = value
        }
        PropertyChange::GenericOverlayHoverSnap(value) => {
            properties.generic_overlay_hover_snap = value
        }
        PropertyChange::GenericOverlayCloseOnClickOutside(value) => {
            properties.generic_overlay_close_on_click_outside = value
        }
        PropertyChange::GenericOverlayOpaque(value) => properties.generic_overlay_opaque = value,
        PropertyChange::GenericOverlayOpaqueAlpha(value) => {
            properties.generic_overlay_opaque_alpha = value.clamp(0.1, 1.0)
        }
        PropertyChange::GenericOverlayHideHeader(value) => {
            properties.generic_overlay_hide_header = value
        }
        PropertyChange::GenericOverlayHideCloseButton(value) => {
            properties.generic_overlay_hide_close_button = value
        }
        PropertyChange::GenericOverlayBlockDragging(value) => {
            properties.generic_overlay_block_dragging = value
        }
        PropertyChange::GenericOverlayResizable(value) => {
            properties.generic_overlay_resizable = value
        }
        PropertyChange::GenericOverlayResetOnClose(value) => {
            properties.generic_overlay_reset_on_close = value
        }
        PropertyChange::GenericOverlayAnimate(value) => properties.generic_overlay_animate = value,
        PropertyChange::GenericOverlayAnimationPreset(value) => {
            properties.generic_overlay_animation_preset = value
        }
        PropertyChange::GenericOverlaySafeTriangle(value) => {
            properties.generic_overlay_safe_triangle = value
        }

        // Date Picker properties
        PropertyChange::DatePickerSelectionMode(value) => properties.date_picker_mode = value,
        PropertyChange::DatePickerShowTime(value) => properties.date_picker_show_time = value,
        PropertyChange::DatePickerInitiallyOpen(value) => {
            properties.date_picker_initially_open = value
        }
        PropertyChange::DatePickerInitialSingleDate(value) => {
            properties.date_picker_initial_single_date = value
        }
        PropertyChange::DatePickerInitialRangeStart(value) => {
            properties.date_picker_initial_range_start = value
        }
        PropertyChange::DatePickerInitialRangeEnd(value) => {
            properties.date_picker_initial_range_end = value
        }
        PropertyChange::DatePickerInitialHour(value) => {
            properties.date_picker_initial_hour = value.min(23)
        }
        PropertyChange::DatePickerInitialMinute(value) => {
            properties.date_picker_initial_minute = value.min(59)
        }

        // PickList properties
        PropertyChange::PickListSelected(value) => properties.picklist_selected = value,
        PropertyChange::PickListPlaceholder(value) => properties.picklist_placeholder = value,
        PropertyChange::PickListOptions(value) => properties.picklist_options = value,

        // Rule properties
        PropertyChange::RuleThickness(v) => properties.rule_thickness = v,

        //Rule + Space properties
        PropertyChange::Orientation(v) => properties.orientation = v,

        // Scrollable properties
        PropertyChange::ScrollableDirection(value) => properties.scroll_dir = value,
        PropertyChange::ScrollableAnchorX(value) => properties.anchor_x = value,
        PropertyChange::ScrollableAnchorY(value) => properties.anchor_y = value,

        // Image properties
        PropertyChange::ImagePath(v) => properties.image_path = v,
        PropertyChange::ImageFit(v) => properties.image_fit = v,

        // Svg properties
        PropertyChange::SvgPath(v) => properties.svg_path = v,
        PropertyChange::SvgFit(v) => properties.svg_fit = v,

        // Tooltip properties
        PropertyChange::TooltipText(v) => properties.tooltip_text = v,
        PropertyChange::TooltipPosition(v) => properties.tooltip_position = v,

        PropertyChange::ComboBoxSelected(v) => properties.combobox_selected = v,
        PropertyChange::ComboBoxPlaceholder(v) => properties.combobox_placeholder = v,
        PropertyChange::ComboBoxState(v) => {
            properties.combobox_options = v.clone();
            // Recreate state with new options
            properties.combobox_state = combo_box::State::new(v);
        }
        PropertyChange::ComboBoxUseOnInput(v) => properties.combobox_use_on_input = v,
        PropertyChange::ComboBoxUseOnOptionHovered(v) => {
            properties.combobox_use_on_option_hovered = v
        }
        PropertyChange::ComboBoxUseOnOpen(v) => properties.combobox_use_on_open = v,
        PropertyChange::ComboBoxUseOnClose(v) => properties.combobox_use_on_close = v,
        PropertyChange::ComboBoxSize(v) => properties.combobox_size = v,
        PropertyChange::ComboBoxIconEnabled(v) => properties.combobox_icon_enabled = v,
        PropertyChange::ComboBoxIconSelected(name, cp) => {
            properties.combobox_icon_name = name;
            properties.combobox_icon_codepoint = cp;
        }
        PropertyChange::ComboBoxIconSize(v) => properties.combobox_icon_size = v,
        PropertyChange::ComboBoxIconSpacing(v) => properties.combobox_icon_spacing = v,
        PropertyChange::ComboBoxIconSide(v) => properties.combobox_icon_side = v,
        PropertyChange::ComboBoxIconPickerFilter(v) => properties.combobox_icon_picker_filter = v,
        PropertyChange::ComboBoxEnumId(id) => {
            //Set referenced_enum Id
            properties.referenced_enum = id;

            //Update combo_box state from Enum
            let state = if let Some(ref enum_id) = properties.referenced_enum {
                if let Some(enum_def) = type_system.get_enum(enum_id.clone()) {
                    let variants: Vec<String> =
                        enum_def.variants.iter().map(|v| v.name.clone()).collect();

                    combo_box::State::new(variants)
                } else {
                    combo_box::State::new(vec![])
                }
            } else {
                combo_box::State::new(vec![])
            };

            properties.combobox_state = state;
        }

        PropertyChange::MarkdownContent(action) => {
            let is_edit = action.is_edit();

            properties.markdown_source.perform(action);

            if is_edit {
                properties.markdown_content =
                    markdown::Content::parse(&properties.markdown_source.text())
                        .items()
                        .to_vec();
            }
        }
        PropertyChange::MarkdownTextSize(v) => properties.markdown_text_size = v,

        PropertyChange::QRCodeData(url) => {
            properties.qrcode_link = url;
            properties.qrcode_data = Some(qr_code::Data::new(&properties.qrcode_link).unwrap())
        }
        PropertyChange::QRCodeCellSize(v) => properties.qrcode_cell_size = v,

        PropertyChange::TableReferencedStruct(v) => properties.table_referenced_struct = v,
        PropertyChange::TablePaddingX(v) => properties.table_padding_x = v,
        PropertyChange::TablePaddingY(v) => properties.table_padding_y = v,
        PropertyChange::TableSeparatorX(v) => properties.table_separator_x = v,
        PropertyChange::TableSeparatorY(v) => properties.table_separator_y = v,
        PropertyChange::TableBoldHeaders(v) => properties.table_bold_headers = v,

        PropertyChange::PinX(v) => properties.pin_point.x = v,
        PropertyChange::PinY(v) => properties.pin_point.y = v,

        PropertyChange::ThemerTheme(v) => properties.themer_theme = v,

        PropertyChange::Noop => {}

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
        PropertyChange::MouseAreaInteraction(interaction) => {
            properties.mousearea_interaction = interaction
        }

        PropertyChange::ViewReferenceId(view_id, name) => {
            properties.referenced_view_id = view_id;
            properties.widget_name = name;
        }
        PropertyChange::AddExtraViewRef => {
            // Placeholder entry; UI should immediately set it via SetExtraViewRef
            if let Some(primary) = properties.referenced_view_id {
                properties.extra_view_ids.push(primary);
            }
        }
        PropertyChange::SetExtraViewRef(idx, view_id) => {
            if let Some(slot) = properties.extra_view_ids.get_mut(idx) {
                *slot = view_id;
            }
        }
        PropertyChange::RemoveExtraViewRef(idx) => {
            if idx < properties.extra_view_ids.len() {
                properties.extra_view_ids.remove(idx);
            }
        }

        // Icon properties
        PropertyChange::IconName(v) => properties.icon_name = v,
        PropertyChange::IconCodepoint(v) => properties.icon_codepoint = v,
        PropertyChange::IconSize(v) => properties.icon_size = v,
        PropertyChange::IconSelected(name, codepoint) => {
            properties.icon_name = name;
            properties.icon_codepoint = codepoint;
        }
        PropertyChange::IconPickerFilter(v) => properties.icon_picker_filter = v,

        // Grid properties
        PropertyChange::GridColumns(v) => properties.grid_columns = v,
        PropertyChange::GridSpacing(v) => properties.grid_spacing = v,
        PropertyChange::GridFixedWidth(v) => properties.grid_fixed_width = v,
        PropertyChange::GridUseFluid(v) => properties.grid_use_fluid = v,
        PropertyChange::GridFluidMaxWidth(v) => properties.grid_fluid_max_width = v,

        // Action system
        PropertyChange::StateFieldOverride(sfr) => properties.state_field_override = sfr,
    }
}

fn parse_color_hex(s: &str) -> Option<iced::Color> {
    let t = s.trim().trim_start_matches('#');
    let hex = |i: usize| u8::from_str_radix(&t[i..i + 2], 16).ok();
    match t.len() {
        6 => {
            if let (Some(r), Some(g), Some(b)) = (hex(0), hex(2), hex(4)) {
                return Some(Color::from_rgba8(r, g, b, 255.0));
            } else {
                None
            }
        }
        8 => {
            if let (Some(r), Some(g), Some(b), Some(a)) = (hex(0), hex(2), hex(4), hex(6)) {
                return Some(Color::from_rgba8(r, g, b, a.into()));
            } else {
                None
            }
        }
        _ => None,
    }
}

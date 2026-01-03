use crate::code_generator::writer::{CodeWriter, to_snake_case, to_pascal_case};
use crate::data_structures::types::types::{WidgetType, Widget, WidgetId};
use crate::data_structures::types::type_implementations::*;
use crate::views::theme_and_stylefn_builder::{CustomThemes, ThemePaneEnum};
use iced::widget::text::LineHeight;
use iced::{Length, Padding, Alignment, Theme};
use std::collections::HashMap;

/// Generate code for a specific widget
pub fn generate_widget_code(
    writer: &mut CodeWriter, 
    widget: &Widget,
    names: &HashMap<WidgetId, String>, 
    use_self: bool,
    custom_styles: &CustomThemes,
) {
    match widget.widget_type {
        WidgetType::Button => generate_button(writer, widget, names, custom_styles, use_self),
        WidgetType::Checkbox => generate_checkbox(writer, widget, names, use_self),
        WidgetType::Column => generate_column(writer, widget, names, custom_styles, use_self),
        WidgetType::ComboBox => generate_combobox(writer, widget, names, use_self),
        WidgetType::Container => generate_container(writer, widget, names, custom_styles, use_self),
        WidgetType::Image => generate_image(writer, widget, names, use_self),
        WidgetType::Markdown => generate_markdown(writer, widget, names, use_self),
        WidgetType::MouseArea => generate_mousearea(writer, widget, names, custom_styles, use_self),
        WidgetType::PickList => generate_picklist(writer, widget, names, use_self),
        WidgetType::Pin => generate_pin(writer, widget, names, use_self),
        WidgetType::ProgressBar => generate_progressbar(writer, widget, names, use_self),
        WidgetType::QRCode => generate_qrcode(writer, widget, names, use_self),
        WidgetType::Radio => generate_radio(writer, widget, names, use_self),
        WidgetType::Row => generate_row(writer, widget, names, custom_styles, use_self),
        WidgetType::Rule => generate_rule(writer, widget, names, use_self),
        WidgetType::Scrollable => generate_scrollable(writer, widget, names, custom_styles, use_self),
        WidgetType::Slider => generate_slider(writer, widget, names, use_self),
        WidgetType::Space => generate_space(writer, widget, names, use_self),
        WidgetType::Stack => generate_stack(writer, widget, names, custom_styles, use_self),
        WidgetType::Svg => generate_svg(writer, widget, names, use_self),
        WidgetType::Text => generate_text(writer, widget, names, use_self),
        WidgetType::TextInput => generate_textinput(writer, widget, names, use_self),
        WidgetType::Themer => generate_themer(writer, widget, names, custom_styles, use_self),
        WidgetType::Toggler => generate_toggler(writer, widget, names, use_self),
        WidgetType::Tooltip => generate_tooltip(writer, widget, names, custom_styles, use_self),
        WidgetType::VerticalSlider => generate_verticalslider(writer, widget, names, use_self),
        WidgetType::ViewReference => {}
    }
}

fn generate_button(writer: &mut CodeWriter, widget: &Widget, names: &HashMap<WidgetId, String>, custom_styles: &CustomThemes, _use_self: bool) {
    let name = names.get(&widget.id).unwrap_or(&"widget".to_string()).clone();

    writer.add_indent();
    writer.add_function("button");
    writer.add_plain("(");
    writer.add_string(&format!("\"{}\"", widget.properties.text_content));
    writer.add_plain(")");

    writer.increase_indent();
    if widget.properties.button_on_press_enabled {
        writer.on_press(&name)
    }

    if widget.properties.button_on_press_with_enabled {
        writer.on_press_with(&name)
    }

    if widget.properties.button_on_press_maybe_enabled {
        writer.on_press_maybe(&name)
    }

    match widget.properties.custom_style_name.clone() {

        Some(style) => {
            if let Some(style_map) = custom_styles.styles().get(&ThemePaneEnum::Button) {
                writer.add_style("styles::button", &style.to_lowercase());
            }  
            else if ButtonStyleType::all().contains(&style) {
                let style = ButtonStyleType::get(&style).unwrap();
                match style {
                    ButtonStyleType::Secondary => {
                        writer.add_style("button","secondary")
                    }
                    ButtonStyleType::Success => {
                        writer.add_style("button","success")
                    }
                    ButtonStyleType::Danger => {
                        writer.add_style("button","danger")
                    }
                    ButtonStyleType::Text => {
                        writer.add_style("button","text")
                    }
                    ButtonStyleType::Background => {
                        writer.add_style("button","background")
                    }
                    ButtonStyleType::Subtle => {
                        writer.add_style("button","subtle")
                    }
                    ButtonStyleType::Primary => {}
                }
            }
        }
        None => {},

    }

    if widget.properties.width != Length::Shrink {
        writer.add_width(widget.properties.width)
    }

    if widget.properties.height != Length::Shrink {
        writer.add_height(widget.properties.height)
    }

    if widget.properties.padding != (Padding { top: 5.0, bottom: 5.0, right: 10.0, left: 10.0 }) {
        writer.generate_padding(&widget.properties.padding, widget.properties.padding_mode);
    }

    if widget.properties.clip {
        writer.add_clip()
    }
    writer.decrease_indent();
}

fn generate_checkbox(writer: &mut CodeWriter, widget: &Widget, names: &HashMap<WidgetId, String>, use_self: bool) {
    let name = names.get(&widget.id).unwrap_or(&"widget".to_string()).clone();

    writer.add_indent();
    writer.add_function("checkbox");
    writer.add_plain("(");
    if use_self {
        writer.add_keyword("self");
        writer.add_operator(".");
        writer.add_identifier(&format!("{}_checked", to_snake_case(&name)));
    } else {
        writer.add_keyword(if widget.properties.checkbox_checked { "true" } else { "false" });
    }

    writer.add_plain(")");
    writer.increase_indent();

    writer.add_keyword(".");
    writer.add_identifier("label");
    writer.add_plain("(");
    writer.add_string(&format!("\"{}\"", widget.properties.checkbox_label));
    writer.add_plain(")");

     writer.on_toggle(&name);
    

    if widget.properties.checkbox_size != 16.0 {
        writer.add_size(widget.properties.checkbox_spacing);
    }

    if widget.properties.checkbox_spacing != 8.0 {
        writer.add_spacing(widget.properties.checkbox_spacing);
    }

    if widget.properties.width != Length::Shrink {
        writer.add_width(widget.properties.width);
    }
    writer.decrease_indent();
}

fn generate_column(writer: &mut CodeWriter, widget: &Widget, names: &HashMap<WidgetId, String>, custom_styles: &CustomThemes, use_self: bool) {
    writer.add_indent();
    writer.add_macro("column!");
    writer.add_plain("[");
    writer.add_newline();
    
    writer.increase_indent();
    if widget.children.is_empty() {
        writer.add_indent();
        writer.add_function("text");
        writer.add_plain("(");
        writer.add_string("\"Column Item\"");
        writer.add_plain(")");
        writer.add_newline();
    } else {
        for (i, child) in widget.children.iter().enumerate() {
            generate_widget_code(writer, child, names, use_self, custom_styles);
            if i < widget.children.len() - 1 {
                writer.add_plain(",");
            }
            writer.add_newline();
        }
    }
    writer.decrease_indent();
    writer.add_indent();
    writer.add_plain("]");
    

    if widget.properties.spacing != 0.0 {
        writer.add_spacing(widget.properties.spacing);
    }

    if widget.properties.width != Length::Shrink {
        writer.add_width(widget.properties.width);
    }

    if let Some(max_w) = widget.properties.max_width {
        writer.add_max_width(max_w)
    }

    if widget.properties.height != Length::Shrink {
        writer.add_height(widget.properties.height);
    }

    if widget.properties.align_items != Alignment::Start {
        writer.add_align_x(widget.properties.align_items);
    }

    if widget.properties.padding != Padding::ZERO {
        writer.generate_padding(&widget.properties.padding, widget.properties.padding_mode);
    }

    if widget.properties.clip {
        writer.add_clip();
    }   
}

fn generate_combobox(writer: &mut CodeWriter, widget: &Widget, names: &HashMap<WidgetId, String>, use_self: bool) {
    let props = &widget.properties;
    let name = names.get(&widget.id).unwrap_or(&"widget".to_string()).clone();

    writer.add_indent();
    writer.add_function("combo_box");
    writer.add_plain("(");
    writer.add_newline();
    writer.increase_indent();
    
    // State reference
    writer.add_indent();
    writer.add_operator("&");
    if use_self {
        writer.add_keyword("self");
        writer.add_operator(".");
        writer.add_identifier(&format!("{}_state", to_snake_case(&name)));
    } else {
        writer.add_plain("state");
    }
    writer.add_plain(",");
    writer.add_newline();
    
    // Placeholder
    writer.add_indent();
    writer.add_string(&format!("\"{}\"", props.combobox_placeholder));
    writer.add_plain(",");
    writer.add_newline();
    
    // Current value
    writer.add_indent();
    if use_self {
        writer.add_plain("Some(");
        writer.add_operator("&");
        writer.add_keyword("self");
        writer.add_operator(".");
        writer.add_identifier(&format!("{}_value", to_snake_case(&name)));
        writer.add_plain(")");
    } else {
        if let Some(ref val) = props.combobox_selected {
            writer.add_plain("Some(");
            writer.add_string(&format!("\"{}\"", val));
            writer.add_plain(")");
        } else {
            writer.add_plain("None");
        }
    }
    writer.add_plain(",");
    writer.add_newline();
    
    // On option selected (always present)
    writer.add_indent();
    if use_self {
        writer.add_type("Message");
        writer.add_operator("::");
        writer.add_plain(&format!("{}Selected", to_pascal_case(&name)));
    } else {
        writer.add_operator("|");
        writer.add_identifier("_");
        writer.add_operator("|");
        writer.add_plain(" ");
        writer.add_type("Message");
        writer.add_operator("::");
        writer.add_plain("Noop");
    }
    writer.add_newline();
    
    writer.decrease_indent();
    writer.add_indent();
    writer.add_plain(")");
    
    // Now add optional methods
    if props.combobox_use_on_input {
        writer.on_input(&name);
    }
    
    if props.combobox_use_on_option_hovered {
        writer.on_option_hovered(&name);
    }
    
    if props.combobox_use_on_open {
        writer.on_open(&name);
    }
    
    if props.combobox_use_on_close {
        writer.on_close(&name);
    }
    
    // Properties
    if props.combobox_size != 16.0 {
        writer.add_size(props.combobox_size);
    }
    
    if !matches!(props.width, Length::Fill) {
        writer.add_width(props.width);
    }   
}

fn generate_container(writer: &mut CodeWriter, widget: &Widget, names: &HashMap<WidgetId, String>, custom_styles: &CustomThemes, use_self: bool) {
    let props = &widget.properties;

    writer.add_indent();
    writer.add_function("container");
    writer.add_plain("(");
    writer.add_newline();
    writer.increase_indent();
    
    if widget.children.is_empty() {
        writer.add_indent();
        writer.add_function("text");
        writer.add_plain("(");
        writer.add_string("\"Container Content\"");
        writer.add_plain(")");
    } else {
        for child in &widget.children {
            generate_widget_code(writer, child, names, use_self, custom_styles);
        }
    }
    
    writer.add_newline();
    writer.decrease_indent();
    writer.add_indent();
    writer.add_plain(")");

    if let Some(ref id) = props.widget_id {
        if !id.is_empty() {
            writer.add_id(id);
        }
    }

    // Sizing
    match props.container_sizing_mode {
        ContainerSizingMode::Manual => {
            // Width
            if !matches!(props.width, Length::Fill) {
                writer.add_width(props.width);
            }
            // Height
            if !matches!(props.height, Length::Fill) {
                writer.add_height(props.height);
            }
            // Alignment
            if !matches!(props.align_x, ContainerAlignX::Left) {
                writer.add_align_x(props.align_x.into())
            }
            if !matches!(props.align_y, ContainerAlignY::Top) {
                writer.add_align_y(props.align_y.into())
            }
        }
        ContainerSizingMode::CenterX => {
            writer.add_center_x(props.container_center_length);
        }
        ContainerSizingMode::CenterY => {
            writer.add_center_y(props.container_center_length);
        }
        ContainerSizingMode::Center => {
            writer.add_center(props.container_center_length);
        }
    }

    // Max width
    if let Some(max_width) = props.max_width {
        writer.add_max_width(max_width);
    }

    // Max Height
    if let Some(max_height) = props.max_height {
        writer.add_max_height(max_height);
    }

    // Padding
    if props.padding != Padding::ZERO {
        writer.generate_padding(&props.padding, props.padding_mode);
    }

    // Clip
    if props.clip {
        writer.add_clip();
    }
}

fn generate_image(writer: &mut CodeWriter, widget: &Widget, _names: &HashMap<WidgetId, String>, _use_self: bool) {
    let props = &widget.properties;

    writer.add_indent();
    writer.add_function("image");
    writer.add_plain("(");
    if props.image_path.is_empty() {
        writer.add_string("\"path/to/image.png\"");
    } else {
        writer.add_plain("r");
        writer.add_string(&format!("\"{}\"", props.image_path));
    }
    writer.add_plain(")");

    // Content fit
    if !matches!(props.image_fit, ContentFitChoice::Contain) {
        writer.add_content_fit(&props.image_fit);
    }
    
    // Width
    if !matches!(props.width, Length::Shrink) {
        writer.add_width(props.width);
    }
    
    // Height
    if !matches!(props.height, Length::Shrink) {
        writer.add_height(props.height);
    }
}

fn generate_markdown(writer: &mut CodeWriter, widget: &Widget, names: &HashMap<WidgetId, String>, use_self: bool) {}

fn generate_mousearea(writer: &mut CodeWriter, widget: &Widget, names: &HashMap<WidgetId, String>, custom_styles: &CustomThemes, use_self: bool) {
    let props = &widget.properties;
    let name = names.get(&widget.id).unwrap_or(&"widget".to_string()).clone();

    writer.add_indent();
    writer.add_function("mouse_area");
    writer.add_plain("(");
    writer.add_newline();
    
    writer.increase_indent();
    if !widget.children.is_empty() { // Generate Child
        generate_widget_code(writer, &widget.children[0], names, use_self, custom_styles);
    }
    writer.decrease_indent();  
    
    writer.add_indent();
    writer.add_plain(")");
    writer.increase_indent();
    
    // Conditionally add event handlers
    if props.mousearea_on_press {
        writer.on_press(&name);
    }

    if props.mousearea_on_release {
        writer.on_release(&name);
    }

    if props.mousearea_on_double_click {
        writer.on_double_click(&name);
    }

    if props.mousearea_on_right_press {
        writer.on_right_press(&name);
    }

    if props.mousearea_on_right_release {
        writer.on_right_release(&name);
    }

    if props.mousearea_on_middle_press {
        writer.on_middle_press(&name);
    }

    if props.mousearea_on_middle_release {
        writer.on_middle_release(&name);
    }
    
    if props.mousearea_on_scroll {
        writer.on_scroll(&name);
    }

    if props.mousearea_on_enter {
        writer.on_enter(&name);
    }
    
    if props.mousearea_on_move {
        writer.on_move(&name);
    }

    if props.mousearea_on_exit {
        writer.on_exit(&name);
    }
    
    if let Some(interaction) = props.mousearea_interaction {
        writer.on_mouse_interaction(&interaction);
    }

    writer.decrease_indent();
}

fn generate_picklist(writer: &mut CodeWriter, widget: &Widget, names: &HashMap<WidgetId, String>, use_self: bool) {
    let props = &widget.properties;
    let name = names.get(&widget.id).unwrap_or(&"widget".to_string()).clone();

    writer.add_indent();
    writer.add_function("pick_list");
    writer.add_plain("(");
    writer.add_newline();
    writer.increase_indent();
    writer.add_indent();
    writer.add_plain("vec![");
    for (i, option) in props.picklist_options.iter().enumerate() {
        writer.add_string(&format!("\"{}\"", option));
        writer.add_operator(".");
        writer.add_function("to_string");
        writer.add_plain("()");
        if i < props.picklist_options.len() - 1 {
            writer.add_plain(", ");
        }
    }
    writer.add_plain("],");
    writer.add_newline();
    writer.add_indent();
    if use_self {
        writer.add_keyword("self");
        writer.add_operator(".");
        writer.add_identifier(&format!("{}_selected", to_snake_case(&name)));
        writer.add_operator(".");
        writer.add_function("clone");
        writer.add_plain("()");
    } else if let Some(ref selected) = props.picklist_selected {
        writer.add_plain("Some(");
        writer.add_string(&format!("\"{}\"", selected));
        writer.add_operator(".");
        writer.add_function("to_string");
        writer.add_plain("())");
    } else {
        writer.add_plain("None");
    }
    writer.add_plain(",");
    writer.add_newline();
    writer.add_indent();
    if use_self {
        writer.add_type("Message");
        writer.add_operator("::");
        writer.add_plain(&format!("{}Selected", to_pascal_case(&name)));
    } else {
        writer.add_operator("|");
        writer.add_identifier("_");
        writer.add_operator("|");
        writer.add_plain(" ");
        writer.add_type("Message");
        writer.add_operator("::");
        writer.add_plain("Noop");
    }
    writer.add_newline();
    writer.decrease_indent();
    writer.add_indent();
    writer.add_plain(")");

    if !props.picklist_placeholder.is_empty() && props.picklist_placeholder != "Choose an option..." {
        writer.add_placeholder(&props.picklist_placeholder);
    }
    
    if !matches!(props.width, Length::Shrink) {
        writer.add_width(props.width);
    }

    if props.padding != (Padding { top: 5.0, bottom: 5.0, right: 10.0, left: 10.0 }) {
        writer.generate_padding(&props.padding, props.padding_mode);
    }
}

fn generate_pin(writer: &mut CodeWriter, widget: &Widget, names: &HashMap<WidgetId, String>, use_self: bool) {}

fn generate_progressbar(writer: &mut CodeWriter, widget: &Widget, names: &HashMap<WidgetId, String>, _use_self: bool) {
    let props = &widget.properties;
    let name = names.get(&widget.id).unwrap_or(&"widget".to_string()).clone();

    writer.add_indent();
    writer.add_function("progress_bar");
    writer.add_plain("(");
    writer.add_number(&format!("{:.1}", props.progress_min));
    writer.add_operator("..=");
    writer.add_number(&format!("{:.1}", props.progress_max));
    writer.add_plain(", ");
    writer.add_number(&format!("{:.2}", props.progress_value));
    writer.add_plain(")");

    writer.increase_indent();
    if !matches!(props.progress_length, Length::Fill) {
        writer.add_length(props.progress_length);
    }
    
    if props.progress_girth != iced::widget::progress_bar::ProgressBar::<Theme>::DEFAULT_GIRTH {
        writer.add_newline();
        writer.add_indent();
        writer.add_operator(".");
        writer.add_function("girth");
        writer.add_plain("(");
        writer.add_number(&format!("{}", props.progress_girth));
        writer.add_plain(")");
    }
    
    if props.progress_vertical {
        writer.add_newline();
        writer.add_indent();
        writer.add_operator(".");
        writer.add_function("vertical");
        writer.add_plain("()");
    }
    writer.decrease_indent();
}

fn generate_qrcode(writer: &mut CodeWriter, widget: &Widget, names: &HashMap<WidgetId, String>, _use_self: bool) {
    let props = &widget.properties;
    let name = names.get(&widget.id).unwrap_or(&"widget".to_string()).clone();

    writer.add_indent();
    writer.add_function("qr_code");
    writer.add_plain("(");
    writer.add_operator("&");
    writer.add_keyword("self");
    writer.add_operator(".");
    writer.add_identifier("qr_data");
    writer.add_plain(")");

    writer.increase_indent();
    if !matches!(props.qrcode_cell_size, 4.0) { // const DEFAULT_CELL_SIZE: f32 = 4.0;
        writer.add_newline();
        writer.add_indent();
        writer.add_operator(".");
        writer.add_function("cell_size");
        writer.add_plain("(");
        writer.add_number(&format!("{}", props.qrcode_cell_size));
        writer.add_plain(")");
    }
    
    writer.decrease_indent();    
}

fn generate_radio(writer: &mut CodeWriter, widget: &Widget, names: &HashMap<WidgetId, String>, use_self: bool) {
    let props = &widget.properties;
    let name = names.get(&widget.id).unwrap_or(&"widget".to_string()).clone();

    writer.add_indent();
    writer.add_macro("column!");
    writer.add_plain("[");
    writer.add_newline();
    writer.increase_indent();
    
    for (i, option) in props.radio_options.iter().enumerate() {
        writer.add_indent();
        writer.add_function("radio");
        writer.add_plain("(");
        writer.add_string(&format!("\"{}\"", option));
        writer.add_plain(", ");
        writer.add_number(&format!("{}", i));
        writer.add_plain(", ");
        if use_self {
            writer.add_plain("Some(");
            writer.add_keyword("self");
            writer.add_operator(".");
            writer.add_identifier(&format!("{}_selected", to_snake_case(&name)));
            writer.add_plain(")");
        } else {
            writer.add_plain("Some(");
            writer.add_number(&format!("{}", props.radio_selected_index));
            writer.add_plain(")");
        }
        writer.add_plain(", ");
        if use_self {
            writer.add_type("Message");
            writer.add_operator("::");
            writer.add_plain(&format!("{}Selected", to_pascal_case(&name)));
        } else {
            writer.add_operator("|");
            writer.add_identifier("_");
            writer.add_operator("|");
            writer.add_plain(" ");
            writer.add_type("Message");
            writer.add_operator("::");
            writer.add_plain("Noop");
        }
        writer.add_plain(")");
        if props.radio_size != 16.0 {
            writer.add_size(props.radio_size);
        }
        if props.width != Length::Shrink {
            writer.add_width(props.width);
        }
        if i < props.radio_options.len() - 1 {
            writer.add_plain(",");
        }
        writer.add_newline();
    }
    
    writer.decrease_indent();
    writer.add_indent();
    writer.add_plain("]");    
}

fn generate_row(writer: &mut CodeWriter, widget: &Widget, names: &HashMap<WidgetId, String>, custom_styles: &CustomThemes, use_self: bool) {
    let props = &widget.properties;

    writer.add_indent();
    writer.add_macro("row!");
    writer.add_plain("[");
    writer.add_newline();
    writer.increase_indent();
    
    if widget.children.is_empty() {
        writer.add_indent();
        writer.add_function("text");
        writer.add_plain("(");
        writer.add_string("\"Row Item\"");
        writer.add_plain(")");
    } else {
        for (i, child) in widget.children.iter().enumerate() {
            generate_widget_code(writer, child, names, use_self, custom_styles);
            if i < widget.children.len() - 1 {
                writer.add_plain(",");
            }
            writer.add_newline();
        }
    }

    writer.decrease_indent();
    writer.add_indent();
    writer.add_plain("]");
    
    if props.spacing != 0.0 {
        writer.add_spacing(props.spacing);
    }

    if !matches!(props.align_items, Alignment::Start) {
        writer.add_align_y(props.align_items);
    }

    if !matches!(props.width, Length::Shrink) {
        writer.add_width(props.width);
    }

    if !matches!(props.height, Length::Shrink) {
        writer.add_height(props.height);
    }

    if !matches!(props.padding, Padding::ZERO) {
        writer.generate_padding(&props.padding, props.padding_mode);
    }
    
    if props.is_wrapping_row {
        writer.add_newline();
        writer.add_indent();
        writer.add_operator(".");
        writer.add_function("wrap");
        writer.add_plain("()");
        
        // Vertical spacing
        if props.match_horizontal_spacing {
            writer.add_newline();
            writer.add_indent();
            writer.add_operator(".");
            writer.add_function("vertical_spacing");
            writer.add_plain("(");
            writer.add_number(&format!("{:.1}", props.wrapping_vertical_spacing));
            writer.add_plain(")");
        }
        
        if !matches!(props.wrapping_align_x, ContainerAlignX::Left) {
            writer.add_align_x(props.wrapping_align_x.into());
        }
    }

    if props.clip {
        writer.add_clip();
    }
}

fn generate_rule(writer: &mut CodeWriter, widget: &Widget, names: &HashMap<WidgetId, String>, _use_self: bool) {
    let props = &widget.properties;

    writer.add_indent();
    match props.orientation {
        Orientation::Horizontal => {
            writer.add_function("rule::horizontal");
        }
        Orientation::Vertical => {
            writer.add_function("rule::vertical");
        }
    }
    writer.add_plain("(");
    writer.add_number(&format!("{}", props.rule_thickness));
    writer.add_plain(")");    
}

fn generate_scrollable(writer: &mut CodeWriter, widget: &Widget, names: &HashMap<WidgetId, String>, custom_styles: &CustomThemes, use_self: bool) {
    let props = &widget.properties;

    writer.add_indent();
    writer.add_function("scrollable");
    writer.add_plain("(");
    writer.add_newline();
    writer.increase_indent();
    
    if widget.children.is_empty() {
        writer.add_indent();
        writer.add_macro("column!");
        writer.add_plain("[");
        writer.add_newline();
        writer.increase_indent();
        for i in 1..=10 {
            writer.add_indent();
            writer.add_function("text");
            writer.add_plain("(");
            writer.add_string(&format!("\"Scrollable Item {}\"", i));
            writer.add_plain(")");
            if i < 10 {
                writer.add_plain(",");
            }
            writer.add_newline();
        }
        writer.decrease_indent();
        writer.add_indent();
        writer.add_plain("]");
    } else {
        for child in &widget.children {
            generate_widget_code(writer, child, names, use_self, custom_styles);
        }
    }
    
    writer.add_newline();
    writer.decrease_indent();
    writer.add_indent();
    writer.add_plain(")");
    writer.increase_indent();

    let is_default_dir = matches!(
        props.scroll_dir,
        iced::widget::scrollable::Direction::Vertical(_)
    );

    if !is_default_dir {
        writer.add_newline();
        writer.add_indent();
        writer.add_operator(".");
        writer.add_function("direction");
        writer.add_plain("(");
        match props.scroll_dir {
            iced::widget::scrollable::Direction::Horizontal(_) => {
                writer.add_plain("scrollable::Direction::Horizontal(scrollable::Scrollbar::default())");
            }
            iced::widget::scrollable::Direction::Both { .. } => {
                writer.add_plain("scrollable::Direction::Both { vertical: scrollable::Scrollbar::default(), horizontal: scrollable::Scrollbar::default() }");
            }
            _ => {}
        }
        writer.add_plain(")");
    }

    if !matches!(props.width, Length::Shrink) {
    writer.add_width(props.width);
    }

    if !matches!(props.height, Length::Shrink) {
    writer.add_height(props.height);
    }

    writer.decrease_indent();
}

fn generate_slider(writer: &mut CodeWriter, widget: &Widget, names: &HashMap<WidgetId, String>, use_self: bool) {
    let props = &widget.properties;
    let name = names.get(&widget.id).unwrap_or(&"widget".to_string()).clone();

    writer.add_indent();
    writer.add_function("slider");
    writer.add_plain("(");
    writer.add_number(&format!("{:.1}", props.slider_min));
    writer.add_operator("..=");
    writer.add_number(&format!("{:.1}", props.slider_max));
    writer.add_plain(", ");
    if use_self {
        writer.add_keyword("self");
        writer.add_operator(".");
        writer.add_identifier(&format!("{}_value", to_snake_case(&name)));
    } else {
        writer.add_number(&format!("{}", props.slider_value));
    }
    writer.add_plain(", ");
    if use_self {
        writer.add_type("Message");
        writer.add_operator("::");
        writer.add_plain(&format!("{}Changed", to_pascal_case(&name)));
    } else {
        writer.add_operator("|");
        writer.add_identifier("_");
        writer.add_operator("|");
        writer.add_plain(" ");
        writer.add_type("Message");
        writer.add_operator("::");
        writer.add_plain("Noop");
    }
    writer.add_plain(")");

    writer.increase_indent();
    if props.slider_step != 1.0 {
        writer.add_step(props.slider_step);
    }
    
    if !matches!(props.slider_height, iced::widget::slider::Slider::<f32, Theme>::DEFAULT_HEIGHT) {
        writer.add_height(Length::Fixed(props.slider_height));
    } 

    writer.decrease_indent();
}

fn generate_space(writer: &mut CodeWriter, widget: &Widget, names: &HashMap<WidgetId, String>, _use_self: bool) {
    let props = &widget.properties;

    writer.add_indent();
    match props.orientation {
        Orientation::Horizontal => {
            writer.add_function("space::horizontal");
            if !matches!(props.width, Length::Fill) {
                writer.add_width(props.width)
            }
            if !matches!(props.height, Length::Shrink) {
                writer.add_height(props.height)
            }
        }
        Orientation::Vertical => {
            writer.add_function("space::vertical");
            if !matches!(props.width, Length::Shrink) {
                writer.add_width(props.width)
            }
            if !matches!(props.height, Length::Fill) {
                writer.add_height(props.height)                
            }
        }
    }
    writer.add_plain("()");
}

fn generate_stack(writer: &mut CodeWriter, widget: &Widget, names: &HashMap<WidgetId, String>, custom_styles: &CustomThemes, use_self: bool) {
    let props = &widget.properties;

    writer.add_indent();
    writer.add_function("stack");
    writer.add_plain("(vec![");
    writer.add_newline();
    writer.increase_indent();
    
    if widget.children.is_empty() {
        writer.add_indent();
        writer.add_function("text");
        writer.add_plain("(");
        writer.add_string("\"Layer 1\"");
        writer.add_plain(").into(),");
        writer.add_newline();
        writer.add_indent();
        writer.add_function("text");
        writer.add_plain("(");
        writer.add_string("\"Layer 2\"");
        writer.add_plain(").into(),");
    } else {
        for (i, child) in widget.children.iter().enumerate() {
            generate_widget_code(writer, child, names, use_self, custom_styles);
            writer.add_operator(".");
            writer.add_function("into");
            writer.add_plain("()");
            if i < widget.children.len() - 1 {
                writer.add_plain(",");
            }
            writer.add_newline();
        }
    }
    
    writer.decrease_indent();
    writer.add_indent();
    writer.add_plain("])");

    if !matches!(props.width, Length::Fill) {
        writer.add_width(props.width)
    }
    
    if !matches!(props.height, Length::Fill) {
        writer.add_height(props.height);
    }  
}

fn generate_svg(writer: &mut CodeWriter, widget: &Widget, names: &HashMap<WidgetId, String>, _use_self: bool) {
    let props = &widget.properties;

    writer.add_indent();
    writer.add_function("svg");
    writer.add_plain("(svg::Handle::from_path(");
    if props.svg_path.is_empty() {
        writer.add_string("\"path/to/icon.svg\"");
    } else {
        writer.add_plain("r");
        writer.add_string(&format!("\"{}\"", props.svg_path));
    }
    writer.add_plain("))");

    if !matches!(props.svg_fit, ContentFitChoice::Contain) {
        writer.add_content_fit(&props.svg_fit);
    }
    
    if !matches!(props.width, Length::Fill) {
        writer.add_width(props.width);
    }
    
    if !matches!(props.height, Length::Shrink) {
        writer.add_height(props.height);
    }
}

fn generate_text(writer: &mut CodeWriter, widget: &Widget, names: &HashMap<WidgetId, String>, _use_self: bool) {
    let props = &widget.properties;

    writer.add_indent();
    writer.add_function("text");
    writer.add_plain("(");
    writer.add_string(&format!("\"{}\"", props.text_content));
    writer.add_plain(")");

    if props.text_size != 16.0 {
        writer.add_size(props.text_size);
    }
    
    if !matches!(props.width, Length::Shrink) {
        writer.add_width(props.width);
    }

    if !matches!(props.height, Length::Shrink) {
        writer.add_height(props.height);
    }  
}

fn generate_textinput(writer: &mut CodeWriter, widget: &Widget, names: &HashMap<WidgetId, String>, use_self: bool) {
    let props = &widget.properties;
    let name = names.get(&widget.id).unwrap_or(&"widget".to_string()).clone();

    writer.add_indent();
    writer.add_function("text_input");
    writer.add_plain("(");
    writer.add_string(&format!("\"{}\"", props.text_input_placeholder));
    writer.add_plain(", ");
    if use_self {
        writer.add_operator("&");
        writer.add_keyword("self");
        writer.add_operator(".");
        writer.add_identifier(&format!("{}_value", to_snake_case(&name)));
    } else {
        writer.add_string("\"\"");
    }
    writer.add_plain(")");
    writer.increase_indent();
    
    writer.on_input(&name);
    
    if props.text_input_on_submit {
        writer.on_submit(&name);
    }
    
    if props.text_input_on_paste {
        writer.on_paste(&name);
    }
    
    // Add secure if enabled
    if props.is_secure {
        writer.add_newline();
        writer.add_indent();
        writer.add_operator(".");
        writer.add_function("secure");
        writer.add_plain("(");
        writer.add_plain("true");
        writer.add_plain(")");
    }
    
    // Add font if not default
    if props.text_input_font != FontType::Default {
        writer.add_font(props.text_input_font);
    }
    
    writer.add_size(props.text_input_size);
    
    writer.generate_padding(&Padding::new(props.text_input_padding), PaddingMode::Uniform);
    
    // Add line_height if not default
    if props.text_input_line_height != LineHeight::default() {
        writer.add_lineheight(&props.text_input_line_height);
    }
    
    if props.text_input_alignment != ContainerAlignX::Left {
        writer.add_align_x(props.text_input_alignment.into());
    }
    
    if !matches!(props.width, Length::Fill) {
        writer.add_width(props.width);
    }
    
    writer.decrease_indent();   
}

fn generate_themer(writer: &mut CodeWriter, widget: &Widget, names: &HashMap<WidgetId, String>, custom_styles: &CustomThemes, use_self: bool) {
    let props = &widget.properties;
    let name = names.get(&widget.id).unwrap_or(&"widget".to_string()).clone();

    writer.add_indent();
    writer.add_function("themer");
    writer.add_plain("(");
    if let Some(theme) = &props.themer_theme {
        writer.add_plain("Some(");
        writer.add_type("Theme");
        writer.add_operator("::");
        match theme {
            Theme::Light => writer.add_plain("Light"),
            Theme::Dark => writer.add_plain("Dark"),
            Theme::Dracula => writer.add_plain("Dracula"),
            Theme::Nord => writer.add_plain("Nord"),
            Theme::CatppuccinFrappe => writer.add_plain("CatppuccinFrappe"),
            Theme::CatppuccinLatte => writer.add_plain("CatppuccinLatte"),
            Theme::CatppuccinMacchiato => writer.add_plain("CatppuccinMacchiato"),
            Theme::CatppuccinMocha => writer.add_plain("CatppuccinMocha"),
            Theme::Ferra => writer.add_plain("Ferra"),
            Theme::GruvboxDark => writer.add_plain("GruvboxDark"),
            Theme::GruvboxLight => writer.add_plain("GruvboxLight"),
            Theme::KanagawaDragon => writer.add_plain("KanagawaDragon"),
            Theme::KanagawaLotus => writer.add_plain("KanagawaLotus"),
            Theme::KanagawaWave => writer.add_plain("KanagawaWave"),
            Theme::Moonfly => writer.add_plain("Moonfly"),
            Theme::Nightfly => writer.add_plain("Nightfly"),
            Theme::Oxocarbon => writer.add_plain("Oxocarbon"),
            Theme::SolarizedDark => writer.add_plain("SolarizedDark"),
            Theme::SolarizedLight => writer.add_plain("SolarizedLight"),
            Theme::TokyoNight => writer.add_plain("TokyoNight"),
            Theme::TokyoNightLight => writer.add_plain("TokyoNightLight"),
            Theme::TokyoNightStorm => writer.add_plain("TokyoNightStorm"),
            Theme::Custom(_) => writer.add_plain(&name),
        }
        writer.add_plain(")");
    } else {
        writer.add_plain("None");
    }
    writer.add_plain(", ");
    writer.add_newline();
    writer.increase_indent();
    
    if widget.children.is_empty() {
        writer.add_indent();
        writer.add_function("container");
        writer.add_plain("(");
        writer.add_function("text");
        writer.add_plain("(");
        writer.add_string("\"Themed content\"");
        writer.add_plain("))");
    } else {
        for child in &widget.children {
            generate_widget_code(writer, child, names, use_self, custom_styles);
        }
    }
    
    writer.add_newline();
    writer.decrease_indent();
    writer.add_indent();
    writer.add_plain(")");

    if !matches!(props.width, Length::Shrink) {
        writer.add_width(props.width);
    }
    
    if !matches!(props.height, Length::Shrink) {
        writer.add_height(props.height);
    }
}

fn generate_toggler(writer: &mut CodeWriter, widget: &Widget, names: &HashMap<WidgetId, String>, use_self: bool) {
    let props = &widget.properties;
    let name = names.get(&widget.id).unwrap_or(&"widget".to_string()).clone();

    writer.add_indent();
    writer.add_function("toggler");
    writer.add_plain("(");
    if use_self {
        writer.add_keyword("self");
        writer.add_operator(".");
        writer.add_identifier(&format!("{}_active", to_snake_case(&name)));
    } else {
        writer.add_keyword(if props.toggler_active { "true" } else { "false" });
    }
    writer.add_plain(")");
    writer.add_newline();
    writer.increase_indent();
    writer.add_indent();
    writer.add_operator(".");
    writer.add_function("on_toggle");
    writer.add_plain("(");
    if use_self {
        writer.add_type("Message");
        writer.add_operator("::");
        writer.add_plain(&format!("{}Toggled", to_pascal_case(&name)));
    } else {
        writer.add_operator("|");
        writer.add_identifier("_");
        writer.add_operator("|");
        writer.add_plain(" ");
        writer.add_type("Message");
        writer.add_operator("::");
        writer.add_plain("Noop");
    }
    writer.add_plain(")");

    if props.toggler_size != iced::widget::toggler::Toggler::<Theme>::DEFAULT_SIZE {
        writer.add_size(props.toggler_size);
    }
    
    if props.toggler_spacing != iced::widget::toggler::Toggler::<Theme>::DEFAULT_SIZE / 2.0 {
        writer.add_spacing(props.toggler_spacing);
    }
    
    if !props.toggler_label.is_empty() {
        writer.add_newline();
        writer.add_indent();
        writer.add_operator(".");
        writer.add_function("label");
        writer.add_plain("(");
        writer.add_string(&format!("\"{}\"", props.toggler_label));
        writer.add_plain(")");
    }

    if !matches!(props.width, Length::Shrink) {
        writer.add_width(props.width);
    }

    writer.decrease_indent();    
}

fn generate_tooltip(writer: &mut CodeWriter, widget: &Widget, names: &HashMap<WidgetId, String>, custom_styles: &CustomThemes, use_self: bool) {
    let props = &widget.properties;
    
    writer.add_indent();
    writer.add_function("tooltip");
    writer.add_plain("(");
    writer.add_newline();
    writer.increase_indent();
    
    // First child (host)
    if let Some(host) = widget.children.get(0) {
        generate_widget_code(writer, host, names, use_self, custom_styles);
    } else {
        writer.add_indent();
        writer.add_function("text");
        writer.add_plain("(");
        writer.add_string("\"Hover me\"");
        writer.add_plain(")");
    }
    writer.add_plain(",");
    writer.add_newline();
    
    // Second child (tooltip content) or text
    if let Some(content) = widget.children.get(1) {
        generate_widget_code(writer, content, names, use_self, custom_styles);
    } else {
        writer.add_indent();
        writer.add_function("text");
        writer.add_plain("(");
        writer.add_string(&format!("\"{}\"", props.tooltip_text));
        writer.add_plain(")");
    }
    writer.add_plain(",");
    writer.add_newline();
    
    // Position
    writer.add_indent();
    writer.add_keyword("tooltip");
    writer.add_operator("::");
    writer.add_type("Position");
    writer.add_operator("::");
    match props.tooltip_position {
        TooltipPosition::Top => writer.add_plain("Top"),
        TooltipPosition::Bottom => writer.add_plain("Bottom"),
        TooltipPosition::Left => writer.add_plain("Left"),
        TooltipPosition::Right => writer.add_plain("Right"),
        TooltipPosition::FollowCursor => writer.add_plain("FollowCursor"),
    }
    
    writer.add_newline();
    writer.decrease_indent();
    writer.add_indent();
    writer.add_plain(")");    
}

fn generate_verticalslider(writer: &mut CodeWriter, widget: &Widget, names: &HashMap<WidgetId, String>, use_self: bool) {
    let props = &widget.properties;
    let name = names.get(&widget.id).unwrap_or(&"widget".to_string()).clone();

    writer.add_indent();
    writer.add_function("vertical_slider");
    writer.add_plain("(");
    writer.add_number(&format!("{:.1}", props.slider_min));
    writer.add_operator("..=");
    writer.add_number(&format!("{:.1}", props.slider_max));
    writer.add_plain(", ");
    if use_self {
        writer.add_keyword("self");
        writer.add_operator(".");
        writer.add_identifier(&format!("{}_value", to_snake_case(&name)));
    } else {
        writer.add_number(&format!("{}", props.slider_value));
    }
    writer.add_plain(", ");
    if use_self {
        writer.add_type("Message");
        writer.add_operator("::");
        writer.add_plain(&format!("{}Changed", to_pascal_case(&name)));
    } else {
        writer.add_operator("|");
        writer.add_identifier("_");
        writer.add_operator("|");
        writer.add_plain(" ");
        writer.add_type("Message");
        writer.add_operator("::");
        writer.add_plain("Noop");
    }
    writer.add_plain(")");

    if props.slider_step != 1.0 {
        writer.add_step(props.slider_step);
    }
    
    // For vertical slider, width can be set as a Length
    if !matches!(props.slider_width, iced::widget::vertical_slider::VerticalSlider::<f32, Theme>::DEFAULT_WIDTH) {
        writer.add_height(Length::Fixed(props.slider_width));
    }
}
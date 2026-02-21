use super::builder::{CodeBuilder, to_pascal_case, to_snake_case};
use super::events::ViewRefInfo;
use crate::data_structures::types::types::{WidgetType, Widget, WidgetId};
use crate::data_structures::types::type_implementations::*;
use crate::enum_builder::TypeSystem;
use crate::views::theme_and_stylefn_builder::{CustomThemes, ThemePaneEnum};
use iced::widget::text::LineHeight;
use iced::{Length, Padding, Alignment, Point, Theme};
use std::collections::HashMap;

pub fn generate_widget_code(
    b: &mut CodeBuilder,
    widget: &Widget,
    names: &HashMap<WidgetId, String>,
    use_self: bool,
    custom_styles: &CustomThemes,
    type_system: &TypeSystem,
    view_refs: &[ViewRefInfo],
) {
    match widget.widget_type {
        WidgetType::Button => generate_button(b, widget, names, custom_styles, use_self, type_system, view_refs),
        WidgetType::Checkbox => generate_checkbox(b, widget, names, custom_styles, use_self),
        WidgetType::Column => generate_column(b, widget, names, custom_styles, use_self, type_system, view_refs),
        WidgetType::ComboBox => generate_combobox(b, widget, names, custom_styles, use_self),
        WidgetType::Container => generate_container(b, widget, names, custom_styles, use_self, type_system, view_refs),
        WidgetType::Image => generate_image(b, widget),
        WidgetType::Markdown => generate_markdown(b, widget, names, use_self),
        WidgetType::MouseArea => generate_mousearea(b, widget, names, custom_styles, use_self, type_system, view_refs),
        WidgetType::PickList => generate_picklist(b, widget, names, use_self),
        WidgetType::Pin => generate_pin(b, widget, names, custom_styles, use_self, type_system, view_refs),
        WidgetType::ProgressBar => generate_progressbar(b, widget),
        WidgetType::QRCode => generate_qrcode(b, widget),
        WidgetType::Radio => generate_radio(b, widget, names, use_self),
        WidgetType::Row => generate_row(b, widget, names, custom_styles, use_self, type_system, view_refs),
        WidgetType::Rule => generate_rule(b, widget, custom_styles),
        WidgetType::Scrollable => generate_scrollable(b, widget, names, custom_styles, use_self, type_system, view_refs),
        WidgetType::Slider => generate_slider(b, widget, names, use_self),
        WidgetType::Space => generate_space(b, widget),
        WidgetType::Stack => generate_stack(b, widget, names, custom_styles, use_self, type_system, view_refs),
        WidgetType::Svg => generate_svg(b, widget),
        WidgetType::Text => generate_text(b, widget),
        WidgetType::TextInput => generate_textinput(b, widget, names, use_self),
        WidgetType::Table => generate_table(b, widget, names, use_self, type_system),
        WidgetType::Themer => generate_themer(b, widget, names, custom_styles, use_self, type_system, view_refs),
        WidgetType::Grid => generate_grid(b, widget, names, custom_styles, use_self, type_system, view_refs),
        WidgetType::Toggler => generate_toggler(b, widget, names, use_self),
        WidgetType::Tooltip => generate_tooltip(b, widget, names, custom_styles, use_self, type_system, view_refs),
        WidgetType::VerticalSlider => generate_verticalslider(b, widget, names, use_self),
        WidgetType::Icon => generate_icon(b, widget),
        WidgetType::ViewReference => generate_view_reference(b, widget, view_refs),
    }
}

fn generate_text(b: &mut CodeBuilder, widget: &Widget) {
    let props = &widget.properties;

    b.indent();
    b.push(&format!("text(\"{}\")", props.text_content));
    b.increase_indent();

    if props.text_size != 16.0 {
        b.add_size(props.text_size);
    }

    if !matches!(props.width, Length::Shrink) {
        b.add_width(props.width);
    }

    if !matches!(props.height, Length::Shrink) {
        b.add_height(props.height);
    }

    b.decrease_indent();
}

fn generate_space(b: &mut CodeBuilder, widget: &Widget) {
    let props = &widget.properties;

    b.indent();
    match props.orientation {
        Orientation::Horizontal => {
            b.push("space::horizontal()");
            if !matches!(props.width, Length::Fill) {
                b.add_width(props.width);
            }
            if !matches!(props.height, Length::Shrink) {
                b.add_height(props.height);
            }
        }
        Orientation::Vertical => {
            b.push("space::vertical()");
            if !matches!(props.width, Length::Shrink) {
                b.add_width(props.width);
            }
            if !matches!(props.height, Length::Fill) {
                b.add_height(props.height);
            }
        }
    }
}

fn generate_rule(b: &mut CodeBuilder, widget: &Widget, custom_styles: &CustomThemes) {
    let props = &widget.properties;

    b.indent();
    match props.orientation {
        Orientation::Horizontal => b.push("rule::horizontal"),
        Orientation::Vertical => b.push("rule::vertical"),
    }
    b.push(&format!("({})", props.rule_thickness));

    if let Some(ref style_name) = props.custom_style_name {
        let is_custom = custom_styles.styles()
            .get(&ThemePaneEnum::Rule)
            .map(|m| m.contains_key(style_name.as_str()))
            .unwrap_or(false);
        if is_custom {
            b.increase_indent();
            b.add_style("styles::rule", &style_name.to_lowercase());
            b.decrease_indent();
        }
    }
}

fn generate_progressbar(b: &mut CodeBuilder, widget: &Widget) {
    let props = &widget.properties;

    b.indent();
    b.push(&format!(
        "progress_bar({:.1}..={:.1}, {:.2})",
        props.progress_min, props.progress_max, props.progress_value
    ));

    b.increase_indent();
    if !matches!(props.progress_length, Length::Fill) {
        b.add_length(props.progress_length);
    }

    if props.progress_girth != iced::widget::progress_bar::ProgressBar::<Theme>::DEFAULT_GIRTH {
        b.dot_method("girth", &format!("{}", props.progress_girth));
    }

    if props.progress_vertical {
        b.dot_method_no_args("vertical");
    }
    b.decrease_indent();
}

fn generate_image(b: &mut CodeBuilder, widget: &Widget) {
    let props = &widget.properties;

    b.indent();
    if props.image_path.is_empty() {
        b.push("image(\"path/to/image.png\")");
    } else {
        b.push(&format!("image(r\"{}\")", props.image_path));
    }

    if !matches!(props.image_fit, ContentFitChoice::Contain) {
        b.add_content_fit(&props.image_fit);
    }

    if !matches!(props.width, Length::Shrink) {
        b.add_width(props.width);
    }

    if !matches!(props.height, Length::Shrink) {
        b.add_height(props.height);
    }
}

fn generate_svg(b: &mut CodeBuilder, widget: &Widget) {
    let props = &widget.properties;

    b.indent();
    if props.svg_path.is_empty() {
        b.push("svg(svg::Handle::from_path(\"path/to/icon.svg\"))");
    } else {
        b.push(&format!("svg(svg::Handle::from_path(r\"{}\"))", props.svg_path));
    }

    if !matches!(props.svg_fit, ContentFitChoice::Contain) {
        b.add_content_fit(&props.svg_fit);
    }

    if !matches!(props.width, Length::Fill) {
        b.add_width(props.width);
    }

    if !matches!(props.height, Length::Shrink) {
        b.add_height(props.height);
    }
}

fn generate_icon(b: &mut CodeBuilder, widget: &Widget) {
    let props = &widget.properties;

    b.indent();
    b.push(&format!("icon::{}()", props.icon_name));
    b.increase_indent();

    if props.icon_size != 24.0 {
        b.add_size(props.icon_size);
    }

    if !matches!(props.width, Length::Shrink) {
        b.add_width(props.width);
    }

    if !matches!(props.height, Length::Shrink) {
        b.add_height(props.height);
    }

    b.decrease_indent();
}

fn generate_button(
    b: &mut CodeBuilder,
    widget: &Widget,
    names: &HashMap<WidgetId, String>,
    custom_styles: &CustomThemes,
    use_self: bool,
    type_system: &TypeSystem,
    view_refs: &[ViewRefInfo],
) {
    let name = names.get(&widget.id).unwrap_or(&"widget".to_string()).clone();

    b.indent();
    b.push("button(");
    b.newline();
    b.increase_indent();

    if widget.children.is_empty() {
        // No child widget — use the text_content property
        b.indent();
        b.push(&format!("text(\"{}\")", widget.properties.text_content));
    } else {
        // Recurse into the single child element
        generate_widget_code(b, &widget.children[0], names, use_self, custom_styles, type_system, view_refs);
    }

    b.newline();
    b.decrease_indent();
    b.indent();
    b.push(")");

    b.increase_indent();
    if widget.properties.button_on_press_enabled {
        b.on_press(&name);
    }

    if widget.properties.button_on_press_with_enabled {
        b.on_press_with(&name);
    }

    if widget.properties.button_on_press_maybe_enabled {
        b.on_press_maybe(&name);
    }

    match widget.properties.custom_style_name.clone() {
        Some(style) => {
            let is_custom = custom_styles.styles()
                .get(&ThemePaneEnum::Button)
                .map(|m| m.contains_key(style.as_str()))
                .unwrap_or(false);
            if is_custom {
                b.add_style("styles::button", &style.to_lowercase());
            } else if ButtonStyleType::all().contains(&style) {
                let bst = ButtonStyleType::get(&style).unwrap();
                match bst {
                    ButtonStyleType::Secondary => b.add_style("button", "secondary"),
                    ButtonStyleType::Success => b.add_style("button", "success"),
                    ButtonStyleType::Danger => b.add_style("button", "danger"),
                    ButtonStyleType::Text => b.add_style("button", "text"),
                    ButtonStyleType::Background => b.add_style("button", "background"),
                    ButtonStyleType::Subtle => b.add_style("button", "subtle"),
                    ButtonStyleType::Primary => {}
                }
            }
        }
        None => {}
    }

    if widget.properties.width != Length::Shrink {
        b.add_width(widget.properties.width);
    }

    if widget.properties.height != Length::Shrink {
        b.add_height(widget.properties.height);
    }

    if widget.properties.padding != (Padding { top: 5.0, bottom: 5.0, right: 10.0, left: 10.0 }) {
        b.add_padding(&widget.properties.padding, widget.properties.padding_mode);
    }

    if widget.properties.clip {
        b.add_clip();
    }
    b.decrease_indent();
}

fn generate_checkbox(
    b: &mut CodeBuilder,
    widget: &Widget,
    names: &HashMap<WidgetId, String>,
    custom_styles: &CustomThemes,
    use_self: bool,
) {
    let name = names.get(&widget.id).unwrap_or(&"widget".to_string()).clone();

    b.indent();
    if use_self {
        b.push(&format!("checkbox(self.{}_checked)", to_snake_case(&name)));
    } else {
        b.push(&format!("checkbox({})", if widget.properties.checkbox_checked { "true" } else { "false" }));
    }
    b.increase_indent();

    b.dot_method("label", &format!("\"{}\"", widget.properties.checkbox_label));
    b.on_toggle(&name);

    if widget.properties.checkbox_size != 16.0 {
        b.add_size(widget.properties.checkbox_spacing);
    }

    if widget.properties.checkbox_spacing != 8.0 {
        b.add_spacing(widget.properties.checkbox_spacing);
    }

    if widget.properties.width != Length::Shrink {
        b.add_width(widget.properties.width);
    }

    if let Some(ref style_name) = widget.properties.custom_style_name {
        let is_custom = custom_styles.styles()
            .get(&ThemePaneEnum::Checkbox)
            .map(|m| m.contains_key(style_name.as_str()))
            .unwrap_or(false);
        if is_custom {
            b.add_style("styles::checkbox", &style_name.to_lowercase());
        }
    }

    b.decrease_indent();
}

fn generate_toggler(
    b: &mut CodeBuilder,
    widget: &Widget,
    names: &HashMap<WidgetId, String>,
    use_self: bool,
) {
    let props = &widget.properties;
    let name = names.get(&widget.id).unwrap_or(&"widget".to_string()).clone();

    b.indent();
    if use_self {
        b.push(&format!("toggler(self.{}_active)", to_snake_case(&name)));
    } else {
        b.push(&format!("toggler({})", if props.toggler_active { "true" } else { "false" }));
    }
    b.newline();
    b.increase_indent();

    b.indent();
    if use_self {
        b.push(&format!(".on_toggle(Message::{}Toggled)", to_pascal_case(&name)));
    } else {
        b.push(".on_toggle(|_| Message::Noop)");
    }

    if props.toggler_size != iced::widget::toggler::Toggler::<Theme>::DEFAULT_SIZE {
        b.add_size(props.toggler_size);
    }

    if props.toggler_spacing != iced::widget::toggler::Toggler::<Theme>::DEFAULT_SIZE / 2.0 {
        b.add_spacing(props.toggler_spacing);
    }

    if !props.toggler_label.is_empty() {
        b.dot_method("label", &format!("\"{}\"", props.toggler_label));
    }

    if !matches!(props.width, Length::Shrink) {
        b.add_width(props.width);
    }

    b.decrease_indent();
}

fn generate_slider(
    b: &mut CodeBuilder,
    widget: &Widget,
    names: &HashMap<WidgetId, String>,
    use_self: bool,
) {
    let props = &widget.properties;
    let name = names.get(&widget.id).unwrap_or(&"widget".to_string()).clone();

    b.indent();
    let value = if use_self {
        format!("self.{}_value", to_snake_case(&name))
    } else {
        format!("{}", props.slider_value)
    };
    let handler = if use_self {
        format!("Message::{}Changed", to_pascal_case(&name))
    } else {
        "|_| Message::Noop".to_string()
    };
    b.push(&format!("slider({:.1}..={:.1}, {}, {})", props.slider_min, props.slider_max, value, handler));

    b.increase_indent();
    if props.slider_step != 1.0 {
        b.add_step(props.slider_step);
    }

    if !matches!(props.slider_height, iced::widget::slider::Slider::<f32, Theme>::DEFAULT_HEIGHT) {
        b.add_height(Length::Fixed(props.slider_height));
    }

    b.decrease_indent();
}

fn generate_verticalslider(
    b: &mut CodeBuilder,
    widget: &Widget,
    names: &HashMap<WidgetId, String>,
    use_self: bool,
) {
    let props = &widget.properties;
    let name = names.get(&widget.id).unwrap_or(&"widget".to_string()).clone();

    b.indent();
    let value = if use_self {
        format!("self.{}_value", to_snake_case(&name))
    } else {
        format!("{}", props.slider_value)
    };
    let handler = if use_self {
        format!("Message::{}Changed", to_pascal_case(&name))
    } else {
        "|_| Message::Noop".to_string()
    };
    b.push(&format!("vertical_slider({:.1}..={:.1}, {}, {})", props.slider_min, props.slider_max, value, handler));

    if props.slider_step != 1.0 {
        b.add_step(props.slider_step);
    }

    if !matches!(props.slider_width, iced::widget::vertical_slider::VerticalSlider::<f32, Theme>::DEFAULT_WIDTH) {
        b.add_height(Length::Fixed(props.slider_width));
    }
}

fn generate_radio(
    b: &mut CodeBuilder,
    widget: &Widget,
    names: &HashMap<WidgetId, String>,
    use_self: bool,
) {
    let props = &widget.properties;
    let name = names.get(&widget.id).unwrap_or(&"widget".to_string()).clone();

    b.indent();
    b.push("column![");
    b.newline();
    b.increase_indent();

    for (i, option) in props.radio_options.iter().enumerate() {
        b.indent();
        let selected = if use_self {
            format!("Some(self.{}_selected)", to_snake_case(&name))
        } else {
            format!("Some({})", props.radio_selected_index)
        };
        let handler = if use_self {
            format!("Message::{}Selected", to_pascal_case(&name))
        } else {
            "|_| Message::Noop".to_string()
        };
        b.push(&format!("radio(\"{}\", {}, {}, {})", option, i, selected, handler));

        if props.radio_size != 16.0 {
            b.add_size(props.radio_size);
        }
        if props.width != Length::Shrink {
            b.add_width(props.width);
        }
        if i < props.radio_options.len() - 1 {
            b.push(",");
        }
        b.newline();
    }

    b.decrease_indent();
    b.indent();
    b.push("]");
}

fn generate_picklist(
    b: &mut CodeBuilder,
    widget: &Widget,
    names: &HashMap<WidgetId, String>,
    use_self: bool,
) {
    let props = &widget.properties;
    let name = names.get(&widget.id).unwrap_or(&"widget".to_string()).clone();

    b.indent();
    b.push("pick_list(");
    b.newline();
    b.increase_indent();

    // Options
    b.indent();
    b.push("vec![");
    for (i, option) in props.picklist_options.iter().enumerate() {
        b.push(&format!("\"{}\".to_string()", option));
        if i < props.picklist_options.len() - 1 {
            b.push(", ");
        }
    }
    b.push("],");
    b.newline();

    b.indent();
    if use_self {
        b.push(&format!("self.{}_selected.clone(),", to_snake_case(&name)));
    } else if let Some(ref selected) = props.picklist_selected {
        b.push(&format!("Some(\"{}\".to_string()),", selected));
    } else {
        b.push("None,");
    }
    b.newline();

    // Handler
    b.indent();
    if use_self {
        b.push(&format!("Message::{}Selected", to_pascal_case(&name)));
    } else {
        b.push("|_| Message::Noop");
    }
    b.newline();

    b.decrease_indent();
    b.indent();
    b.push(")");

    if !props.picklist_placeholder.is_empty() && props.picklist_placeholder != "Choose an option..." {
        b.add_placeholder(&props.picklist_placeholder);
    }

    if !matches!(props.width, Length::Shrink) {
        b.add_width(props.width);
    }

    if props.padding != (Padding { top: 5.0, bottom: 5.0, right: 10.0, left: 10.0 }) {
        b.add_padding(&props.padding, props.padding_mode);
    }
}

fn generate_textinput(
    b: &mut CodeBuilder,
    widget: &Widget,
    names: &HashMap<WidgetId, String>,
    use_self: bool,
) {
    let props = &widget.properties;
    let name = names.get(&widget.id).unwrap_or(&"widget".to_string()).clone();

    b.indent();
    let value_arg = if use_self {
        format!("&self.{}_value", to_snake_case(&name))
    } else {
        "\"\"".to_string()
    };
    b.push(&format!("text_input(\"{}\", {})", props.text_input_placeholder, value_arg));
    b.increase_indent();

    b.on_input(&name);

    if props.text_input_on_submit {
        b.on_submit(&name);
    }

    if props.text_input_on_paste {
        b.on_paste(&name);
    }

    if props.is_secure {
        b.dot_method("secure", "true");
    }

    if props.text_input_font != FontType::Default {
        b.add_font(props.text_input_font);
    }

    b.add_size(props.text_input_size);

    b.add_padding(&Padding::new(props.text_input_padding), PaddingMode::Uniform);

    if props.text_input_line_height != LineHeight::default() {
        b.add_lineheight(&props.text_input_line_height);
    }

    if props.text_input_alignment != ContainerAlignX::Left {
        b.add_align_x(props.text_input_alignment.into());
    }

    if props.text_input_icon_enabled {
        let side = match props.text_input_icon_side {
            TextInputIconSide::Left => "text_input::Side::Left",
            TextInputIconSide::Right => "text_input::Side::Right",
        };
        let size_arg = if props.text_input_icon_size > 0.0 {
            format!("Some({:.1}.into())", props.text_input_icon_size)
        } else {
            "None".to_string()
        };
        b.dot_method("icon", &format!(
            "text_input::Icon {{ font: Font::with_name(\"lucide\"), code_point: '\\u{{{:04X}}}', size: {}, spacing: {:.1}, side: {} }}",
            props.text_input_icon_codepoint, size_arg, props.text_input_icon_spacing, side
        ));
    }

    if !matches!(props.width, Length::Fill) {
        b.add_width(props.width);
    }

    b.decrease_indent();
}

fn generate_combobox(
    b: &mut CodeBuilder,
    widget: &Widget,
    names: &HashMap<WidgetId, String>,
    custom_styles: &CustomThemes,
    use_self: bool,
) {
    let props = &widget.properties;
    let name = names.get(&widget.id).unwrap_or(&"widget".to_string()).clone();

    b.indent();
    b.push("combo_box(");
    b.newline();
    b.increase_indent();

    b.indent();
    if use_self {
        b.push(&format!("&self.{}_state,", to_snake_case(&name)));
    } else {
        b.push("&state,");
    }
    b.newline();

    b.indent();
    b.push(&format!("\"{}\",", props.combobox_placeholder));
    b.newline();

    b.indent();
    if use_self {
        b.push(&format!("Some(&self.{}_value),", to_snake_case(&name)));
    } else {
        if let Some(ref val) = props.combobox_selected {
            b.push(&format!("Some(\"{}\"),", val));
        } else {
            b.push("None,");
        }
    }
    b.newline();

    b.indent();
    if use_self {
        b.push(&format!("Message::{}Selected", to_pascal_case(&name)));
    } else {
        b.push("|_| Message::Noop");
    }
    b.newline();

    b.decrease_indent();
    b.indent();
    b.push(")");

    if props.combobox_use_on_input {
        b.on_input(&name);
    }

    if props.combobox_use_on_option_hovered {
        b.on_option_hovered(&name);
    }

    if props.combobox_use_on_open {
        b.on_open(&name);
    }

    if props.combobox_use_on_close {
        b.on_close(&name);
    }

    if props.combobox_size != 16.0 {
        b.add_size(props.combobox_size);
    }

    if !matches!(props.width, Length::Fill) {
        b.add_width(props.width);
    }

    if props.combobox_icon_enabled {
        let side = match props.combobox_icon_side {
            TextInputIconSide::Left => "text_input::Side::Left",
            TextInputIconSide::Right => "text_input::Side::Right",
        };
        let size_arg = if props.combobox_icon_size > 0.0 {
            format!("Some({:.1}.into())", props.combobox_icon_size)
        } else {
            "None".to_string()
        };
        b.dot_method("icon", &format!(
            "text_input::Icon {{ font: Font::with_name(\"lucide\"), code_point: '\\u{{{:04X}}}', size: {}, spacing: {:.1}, side: {} }}",
            props.combobox_icon_codepoint, size_arg, props.combobox_icon_spacing, side
        ));
    }

    if let Some(ref style) = props.custom_style_name {
        let is_custom = custom_styles.styles()
            .get(&ThemePaneEnum::Combobox)
            .map(|m| m.contains_key(style.as_str()))
            .unwrap_or(false);
        if is_custom {
            b.add_input_style("styles::combo_box", &format!("{}_input_style", style.to_lowercase()));
            b.add_menu_style("styles::combo_box", &format!("{}_menu_style", style.to_lowercase()));
        }
    }
}

fn generate_qrcode(b: &mut CodeBuilder, widget: &Widget) {
    let props = &widget.properties;

    b.indent();
    b.push("qr_code(&self.qr_data)");

    b.increase_indent();
    if props.qrcode_cell_size != 4.0 {
        b.dot_method("cell_size", &format!("{}", props.qrcode_cell_size));
    }
    b.decrease_indent();
}

fn generate_column(
    b: &mut CodeBuilder,
    widget: &Widget,
    names: &HashMap<WidgetId, String>,
    custom_styles: &CustomThemes,
    use_self: bool,
    type_system: &TypeSystem,
    view_refs: &[ViewRefInfo],
) {
    b.indent();
    b.push("column![");
    b.newline();

    b.increase_indent();
    if use_self {
        if widget.children.is_empty() {
            b.indent();
            b.push("text(\"Column Item\")");
            b.newline();
        } else {
            for (i, child) in widget.children.iter().enumerate() {
                generate_widget_code(b, child, names, use_self, custom_styles, type_system, view_refs);
                if i < widget.children.len() - 1 {
                    b.push(",");
                }
                b.newline();
            }
        }
    } else {
        b.indent();
        b.push("// child widgets");
        b.newline();
    }
    b.decrease_indent();
    b.indent();
    b.push("]");

    if widget.properties.spacing != 0.0 {
        b.add_spacing(widget.properties.spacing);
    }

    if widget.properties.width != Length::Shrink {
        b.add_width(widget.properties.width);
    }

    if let Some(max_w) = widget.properties.max_width {
        b.add_max_width(max_w);
    }

    if widget.properties.height != Length::Shrink {
        b.add_height(widget.properties.height);
    }

    if widget.properties.align_items != Alignment::Start {
        b.add_align_x(widget.properties.align_items);
    }

    if widget.properties.padding != Padding::ZERO {
        b.add_padding(&widget.properties.padding, widget.properties.padding_mode);
    }

    if widget.properties.clip {
        b.add_clip();
    }
}

fn generate_row(
    b: &mut CodeBuilder,
    widget: &Widget,
    names: &HashMap<WidgetId, String>,
    custom_styles: &CustomThemes,
    use_self: bool,
    type_system: &TypeSystem,
    view_refs: &[ViewRefInfo],
) {
    let props = &widget.properties;

    b.indent();
    b.push("row![");
    b.newline();
    b.increase_indent();

    if use_self {
        if widget.children.is_empty() {
            b.indent();
            b.push("text(\"Row Item\")");
            b.newline();
        } else {
            for (i, child) in widget.children.iter().enumerate() {
                generate_widget_code(b, child, names, use_self, custom_styles, type_system, view_refs);
                if i < widget.children.len() - 1 {
                    b.push(",");
                }
                b.newline();
            }
        }
    } else {
        b.indent();
        b.push("// child widgets");
        b.newline();
    }

    b.decrease_indent();
    b.indent();
    b.push("]");

    if props.spacing != 0.0 {
        b.add_spacing(props.spacing);
    }

    if !matches!(props.align_items, Alignment::Start) {
        b.add_align_y(props.align_items);
    }

    if !matches!(props.width, Length::Shrink) {
        b.add_width(props.width);
    }

    if !matches!(props.height, Length::Shrink) {
        b.add_height(props.height);
    }

    if !matches!(props.padding, Padding::ZERO) {
        b.add_padding(&props.padding, props.padding_mode);
    }

    if props.is_wrapping_row {
        b.dot_method_no_args("wrap");

        if props.match_horizontal_spacing {
            b.dot_method("vertical_spacing", &format!("{:.1}", props.wrapping_vertical_spacing));
        }

        if !matches!(props.wrapping_align_x, ContainerAlignX::Left) {
            b.add_align_x(props.wrapping_align_x.into());
        }
    }

    if props.clip {
        b.add_clip();
    }
}

fn generate_container(
    b: &mut CodeBuilder,
    widget: &Widget,
    names: &HashMap<WidgetId, String>,
    custom_styles: &CustomThemes,
    use_self: bool,
    type_system: &TypeSystem,
    view_refs: &[ViewRefInfo],
) {
    let props = &widget.properties;

    b.indent();
    b.push("container(");
    b.newline();
    b.increase_indent();

    if use_self {
        if widget.children.is_empty() {
            b.indent();
            b.push("text(\"Container Content\")");
        } else {
            for child in &widget.children {
                generate_widget_code(b, child, names, use_self, custom_styles, type_system, view_refs);
            }
        }
    } else {
        b.indent();
        b.push("// child widgets");
    }

    b.newline();
    b.decrease_indent();
    b.indent();
    b.push(")");

    if let Some(ref id) = props.widget_id {
        if !id.is_empty() {
            b.add_id(id);
        }
    }

    // Sizing
    match props.container_sizing_mode {
        ContainerSizingMode::Manual => {
            // Shrink = "use iced's fluid default" → skip emission.
            // Any other value (Fill, Fixed, FillPortion) is explicit → always emit.
            if !matches!(props.width, Length::Shrink) {
                b.add_width(props.width);
            }
            if !matches!(props.height, Length::Shrink) {
                b.add_height(props.height);
            }
            if !matches!(props.align_x, ContainerAlignX::Left) {
                b.add_align_x(props.align_x.into());
            }
            if !matches!(props.align_y, ContainerAlignY::Top) {
                b.add_align_y(props.align_y.into());
            }
        }
        ContainerSizingMode::CenterX => {
            b.add_center_x(props.container_center_length);
        }
        ContainerSizingMode::CenterY => {
            b.add_center_y(props.container_center_length);
        }
        ContainerSizingMode::Center => {
            b.add_center(props.container_center_length);
        }
    }

    if let Some(max_width) = props.max_width {
        b.add_max_width(max_width);
    }

    if let Some(max_height) = props.max_height {
        b.add_max_height(max_height);
    }

    if props.padding != Padding::ZERO {
        b.add_padding(&props.padding, props.padding_mode);
    }

    if props.clip {
        b.add_clip();
    }

    // Style generation — only when a named style has been explicitly assigned
    if let Some(ref style_name) = props.custom_style_name {
        let snake = to_snake_case(style_name);
        let is_custom = custom_styles.styles()
            .get(&ThemePaneEnum::Container)
            .map(|m| m.contains_key(style_name.as_str()))
            .unwrap_or(false);
        if is_custom {
            b.add_style("styles::container", &snake);
        } else {
            // Built-in iced container style (e.g. bordered_box, rounded_box)
            b.add_style("container", &snake);
        }
    }
}

fn generate_scrollable(
    b: &mut CodeBuilder,
    widget: &Widget,
    names: &HashMap<WidgetId, String>,
    custom_styles: &CustomThemes,
    use_self: bool,
    type_system: &TypeSystem,
    view_refs: &[ViewRefInfo],
) {
    let props = &widget.properties;

    b.indent();
    b.push("scrollable(");
    b.newline();
    b.increase_indent();

    if use_self {
        if widget.children.is_empty() {
            b.indent();
            b.push("column![");
            b.newline();
            b.increase_indent();
            for i in 1..=10 {
                b.indent();
                b.push(&format!("text(\"Scrollable Item {}\")", i));
                if i < 10 {
                    b.push(",");
                }
                b.newline();
            }
            b.decrease_indent();
            b.indent();
            b.push("]");
        } else {
            for child in &widget.children {
                generate_widget_code(b, child, names, use_self, custom_styles, type_system, view_refs);
            }
        }
    } else {
        b.indent();
        b.push("// child widgets");
    }

    b.newline();
    b.decrease_indent();
    b.indent();
    b.push(")");
    b.increase_indent();

    let is_default_dir = matches!(
        props.scroll_dir,
        iced::widget::scrollable::Direction::Vertical(_)
    );

    if !is_default_dir {
        let dir_arg = match props.scroll_dir {
            iced::widget::scrollable::Direction::Horizontal(_) => {
                "scrollable::Direction::Horizontal(scrollable::Scrollbar::default())"
            }
            iced::widget::scrollable::Direction::Both { .. } => {
                "scrollable::Direction::Both { vertical: scrollable::Scrollbar::default(), horizontal: scrollable::Scrollbar::default() }"
            }
            _ => "",
        };
        if !dir_arg.is_empty() {
            b.dot_method("direction", dir_arg);
        }
    }

    if !matches!(props.width, Length::Shrink) {
        b.add_width(props.width);
    }

    if !matches!(props.height, Length::Shrink) {
        b.add_height(props.height);
    }

    b.decrease_indent();
}

fn generate_stack(
    b: &mut CodeBuilder,
    widget: &Widget,
    names: &HashMap<WidgetId, String>,
    custom_styles: &CustomThemes,
    use_self: bool,
    type_system: &TypeSystem,
    view_refs: &[ViewRefInfo],
) {
    let props = &widget.properties;

    b.indent();
    b.push("stack![");
    b.newline();
    b.increase_indent();

    if use_self {
        if widget.children.is_empty() {
            b.indent();
            b.push("text(\"Layer 1\"),");
            b.newline();
            b.indent();
            b.push("text(\"Layer 2\"),");
        } else {
            for (i, child) in widget.children.iter().enumerate() {
                generate_widget_code(b, child, names, use_self, custom_styles, type_system, view_refs);
                if i < widget.children.len() - 1 {
                    b.push(",");
                }
                b.newline();
            }
        }
    } else {
        b.indent();
        b.push("// child widgets");
    }
    b.newline();

    b.decrease_indent();
    b.indent();
    b.push("]");

    if !matches!(props.width, Length::Fill) {
        b.add_width(props.width);
    }
    if !matches!(props.height, Length::Fill) {
        b.add_height(props.height);
    }

}

fn generate_mousearea(
    b: &mut CodeBuilder,
    widget: &Widget,
    names: &HashMap<WidgetId, String>,
    custom_styles: &CustomThemes,
    use_self: bool,
    type_system: &TypeSystem,
    view_refs: &[ViewRefInfo],
) {
    let props = &widget.properties;
    let name = names.get(&widget.id).unwrap_or(&"widget".to_string()).clone();

    b.indent();
    b.push("mouse_area(");
    b.newline();

    b.increase_indent();
    if use_self {
        if !widget.children.is_empty() {
            generate_widget_code(b, &widget.children[0], names, use_self, custom_styles, type_system, view_refs);
        }
    } else {
        b.indent();
        b.push("// child widgets");
        b.newline();
    }
    b.decrease_indent();

    b.indent();
    b.push(")");
    b.increase_indent();

    if props.mousearea_on_press { b.on_press(&name); }
    if props.mousearea_on_release { b.on_release(&name); }
    if props.mousearea_on_double_click { b.on_double_click(&name); }
    if props.mousearea_on_right_press { b.on_right_press(&name); }
    if props.mousearea_on_right_release { b.on_right_release(&name); }
    if props.mousearea_on_middle_press { b.on_middle_press(&name); }
    if props.mousearea_on_middle_release { b.on_middle_release(&name); }
    if props.mousearea_on_scroll { b.on_scroll(&name); }
    if props.mousearea_on_enter { b.on_enter(&name); }
    if props.mousearea_on_move { b.on_move(&name); }
    if props.mousearea_on_exit { b.on_exit(&name); }

    if let Some(interaction) = props.mousearea_interaction {
        b.on_mouse_interaction(&interaction);
    }

    b.decrease_indent();
}

fn generate_tooltip(
    b: &mut CodeBuilder,
    widget: &Widget,
    names: &HashMap<WidgetId, String>,
    custom_styles: &CustomThemes,
    use_self: bool,
    type_system: &TypeSystem,
    view_refs: &[ViewRefInfo],
) {
    let props = &widget.properties;

    b.indent();
    b.push("tooltip(");
    b.newline();
    b.increase_indent();

    if use_self {
        if let Some(host) = widget.children.get(0) {
            generate_widget_code(b, host, names, use_self, custom_styles, type_system, view_refs);
        } else {
            b.indent();
            b.push("text(\"Hover me\")");
        }
        b.push(",");
        b.newline();

        if let Some(content) = widget.children.get(1) {
            generate_widget_code(b, content, names, use_self, custom_styles, type_system, view_refs);
        } else {
            b.indent();
            b.push(&format!("text(\"{}\")", props.tooltip_text));
        }
    } else {
        b.indent();
        b.push("// host widget");
        b.push(",");
        b.newline();
        b.indent();
        b.push("// tooltip content");
    }
    b.push(",");
    b.newline();

    b.indent();
    let pos = match props.tooltip_position {
        TooltipPosition::Top => "tooltip::Position::Top",
        TooltipPosition::Bottom => "tooltip::Position::Bottom",
        TooltipPosition::Left => "tooltip::Position::Left",
        TooltipPosition::Right => "tooltip::Position::Right",
        TooltipPosition::FollowCursor => "tooltip::Position::FollowCursor",
    };
    b.push(pos);
    b.newline();

    b.decrease_indent();
    b.indent();
    b.push(")");
}

fn generate_grid(
    b: &mut CodeBuilder,
    widget: &Widget,
    names: &HashMap<WidgetId, String>,
    custom_styles: &CustomThemes,
    use_self: bool,
    type_system: &TypeSystem,
    view_refs: &[ViewRefInfo],
) {
    let props = &widget.properties;

    b.indent();
    b.push("grid(vec![");
    b.newline();
    b.increase_indent();

    if use_self {
        if widget.children.is_empty() {
            b.indent();
            b.push("text(\"Grid Cell\").into(),");
        } else {
            for (i, child) in widget.children.iter().enumerate() {
                generate_widget_code(b, child, names, use_self, custom_styles, type_system, view_refs);
                b.push(".into()");
                if i < widget.children.len() - 1 {
                    b.push(",");
                }
                b.newline();
            }
        }
    } else {
        b.indent();
        b.push("// child widgets");
    }
    b.newline();

    b.decrease_indent();
    b.indent();
    b.push("])");

    b.increase_indent();
    if props.grid_use_fluid {
        b.dot_method("fluid", &format!("{:.1}", props.grid_fluid_max_width));
    } else if props.grid_columns != 3 {
        b.dot_method("columns", &format!("{}", props.grid_columns));
    }
    if props.grid_spacing != 0.0 {
        b.add_spacing(props.grid_spacing);
    }
    if let Some(w) = props.grid_fixed_width {
        b.dot_method("width", &format!("{:.1}", w));
    }
    b.decrease_indent();
}

fn generate_themer(
    b: &mut CodeBuilder,
    widget: &Widget,
    names: &HashMap<WidgetId, String>,
    custom_styles: &CustomThemes,
    use_self: bool,
    type_system: &TypeSystem,
    view_refs: &[ViewRefInfo],
) {
    let props = &widget.properties;
    let name = names.get(&widget.id).unwrap_or(&"widget".to_string()).clone();

    b.indent();
    b.push("themer(");
    if let Some(theme) = &props.themer_theme {
        let variant = match theme {
            Theme::Light => "Light",
            Theme::Dark => "Dark",
            Theme::Dracula => "Dracula",
            Theme::Nord => "Nord",
            Theme::CatppuccinFrappe => "CatppuccinFrappe",
            Theme::CatppuccinLatte => "CatppuccinLatte",
            Theme::CatppuccinMacchiato => "CatppuccinMacchiato",
            Theme::CatppuccinMocha => "CatppuccinMocha",
            Theme::Ferra => "Ferra",
            Theme::GruvboxDark => "GruvboxDark",
            Theme::GruvboxLight => "GruvboxLight",
            Theme::KanagawaDragon => "KanagawaDragon",
            Theme::KanagawaLotus => "KanagawaLotus",
            Theme::KanagawaWave => "KanagawaWave",
            Theme::Moonfly => "Moonfly",
            Theme::Nightfly => "Nightfly",
            Theme::Oxocarbon => "Oxocarbon",
            Theme::SolarizedDark => "SolarizedDark",
            Theme::SolarizedLight => "SolarizedLight",
            Theme::TokyoNight => "TokyoNight",
            Theme::TokyoNightLight => "TokyoNightLight",
            Theme::TokyoNightStorm => "TokyoNightStorm",
            Theme::Custom(_) => &name,
        };
        b.push(&format!("Some(Theme::{})", variant));
    } else {
        b.push("None");
    }
    b.push(", ");
    b.newline();
    b.increase_indent();

    if use_self {
        if widget.children.is_empty() {
            b.indent();
            b.push("container(text(\"Themed content\"))");
        } else {
            for child in &widget.children {
                generate_widget_code(b, child, names, use_self, custom_styles, type_system, view_refs);
            }
        }
    } else {
        b.indent();
        b.push("// child widgets");
    }

    b.newline();
    b.decrease_indent();
    b.indent();
    b.push(")");
}

fn generate_pin(
    b: &mut CodeBuilder,
    widget: &Widget,
    names: &HashMap<WidgetId, String>,
    custom_styles: &CustomThemes,
    use_self: bool,
    type_system: &TypeSystem,
    view_refs: &[ViewRefInfo],
) {
    let props = &widget.properties;

    b.indent();
    b.push("pin(");
    b.newline();
    b.increase_indent();

    if use_self {
        if !widget.children.is_empty() {
            generate_widget_code(b, &widget.children[0], names, use_self, custom_styles, type_system, view_refs);
        } else {
            b.indent();
            b.push("text(\"Pinned Content\")");
        }
    } else {
        b.indent();
        b.push("// child widget");
    }

    b.newline();
    b.decrease_indent();
    b.indent();
    b.push(")");
    b.increase_indent();

    if props.pin_point != Point::ORIGIN {
        if props.pin_point.x != 0.0 && props.pin_point.y != 0.0 {
            b.dot_method("position", &format!("Point::new({:.1}, {:.1})", props.pin_point.x, props.pin_point.y));
        } else if props.pin_point.x != 0.0 {
            b.dot_method("x", &format!("{:.1}", props.pin_point.x));
        } else {
            b.dot_method("y", &format!("{:.1}", props.pin_point.y));
        }
    }

    if !matches!(props.width, Length::Fill) {
        b.add_width(props.width);
    }

    if !matches!(props.height, Length::Fill) {
        b.add_height(props.height);
    }

    b.decrease_indent();
}

fn generate_markdown(
    b: &mut CodeBuilder,
    widget: &Widget,
    names: &HashMap<WidgetId, String>,
    use_self: bool,
) {
    let props = &widget.properties;
    let name = names.get(&widget.id).unwrap_or(&"widget".to_string()).clone();

    b.indent();
    if use_self {
        if props.markdown_text_size != 16.0 {
            b.push(&format!(
                "markdown::view(&self.{}_items, markdown::Settings::with_text_size({}, &self.theme))",
                to_snake_case(&name),
                props.markdown_text_size
            ));
        } else {
            b.push(&format!(
                "markdown::view(&self.{}_items, &self.theme)",
                to_snake_case(&name)
            ));
        }
        b.increase_indent();
        b.dot_method("map", &format!("Message::{}LinkClicked", to_pascal_case(&name)));
        b.decrease_indent();
    } else {
        if props.markdown_text_size != 16.0 {
            b.push(&format!(
                "markdown::view(&items, markdown::Settings::with_text_size({}, &self.theme))",
                props.markdown_text_size
            ));
        } else {
            b.push("markdown::view(&items, &self.theme)");
        }
        b.increase_indent();
        b.dot_method("map", "Message::LinkClicked");
        b.decrease_indent();
    }
}

fn generate_table(
    b: &mut CodeBuilder,
    widget: &Widget,
    names: &HashMap<WidgetId, String>,
    use_self: bool,
    type_system: &TypeSystem,
) {
    let props = &widget.properties;
    let name = names
        .get(&widget.id)
        .unwrap_or(&"widget".to_string())
        .clone();

    let struct_def = props.table_referenced_struct
        .and_then(|id| type_system.get_struct(id));

    b.indent();
    if let Some(sdef) = struct_def {
        // Build column vec inline
        b.push("table(");
        b.newline();
        b.increase_indent();

        // Columns
        b.indent();
        b.push("vec![");
        b.newline();
        b.increase_indent();

        for field in &sdef.fields {
            b.indent();
            // Enums don't implement IntoFragment, so call .to_string() on them
            let cell_expr = match &field.field_type {
                crate::enum_builder::FieldType::CustomEnum(_) => {
                    format!("text(row.{}.to_string())", field.name)
                }
                _ => format!("text(&row.{})", field.name),
            };
            if props.table_bold_headers {
                b.push(&format!(
                    "table::column(text(\"{}\").font(Font {{ weight: font::Weight::Bold, ..Font::DEFAULT }}), |row: &{}| {}),",
                    field.name, sdef.name, cell_expr,
                ));
            } else {
                b.push(&format!(
                    "table::column(\"{}\", |row: &{}| {}),",
                    field.name, sdef.name, cell_expr,
                ));
            }
            b.newline();
        }
        b.decrease_indent();
        b.indent();
        b.push("],");
        b.newline();

        // Rows
        b.indent();
        if use_self {
            b.push(&format!("&self.{}_rows,", to_snake_case(&name)));
        } else {
            b.push("&rows,");
        }
        b.newline();

        b.decrease_indent();
        b.indent();
        b.push(")");

        b.increase_indent();
        if props.table_padding_x != 10.0 {
            b.dot_method("padding_x", &format!("{:.1}", props.table_padding_x));
        }
        if props.table_padding_y != 5.0 {
            b.dot_method("padding_y", &format!("{:.1}", props.table_padding_y));
        }
        if props.table_separator_x != 1.0 {
            b.dot_method("separator_x", &format!("{:.1}", props.table_separator_x));
        }
        if props.table_separator_y != 1.0 {
            b.dot_method("separator_y", &format!("{:.1}", props.table_separator_y));
        }
        b.decrease_indent();
    } else {
        b.push("// table: no struct selected");
        b.newline();
        b.indent();
        b.push("text(\"Table: Select a struct type\")");
    }
}

fn generate_view_reference(
    b: &mut CodeBuilder,
    widget: &Widget,
    view_refs: &[ViewRefInfo],
) {
    if let Some(vr) = view_refs.iter().find(|vr| vr.widget_id == widget.id) {
        let variant = to_pascal_case(&vr.field_name);
        b.indent();
        b.push(&format!(
            "self.{}.view().map(|msg| Message::ViewMessages(ViewMessages::{}(msg)))",
            vr.field_name, variant
        ));
    } else {
        b.indent();
        b.push("text(\"ViewReference: not resolved\")");
    }
}

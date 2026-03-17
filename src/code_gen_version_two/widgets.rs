use super::builder::{CodeBuilder, format_length, to_pascal_case, to_snake_case};
use super::events::ViewRefInfo;
use crate::data_structures::types::type_implementations::*;
use crate::data_structures::types::types::{Widget, WidgetId, WidgetType};
use crate::enum_builder::TypeSystem;
use crate::views::theme_and_stylefn_builder::{CustomThemes, ThemePaneEnum};
use iced::widget::text::LineHeight;
use iced::{Alignment, Length, Padding, Point, Theme};
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
        WidgetType::Button => generate_button(
            b,
            widget,
            names,
            custom_styles,
            use_self,
            type_system,
            view_refs,
        ),
        WidgetType::Checkbox => generate_checkbox(b, widget, names, custom_styles, use_self),
        WidgetType::Collapsible => generate_collapsible(
            b,
            widget,
            names,
            custom_styles,
            use_self,
            type_system,
            view_refs,
        ),
        WidgetType::CollapsibleGroup => generate_collapsible_group(
            b,
            widget,
            names,
            custom_styles,
            use_self,
            type_system,
            view_refs,
        ),
        WidgetType::GenericOverlay => generate_generic_overlay(
            b,
            widget,
            names,
            custom_styles,
            use_self,
            type_system,
            view_refs,
        ),
        WidgetType::DatePicker => generate_date_picker(
            b,
            widget,
            names,
            custom_styles,
            use_self,
            type_system,
            view_refs,
        ),
        WidgetType::Column => generate_column(
            b,
            widget,
            names,
            custom_styles,
            use_self,
            type_system,
            view_refs,
        ),
        WidgetType::ComboBox => generate_combobox(b, widget, names, custom_styles, use_self),
        WidgetType::Container => generate_container(
            b,
            widget,
            names,
            custom_styles,
            use_self,
            type_system,
            view_refs,
        ),
        WidgetType::Image => generate_image(b, widget),
        WidgetType::Markdown => generate_markdown(b, widget, names, use_self),
        WidgetType::MouseArea => generate_mousearea(
            b,
            widget,
            names,
            custom_styles,
            use_self,
            type_system,
            view_refs,
        ),
        WidgetType::PickList => generate_picklist(b, widget, names, custom_styles, use_self),
        WidgetType::Pin => generate_pin(
            b,
            widget,
            names,
            custom_styles,
            use_self,
            type_system,
            view_refs,
        ),
        WidgetType::ProgressBar => generate_progressbar(b, widget, custom_styles),
        WidgetType::QRCode => generate_qrcode(b, widget),
        WidgetType::Radio => generate_radio(b, widget, names, custom_styles, use_self),
        WidgetType::Row => generate_row(
            b,
            widget,
            names,
            custom_styles,
            use_self,
            type_system,
            view_refs,
        ),
        WidgetType::Rule => generate_rule(b, widget, custom_styles),
        WidgetType::Scrollable => generate_scrollable(
            b,
            widget,
            names,
            custom_styles,
            use_self,
            type_system,
            view_refs,
        ),
        WidgetType::Slider => generate_slider(b, widget, names, custom_styles, use_self),
        WidgetType::Space => generate_space(b, widget),
        WidgetType::Stack => generate_stack(
            b,
            widget,
            names,
            custom_styles,
            use_self,
            type_system,
            view_refs,
        ),
        WidgetType::Svg => generate_svg(b, widget),
        WidgetType::Text => generate_text(b, widget),
        WidgetType::TextInput => generate_textinput(b, widget, names, custom_styles, use_self),
        WidgetType::Table => generate_table(b, widget, names, use_self, type_system),
        WidgetType::Themer => generate_themer(
            b,
            widget,
            names,
            custom_styles,
            use_self,
            type_system,
            view_refs,
        ),
        WidgetType::Grid => generate_grid(
            b,
            widget,
            names,
            custom_styles,
            use_self,
            type_system,
            view_refs,
        ),
        WidgetType::Toggler => generate_toggler(b, widget, names, custom_styles, use_self),
        WidgetType::Tooltip => generate_tooltip(
            b,
            widget,
            names,
            custom_styles,
            use_self,
            type_system,
            view_refs,
        ),
        WidgetType::VerticalSlider => {
            generate_verticalslider(b, widget, names, custom_styles, use_self)
        }
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
        let is_custom = custom_styles
            .styles()
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

fn has_custom_style(custom_styles: &CustomThemes, pane: ThemePaneEnum, style_name: &str) -> bool {
    custom_styles
        .styles()
        .get(&pane)
        .map(|styles| styles.contains_key(style_name))
        .unwrap_or(false)
}

fn resolve_custom_style_fn(
    custom_styles: &CustomThemes,
    pane: ThemePaneEnum,
    module: &str,
    style_name: &str,
) -> Option<String> {
    has_custom_style(custom_styles, pane, style_name)
        .then(|| format!("{}::{}", module, style_name.to_lowercase()))
}

fn resolve_checkbox_style_fn(custom_styles: &CustomThemes, style_name: &str) -> Option<String> {
    resolve_custom_style_fn(
        custom_styles,
        ThemePaneEnum::Checkbox,
        "styles::checkbox",
        style_name,
    )
    .or_else(|| match style_name {
        "Primary" => Some("checkbox::primary".to_string()),
        "Secondary" => Some("checkbox::secondary".to_string()),
        "Success" => Some("checkbox::success".to_string()),
        "Danger" => Some("checkbox::danger".to_string()),
        _ => None,
    })
}

fn resolve_slider_style_fn(custom_styles: &CustomThemes, style_name: &str) -> Option<String> {
    resolve_custom_style_fn(
        custom_styles,
        ThemePaneEnum::Slider,
        "styles::slider",
        style_name,
    )
    .or_else(|| match style_name {
        "Default" => Some("slider::default".to_string()),
        _ => None,
    })
}

fn resolve_progress_bar_style_fn(custom_styles: &CustomThemes, style_name: &str) -> Option<String> {
    resolve_custom_style_fn(
        custom_styles,
        ThemePaneEnum::Progressbar,
        "styles::progress_bar",
        style_name,
    )
    .or_else(|| match style_name {
        "Primary" => Some("progress_bar::primary".to_string()),
        "Secondary" => Some("progress_bar::secondary".to_string()),
        "Success" => Some("progress_bar::success".to_string()),
        "Warning" => Some("progress_bar::warning".to_string()),
        "Danger" => Some("progress_bar::danger".to_string()),
        _ => None,
    })
}

fn resolve_radio_style_fn(custom_styles: &CustomThemes, style_name: &str) -> Option<String> {
    resolve_custom_style_fn(
        custom_styles,
        ThemePaneEnum::Radio,
        "styles::radio",
        style_name,
    )
    .or_else(|| match style_name {
        "Default" => Some("radio::default".to_string()),
        _ => None,
    })
}

fn resolve_toggler_style_fn(custom_styles: &CustomThemes, style_name: &str) -> Option<String> {
    resolve_custom_style_fn(
        custom_styles,
        ThemePaneEnum::Toggler,
        "styles::toggler",
        style_name,
    )
    .or_else(|| match style_name {
        "Default" => Some("toggler::default".to_string()),
        _ => None,
    })
}

fn resolve_pick_list_style_fn(custom_styles: &CustomThemes, style_name: &str) -> Option<String> {
    resolve_custom_style_fn(
        custom_styles,
        ThemePaneEnum::Picklist,
        "styles::pick_list",
        style_name,
    )
    .or_else(|| match style_name {
        "Default" => Some("pick_list::default".to_string()),
        _ => None,
    })
}

fn resolve_collapsible_style_fn(style_name: &str) -> Option<String> {
    match style_name {
        "Default" => Some("widgets::collapsible::default".to_string()),
        "Primary" => Some("widgets::collapsible::primary".to_string()),
        "Success" => Some("widgets::collapsible::success".to_string()),
        "Danger" => Some("widgets::collapsible::danger".to_string()),
        "Warning" => Some("widgets::collapsible::warning".to_string()),
        _ => None,
    }
}

fn resolve_generic_overlay_style_fn(style_name: &str) -> Option<String> {
    match style_name {
        "Primary" => Some("widgets::generic_overlay::primary".to_string()),
        "Success" => Some("widgets::generic_overlay::success".to_string()),
        "Danger" => Some("widgets::generic_overlay::danger".to_string()),
        "Warning" => Some("widgets::generic_overlay::warning".to_string()),
        "Blank" => Some("widgets::generic_overlay::blank".to_string()),
        _ => None,
    }
}

fn resolve_generic_overlay_trigger_style_fn(
    custom_styles: &CustomThemes,
    style_name: &str,
) -> Option<String> {
    let is_custom = custom_styles
        .styles()
        .get(&ThemePaneEnum::Button)
        .map(|styles| styles.contains_key(style_name))
        .unwrap_or(false);

    if is_custom {
        return Some(format!("styles::button_{}", style_name.to_lowercase()));
    }

    ButtonStyleType::get(style_name).map(|style| match style {
        ButtonStyleType::Primary => "iced::widget::button::primary".to_string(),
        ButtonStyleType::Secondary => "iced::widget::button::secondary".to_string(),
        ButtonStyleType::Success => "iced::widget::button::success".to_string(),
        ButtonStyleType::Danger => "iced::widget::button::danger".to_string(),
        ButtonStyleType::Text => "iced::widget::button::text".to_string(),
        ButtonStyleType::Background => "iced::widget::button::background".to_string(),
        ButtonStyleType::Subtle => "iced::widget::button::subtle".to_string(),
    })
}

fn generic_overlay_position_code(position: GenericOverlayPosition) -> &'static str {
    match position {
        GenericOverlayPosition::Top => "widgets::generic_overlay::Position::Top",
        GenericOverlayPosition::Bottom => "widgets::generic_overlay::Position::Bottom",
        GenericOverlayPosition::Left => "widgets::generic_overlay::Position::Left",
        GenericOverlayPosition::Right => "widgets::generic_overlay::Position::Right",
    }
}

fn generic_overlay_alignment_code(alignment: ContainerAlignX) -> &'static str {
    match alignment {
        ContainerAlignX::Left => "iced::Alignment::Start",
        ContainerAlignX::Center => "iced::Alignment::Center",
        ContainerAlignX::Right => "iced::Alignment::End",
    }
}

fn generic_overlay_position_mode_code(mode: GenericOverlayPositionMode) -> &'static str {
    match mode {
        GenericOverlayPositionMode::Outside => "widgets::generic_overlay::PositionMode::Outside",
        GenericOverlayPositionMode::Inside => "widgets::generic_overlay::PositionMode::Inside",
    }
}

fn generic_overlay_resize_mode_code(mode: GenericOverlayResizeMode) -> &'static str {
    match mode {
        GenericOverlayResizeMode::None => "widgets::generic_overlay::ResizeMode::None",
        GenericOverlayResizeMode::Always => "widgets::generic_overlay::ResizeMode::Always",
        GenericOverlayResizeMode::WithCtrl => "widgets::generic_overlay::ResizeMode::WithCtrl",
    }
}

fn style_condition_code(widget: &Widget) -> (bool, Option<String>) {
    let uses_condition = widget.properties.style_condition_field.is_some();
    let condition = match (
        &widget.properties.style_condition_field,
        &widget.properties.style_condition_value,
    ) {
        (Some(field), Some(value)) if !field.is_empty() && !value.is_empty() => {
            Some(format!("self.{} == {}", field, value))
        }
        _ => None,
    };

    (uses_condition, condition)
}

fn escape_string_literal(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

fn generate_progressbar(b: &mut CodeBuilder, widget: &Widget, custom_styles: &CustomThemes) {
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

    let default_style = props
        .custom_style_name
        .as_deref()
        .and_then(|style_name| resolve_progress_bar_style_fn(custom_styles, style_name));
    let alternate_style = props
        .active_style_name
        .as_deref()
        .and_then(|style_name| resolve_progress_bar_style_fn(custom_styles, style_name));
    let (uses_condition, condition) = style_condition_code(widget);

    match (&default_style, &alternate_style, &condition, uses_condition) {
        (_, Some(alternate_fn), Some(cond), true) => {
            let default_fn = default_style.as_deref().unwrap_or("progress_bar::primary");
            b.dot_method("style", &format!(
                "{{ let _use_alternate = {}; move |theme: &Theme| if _use_alternate {{ {}(theme) }} else {{ {}(theme) }} }}",
                cond, alternate_fn, default_fn
            ));
        }
        (_, Some(alternate_fn), _, false) => {
            b.dot_method("style", alternate_fn);
        }
        (Some(default_fn), _, _, _) => {
            b.dot_method("style", default_fn);
        }
        _ => {}
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
        b.push(&format!(
            "svg(svg::Handle::from_path(r\"{}\"))",
            props.svg_path
        ));
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
    let name = names
        .get(&widget.id)
        .unwrap_or(&"widget".to_string())
        .clone();
    let has_custom_name = !widget.properties.widget_name.trim().is_empty();

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
        generate_widget_code(
            b,
            &widget.children[0],
            names,
            use_self,
            custom_styles,
            type_system,
            view_refs,
        );
    }

    b.newline();
    b.decrease_indent();
    b.indent();
    b.push(")");

    b.increase_indent();
    if widget.properties.button_on_press_enabled {
        let variant = CodeBuilder::msg_variant(&name, "Pressed", has_custom_name);
        b.dot_method("on_press", &format!("Message::{}", variant));
    }

    if widget.properties.button_on_press_with_enabled {
        let variant = CodeBuilder::msg_variant(&name, "Pressed", has_custom_name);
        b.newline();
        b.indent();
        b.push(&format!(".on_press_with(|| Message::{})", variant));
    }

    if widget.properties.button_on_press_maybe_enabled {
        let variant = CodeBuilder::msg_variant(&name, "Pressed", has_custom_name);
        b.newline();
        b.indent();
        b.push(&format!(".on_press_maybe(Some(Message::{}))", variant));
    }

    let resolve_btn_style = |style: &str| -> Option<String> {
        let is_custom = custom_styles
            .styles()
            .get(&ThemePaneEnum::Button)
            .map(|m| m.contains_key(style))
            .unwrap_or(false);
        if is_custom {
            Some(format!("styles::button_{}", style.to_lowercase()))
        } else if let Some(bst) = ButtonStyleType::get(style) {
            match bst {
                ButtonStyleType::Secondary => Some("button::secondary".to_string()),
                ButtonStyleType::Success => Some("button::success".to_string()),
                ButtonStyleType::Danger => Some("button::danger".to_string()),
                ButtonStyleType::Text => Some("button::text".to_string()),
                ButtonStyleType::Background => Some("button::background".to_string()),
                ButtonStyleType::Subtle => Some("button::subtle".to_string()),
                ButtonStyleType::Primary => None,
            }
        } else {
            None
        }
    };

    let default_style = widget
        .properties
        .custom_style_name
        .as_deref()
        .and_then(resolve_btn_style);
    let active_style = widget
        .properties
        .active_style_name
        .as_deref()
        .and_then(resolve_btn_style);
    let uses_condition = widget.properties.style_condition_field.is_some();

    let condition: Option<String> = match (
        &widget.properties.style_condition_field,
        &widget.properties.style_condition_value,
    ) {
        (Some(field), Some(value)) if !field.is_empty() && !value.is_empty() => {
            Some(format!("self.{} == {}", field, value))
        }
        _ => None,
    };

    match (&default_style, &active_style, &condition, uses_condition) {
        (_, Some(active_fn), Some(cond), true) => {
            // Conditional style: if condition use active, else use default (or built-in primary)
            let default_fn = default_style.as_deref().unwrap_or("button::primary");
            b.dot_method("style", &format!(
                "{{ let _is_active = {}; move |theme: &Theme, status: button::Status| if _is_active {{ {}(theme, status) }} else {{ {}(theme, status) }} }}",
                cond, active_fn, default_fn
            ));
        }
        (_, Some(active_fn), _, false) => {
            // Alternate style always applied when no condition is configured.
            b.dot_method("style", active_fn);
        }
        (Some(default_fn), _, _, _) => {
            b.dot_method("style", default_fn);
        }
        _ => {}
    }

    if widget.properties.width != Length::Shrink {
        b.add_width(widget.properties.width);
    }

    if widget.properties.height != Length::Shrink {
        b.add_height(widget.properties.height);
    }

    if widget.properties.padding
        != (Padding {
            top: 5.0,
            bottom: 5.0,
            right: 10.0,
            left: 10.0,
        })
    {
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
    let name = names
        .get(&widget.id)
        .unwrap_or(&"widget".to_string())
        .clone();
    let has_custom_name = !widget.properties.widget_name.trim().is_empty();

    b.indent();
    if use_self {
        b.push(&format!("checkbox(self.{}_checked)", to_snake_case(&name)));
    } else {
        b.push(&format!(
            "checkbox({})",
            if widget.properties.checkbox_checked {
                "true"
            } else {
                "false"
            }
        ));
    }
    b.increase_indent();

    b.dot_method(
        "label",
        &format!("\"{}\"", widget.properties.checkbox_label),
    );
    let toggle_variant = CodeBuilder::msg_variant(&name, "Toggled", has_custom_name);
    b.dot_method("on_toggle", &format!("Message::{}", toggle_variant));

    if widget.properties.checkbox_size != 16.0 {
        b.add_size(widget.properties.checkbox_spacing);
    }

    if widget.properties.checkbox_spacing != 8.0 {
        b.add_spacing(widget.properties.checkbox_spacing);
    }

    if widget.properties.width != Length::Shrink {
        b.add_width(widget.properties.width);
    }

    let default_style = widget
        .properties
        .custom_style_name
        .as_deref()
        .and_then(|style_name| resolve_checkbox_style_fn(custom_styles, style_name));
    let alternate_style = widget
        .properties
        .active_style_name
        .as_deref()
        .and_then(|style_name| resolve_checkbox_style_fn(custom_styles, style_name));
    let (uses_condition, condition) = style_condition_code(widget);

    match (&default_style, &alternate_style, &condition, uses_condition) {
        (_, Some(alternate_fn), Some(cond), true) => {
            let default_fn = default_style.as_deref().unwrap_or("checkbox::primary");
            b.dot_method("style", &format!(
                "{{ let _use_alternate = {}; move |theme: &Theme, status: checkbox::Status| if _use_alternate {{ {}(theme, status) }} else {{ {}(theme, status) }} }}",
                cond, alternate_fn, default_fn
            ));
        }
        (_, Some(alternate_fn), _, false) => {
            b.dot_method("style", alternate_fn);
        }
        (Some(default_fn), _, _, _) => {
            b.dot_method("style", default_fn);
        }
        _ => {}
    }

    b.decrease_indent();
}

fn generate_toggler(
    b: &mut CodeBuilder,
    widget: &Widget,
    names: &HashMap<WidgetId, String>,
    custom_styles: &CustomThemes,
    use_self: bool,
) {
    let props = &widget.properties;
    let name = names
        .get(&widget.id)
        .unwrap_or(&"widget".to_string())
        .clone();
    let has_custom_name = !widget.properties.widget_name.trim().is_empty();

    b.indent();
    if use_self {
        b.push(&format!("toggler(self.{}_active)", to_snake_case(&name)));
    } else {
        b.push(&format!(
            "toggler({})",
            if props.toggler_active {
                "true"
            } else {
                "false"
            }
        ));
    }
    b.newline();
    b.increase_indent();

    b.indent();
    if use_self {
        let variant = CodeBuilder::msg_variant(&name, "Toggled", has_custom_name);
        b.push(&format!(".on_toggle(Message::{})", variant));
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

    let default_style = props
        .custom_style_name
        .as_deref()
        .and_then(|style_name| resolve_toggler_style_fn(custom_styles, style_name));
    let alternate_style = props
        .active_style_name
        .as_deref()
        .and_then(|style_name| resolve_toggler_style_fn(custom_styles, style_name));
    let (uses_condition, condition) = style_condition_code(widget);

    match (&default_style, &alternate_style, &condition, uses_condition) {
        (_, Some(alternate_fn), Some(cond), true) => {
            let default_fn = default_style.as_deref().unwrap_or("toggler::default");
            b.dot_method("style", &format!(
                "{{ let _use_alternate = {}; move |theme: &Theme, status: toggler::Status| if _use_alternate {{ {}(theme, status) }} else {{ {}(theme, status) }} }}",
                cond, alternate_fn, default_fn
            ));
        }
        (_, Some(alternate_fn), _, false) => {
            b.dot_method("style", alternate_fn);
        }
        (Some(default_fn), _, _, _) => {
            b.dot_method("style", default_fn);
        }
        _ => {}
    }

    b.decrease_indent();
}

fn generate_slider(
    b: &mut CodeBuilder,
    widget: &Widget,
    names: &HashMap<WidgetId, String>,
    custom_styles: &CustomThemes,
    use_self: bool,
) {
    let props = &widget.properties;
    let name = names
        .get(&widget.id)
        .unwrap_or(&"widget".to_string())
        .clone();
    let has_custom_name = !widget.properties.widget_name.trim().is_empty();

    b.indent();
    let value = if use_self {
        format!("self.{}_value", to_snake_case(&name))
    } else {
        format!("{}", props.slider_value)
    };
    let handler = if use_self {
        format!(
            "Message::{}",
            CodeBuilder::msg_variant(&name, "Changed", has_custom_name)
        )
    } else {
        "|_| Message::Noop".to_string()
    };
    b.push(&format!(
        "slider({:.1}..={:.1}, {}, {})",
        props.slider_min, props.slider_max, value, handler
    ));

    b.increase_indent();
    if props.slider_step != 1.0 {
        b.add_step(props.slider_step);
    }

    if !matches!(
        props.slider_height,
        iced::widget::slider::Slider::<f32, Theme>::DEFAULT_HEIGHT
    ) {
        b.add_height(Length::Fixed(props.slider_height));
    }

    let default_style = props
        .custom_style_name
        .as_deref()
        .and_then(|style_name| resolve_slider_style_fn(custom_styles, style_name));
    let alternate_style = props
        .active_style_name
        .as_deref()
        .and_then(|style_name| resolve_slider_style_fn(custom_styles, style_name));
    let (uses_condition, condition) = style_condition_code(widget);

    match (&default_style, &alternate_style, &condition, uses_condition) {
        (_, Some(alternate_fn), Some(cond), true) => {
            let default_fn = default_style.as_deref().unwrap_or("slider::default");
            b.dot_method("style", &format!(
                "{{ let _use_alternate = {}; move |theme: &Theme, status: slider::Status| if _use_alternate {{ {}(theme, status) }} else {{ {}(theme, status) }} }}",
                cond, alternate_fn, default_fn
            ));
        }
        (_, Some(alternate_fn), _, false) => {
            b.dot_method("style", alternate_fn);
        }
        (Some(default_fn), _, _, _) => {
            b.dot_method("style", default_fn);
        }
        _ => {}
    }

    b.decrease_indent();
}

fn generate_verticalslider(
    b: &mut CodeBuilder,
    widget: &Widget,
    names: &HashMap<WidgetId, String>,
    custom_styles: &CustomThemes,
    use_self: bool,
) {
    let props = &widget.properties;
    let name = names
        .get(&widget.id)
        .unwrap_or(&"widget".to_string())
        .clone();
    let has_custom_name = !widget.properties.widget_name.trim().is_empty();

    b.indent();
    let value = if use_self {
        format!("self.{}_value", to_snake_case(&name))
    } else {
        format!("{}", props.slider_value)
    };
    let handler = if use_self {
        format!(
            "Message::{}",
            CodeBuilder::msg_variant(&name, "Changed", has_custom_name)
        )
    } else {
        "|_| Message::Noop".to_string()
    };
    b.push(&format!(
        "vertical_slider({:.1}..={:.1}, {}, {})",
        props.slider_min, props.slider_max, value, handler
    ));

    if props.slider_step != 1.0 {
        b.add_step(props.slider_step);
    }

    if !matches!(
        props.slider_width,
        iced::widget::vertical_slider::VerticalSlider::<f32, Theme>::DEFAULT_WIDTH
    ) {
        b.add_height(Length::Fixed(props.slider_width));
    }

    let default_style = props
        .custom_style_name
        .as_deref()
        .and_then(|style_name| resolve_slider_style_fn(custom_styles, style_name));
    let alternate_style = props
        .active_style_name
        .as_deref()
        .and_then(|style_name| resolve_slider_style_fn(custom_styles, style_name));
    let (uses_condition, condition) = style_condition_code(widget);

    match (&default_style, &alternate_style, &condition, uses_condition) {
        (_, Some(alternate_fn), Some(cond), true) => {
            let default_fn = default_style.as_deref().unwrap_or("slider::default");
            b.dot_method("style", &format!(
                "{{ let _use_alternate = {}; move |theme: &Theme, status: slider::Status| if _use_alternate {{ {}(theme, status) }} else {{ {}(theme, status) }} }}",
                cond, alternate_fn, default_fn
            ));
        }
        (_, Some(alternate_fn), _, false) => {
            b.dot_method("style", alternate_fn);
        }
        (Some(default_fn), _, _, _) => {
            b.dot_method("style", default_fn);
        }
        _ => {}
    }
}

fn generate_radio(
    b: &mut CodeBuilder,
    widget: &Widget,
    names: &HashMap<WidgetId, String>,
    custom_styles: &CustomThemes,
    use_self: bool,
) {
    let props = &widget.properties;
    let name = names
        .get(&widget.id)
        .unwrap_or(&"widget".to_string())
        .clone();
    let has_custom_name = !widget.properties.widget_name.trim().is_empty();
    let default_style = props
        .custom_style_name
        .as_deref()
        .and_then(|style_name| resolve_radio_style_fn(custom_styles, style_name));
    let alternate_style = props
        .active_style_name
        .as_deref()
        .and_then(|style_name| resolve_radio_style_fn(custom_styles, style_name));
    let (uses_condition, condition) = style_condition_code(widget);

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
            format!(
                "Message::{}",
                CodeBuilder::msg_variant(&name, "Selected", has_custom_name)
            )
        } else {
            "|_| Message::Noop".to_string()
        };
        b.push(&format!(
            "radio(\"{}\", {}, {}, {})",
            option, i, selected, handler
        ));

        if props.radio_size != 16.0 {
            b.add_size(props.radio_size);
        }
        match (&default_style, &alternate_style, &condition, uses_condition) {
            (_, Some(alternate_fn), Some(cond), true) => {
                let default_fn = default_style.as_deref().unwrap_or("radio::default");
                b.dot_method("style", &format!(
                    "{{ let _use_alternate = {}; move |theme: &Theme, status: radio::Status| if _use_alternate {{ {}(theme, status) }} else {{ {}(theme, status) }} }}",
                    cond, alternate_fn, default_fn
                ));
            }
            (_, Some(alternate_fn), _, false) => {
                b.dot_method("style", alternate_fn);
            }
            (Some(default_fn), _, _, _) => {
                b.dot_method("style", default_fn);
            }
            _ => {}
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
    custom_styles: &CustomThemes,
    use_self: bool,
) {
    let props = &widget.properties;
    let name = names
        .get(&widget.id)
        .unwrap_or(&"widget".to_string())
        .clone();
    let has_custom_name = !widget.properties.widget_name.trim().is_empty();

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
        let variant = CodeBuilder::msg_variant(&name, "Selected", has_custom_name);
        b.push(&format!("Message::{}", variant));
    } else {
        b.push("|_| Message::Noop");
    }
    b.newline();

    b.decrease_indent();
    b.indent();
    b.push(")");

    if !props.picklist_placeholder.is_empty() && props.picklist_placeholder != "Choose an option..."
    {
        b.add_placeholder(&props.picklist_placeholder);
    }

    if !matches!(props.width, Length::Shrink) {
        b.add_width(props.width);
    }

    if props.padding
        != (Padding {
            top: 5.0,
            bottom: 5.0,
            right: 10.0,
            left: 10.0,
        })
    {
        b.add_padding(&props.padding, props.padding_mode);
    }

    let default_style = props
        .custom_style_name
        .as_deref()
        .and_then(|style_name| resolve_pick_list_style_fn(custom_styles, style_name));
    let alternate_style = props
        .active_style_name
        .as_deref()
        .and_then(|style_name| resolve_pick_list_style_fn(custom_styles, style_name));
    let (uses_condition, condition) = style_condition_code(widget);

    match (&default_style, &alternate_style, &condition, uses_condition) {
        (_, Some(alternate_fn), Some(cond), true) => {
            let default_fn = default_style.as_deref().unwrap_or("pick_list::default");
            b.dot_method("style", &format!(
                "{{ let _use_alternate = {}; move |theme: &Theme, status: pick_list::Status| if _use_alternate {{ {}(theme, status) }} else {{ {}(theme, status) }} }}",
                cond, alternate_fn, default_fn
            ));
        }
        (_, Some(alternate_fn), _, false) => {
            b.dot_method("style", alternate_fn);
        }
        (Some(default_fn), _, _, _) => {
            b.dot_method("style", default_fn);
        }
        _ => {}
    }

    if let Some(style_name) = props.menu_style_name.as_deref() {
        if has_custom_style(custom_styles, ThemePaneEnum::Menu, style_name) {
            b.add_menu_style("styles::menu", &style_name.to_lowercase());
        }
    }
}

fn generate_textinput(
    b: &mut CodeBuilder,
    widget: &Widget,
    names: &HashMap<WidgetId, String>,
    custom_styles: &CustomThemes,
    use_self: bool,
) {
    let props = &widget.properties;
    let name = names
        .get(&widget.id)
        .unwrap_or(&"widget".to_string())
        .clone();
    let has_custom_name = !widget.properties.widget_name.trim().is_empty();

    b.indent();
    let value_arg = if use_self {
        format!("&self.{}_value", to_snake_case(&name))
    } else {
        "\"\"".to_string()
    };
    b.push(&format!(
        "text_input(\"{}\", {})",
        props.text_input_placeholder, value_arg
    ));
    b.increase_indent();

    let input_variant = CodeBuilder::msg_variant(&name, "OnInput", has_custom_name);
    b.dot_method("on_input", &format!("Message::{}", input_variant));

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

    b.add_padding(
        &Padding::new(props.text_input_padding),
        PaddingMode::Uniform,
    );

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

    if let Some(style_name) = props.custom_style_name.as_deref() {
        if has_custom_style(custom_styles, ThemePaneEnum::TextInput, style_name) {
            b.add_style("styles::text_input", &style_name.to_lowercase());
        }
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
    let name = names
        .get(&widget.id)
        .unwrap_or(&"widget".to_string())
        .clone();
    let has_custom_name = !widget.properties.widget_name.trim().is_empty();

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
        let variant = CodeBuilder::msg_variant(&name, "Selected", has_custom_name);
        b.push(&format!("Message::{}", variant));
    } else {
        b.push("|_| Message::Noop");
    }
    b.newline();

    b.decrease_indent();
    b.indent();
    b.push(")");

    if props.combobox_use_on_input {
        let input_variant = CodeBuilder::msg_variant(&name, "OnInput", has_custom_name);
        b.dot_method("on_input", &format!("Message::{}", input_variant));
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

    let mut uses_split_styles = false;
    if let Some(style_name) = props.custom_style_name.as_deref() {
        if has_custom_style(custom_styles, ThemePaneEnum::TextInput, style_name) {
            b.add_input_style("styles::text_input", &style_name.to_lowercase());
            uses_split_styles = true;
        }
    }

    if let Some(style_name) = props.menu_style_name.as_deref() {
        if has_custom_style(custom_styles, ThemePaneEnum::Menu, style_name) {
            b.add_menu_style("styles::menu", &style_name.to_lowercase());
            uses_split_styles = true;
        }
    }

    if !uses_split_styles {
        if let Some(style) = props.custom_style_name.as_deref() {
            if has_custom_style(custom_styles, ThemePaneEnum::Combobox, style) {
                b.add_input_style(
                    "styles::combo_box",
                    &format!("{}_input_style", style.to_lowercase()),
                );
                b.add_menu_style(
                    "styles::combo_box",
                    &format!("{}_menu_style", style.to_lowercase()),
                );
            }
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
                generate_widget_code(
                    b,
                    child,
                    names,
                    use_self,
                    custom_styles,
                    type_system,
                    view_refs,
                );
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
                generate_widget_code(
                    b,
                    child,
                    names,
                    use_self,
                    custom_styles,
                    type_system,
                    view_refs,
                );
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
            b.dot_method(
                "vertical_spacing",
                &format!("{:.1}", props.wrapping_vertical_spacing),
            );
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
                generate_widget_code(
                    b,
                    child,
                    names,
                    use_self,
                    custom_styles,
                    type_system,
                    view_refs,
                );
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
    let resolve_container_style = |style: &str| -> Option<String> {
        let snake = to_snake_case(style);
        let is_custom = custom_styles
            .styles()
            .get(&ThemePaneEnum::Container)
            .map(|m| m.contains_key(style))
            .unwrap_or(false);
        if is_custom {
            Some(format!("styles::container::{}", snake))
        } else if ContainerStyleType::get(style).is_some() {
            Some(format!("container::{}", snake))
        } else {
            None
        }
    };

    let default_style = props
        .custom_style_name
        .as_deref()
        .and_then(resolve_container_style);
    let alternate_style = props
        .active_style_name
        .as_deref()
        .and_then(resolve_container_style);
    let uses_condition = props.style_condition_field.is_some();
    let condition: Option<String> =
        match (&props.style_condition_field, &props.style_condition_value) {
            (Some(field), Some(value)) if !field.is_empty() && !value.is_empty() => {
                Some(format!("self.{} == {}", field, value))
            }
            _ => None,
        };

    match (&default_style, &alternate_style, &condition, uses_condition) {
        (_, Some(alternate_fn), Some(cond), true) => {
            let default_fn = default_style.as_deref().unwrap_or("container::transparent");
            b.dot_method("style", &format!(
                "{{ let _use_alternate = {}; move |theme: &Theme| if _use_alternate {{ {}(theme) }} else {{ {}(theme) }} }}",
                cond, alternate_fn, default_fn
            ));
        }
        (_, Some(alternate_fn), _, false) => {
            b.dot_method("style", alternate_fn);
        }
        (Some(default_fn), _, _, _) => {
            b.dot_method("style", default_fn);
        }
        _ => {}
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
                generate_widget_code(
                    b,
                    child,
                    names,
                    use_self,
                    custom_styles,
                    type_system,
                    view_refs,
                );
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
                generate_widget_code(
                    b,
                    child,
                    names,
                    use_self,
                    custom_styles,
                    type_system,
                    view_refs,
                );
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

fn generate_collapsible(
    b: &mut CodeBuilder,
    widget: &Widget,
    names: &HashMap<WidgetId, String>,
    _custom_styles: &CustomThemes,
    use_self: bool,
    type_system: &TypeSystem,
    view_refs: &[ViewRefInfo],
) {
    let props = &widget.properties;
    let default_padding = Padding {
        top: 4.0,
        right: 8.0,
        bottom: 4.0,
        left: 8.0,
    };

    b.indent();
    b.push("widgets::collapsible::collapsible(");
    b.newline();
    b.increase_indent();

    b.indent();
    b.push(&format!("\"{}\",", props.collapsible_title));
    b.newline();

    if use_self {
        if let Some(child) = widget.children.get(0) {
            generate_widget_code(
                b,
                child,
                names,
                use_self,
                _custom_styles,
                type_system,
                view_refs,
            );
        } else {
            b.indent();
            b.push("space::horizontal().height(Length::Shrink)");
        }
    } else {
        b.indent();
        b.push("// collapsible content");
    }

    b.newline();
    b.decrease_indent();
    b.indent();
    b.push(")");
    b.increase_indent();

    if !matches!(props.width, Length::Fill) {
        b.add_width(props.width);
    }
    if !matches!(props.height, Length::Shrink) {
        b.add_height(props.height);
    }
    if (props.collapsible_header_height - 32.0).abs() > f32::EPSILON {
        b.dot_method(
            "header_height",
            &format!("{:.1}", props.collapsible_header_height),
        );
    }
    match props.align_x {
        ContainerAlignX::Left => {}
        ContainerAlignX::Center => b.dot_method("title_alignment", "Alignment::Center"),
        ContainerAlignX::Right => b.dot_method("title_alignment", "Alignment::End"),
    }
    if !props.collapsible_header_clickable {
        b.dot_method("header_clickable", "false");
    }
    if props.padding_mode != PaddingMode::Symmetric
        || props.padding.top != default_padding.top
        || props.padding.right != default_padding.right
        || props.padding.bottom != default_padding.bottom
        || props.padding.left != default_padding.left
    {
        b.add_padding(&props.padding, props.padding_mode);
    }
    if props.collapsible_expanded {
        b.dot_method("expanded", "true");
    }
    if (props.text_size - 16.0).abs() > f32::EPSILON {
        b.dot_method("text_size", &format!("{:.1}", props.text_size));
    }
    if props.font != FontType::Default {
        b.add_font(props.font);
    }

    let default_style = props
        .custom_style_name
        .as_deref()
        .and_then(resolve_collapsible_style_fn);
    let alternate_style = props
        .active_style_name
        .as_deref()
        .and_then(resolve_collapsible_style_fn);
    let (uses_condition, condition) = style_condition_code(widget);

    match (&default_style, &alternate_style, &condition, uses_condition) {
        (_, Some(alternate_fn), Some(cond), true) => {
            let default_fn = default_style
                .as_deref()
                .unwrap_or("widgets::collapsible::default");
            b.dot_method(
                "style",
                &format!(
                    "{{ let _use_alternate = {}; move |theme: &Theme, status| if _use_alternate {{ {}(theme, status) }} else {{ {}(theme, status) }} }}",
                    cond, alternate_fn, default_fn
                ),
            );
        }
        (_, Some(alternate_fn), _, false) => {
            b.dot_method("style", alternate_fn);
        }
        (Some(default_fn), _, _, _) => {
            b.dot_method("style", default_fn);
        }
        _ => {}
    }

    b.decrease_indent();
}

fn generate_collapsible_group(
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
    b.push("widgets::collapsible::CollapsibleGroup::new(vec![");
    b.newline();
    b.increase_indent();

    if use_self {
        for (index, child) in widget.children.iter().enumerate() {
            generate_widget_code(
                b,
                child,
                names,
                use_self,
                custom_styles,
                type_system,
                view_refs,
            );
            b.push(".into()");
            if index + 1 < widget.children.len() {
                b.push(",");
            }
            b.newline();
        }
    } else {
        b.indent();
        b.push("// collapsible items");
        b.newline();
    }

    b.decrease_indent();
    b.indent();
    b.push("])");
    b.increase_indent();

    if !matches!(props.width, Length::Fill) {
        b.add_width(props.width);
    }
    if !matches!(props.height, Length::Shrink) {
        b.add_height(props.height);
    }
    if props.spacing != 0.0 {
        b.add_spacing(props.spacing);
    }

    b.decrease_indent();
}

fn generate_generic_overlay(
    b: &mut CodeBuilder,
    widget: &Widget,
    names: &HashMap<WidgetId, String>,
    custom_styles: &CustomThemes,
    use_self: bool,
    type_system: &TypeSystem,
    view_refs: &[ViewRefInfo],
) {
    let props = &widget.properties;
    let name = names
        .get(&widget.id)
        .unwrap_or(&"widget".to_string())
        .clone();
    let has_custom_name = !props.widget_name.trim().is_empty();
    let has_trigger_child = widget.children.get(0).is_some();
    let uses_hover_placement =
        props.generic_overlay_on_hover || props.generic_overlay_hover_positions_on_click;
    let default_padding = Padding {
        top: 5.0,
        bottom: 5.0,
        right: 10.0,
        left: 10.0,
    };

    b.indent();
    b.push("widgets::generic_overlay::overlay_button(");
    b.newline();
    b.increase_indent();

    if use_self {
        if let Some(child) = widget.children.get(0) {
            generate_widget_code(
                b,
                child,
                names,
                use_self,
                custom_styles,
                type_system,
                view_refs,
            );
        } else {
            b.indent();
            b.push(&format!("iced::widget::text(\"{}\")", props.text_content));
        }
    } else {
        b.indent();
        b.push("// trigger content");
    }

    b.push(",");
    b.newline();
    b.indent();
    b.push(&format!("\"{}\",", props.generic_overlay_title));
    b.newline();

    if use_self {
        if let Some(child) = widget.children.get(1) {
            generate_widget_code(
                b,
                child,
                names,
                use_self,
                custom_styles,
                type_system,
                view_refs,
            );
        } else {
            b.indent();
            b.push("iced::widget::text(\"Overlay content\")");
        }
    } else {
        b.indent();
        b.push("// overlay content");
    }

    b.newline();
    b.decrease_indent();
    b.indent();
    b.push(")");
    b.increase_indent();

    if let Some(ref id) = props.widget_id {
        if !id.is_empty() {
            b.add_id(id);
        }
    }

    if has_trigger_child {
        b.dot_method("width", "Length::Shrink");
        b.dot_method("height", "Length::Shrink");
        b.dot_method("padding", "0.0");
        b.dot_method("style", "iced::widget::button::text");
    } else {
        if !matches!(props.width, Length::Shrink) {
            b.add_width(props.width);
        }
        if !matches!(props.height, Length::Shrink) {
            b.add_height(props.height);
        }
        if props.padding_mode != PaddingMode::Symmetric || props.padding != default_padding {
            b.add_padding(&props.padding, props.padding_mode);
        }
    }
    if props.clip {
        b.dot_method("button_clip", "true");
    }

    if props.generic_overlay_overlay_width_dynamic {
        b.dot_method(
            "overlay_width_dynamic",
            &format!(
                "|available| Length::Fixed(available * {:.4})",
                props.generic_overlay_overlay_width_dynamic_factor
            ),
        );
    } else if props.generic_overlay_overlay_width != Length::Fixed(400.0) {
        b.dot_method(
            "overlay_width",
            &format_length(props.generic_overlay_overlay_width),
        );
    }
    if props.generic_overlay_overlay_height_dynamic {
        b.dot_method(
            "overlay_height_dynamic",
            &format!(
                "|available| Length::Fixed(available * {:.4})",
                props.generic_overlay_overlay_height_dynamic_factor
            ),
        );
    } else if !matches!(props.generic_overlay_overlay_height, Length::Shrink) {
        b.dot_method(
            "overlay_height",
            &format_length(props.generic_overlay_overlay_height),
        );
    }
    if (props.generic_overlay_overlay_padding - 15.0).abs() > f32::EPSILON {
        b.dot_method(
            "overlay_padding",
            &format!("{:.1}", props.generic_overlay_overlay_padding),
        );
    }
    if (props.generic_overlay_overlay_radius - 12.0).abs() > f32::EPSILON {
        b.dot_method(
            "overlay_radius",
            &format!("{:.1}", props.generic_overlay_overlay_radius),
        );
    }

    if props.generic_overlay_on_hover {
        b.dot_method_no_args("on_hover");
    }
    if props.generic_overlay_hover_positions_on_click {
        b.dot_method_no_args("hover_positions_on_click");
    }
    if uses_hover_placement {
        if props.generic_overlay_hover_position != GenericOverlayPosition::Right {
            b.dot_method(
                "hover_position",
                generic_overlay_position_code(props.generic_overlay_hover_position),
            );
        }
        if (props.generic_overlay_hover_gap - 5.0).abs() > f32::EPSILON {
            b.dot_method(
                "hover_gap",
                &format!("{:.1}", props.generic_overlay_hover_gap),
            );
        }
        if props.generic_overlay_hover_alignment != ContainerAlignX::Center {
            b.dot_method(
                "hover_alignment",
                generic_overlay_alignment_code(props.generic_overlay_hover_alignment),
            );
        }
        if props.generic_overlay_hover_mode != GenericOverlayPositionMode::Outside {
            b.dot_method(
                "hover_mode",
                generic_overlay_position_mode_code(props.generic_overlay_hover_mode),
            );
        }
        if !props.generic_overlay_hover_snap {
            b.dot_method("hover_snap", "false");
        }
        if !props.generic_overlay_safe_triangle {
            b.dot_method("safe_triangle", "false");
        }
    }

    if props.generic_overlay_close_on_click_outside {
        b.dot_method_no_args("close_on_click_outside");
    }
    if props.generic_overlay_opaque {
        b.dot_method("opaque", "true");
    }
    if (props.generic_overlay_opaque_alpha - 0.3).abs() > f32::EPSILON {
        b.dot_method(
            "opaque_alpha",
            &format!("{:.2}", props.generic_overlay_opaque_alpha),
        );
    }
    if props.generic_overlay_hide_header {
        b.dot_method_no_args("hide_header");
    }
    if props.generic_overlay_hide_close_button {
        b.dot_method_no_args("hide_close_button");
    }
    if props.generic_overlay_block_dragging {
        b.dot_method_no_args("block_dragging");
    }
    if props.generic_overlay_resizable != GenericOverlayResizeMode::None {
        b.dot_method(
            "resizable",
            generic_overlay_resize_mode_code(props.generic_overlay_resizable),
        );
    }
    if props.generic_overlay_reset_on_close {
        b.dot_method_no_args("reset_on_close");
    }
    b.dot_method("interactive_base", "true");
    if use_self {
        let variant = CodeBuilder::msg_variant(&name, "Toggled", has_custom_name);
        b.dot_method("is_open", &format!("self.{}_open", to_snake_case(&name)));
        b.dot_method("on_toggle", &format!("Message::{}", variant));
    } else {
        b.dot_method("is_open", "false");
        b.newline();
        b.indent();
        b.push(".on_toggle(|_| Message::Noop)");
    }
    if props.generic_overlay_animate {
        match props.generic_overlay_animation_preset {
            GenericOverlayAnimationPreset::Default => b.dot_method("animate", "true"),
            GenericOverlayAnimationPreset::Quick => b.dot_method_no_args("quick_animation"),
            GenericOverlayAnimationPreset::Slow => b.dot_method_no_args("slow_animation"),
        }
    }

    let default_style = props
        .custom_style_name
        .as_deref()
        .and_then(|style_name| resolve_generic_overlay_trigger_style_fn(custom_styles, style_name));
    let alternate_style = props
        .active_style_name
        .as_deref()
        .and_then(|style_name| resolve_generic_overlay_trigger_style_fn(custom_styles, style_name));
    let (uses_condition, condition) = style_condition_code(widget);

    if !has_trigger_child {
        match (&default_style, &alternate_style, &condition, uses_condition) {
            (_, Some(alternate_fn), Some(cond), true) => {
                let default_fn = default_style
                    .as_deref()
                    .unwrap_or("iced::widget::button::primary");
                b.dot_method(
                    "style",
                    &format!(
                        "{{ let _use_alternate = {}; move |theme: &Theme, status: iced::widget::button::Status| if _use_alternate {{ {}(theme, status) }} else {{ {}(theme, status) }} }}",
                        cond, alternate_fn, default_fn
                    ),
                );
            }
            (_, Some(alternate_fn), _, false) => {
                b.dot_method("style", alternate_fn);
            }
            (Some(default_fn), _, _, _) => {
                b.dot_method("style", default_fn);
            }
            _ => {}
        }
    }

    if let Some(ref style_name) = props.generic_overlay_overlay_style_name {
        if let Some(style_fn) = resolve_generic_overlay_style_fn(style_name) {
            b.dot_method("overlay_style", &style_fn);
        }
    }

    b.decrease_indent();
}

fn generate_date_picker(
    b: &mut CodeBuilder,
    widget: &Widget,
    names: &HashMap<WidgetId, String>,
    custom_styles: &CustomThemes,
    use_self: bool,
    _type_system: &TypeSystem,
    _view_refs: &[ViewRefInfo],
) {
    let props = &widget.properties;
    let name = names
        .get(&widget.id)
        .unwrap_or(&"widget".to_string())
        .clone();
    let sname = to_snake_case(&name);
    let default_padding = Padding {
        top: 5.0,
        bottom: 5.0,
        right: 10.0,
        left: 10.0,
    };
    let placeholder = escape_string_literal(&props.text_content);

    b.indent();
    b.push("stack![");
    b.newline();
    b.increase_indent();

    b.indent();
    b.push("button(");
    b.newline();
    b.increase_indent();
    b.indent();
    if use_self {
        b.push(&format!(
            "text(Self::date_picker_button_label(&self.{}_selection, self.{}_time, \"{}\", {}))",
            sname,
            sname,
            placeholder,
            if props.date_picker_show_time {
                "true"
            } else {
                "false"
            }
        ));
    } else {
        b.push(&format!("text(\"{}\")", placeholder));
    }
    b.newline();
    b.decrease_indent();
    b.indent();
    b.push(")");
    b.increase_indent();

    if use_self {
        b.dot_method(
            "on_press",
            &format!("Message::{}OpenRequested", to_pascal_case(&name)),
        );
    }

    if !matches!(props.width, Length::Shrink) {
        b.add_width(props.width);
    }
    if !matches!(props.height, Length::Shrink) {
        b.add_height(props.height);
    }
    if props.padding_mode != PaddingMode::Symmetric || props.padding != default_padding {
        b.add_padding(&props.padding, props.padding_mode);
    }
    if props.clip {
        b.dot_method("clip", "true");
    }

    let default_style = props
        .custom_style_name
        .as_deref()
        .and_then(|style_name| resolve_generic_overlay_trigger_style_fn(custom_styles, style_name));
    let alternate_style = props
        .active_style_name
        .as_deref()
        .and_then(|style_name| resolve_generic_overlay_trigger_style_fn(custom_styles, style_name));
    let (uses_condition, condition) = style_condition_code(widget);

    match (&default_style, &alternate_style, &condition, uses_condition) {
        (_, Some(alternate_fn), Some(cond), true) => {
            let default_fn = default_style
                .as_deref()
                .unwrap_or("iced::widget::button::primary");
            b.dot_method(
                "style",
                &format!(
                    "{{ let _use_alternate = {}; move |theme: &Theme, status: iced::widget::button::Status| if _use_alternate {{ {}(theme, status) }} else {{ {}(theme, status) }} }}",
                    cond, alternate_fn, default_fn
                ),
            );
        }
        (_, Some(alternate_fn), _, false) => {
            b.dot_method("style", alternate_fn);
        }
        (Some(default_fn), _, _, _) => {
            b.dot_method("style", default_fn);
        }
        _ => {}
    }

    b.push(",");
    b.newline();

    b.decrease_indent();
    b.indent();
    if use_self {
        b.push(&format!(
            "widgets::date_picker::date_picker(self.{}_open, self.{}_selection.clone())",
            sname, sname
        ));
        b.increase_indent();
        if props.date_picker_show_time {
            b.dot_method_no_args("show_time");
            b.dot_method("initial_time", &format!("self.{}_time", sname));
            b.newline();
            b.indent();
            b.push(&format!(
                ".on_change_with_time(Message::{}ChangedWithTime)",
                to_pascal_case(&name)
            ));
        } else {
            b.newline();
            b.indent();
            b.push(&format!(
                ".on_change(Message::{}Changed)",
                to_pascal_case(&name)
            ));
        }
        b.newline();
        b.indent();
        b.push(&format!(
            ".on_close(|| Message::{}Closed)",
            to_pascal_case(&name)
        ));
        b.decrease_indent();
    } else {
        b.push("widgets::date_picker::date_picker(false, widgets::date_picker::DateSelection::single())");
        b.increase_indent();
        if props.date_picker_show_time {
            b.dot_method_no_args("show_time");
            b.dot_method(
                "initial_time",
                "widgets::date_picker::TimeSelection::default()",
            );
            b.newline();
            b.indent();
            b.push(".on_change_with_time(|_, _| Message::Noop)");
        } else {
            b.newline();
            b.indent();
            b.push(".on_change(|_| Message::Noop)");
        }
        b.newline();
        b.indent();
        b.push(".on_close(|| Message::Noop)");
        b.decrease_indent();
    }
    b.newline();

    b.decrease_indent();
    b.indent();
    b.push("]");
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
    let name = names
        .get(&widget.id)
        .unwrap_or(&"widget".to_string())
        .clone();

    b.indent();
    b.push("mouse_area(");
    b.newline();

    b.increase_indent();
    if use_self {
        if !widget.children.is_empty() {
            generate_widget_code(
                b,
                &widget.children[0],
                names,
                use_self,
                custom_styles,
                type_system,
                view_refs,
            );
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

    if props.mousearea_on_press {
        b.on_press(&name);
    }
    if props.mousearea_on_release {
        b.on_release(&name);
    }
    if props.mousearea_on_double_click {
        b.on_double_click(&name);
    }
    if props.mousearea_on_right_press {
        b.on_right_press(&name);
    }
    if props.mousearea_on_right_release {
        b.on_right_release(&name);
    }
    if props.mousearea_on_middle_press {
        b.on_middle_press(&name);
    }
    if props.mousearea_on_middle_release {
        b.on_middle_release(&name);
    }
    if props.mousearea_on_scroll {
        b.on_scroll(&name);
    }
    if props.mousearea_on_enter {
        b.on_enter(&name);
    }
    if props.mousearea_on_move {
        b.on_move(&name);
    }
    if props.mousearea_on_exit {
        b.on_exit(&name);
    }

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
            generate_widget_code(
                b,
                host,
                names,
                use_self,
                custom_styles,
                type_system,
                view_refs,
            );
        } else {
            b.indent();
            b.push("text(\"Hover me\")");
        }
        b.push(",");
        b.newline();

        if let Some(content) = widget.children.get(1) {
            generate_widget_code(
                b,
                content,
                names,
                use_self,
                custom_styles,
                type_system,
                view_refs,
            );
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
                generate_widget_code(
                    b,
                    child,
                    names,
                    use_self,
                    custom_styles,
                    type_system,
                    view_refs,
                );
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
    let name = names
        .get(&widget.id)
        .unwrap_or(&"widget".to_string())
        .clone();

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
                generate_widget_code(
                    b,
                    child,
                    names,
                    use_self,
                    custom_styles,
                    type_system,
                    view_refs,
                );
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
            generate_widget_code(
                b,
                &widget.children[0],
                names,
                use_self,
                custom_styles,
                type_system,
                view_refs,
            );
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
            b.dot_method(
                "position",
                &format!(
                    "Point::new({:.1}, {:.1})",
                    props.pin_point.x, props.pin_point.y
                ),
            );
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
    let name = names
        .get(&widget.id)
        .unwrap_or(&"widget".to_string())
        .clone();

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
        b.dot_method(
            "map",
            &format!("Message::{}LinkClicked", to_pascal_case(&name)),
        );
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

    let struct_def = props
        .table_referenced_struct
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
            // Custom enum/struct fields need string conversion before rendering in table cells.
            let cell_expr = match &field.field_type {
                crate::enum_builder::FieldType::CustomEnum(_) => {
                    format!("text(row.{}.to_string())", field.name)
                }
                crate::enum_builder::FieldType::CustomStruct(_) => {
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

fn generate_view_reference(b: &mut CodeBuilder, widget: &Widget, view_refs: &[ViewRefInfo]) {
    if let Some(vr) = view_refs.iter().find(|vr| vr.widget_id == widget.id) {
        let sel_variant = vr.primary_variant(); // for Selection enum match arm
        let msg_variant = vr.msg_variant(); // for ViewMessages routing
        if vr.is_multi() {
            // Emit a match on the selection enum
            let sel_type = vr.selection_type();
            b.indent();
            b.push(&format!("match self.{}_selection {{", vr.field_name));
            b.newline();
            b.increase_indent();
            b.indent();
            b.push(&format!(
                "{}::{} => self.{}.view().map(|msg| Message::ViewMessages(ViewMessages::{}(msg))),",
                sel_type, sel_variant, vr.field_name, msg_variant
            ));
            b.newline();
            for (ef, _, _) in &vr.extra_views {
                let ep = to_pascal_case(ef);
                b.indent();
                b.push(&format!(
                    "{}::{} => self.{}.view().map(|msg| Message::ViewMessages(ViewMessages::{}(msg))),",
                    sel_type, ep, ef, ep
                ));
                b.newline();
            }
            b.decrease_indent();
            b.indent();
            b.push("}");
        } else {
            b.indent();
            b.push(&format!(
                "self.{}.view().map(|msg| Message::ViewMessages(ViewMessages::{}(msg)))",
                vr.field_name, msg_variant
            ));
        }
    } else {
        b.indent();
        b.push("text(\"ViewReference: not resolved\")");
    }
}

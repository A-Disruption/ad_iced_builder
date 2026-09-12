use super::builder::{CodeBuilder, format_length, to_pascal_case, to_snake_case};
use super::helpers::style_fn_name;
use super::events::ViewRefInfo;
use crate::data_structures::types::type_implementations::*;
use crate::data_structures::properties::properties::Properties;
use crate::data_structures::types::types::{Widget, WidgetId, WidgetType};
use crate::enum_builder::TypeSystem;
use crate::views::theme_and_stylefn_builder::{CustomThemes, ThemePaneEnum};
use iced::widget::text::LineHeight;
use iced::advanced::graphics::core::length::Bounds;
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
        WidgetType::Popover => generate_popover(
            b,
            widget,
            names,
            custom_styles,
            use_self,
            type_system,
            view_refs,
        ),
        WidgetType::Menu => generate_menu(
            b,
            widget,
            names,
            custom_styles,
            use_self,
            type_system,
            view_refs,
        ),
        WidgetType::Layer => generate_layer(
            b,
            widget,
            names,
            custom_styles,
            use_self,
            type_system,
            view_refs,
        ),
        WidgetType::Toast => generate_toast(
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
        WidgetType::Split => generate_split(
            b,
            widget,
            names,
            custom_styles,
            use_self,
            type_system,
            view_refs,
        ),
        WidgetType::Float => generate_float(
            b,
            widget,
            names,
            custom_styles,
            use_self,
            type_system,
            view_refs,
        ),
        WidgetType::Responsive => generate_responsive(
            b,
            widget,
            names,
            custom_styles,
            use_self,
            type_system,
            view_refs,
        ),
        WidgetType::Sensor => generate_sensor(
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
        WidgetType::TextEditor => generate_text_editor(b, widget, names, custom_styles, use_self),
        WidgetType::TextInput => generate_textinput(b, widget, names, custom_styles, use_self),
        WidgetType::Table => generate_table(b, widget, names, use_self, type_system),
        WidgetType::DataTable => generate_data_table(b, widget, names, use_self, type_system),
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
            b.add_style("styles::rule", &style_fn_name(style_name));
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
        .then(|| format!("{}::{}", module, style_fn_name(style_name)))
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

fn resolve_text_editor_style_fn(custom_styles: &CustomThemes, style_name: &str) -> Option<String> {
    resolve_custom_style_fn(
        custom_styles,
        ThemePaneEnum::TextEditor,
        "styles::text_editor",
        style_name,
    )
}

fn resolve_collapsible_style_fn(style_name: &str) -> Option<String> {
    match style_name {
        "Default" => Some("widgets::collapsible::default".to_string()),
        "Primary" => Some("widgets::collapsible::primary".to_string()),
        "Success" => Some("widgets::collapsible::success".to_string()),
        "Danger" => Some("widgets::collapsible::danger".to_string()),
        "Warning" => Some("widgets::collapsible::warning".to_string()),
        crate::controls::MATERIAL_STYLE => {
            Some("widgets::m3::collapsible::material".to_string())
        }
        _ => None,
    }
}

/// A style closure for one of the overlay widgets, from a preset name.
///
/// The widgets crate dropped `generic_overlay`'s named `primary` / `success` /
/// `danger` / `warning` / `blank` helpers when it split that widget in three,
/// so the generated code builds the same five over the widget's own default,
/// the way the preview does. `module` is `popover`, `menu` or `layer`.
fn resolve_overlay_style_fn(module: &str, style_name: &str) -> Option<String> {
    // The M3 styles are real functions in the widgets crate rather than
    // something built here, so they are named rather than derived.
    if style_name == crate::controls::MATERIAL_STYLE {
        return Some(format!("widgets::m3::{module}::material"));
    }

    let swatch = match style_name {
        "Primary" => "primary",
        "Success" => "success",
        "Danger" => "danger",
        "Warning" => "warning",
        "Blank" => "background",
        _ => return None,
    };

    Some(format!(
        "|theme: &Theme| {{ let palette = theme.palette(); \
         widgets::{module}::Style {{ \
         background: iced::Background::Color(palette.{swatch}.base.color), \
         text_color: palette.{swatch}.base.text, \
         ..widgets::{module}::default(theme) }} }}"
    ))
}

fn resolve_overlay_trigger_style_fn(
    custom_styles: &CustomThemes,
    style_name: &str,
) -> Option<String> {
    let is_custom = custom_styles
        .styles()
        .get(&ThemePaneEnum::Button)
        .map(|styles| styles.contains_key(style_name))
        .unwrap_or(false);

    if is_custom {
        return Some(format!("styles::button_{}", style_fn_name(style_name)));
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

/// The side of the trigger an anchored overlay opens on.
fn overlay_position_code(position: OverlayPosition) -> &'static str {
    match position {
        OverlayPosition::Top => "widgets::anchor::Side::Top",
        OverlayPosition::Bottom => "widgets::anchor::Side::Bottom",
        OverlayPosition::Left => "widgets::anchor::Side::Left",
        OverlayPosition::Right => "widgets::anchor::Side::Right",
    }
}

fn overlay_alignment_code(alignment: ContainerAlignX) -> &'static str {
    match alignment {
        ContainerAlignX::Left => "widgets::anchor::Align::Start",
        ContainerAlignX::Center => "widgets::anchor::Align::Center",
        ContainerAlignX::Right => "widgets::anchor::Align::End",
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

    if !matches!(props.width, Length::Shrink) {
        b.add_width(props.width);
    }

    if !matches!(props.height, Length::Shrink) {
        b.add_height(props.height);
    }
}

/// The icon element that sits beside a `text_input` or `combo_box`.
///
/// iced master removed `text_input::Icon`, which drew a glyph inside the
/// field's border. Composing a row is what replaces it: the icon sits outside
/// the border rather than within it, which is a visual change, but it is the
/// only shape the current API supports.
/// An icon glyph, sized when a size was set.
///
/// No colour: an icon with none inherits the one around it, which is what a
/// menu row or a button label wants.
fn icon_expr(family: &str, name: &str, size: f32) -> String {
    let call = format!("icon::{}()", super::icon_fn_name(family, name));

    if size > 0.0 {
        format!("{call}.size({size:.1})")
    } else {
        call
    }
}

/// The icon composed beside an input.
///
/// Unlike [`icon_expr`], this one is coloured — the input's style says what
/// the icon inside the field looks like, and there is nothing around it to
/// inherit from. It goes through the style closure rather than `.color(..)`
/// so it can read the theme, which `view` has none of.
fn input_icon_expr(family: &str, name: &str, size: f32, color_fn: Option<&str>) -> String {
    let call = icon_expr(family, name, size);

    // With no style to read, iced 0.14's own default for the `text_input`
    // field it removed.
    let color = match color_fn {
        Some(path) => format!("{path}(theme)"),
        None => "theme.palette().background.weak.text".to_string(),
    };

    format!(
        "{call}.style(|theme: &Theme| iced::widget::text::Style {{ color: Some({color}) }})"
    )
}

/// The style's icon-colour companion, if the input has a style that carries
/// one.
///
/// iced master dropped `text_input::Style::icon`, so the colour a style
/// defines for the icon lives in a function beside the style rather than in
/// it. See `styles::input_icon_color_fn`.
fn input_icon_color_fn(custom_styles: &CustomThemes, style_name: Option<&str>) -> Option<String> {
    let style_name = style_name?;

    if has_custom_style(custom_styles, ThemePaneEnum::TextInput, style_name) {
        return Some(format!(
            "styles::text_input::{}_icon",
            style_fn_name(style_name)
        ));
    }

    if has_custom_style(custom_styles, ThemePaneEnum::Combobox, style_name) {
        return Some(format!(
            "styles::combo_box::{}_input_style_icon",
            style_fn_name(style_name)
        ));
    }

    None
}

/// Opens the `row!` an input's icon is composed into, emitting the icon first
/// when it belongs on the left.
fn open_input_icon_row(b: &mut CodeBuilder, icon: &str, side: TextInputIconSide) {
    b.line("row![");
    b.increase_indent();

    if side == TextInputIconSide::Left {
        b.line(&format!("{icon},"));
    }

    b.indent();
}

/// Closes that `row!`, emitting the icon last when it belongs on the right.
fn close_input_icon_row(b: &mut CodeBuilder, icon: &str, side: TextInputIconSide, spacing: f32) {
    b.push(",");
    b.newline();

    if side == TextInputIconSide::Right {
        b.line(&format!("{icon},"));
    }

    b.decrease_indent();
    b.indent();
    b.push("]");
    b.increase_indent();
    b.dot_method("spacing", &format!("{spacing:.1}"));
    b.dot_method("align_y", "Alignment::Center");
    b.decrease_indent();
}

fn generate_icon(b: &mut CodeBuilder, widget: &Widget) {
    let props = &widget.properties;

    b.indent();
    b.push(&format!(
        "icon::{}()",
        super::icon_fn_name(&props.icon_family, &props.icon_name)
    ));
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
            Some(format!("styles::button_{}", style_fn_name(style)))
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
            b.add_menu_style("styles::menu", &style_fn_name(style_name));
        }
    }
}

fn generate_text_editor(
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
    let has_custom_name = !props.widget_name.trim().is_empty();
    let sname = to_snake_case(&name);

    b.indent();
    b.push("container(");
    b.newline();
    b.increase_indent();
    b.indent();
    if use_self {
        b.push(&format!("text_editor(&self.{}_content)", sname));
    } else {
        let source_text =
            escape_string_literal(&props.text_editor_content.text()).replace('\n', "\\n");
        b.push(&format!(
            "text_editor(&text_editor::Content::with_text(\"{}\"))",
            source_text
        ));
    }
    b.increase_indent();
    b.dot_method(
        "placeholder",
        &format!(
            "\"{}\"",
            escape_string_literal(&props.text_editor_placeholder)
        ),
    );
    b.dot_method("height", &format_length(props.height));
    if use_self {
        let variant = CodeBuilder::msg_variant(&name, "Edited", has_custom_name);
        b.dot_method("on_action", &format!("Message::{}", variant));
    }
    if props.text_editor_font != FontType::Default {
        b.add_font(props.text_editor_font);
    }
    if props.text_editor_size != 16.0 {
        b.add_size(props.text_editor_size);
    }
    if props.text_editor_line_height != LineHeight::default() {
        b.add_lineheight(&props.text_editor_line_height);
    }
    if props.text_editor_padding != 5.0 {
        b.dot_method("padding", &format!("{:.1}", props.text_editor_padding));
    }
    if props.text_editor_wrapping != iced::widget::text::Wrapping::default() {
        let wrapping = match props.text_editor_wrapping {
            iced::widget::text::Wrapping::None => "Wrapping::None",
            iced::widget::text::Wrapping::Word => "Wrapping::Word",
            iced::widget::text::Wrapping::Glyph => "Wrapping::Glyph",
            iced::widget::text::Wrapping::WordOrGlyph => "Wrapping::WordOrGlyph",
        };
        b.dot_method("wrapping", wrapping);
    }
    if let Some(style_name) = props.custom_style_name.as_deref() {
        if let Some(style_fn) = resolve_text_editor_style_fn(custom_styles, style_name) {
            b.dot_method("style", &style_fn);
        }
    }
    b.decrease_indent();
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
    if let Some(max_width) = props.max_width {
        b.add_max_width(max_width);
    }
    if let Some(max_height) = props.max_height {
        b.add_max_height(max_height);
    }
    if props.clip {
        b.add_clip();
    }
    b.decrease_indent();
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

    let icon = props.text_input_icon_enabled.then(|| {
        let color_fn = input_icon_color_fn(custom_styles, props.custom_style_name.as_deref());

        input_icon_expr(
            &props.text_input_icon_family,
            &props.text_input_icon_name,
            props.text_input_icon_size,
            color_fn.as_deref(),
        )
    });

    match &icon {
        Some(icon) => open_input_icon_row(b, icon, props.text_input_icon_side),
        None => b.indent(),
    }

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

    if !matches!(props.width, Length::Fill) {
        b.add_width(props.width);
    }

    if let Some(style_name) = props.custom_style_name.as_deref() {
        if has_custom_style(custom_styles, ThemePaneEnum::TextInput, style_name) {
            b.add_style("styles::text_input", &style_fn_name(style_name));
        }
    }

    b.decrease_indent();

    if let Some(icon) = &icon {
        close_input_icon_row(
            b,
            icon,
            props.text_input_icon_side,
            props.text_input_icon_spacing,
        );
    }
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

    let icon = props.combobox_icon_enabled.then(|| {
        let color_fn = input_icon_color_fn(custom_styles, props.custom_style_name.as_deref());

        input_icon_expr(
            &props.combobox_icon_family,
            &props.combobox_icon_name,
            props.combobox_icon_size,
            color_fn.as_deref(),
        )
    });

    match &icon {
        Some(icon) => open_input_icon_row(b, icon, props.combobox_icon_side),
        None => b.indent(),
    }

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

    let mut uses_split_styles = false;
    if let Some(style_name) = props.custom_style_name.as_deref() {
        if has_custom_style(custom_styles, ThemePaneEnum::TextInput, style_name) {
            b.add_input_style("styles::text_input", &style_fn_name(style_name));
            uses_split_styles = true;
        }
    }

    if let Some(style_name) = props.menu_style_name.as_deref() {
        if has_custom_style(custom_styles, ThemePaneEnum::Menu, style_name) {
            b.add_menu_style("styles::menu", &style_fn_name(style_name));
            uses_split_styles = true;
        }
    }

    if !uses_split_styles {
        if let Some(style) = props.custom_style_name.as_deref() {
            if has_custom_style(custom_styles, ThemePaneEnum::Combobox, style) {
                b.add_input_style(
                    "styles::combo_box",
                    &format!("{}_input_style", style_fn_name(style)),
                );
                b.add_menu_style(
                    "styles::combo_box",
                    &format!("{}_menu_style", style_fn_name(style)),
                );
            }
        }
    }

    if let Some(icon) = &icon {
        close_input_icon_row(b, icon, props.combobox_icon_side, props.combobox_icon_spacing);
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

fn split_strategy_code(strategy: SplitStrategy) -> &'static str {
    match strategy {
        SplitStrategy::Relative => "iced_split::Strategy::Relative",
        SplitStrategy::Start => "iced_split::Strategy::Start",
        SplitStrategy::End => "iced_split::Strategy::End",
    }
}

fn generate_child_or_placeholder(
    b: &mut CodeBuilder,
    child: Option<&Widget>,
    placeholder: &str,
    names: &HashMap<WidgetId, String>,
    custom_styles: &CustomThemes,
    use_self: bool,
    type_system: &TypeSystem,
    view_refs: &[ViewRefInfo],
) {
    if let Some(child) = child {
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
        b.push(&format!(
            "container(text(\"{}\")).center(Length::Fill)",
            placeholder
        ));
    }
}

fn generate_split(
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
    let split_at = if use_self {
        format!("self.{}_split_at", to_snake_case(&name))
    } else {
        format!("{:.4}", props.split_at)
    };
    let on_drag = if use_self {
        format!(
            "Message::{}",
            CodeBuilder::msg_variant(&name, "Changed", has_custom_name)
        )
    } else {
        "|_| Message::Noop".to_string()
    };
    let constructor = match props.orientation {
        Orientation::Vertical => "iced_split::vertical_split",
        Orientation::Horizontal => "iced_split::horizontal_split",
    };

    b.indent();
    b.push("container(");
    b.newline();
    b.increase_indent();

    b.indent();
    b.push(&format!("{}(", constructor));
    b.newline();
    b.increase_indent();

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
        b.push("container(text(\"Start Pane\")).center(Length::Fill)");
    }
    b.push(",");
    b.newline();

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
        b.push("container(text(\"End Pane\")).center(Length::Fill)");
    }
    b.push(",");
    b.newline();

    b.indent();
    b.push(&split_at);
    b.push(",");
    b.newline();

    b.indent();
    b.push(&on_drag);
    b.newline();

    b.decrease_indent();
    b.indent();
    b.push(")");
    b.increase_indent();
    b.dot_method("strategy", split_strategy_code(props.split_strategy));
    if (props.split_handle_width - 11.0).abs() > f32::EPSILON {
        b.dot_method("handle_width", &format!("{:.1}", props.split_handle_width));
    }
    b.decrease_indent();
    b.newline();
    b.decrease_indent();
    b.indent();
    b.push(")");

    if !matches!(props.width, Length::Shrink) {
        b.add_width(props.width);
    }
    if !matches!(props.height, Length::Shrink) {
        b.add_height(props.height);
    }
    if let Some(max_width) = props.max_width {
        b.add_max_width(max_width);
    }
    if let Some(max_height) = props.max_height {
        b.add_max_height(max_height);
    }
    if props.clip {
        b.add_clip();
    }
}

fn generate_float(
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

    b.indent();
    b.push("float(");
    b.newline();
    b.increase_indent();
    generate_child_or_placeholder(
        b,
        widget.children.first(),
        "Float Content",
        names,
        custom_styles,
        use_self,
        type_system,
        view_refs,
    );
    b.newline();
    b.decrease_indent();
    b.indent();
    b.push(")");
    b.increase_indent();
    if (props.float_scale - 1.0).abs() > f32::EPSILON {
        b.dot_method("scale", &format!("{:.2}", props.float_scale));
    }
    if props.float_translate_x != 0.0 || props.float_translate_y != 0.0 {
        b.dot_method(
            "translate",
            &format!(
                "move |_, _| Vector::new({:.1}, {:.1})",
                props.float_translate_x, props.float_translate_y
            ),
        );
    }
    b.decrease_indent();
    b.newline();
    b.decrease_indent();
    b.indent();
    b.push(")");

    if !matches!(props.width, Length::Shrink) {
        b.add_width(props.width);
    }
    if !matches!(props.height, Length::Shrink) {
        b.add_height(props.height);
    }
    if let Some(max_width) = props.max_width {
        b.add_max_width(max_width);
    }
    if let Some(max_height) = props.max_height {
        b.add_max_height(max_height);
    }
    if props.clip {
        b.add_clip();
    }
}

fn generate_responsive(
    b: &mut CodeBuilder,
    widget: &Widget,
    names: &HashMap<WidgetId, String>,
    custom_styles: &CustomThemes,
    use_self: bool,
    type_system: &TypeSystem,
    view_refs: &[ViewRefInfo],
) {
    let props = &widget.properties;
    let compact = widget.children.first();
    let expanded = widget.children.get(1);
    let breakpoint = props.responsive_breakpoint.max(0.0);

    b.indent();
    b.push("container(");
    b.newline();
    b.increase_indent();

    b.line("responsive(move |size| {");
    b.increase_indent();
    b.line(&format!("if size.width <= {:.1} {{", breakpoint));
    b.increase_indent();
    generate_child_or_placeholder(
        b,
        compact.or(expanded),
        "Compact View",
        names,
        custom_styles,
        use_self,
        type_system,
        view_refs,
    );
    b.newline();
    b.decrease_indent();
    b.line("} else {");
    b.increase_indent();
    generate_child_or_placeholder(
        b,
        expanded.or(compact),
        "Expanded View",
        names,
        custom_styles,
        use_self,
        type_system,
        view_refs,
    );
    b.newline();
    b.decrease_indent();
    b.line("}");
    b.decrease_indent();
    b.indent();
    b.push("})");
    b.increase_indent();
    if !matches!(props.width, Length::Fill) {
        b.add_width(props.width);
    }
    if !matches!(props.height, Length::Fill) {
        b.add_height(props.height);
    }
    b.decrease_indent();
    b.newline();
    b.decrease_indent();
    b.indent();
    b.push(")");

    if let Some(max_width) = props.max_width {
        b.add_max_width(max_width);
    }
    if let Some(max_height) = props.max_height {
        b.add_max_height(max_height);
    }
    if props.clip {
        b.add_clip();
    }
}

fn generate_sensor(
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
    let pascal_name = to_pascal_case(&name);

    b.indent();
    b.push("container(");
    b.newline();
    b.increase_indent();

    b.indent();
    b.push("sensor(");
    b.newline();
    b.increase_indent();
    generate_child_or_placeholder(
        b,
        widget.children.first(),
        "Sensor Content",
        names,
        custom_styles,
        use_self,
        type_system,
        view_refs,
    );
    b.newline();
    b.decrease_indent();
    b.indent();
    b.push(")");
    b.increase_indent();
    if props.sensor_on_show {
        b.dot_method(
            "on_show",
            &format!(
                "|size| Message::{}Shown(size.width, size.height)",
                pascal_name
            ),
        );
    }
    if props.sensor_on_resize {
        b.dot_method(
            "on_resize",
            &format!(
                "|size| Message::{}Resized(size.width, size.height)",
                pascal_name
            ),
        );
    }
    if props.sensor_on_hide {
        b.dot_method("on_hide", &format!("Message::{}Hidden", pascal_name));
    }
    if props.sensor_anticipate > 0.0 {
        b.dot_method("anticipate", &format!("{:.1}", props.sensor_anticipate));
    }
    if props.sensor_delay_ms > 0 {
        b.dot_method(
            "delay",
            &format!(
                "std::time::Duration::from_millis({})",
                props.sensor_delay_ms
            ),
        );
    }
    b.decrease_indent();
    b.newline();
    b.decrease_indent();
    b.indent();
    b.push(")");

    if !matches!(props.width, Length::Shrink) {
        b.add_width(props.width);
    }
    if !matches!(props.height, Length::Shrink) {
        b.add_height(props.height);
    }
    if let Some(max_width) = props.max_width {
        b.add_max_width(max_width);
    }
    if let Some(max_height) = props.max_height {
        b.add_max_height(max_height);
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

/// Emits the child at `index`, or a stand-in when there is none.
fn generate_overlay_child(
    b: &mut CodeBuilder,
    widget: &Widget,
    index: usize,
    placeholder: &str,
    names: &HashMap<WidgetId, String>,
    custom_styles: &CustomThemes,
    use_self: bool,
    type_system: &TypeSystem,
    view_refs: &[ViewRefInfo],
) {
    match (use_self, widget.children.get(index)) {
        (true, Some(child)) => generate_widget_code(
            b,
            child,
            names,
            use_self,
            custom_styles,
            type_system,
            view_refs,
        ),
        (true, None) if index == 0 => {
            // The trigger slot falls back to a button carrying the widget's own
            // size, padding and style, matching what the preview draws.
            let props = &widget.properties;

            let label = if props.text_content.is_empty() {
                placeholder
            } else {
                &props.text_content
            };

            b.indent();
            b.push(&format!("iced::widget::button(iced::widget::text(\"{label}\"))"));

            if !matches!(props.width, Length::Fit) {
                b.push(&format!(".width({})", format_length(props.width)));
            }
            if !matches!(props.height, Length::Fit) {
                b.push(&format!(".height({})", format_length(props.height)));
            }

            if let Some(style_fn) = props
                .custom_style_name
                .as_deref()
                .and_then(|name| resolve_overlay_trigger_style_fn(custom_styles, name))
            {
                b.push(&format!(".style({style_fn})"));
            }
        }
        (true, None) => {
            b.indent();
            b.push(&format!("iced::widget::text(\"{placeholder}\")"));
        }
        (false, _) => {
            b.indent();
            b.push(&format!("// {placeholder}"));
        }
    }
}

/// The `widgets::animation::Motion` constant for an animation preset.
fn overlay_motion_code(preset: OverlayAnimationPreset) -> &'static str {
    match preset {
        OverlayAnimationPreset::Default => "widgets::animation::Motion::SMOOTH",
        OverlayAnimationPreset::Quick => "widgets::animation::Motion::QUICK",
        OverlayAnimationPreset::Slow => {
            "widgets::animation::Motion::SMOOTH.duration(std::time::Duration::from_millis(420))"
        }
    }
}

/// Emits the shared surface settings of an anchored overlay.
/// The values a widget's own constructor starts from.
///
/// Generated code only calls a builder method when the project differs from
/// this, so the output reads like hand-written code rather than a full
/// restatement of every default. Each widget has its own set, which is why
/// this is passed in rather than being a set of constants.
struct OverlayDefaults {
    side: OverlayPosition,
    align: ContainerAlignX,
    gap: f32,
    padding: f32,
    radius: f32,
    motion: OverlayAnimationPreset,
}

impl OverlayDefaults {
    /// `widgets::popover::Popover::new`
    const POPOVER: Self = Self {
        side: OverlayPosition::Bottom,
        align: ContainerAlignX::Center,
        gap: 6.0,
        padding: 12.0,
        radius: 8.0,
        motion: OverlayAnimationPreset::Quick,
    };

    /// `widgets::menu::Menu::new`
    const MENU: Self = Self {
        side: OverlayPosition::Bottom,
        align: ContainerAlignX::Left,
        gap: 4.0,
        padding: 4.0,
        radius: 6.0,
        motion: OverlayAnimationPreset::Quick,
    };

    /// `widgets::layer::Layer::new`
    const LAYER: Self = Self {
        side: OverlayPosition::Bottom,
        align: ContainerAlignX::Center,
        gap: 16.0,
        padding: 16.0,
        radius: 10.0,
        motion: OverlayAnimationPreset::Default,
    };
}

/// Emits the surface settings shared by the anchored overlays.
fn generate_anchored_common(
    b: &mut CodeBuilder,
    props: &Properties,
    defaults: &OverlayDefaults,
    include_side_extras: bool,
) {
    if props.overlay_hover_position != defaults.side {
        b.dot_method("side", overlay_position_code(props.overlay_hover_position));
    }
    if props.overlay_hover_alignment != defaults.align {
        b.dot_method(
            "align",
            overlay_alignment_code(props.overlay_hover_alignment),
        );
    }

    // `PositionMode::Inside` had no counterpart in the new widgets; a negative
    // gap is what pulls a surface back over its trigger.
    let gap = props.overlay_hover_mode.signed_gap(props.overlay_hover_gap);

    if (gap - defaults.gap).abs() > f32::EPSILON {
        b.dot_method("gap", &format!("{gap:.1}"));
    }

    if include_side_extras && !props.overlay_hover_snap {
        b.dot_method("flip", "false");
    }

    if props.overlay_min_width > 0.0 {
        b.dot_method("min_width", &format!("{:.1}", props.overlay_min_width));
    }
    if let Some(max) = props.overlay_max_width {
        b.dot_method("max_width", &format!("{max:.1}"));
    }

    if (props.overlay_padding - defaults.padding).abs() > f32::EPSILON {
        b.dot_method("padding", &format!("{:.1}", props.overlay_padding));
    }
    if (props.overlay_radius - defaults.radius).abs() > f32::EPSILON {
        b.dot_method("radius", &format!("{:.1}", props.overlay_radius));
    }

    if props.overlay_animate {
        if props.overlay_animation_preset != defaults.motion {
            b.dot_method(
                "motion",
                overlay_motion_code(props.overlay_animation_preset),
            );
        }
    } else {
        b.dot_method_no_args("no_animation");
    }
}

fn generate_popover(
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

    b.indent();
    b.push("widgets::popover::popover(");
    b.newline();
    b.increase_indent();

    generate_overlay_child(
        b,
        widget,
        0,
        "Trigger",
        names,
        custom_styles,
        use_self,
        type_system,
        view_refs,
    );
    b.push(",");
    b.newline();
    generate_overlay_child(
        b,
        widget,
        1,
        "Popover content",
        names,
        custom_styles,
        use_self,
        type_system,
        view_refs,
    );
    b.newline();
    b.decrease_indent();
    b.indent();
    b.push(")");
    b.increase_indent();

    if let Some(ref id) = props.widget_id
        && !id.is_empty()
    {
        b.add_id(id);
    }

    if props.overlay_on_hover {
        b.dot_method_no_args("on_hover");
    }

    generate_anchored_common(b, props, &OverlayDefaults::POPOVER, true);

    if props.overlay_close_on_click_outside {
        b.dot_method("dismiss_on_outside_press", "true");
    }
    if !props.overlay_safe_triangle {
        b.dot_method("safe_corridor", "false");
    }

    if use_self {
        let variant = CodeBuilder::msg_variant(&name, "Toggled", has_custom_name);
        b.dot_method("open", &format!("self.{}_open", to_snake_case(&name)));
        b.dot_method("on_toggle", &format!("Message::{variant}"));
    } else {
        b.dot_method("open", "false");
        b.newline();
        b.indent();
        b.push(".on_toggle(|_| Message::Noop)");
    }

    if let Some(ref style_name) = props.overlay_style_name
        && let Some(style_fn) = resolve_overlay_style_fn("popover", style_name)
    {
        b.dot_method("style", &style_fn);
    }

    b.decrease_indent();
}

/// One row of a menu panel.
///
/// The kind is not decoration: a Submenu emitted as a plain entry silently
/// loses its nested rows, and a Toggle loses its checkmark. The preview reads
/// it through `menu_item_for`, and this is its counterpart.
fn generate_menu_row(
    b: &mut CodeBuilder,
    child: &Widget,
    names: &HashMap<WidgetId, String>,
    custom_styles: &CustomThemes,
    use_self: bool,
    type_system: &TypeSystem,
    view_refs: &[ViewRefInfo],
) {
    let props = &child.properties;

    // A rule drawn across the panel: no content, so nothing else applies.
    if props.menu_item_kind == MenuItemKind::Separator {
        b.line("widgets::menu::separator(),");
        return;
    }

    b.indent();
    b.push(match props.menu_item_kind {
        MenuItemKind::Submenu => "widgets::menu::submenu(",
        MenuItemKind::Toggle => "widgets::menu::toggle(",
        _ => "widgets::menu::item(",
    });
    b.newline();
    b.increase_indent();

    generate_widget_code(
        b,
        child,
        names,
        use_self,
        custom_styles,
        type_system,
        view_refs,
    );

    match props.menu_item_kind {
        MenuItemKind::Submenu => {
            b.push(",");
            b.newline();
            b.line("vec![");
            b.increase_indent();

            for grandchild in &child.children {
                generate_menu_row(
                    b,
                    grandchild,
                    names,
                    custom_styles,
                    use_self,
                    type_system,
                    view_refs,
                );
            }

            b.decrease_indent();
            b.line("],");
        }
        MenuItemKind::Toggle => {
            b.push(",");
            b.newline();
            b.line(&format!("{},", props.menu_item_checked));
        }
        _ => b.newline(),
    }

    b.decrease_indent();
    b.indent();
    b.push(")");
    b.increase_indent();

    // The panel indents every row past the widest icon, so a row with one
    // does not push its neighbours' labels out of line.
    if props.menu_item_icon_enabled {
        b.dot_method(
            "icon",
            &icon_expr(
                &props.menu_item_icon_family,
                &props.menu_item_icon_name,
                props.menu_item_icon_size,
            ),
        );
    }

    if !props.menu_item_enabled {
        b.dot_method("enabled", "false");
    }

    b.decrease_indent();
    b.push(",");
    b.newline();
}

fn generate_menu(
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

    b.indent();
    b.push("widgets::menu::menu(");
    b.newline();
    b.increase_indent();

    generate_overlay_child(
        b,
        widget,
        0,
        "Trigger",
        names,
        custom_styles,
        use_self,
        type_system,
        view_refs,
    );
    b.push(",");
    b.newline();

    // Every child after the trigger is one row of the panel.
    b.indent();
    b.push("vec![");
    b.newline();
    b.increase_indent();

    if use_self && widget.children.len() > 1 {
        for child in &widget.children[1..] {
            generate_menu_row(
                b,
                child,
                names,
                custom_styles,
                use_self,
                type_system,
                view_refs,
            );
        }
    } else {
        b.indent();
        b.push("// menu items");
        b.newline();
    }

    b.decrease_indent();
    b.indent();
    b.push("],");
    b.newline();
    b.decrease_indent();
    b.indent();
    b.push(")");
    b.increase_indent();

    if let Some(ref id) = props.widget_id
        && !id.is_empty()
    {
        b.add_id(id);
    }

    generate_anchored_common(b, props, &OverlayDefaults::MENU, false);

    // The panel chrome a menu has and the other overlays do not.
    if (props.menu_submenu_gap - (-4.0)).abs() > f32::EPSILON {
        b.dot_method("submenu_gap", &format!("{:.1}", props.menu_submenu_gap));
    }
    if props.menu_item_padding != Padding::from([4.0, 8.0]) {
        b.dot_method("item_padding", &format_padding(&props.menu_item_padding));
    }
    if props.menu_trigger_padding != Padding::from([4.0, 10.0]) {
        b.dot_method(
            "trigger_padding",
            &format_padding(&props.menu_trigger_padding),
        );
    }
    if props.menu_plain_trigger {
        b.dot_method_no_args("plain_trigger");
    }
    if props.menu_check_side != MenuCheckSide::Leading {
        b.dot_method("check_side", "widgets::menu::CheckSide::Trailing");
    }
    if (props.menu_gutter_spacing - 8.0).abs() > f32::EPSILON {
        b.dot_method(
            "gutter_spacing",
            &format!("{:.1}", props.menu_gutter_spacing),
        );
    }

    if use_self {
        let variant = CodeBuilder::msg_variant(&name, "Toggled", has_custom_name);
        b.dot_method("on_toggle", &format!("Message::{variant}"));
    } else {
        b.newline();
        b.indent();
        b.push(".on_toggle(|_| Message::Noop)");
    }

    if let Some(ref style_name) = props.overlay_style_name
        && let Some(style_fn) = resolve_overlay_style_fn("menu", style_name)
    {
        b.dot_method("style", &style_fn);
    }

    b.decrease_indent();
}

fn generate_layer(
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

    b.indent();
    b.push("widgets::layer::layer(");
    b.newline();
    b.increase_indent();

    generate_overlay_child(
        b,
        widget,
        0,
        "Layer content",
        names,
        custom_styles,
        use_self,
        type_system,
        view_refs,
    );

    b.newline();
    b.decrease_indent();
    b.indent();
    b.push(")");
    b.increase_indent();

    if let Some(ref id) = props.widget_id
        && !id.is_empty()
    {
        b.add_id(id);
    }

    if use_self {
        b.dot_method("open", &format!("self.{}_open", to_snake_case(&name)));
    } else {
        b.dot_method("open", "false");
    }

    // A layer is placed against the viewport rather than against a trigger, so
    // it carries its own anchor and the gap becomes a margin.
    if props.layer_anchor != OverlayAnchor::Center {
        b.dot_method("anchor", overlay_anchor_code(props.layer_anchor));
    }
    if props.layer_position_enabled {
        // An explicit point overrides the anchor entirely.
        b.dot_method(
            "at",
            &format!(
                "iced::Point::new({:.1}, {:.1})",
                props.layer_position_x, props.layer_position_y
            ),
        );
    }
    if (props.overlay_hover_gap - OverlayDefaults::LAYER.gap).abs() > f32::EPSILON {
        b.dot_method("margin", &format!("{:.1}", props.overlay_hover_gap));
    }

    if !props.overlay_hide_header {
        b.dot_method("title", &format!("\"{}\"", props.overlay_title));
    }
    // `Layer::new` draws a close button, is neither draggable nor resizable,
    // has no backdrop, and dismisses on both the backdrop and Escape.
    if props.overlay_hide_close_button {
        b.dot_method("close_button", "false");
    }
    if !props.overlay_block_dragging {
        b.dot_method("draggable", "true");
    }
    if props.overlay_resizable.is_resizable() {
        b.dot_method("resizable", "true");
    }
    if props.layer_stretch {
        b.dot_method("stretch", "true");
    }
    if props.overlay_opaque {
        b.dot_method("backdrop", &format!("{:.2}", props.overlay_opaque_alpha));
    }
    if !props.overlay_close_on_click_outside {
        b.dot_method("dismiss_on_backdrop", "false");
    }
    if !props.layer_dismiss_on_escape {
        b.dot_method("dismiss_on_escape", "false");
    }

    if props.overlay_min_width > 0.0 {
        b.dot_method("min_width", &format!("{:.1}", props.overlay_min_width));
    }
    if let Some(max) = props.overlay_max_width {
        b.dot_method("max_width", &format!("{max:.1}"));
    }
    if props.layer_min_height > 0.0 {
        b.dot_method("min_height", &format!("{:.1}", props.layer_min_height));
    }
    if (props.overlay_padding - OverlayDefaults::LAYER.padding).abs() > f32::EPSILON {
        b.dot_method("padding", &format!("{:.1}", props.overlay_padding));
    }
    if (props.overlay_radius - OverlayDefaults::LAYER.radius).abs() > f32::EPSILON {
        b.dot_method("radius", &format!("{:.1}", props.overlay_radius));
    }

    if props.overlay_animate {
        if props.overlay_animation_preset != OverlayDefaults::LAYER.motion {
            b.dot_method(
                "motion",
                overlay_motion_code(props.overlay_animation_preset),
            );
        }
    } else {
        b.dot_method_no_args("no_animation");
    }

    // A layer has no trigger of its own, so the only message it sends is the
    // one that closes it.
    if use_self {
        let variant = CodeBuilder::msg_variant(&name, "Toggled", has_custom_name);
        b.dot_method("on_dismiss", &format!("Message::{variant}(false)"));
    } else {
        b.dot_method("on_dismiss", "Message::Noop");
    }

    if let Some(ref style_name) = props.overlay_style_name
        && let Some(style_fn) = resolve_overlay_style_fn("layer", style_name)
    {
        b.dot_method("style", &style_fn);
    }

    b.decrease_indent();
}

fn generate_toast(
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

    b.indent();
    b.push("widgets::toast::toasts(");
    b.newline();
    b.increase_indent();
    b.indent();
    b.push("vec![");
    b.newline();
    b.increase_indent();

    // Each child is one notification, carrying its own variant and timeout.
    if use_self && !widget.children.is_empty() {
        for child in &widget.children {
            let child_props = &child.properties;

            b.indent();
            b.push("widgets::toast::toast(");
            b.newline();
            b.increase_indent();
            b.indent();
            b.push(&format!("{}u64,", child.id.0));
            b.newline();
            generate_widget_code(
                b,
                child,
                names,
                use_self,
                custom_styles,
                type_system,
                view_refs,
            );
            b.newline();
            b.decrease_indent();
            b.indent();
            b.push(")");

            if child_props.toast_variant != ToastVariant::Neutral {
                b.push(&format!(
                    ".variant({})",
                    toast_variant_code(child_props.toast_variant)
                ));
            }
            if child_props.toast_without_icon {
                b.push(".without_icon()");
            }
            if child_props.toast_timeout_secs > 0.0 {
                b.push(&format!(
                    ".timeout(std::time::Duration::from_secs_f32({:.1}))",
                    child_props.toast_timeout_secs
                ));
            }

            b.push(",");
            b.newline();
        }
    } else {
        b.indent();
        b.push("// toasts");
        b.newline();
    }

    b.decrease_indent();
    b.indent();
    b.push("],");
    b.newline();
    b.decrease_indent();
    b.indent();
    b.push(")");
    b.increase_indent();

    if let Some(ref id) = props.widget_id
        && !id.is_empty()
    {
        b.add_id(id);
    }

    if props.toast_anchor != OverlayAnchor::BottomRight {
        b.dot_method("anchor", overlay_anchor_code(props.toast_anchor));
    }
    if (props.toast_margin - 16.0).abs() > f32::EPSILON {
        b.dot_method("margin", &format!("{:.1}", props.toast_margin));
    }
    if (props.toast_spacing - 10.0).abs() > f32::EPSILON {
        b.dot_method("spacing", &format!("{:.1}", props.toast_spacing));
    }
    if (props.overlay_padding - 14.0).abs() > f32::EPSILON {
        b.dot_method("padding", &format!("{:.1}", props.overlay_padding));
    }
    if (props.overlay_radius - 8.0).abs() > f32::EPSILON {
        b.dot_method("radius", &format!("{:.1}", props.overlay_radius));
    }
    if (props.toast_width - 360.0).abs() > f32::EPSILON {
        b.dot_method("width", &format!("{:.1}", props.toast_width));
    }

    if props.overlay_animate {
        if props.overlay_animation_preset != OverlayAnimationPreset::Default {
            b.dot_method(
                "motion",
                overlay_motion_code(props.overlay_animation_preset),
            );
        }
    } else {
        b.dot_method_no_args("no_animation");
    }

    // A toast's style is per-variant, so the swatch presets the other three
    // overlays offer cannot apply here — the panel offers only the M3 style,
    // which keeps the variants apart.
    if props.overlay_style_name.as_deref() == Some(crate::controls::MATERIAL_STYLE) {
        b.dot_method("style", "widgets::m3::toast::material");
    }

    let _ = (name, has_custom_name);

    b.decrease_indent();
}

/// The path to a `toast::Variant`, for generated code.
fn toast_variant_code(variant: ToastVariant) -> &'static str {
    match variant {
        ToastVariant::Neutral => "widgets::toast::Variant::Neutral",
        ToastVariant::Info => "widgets::toast::Variant::Info",
        ToastVariant::Warning => "widgets::toast::Variant::Warning",
        ToastVariant::Success => "widgets::toast::Variant::Success",
        ToastVariant::Danger => "widgets::toast::Variant::Danger",
    }
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
        .and_then(|style_name| resolve_overlay_trigger_style_fn(custom_styles, style_name));
    let alternate_style = props
        .active_style_name
        .as_deref()
        .and_then(|style_name| resolve_overlay_trigger_style_fn(custom_styles, style_name));
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

/// The `advanced-table` data grid.
///
/// Every part of this widget that reports something back leaves the state with
/// the host: it renders the selection and the sort it is handed, and says what
/// each should become. So a table with selection or sorting turned on has
/// fields in the generated app (see `app.rs`) and messages that write them
/// (see `events.rs`), and this is where they are read back out.
fn generate_data_table(
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
    let field = to_snake_case(&name);
    // A data table emits several messages, so they all keep their suffix --
    // `msg_variant` drops it for a named widget, which would collapse them
    // into one another.
    let variant = |suffix: &str| format!("{}{suffix}", to_pascal_case(&name));

    let struct_def = props
        .data_table_referenced_struct
        .and_then(|id| type_system.get_struct(id));

    let Some(struct_def) = struct_def else {
        b.indent();
        b.push("text(\"Data Table: select a struct type\")");
        return;
    };

    if props.data_table_columns.is_empty() {
        b.indent();
        b.push(&format!(
            "text(\"Data Table: {} has no fields\")",
            struct_def.name
        ));
        return;
    }

    b.indent();
    b.push("advanced_table::data_table(");
    b.newline();
    b.increase_indent();

    generate_data_table_header(b, props);

    b.decrease_indent();
    b.indent();
    b.push(")");
    b.increase_indent();

    // ── The body ───────────────────────────────────────────────────────────
    // The view function is indexed by *leaf column*, which is the order the
    // columns are in here rather than the order the struct declares its
    // fields in.
    let rows = if use_self {
        format!("&self.{field}_rows")
    } else {
        "&rows".to_string()
    };

    b.newline();
    b.indent();
    b.push(&format!(
        ".rows({rows}, |row: &{}, column| match column {{",
        struct_def.name
    ));
    b.newline();
    b.increase_indent();

    let last = props.data_table_columns.len() - 1;

    for (index, column) in props.data_table_columns.iter().enumerate() {
        let cell = data_table_cell_expr(struct_def, &column.field);

        b.indent();
        // The last arm is the catch-all: `column` is a `usize`, so a match on
        // it needs one whatever the column count.
        if index == last {
            b.push(&format!("_ => {cell},"));
        } else {
            b.push(&format!("{index} => {cell},"));
        }
        b.newline();
    }

    b.decrease_indent();
    b.indent();
    b.push("})");

    // ── The scalars, each omitted when it matches the widget's own default ──
    if props.data_table_spacing != 0.0 {
        b.dot_method("spacing", &format!("{:.1}", props.data_table_spacing));
    }

    let padding = Padding {
        top: props.data_table_padding_y,
        right: props.data_table_padding_x,
        bottom: props.data_table_padding_y,
        left: props.data_table_padding_x,
    };

    if props.data_table_padding_y != 6.0 || props.data_table_padding_x != 10.0 {
        b.dot_method("padding", &format_padding(&padding));
    }

    if props.data_table_overflow != DataTableOverflow::Scroll {
        b.dot_method("overflow", "advanced_table::Overflow::Shrink");
    }

    if props.data_table_measure_all {
        b.dot_method_no_args("measure_all");
    } else if props.data_table_measure_sample != 200.0 {
        b.dot_method(
            "measure_sample",
            &format!("{}", props.data_table_measure_sample.max(0.0) as usize),
        );
    }

    if props.data_table_min_row_height != 0.0 {
        b.dot_method(
            "min_row_height",
            &format!("{:.1}", props.data_table_min_row_height),
        );
    }

    if props.data_table_scrollbar_width != 10.0 {
        b.dot_method(
            "scrollbar_width",
            &format!("{:.1}", props.data_table_scrollbar_width),
        );
    }

    if props.data_table_vertical_scroll != DataTableScrollPolicy::Auto {
        b.dot_method("vertical_scroll", "advanced_table::Policy::Hidden");
    }

    if props.data_table_horizontal_scroll != DataTableScrollPolicy::Auto {
        b.dot_method("horizontal_scroll", "advanced_table::Policy::Hidden");
    }

    if props.data_table_gutter != 8.0 {
        b.dot_method("gutter", &format!("{:.1}", props.data_table_gutter));
    }

    if !props.data_table_resizable {
        b.dot_method("resizable", "false");
    }

    if props.data_table_min_column_width != 32.0 {
        b.dot_method(
            "min_column_width",
            &format!("{:.1}", props.data_table_min_column_width),
        );
    }

    // ── What it reports ────────────────────────────────────────────────────
    // Only with `use_self`: without somewhere to store what comes back, a
    // selection would render from nothing and a message would have no arm.
    if use_self {
        if props.data_table_row_selection.is_on() {
            let variant = variant("RowsSelected");

            b.dot_method(
                "selection",
                &format!(
                    "{}, &self.{field}_selected_rows, Message::{variant}",
                    props.data_table_row_selection.code()
                ),
            );
        }

        if props.data_table_column_selection.is_on() {
            let variant = variant("ColumnsSelected");

            b.dot_method(
                "column_selection",
                &format!(
                    "{}, &self.{field}_selected_columns, Message::{variant}",
                    props.data_table_column_selection.code()
                ),
            );
        }

        if props.data_table_cell_selection.is_on() {
            let variant = variant("CellsSelected");

            b.dot_method(
                "cell_selection",
                &format!(
                    "{}, &self.{field}_selected_cells, Message::{variant}",
                    props.data_table_cell_selection.code()
                ),
            );
        }

        if props.data_table_sorting {
            let variant = variant("Sorted");

            b.dot_method(
                "sorting",
                &format!("self.{field}_sort, Message::{variant}"),
            );
        }

        if props.data_table_on_copy {
            let variant = variant("Copied");

            b.dot_method("on_copy", &format!("|| Message::{variant}"));
        }

        if props.data_table_on_right_click {
            let variant = variant("RightClicked");

            b.dot_method("on_right_click", &format!("Message::{variant}"));
        }
    }

    if !matches!(props.width, Length::Fill) {
        b.add_width(props.width);
    }

    if !matches!(props.height, Length::Fill) {
        b.add_height(props.height);
    }

    if let Some(ref id) = props.widget_id
        && !id.is_empty()
    {
        b.add_id(id);
    }

    b.decrease_indent();
}

/// The header tree: a leaf per column, wrapped in a group wherever a run of
/// columns names one.
///
/// Grouped columns are always adjacent — `Properties` keeps them that way when
/// a column joins or moves — so a run is a stretch of equal group indices.
fn generate_data_table_header(b: &mut CodeBuilder, props: &Properties) {
    b.line("vec![");
    b.increase_indent();

    let mut index = 0;

    while index < props.data_table_columns.len() {
        let column = &props.data_table_columns[index];

        let Some(group) = column.group else {
            b.indent();
            b.push(&format!(
                "advanced_table::leaf({})",
                data_table_header_expr(props, &data_table_column_label(column))
            ));
            b.increase_indent();
            generate_data_table_leaf_calls(b, column);

            if column.sticky {
                b.dot_method_no_args("sticky");
            }

            b.push(",");
            b.decrease_indent();
            b.newline();

            index += 1;
            continue;
        };

        let end = props.data_table_columns[index..]
            .iter()
            .position(|candidate| candidate.group != Some(group))
            .map_or(props.data_table_columns.len(), |offset| index + offset);

        let definition = props.data_table_groups.get(group);

        let label = definition
            .map(|definition| definition.header.clone())
            .filter(|header| !header.trim().is_empty())
            .unwrap_or_else(|| format!("Group {}", group + 1));

        b.indent();
        b.push(&format!(
            "advanced_table::group({}, vec![",
            data_table_header_expr(props, &label)
        ));
        b.newline();
        b.increase_indent();

        for column in &props.data_table_columns[index..end] {
            b.indent();
            b.push(&format!(
                "advanced_table::leaf({})",
                data_table_header_expr(props, &data_table_column_label(column))
            ));
            b.increase_indent();
            generate_data_table_leaf_calls(b, column);
            b.push(",");
            b.decrease_indent();
            b.newline();
        }

        b.decrease_indent();
        b.indent();
        b.push("])");
        b.increase_indent();

        // A sticky group takes every column beneath it, which is what keeps
        // one from being split by the frozen boundary.
        if definition.is_some_and(|definition| definition.sticky) {
            b.dot_method_no_args("sticky");
        }

        b.push(",");
        b.decrease_indent();
        b.newline();

        index = end;
    }

    b.decrease_indent();
    b.line("]");
}

/// The calls that describe one leaf column: how wide, how aligned, and whether
/// it sorts.
fn generate_data_table_leaf_calls(b: &mut CodeBuilder, column: &DataTableColumn) {
    let sizing = match column.sizing {
        DataTableSizing::Fixed => {
            format!("advanced_table::Sizing::Fixed({:.1})", column.width)
        }
        DataTableSizing::Fit => format!(
            "advanced_table::Sizing::Fit {{ min: {:.1}, max: {} }}",
            column.min_width,
            // No cap entered means no cap, not a zero-width column.
            if column.max_width > 0.0 {
                format!("{:.1}", column.max_width)
            } else {
                "f32::INFINITY".to_string()
            }
        ),
        DataTableSizing::Fill => format!(
            "advanced_table::Sizing::Fill {{ weight: {}, min: {:.1} }}",
            column.weight.max(1),
            column.min_width
        ),
    };

    b.dot_method("sizing", &sizing);

    if column.align != ContainerAlignX::Left {
        b.dot_method(
            "align",
            match column.align {
                ContainerAlignX::Center => "iced::alignment::Horizontal::Center",
                ContainerAlignX::Right => "iced::alignment::Horizontal::Right",
                ContainerAlignX::Left => "iced::alignment::Horizontal::Left",
            },
        );
    }

    if column.sortable {
        b.dot_method_no_args("sortable");
    }
}

/// A header cell's content expression.
fn data_table_header_expr(props: &Properties, label: &str) -> String {
    if props.data_table_bold_headers {
        format!(
            "text(\"{label}\").font(Font {{ weight: font::Weight::Bold, ..Font::DEFAULT }})"
        )
    } else {
        format!("text(\"{label}\")")
    }
}

/// What a column's header reads: its own text, or the field name it shows.
fn data_table_column_label(column: &DataTableColumn) -> String {
    if column.header.trim().is_empty() {
        column.field.clone()
    } else {
        column.header.trim().to_string()
    }
}

/// One cell: the field, rendered.
///
/// A custom enum or struct has no `Display` for `text` to borrow, so it goes
/// through `to_string` — the same rule `generate_table` follows.
fn data_table_cell_expr(
    struct_def: &crate::enum_builder::StructDef,
    field_name: &str,
) -> String {
    use crate::enum_builder::FieldType;

    let field_type = struct_def
        .fields
        .iter()
        .find(|field| field.name == field_name)
        .map(|field| field.field_type.clone());

    match field_type {
        Some(FieldType::CustomEnum(_)) | Some(FieldType::CustomStruct(_)) => {
            format!("text(row.{field_name}.to_string()).into()")
        }
        Some(FieldType::String) => format!("text(&row.{field_name}).into()"),
        Some(_) => format!("text(row.{field_name}.to_string()).into()"),
        // The struct lost the field. Better an empty cell than a reference to
        // something that is not there.
        None => "text(\"\").into()".to_string(),
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

/// A `Padding` literal for generated code.
///
/// The menu takes padding for its rows and its trigger separately from the
/// panel's own, and neither goes through `CodeBuilder::add_padding`, which
/// emits a `.padding(..)` call rather than a bare value.
fn format_padding(padding: &Padding) -> String {
    if padding.top == padding.bottom && padding.left == padding.right {
        // No `.into()`: both call sites take `impl Into<Padding>`, and calling
        // `into()` on something that is already the argument leaves the target
        // type unfixed — `[f32; 2]` converts into more than one thing.
        format!("[{:.1}, {:.1}]", padding.top, padding.left)
    } else {
        format!(
            "iced::Padding {{ top: {:.1}, right: {:.1}, bottom: {:.1}, left: {:.1} }}",
            padding.top, padding.right, padding.bottom, padding.left
        )
    }
}

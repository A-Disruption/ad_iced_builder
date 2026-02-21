use iced::Color;
use super::helpers::{format_color_with_source, format_radius, format_shadow};
use crate::styles::style_enum::RuleFillMode;

/// Generate button style function code as a String (tree_sitter handles highlighting)
pub fn generate_button_style_code(
    style_name: &str,
    text_color: Color,
    text_color_source: &Option<String>,
    background_color: Color,
    background_color_source: &Option<String>,
    border_color: Color,
    border_color_source: &Option<String>,
    border_width: f32,
    border_radius_top_left: f32,
    border_radius_top_right: f32,
    border_radius_bottom_right: f32,
    border_radius_bottom_left: f32,
    shadow_enabled: bool,
    shadow_color: Color,
    shadow_color_source: &Option<String>,
    shadow_offset_x: f32,
    shadow_offset_y: f32,
    shadow_blur_radius: f32,
    snap: bool,
) -> String {
    let mut code = String::new();

    code.push_str(&format!("pub fn {}(theme: &Theme, status: Status) -> Style {{\n", style_name));

    code.push_str("    let palette = theme.extended_palette();\n\n");

    code.push_str("    let base = Style {\n");

    code.push_str("        text_color: ");
    code.push_str(&format_color_with_source(text_color, text_color_source));
    code.push_str(",\n");

    code.push_str("        background: Some(Background::Color(");
    code.push_str(&format_color_with_source(background_color, background_color_source));
    code.push_str(")),\n");

    code.push_str("        border: Border {\n");
    code.push_str("            color: ");
    code.push_str(&format_color_with_source(border_color, border_color_source));
    code.push_str(",\n");
    code.push_str(&format!("            width: {:.1},\n", border_width));
    code.push_str("            radius: ");
    code.push_str(&format_radius(
        border_radius_top_left,
        border_radius_top_right,
        border_radius_bottom_right,
        border_radius_bottom_left,
        "            ",
    ));
    code.push_str(",\n");
    code.push_str("        },\n");

    code.push_str("        shadow: ");
    code.push_str(&format_shadow(
        shadow_enabled,
        shadow_color,
        shadow_color_source,
        shadow_offset_x,
        shadow_offset_y,
        shadow_blur_radius,
        "        ",
    ));
    code.push_str(",\n");

    code.push_str(&format!("        snap: {},\n", snap));

    code.push_str("    };\n\n");

    code.push_str("    match status {\n");
    code.push_str("        Status::Active | Status::Pressed => base,\n");
    code.push_str("        Status::Hovered => Style {\n");
    code.push_str("            text_color: base.text_color.scale_alpha(0.8),\n");
    code.push_str("            ..base\n");
    code.push_str("        },\n");
    code.push_str("        Status::Disabled => Style {\n");
    code.push_str("            background: base.background.map(|bg| bg.scale_alpha(0.5)),\n");
    code.push_str("            text_color: base.text_color.scale_alpha(0.5),\n");
    code.push_str("            ..base\n");
    code.push_str("        },\n");
    code.push_str("    }\n");
    code.push_str("}");

    code
}

pub fn generate_container_style_code(
    style_name: &str,
    text_color: Color,
    text_color_source: &Option<String>,
    background_color: Color,
    background_color_source: &Option<String>,
    border_color: Color,
    border_color_source: &Option<String>,
    border_width: f32,
    border_radius_top_left: f32,
    border_radius_top_right: f32,
    border_radius_bottom_right: f32,
    border_radius_bottom_left: f32,
    shadow_enabled: bool,
    shadow_color: Color,
    shadow_color_source: &Option<String>,
    shadow_offset_x: f32,
    shadow_offset_y: f32,
    shadow_blur_radius: f32,
    snap: bool,
) -> String {
    let mut code = String::new();

    code.push_str(&format!("pub fn {}(theme: &Theme) -> Style {{\n", style_name));

    code.push_str("    let palette = theme.extended_palette();\n\n");

    code.push_str("    Style {\n");

    code.push_str("        text_color: Some(");
    code.push_str(&format_color_with_source(text_color, text_color_source));
    code.push_str("),\n");

    code.push_str("        background: Some(Background::Color(");
    code.push_str(&format_color_with_source(background_color, background_color_source));
    code.push_str(")),\n");

    code.push_str("        border: Border {\n");
    code.push_str("            color: ");
    code.push_str(&format_color_with_source(border_color, border_color_source));
    code.push_str(",\n");
    code.push_str(&format!("            width: {:.1},\n", border_width));
    code.push_str("            radius: ");
    code.push_str(&format_radius(
        border_radius_top_left,
        border_radius_top_right,
        border_radius_bottom_right,
        border_radius_bottom_left,
        "            ",
    ));
    code.push_str(",\n");
    code.push_str("        },\n");

    code.push_str("        shadow: ");
    code.push_str(&format_shadow(
        shadow_enabled,
        shadow_color,
        shadow_color_source,
        shadow_offset_x,
        shadow_offset_y,
        shadow_blur_radius,
        "        ",
    ));
    code.push_str(",\n");

    code.push_str(&format!("        snap: {},\n", snap));

    code.push_str("    }\n");
    code.push_str("}");

    code
}

pub fn generate_checkbox_style_code(
    style_name: &str,
    text_color: Color,
    text_color_source: &Option<String>,
    background_color: Color,
    background_color_source: &Option<String>,
    icon_color: Color,
    icon_color_source: &Option<String>,
    border_color: Color,
    border_color_source: &Option<String>,
    border_width: f32,
    border_radius_top_left: f32,
    border_radius_top_right: f32,
    border_radius_bottom_right: f32,
    border_radius_bottom_left: f32,
) -> String {
    let mut code = String::new();

    code.push_str(&format!("pub fn {}(theme: &Theme, status: Status) -> Style {{\n", style_name));

    code.push_str("    let palette = theme.extended_palette();\n\n");

    code.push_str("    let base = Style {\n");

    code.push_str("        background: Background::Color(");
    code.push_str(&format_color_with_source(background_color, background_color_source));
    code.push_str("),\n");

    code.push_str("        icon_color: ");
    code.push_str(&format_color_with_source(icon_color, icon_color_source));
    code.push_str(",\n");

    code.push_str("        border: Border {\n");
    code.push_str("            color: ");
    code.push_str(&format_color_with_source(border_color, border_color_source));
    code.push_str(",\n");
    code.push_str(&format!("            width: {:.1},\n", border_width));
    code.push_str("            radius: ");
    code.push_str(&format_radius(
        border_radius_top_left,
        border_radius_top_right,
        border_radius_bottom_right,
        border_radius_bottom_left,
        "            ",
    ));
    code.push_str(",\n");
    code.push_str("        },\n");

    code.push_str("        text_color: Some(");
    code.push_str(&format_color_with_source(text_color, text_color_source));
    code.push_str("),\n");

    code.push_str("    };\n\n");

    code.push_str("    match status {\n");
    code.push_str("        Status::Active { .. } => base,\n");
    code.push_str("        Status::Hovered { .. } => Style {\n");
    code.push_str("            text_color: base.text_color.map(|c| c.scale_alpha(0.8)),\n");
    code.push_str("            ..base\n");
    code.push_str("        },\n");
    code.push_str("        Status::Disabled { .. } => Style {\n");
    code.push_str("            icon_color: base.icon_color.scale_alpha(0.5),\n");
    code.push_str("            text_color: base.text_color.map(|c| c.scale_alpha(0.5)),\n");
    code.push_str("            ..base\n");
    code.push_str("        },\n");
    code.push_str("    }\n");
    code.push_str("}");

    code
}

pub fn generate_rule_style_code(
    style_name: &str,
    border_color: Color,
    border_color_source: &Option<String>,
    border_radius_top_left: f32,
    border_radius_top_right: f32,
    border_radius_bottom_right: f32,
    border_radius_bottom_left: f32,
    fill_mode: &RuleFillMode,
    snap: bool,
) -> String {
    let mut code = String::new();

    code.push_str(&format!("pub fn {}(theme: &Theme) -> Style {{\n", style_name));
    code.push_str("    let palette = theme.extended_palette();\n\n");
    code.push_str("    Style {\n");

    code.push_str("        color: ");
    code.push_str(&format_color_with_source(border_color, border_color_source));
    code.push_str(",\n");

    code.push_str("        radius: ");
    code.push_str(&format_radius(
        border_radius_top_left,
        border_radius_top_right,
        border_radius_bottom_right,
        border_radius_bottom_left,
        "        ",
    ));
    code.push_str(",\n");

    let fill_mode_str = match fill_mode {
        RuleFillMode::Full => "FillMode::Full".to_string(),
        RuleFillMode::Percent(p) => format!("FillMode::Percent({:.1})", p),
        RuleFillMode::Padded(p) => format!("FillMode::Padded({})", p),
        RuleFillMode::AsymmetricPadding(a, b) => format!("FillMode::AsymmetricPadding({}, {})", a, b),
    };
    code.push_str(&format!("        fill_mode: {},\n", fill_mode_str));
    code.push_str(&format!("        snap: {},\n", snap));

    code.push_str("    }\n");
    code.push_str("}");

    code
}

pub fn generate_combo_box_style_code(
    style_name: &str,
    // text_input style fields
    background_color: Color,
    background_color_source: &Option<String>,
    border_color: Color,
    border_color_source: &Option<String>,
    border_width: f32,
    border_radius_top_left: f32,
    border_radius_top_right: f32,
    border_radius_bottom_right: f32,
    border_radius_bottom_left: f32,
    icon_color: Color,
    icon_color_source: &Option<String>,
    placeholder_color: Color,
    placeholder_color_source: &Option<String>,
    text_color: Color,
    text_color_source: &Option<String>,
    selection_color: Color,
    selection_color_source: &Option<String>,
    // menu style fields
    selected_text_color: Color,
    selected_text_color_source: &Option<String>,
    selected_background_color: Color,
    selected_background_color_source: &Option<String>,
    shadow_enabled: bool,
    shadow_color: Color,
    shadow_color_source: &Option<String>,
    shadow_offset_x: f32,
    shadow_offset_y: f32,
    shadow_blur_radius: f32,
) -> String {
    let mut code = String::new();

    code.push_str(&format!("pub fn {}_input_style(theme: &Theme, status: text_input::Status) -> text_input::Style {{\n", style_name));
    code.push_str("    let palette = theme.extended_palette();\n\n");

    code.push_str("    let base = text_input::Style {\n");

    code.push_str("        background: Background::Color(");
    code.push_str(&format_color_with_source(background_color, background_color_source));
    code.push_str("),\n");

    code.push_str("        border: Border {\n");
    code.push_str("            color: ");
    code.push_str(&format_color_with_source(border_color, border_color_source));
    code.push_str(",\n");
    code.push_str(&format!("            width: {:.1},\n", border_width));
    code.push_str("            radius: ");
    code.push_str(&format_radius(
        border_radius_top_left,
        border_radius_top_right,
        border_radius_bottom_right,
        border_radius_bottom_left,
        "            ",
    ));
    code.push_str(",\n");
    code.push_str("        },\n");

    code.push_str("        icon: ");
    code.push_str(&format_color_with_source(icon_color, icon_color_source));
    code.push_str(",\n");

    code.push_str("        placeholder: ");
    code.push_str(&format_color_with_source(placeholder_color, placeholder_color_source));
    code.push_str(",\n");

    code.push_str("        value: ");
    code.push_str(&format_color_with_source(text_color, text_color_source));
    code.push_str(",\n");

    code.push_str("        selection: ");
    code.push_str(&format_color_with_source(selection_color, selection_color_source));
    code.push_str(",\n");

    code.push_str("    };\n\n");

    code.push_str("    match status {\n");
    code.push_str("        text_input::Status::Active => base,\n");
    code.push_str("        text_input::Status::Hovered => text_input::Style {\n");
    code.push_str("            border: Border {\n");
    code.push_str("                color: base.border.color.scale_alpha(0.8),\n");
    code.push_str("                ..base.border\n");
    code.push_str("            },\n");
    code.push_str("            ..base\n");
    code.push_str("        },\n");
    code.push_str("        text_input::Status::Focused { .. } => text_input::Style {\n");
    code.push_str("            border: Border {\n");
    code.push_str("                color: base.border.color,\n");
    code.push_str("                width: base.border.width.max(1.0),\n");
    code.push_str("                ..base.border\n");
    code.push_str("            },\n");
    code.push_str("            ..base\n");
    code.push_str("        },\n");
    code.push_str("        text_input::Status::Disabled => text_input::Style {\n");
    code.push_str("            background: base.background.scale_alpha(0.5),\n");
    code.push_str("            value: base.value.scale_alpha(0.5),\n");
    code.push_str("            placeholder: base.placeholder.scale_alpha(0.5),\n");
    code.push_str("            ..base\n");
    code.push_str("        },\n");
    code.push_str("    }\n");
    code.push_str("}\n\n");

    code.push_str(&format!("pub fn {}_menu_style(theme: &Theme) -> menu::Style {{\n", style_name));
    code.push_str("    let palette = theme.extended_palette();\n\n");
    code.push_str("    menu::Style {\n");

    code.push_str("        background: Background::Color(");
    code.push_str(&format_color_with_source(background_color, background_color_source));
    code.push_str("),\n");

    code.push_str("        border: Border {\n");
    code.push_str("            color: ");
    code.push_str(&format_color_with_source(border_color, border_color_source));
    code.push_str(",\n");
    code.push_str(&format!("            width: {:.1},\n", border_width));
    code.push_str("            radius: ");
    code.push_str(&format_radius(
        border_radius_top_left,
        border_radius_top_right,
        border_radius_bottom_right,
        border_radius_bottom_left,
        "            ",
    ));
    code.push_str(",\n");
    code.push_str("        },\n");

    code.push_str("        text_color: ");
    code.push_str(&format_color_with_source(text_color, text_color_source));
    code.push_str(",\n");

    code.push_str("        selected_text_color: ");
    code.push_str(&format_color_with_source(selected_text_color, selected_text_color_source));
    code.push_str(",\n");

    code.push_str("        selected_background: Background::Color(");
    code.push_str(&format_color_with_source(selected_background_color, selected_background_color_source));
    code.push_str("),\n");

    code.push_str("        shadow: ");
    code.push_str(&format_shadow(
        shadow_enabled,
        shadow_color,
        shadow_color_source,
        shadow_offset_x,
        shadow_offset_y,
        shadow_blur_radius,
        "        ",
    ));
    code.push_str(",\n");

    code.push_str("    }\n");
    code.push_str("}");

    code
}

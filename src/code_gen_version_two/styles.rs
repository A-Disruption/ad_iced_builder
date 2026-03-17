use super::helpers::{format_color_with_source, format_radius, format_shadow};
use crate::styles::style_enum::{RuleFillMode, StatusColorOverride};
use crate::views::theme_and_stylefn_builder::{CustomThemes, ThemePaneEnum};
use iced::Color;

/// Emit a single override field line like `text_color: Color { r:…, g:…, b:…, a:… },\n`
/// Returns empty string if the field has no override.
fn emit_ov_color(
    indent: &str,
    field: &str,
    val: Option<Color>,
    src: &Option<String>,
    base: Color,
) -> String {
    if src.is_some() || val.is_some() {
        let resolved = val.unwrap_or(base);
        format!(
            "{}{}: {},\n",
            indent,
            field,
            format_color_with_source(resolved, src)
        )
    } else {
        String::new()
    }
}

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
    status_hovered: Option<&StatusColorOverride>,
    status_pressed: Option<&StatusColorOverride>,
    status_disabled: Option<&StatusColorOverride>,
) -> String {
    let mut code = String::new();

    code.push_str(&format!(
        "pub fn {}(theme: &Theme, status: Status) -> Style {{\n",
        style_name
    ));

    code.push_str("    let palette = theme.extended_palette();\n\n");

    code.push_str("    let base = Style {\n");

    code.push_str("        text_color: ");
    code.push_str(&format_color_with_source(text_color, text_color_source));
    code.push_str(",\n");

    code.push_str("        background: Some(Background::Color(");
    code.push_str(&format_color_with_source(
        background_color,
        background_color_source,
    ));
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
    // Active arm
    code.push_str("        Status::Active => base,\n");
    // Pressed arm
    code.push_str("        Status::Pressed => ");
    if let Some(ov) = status_pressed {
        let tc = emit_ov_color(
            "            ",
            "text_color",
            ov.text_color,
            &ov.text_color_source,
            text_color,
        );
        let bg = emit_ov_color(
            "                ",
            "color",
            ov.background_color,
            &ov.background_color_source,
            background_color,
        );
        let bc = emit_ov_color(
            "                ",
            "color",
            ov.border_color,
            &ov.border_color_source,
            border_color,
        );
        if tc.is_empty() && bg.is_empty() && bc.is_empty() {
            code.push_str("base,\n");
        } else {
            code.push_str("Style {\n");
            if !tc.is_empty() {
                code.push_str(&tc);
            }
            if !bg.is_empty() {
                code.push_str("            background: Some(Background::Color(");
                let resolved = ov.background_color.unwrap_or(background_color);
                code.push_str(&format_color_with_source(
                    resolved,
                    &ov.background_color_source,
                ));
                code.push_str(")),\n");
            }
            if !bc.is_empty() {
                code.push_str("            border: Border { color: ");
                let resolved = ov.border_color.unwrap_or(border_color);
                code.push_str(&format_color_with_source(resolved, &ov.border_color_source));
                code.push_str(", ..base.border },\n");
            }
            code.push_str("            ..base\n        },\n");
        }
    } else {
        code.push_str("base,\n");
    }
    // Hovered arm
    code.push_str("        Status::Hovered => ");
    if let Some(ov) = status_hovered {
        let tc = ov.text_color.is_some() || ov.text_color_source.is_some();
        let bg = ov.background_color.is_some() || ov.background_color_source.is_some();
        let bc = ov.border_color.is_some() || ov.border_color_source.is_some();
        if !tc && !bg && !bc {
            code.push_str("Style { text_color: base.text_color.scale_alpha(0.8), ..base },\n");
        } else {
            code.push_str("Style {\n");
            if tc {
                code.push_str(&emit_ov_color(
                    "            ",
                    "text_color",
                    ov.text_color,
                    &ov.text_color_source,
                    text_color,
                ));
            }
            if bg {
                let resolved = ov.background_color.unwrap_or(background_color);
                code.push_str(&format!(
                    "            background: Some(Background::Color({})),\n",
                    format_color_with_source(resolved, &ov.background_color_source)
                ));
            }
            if bc {
                let resolved = ov.border_color.unwrap_or(border_color);
                code.push_str(&format!(
                    "            border: Border {{ color: {}, ..base.border }},\n",
                    format_color_with_source(resolved, &ov.border_color_source)
                ));
            }
            code.push_str("            ..base\n        },\n");
        }
    } else {
        code.push_str("Style { text_color: base.text_color.scale_alpha(0.8), ..base },\n");
    }
    // Disabled arm
    code.push_str("        Status::Disabled => ");
    if let Some(ov) = status_disabled {
        let tc = ov.text_color.is_some() || ov.text_color_source.is_some();
        let bg = ov.background_color.is_some() || ov.background_color_source.is_some();
        let bc = ov.border_color.is_some() || ov.border_color_source.is_some();
        if !tc && !bg && !bc {
            code.push_str("Style { background: base.background.map(|bg| bg.scale_alpha(0.5)), text_color: base.text_color.scale_alpha(0.5), ..base },\n");
        } else {
            code.push_str("Style {\n");
            if tc {
                code.push_str(&emit_ov_color(
                    "            ",
                    "text_color",
                    ov.text_color,
                    &ov.text_color_source,
                    text_color,
                ));
            }
            if bg {
                let resolved = ov.background_color.unwrap_or(background_color);
                code.push_str(&format!(
                    "            background: Some(Background::Color({})),\n",
                    format_color_with_source(resolved, &ov.background_color_source)
                ));
            }
            if bc {
                let resolved = ov.border_color.unwrap_or(border_color);
                code.push_str(&format!(
                    "            border: Border {{ color: {}, ..base.border }},\n",
                    format_color_with_source(resolved, &ov.border_color_source)
                ));
            }
            code.push_str("            ..base\n        },\n");
        }
    } else {
        code.push_str("Style { background: base.background.map(|bg| bg.scale_alpha(0.5)), text_color: base.text_color.scale_alpha(0.5), ..base },\n");
    }
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

    code.push_str(&format!(
        "pub fn {}(theme: &Theme) -> Style {{\n",
        style_name
    ));

    code.push_str("    let palette = theme.extended_palette();\n\n");

    code.push_str("    Style {\n");

    code.push_str("        text_color: Some(");
    code.push_str(&format_color_with_source(text_color, text_color_source));
    code.push_str("),\n");

    code.push_str("        background: Some(Background::Color(");
    code.push_str(&format_color_with_source(
        background_color,
        background_color_source,
    ));
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
    status_hovered: Option<&StatusColorOverride>,
    status_disabled: Option<&StatusColorOverride>,
) -> String {
    let mut code = String::new();

    code.push_str(&format!(
        "pub fn {}(theme: &Theme, status: Status) -> Style {{\n",
        style_name
    ));

    code.push_str("    let palette = theme.extended_palette();\n\n");

    code.push_str("    let base = Style {\n");

    code.push_str("        background: Background::Color(");
    code.push_str(&format_color_with_source(
        background_color,
        background_color_source,
    ));
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
    // Hovered
    code.push_str("        Status::Hovered { .. } => ");
    if let Some(ov) = status_hovered {
        let tc = ov.text_color.is_some() || ov.text_color_source.is_some();
        let ic = ov.icon_color.is_some() || ov.icon_color_source.is_some();
        if !tc && !ic {
            code.push_str(
                "Style { text_color: base.text_color.map(|c| c.scale_alpha(0.8)), ..base },\n",
            );
        } else {
            code.push_str("Style {\n");
            if tc {
                code.push_str(&format!(
                    "            text_color: Some({}),\n",
                    format_color_with_source(
                        ov.text_color.unwrap_or(text_color),
                        &ov.text_color_source
                    )
                ));
            }
            if ic {
                code.push_str(&format!(
                    "            icon_color: {},\n",
                    format_color_with_source(
                        ov.icon_color.unwrap_or(icon_color),
                        &ov.icon_color_source
                    )
                ));
            }
            code.push_str("            ..base\n        },\n");
        }
    } else {
        code.push_str(
            "Style { text_color: base.text_color.map(|c| c.scale_alpha(0.8)), ..base },\n",
        );
    }
    // Disabled
    code.push_str("        Status::Disabled { .. } => ");
    if let Some(ov) = status_disabled {
        let tc = ov.text_color.is_some() || ov.text_color_source.is_some();
        let ic = ov.icon_color.is_some() || ov.icon_color_source.is_some();
        if !tc && !ic {
            code.push_str("Style { icon_color: base.icon_color.scale_alpha(0.5), text_color: base.text_color.map(|c| c.scale_alpha(0.5)), ..base },\n");
        } else {
            code.push_str("Style {\n");
            if tc {
                code.push_str(&format!(
                    "            text_color: Some({}),\n",
                    format_color_with_source(
                        ov.text_color.unwrap_or(text_color),
                        &ov.text_color_source
                    )
                ));
            }
            if ic {
                code.push_str(&format!(
                    "            icon_color: {},\n",
                    format_color_with_source(
                        ov.icon_color.unwrap_or(icon_color),
                        &ov.icon_color_source
                    )
                ));
            }
            code.push_str("            ..base\n        },\n");
        }
    } else {
        code.push_str("Style { icon_color: base.icon_color.scale_alpha(0.5), text_color: base.text_color.map(|c| c.scale_alpha(0.5)), ..base },\n");
    }
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

    code.push_str(&format!(
        "pub fn {}(theme: &Theme) -> Style {{\n",
        style_name
    ));
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
        RuleFillMode::AsymmetricPadding(a, b) => {
            format!("FillMode::AsymmetricPadding({}, {})", a, b)
        }
    };
    code.push_str(&format!("        fill_mode: {},\n", fill_mode_str));
    code.push_str(&format!("        snap: {},\n", snap));

    code.push_str("    }\n");
    code.push_str("}");

    code
}

pub fn generate_text_input_style_code(
    function_name: &str,
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
    status_hovered: Option<&StatusColorOverride>,
    status_focused: Option<&StatusColorOverride>,
    status_disabled: Option<&StatusColorOverride>,
) -> String {
    let mut code = String::new();

    code.push_str(&format!(
        "pub fn {}(theme: &Theme, status: text_input::Status) -> text_input::Style {{\n",
        function_name
    ));
    code.push_str("    let palette = theme.extended_palette();\n\n");
    code.push_str("    let base = text_input::Style {\n");
    code.push_str("        background: Background::Color(");
    code.push_str(&format_color_with_source(
        background_color,
        background_color_source,
    ));
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
    code.push_str(&format_color_with_source(
        placeholder_color,
        placeholder_color_source,
    ));
    code.push_str(",\n");
    code.push_str("        value: ");
    code.push_str(&format_color_with_source(text_color, text_color_source));
    code.push_str(",\n");
    code.push_str("        selection: ");
    code.push_str(&format_color_with_source(
        selection_color,
        selection_color_source,
    ));
    code.push_str(",\n");
    code.push_str("    };\n\n");
    code.push_str("    match status {\n");
    code.push_str("        text_input::Status::Active => base,\n");

    code.push_str("        text_input::Status::Hovered => ");
    if let Some(ov) = status_hovered {
        let has_text = ov.text_color.is_some() || ov.text_color_source.is_some();
        let has_icon = ov.icon_color.is_some() || ov.icon_color_source.is_some();
        let has_background = ov.background_color.is_some() || ov.background_color_source.is_some();
        let has_border = ov.border_color.is_some() || ov.border_color_source.is_some();
        if has_text || has_icon || has_background || has_border {
            code.push_str("text_input::Style {\n");
            if has_text {
                code.push_str(&format!(
                    "            value: {},\n",
                    format_color_with_source(
                        ov.text_color.unwrap_or(text_color),
                        &ov.text_color_source
                    )
                ));
            }
            if has_icon {
                code.push_str(&format!(
                    "            icon: {},\n",
                    format_color_with_source(
                        ov.icon_color.unwrap_or(icon_color),
                        &ov.icon_color_source
                    )
                ));
            }
            if has_background {
                code.push_str(&format!(
                    "            background: Background::Color({}),\n",
                    format_color_with_source(
                        ov.background_color.unwrap_or(background_color),
                        &ov.background_color_source
                    )
                ));
            }
            if has_border {
                code.push_str(&format!(
                    "            border: Border {{ color: {}, ..base.border }},\n",
                    format_color_with_source(
                        ov.border_color.unwrap_or(border_color),
                        &ov.border_color_source
                    )
                ));
            }
            code.push_str("            ..base\n        },\n");
        } else {
            code.push_str(
                "text_input::Style { border: Border { color: base.border.color.scale_alpha(0.8), ..base.border }, ..base },\n",
            );
        }
    } else {
        code.push_str(
            "text_input::Style { border: Border { color: base.border.color.scale_alpha(0.8), ..base.border }, ..base },\n",
        );
    }

    code.push_str("        text_input::Status::Focused { .. } => ");
    if let Some(ov) = status_focused {
        let has_text = ov.text_color.is_some() || ov.text_color_source.is_some();
        let has_icon = ov.icon_color.is_some() || ov.icon_color_source.is_some();
        let has_background = ov.background_color.is_some() || ov.background_color_source.is_some();
        let has_border = ov.border_color.is_some() || ov.border_color_source.is_some();
        if has_text || has_icon || has_background || has_border {
            code.push_str("text_input::Style {\n");
            if has_text {
                code.push_str(&format!(
                    "            value: {},\n",
                    format_color_with_source(
                        ov.text_color.unwrap_or(text_color),
                        &ov.text_color_source
                    )
                ));
            }
            if has_icon {
                code.push_str(&format!(
                    "            icon: {},\n",
                    format_color_with_source(
                        ov.icon_color.unwrap_or(icon_color),
                        &ov.icon_color_source
                    )
                ));
            }
            if has_background {
                code.push_str(&format!(
                    "            background: Background::Color({}),\n",
                    format_color_with_source(
                        ov.background_color.unwrap_or(background_color),
                        &ov.background_color_source
                    )
                ));
            }
            if has_border {
                code.push_str(&format!(
                    "            border: Border {{ color: {}, width: base.border.width.max(1.0), ..base.border }},\n",
                    format_color_with_source(
                        ov.border_color.unwrap_or(border_color),
                        &ov.border_color_source
                    )
                ));
            }
            code.push_str("            ..base\n        },\n");
        } else {
            code.push_str(
                "text_input::Style { border: Border { color: base.border.color, width: base.border.width.max(1.0), ..base.border }, ..base },\n",
            );
        }
    } else {
        code.push_str(
            "text_input::Style { border: Border { color: base.border.color, width: base.border.width.max(1.0), ..base.border }, ..base },\n",
        );
    }

    code.push_str("        text_input::Status::Disabled => ");
    if let Some(ov) = status_disabled {
        code.push_str("text_input::Style {\n");
        code.push_str(&format!(
            "            background: Background::Color({}),\n",
            format_color_with_source(
                ov.background_color.unwrap_or(background_color),
                &ov.background_color_source
            )
        ));
        code.push_str(&format!(
            "            value: {},\n",
            format_color_with_source(
                ov.text_color.unwrap_or(placeholder_color),
                &ov.text_color_source
            )
        ));
        code.push_str(&format!(
            "            icon: {},\n",
            format_color_with_source(ov.icon_color.unwrap_or(icon_color), &ov.icon_color_source)
        ));
        code.push_str(&format!(
            "            border: Border {{ color: {}, ..base.border }},\n",
            format_color_with_source(
                ov.border_color.unwrap_or(border_color),
                &ov.border_color_source
            )
        ));
        code.push_str("            placeholder: base.placeholder.scale_alpha(0.5),\n");
        code.push_str("            ..base\n        },\n");
    } else {
        code.push_str(
            "text_input::Style { background: base.background.scale_alpha(0.5), icon: base.icon.scale_alpha(0.5), value: base.placeholder, placeholder: base.placeholder.scale_alpha(0.5), ..base },\n",
        );
    }

    code.push_str("    }\n");
    code.push_str("}");
    code
}

pub fn generate_menu_style_code(
    function_name: &str,
    background_color: Color,
    background_color_source: &Option<String>,
    border_color: Color,
    border_color_source: &Option<String>,
    border_width: f32,
    border_radius_top_left: f32,
    border_radius_top_right: f32,
    border_radius_bottom_right: f32,
    border_radius_bottom_left: f32,
    text_color: Color,
    text_color_source: &Option<String>,
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

    code.push_str(&format!(
        "pub fn {}(theme: &Theme) -> menu::Style {{\n",
        function_name
    ));
    code.push_str("    let palette = theme.extended_palette();\n\n");
    code.push_str("    menu::Style {\n");
    code.push_str("        background: Background::Color(");
    code.push_str(&format_color_with_source(
        background_color,
        background_color_source,
    ));
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
    code.push_str(&format_color_with_source(
        selected_text_color,
        selected_text_color_source,
    ));
    code.push_str(",\n");
    code.push_str("        selected_background: Background::Color(");
    code.push_str(&format_color_with_source(
        selected_background_color,
        selected_background_color_source,
    ));
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

pub fn generate_pick_list_style_code(
    function_name: &str,
    text_color: Color,
    text_color_source: &Option<String>,
    background_color: Color,
    background_color_source: &Option<String>,
    placeholder_color: Color,
    placeholder_color_source: &Option<String>,
    handle_color: Color,
    handle_color_source: &Option<String>,
    border_color: Color,
    border_color_source: &Option<String>,
    border_width: f32,
    border_radius_top_left: f32,
    border_radius_top_right: f32,
    border_radius_bottom_right: f32,
    border_radius_bottom_left: f32,
    status_hovered: Option<&StatusColorOverride>,
    status_opened: Option<&StatusColorOverride>,
) -> String {
    let mut code = String::new();

    code.push_str(&format!(
        "pub fn {}(theme: &Theme, status: pick_list::Status) -> pick_list::Style {{\n",
        function_name
    ));
    code.push_str("    let palette = theme.extended_palette();\n\n");
    code.push_str("    let base = pick_list::Style {\n");
    code.push_str("        text_color: ");
    code.push_str(&format_color_with_source(text_color, text_color_source));
    code.push_str(",\n");
    code.push_str("        placeholder_color: ");
    code.push_str(&format_color_with_source(
        placeholder_color,
        placeholder_color_source,
    ));
    code.push_str(",\n");
    code.push_str("        handle_color: ");
    code.push_str(&format_color_with_source(handle_color, handle_color_source));
    code.push_str(",\n");
    code.push_str("        background: Background::Color(");
    code.push_str(&format_color_with_source(
        background_color,
        background_color_source,
    ));
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
    code.push_str("    };\n\n");
    code.push_str("    match status {\n");
    code.push_str("        pick_list::Status::Active => base,\n");
    code.push_str("        pick_list::Status::Hovered => ");
    if let Some(ov) = status_hovered {
        let has_text = ov.text_color.is_some() || ov.text_color_source.is_some();
        let has_handle = ov.icon_color.is_some() || ov.icon_color_source.is_some();
        let has_background = ov.background_color.is_some() || ov.background_color_source.is_some();
        let has_border = ov.border_color.is_some() || ov.border_color_source.is_some();
        if has_text || has_handle || has_background || has_border {
            code.push_str("pick_list::Style {\n");
            if has_text {
                code.push_str(&format!(
                    "            text_color: {},\n",
                    format_color_with_source(
                        ov.text_color.unwrap_or(text_color),
                        &ov.text_color_source
                    )
                ));
            }
            if has_handle {
                code.push_str(&format!(
                    "            handle_color: {},\n",
                    format_color_with_source(
                        ov.icon_color.unwrap_or(handle_color),
                        &ov.icon_color_source
                    )
                ));
            }
            if has_background {
                code.push_str(&format!(
                    "            background: Background::Color({}),\n",
                    format_color_with_source(
                        ov.background_color.unwrap_or(background_color),
                        &ov.background_color_source
                    )
                ));
            }
            if has_border {
                code.push_str(&format!(
                    "            border: Border {{ color: {}, ..base.border }},\n",
                    format_color_with_source(
                        ov.border_color.unwrap_or(border_color),
                        &ov.border_color_source
                    )
                ));
            }
            code.push_str("            ..base\n        },\n");
        } else {
            code.push_str(
                "pick_list::Style { border: Border { color: base.border.color.scale_alpha(0.8), ..base.border }, ..base },\n",
            );
        }
    } else {
        code.push_str(
            "pick_list::Style { border: Border { color: base.border.color.scale_alpha(0.8), ..base.border }, ..base },\n",
        );
    }

    code.push_str("        pick_list::Status::Opened { .. } => ");
    if let Some(ov) = status_opened {
        let has_text = ov.text_color.is_some() || ov.text_color_source.is_some();
        let has_handle = ov.icon_color.is_some() || ov.icon_color_source.is_some();
        let has_background = ov.background_color.is_some() || ov.background_color_source.is_some();
        let has_border = ov.border_color.is_some() || ov.border_color_source.is_some();
        if has_text || has_handle || has_background || has_border {
            code.push_str("pick_list::Style {\n");
            if has_text {
                code.push_str(&format!(
                    "            text_color: {},\n",
                    format_color_with_source(
                        ov.text_color.unwrap_or(text_color),
                        &ov.text_color_source
                    )
                ));
            }
            if has_handle {
                code.push_str(&format!(
                    "            handle_color: {},\n",
                    format_color_with_source(
                        ov.icon_color.unwrap_or(handle_color),
                        &ov.icon_color_source
                    )
                ));
            }
            if has_background {
                code.push_str(&format!(
                    "            background: Background::Color({}),\n",
                    format_color_with_source(
                        ov.background_color.unwrap_or(background_color),
                        &ov.background_color_source
                    )
                ));
            }
            if has_border {
                code.push_str(&format!(
                    "            border: Border {{ color: {}, width: base.border.width.max(1.0), ..base.border }},\n",
                    format_color_with_source(
                        ov.border_color.unwrap_or(border_color),
                        &ov.border_color_source
                    )
                ));
            }
            code.push_str("            ..base\n        },\n");
        } else {
            code.push_str(
                "pick_list::Style { border: Border { color: base.border.color, width: base.border.width.max(1.0), ..base.border }, ..base },\n",
            );
        }
    } else {
        code.push_str(
            "pick_list::Style { border: Border { color: base.border.color, width: base.border.width.max(1.0), ..base.border }, ..base },\n",
        );
    }
    code.push_str("    }\n");
    code.push_str("}");
    code
}

pub fn generate_slider_style_code(
    function_name: &str,
    active_rail_color: Color,
    active_rail_color_source: &Option<String>,
    inactive_rail_color: Color,
    inactive_rail_color_source: &Option<String>,
    handle_color: Color,
    handle_color_source: &Option<String>,
    border_color: Color,
    border_color_source: &Option<String>,
    border_width: f32,
    border_radius_top_left: f32,
    border_radius_top_right: f32,
    border_radius_bottom_right: f32,
    border_radius_bottom_left: f32,
    status_hovered: Option<&StatusColorOverride>,
    status_dragged: Option<&StatusColorOverride>,
) -> String {
    let handle_radius = border_radius_top_left
        .max(border_radius_top_right)
        .max(border_radius_bottom_right)
        .max(border_radius_bottom_left)
        .max(6.0);
    let mut code = String::new();

    code.push_str(&format!(
        "pub fn {}(theme: &Theme, status: slider::Status) -> slider::Style {{\n",
        function_name
    ));
    code.push_str("    let palette = theme.extended_palette();\n\n");
    code.push_str("    let base = slider::Style {\n");
    code.push_str("        rail: slider::Rail {\n");
    code.push_str("            backgrounds: (\n");
    code.push_str("                Background::Color(");
    code.push_str(&format_color_with_source(
        active_rail_color,
        active_rail_color_source,
    ));
    code.push_str("),\n");
    code.push_str("                Background::Color(");
    code.push_str(&format_color_with_source(
        inactive_rail_color,
        inactive_rail_color_source,
    ));
    code.push_str("),\n");
    code.push_str("            ),\n");
    code.push_str(&format!(
        "            width: {:.1},\n",
        border_width.max(2.0)
    ));
    code.push_str("            border: Border {\n");
    code.push_str("                color: ");
    code.push_str(&format_color_with_source(border_color, border_color_source));
    code.push_str(",\n");
    code.push_str(&format!("                width: {:.1},\n", border_width));
    code.push_str(&format!(
        "                radius: {:.1}.into(),\n",
        handle_radius / 2.0
    ));
    code.push_str("            },\n");
    code.push_str("        },\n");
    code.push_str("        handle: slider::Handle {\n");
    code.push_str(&format!(
        "            shape: slider::HandleShape::Circle {{ radius: {:.1} }},\n",
        handle_radius
    ));
    code.push_str("            background: Background::Color(");
    code.push_str(&format_color_with_source(handle_color, handle_color_source));
    code.push_str("),\n");
    code.push_str(&format!("            border_width: {:.1},\n", border_width));
    code.push_str("            border_color: ");
    code.push_str(&format_color_with_source(border_color, border_color_source));
    code.push_str(",\n");
    code.push_str("        },\n");
    code.push_str("    };\n\n");
    code.push_str("    match status {\n");
    code.push_str("        slider::Status::Active => base,\n");
    code.push_str("        slider::Status::Hovered => ");
    if let Some(ov) = status_hovered {
        let has_override = ov.text_color.is_some()
            || ov.text_color_source.is_some()
            || ov.background_color.is_some()
            || ov.background_color_source.is_some()
            || ov.border_color.is_some()
            || ov.border_color_source.is_some()
            || ov.icon_color.is_some()
            || ov.icon_color_source.is_some();
        if has_override {
            code.push_str("slider::Style {\n");
            code.push_str(&format!(
                "            rail: slider::Rail {{ backgrounds: (Background::Color({}), Background::Color({})), border: Border {{ color: {}, ..base.rail.border }}, ..base.rail }},\n",
                format_color_with_source(
                    ov.text_color.unwrap_or(active_rail_color),
                    &ov.text_color_source
                ),
                format_color_with_source(
                    ov.background_color.unwrap_or(inactive_rail_color),
                    &ov.background_color_source
                ),
                format_color_with_source(
                    ov.border_color.unwrap_or(border_color),
                    &ov.border_color_source
                )
            ));
            code.push_str(&format!(
                "            handle: slider::Handle {{ background: Background::Color({}), border_color: {}, ..base.handle }},\n",
                format_color_with_source(
                    ov.icon_color.unwrap_or(handle_color),
                    &ov.icon_color_source
                ),
                format_color_with_source(
                    ov.border_color.unwrap_or(border_color),
                    &ov.border_color_source
                )
            ));
            code.push_str("        },\n");
        } else {
            code.push_str(
                "slider::Style { handle: slider::Handle { background: base.handle.background.scale_alpha(0.85), ..base.handle }, ..base },\n",
            );
        }
    } else {
        code.push_str(
            "slider::Style { handle: slider::Handle { background: base.handle.background.scale_alpha(0.85), ..base.handle }, ..base },\n",
        );
    }
    code.push_str("        slider::Status::Dragged => ");
    if let Some(ov) = status_dragged {
        let has_override = ov.text_color.is_some()
            || ov.text_color_source.is_some()
            || ov.background_color.is_some()
            || ov.background_color_source.is_some()
            || ov.border_color.is_some()
            || ov.border_color_source.is_some()
            || ov.icon_color.is_some()
            || ov.icon_color_source.is_some();
        if has_override {
            code.push_str("slider::Style {\n");
            code.push_str(&format!(
                "            rail: slider::Rail {{ backgrounds: (Background::Color({}), Background::Color({})), border: Border {{ color: {}, ..base.rail.border }}, ..base.rail }},\n",
                format_color_with_source(
                    ov.text_color.unwrap_or(active_rail_color),
                    &ov.text_color_source
                ),
                format_color_with_source(
                    ov.background_color.unwrap_or(inactive_rail_color),
                    &ov.background_color_source
                ),
                format_color_with_source(
                    ov.border_color.unwrap_or(border_color),
                    &ov.border_color_source
                )
            ));
            code.push_str(&format!(
                "            handle: slider::Handle {{ background: Background::Color({}), border_color: {}, ..base.handle }},\n",
                format_color_with_source(
                    ov.icon_color.unwrap_or(handle_color),
                    &ov.icon_color_source
                ),
                format_color_with_source(
                    ov.border_color.unwrap_or(border_color),
                    &ov.border_color_source
                )
            ));
            code.push_str("        },\n");
        } else {
            code.push_str(
                "slider::Style { rail: slider::Rail { backgrounds: (base.rail.backgrounds.0.scale_alpha(0.9), base.rail.backgrounds.1.scale_alpha(0.9)), ..base.rail }, ..base },\n",
            );
        }
    } else {
        code.push_str(
            "slider::Style { rail: slider::Rail { backgrounds: (base.rail.backgrounds.0.scale_alpha(0.9), base.rail.backgrounds.1.scale_alpha(0.9)), ..base.rail }, ..base },\n",
        );
    }
    code.push_str("    }\n");
    code.push_str("}");
    code
}

pub fn generate_progress_bar_style_code(
    function_name: &str,
    bar_color: Color,
    bar_color_source: &Option<String>,
    background_color: Color,
    background_color_source: &Option<String>,
    border_color: Color,
    border_color_source: &Option<String>,
    border_width: f32,
    border_radius_top_left: f32,
    border_radius_top_right: f32,
    border_radius_bottom_right: f32,
    border_radius_bottom_left: f32,
) -> String {
    let mut code = String::new();

    code.push_str(&format!(
        "pub fn {}(theme: &Theme) -> progress_bar::Style {{\n",
        function_name
    ));
    code.push_str("    let palette = theme.extended_palette();\n\n");
    code.push_str("    progress_bar::Style {\n");
    code.push_str("        background: Background::Color(");
    code.push_str(&format_color_with_source(
        background_color,
        background_color_source,
    ));
    code.push_str("),\n");
    code.push_str("        bar: Background::Color(");
    code.push_str(&format_color_with_source(bar_color, bar_color_source));
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
    code.push_str("    }\n");
    code.push_str("}");
    code
}

pub fn generate_radio_style_code(
    function_name: &str,
    text_color: Color,
    text_color_source: &Option<String>,
    background_color: Color,
    background_color_source: &Option<String>,
    dot_color: Color,
    dot_color_source: &Option<String>,
    border_color: Color,
    border_color_source: &Option<String>,
    border_width: f32,
    status_hovered: Option<&StatusColorOverride>,
) -> String {
    let mut code = String::new();

    code.push_str(&format!(
        "pub fn {}(theme: &Theme, status: radio::Status) -> radio::Style {{\n",
        function_name
    ));
    code.push_str("    let palette = theme.extended_palette();\n\n");
    code.push_str("    let base = radio::Style {\n");
    code.push_str("        background: Background::Color(");
    code.push_str(&format_color_with_source(
        background_color,
        background_color_source,
    ));
    code.push_str("),\n");
    code.push_str("        dot_color: ");
    code.push_str(&format_color_with_source(dot_color, dot_color_source));
    code.push_str(",\n");
    code.push_str(&format!(
        "        border_width: {:.1},\n",
        border_width.max(1.0)
    ));
    code.push_str("        border_color: ");
    code.push_str(&format_color_with_source(border_color, border_color_source));
    code.push_str(",\n");
    code.push_str("        text_color: Some(");
    code.push_str(&format_color_with_source(text_color, text_color_source));
    code.push_str("),\n");
    code.push_str("    };\n\n");
    code.push_str("    match status {\n");
    code.push_str("        radio::Status::Active { .. } => base,\n");
    code.push_str("        radio::Status::Hovered { .. } => ");
    if let Some(ov) = status_hovered {
        let has_override = ov.text_color.is_some()
            || ov.text_color_source.is_some()
            || ov.background_color.is_some()
            || ov.background_color_source.is_some()
            || ov.border_color.is_some()
            || ov.border_color_source.is_some()
            || ov.icon_color.is_some()
            || ov.icon_color_source.is_some();
        if has_override {
            code.push_str("radio::Style {\n");
            code.push_str(&format!(
                "            background: Background::Color({}),\n",
                format_color_with_source(
                    ov.background_color.unwrap_or(background_color),
                    &ov.background_color_source
                )
            ));
            code.push_str(&format!(
                "            dot_color: {},\n",
                format_color_with_source(ov.icon_color.unwrap_or(dot_color), &ov.icon_color_source)
            ));
            code.push_str(&format!(
                "            border_color: {},\n",
                format_color_with_source(
                    ov.border_color.unwrap_or(border_color),
                    &ov.border_color_source
                )
            ));
            code.push_str(&format!(
                "            text_color: Some({}),\n",
                format_color_with_source(
                    ov.text_color.unwrap_or(text_color),
                    &ov.text_color_source
                )
            ));
            code.push_str("            ..base\n        },\n");
        } else {
            code.push_str(
                "radio::Style { background: base.background.scale_alpha(0.85), ..base },\n",
            );
        }
    } else {
        code.push_str("radio::Style { background: base.background.scale_alpha(0.85), ..base },\n");
    }
    code.push_str("    }\n");
    code.push_str("}");
    code
}

pub fn generate_toggler_style_code(
    function_name: &str,
    text_color: Color,
    text_color_source: &Option<String>,
    background_color: Color,
    background_color_source: &Option<String>,
    foreground_color: Color,
    foreground_color_source: &Option<String>,
    border_color: Color,
    border_color_source: &Option<String>,
    border_width: f32,
    border_radius_top_left: f32,
    border_radius_top_right: f32,
    border_radius_bottom_right: f32,
    border_radius_bottom_left: f32,
    status_hovered: Option<&StatusColorOverride>,
    status_disabled: Option<&StatusColorOverride>,
) -> String {
    let radius = if border_radius_top_left == 0.0
        && border_radius_top_right == 0.0
        && border_radius_bottom_right == 0.0
        && border_radius_bottom_left == 0.0
    {
        "None".to_string()
    } else {
        format!(
            "Some({})",
            format_radius(
                border_radius_top_left,
                border_radius_top_right,
                border_radius_bottom_right,
                border_radius_bottom_left,
                "            ",
            )
        )
    };
    let mut code = String::new();

    code.push_str(&format!(
        "pub fn {}(theme: &Theme, status: toggler::Status) -> toggler::Style {{\n",
        function_name
    ));
    code.push_str("    let palette = theme.extended_palette();\n\n");
    code.push_str("    let base = toggler::Style {\n");
    code.push_str("        background: Background::Color(");
    code.push_str(&format_color_with_source(
        background_color,
        background_color_source,
    ));
    code.push_str("),\n");
    code.push_str(&format!(
        "        background_border_width: {:.1},\n",
        border_width
    ));
    code.push_str("        background_border_color: ");
    code.push_str(&format_color_with_source(border_color, border_color_source));
    code.push_str(",\n");
    code.push_str("        foreground: Background::Color(");
    code.push_str(&format_color_with_source(
        foreground_color,
        foreground_color_source,
    ));
    code.push_str("),\n");
    code.push_str(&format!(
        "        foreground_border_width: {:.1},\n",
        border_width
    ));
    code.push_str("        foreground_border_color: ");
    code.push_str(&format_color_with_source(border_color, border_color_source));
    code.push_str(",\n");
    code.push_str("        text_color: Some(");
    code.push_str(&format_color_with_source(text_color, text_color_source));
    code.push_str("),\n");
    code.push_str(&format!("        border_radius: {},\n", radius));
    code.push_str("        padding_ratio: 0.1,\n");
    code.push_str("    };\n\n");
    code.push_str("    match status {\n");
    code.push_str("        toggler::Status::Active { .. } => base,\n");
    code.push_str("        toggler::Status::Hovered { .. } => ");
    if let Some(ov) = status_hovered {
        let has_override = ov.text_color.is_some()
            || ov.text_color_source.is_some()
            || ov.background_color.is_some()
            || ov.background_color_source.is_some()
            || ov.border_color.is_some()
            || ov.border_color_source.is_some()
            || ov.icon_color.is_some()
            || ov.icon_color_source.is_some();
        if has_override {
            code.push_str("toggler::Style {\n");
            code.push_str(&format!(
                "            background: Background::Color({}),\n",
                format_color_with_source(
                    ov.background_color.unwrap_or(background_color),
                    &ov.background_color_source
                )
            ));
            code.push_str(&format!(
                "            foreground: Background::Color({}),\n",
                format_color_with_source(
                    ov.icon_color.unwrap_or(foreground_color),
                    &ov.icon_color_source
                )
            ));
            let border_value = format_color_with_source(
                ov.border_color.unwrap_or(border_color),
                &ov.border_color_source,
            );
            code.push_str(&format!(
                "            background_border_color: {},\n            foreground_border_color: {},\n",
                border_value, border_value
            ));
            code.push_str(&format!(
                "            text_color: Some({}),\n",
                format_color_with_source(
                    ov.text_color.unwrap_or(text_color),
                    &ov.text_color_source
                )
            ));
            code.push_str("            ..base\n        },\n");
        } else {
            code.push_str(
                "toggler::Style { foreground: base.foreground.scale_alpha(0.85), ..base },\n",
            );
        }
    } else {
        code.push_str(
            "toggler::Style { foreground: base.foreground.scale_alpha(0.85), ..base },\n",
        );
    }
    code.push_str("        toggler::Status::Disabled { .. } => ");
    if let Some(ov) = status_disabled {
        let border_value = format_color_with_source(
            ov.border_color.unwrap_or(border_color),
            &ov.border_color_source,
        );
        code.push_str("toggler::Style {\n");
        code.push_str(&format!(
            "            background: Background::Color({}),\n",
            format_color_with_source(
                ov.background_color.unwrap_or(background_color),
                &ov.background_color_source
            )
        ));
        code.push_str(&format!(
            "            foreground: Background::Color({}),\n",
            format_color_with_source(
                ov.icon_color.unwrap_or(foreground_color),
                &ov.icon_color_source
            )
        ));
        code.push_str(&format!(
            "            background_border_color: {},\n            foreground_border_color: {},\n",
            border_value, border_value
        ));
        code.push_str(&format!(
            "            text_color: Some({}),\n",
            format_color_with_source(ov.text_color.unwrap_or(text_color), &ov.text_color_source)
        ));
        code.push_str("            ..base\n        },\n");
    } else {
        code.push_str(
            "toggler::Style { background: base.background.scale_alpha(0.5), foreground: base.foreground.scale_alpha(0.5), background_border_color: base.background_border_color.scale_alpha(0.5), foreground_border_color: base.foreground_border_color.scale_alpha(0.5), text_color: base.text_color.map(|c| c.scale_alpha(0.5)), ..base },\n",
        );
    }
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
    status_hovered: Option<&StatusColorOverride>,
    status_focused: Option<&StatusColorOverride>,
    status_disabled: Option<&StatusColorOverride>,
) -> String {
    let mut code = String::new();

    code.push_str(&format!("pub fn {}_input_style(theme: &Theme, status: text_input::Status) -> text_input::Style {{\n", style_name));
    code.push_str("    let palette = theme.extended_palette();\n\n");

    code.push_str("    let base = text_input::Style {\n");

    code.push_str("        background: Background::Color(");
    code.push_str(&format_color_with_source(
        background_color,
        background_color_source,
    ));
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
    code.push_str(&format_color_with_source(
        placeholder_color,
        placeholder_color_source,
    ));
    code.push_str(",\n");

    code.push_str("        value: ");
    code.push_str(&format_color_with_source(text_color, text_color_source));
    code.push_str(",\n");

    code.push_str("        selection: ");
    code.push_str(&format_color_with_source(
        selection_color,
        selection_color_source,
    ));
    code.push_str(",\n");

    code.push_str("    };\n\n");

    code.push_str("    match status {\n");
    code.push_str("        text_input::Status::Active => base,\n");
    // Hovered
    code.push_str("        text_input::Status::Hovered => ");
    let hov_bc = status_hovered.and_then(|ov| {
        if ov.border_color.is_some() || ov.border_color_source.is_some() {
            Some(ov)
        } else {
            None
        }
    });
    if let Some(ov) = hov_bc {
        let resolved = ov.border_color.unwrap_or(border_color);
        code.push_str(&format!(
            "text_input::Style {{ border: Border {{ color: {}, ..base.border }}, ..base }},\n",
            format_color_with_source(resolved, &ov.border_color_source)
        ));
    } else {
        code.push_str("text_input::Style { border: Border { color: base.border.color.scale_alpha(0.8), ..base.border }, ..base },\n");
    }
    // Focused
    code.push_str("        text_input::Status::Focused { .. } => ");
    let foc_bc = status_focused.and_then(|ov| {
        if ov.border_color.is_some() || ov.border_color_source.is_some() {
            Some(ov)
        } else {
            None
        }
    });
    if let Some(ov) = foc_bc {
        let resolved = ov.border_color.unwrap_or(border_color);
        code.push_str(&format!("text_input::Style {{ border: Border {{ color: {}, width: base.border.width.max(1.0), ..base.border }}, ..base }},\n", format_color_with_source(resolved, &ov.border_color_source)));
    } else {
        code.push_str("text_input::Style { border: Border { color: base.border.color, width: base.border.width.max(1.0), ..base.border }, ..base },\n");
    }
    // Disabled
    code.push_str("        text_input::Status::Disabled => ");
    let dis_bg = status_disabled.and_then(|ov| {
        if ov.background_color.is_some() || ov.background_color_source.is_some() {
            Some(ov)
        } else {
            None
        }
    });
    if let Some(ov) = dis_bg {
        let resolved = ov.background_color.unwrap_or(background_color);
        code.push_str(&format!("text_input::Style {{ background: Background::Color({}), value: base.value.scale_alpha(0.5), placeholder: base.placeholder.scale_alpha(0.5), ..base }},\n", format_color_with_source(resolved, &ov.background_color_source)));
    } else {
        code.push_str("text_input::Style { background: base.background.scale_alpha(0.5), value: base.value.scale_alpha(0.5), placeholder: base.placeholder.scale_alpha(0.5), ..base },\n");
    }
    code.push_str("    }\n");
    code.push_str("}\n\n");

    code.push_str(&format!(
        "pub fn {}_menu_style(theme: &Theme) -> menu::Style {{\n",
        style_name
    ));
    code.push_str("    let palette = theme.extended_palette();\n\n");
    code.push_str("    menu::Style {\n");

    code.push_str("        background: Background::Color(");
    code.push_str(&format_color_with_source(
        background_color,
        background_color_source,
    ));
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
    code.push_str(&format_color_with_source(
        selected_text_color,
        selected_text_color_source,
    ));
    code.push_str(",\n");

    code.push_str("        selected_background: Background::Color(");
    code.push_str(&format_color_with_source(
        selected_background_color,
        selected_background_color_source,
    ));
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

/// Generate a complete `styles.rs` file containing all custom styles, organized as:
///   pub mod button { ... }
///   pub mod container { ... }
///   pub mod rule { ... }
/// Returns `None` if there are no custom styles to generate.
pub fn generate_all_styles_file(custom_styles: &CustomThemes) -> Option<String> {
    let styles = custom_styles.styles();

    let has_button = styles
        .get(&ThemePaneEnum::Button)
        .map(|m| !m.is_empty())
        .unwrap_or(false);
    let has_container = styles
        .get(&ThemePaneEnum::Container)
        .map(|m| !m.is_empty())
        .unwrap_or(false);
    let has_checkbox = styles
        .get(&ThemePaneEnum::Checkbox)
        .map(|m| !m.is_empty())
        .unwrap_or(false);
    let has_text_input = styles
        .get(&ThemePaneEnum::TextInput)
        .map(|m| !m.is_empty())
        .unwrap_or(false);
    let has_menu = styles
        .get(&ThemePaneEnum::Menu)
        .map(|m| !m.is_empty())
        .unwrap_or(false);
    let has_picklist = styles
        .get(&ThemePaneEnum::Picklist)
        .map(|m| !m.is_empty())
        .unwrap_or(false);
    let has_slider = styles
        .get(&ThemePaneEnum::Slider)
        .map(|m| !m.is_empty())
        .unwrap_or(false);
    let has_progressbar = styles
        .get(&ThemePaneEnum::Progressbar)
        .map(|m| !m.is_empty())
        .unwrap_or(false);
    let has_radio = styles
        .get(&ThemePaneEnum::Radio)
        .map(|m| !m.is_empty())
        .unwrap_or(false);
    let has_toggler = styles
        .get(&ThemePaneEnum::Toggler)
        .map(|m| !m.is_empty())
        .unwrap_or(false);
    let has_combobox = styles
        .get(&ThemePaneEnum::Combobox)
        .map(|m| !m.is_empty())
        .unwrap_or(false);
    let has_rule = styles
        .get(&ThemePaneEnum::Rule)
        .map(|m| !m.is_empty())
        .unwrap_or(false);

    if !has_button
        && !has_container
        && !has_checkbox
        && !has_text_input
        && !has_menu
        && !has_picklist
        && !has_slider
        && !has_progressbar
        && !has_radio
        && !has_toggler
        && !has_combobox
        && !has_rule
    {
        return None;
    }

    let mut out = String::new();

    // Top-level imports
    out.push_str("use iced::{Background, Border, Color, Shadow, Theme, Vector};\n");
    if has_text_input
        || has_menu
        || has_picklist
        || has_slider
        || has_progressbar
        || has_radio
        || has_toggler
        || has_combobox
    {
        out.push_str(
            "use iced::widget::{overlay::menu, pick_list, progress_bar, radio, slider, text_input, toggler};\n",
        );
    }
    out.push_str("\n");

    // Button module
    if has_button {
        out.push_str("pub mod button {\n");
        out.push_str("    use super::*;\n");
        out.push_str("    use iced::widget::button::{Status, Style};\n");
        if let Some(map) = styles.get(&ThemePaneEnum::Button) {
            for (_, def) in map {
                out.push_str("\n");
                let fn_code = generate_button_style_code(
                    &def.name.to_lowercase().replace(' ', "_"),
                    def.text_color,
                    &def.text_color_source,
                    def.background_color,
                    &def.background_color_source,
                    def.border_color,
                    &def.border_color_source,
                    def.border_width,
                    def.border_radius_top_left,
                    def.border_radius_top_right,
                    def.border_radius_bottom_right,
                    def.border_radius_bottom_left,
                    def.shadow_enabled,
                    def.shadow_color,
                    &def.shadow_color_source,
                    def.shadow_offset_x,
                    def.shadow_offset_y,
                    def.shadow_blur_radius,
                    def.snap,
                    def.status_hovered.as_ref(),
                    def.status_pressed.as_ref(),
                    def.status_disabled.as_ref(),
                );
                for line in fn_code.lines() {
                    out.push_str("    ");
                    out.push_str(line);
                    out.push('\n');
                }
            }
        }
        out.push_str("}\n\n");
    }

    // Container module
    if has_container {
        out.push_str("pub mod container {\n");
        out.push_str("    use super::*;\n");
        out.push_str("    use iced::widget::container::Style;\n");
        if let Some(map) = styles.get(&ThemePaneEnum::Container) {
            for (_, def) in map {
                out.push_str("\n");
                let fn_code = generate_container_style_code(
                    &def.name.to_lowercase().replace(' ', "_"),
                    def.text_color,
                    &def.text_color_source,
                    def.background_color,
                    &def.background_color_source,
                    def.border_color,
                    &def.border_color_source,
                    def.border_width,
                    def.border_radius_top_left,
                    def.border_radius_top_right,
                    def.border_radius_bottom_right,
                    def.border_radius_bottom_left,
                    def.shadow_enabled,
                    def.shadow_color,
                    &def.shadow_color_source,
                    def.shadow_offset_x,
                    def.shadow_offset_y,
                    def.shadow_blur_radius,
                    def.snap,
                );
                for line in fn_code.lines() {
                    out.push_str("    ");
                    out.push_str(line);
                    out.push('\n');
                }
            }
        }
        out.push_str("}\n\n");
    }

    // Checkbox module
    if has_checkbox {
        out.push_str("pub mod checkbox {\n");
        out.push_str("    use super::*;\n");
        out.push_str("    use iced::widget::checkbox::{Status, Style};\n");
        if let Some(map) = styles.get(&ThemePaneEnum::Checkbox) {
            for (_, def) in map {
                out.push_str("\n");
                let fn_code = generate_checkbox_style_code(
                    &def.name.to_lowercase().replace(' ', "_"),
                    def.text_color,
                    &def.text_color_source,
                    def.background_color,
                    &def.background_color_source,
                    def.icon_color,
                    &def.icon_color_source,
                    def.border_color,
                    &def.border_color_source,
                    def.border_width,
                    def.border_radius_top_left,
                    def.border_radius_top_right,
                    def.border_radius_bottom_right,
                    def.border_radius_bottom_left,
                    def.status_hovered.as_ref(),
                    def.status_disabled.as_ref(),
                );
                for line in fn_code.lines() {
                    out.push_str("    ");
                    out.push_str(line);
                    out.push('\n');
                }
            }
        }
        out.push_str("}\n\n");
    }

    if has_text_input {
        out.push_str("pub mod text_input {\n");
        out.push_str("    use super::*;\n");
        if let Some(map) = styles.get(&ThemePaneEnum::TextInput) {
            for (_, def) in map {
                out.push_str("\n");
                let fn_code = generate_text_input_style_code(
                    &def.name.to_lowercase().replace(' ', "_"),
                    def.background_color,
                    &def.background_color_source,
                    def.border_color,
                    &def.border_color_source,
                    def.border_width,
                    def.border_radius_top_left,
                    def.border_radius_top_right,
                    def.border_radius_bottom_right,
                    def.border_radius_bottom_left,
                    def.icon_color,
                    &def.icon_color_source,
                    def.placeholder_color,
                    &def.placeholder_color_source,
                    def.text_color,
                    &def.text_color_source,
                    def.selection_color,
                    &def.selection_color_source,
                    def.status_hovered.as_ref(),
                    def.status_focused.as_ref(),
                    def.status_disabled.as_ref(),
                );
                for line in fn_code.lines() {
                    out.push_str("    ");
                    out.push_str(line);
                    out.push('\n');
                }
            }
        }
        out.push_str("}\n\n");
    }

    if has_menu {
        out.push_str("pub mod menu {\n");
        out.push_str("    use super::*;\n");
        if let Some(map) = styles.get(&ThemePaneEnum::Menu) {
            for (_, def) in map {
                out.push_str("\n");
                let fn_code = generate_menu_style_code(
                    &def.name.to_lowercase().replace(' ', "_"),
                    def.background_color,
                    &def.background_color_source,
                    def.border_color,
                    &def.border_color_source,
                    def.border_width,
                    def.border_radius_top_left,
                    def.border_radius_top_right,
                    def.border_radius_bottom_right,
                    def.border_radius_bottom_left,
                    def.text_color,
                    &def.text_color_source,
                    def.selected_text_color,
                    &def.selected_text_color_source,
                    def.selected_background_color,
                    &def.selected_background_color_source,
                    def.shadow_enabled,
                    def.shadow_color,
                    &def.shadow_color_source,
                    def.shadow_offset_x,
                    def.shadow_offset_y,
                    def.shadow_blur_radius,
                );
                for line in fn_code.lines() {
                    out.push_str("    ");
                    out.push_str(line);
                    out.push('\n');
                }
            }
        }
        out.push_str("}\n\n");
    }

    if has_picklist {
        out.push_str("pub mod pick_list {\n");
        out.push_str("    use super::*;\n");
        if let Some(map) = styles.get(&ThemePaneEnum::Picklist) {
            for (_, def) in map {
                out.push_str("\n");
                let fn_code = generate_pick_list_style_code(
                    &def.name.to_lowercase().replace(' ', "_"),
                    def.text_color,
                    &def.text_color_source,
                    def.background_color,
                    &def.background_color_source,
                    def.placeholder_color,
                    &def.placeholder_color_source,
                    def.icon_color,
                    &def.icon_color_source,
                    def.border_color,
                    &def.border_color_source,
                    def.border_width,
                    def.border_radius_top_left,
                    def.border_radius_top_right,
                    def.border_radius_bottom_right,
                    def.border_radius_bottom_left,
                    def.status_hovered.as_ref(),
                    def.status_focused.as_ref(),
                );
                for line in fn_code.lines() {
                    out.push_str("    ");
                    out.push_str(line);
                    out.push('\n');
                }
            }
        }
        out.push_str("}\n\n");
    }

    if has_slider {
        out.push_str("pub mod slider {\n");
        out.push_str("    use super::*;\n");
        if let Some(map) = styles.get(&ThemePaneEnum::Slider) {
            for (_, def) in map {
                out.push_str("\n");
                let fn_code = generate_slider_style_code(
                    &def.name.to_lowercase().replace(' ', "_"),
                    def.text_color,
                    &def.text_color_source,
                    def.background_color,
                    &def.background_color_source,
                    def.icon_color,
                    &def.icon_color_source,
                    def.border_color,
                    &def.border_color_source,
                    def.border_width,
                    def.border_radius_top_left,
                    def.border_radius_top_right,
                    def.border_radius_bottom_right,
                    def.border_radius_bottom_left,
                    def.status_hovered.as_ref(),
                    def.status_pressed.as_ref(),
                );
                for line in fn_code.lines() {
                    out.push_str("    ");
                    out.push_str(line);
                    out.push('\n');
                }
            }
        }
        out.push_str("}\n\n");
    }

    if has_progressbar {
        out.push_str("pub mod progress_bar {\n");
        out.push_str("    use super::*;\n");
        if let Some(map) = styles.get(&ThemePaneEnum::Progressbar) {
            for (_, def) in map {
                out.push_str("\n");
                let fn_code = generate_progress_bar_style_code(
                    &def.name.to_lowercase().replace(' ', "_"),
                    def.text_color,
                    &def.text_color_source,
                    def.background_color,
                    &def.background_color_source,
                    def.border_color,
                    &def.border_color_source,
                    def.border_width,
                    def.border_radius_top_left,
                    def.border_radius_top_right,
                    def.border_radius_bottom_right,
                    def.border_radius_bottom_left,
                );
                for line in fn_code.lines() {
                    out.push_str("    ");
                    out.push_str(line);
                    out.push('\n');
                }
            }
        }
        out.push_str("}\n\n");
    }

    if has_radio {
        out.push_str("pub mod radio {\n");
        out.push_str("    use super::*;\n");
        if let Some(map) = styles.get(&ThemePaneEnum::Radio) {
            for (_, def) in map {
                out.push_str("\n");
                let fn_code = generate_radio_style_code(
                    &def.name.to_lowercase().replace(' ', "_"),
                    def.text_color,
                    &def.text_color_source,
                    def.background_color,
                    &def.background_color_source,
                    def.icon_color,
                    &def.icon_color_source,
                    def.border_color,
                    &def.border_color_source,
                    def.border_width,
                    def.status_hovered.as_ref(),
                );
                for line in fn_code.lines() {
                    out.push_str("    ");
                    out.push_str(line);
                    out.push('\n');
                }
            }
        }
        out.push_str("}\n\n");
    }

    if has_toggler {
        out.push_str("pub mod toggler {\n");
        out.push_str("    use super::*;\n");
        if let Some(map) = styles.get(&ThemePaneEnum::Toggler) {
            for (_, def) in map {
                out.push_str("\n");
                let fn_code = generate_toggler_style_code(
                    &def.name.to_lowercase().replace(' ', "_"),
                    def.text_color,
                    &def.text_color_source,
                    def.background_color,
                    &def.background_color_source,
                    def.icon_color,
                    &def.icon_color_source,
                    def.border_color,
                    &def.border_color_source,
                    def.border_width,
                    def.border_radius_top_left,
                    def.border_radius_top_right,
                    def.border_radius_bottom_right,
                    def.border_radius_bottom_left,
                    def.status_hovered.as_ref(),
                    def.status_disabled.as_ref(),
                );
                for line in fn_code.lines() {
                    out.push_str("    ");
                    out.push_str(line);
                    out.push('\n');
                }
            }
        }
        out.push_str("}\n\n");
    }

    // ComboBox module
    if has_combobox {
        out.push_str("pub mod combo_box {\n");
        out.push_str("    use super::*;\n");
        if let Some(map) = styles.get(&ThemePaneEnum::Combobox) {
            for (_, def) in map {
                out.push_str("\n");
                let fn_code = generate_combo_box_style_code(
                    &def.name.to_lowercase().replace(' ', "_"),
                    def.background_color,
                    &def.background_color_source,
                    def.border_color,
                    &def.border_color_source,
                    def.border_width,
                    def.border_radius_top_left,
                    def.border_radius_top_right,
                    def.border_radius_bottom_right,
                    def.border_radius_bottom_left,
                    def.icon_color,
                    &def.icon_color_source,
                    def.placeholder_color,
                    &def.placeholder_color_source,
                    def.text_color,
                    &def.text_color_source,
                    def.selection_color,
                    &def.selection_color_source,
                    def.selected_text_color,
                    &def.selected_text_color_source,
                    def.selected_background_color,
                    &def.selected_background_color_source,
                    def.shadow_enabled,
                    def.shadow_color,
                    &def.shadow_color_source,
                    def.shadow_offset_x,
                    def.shadow_offset_y,
                    def.shadow_blur_radius,
                    def.status_hovered.as_ref(),
                    def.status_focused.as_ref(),
                    def.status_disabled.as_ref(),
                );
                for line in fn_code.lines() {
                    out.push_str("    ");
                    out.push_str(line);
                    out.push('\n');
                }
            }
        }
        out.push_str("}\n\n");
    }

    // Rule module
    if has_rule {
        out.push_str("pub mod rule {\n");
        out.push_str("    use super::*;\n");
        out.push_str("    use iced::widget::rule::{FillMode, Style};\n");
        if let Some(map) = styles.get(&ThemePaneEnum::Rule) {
            for (_, def) in map {
                out.push_str("\n");
                let fn_code = generate_rule_style_code(
                    &def.name.to_lowercase().replace(' ', "_"),
                    def.border_color,
                    &def.border_color_source,
                    def.border_radius_top_left,
                    def.border_radius_top_right,
                    def.border_radius_bottom_right,
                    def.border_radius_bottom_left,
                    &def.rule_fill_mode,
                    def.snap,
                );
                for line in fn_code.lines() {
                    out.push_str("    ");
                    out.push_str(line);
                    out.push('\n');
                }
            }
        }
        out.push_str("}\n");
    }

    Some(out)
}

use iced::Color;
use super::helpers::{format_color_with_source, format_radius, format_shadow};
use crate::styles::style_enum::RuleFillMode;
use crate::views::theme_and_stylefn_builder::{CustomThemes, ThemePaneEnum};

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

/// Generate a complete `styles.rs` file containing all custom styles, organized as:
///   pub mod button { ... }
///   pub mod container { ... }
///   pub mod rule { ... }
/// Returns `None` if there are no custom styles to generate.
pub fn generate_all_styles_file(custom_styles: &CustomThemes) -> Option<String> {
    let styles = custom_styles.styles();

    let has_button    = styles.get(&ThemePaneEnum::Button).map(|m| !m.is_empty()).unwrap_or(false);
    let has_container = styles.get(&ThemePaneEnum::Container).map(|m| !m.is_empty()).unwrap_or(false);
    let has_checkbox  = styles.get(&ThemePaneEnum::Checkbox).map(|m| !m.is_empty()).unwrap_or(false);
    let has_combobox  = styles.get(&ThemePaneEnum::Combobox).map(|m| !m.is_empty()).unwrap_or(false);
    let has_rule      = styles.get(&ThemePaneEnum::Rule).map(|m| !m.is_empty()).unwrap_or(false);

    if !has_button && !has_container && !has_checkbox && !has_combobox && !has_rule {
        return None;
    }

    let mut out = String::new();

    // Top-level imports
    out.push_str("use iced::{Background, Border, Color, Shadow, Theme, Vector};\n");
    if has_combobox {
        out.push_str("use iced::widget::{combo_box::menu, text_input};\n");
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
                    def.text_color, &def.text_color_source,
                    def.background_color, &def.background_color_source,
                    def.border_color, &def.border_color_source,
                    def.border_width,
                    def.border_radius_top_left, def.border_radius_top_right,
                    def.border_radius_bottom_right, def.border_radius_bottom_left,
                    def.shadow_enabled,
                    def.shadow_color, &def.shadow_color_source,
                    def.shadow_offset_x, def.shadow_offset_y, def.shadow_blur_radius,
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
                    def.text_color, &def.text_color_source,
                    def.background_color, &def.background_color_source,
                    def.border_color, &def.border_color_source,
                    def.border_width,
                    def.border_radius_top_left, def.border_radius_top_right,
                    def.border_radius_bottom_right, def.border_radius_bottom_left,
                    def.shadow_enabled,
                    def.shadow_color, &def.shadow_color_source,
                    def.shadow_offset_x, def.shadow_offset_y, def.shadow_blur_radius,
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
                    def.text_color, &def.text_color_source,
                    def.background_color, &def.background_color_source,
                    def.icon_color, &def.icon_color_source,
                    def.border_color, &def.border_color_source,
                    def.border_width,
                    def.border_radius_top_left, def.border_radius_top_right,
                    def.border_radius_bottom_right, def.border_radius_bottom_left,
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
                    def.background_color, &def.background_color_source,
                    def.border_color, &def.border_color_source,
                    def.border_width,
                    def.border_radius_top_left, def.border_radius_top_right,
                    def.border_radius_bottom_right, def.border_radius_bottom_left,
                    def.icon_color, &def.icon_color_source,
                    def.placeholder_color, &def.placeholder_color_source,
                    def.text_color, &def.text_color_source,
                    def.selection_color, &def.selection_color_source,
                    def.selected_text_color, &def.selected_text_color_source,
                    def.selected_background_color, &def.selected_background_color_source,
                    def.shadow_enabled,
                    def.shadow_color, &def.shadow_color_source,
                    def.shadow_offset_x, def.shadow_offset_y, def.shadow_blur_radius,
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
                    def.border_color, &def.border_color_source,
                    def.border_radius_top_left, def.border_radius_top_right,
                    def.border_radius_bottom_right, def.border_radius_bottom_left,
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

use iced::Color;

/// Format a color value as code, using theme source if available, otherwise RGBA.
pub fn format_color_with_source(color: Color, source: &Option<String>) -> String {
    if let Some(src) = source {
        // Strip "theme.extended_palette()." prefix if present
        let path = src
            .strip_prefix("theme.extended_palette().")
            .unwrap_or(src);
        //format!("extended.{}", path)
        path.to_string()
    } else {
        format!(
            "Color {{ r: {:.3}, g: {:.3}, b: {:.3}, a: {:.3} }}",
            color.r, color.g, color.b, color.a
        )
    }
}

/// Format border radius as a code string.
/// If all values are the same, outputs the compact `{value}.into()` form.
/// If values differ, outputs the full `Radius { top_left, top_right, bottom_right, bottom_left }` struct.
pub fn format_radius(
    top_left: f32,
    top_right: f32,
    bottom_right: f32,
    bottom_left: f32,
    indent: &str,
) -> String {
    if top_left == top_right && top_right == bottom_right && bottom_right == bottom_left {
        format!("{:.1}.into()", top_left)
    } else {
        let inner = format!("{}    ", indent);
        format!(
            "Radius {{\n\
             {inner}top_left: {:.1},\n\
             {inner}top_right: {:.1},\n\
             {inner}bottom_right: {:.1},\n\
             {inner}bottom_left: {:.1},\n\
             {indent}}}",
            top_left, top_right, bottom_right, bottom_left,
        )
    }
}

/// Format a shadow struct as a code string.
pub fn format_shadow(
    enabled: bool,
    color: Color,
    color_source: &Option<String>,
    offset_x: f32,
    offset_y: f32,
    blur_radius: f32,
    indent: &str,
) -> String {
    if enabled {
        let inner = format!("{}    ", indent);
        format!(
            "Shadow {{\n\
             {inner}color: {},\n\
             {inner}offset: Vector {{ x: {:.1}, y: {:.1} }},\n\
             {inner}blur_radius: {:.1},\n\
             {indent}}}",
            format_color_with_source(color, color_source),
            offset_x, offset_y,
            blur_radius,
        )
    } else {
        "Shadow::default()".to_string()
    }
}

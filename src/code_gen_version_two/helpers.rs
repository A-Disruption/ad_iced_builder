use iced::{Color, Element, Length};
use iced::widget::button;
use widgets::generic_overlay::OverlayButton;
use widgets;

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

pub fn internal_overlay<'a, Message, Theme, Renderer>(
    hover_element: impl Into<Element<'a, Message, Theme, Renderer>>,
    copy_button: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> OverlayButton<'a, Message, Theme, Renderer> 
where 
    Renderer: iced::advanced::Renderer + iced::advanced::text::Renderer,
    Theme: widgets::generic_overlay::Catalog + button::Catalog,
{
    OverlayButton::new(hover_element, "", copy_button)
        .hide_header()
        .on_hover()
        .overlay_padding(0.0)
        .overlay_padding(0.0)
        .hover_gap(11.0)
        .overlay_height(Length::Shrink)
        .overlay_width(Length::Shrink)
        .hover_position(widgets::generic_overlay::Position::Right)
        .hover_mode(widgets::generic_overlay::PositionMode::Inside)
        .hover_alignment(iced::Alignment::Start)
        .interactive_base(true)
}
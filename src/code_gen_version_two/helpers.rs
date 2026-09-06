use iced::{Color, Element};
use widgets::anchor::{Align, Side};
use widgets::popover::{Popover, hover_popover};

/// Format a color value as code, using theme source if available, otherwise RGBA.
pub fn format_color_with_source(color: Color, source: &Option<String>) -> String {
    if let Some(src) = source {
        // Strip "theme.palette()." prefix if present
        let path = src.strip_prefix("theme.palette().").unwrap_or(src);
        format!("palette.{}", path)
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
            offset_x,
            offset_y,
            blur_radius,
        )
    } else {
        "Shadow::default()".to_string()
    }
}

/// Reveals `corner` over the top-right of `content` on hover.
///
/// Replaces the `generic_overlay` hover button the widgets crate dropped when
/// it moved to iced master. `hover_popover` is the same interaction — the
/// cursor can travel into the surface and press what is inside it — and the
/// negative gap reproduces the old `PositionMode::Inside`, pulling the corner
/// element back over the trailing edge of the content instead of sitting
/// outside it.
pub fn internal_overlay<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
    corner: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Popover<'a, Message, Theme, Renderer>
where
    Theme: widgets::popover::Catalog,
    Renderer: iced::advanced::Renderer,
{
    hover_popover(content, corner)
        .side(Side::Right)
        .align(Align::Start)
        .gap(-COPY_BUTTON_INSET)
        .flip(false)
        .padding(0.0)
}

/// How far back over the content edge `internal_overlay` pulls its corner
/// element: the width of an icon button plus the old overlay's 11px gap.
const COPY_BUTTON_INSET: f32 = 41.0;

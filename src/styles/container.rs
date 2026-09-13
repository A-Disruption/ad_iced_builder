use iced::widget::container::*;
use iced::{Background, Border, Theme};

/// One pane of the editor's grid.
///
/// Just enough edge to say where a pane stops, since the gap between them is
/// also the resize handle and needs to read as a seam rather than as padding.
pub fn pane(theme: &Theme) -> Style {
    let palette = theme.palette();

    Style {
        background: Some(Background::Color(palette.background.base.color)),
        border: Border {
            color: palette.background.strong.color,
            width: 1.0,
            radius: 4.0.into(),
        },
        ..Default::default()
    }
}

pub fn error_box(theme: &Theme) -> Style {
    let palette = theme.palette();

    Style {
        background: Some(Background::Color(palette.danger.base.color)),
        border: Border {
            color: palette.danger.base.text,
            width: 1.0,
            radius: 5.0.into(),
        },
        ..Default::default()
    }
}

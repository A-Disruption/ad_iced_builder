use iced::widget::container::*;
use iced::{border::{radius, Radius}, Border, Theme, Shadow, Color, Background, Vector};


fn custom_style() -> Style {
    Style {
        text_color: Some(Color::from_rgba(1.0, 0.0, 1.0, 1.0)),
        background: Some(Background::Color(Color::from_rgba(0.3, 0.0, 0.5, 1.0))),
        border: Border {
            color: Color::from_rgba(1.0, 0.0, 1.0, 1.0),
            width: 1.0,
            radius: Radius {
                top_left: 30.0,
                top_right: 30.0,
                bottom_right: 30.0,
                bottom_left: 30.0,
            },
        },
        shadow: Shadow {
            color: Color::from_rgba(0.3, 0.1, 0.3, 1.0),
            offset: Vector { x: 2.0, y: 2.0 },
            blur_radius: 15.0,
        },
        snap: true,
    }
}

pub fn rounded_box(theme: &Theme) -> Style {
    let palette = theme.extended_palette();

    Style {
        background: Some(Background::Color(palette.background.base.color)),
        border: Border {
            color: palette.background.base.text,
            width: 1.0,
            radius: 8.0.into(),
        },
        ..Default::default()
    }
}

pub fn warning_box(theme: &Theme) -> Style {
    let palette = theme.extended_palette();

    Style {
        background: Some(Background::Color(palette.warning.weak.color)),
        border: Border {
            color: palette.warning.weak.text,
            width: 1.0,
            radius: 5.0.into(),
        },
        ..Default::default()
    }
}

pub fn error_box(theme: &Theme) -> Style {
    let palette = theme.extended_palette();

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
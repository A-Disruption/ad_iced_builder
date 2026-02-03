pub use iced::widget::checkbox::*;
use iced::{border::{radius, Radius}, Border, Theme, Color, Background, Vector, Shadow};

pub fn checkbox_style(theme: &Theme, status: Status) -> Style {
    let extended = theme.extended_palette();

    let base = Style {
        background: Background::Color(extended.background.base.color),
        icon_color: extended.primary.base.text,
        border: Border {
            color: extended.background.strong.color,
            width: 1.0,
            radius: 2.0.into(),
        },
        text_color: Some(extended.background.base.text),
    };

    match status {
        Status::Active { .. } => base,
        Status::Hovered { .. } => Style {
            text_color: base.text_color.map(|c| c.scale_alpha(0.8)),
            ..base
        },
        Status::Disabled { .. } => Style {
            icon_color: base.icon_color.scale_alpha(0.5),
            text_color: base.text_color.map(|c| c.scale_alpha(0.5)),
            ..base
        },
    }
}
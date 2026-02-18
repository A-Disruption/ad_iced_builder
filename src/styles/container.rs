use iced::widget::container::*;
use iced::{Border, Theme, Background};

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
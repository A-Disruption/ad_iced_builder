pub use iced::widget::button::*;
use iced::{border::radius, Border, Theme};


pub fn cancel(theme: &Theme, status: Status) -> Style {

    let palette = theme.extended_palette();

    let base = Style {
        text_color: palette.danger.strong.color,
        ..Style::default()
    };

    match status {
        Status::Active | Status::Pressed => base,
        Status::Hovered => Style {
            text_color: palette.danger.strong.color.scale_alpha(0.8),
            ..base
        },
        Status::Disabled => disabled(base),
    }
}

pub fn edit(theme: &Theme, status: Status) -> Style {

    let palette = theme.extended_palette();

    let base = Style {
        text_color: palette.primary.strong.color,
        border: Border {
            radius: radius(4.0),
            ..Border::default()
        },
        ..Style::default()
    };

    match status {
        Status::Active | Status::Pressed => base,
        Status::Hovered => Style {
            text_color: palette.primary.strong.color.scale_alpha(0.8),
            ..base
        },
        Status::Disabled => disabled(base),
    }
}

/// A 'selected' text button; used for the selected 'Pane' on the view selection menu
pub fn selected_text(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();

    let base = Style {
        text_color: palette.primary.base.color,
        ..Style::default()
    };

    match status {
        Status::Active | Status::Pressed => base,
        Status::Hovered => Style {
            text_color: palette.primary.base.color.scale_alpha(0.8),
            ..base
        },
        Status::Disabled => disabled(base),
    }
}

fn disabled(style: Style) -> Style {
    Style {
        background: style
            .background
            .map(|background| background.scale_alpha(0.5)),
        text_color: style.text_color.scale_alpha(0.5),
        ..style
    }
}

pub fn invisible(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();

    let base = Style {
        text_color: palette.background.neutral.color.scale_alpha(0.000),
        background: Some(iced::Background::Color(palette.background.neutral.color.scale_alpha(0.000))),
        border: Border {
            color: palette.primary.strong.color,
            width: 0.0,
            radius: 0.0.into(),
        },
        ..Style::default()
    };

    match status {
        Status::Active | Status::Pressed => base,
        Status::Hovered => Style {
            text_color: base.text_color.scale_alpha(0.8),
            ..base
        },
        Status::Disabled => Style {
            background: base.background.map(|bg| bg.scale_alpha(0.5)),
            text_color: base.text_color.scale_alpha(0.5),
            ..base
        },
    }
}
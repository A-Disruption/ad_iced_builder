pub use iced::widget::{text_input, overlay::menu};
use iced::{border::{radius, Radius}, Border, Theme, Color, Background, Vector, Shadow};

pub fn test_input_style(theme: &Theme, status: text_input::Status) -> text_input::Style {
    let palette = theme.extended_palette();

    let base = text_input::Style {
        background: Background::Color(palette.background.base.color),
        border: Border {
            color: palette.background.strong.color,
            width: 1.0,
            radius: 4.0.into(),
        },
        icon: palette.background.weak.text,
        placeholder: palette.background.weak.text,
        value: palette.background.base.text,
        selection: palette.primary.weak.color,
    };

    match status {
        text_input::Status::Active => base,
        text_input::Status::Hovered => text_input::Style {
            border: Border {
                color: base.border.color.scale_alpha(0.8),
                ..base.border
            },
            ..base
        },
        text_input::Status::Focused { .. } => text_input::Style {
            border: Border {
                color: base.border.color,
                width: base.border.width.max(1.0),
                ..base.border
            },
            ..base
        },
        text_input::Status::Disabled => text_input::Style {
            background: base.background.scale_alpha(0.5),
            value: base.value.scale_alpha(0.5),
            placeholder: base.placeholder.scale_alpha(0.5),
            ..base
        },
    }
}

pub fn test_menu_style(theme: &Theme) -> menu::Style {
    let palette = theme.extended_palette();

    menu::Style {
        background: Background::Color(palette.background.base.color),
        border: Border {
            color: palette.background.strong.color,
            width: 1.0,
            radius: 4.0.into(),
        },
        text_color: palette.background.base.text,
        selected_text_color: palette.primary.base.text,
        selected_background: Background::Color(palette.primary.base.color),
        shadow: Shadow {
            color: palette.background.strong.color,
            offset: Vector { x: 0.0, y: 2.0 },
            blur_radius: 8.0,
        },
    }
}
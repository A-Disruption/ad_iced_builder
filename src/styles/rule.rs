pub use iced::widget::rule::*;
use iced::{border::{radius, Radius}, Border, Theme};

/// A [`Rule`] styling using the weak background color.
pub fn rule_weak(theme: &Theme) -> Style {
    let palette = theme.extended_palette();

    Style {
        color: palette.background.weak.color,
        radius: 0.0.into(),
        fill_mode: FillMode::Full,
        snap: true,
    }
}

/// A [`Rule`] styling using the strong background color.
pub fn rule_strong(theme: &Theme) -> Style {
    let palette = theme.extended_palette();

    Style {
        color: palette.background.strong.color,
        radius: 0.0.into(),
        fill_mode: FillMode::Full,
        snap: true,
    }
}
use iced::widget::{
    button, checkbox, container, overlay::menu, pick_list, progress_bar, radio, rule, slider,
    text_input, toggler,
};
use iced::{Background, Border, Color, Shadow, Theme};
use serde::{Deserialize, Serialize};

use crate::views::theme_and_stylefn_builder::ThemePaneEnum;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub enum RuleFillMode {
    #[default]
    Full,
    Percent(f32),
    Padded(u16),
    AsymmetricPadding(u16, u16),
}

/// Per-status color overrides. `None` fields fall back to the base `scale_alpha` behaviour.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StatusColorOverride {
    #[serde(default, with = "crate::persistence::serde_iced::option_color")]
    pub text_color: Option<iced::Color>,
    pub text_color_source: Option<String>,
    #[serde(default, with = "crate::persistence::serde_iced::option_color")]
    pub background_color: Option<iced::Color>,
    pub background_color_source: Option<String>,
    #[serde(default, with = "crate::persistence::serde_iced::option_color")]
    pub border_color: Option<iced::Color>,
    pub border_color_source: Option<String>,
    #[serde(default, with = "crate::persistence::serde_iced::option_color")]
    pub icon_color: Option<iced::Color>,
    pub icon_color_source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedStyleDefinition {
    pub name: String,
    pub widget_type: ThemePaneEnum,

    #[serde(with = "crate::persistence::serde_iced::color")]
    pub text_color: Color,
    pub text_color_source: Option<String>,
    #[serde(with = "crate::persistence::serde_iced::color")]
    pub background_color: Color,
    pub background_color_source: Option<String>,
    #[serde(with = "crate::persistence::serde_iced::color")]
    pub border_color: Color,
    pub border_color_source: Option<String>,
    pub border_width: f32,
    pub border_radius_top_left: f32,
    pub border_radius_top_right: f32,
    pub border_radius_bottom_right: f32,
    pub border_radius_bottom_left: f32,
    pub shadow_enabled: bool,
    #[serde(with = "crate::persistence::serde_iced::color")]
    pub shadow_color: Color,
    pub shadow_color_source: Option<String>,
    pub shadow_offset_x: f32,
    pub shadow_offset_y: f32,
    pub shadow_blur_radius: f32,
    pub snap: bool,
    pub rule_fill_mode: RuleFillMode,
    #[serde(with = "crate::persistence::serde_iced::color")]
    pub icon_color: Color,
    pub icon_color_source: Option<String>,
    #[serde(with = "crate::persistence::serde_iced::color")]
    pub placeholder_color: Color,
    pub placeholder_color_source: Option<String>,
    #[serde(with = "crate::persistence::serde_iced::color")]
    pub selection_color: Color,
    pub selection_color_source: Option<String>,
    #[serde(with = "crate::persistence::serde_iced::color")]
    pub selected_text_color: Color,
    pub selected_text_color_source: Option<String>,
    #[serde(with = "crate::persistence::serde_iced::color")]
    pub selected_background_color: Color,
    pub selected_background_color_source: Option<String>,

    // Per-status color overrides — `None` = auto scale_alpha fallback (backward-compatible).
    #[serde(default)]
    pub status_hovered: Option<StatusColorOverride>,
    #[serde(default)]
    pub status_pressed: Option<StatusColorOverride>,
    #[serde(default)]
    pub status_disabled: Option<StatusColorOverride>,
    #[serde(default)]
    pub status_focused: Option<StatusColorOverride>,
}

impl SavedStyleDefinition {
    /// Resolve an override color: source string → theme eval, then override value, then base.
    fn resolve_ov(
        ov_val: Option<Color>,
        ov_src: &Option<String>,
        base: Color,
        theme: &Theme,
    ) -> Color {
        if let Some(src) = ov_src {
            evaluate_theme_expression(theme, src).unwrap_or(base)
        } else {
            ov_val.unwrap_or(base)
        }
    }

    fn resolve_color(&self, theme: &Theme, color: Color, source: &Option<String>) -> Color {
        if let Some(source) = source {
            evaluate_theme_expression(theme, source).unwrap_or(color)
        } else {
            color
        }
    }

    fn resolved_border(&self, theme: &Theme) -> Border {
        Border {
            color: self.resolve_color(theme, self.border_color, &self.border_color_source),
            width: self.border_width,
            radius: iced::border::Radius {
                top_left: self.border_radius_top_left,
                top_right: self.border_radius_top_right,
                bottom_right: self.border_radius_bottom_right,
                bottom_left: self.border_radius_bottom_left,
            },
        }
    }

    fn resolved_shadow(&self, theme: &Theme) -> Shadow {
        if self.shadow_enabled {
            Shadow {
                color: self.resolve_color(theme, self.shadow_color, &self.shadow_color_source),
                offset: iced::Vector {
                    x: self.shadow_offset_x,
                    y: self.shadow_offset_y,
                },
                blur_radius: self.shadow_blur_radius,
            }
        } else {
            Shadow::default()
        }
    }

    fn resolved_toggle_radius(&self) -> Option<iced::border::Radius> {
        let radius = iced::border::Radius {
            top_left: self.border_radius_top_left,
            top_right: self.border_radius_top_right,
            bottom_right: self.border_radius_bottom_right,
            bottom_left: self.border_radius_bottom_left,
        };

        if self.border_radius_top_left == 0.0
            && self.border_radius_top_right == 0.0
            && self.border_radius_bottom_right == 0.0
            && self.border_radius_bottom_left == 0.0
        {
            None
        } else {
            Some(radius)
        }
    }

    fn resolved_slider_handle_radius(&self) -> f32 {
        self.border_radius_top_left
            .max(self.border_radius_top_right)
            .max(self.border_radius_bottom_right)
            .max(self.border_radius_bottom_left)
            .max(6.0)
    }

    /// Evaluate this definition against a theme to produce an actual style
    pub fn to_container_style(&self, theme: &Theme) -> container::Style {
        // Evaluate each color - use theme path if available, fallback to stored color
        let text_color = if let Some(ref source) = self.text_color_source {
            evaluate_theme_expression(theme, source).unwrap_or(self.text_color)
        } else {
            self.text_color
        };

        let background_color = if let Some(ref source) = self.background_color_source {
            evaluate_theme_expression(theme, source).unwrap_or(self.background_color)
        } else {
            self.background_color
        };

        let border_color = if let Some(ref source) = self.border_color_source {
            evaluate_theme_expression(theme, source).unwrap_or(self.border_color)
        } else {
            self.border_color
        };

        let shadow_color = if let Some(ref source) = self.shadow_color_source {
            evaluate_theme_expression(theme, source).unwrap_or(self.shadow_color)
        } else {
            self.shadow_color
        };

        container::Style {
            text_color: Some(text_color),
            background: Some(Background::Color(background_color)),
            border: Border {
                color: border_color,
                width: self.border_width,
                radius: iced::border::Radius {
                    top_left: self.border_radius_top_left,
                    top_right: self.border_radius_top_right,
                    bottom_right: self.border_radius_bottom_right,
                    bottom_left: self.border_radius_bottom_left,
                },
            },
            shadow: if self.shadow_enabled {
                Shadow {
                    color: shadow_color,
                    offset: iced::Vector {
                        x: self.shadow_offset_x,
                        y: self.shadow_offset_y,
                    },
                    blur_radius: self.shadow_blur_radius,
                }
            } else {
                Shadow::default()
            },
            snap: self.snap,
        }
    }

    pub fn to_text_input_style(
        &self,
        theme: &Theme,
        status: text_input::Status,
    ) -> text_input::Style {
        let background_color =
            self.resolve_color(theme, self.background_color, &self.background_color_source);
        let icon_color = self.resolve_color(theme, self.icon_color, &self.icon_color_source);
        let placeholder_color = self.resolve_color(
            theme,
            self.placeholder_color,
            &self.placeholder_color_source,
        );
        let text_color = self.resolve_color(theme, self.text_color, &self.text_color_source);
        let selection_color =
            self.resolve_color(theme, self.selection_color, &self.selection_color_source);

        let base = text_input::Style {
            background: Background::Color(background_color),
            border: self.resolved_border(theme),
            icon: icon_color,
            placeholder: placeholder_color,
            value: text_color,
            selection: selection_color,
        };

        match status {
            text_input::Status::Active => base,
            text_input::Status::Hovered => {
                if let Some(ref ov) = self.status_hovered {
                    text_input::Style {
                        value: ov
                            .text_color
                            .map(|c| {
                                Self::resolve_ov(Some(c), &ov.text_color_source, text_color, theme)
                            })
                            .unwrap_or(base.value),
                        icon: ov
                            .icon_color
                            .map(|c| {
                                Self::resolve_ov(Some(c), &ov.icon_color_source, icon_color, theme)
                            })
                            .unwrap_or(base.icon),
                        background: ov
                            .background_color
                            .map(|c| {
                                Background::Color(Self::resolve_ov(
                                    Some(c),
                                    &ov.background_color_source,
                                    background_color,
                                    theme,
                                ))
                            })
                            .unwrap_or(base.background),
                        border: Border {
                            color: Self::resolve_ov(
                                ov.border_color,
                                &ov.border_color_source,
                                base.border.color,
                                theme,
                            ),
                            ..base.border
                        },
                        ..base
                    }
                } else {
                    text_input::Style {
                        border: Border {
                            color: base.border.color.scale_alpha(0.8),
                            ..base.border
                        },
                        ..base
                    }
                }
            }
            text_input::Status::Focused { .. } => {
                if let Some(ref ov) = self.status_focused {
                    text_input::Style {
                        value: ov
                            .text_color
                            .map(|c| {
                                Self::resolve_ov(Some(c), &ov.text_color_source, text_color, theme)
                            })
                            .unwrap_or(base.value),
                        icon: ov
                            .icon_color
                            .map(|c| {
                                Self::resolve_ov(Some(c), &ov.icon_color_source, icon_color, theme)
                            })
                            .unwrap_or(base.icon),
                        background: ov
                            .background_color
                            .map(|c| {
                                Background::Color(Self::resolve_ov(
                                    Some(c),
                                    &ov.background_color_source,
                                    background_color,
                                    theme,
                                ))
                            })
                            .unwrap_or(base.background),
                        border: Border {
                            color: Self::resolve_ov(
                                ov.border_color,
                                &ov.border_color_source,
                                base.border.color,
                                theme,
                            ),
                            width: base.border.width.max(1.0),
                            ..base.border
                        },
                        ..base
                    }
                } else {
                    text_input::Style {
                        border: Border {
                            color: base.border.color,
                            width: base.border.width.max(1.0),
                            ..base.border
                        },
                        ..base
                    }
                }
            }
            text_input::Status::Disabled => {
                if let Some(ref ov) = self.status_disabled {
                    text_input::Style {
                        value: ov
                            .text_color
                            .map(|c| {
                                Self::resolve_ov(Some(c), &ov.text_color_source, text_color, theme)
                            })
                            .unwrap_or(base.placeholder),
                        icon: ov
                            .icon_color
                            .map(|c| {
                                Self::resolve_ov(Some(c), &ov.icon_color_source, icon_color, theme)
                            })
                            .unwrap_or(base.icon.scale_alpha(0.5)),
                        background: Background::Color(Self::resolve_ov(
                            ov.background_color,
                            &ov.background_color_source,
                            background_color.scale_alpha(0.5),
                            theme,
                        )),
                        border: Border {
                            color: Self::resolve_ov(
                                ov.border_color,
                                &ov.border_color_source,
                                base.border.color.scale_alpha(0.5),
                                theme,
                            ),
                            ..base.border
                        },
                        placeholder: base.placeholder.scale_alpha(0.5),
                        ..base
                    }
                } else {
                    text_input::Style {
                        background: Background::Color(background_color.scale_alpha(0.5)),
                        icon: base.icon.scale_alpha(0.5),
                        value: base.placeholder,
                        placeholder: base.placeholder.scale_alpha(0.5),
                        ..base
                    }
                }
            }
        }
    }

    pub fn to_menu_style(&self, theme: &Theme) -> menu::Style {
        let background_color =
            self.resolve_color(theme, self.background_color, &self.background_color_source);
        let text_color = self.resolve_color(theme, self.text_color, &self.text_color_source);
        let selected_text_color = self.resolve_color(
            theme,
            self.selected_text_color,
            &self.selected_text_color_source,
        );
        let selected_background_color = self.resolve_color(
            theme,
            self.selected_background_color,
            &self.selected_background_color_source,
        );

        menu::Style {
            background: Background::Color(background_color),
            border: self.resolved_border(theme),
            text_color,
            selected_text_color,
            selected_background: Background::Color(selected_background_color),
            shadow: self.resolved_shadow(theme),
        }
    }

    pub fn to_pick_list_style(&self, theme: &Theme, status: pick_list::Status) -> pick_list::Style {
        let text_color = self.resolve_color(theme, self.text_color, &self.text_color_source);
        let background_color =
            self.resolve_color(theme, self.background_color, &self.background_color_source);
        let placeholder_color = self.resolve_color(
            theme,
            self.placeholder_color,
            &self.placeholder_color_source,
        );
        let handle_color = self.resolve_color(theme, self.icon_color, &self.icon_color_source);

        let base = pick_list::Style {
            text_color,
            placeholder_color,
            handle_color,
            background: Background::Color(background_color),
            border: self.resolved_border(theme),
        };

        match status {
            pick_list::Status::Active => base,
            pick_list::Status::Hovered => {
                if let Some(ref ov) = self.status_hovered {
                    pick_list::Style {
                        text_color: ov
                            .text_color
                            .map(|c| {
                                Self::resolve_ov(Some(c), &ov.text_color_source, text_color, theme)
                            })
                            .unwrap_or(base.text_color),
                        handle_color: ov
                            .icon_color
                            .map(|c| {
                                Self::resolve_ov(
                                    Some(c),
                                    &ov.icon_color_source,
                                    handle_color,
                                    theme,
                                )
                            })
                            .unwrap_or(base.handle_color),
                        background: ov
                            .background_color
                            .map(|c| {
                                Background::Color(Self::resolve_ov(
                                    Some(c),
                                    &ov.background_color_source,
                                    background_color,
                                    theme,
                                ))
                            })
                            .unwrap_or(base.background),
                        border: Border {
                            color: Self::resolve_ov(
                                ov.border_color,
                                &ov.border_color_source,
                                base.border.color,
                                theme,
                            ),
                            ..base.border
                        },
                        ..base
                    }
                } else {
                    pick_list::Style {
                        border: Border {
                            color: base.border.color.scale_alpha(0.8),
                            ..base.border
                        },
                        ..base
                    }
                }
            }
            pick_list::Status::Opened { .. } => {
                if let Some(ref ov) = self.status_focused {
                    pick_list::Style {
                        text_color: ov
                            .text_color
                            .map(|c| {
                                Self::resolve_ov(Some(c), &ov.text_color_source, text_color, theme)
                            })
                            .unwrap_or(base.text_color),
                        handle_color: ov
                            .icon_color
                            .map(|c| {
                                Self::resolve_ov(
                                    Some(c),
                                    &ov.icon_color_source,
                                    handle_color,
                                    theme,
                                )
                            })
                            .unwrap_or(base.handle_color),
                        background: ov
                            .background_color
                            .map(|c| {
                                Background::Color(Self::resolve_ov(
                                    Some(c),
                                    &ov.background_color_source,
                                    background_color,
                                    theme,
                                ))
                            })
                            .unwrap_or(base.background),
                        border: Border {
                            color: Self::resolve_ov(
                                ov.border_color,
                                &ov.border_color_source,
                                base.border.color,
                                theme,
                            ),
                            width: base.border.width.max(1.0),
                            ..base.border
                        },
                        ..base
                    }
                } else {
                    pick_list::Style {
                        border: Border {
                            color: base.border.color,
                            width: base.border.width.max(1.0),
                            ..base.border
                        },
                        ..base
                    }
                }
            }
        }
    }

    pub fn to_slider_style(&self, theme: &Theme, status: slider::Status) -> slider::Style {
        let active_rail_color = self.resolve_color(theme, self.text_color, &self.text_color_source);
        let inactive_rail_color =
            self.resolve_color(theme, self.background_color, &self.background_color_source);
        let handle_color = self.resolve_color(theme, self.icon_color, &self.icon_color_source);
        let border_color = self.resolve_color(theme, self.border_color, &self.border_color_source);
        let rail_width = self.border_width.max(2.0);
        let handle_radius = self.resolved_slider_handle_radius();

        let base = slider::Style {
            rail: slider::Rail {
                backgrounds: (
                    Background::Color(active_rail_color),
                    Background::Color(inactive_rail_color),
                ),
                width: rail_width,
                border: Border {
                    color: border_color,
                    width: self.border_width,
                    radius: iced::border::Radius::from(handle_radius / 2.0),
                },
            },
            handle: slider::Handle {
                shape: slider::HandleShape::Circle {
                    radius: handle_radius,
                },
                background: Background::Color(handle_color),
                border_width: self.border_width,
                border_color,
            },
        };

        match status {
            slider::Status::Active => base,
            slider::Status::Hovered => {
                if let Some(ref ov) = self.status_hovered {
                    slider::Style {
                        rail: slider::Rail {
                            backgrounds: (
                                Background::Color(Self::resolve_ov(
                                    ov.text_color,
                                    &ov.text_color_source,
                                    active_rail_color,
                                    theme,
                                )),
                                Background::Color(Self::resolve_ov(
                                    ov.background_color,
                                    &ov.background_color_source,
                                    inactive_rail_color,
                                    theme,
                                )),
                            ),
                            border: Border {
                                color: Self::resolve_ov(
                                    ov.border_color,
                                    &ov.border_color_source,
                                    border_color,
                                    theme,
                                ),
                                ..base.rail.border
                            },
                            ..base.rail
                        },
                        handle: slider::Handle {
                            background: Background::Color(Self::resolve_ov(
                                ov.icon_color,
                                &ov.icon_color_source,
                                handle_color,
                                theme,
                            )),
                            border_color: Self::resolve_ov(
                                ov.border_color,
                                &ov.border_color_source,
                                border_color,
                                theme,
                            ),
                            ..base.handle
                        },
                    }
                } else {
                    slider::Style {
                        handle: slider::Handle {
                            background: Background::Color(handle_color.scale_alpha(0.85)),
                            ..base.handle
                        },
                        ..base
                    }
                }
            }
            slider::Status::Dragged => {
                if let Some(ref ov) = self.status_pressed {
                    slider::Style {
                        rail: slider::Rail {
                            backgrounds: (
                                Background::Color(Self::resolve_ov(
                                    ov.text_color,
                                    &ov.text_color_source,
                                    active_rail_color,
                                    theme,
                                )),
                                Background::Color(Self::resolve_ov(
                                    ov.background_color,
                                    &ov.background_color_source,
                                    inactive_rail_color,
                                    theme,
                                )),
                            ),
                            border: Border {
                                color: Self::resolve_ov(
                                    ov.border_color,
                                    &ov.border_color_source,
                                    border_color,
                                    theme,
                                ),
                                ..base.rail.border
                            },
                            ..base.rail
                        },
                        handle: slider::Handle {
                            background: Background::Color(Self::resolve_ov(
                                ov.icon_color,
                                &ov.icon_color_source,
                                handle_color,
                                theme,
                            )),
                            border_color: Self::resolve_ov(
                                ov.border_color,
                                &ov.border_color_source,
                                border_color,
                                theme,
                            ),
                            ..base.handle
                        },
                    }
                } else {
                    slider::Style {
                        rail: slider::Rail {
                            backgrounds: (
                                Background::Color(active_rail_color.scale_alpha(0.9)),
                                Background::Color(inactive_rail_color.scale_alpha(0.9)),
                            ),
                            ..base.rail
                        },
                        ..base
                    }
                }
            }
        }
    }

    pub fn to_progress_bar_style(&self, theme: &Theme) -> progress_bar::Style {
        progress_bar::Style {
            background: Background::Color(self.resolve_color(
                theme,
                self.background_color,
                &self.background_color_source,
            )),
            bar: Background::Color(self.resolve_color(
                theme,
                self.text_color,
                &self.text_color_source,
            )),
            border: self.resolved_border(theme),
        }
    }

    pub fn to_radio_style(&self, theme: &Theme, status: radio::Status) -> radio::Style {
        let text_color = self.resolve_color(theme, self.text_color, &self.text_color_source);
        let background_color =
            self.resolve_color(theme, self.background_color, &self.background_color_source);
        let dot_color = self.resolve_color(theme, self.icon_color, &self.icon_color_source);
        let border_color = self.resolve_color(theme, self.border_color, &self.border_color_source);

        let base = radio::Style {
            background: Background::Color(background_color),
            dot_color,
            border_width: self.border_width.max(1.0),
            border_color,
            text_color: Some(text_color),
        };

        match status {
            radio::Status::Active { .. } => base,
            radio::Status::Hovered { .. } => {
                if let Some(ref ov) = self.status_hovered {
                    radio::Style {
                        background: Background::Color(Self::resolve_ov(
                            ov.background_color,
                            &ov.background_color_source,
                            background_color,
                            theme,
                        )),
                        dot_color: Self::resolve_ov(
                            ov.icon_color,
                            &ov.icon_color_source,
                            dot_color,
                            theme,
                        ),
                        border_color: Self::resolve_ov(
                            ov.border_color,
                            &ov.border_color_source,
                            border_color,
                            theme,
                        ),
                        text_color: Some(Self::resolve_ov(
                            ov.text_color,
                            &ov.text_color_source,
                            text_color,
                            theme,
                        )),
                        ..base
                    }
                } else {
                    radio::Style {
                        background: Background::Color(background_color.scale_alpha(0.85)),
                        ..base
                    }
                }
            }
        }
    }

    pub fn to_toggler_style(&self, theme: &Theme, status: toggler::Status) -> toggler::Style {
        let text_color = self.resolve_color(theme, self.text_color, &self.text_color_source);
        let background_color =
            self.resolve_color(theme, self.background_color, &self.background_color_source);
        let foreground_color = self.resolve_color(theme, self.icon_color, &self.icon_color_source);
        let border_color = self.resolve_color(theme, self.border_color, &self.border_color_source);

        let base = toggler::Style {
            background: Background::Color(background_color),
            background_border_width: self.border_width,
            background_border_color: border_color,
            foreground: Background::Color(foreground_color),
            foreground_border_width: self.border_width,
            foreground_border_color: border_color,
            text_color: Some(text_color),
            border_radius: self.resolved_toggle_radius(),
            padding_ratio: 0.1,
        };

        match status {
            toggler::Status::Active { .. } => base,
            toggler::Status::Hovered { .. } => {
                if let Some(ref ov) = self.status_hovered {
                    toggler::Style {
                        background: Background::Color(Self::resolve_ov(
                            ov.background_color,
                            &ov.background_color_source,
                            background_color,
                            theme,
                        )),
                        foreground: Background::Color(Self::resolve_ov(
                            ov.icon_color,
                            &ov.icon_color_source,
                            foreground_color,
                            theme,
                        )),
                        background_border_color: Self::resolve_ov(
                            ov.border_color,
                            &ov.border_color_source,
                            border_color,
                            theme,
                        ),
                        foreground_border_color: Self::resolve_ov(
                            ov.border_color,
                            &ov.border_color_source,
                            border_color,
                            theme,
                        ),
                        text_color: Some(Self::resolve_ov(
                            ov.text_color,
                            &ov.text_color_source,
                            text_color,
                            theme,
                        )),
                        ..base
                    }
                } else {
                    toggler::Style {
                        foreground: Background::Color(foreground_color.scale_alpha(0.85)),
                        ..base
                    }
                }
            }
            toggler::Status::Disabled { .. } => {
                if let Some(ref ov) = self.status_disabled {
                    toggler::Style {
                        background: Background::Color(Self::resolve_ov(
                            ov.background_color,
                            &ov.background_color_source,
                            background_color.scale_alpha(0.5),
                            theme,
                        )),
                        foreground: Background::Color(Self::resolve_ov(
                            ov.icon_color,
                            &ov.icon_color_source,
                            foreground_color.scale_alpha(0.5),
                            theme,
                        )),
                        background_border_color: Self::resolve_ov(
                            ov.border_color,
                            &ov.border_color_source,
                            border_color.scale_alpha(0.5),
                            theme,
                        ),
                        foreground_border_color: Self::resolve_ov(
                            ov.border_color,
                            &ov.border_color_source,
                            border_color.scale_alpha(0.5),
                            theme,
                        ),
                        text_color: Some(Self::resolve_ov(
                            ov.text_color,
                            &ov.text_color_source,
                            text_color.scale_alpha(0.5),
                            theme,
                        )),
                        ..base
                    }
                } else {
                    toggler::Style {
                        background: Background::Color(background_color.scale_alpha(0.5)),
                        foreground: Background::Color(foreground_color.scale_alpha(0.5)),
                        background_border_color: border_color.scale_alpha(0.5),
                        foreground_border_color: border_color.scale_alpha(0.5),
                        text_color: Some(text_color.scale_alpha(0.5)),
                        ..base
                    }
                }
            }
        }
    }

    pub fn to_button_style(&self, theme: &Theme, status: button::Status) -> button::Style {
        // Similar implementation for button
        let text_color = if let Some(ref source) = self.text_color_source {
            evaluate_theme_expression(theme, source).unwrap_or(self.text_color)
        } else {
            self.text_color
        };

        let background_color = if let Some(ref source) = self.background_color_source {
            evaluate_theme_expression(theme, source).unwrap_or(self.background_color)
        } else {
            self.background_color
        };

        let border_color = if let Some(ref source) = self.border_color_source {
            evaluate_theme_expression(theme, source).unwrap_or(self.border_color)
        } else {
            self.border_color
        };

        let shadow_color = if let Some(ref source) = self.shadow_color_source {
            evaluate_theme_expression(theme, source).unwrap_or(self.shadow_color)
        } else {
            self.shadow_color
        };

        let base = button::Style {
            text_color,
            background: Some(Background::Color(background_color)),
            border: Border {
                color: border_color,
                width: self.border_width,
                radius: self.border_radius_top_left.into(),
            },
            shadow: if self.shadow_enabled {
                Shadow {
                    color: shadow_color,
                    offset: iced::Vector {
                        x: self.shadow_offset_x,
                        y: self.shadow_offset_y,
                    },
                    blur_radius: self.shadow_blur_radius,
                }
            } else {
                Shadow::default()
            },
            snap: self.snap,
        };

        match status {
            button::Status::Active => base,
            button::Status::Pressed => {
                if let Some(ref ov) = self.status_pressed {
                    button::Style {
                        text_color: Self::resolve_ov(
                            ov.text_color,
                            &ov.text_color_source,
                            base.text_color,
                            theme,
                        ),
                        background: ov
                            .background_color
                            .map(|c| {
                                Some(Background::Color(Self::resolve_ov(
                                    Some(c),
                                    &ov.background_color_source,
                                    c,
                                    theme,
                                )))
                            })
                            .unwrap_or(base.background),
                        border: Border {
                            color: Self::resolve_ov(
                                ov.border_color,
                                &ov.border_color_source,
                                base.border.color,
                                theme,
                            ),
                            ..base.border
                        },
                        ..base
                    }
                } else {
                    base
                }
            }
            button::Status::Hovered => {
                if let Some(ref ov) = self.status_hovered {
                    button::Style {
                        text_color: Self::resolve_ov(
                            ov.text_color,
                            &ov.text_color_source,
                            base.text_color,
                            theme,
                        ),
                        background: ov
                            .background_color
                            .map(|c| {
                                Some(Background::Color(Self::resolve_ov(
                                    Some(c),
                                    &ov.background_color_source,
                                    c,
                                    theme,
                                )))
                            })
                            .unwrap_or(base.background),
                        border: Border {
                            color: Self::resolve_ov(
                                ov.border_color,
                                &ov.border_color_source,
                                base.border.color,
                                theme,
                            ),
                            ..base.border
                        },
                        ..base
                    }
                } else {
                    button::Style {
                        text_color: base.text_color.scale_alpha(0.8),
                        ..base
                    }
                }
            }
            button::Status::Disabled => {
                if let Some(ref ov) = self.status_disabled {
                    button::Style {
                        text_color: Self::resolve_ov(
                            ov.text_color,
                            &ov.text_color_source,
                            base.text_color,
                            theme,
                        ),
                        background: ov
                            .background_color
                            .map(|c| {
                                Some(Background::Color(Self::resolve_ov(
                                    Some(c),
                                    &ov.background_color_source,
                                    c,
                                    theme,
                                )))
                            })
                            .unwrap_or(base.background),
                        border: Border {
                            color: Self::resolve_ov(
                                ov.border_color,
                                &ov.border_color_source,
                                base.border.color,
                                theme,
                            ),
                            ..base.border
                        },
                        ..base
                    }
                } else {
                    button::Style {
                        background: base.background.map(|bg| bg.scale_alpha(0.5)),
                        text_color: base.text_color.scale_alpha(0.5),
                        ..base
                    }
                }
            }
        }
    }

    pub fn to_checkbox_style(&self, theme: &Theme, status: checkbox::Status) -> checkbox::Style {
        let text_color = if let Some(ref source) = self.text_color_source {
            evaluate_theme_expression(theme, source).unwrap_or(self.text_color)
        } else {
            self.text_color
        };

        let background_color = if let Some(ref source) = self.background_color_source {
            evaluate_theme_expression(theme, source).unwrap_or(self.background_color)
        } else {
            self.background_color
        };

        let border_color = if let Some(ref source) = self.border_color_source {
            evaluate_theme_expression(theme, source).unwrap_or(self.border_color)
        } else {
            self.border_color
        };

        let icon_color = if let Some(ref source) = self.icon_color_source {
            evaluate_theme_expression(theme, source).unwrap_or(self.icon_color)
        } else {
            self.icon_color
        };

        let base = checkbox::Style {
            background: Background::Color(background_color),
            icon_color,
            border: Border {
                color: border_color,
                width: self.border_width,
                radius: iced::border::Radius {
                    top_left: self.border_radius_top_left,
                    top_right: self.border_radius_top_right,
                    bottom_right: self.border_radius_bottom_right,
                    bottom_left: self.border_radius_bottom_left,
                },
            },
            text_color: Some(text_color),
        };

        match status {
            checkbox::Status::Active { .. } => base,
            checkbox::Status::Hovered { .. } => {
                if let Some(ref ov) = self.status_hovered {
                    checkbox::Style {
                        text_color: Some(Self::resolve_ov(
                            ov.text_color,
                            &ov.text_color_source,
                            base.text_color.unwrap_or(Color::BLACK),
                            theme,
                        )),
                        icon_color: ov
                            .icon_color
                            .map(|c| Self::resolve_ov(Some(c), &ov.icon_color_source, c, theme))
                            .unwrap_or(base.icon_color),
                        ..base
                    }
                } else {
                    checkbox::Style {
                        text_color: base.text_color.map(|c| c.scale_alpha(0.8)),
                        ..base
                    }
                }
            }
            checkbox::Status::Disabled { .. } => {
                if let Some(ref ov) = self.status_disabled {
                    checkbox::Style {
                        text_color: Some(Self::resolve_ov(
                            ov.text_color,
                            &ov.text_color_source,
                            base.text_color.unwrap_or(Color::BLACK),
                            theme,
                        )),
                        icon_color: ov
                            .icon_color
                            .map(|c| Self::resolve_ov(Some(c), &ov.icon_color_source, c, theme))
                            .unwrap_or(base.icon_color),
                        ..base
                    }
                } else {
                    checkbox::Style {
                        icon_color: base.icon_color.scale_alpha(0.5),
                        text_color: base.text_color.map(|c| c.scale_alpha(0.5)),
                        ..base
                    }
                }
            }
        }
    }

    pub fn to_combo_box_input_style(
        &self,
        theme: &Theme,
        status: text_input::Status,
    ) -> text_input::Style {
        self.to_text_input_style(theme, status)
    }

    pub fn to_rule_style(&self, theme: &Theme) -> rule::Style {
        let color = if let Some(ref source) = self.border_color_source {
            evaluate_theme_expression(theme, source).unwrap_or(self.border_color)
        } else {
            self.border_color
        };

        let fill_mode = match &self.rule_fill_mode {
            RuleFillMode::Full => rule::FillMode::Full,
            RuleFillMode::Percent(p) => rule::FillMode::Percent(*p),
            RuleFillMode::Padded(p) => rule::FillMode::Padded(*p),
            RuleFillMode::AsymmetricPadding(a, b) => rule::FillMode::AsymmetricPadding(*a, *b),
        };

        rule::Style {
            color,
            radius: iced::border::Radius {
                top_left: self.border_radius_top_left,
                top_right: self.border_radius_top_right,
                bottom_right: self.border_radius_bottom_right,
                bottom_left: self.border_radius_bottom_left,
            },
            fill_mode,
            snap: self.snap,
        }
    }

    pub fn to_combo_box_menu_style(&self, theme: &Theme) -> menu::Style {
        self.to_menu_style(theme)
    }
}

/// Evaluates theme path expressions like "theme.extended_palette().primary.strong.color"
pub fn evaluate_theme_expression(theme: &Theme, expression: &str) -> Option<Color> {
    let palette = theme.extended_palette();

    // Parse the expression and navigate the theme structure
    let expression = expression.replace("theme.extended_palette().", "");
    let parts: Vec<&str> = expression.split('.').collect();

    if parts.is_empty() {
        return None;
    }

    // Navigate the palette structure based on the path
    let mut color = None;
    let mut alpha_scale = 1.0;

    // First part is the color family: primary, secondary, success, warning, danger, background
    match parts.get(0) {
        Some(&"primary") => {
            color = match parts.get(1) {
                Some(&"base") => match parts.get(2) {
                    Some(&"color") => Some(palette.primary.base.color),
                    Some(&"text") => Some(palette.primary.base.text),
                    _ => None,
                },
                Some(&"weak") => match parts.get(2) {
                    Some(&"color") => Some(palette.primary.weak.color),
                    Some(&"text") => Some(palette.primary.weak.text),
                    _ => None,
                },
                Some(&"strong") => match parts.get(2) {
                    Some(&"color") => Some(palette.primary.strong.color),
                    Some(&"text") => Some(palette.primary.strong.text),
                    _ => None,
                },
                _ => None,
            };
        }
        Some(&"secondary") => {
            color = match parts.get(1) {
                Some(&"base") => match parts.get(2) {
                    Some(&"color") => Some(palette.secondary.base.color),
                    Some(&"text") => Some(palette.secondary.base.text),
                    _ => None,
                },
                Some(&"weak") => match parts.get(2) {
                    Some(&"color") => Some(palette.secondary.weak.color),
                    Some(&"text") => Some(palette.secondary.weak.text),
                    _ => None,
                },
                Some(&"strong") => match parts.get(2) {
                    Some(&"color") => Some(palette.secondary.strong.color),
                    Some(&"text") => Some(palette.secondary.strong.text),
                    _ => None,
                },
                _ => None,
            };
        }
        Some(&"success") => {
            color = match parts.get(1) {
                Some(&"base") => match parts.get(2) {
                    Some(&"color") => Some(palette.success.base.color),
                    Some(&"text") => Some(palette.success.base.text),
                    _ => None,
                },
                Some(&"weak") => match parts.get(2) {
                    Some(&"color") => Some(palette.success.weak.color),
                    Some(&"text") => Some(palette.success.weak.text),
                    _ => None,
                },
                Some(&"strong") => match parts.get(2) {
                    Some(&"color") => Some(palette.success.strong.color),
                    Some(&"text") => Some(palette.success.strong.text),
                    _ => None,
                },
                _ => None,
            };
        }
        Some(&"warning") => {
            color = match parts.get(1) {
                Some(&"base") => match parts.get(2) {
                    Some(&"color") => Some(palette.warning.base.color),
                    Some(&"text") => Some(palette.warning.base.text),
                    _ => None,
                },
                Some(&"weak") => match parts.get(2) {
                    Some(&"color") => Some(palette.warning.weak.color),
                    Some(&"text") => Some(palette.warning.weak.text),
                    _ => None,
                },
                Some(&"strong") => match parts.get(2) {
                    Some(&"color") => Some(palette.warning.strong.color),
                    Some(&"text") => Some(palette.warning.strong.text),
                    _ => None,
                },
                _ => None,
            };
        }
        Some(&"danger") => {
            color = match parts.get(1) {
                Some(&"base") => match parts.get(2) {
                    Some(&"color") => Some(palette.danger.base.color),
                    Some(&"text") => Some(palette.danger.base.text),
                    _ => None,
                },
                Some(&"weak") => match parts.get(2) {
                    Some(&"color") => Some(palette.danger.weak.color),
                    Some(&"text") => Some(palette.danger.weak.text),
                    _ => None,
                },
                Some(&"strong") => match parts.get(2) {
                    Some(&"color") => Some(palette.danger.strong.color),
                    Some(&"text") => Some(palette.danger.strong.text),
                    _ => None,
                },
                _ => None,
            };
        }
        Some(&"background") => {
            color = match parts.get(1) {
                Some(&"base") => match parts.get(2) {
                    Some(&"color") => Some(palette.background.base.color),
                    Some(&"text") => Some(palette.background.base.text),
                    _ => None,
                },
                Some(&"weak") => match parts.get(2) {
                    Some(&"color") => Some(palette.background.weak.color),
                    Some(&"text") => Some(palette.background.weak.text),
                    _ => None,
                },
                Some(&"strong") => match parts.get(2) {
                    Some(&"color") => Some(palette.background.strong.color),
                    Some(&"text") => Some(palette.background.strong.text),
                    _ => None,
                },
                Some(&"weaker") => match parts.get(2) {
                    Some(&"color") => Some(palette.background.weaker.color),
                    Some(&"text") => Some(palette.background.weaker.text),
                    _ => None,
                },
                Some(&"weakest") => match parts.get(2) {
                    Some(&"color") => Some(palette.background.weakest.color),
                    Some(&"text") => Some(palette.background.weakest.text),
                    _ => None,
                },
                Some(&"stronger") => match parts.get(2) {
                    Some(&"color") => Some(palette.background.stronger.color),
                    Some(&"text") => Some(palette.background.stronger.text),
                    _ => None,
                },
                Some(&"strongest") => match parts.get(2) {
                    Some(&"color") => Some(palette.background.strongest.color),
                    Some(&"text") => Some(palette.background.strongest.text),
                    _ => None,
                },
                Some(&"neutral") => match parts.get(2) {
                    Some(&"color") => Some(palette.background.neutral.color),
                    Some(&"text") => Some(palette.background.neutral.text),
                    _ => None,
                },
                _ => None,
            };
        }
        _ => {}
    }

    if parts.len() == 5 {
        let part4 = parts.get(3).unwrap().strip_prefix("scale_alpha(").unwrap();
        let part5 = parts.get(4).unwrap().strip_suffix(")").unwrap();
        let alpha = part4.to_string() + "." + part5;
        if let Ok(value) = alpha.parse::<f32>() {
            alpha_scale = value;
        }
    }

    // Apply alpha scaling if present
    color.map(|c| {
        if (alpha_scale - 1.0).abs() > 0.001 {
            let color = Color {
                a: c.a * alpha_scale,
                ..c
            };
            println!("Color: {}", color);
            color
        } else {
            c
        }
    })
}

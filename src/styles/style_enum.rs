use iced::widget::{button, container};
use iced::{Background, Border, Color, Shadow, Theme};

use crate::views::theme_and_stylefn_builder::ThemePaneEnum;

#[derive(Debug, Clone, Copy)]
pub enum WidgetStyle {
    Container(container::Style),
    Button(button::Style),
}

#[derive(Debug, Clone)]
pub struct SavedStyleDefinition  {
    pub name: String,
    pub widget_type: ThemePaneEnum,
    
    pub text_color: Color,
    pub text_color_source: Option<String>,
    pub background_color: Color,
    pub background_color_source: Option<String>,
    pub border_color: Color,
    pub border_color_source: Option<String>,
    pub border_width: f32,
    pub border_radius_top_left: f32,
    pub border_radius_top_right: f32,
    pub border_radius_bottom_right: f32,
    pub border_radius_bottom_left: f32,
    pub shadow_enabled: bool,
    pub shadow_color: Color,
    pub shadow_color_source: Option<String>,
    pub shadow_offset_x: f32,
    pub shadow_offset_y: f32,
    pub shadow_blur_radius: f32,
    pub snap: bool,
}

impl SavedStyleDefinition {
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
    
    pub fn to_button_style(&self, theme: &Theme) -> button::Style {
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
        
        button::Style {
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
        }
    }
}

/// Evaluates theme path expressions like "theme.extended_palette().primary.strong.color"
pub fn evaluate_theme_expression(theme: &Theme, expression: &str) -> Option<Color> {
    let palette = theme.extended_palette();
    
    // Parse the expression and navigate the theme structure
    let expression = expression.replace("theme.extended_palette().",  "");
    let parts: Vec<&str> = expression
        .split('.')
        .collect();
    
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
            let color = Color { a: c.a * alpha_scale, ..c };
            println!("Color: {}", color);
            color
        } else {
            c
        }
    })
}
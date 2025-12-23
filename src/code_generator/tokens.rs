use iced::{Color, Theme};

/// Token types for syntax highlighting
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    Keyword,      // let, fn, impl, use, pub, enum, match, move
    Type,         // Element, Message, Length, Color, etc.
    Function,     // function names
    String,       // string literals
    Number,       // numeric literals
    Comment,      // comments
    Operator,     // =, ->, ::, .
    Identifier,   // variable names
    Macro,        // column!, row!, etc.
    Plain,        // everything else
}

impl TokenType {
    pub fn color(&self) -> Color {
        match self {
            TokenType::Keyword => Color::from_rgb8(86, 156, 214),     // Blue
            TokenType::Type => Color::from_rgb8(78, 201, 176),        // Teal
            TokenType::Function => Color::from_rgb8(220, 220, 170),   // Light yellow
            TokenType::String => Color::from_rgb8(206, 145, 120),     // Orange
            TokenType::Number => Color::from_rgb8(181, 206, 168),     // Light green
            TokenType::Comment => Color::from_rgb8(106, 153, 85),     // Green
            TokenType::Operator => Color::from_rgb8(212, 212, 212),   // Light gray
            TokenType::Identifier => Color::from_rgb8(156, 220, 254), // Light blue
            TokenType::Macro => Color::from_rgb8(197, 134, 192),      // Purple
            TokenType::Plain => Color::from_rgb8(212, 212, 212),      // Light gray
        }
    }
}

impl TokenType {
    pub fn color_for_theme(&self, theme: &Theme) -> Color {
        let palette = theme.extended_palette();

        match theme {
            Theme::Light => match self {
                TokenType::Keyword => Color::from_rgb8(0, 0, 255),         // Blue (like VSCode)
                TokenType::Type => Color::from_rgb8(0, 128, 128),          // Teal
                TokenType::Function => Color::from_rgb8(121, 94, 38),      // Brown/yellow
                TokenType::String => Color::from_rgb8(163, 21, 21),        // Dark red
                TokenType::Number => Color::from_rgb8(9, 134, 88),         // Green
                TokenType::Comment => Color::from_rgb8(0, 128, 0),         // Green
                TokenType::Operator => Color::from_rgb8(0, 0, 0),          // Black
                TokenType::Identifier => Color::from_rgb8(0, 16, 128),     // Dark blue
                TokenType::Macro => Color::from_rgb8(175, 0, 219),         // Purple
                TokenType::Plain => Color::from_rgb8(0, 0, 0),             // Black
            },
            Theme::Dark => match self {
                TokenType::Keyword => Color::from_rgb8(86, 156, 214),      // Blue
                TokenType::Type => Color::from_rgb8(78, 201, 176),        // Teal/cyan
                TokenType::Function => Color::from_rgb8(220, 220, 170),    // Light yellow
                TokenType::String => Color::from_rgb8(206, 145, 120),      // Orange/salmon
                TokenType::Number => Color::from_rgb8(181, 206, 168),      // Light green
                TokenType::Comment => Color::from_rgb8(106, 153, 85),      // Green
                TokenType::Operator => Color::from_rgb8(212, 212, 212),    // Light gray
                TokenType::Identifier => Color::from_rgb8(156, 220, 254),  // Light blue
                TokenType::Macro => Color::from_rgb8(197, 134, 192),       // Purple
                TokenType::Plain => Color::from_rgb8(212, 212, 212),       // Light gray
            },
            _ => {
                // Default/custom theme colors
                match self {
                    TokenType::Keyword => palette.danger.base.color,
                    TokenType::Type => palette.primary.strong.color,
                    TokenType::Function => palette.warning.weak.color,
                    TokenType::String => palette.warning.base.color,
                    TokenType::Number => palette.primary.weak.color,
                    TokenType::Comment => palette.success.weak.color,
                    TokenType::Operator => palette.danger.weak.color,
                    TokenType::Identifier => palette.primary.base.color,
                    TokenType::Macro => palette.success.base.color,
                    TokenType::Plain => palette.secondary.base.color,
                }
            }
        }
    }
}

/// A highlighted token in the code
#[derive(Debug, Clone)]
pub struct Token {
    pub text: String,
    pub token_type: TokenType,
}

/// Helper struct for building token streams with proper syntax highlighting
pub struct TokenBuilder {
    tokens: Vec<Token>,
    indent_level: usize,
}

impl TokenBuilder {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            indent_level: 0,
        }
    }

    pub fn into_tokens(self) -> Vec<Token> {
        self.tokens
    }

    pub fn set_indent(&mut self, level: usize) {
        self.indent_level = level;
    }

    pub fn increase_indent(&mut self) {
        self.indent_level += 1;
    }

    pub fn decrease_indent(&mut self) {
        if self.indent_level > 0 {
            self.indent_level -= 1;
        }
    }

    pub fn add_keyword(&mut self, text: &str) {
        self.tokens.push(Token {
            text: text.to_string(),
            token_type: TokenType::Keyword,
        });
    }

    pub fn add_type(&mut self, text: &str) {
        self.tokens.push(Token {
            text: text.to_string(),
            token_type: TokenType::Type,
        });
    }

    pub fn add_function(&mut self, text: &str) {
        self.tokens.push(Token {
            text: text.to_string(),
            token_type: TokenType::Function,
        });
    }

    pub fn add_number(&mut self, text: &str) {
        self.tokens.push(Token {
            text: text.to_string(),
            token_type: TokenType::Number,
        });
    }

    pub fn add_plain(&mut self, text: &str) {
        self.tokens.push(Token {
            text: text.to_string(),
            token_type: TokenType::Plain,
        });
    }

    pub fn add_operator(&mut self, text: &str) {
        self.tokens.push(Token {
            text: text.to_string(),
            token_type: TokenType::Operator,
        });
    }

    pub fn add_identifier(&mut self, text: &str) {
        self.tokens.push(Token {
            text: text.to_string(),
            token_type: TokenType::Identifier,
        });
    }

    pub fn add_string(&mut self, text: &str) {
        self.tokens.push(Token {
            text: text.to_string(),
            token_type: TokenType::String,
        });
    }

    pub fn add_comment(&mut self, text: &str) {
        self.tokens.push(Token {
            text: text.to_string(),
            token_type: TokenType::Comment,
        });
    }

    pub fn add_macro(&mut self, text: &str) {
        self.tokens.push(Token {
            text: text.to_string(),
            token_type: TokenType::Macro,
        });
    }

    pub fn add_newline(&mut self) {
        self.tokens.push(Token {
            text: "\n".to_string(),
            token_type: TokenType::Plain,
        });
    }

    pub fn add_indent(&mut self) {
        self.tokens.push(Token {
            text: "    ".repeat(self.indent_level),
            token_type: TokenType::Plain,
        });
    }

    pub fn add_space(&mut self) {
        self.tokens.push(Token {
            text: " ".to_string(),
            token_type: TokenType::Plain,
        });
    }

    // Helper methods for common patterns
    pub fn add_color(&mut self, color: Color) {
        self.add_type("Color");
        self.add_operator("::");
        self.add_function("from_rgba");
        self.add_plain("(");
        self.add_number(&format!("{:.1}", color.r));
        self.add_plain(", ");
        self.add_number(&format!("{:.1}", color.g));
        self.add_plain(", ");
        self.add_number(&format!("{:.1}", color.b));
        self.add_plain(", ");
        self.add_number(&format!("{:.1}", color.a));
        self.add_plain(")");
    }

    pub fn add_field(&mut self, name: &str, value_fn: impl FnOnce(&mut Self)) {
        self.add_indent();
        self.add_plain(name);
        self.add_operator(":");
        self.add_space();
        value_fn(self);
        self.add_plain(",");
        self.add_newline();
    }

    pub fn add_struct(&mut self, name: &str, fields_fn: impl FnOnce(&mut Self)) {
        self.add_type(name);
        self.add_space();
        self.add_plain("{");
        self.add_newline();
        self.increase_indent();
        fields_fn(self);
        self.decrease_indent();
        self.add_indent();
        self.add_plain("}");
    }
}
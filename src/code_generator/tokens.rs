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
    pub fn color_for_theme(&self, theme: &Theme) -> Color {
        let palette = theme.extended_palette();

        match self {
            TokenType::Keyword =>       palette.danger.base.color,
            TokenType::Type =>          palette.primary.strong.color,
            TokenType::Function =>      palette.warning.strong.color,
            TokenType::String =>        palette.warning.base.color,
            TokenType::Number =>        palette.success.strong.color,
            TokenType::Comment =>       palette.success.weak.color,
            TokenType::Operator =>      palette.danger.weak.color,
            TokenType::Identifier =>    palette.primary.base.color,
            TokenType::Macro =>         palette.success.base.color,
            TokenType::Plain =>         palette.secondary.strong.color,
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

    /// Add a color value, using theme palette path if available, otherwise Color::from_rgba
    pub fn add_color_with_source(&mut self, color: Color, source: &Option<String>,) {
        if let Some(theme_path) = source {
            // Parse the theme path and add it with proper syntax highlighting
            // Example: "palette.primary.strong.color"
            self.add_theme_path(theme_path);
        } else {
            // Fall back to Color::from_rgba for custom colors
            self.add_color(color);
        }
    }

    pub fn add_extended_palette_var(&mut self) {
        self.add_indent();
        self.add_keyword("let ");
        self.add_identifier("palette ");
        self.add_operator("= ");
        self.add_identifier("theme");
        self.add_operator(".");
        self.add_function("extended_palette");
        self.add_plain("();");
    }

    /// Parse and add theme path
    fn add_theme_path(&mut self, path: &str) {
        let parts: Vec<&str> = path.split('.').collect();
        
        for (i, part) in parts.iter().enumerate() {
            if i > 1 {
                 println!("part: {} {}", i, part);
                 self.add_operator(".");
            }
           
            
            if part.contains("()") {
                // This is a function call like "extended_palette()"
                // not writing this because we are using palette instead of theme.extended_palette()
            } else if part.starts_with("scale_alpha(") {
                // Handle scale_alpha(value) specially
                self.add_function("scale_alpha");
                self.add_plain("(");
                let value = part.trim_start_matches("scale_alpha(").trim_end_matches(")");
                self.add_number(value);
            } else if *part == "theme" {
                // "theme" is typically an identifier/parameter
                self.add_identifier("palette");
            } else if part.starts_with(|c| char::is_numeric(c)) {
                let value = part.trim_end_matches(")");
                self.add_number(value);
                self.add_plain(")");
            }
            else {
                // Properties like "primary", "strong", "color", "text"
                self.add_identifier(part);
            }
        }
        println!("{}", path);
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
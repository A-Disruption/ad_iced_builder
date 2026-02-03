use iced::{Color, Length, Padding, Alignment};
use iced::widget::text::LineHeight;
use super::tokens::{Token, TokenBuilder, TokenType};
use crate::data_structures::types::type_implementations::{ContentFitChoice, FontType, MouseInteraction, PaddingMode};

/// Handles the mechanical act of writing formatted code
pub struct CodeWriter {
    builder: TokenBuilder,
}

impl CodeWriter {
    pub fn new() -> Self {
        Self { builder: TokenBuilder::new() }
    }

    pub fn tokens(self) -> Vec<Token> {
        self.builder.into_tokens()
    }

    // Token helper methods
    pub fn add_keyword(&mut self, text: &str) {
        self.builder.add_keyword(text);
    }

    pub fn add_type(&mut self, text: &str) {
        self.builder.add_type(text);
    }

    pub fn add_function(&mut self, text: &str) {
        self.builder.add_function(text);
    }

    pub fn add_string(&mut self, text: &str) {
        self.builder.add_string(text);
    }

    pub fn add_number(&mut self, text: &str) {
        self.builder.add_number(text);
    }

    pub fn add_comment(&mut self, text: &str) {
        self.builder.add_comment(text);
    }

    pub fn add_operator(&mut self, text: &str) {
        self.builder.add_operator(text);
    }

    pub fn add_identifier(&mut self, text: &str) {
        self.builder.add_identifier(text);
    }

    pub fn add_macro(&mut self, text: &str) {
        self.builder.add_macro(text);
    }

    pub fn add_plain(&mut self, text: &str) {
        self.builder.add_plain(text);
    }

    pub fn add_newline(&mut self) {
        self.builder.add_newline();
    }

    pub fn add_indent(&mut self) {
        self.builder.add_indent();
    }

    pub fn increase_indent(&mut self) {
        self.builder.increase_indent();
    }

    pub fn decrease_indent(&mut self) {
        self.builder.decrease_indent();
    }

    pub fn add_color(&mut self, color: Color) {
        self.add_type("Color");
        self.add_operator("::");
        self.add_function("from_rgba");
        self.add_plain("(");
        self.add_number(&format!("{:.3}", color.r));
        self.add_plain(", ");
        self.add_number(&format!("{:.3}", color.g));
        self.add_plain(", ");
        self.add_number(&format!("{:.3}", color.b));
        self.add_plain(", ");
        self.add_number(&format!("{:.3}", color.a));
        self.add_plain(")");
    }

    pub fn generate_padding(&mut self, padding: &Padding, padding_mode: PaddingMode) {
        self.add_newline();
        self.add_indent();
        self.add_operator(".");
        self.add_function("padding");
        self.add_plain("(");
        
        match padding_mode {
            PaddingMode::Uniform => {
                // .padding(10.0)
                self.add_number(&format!("{:.1}", padding.top));
            }
            PaddingMode::Symmetric => {
                // .padding([vertical, horizontal])
                self.add_plain("[");
                self.add_number(&format!("{:.1}", padding.top));
                self.add_plain(", ");
                self.add_number(&format!("{:.1}", padding.left));
                self.add_plain("]");
            }
            PaddingMode::Individual => {
                // .padding(Padding { top, right, bottom, left })
                self.add_type(" Padding");
                self.add_plain(" { ");
                self.add_newline();
                self.increase_indent();
                self.add_indent();
                self.add_plain("top: ");
                self.add_number(&format!("{:.1}", padding.top));
                self.add_plain(", ");
                self.add_plain("right: ");
                self.add_number(&format!("{:.1}", padding.right));
                self.add_plain(", ");
                self.add_newline();
                self.add_indent();
                self.add_plain("bottom: ");
                self.add_number(&format!("{:.1}", padding.bottom));
                self.add_plain(", ");
                self.add_plain("left: ");
                self.add_number(&format!("{:.1}", padding.left));
                self.add_newline();
                self.decrease_indent();
                self.add_indent();
                self.add_plain(" }");
            }
        }
        
        self.add_plain(")");
    }

    pub fn add_id(&mut self, id: &String) {
        self.add_newline();
        self.add_indent();
        self.add_operator(".");
        self.add_function("id");
        self.add_plain("(");
        self.add_string(&format!("\"{}\"", id));
        self.add_plain(")");        
    }

    pub fn add_width(&mut self, length: Length) {
            self.add_newline();
            self.add_indent();
            self.add_operator(".");
            self.add_function("width");
            self.add_plain("(");
            self.add_length(length);
            self.add_plain(")");
    }

    pub fn add_max_width(&mut self, max_width: f32) {
        self.add_newline();
        self.add_indent();
        self.add_operator(".");
        self.add_function("max_width");
        self.add_plain("(");
        self.add_number(&format!("{:.1}", max_width));
        self.add_plain(")");
    }

    pub fn add_height(&mut self, length: Length) {
            self.add_newline();
            self.add_indent();
            self.add_operator(".");
            self.add_function("height");
            self.add_plain("(");
            self.add_length(length);
            self.add_plain(")");
    }

    pub fn add_max_height(&mut self, max_height: f32) {
        self.add_newline();
        self.add_indent();
        self.add_operator(".");
        self.add_function("max_height");
        self.add_plain("(");
        self.add_number(&format!("{:.1}", max_height));
        self.add_plain(")");
    }

    pub fn add_length(&mut self, length: Length) {
        match length {
            Length::Fill => {
                self.add_type("Length");
                self.add_operator("::");
                self.add_plain("Fill");
            }
            Length::Shrink => {
                self.add_type("Length");
                self.add_operator("::");
                self.add_plain("Shrink");
            }
            Length::Fixed(px) => {
                self.add_type("Length");
                self.add_operator("::");
                self.add_plain("Fixed");
                self.add_plain("(");
                self.add_number(&format!("{:.1}", px));
                self.add_plain(")");
            }
            Length::FillPortion(p) => {
                self.add_type("Length");
                self.add_operator("::");
                self.add_plain("FillPortion");
                self.add_plain("(");
                self.add_number(&format!("{}", p));
                self.add_plain(")");
            }
            _ => {
                self.add_type("Length");
                self.add_operator("::");
                self.add_plain("Shrink");
            }
        }
    }

    pub fn add_align_y(&mut self, alignment: Alignment) {
        self.add_newline();
        self.add_indent();
        self.add_operator(".");
        self.add_function("align_y");
        self.add_plain("(");
        self.add_alignment(alignment);
        self.add_plain(")");
    }

    pub fn add_align_x(&mut self, alignment: Alignment) {
        self.add_newline();
        self.add_indent();
        self.add_operator(".");
        self.add_function("align_x");
        self.add_plain("(");
        self.add_alignment(alignment);
        self.add_plain(")");        
    }

    pub fn add_center_x(&mut self, length: Length) {
        self.add_newline();
        self.add_indent();
        self.add_operator(".");
        self.add_function("center_x");
        self.add_plain("(");
        self.add_length(length);
        self.add_plain(")");              
    }

    pub fn add_center_y(&mut self, length: Length) {
        self.add_newline();
        self.add_indent();
        self.add_operator(".");
        self.add_function("center_y");
        self.add_plain("(");
        self.add_length(length);
        self.add_plain(")");              
    }

    pub fn add_center(&mut self, length: Length) {
        self.add_newline();
        self.add_indent();
        self.add_operator(".");
        self.add_function("center");
        self.add_plain("(");
        self.add_length(length);
        self.add_plain(")");              
    }

    fn add_alignment(&mut self, alignment: Alignment) {
        match alignment {
            Alignment::Start => self.add_type("Alignment::Start"),
            Alignment::Center => self.add_type("Alignment::Center"),
            Alignment::End => self.add_type("Alignment::End"),            
        }
    }

    pub fn add_clip(&mut self) {
        self.add_newline();
        self.add_indent();
        self.add_operator(".");
        self.add_function("clip");
        self.add_plain("(");
        self.add_keyword("true");
        self.add_plain(")");        
    }

    pub fn add_placeholder(&mut self, placeholder: &str) {
        self.add_newline();
        self.add_indent();
        self.add_operator(".");
        self.add_function("placeholder");
        self.add_plain("(");
        self.add_string(&format!("\"{}\"", placeholder));
        self.add_plain(")");        
    }

    pub fn add_font(&mut self, font_type: FontType) {
        self.add_newline();
        self.add_indent();
        self.add_operator(".");
        self.add_function("font");
        self.add_plain("(");
        match font_type {
            FontType::Monospace => {
                self.add_type("Font");
                self.add_operator("::");
                self.add_plain("MONOSPACE");
            }
            _ => {
                self.add_type("Font");
                self.add_operator("::");
                self.add_plain("default()");
            }
        }
        self.add_plain(")");        
    }

    pub fn add_lineheight(&mut self, lineheight: &LineHeight) {
        self.add_newline();
        self.add_indent();
        self.add_operator(".");
        self.add_function("line_height");
        self.add_plain("(");
        // Generate line_height value based on type
        match lineheight {
            LineHeight::Absolute(pixels) => {
                self.add_plain(&format!("{}", pixels.0));
            }
            LineHeight::Relative(factor) => {
                self.add_plain("text::LineHeight::Relative(");
                self.add_plain(&format!("{}", factor));
                self.add_plain(")");
            }
        }
        self.add_plain(")");
    }

    pub fn on_press(&mut self, name: &str) {
        self.add_newline();
        self.add_indent();
        self.add_operator(".");
        self.add_function("on_press");
        self.add_plain("(");
        self.add_type("Message");
        self.add_operator("::");
        self.add_plain(&format!("{}Pressed", to_pascal_case(name)));
        self.add_plain(")");
    }

    pub fn on_press_with(&mut self, name: &str) {
        self.add_newline();
        self.add_indent();
        self.add_operator(".");
        self.add_function("on_press_with");
        self.add_plain("(|| ");
        self.add_type("Message");
        self.add_operator("::");
        self.add_plain(&format!("{}Pressed", to_pascal_case(name)));
        self.add_plain(")");
    }

    pub fn on_press_maybe(&mut self, name: &str) {
        self.add_newline();
        self.add_indent();
        self.add_operator(".");
        self.add_function("on_press_maybe");
        self.add_plain("(Some(");
        self.add_type("Message");
        self.add_operator("::");
        self.add_plain(&format!("{}Pressed", to_pascal_case(name)));
        self.add_plain("))");
    }

    pub fn on_release(&mut self, name: &str) {
        self.add_newline();
        self.add_indent();
        self.add_operator(".");
        self.add_function("on_release");
        self.add_plain("(");
        self.add_type("Message");
        self.add_operator("::");
        self.add_plain(&format!("{}Released", to_pascal_case(name)));
        self.add_plain(")");
    }

    pub fn on_double_click(&mut self, name: &str) {
        self.add_newline();
        self.add_indent();
        self.add_operator(".");
        self.add_function("on_double_click");
        self.add_plain("(");
        self.add_type("Message");
        self.add_operator("::");
        self.add_plain(&format!("{}DoubleClicked", to_pascal_case(name)));
        self.add_plain(")");
    }

    pub fn on_right_press(&mut self, name: &str) {
        self.add_newline();
        self.add_indent();
        self.add_operator(".");
        self.add_function("on_right_press");
        self.add_plain("(");
        self.add_type("Message");
        self.add_operator("::");
        self.add_plain(&format!("{}RightPressed", to_pascal_case(name)));
        self.add_plain(")");
    }

    pub fn on_right_release(&mut self, name: &str) {
        self.add_newline();
        self.add_indent();
        self.add_operator(".");
        self.add_function("on_right_release");
        self.add_plain("(");
        self.add_type("Message");
        self.add_operator("::");
        self.add_plain(&format!("{}RightReleased", to_pascal_case(name)));
        self.add_plain(")");
    }

    pub fn on_middle_press(&mut self, name: &str) {
        self.add_newline();
        self.add_indent();
        self.add_operator(".");
        self.add_function("on_middle_press");
        self.add_plain("(");
        self.add_type("Message");
        self.add_operator("::");
        self.add_plain(&format!("{}MiddlePressed", to_pascal_case(name)));
        self.add_plain(")");
    }

    pub fn on_middle_release(&mut self, name: &str) {
        self.add_newline();
        self.add_indent();
        self.add_operator(".");
        self.add_function("on_middle_release");
        self.add_plain("(");
        self.add_type("Message");
        self.add_operator("::");
        self.add_plain(&format!("{}MiddleReleased", to_pascal_case(name)));
        self.add_plain(")");
    }

    pub fn on_scroll(&mut self, name: &str) {
        self.add_newline();
        self.add_indent();
        self.add_operator(".");
        self.add_function("on_scroll");
        self.add_plain("(|delta| ");
        self.add_type("Message");
        self.add_operator("::");
        self.add_plain(&format!("{}Scrolled", to_pascal_case(name)));
        self.add_plain("(delta))");
    }

    pub fn on_enter(&mut self, name: &str) {
        self.add_newline();
        self.add_indent();
        self.add_operator(".");
        self.add_function("on_enter");
        self.add_plain("(|point| ");
        self.add_type("Message");
        self.add_operator("::");
        self.add_plain(&format!("{}Entered", to_pascal_case(name)));
        self.add_plain("(point))");
    }

    pub fn on_move(&mut self, name: &str) {
        self.add_newline();
        self.add_indent();
        self.add_operator(".");
        self.add_function("on_move");
        self.add_plain("(|point| ");
        self.add_type("Message");
        self.add_operator("::");
        self.add_plain(&format!("{}Moved", to_pascal_case(name)));
        self.add_plain("(point))");
    }

    pub fn on_exit(&mut self, name: &str) {
        self.add_newline();
        self.add_indent();
        self.add_operator(".");
        self.add_function("on_exit");
        self.add_plain("(|point| ");
        self.add_type("Message");
        self.add_operator("::");
        self.add_plain(&format!("{}Exited", to_pascal_case(name)));
        self.add_plain("(point))");
    }

    pub fn on_mouse_interaction(&mut self, interaction: &MouseInteraction) {
        self.add_newline();
        self.add_indent();
        self.add_operator(".");
        self.add_function("interaction");
        self.add_plain("(");
        self.add_type("Interaction");
        self.add_operator("::");
        self.add_plain(&format!("{:?}", interaction));
        self.add_plain(")");
    }

    pub fn on_toggle(&mut self, name: &str) {
        self.add_newline();
        self.add_indent();
        self.add_operator(".");
        self.add_function("on_toggle");
        self.add_plain("(");
        self.add_type("Message");
        self.add_operator("::");
        self.add_plain(&format!("{}Toggled", to_pascal_case(&name)));
        self.add_plain(")");        
    }

    pub fn on_input(&mut self, name: &str) {
        self.add_newline();
        self.add_indent();
        self.add_operator(".");
        self.add_function("on_input");
        self.add_plain("(");
        self.add_type("Message");
        self.add_operator("::");
        self.add_plain(&format!("{}OnInput", to_pascal_case(&name)));
        self.add_plain(")");        
    }

    pub fn on_submit(&mut self, name: &str) {
        self.add_newline();
        self.add_indent();
        self.add_operator(".");
        self.add_function("on_submit");
        self.add_plain("(");
        self.add_type("Message");
        self.add_operator("::");
        self.add_plain(&format!("{}Submitted", to_pascal_case(&name)));
        self.add_plain(")");        
    }

    pub fn on_paste(&mut self, name: &str) {
        self.add_newline();
        self.add_indent();
        self.add_operator(".");
        self.add_function("on_paste");
        self.add_plain("(");
        self.add_type("Message");
        self.add_operator("::");
        self.add_plain(&format!("{}Pasted", to_pascal_case(&name)));
        self.add_plain(")");        
    }    

    pub fn on_option_hovered(&mut self, name: &str) {
        self.add_newline();
        self.add_indent();
        self.add_operator(".");
        self.add_function("on_option_hovered");
        self.add_plain("(");
        self.add_type("Message");
        self.add_operator("::");
        self.add_plain(&format!("{}OnOptionHovered", to_pascal_case(&name)));
        self.add_plain(")");
    }

    pub fn on_open(&mut self, name: &str) {
        self.add_newline();
        self.add_indent();
        self.add_operator(".");
        self.add_function("on_open");
        self.add_plain("(");
        self.add_type("Message");
        self.add_operator("::");
        self.add_plain(&format!("{}OnOpen", to_pascal_case(&name)));
        self.add_plain(")");
    }

    pub fn on_close(&mut self, name: &str) {
        self.add_newline();
        self.add_indent();
        self.add_operator(".");
        self.add_function("on_close");
        self.add_plain("(");
        self.add_type("Message");
        self.add_operator("::");
        self.add_plain(&format!("{}OnClose", to_pascal_case(&name)));
        self.add_plain(")");
    }

    pub fn add_size(&mut self, size: f32) {
            self.add_newline();
            self.add_indent();
            self.add_operator(".");
            self.add_function("size");
            self.add_plain("(");
            self.add_number(&format!("{}", size));
            self.add_plain(")");        
    }

    pub fn add_spacing(&mut self, spacing: f32) {
            self.add_newline();
            self.add_indent();
            self.add_operator(".");
            self.add_function("spacing");
            self.add_plain("(");
            self.add_number(&format!("{}", spacing));
            self.add_plain(")");
    }

    pub fn add_step(&mut self, step: f32) {
        self.add_newline();
        self.increase_indent();
        self.add_indent();
        self.add_operator(".");
        self.add_function("step");
        self.add_plain("(");
        self.add_number(&format!("{}", step));
        self.add_plain(")");
    }

    pub fn add_content_fit(&mut self, choice: &ContentFitChoice) {
        self.add_newline();
        self.add_indent();
        self.add_operator(".");
        self.add_function("content_fit");
        self.add_plain("(ContentFit::");
        match choice {
            ContentFitChoice::Cover => self.add_plain("Cover"),
            ContentFitChoice::Fill => self.add_plain("Fill"),
            ContentFitChoice::ScaleDown => self.add_plain("ScaleDown"),
            ContentFitChoice::None => self.add_plain("None"),
            _ => self.add_plain("Contain"),
        }
        self.add_plain(")");
    }

    pub fn add_style(&mut self, widget_type: &str, style: &str) {
        self.add_newline();
        self.add_indent();
        self.add_operator(".");
        self.add_function("style");
        self.add_number(&format!("({}", widget_type));
        self.add_operator("::");
        self.add_function(&format!("{})", style));
    }

    pub fn add_input_style(&mut self, module: &str, function: &str) {
        self.add_newline();
        self.add_indent();
        self.add_operator(".");
        self.add_function("input_style");
        self.add_number(&format!("({}", module));
        self.add_operator("::");
        self.add_function(&format!("{})", function));
    }

    pub fn add_menu_style(&mut self, module: &str, function: &str) {
        self.add_newline();
        self.add_indent();
        self.add_operator(".");
        self.add_function("menu_style");
        self.add_number(&format!("({}", module));
        self.add_operator("::");
        self.add_function(&format!("{})", function));
    }

    pub fn add_derive(&mut self, csv: &str) {
        let derives = csv.split(", ");
        self.add_plain("#");
        self.add_keyword("[");
        self.add_function("derive");
        self.add_plain("(");
        derives.into_iter().for_each(|derive| {
            self.add_identifier(derive);
            self.add_plain(", ");
        });
        self.add_plain(")");
        self.add_keyword("]");
    }

    pub fn sanitize_name(&self, name: &str) -> String {
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return "widget".to_string();
        }
        
        // Replace spaces and special characters with underscores
        let sanitized = trimmed
            .chars()
            .map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' })
            .collect::<String>()
            .to_lowercase();
        
        // Ensure it starts with a letter or underscore
        if sanitized.chars().next().map_or(false, |c| c.is_numeric()) {
            format!("_{}", sanitized)
        } else if sanitized.is_empty() {
            "widget".to_string()
        } else {
            sanitized
        }
    }
}

pub fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}

pub fn to_snake_case(s: &str) -> String {
    s.to_lowercase().replace(' ', "_")
}

pub fn handle_whitespace(s: &str) -> String {
    s.replace(' ', "_")
}
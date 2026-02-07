use iced::{Alignment, Length, Padding};
use iced::widget::text::LineHeight;
use crate::data_structures::types::type_implementations::{ContentFitChoice, FontType, MouseInteraction, PaddingMode};

pub struct CodeBuilder {
    code: String,
    indent_level: usize,
}

impl CodeBuilder {
    pub fn new() -> Self {
        Self {
            code: String::new(),
            indent_level: 0,
        }
    }

    pub fn build(self) -> String {
        self.code
    }

    pub fn as_str(&self) -> &str {
        &self.code
    }

    pub fn push(&mut self, s: &str) {
        self.code.push_str(s);
    }

    pub fn newline(&mut self) {
        self.code.push('\n');
    }

    pub fn indent(&mut self) {
        for _ in 0..self.indent_level {
            self.code.push_str("    ");
        }
    }

    pub fn indent_str(&self) -> String {
        "    ".repeat(self.indent_level)
    }

    pub fn increase_indent(&mut self) {
        self.indent_level += 1;
    }

    pub fn decrease_indent(&mut self) {
        if self.indent_level > 0 {
            self.indent_level -= 1;
        }
    }

    pub fn line(&mut self, s: &str) {
        self.indent();
        self.code.push_str(s);
        self.code.push('\n');
    }

    pub fn blank_line(&mut self) {
        self.code.push('\n');
    }

    pub fn dot_method(&mut self, name: &str, args: &str) {
        self.newline();
        self.indent();
        self.code.push_str(&format!(".{}({})", name, args));
    }

    pub fn dot_method_no_args(&mut self, name: &str) {
        self.newline();
        self.indent();
        self.code.push_str(&format!(".{}()", name));
    }

    fn dot_message(&mut self, method: &str, name: &str, suffix: &str) {
        self.newline();
        self.indent();
        self.code.push_str(&format!(".{}(Message::{}{})", method, to_pascal_case(name), suffix));
    }

    fn dot_closure_message(&mut self, method: &str, name: &str, suffix: &str, param: &str) {
        self.newline();
        self.indent();
        self.code.push_str(&format!(
            ".{}(|{}| Message::{}{}({}))",
            method, param, to_pascal_case(name), suffix, param
        ));
    }

    pub fn add_width(&mut self, length: Length) {
        self.dot_method("width", &format_length(length));
    }

    pub fn add_height(&mut self, length: Length) {
        self.dot_method("height", &format_length(length));
    }

    pub fn add_max_width(&mut self, max_width: f32) {
        self.dot_method("max_width", &format!("{:.1}", max_width));
    }

    pub fn add_max_height(&mut self, max_height: f32) {
        self.dot_method("max_height", &format!("{:.1}", max_height));
    }

    pub fn add_length(&mut self, length: Length) {
        self.dot_method("length", &format_length(length));
    }

    pub fn add_align_x(&mut self, alignment: Alignment) {
        self.dot_method("align_x", &format_alignment(alignment));
    }

    pub fn add_align_y(&mut self, alignment: Alignment) {
        self.dot_method("align_y", &format_alignment(alignment));
    }

    pub fn add_center_x(&mut self, length: Length) {
        self.dot_method("center_x", &format_length(length));
    }

    pub fn add_center_y(&mut self, length: Length) {
        self.dot_method("center_y", &format_length(length));
    }

    pub fn add_center(&mut self, length: Length) {
        self.dot_method("center", &format_length(length));
    }

    pub fn add_spacing(&mut self, spacing: f32) {
        self.dot_method("spacing", &format!("{}", spacing));
    }

    pub fn add_size(&mut self, size: f32) {
        self.dot_method("size", &format!("{}", size));
    }

    pub fn add_step(&mut self, step: f32) {
        self.dot_method("step", &format!("{}", step));
    }

    pub fn add_clip(&mut self) {
        self.dot_method("clip", "true");
    }
    pub fn add_padding(&mut self, padding: &Padding, padding_mode: PaddingMode) {
        let args = match padding_mode {
            PaddingMode::Uniform => {
                format!("{:.1}", padding.top)
            }
            PaddingMode::Symmetric => {
                format!("[{:.1}, {:.1}]", padding.top, padding.left)
            }
            PaddingMode::Individual => {
                let ind = self.indent_str();
                let inner = format!("{}    ", ind);
                format!(
                    " Padding {{\n\
                     {inner}top: {:.1}, right: {:.1},\n\
                     {inner}bottom: {:.1}, left: {:.1}\n\
                     {ind} }}",
                    padding.top, padding.right, padding.bottom, padding.left
                )
            }
        };
        self.dot_method("padding", &args);
    }

    pub fn add_id(&mut self, id: &str) {
        self.dot_method("id", &format!("\"{}\"", id));
    }

    pub fn add_placeholder(&mut self, placeholder: &str) {
        self.dot_method("placeholder", &format!("\"{}\"", placeholder));
    }

    pub fn add_font(&mut self, font_type: FontType) {
        let arg = match font_type {
            FontType::Monospace => "Font::MONOSPACE",
            _ => "Font::default()",
        };
        self.dot_method("font", arg);
    }

    pub fn add_lineheight(&mut self, lineheight: &LineHeight) {
        let arg = match lineheight {
            LineHeight::Absolute(pixels) => format!("{}", pixels.0),
            LineHeight::Relative(factor) => format!("text::LineHeight::Relative({})", factor),
        };
        self.dot_method("line_height", &arg);
    }

    pub fn add_content_fit(&mut self, choice: &ContentFitChoice) {
        let variant = match choice {
            ContentFitChoice::Cover => "Cover",
            ContentFitChoice::Fill => "Fill",
            ContentFitChoice::ScaleDown => "ScaleDown",
            ContentFitChoice::None => "None",
            _ => "Contain",
        };
        self.dot_method("content_fit", &format!("ContentFit::{}", variant));
    }

    pub fn add_style(&mut self, widget_type: &str, style: &str) {
        self.dot_method("style", &format!("{}::{}", widget_type, style));
    }

    pub fn add_input_style(&mut self, module: &str, function: &str) {
        self.dot_method("input_style", &format!("{}::{}", module, function));
    }

    pub fn add_menu_style(&mut self, module: &str, function: &str) {
        self.dot_method("menu_style", &format!("{}::{}", module, function));
    }

    pub fn on_press(&mut self, name: &str) {
        self.dot_message("on_press", name, "Pressed");
    }

    pub fn on_press_with(&mut self, name: &str) {
        self.newline();
        self.indent();
        self.code.push_str(&format!(
            ".on_press_with(|| Message::{}Pressed)",
            to_pascal_case(name)
        ));
    }

    pub fn on_press_maybe(&mut self, name: &str) {
        self.newline();
        self.indent();
        self.code.push_str(&format!(
            ".on_press_maybe(Some(Message::{}Pressed))",
            to_pascal_case(name)
        ));
    }

    pub fn on_release(&mut self, name: &str) {
        self.dot_message("on_release", name, "Released");
    }

    pub fn on_double_click(&mut self, name: &str) {
        self.dot_message("on_double_click", name, "DoubleClicked");
    }

    pub fn on_right_press(&mut self, name: &str) {
        self.dot_message("on_right_press", name, "RightPressed");
    }

    pub fn on_right_release(&mut self, name: &str) {
        self.dot_message("on_right_release", name, "RightReleased");
    }

    pub fn on_middle_press(&mut self, name: &str) {
        self.dot_message("on_middle_press", name, "MiddlePressed");
    }

    pub fn on_middle_release(&mut self, name: &str) {
        self.dot_message("on_middle_release", name, "MiddleReleased");
    }

    pub fn on_scroll(&mut self, name: &str) {
        self.dot_closure_message("on_scroll", name, "Scrolled", "delta");
    }

    pub fn on_enter(&mut self, name: &str) {
        self.dot_closure_message("on_enter", name, "Entered", "point");
    }

    pub fn on_move(&mut self, name: &str) {
        self.dot_closure_message("on_move", name, "Moved", "point");
    }

    pub fn on_exit(&mut self, name: &str) {
        self.dot_closure_message("on_exit", name, "Exited", "point");
    }

    pub fn on_mouse_interaction(&mut self, interaction: &MouseInteraction) {
        self.dot_method("interaction", &format!("Interaction::{:?}", interaction));
    }

    pub fn on_toggle(&mut self, name: &str) {
        self.dot_message("on_toggle", name, "Toggled");
    }

    pub fn on_input(&mut self, name: &str) {
        self.dot_message("on_input", name, "OnInput");
    }

    pub fn on_submit(&mut self, name: &str) {
        self.dot_message("on_submit", name, "Submitted");
    }

    pub fn on_paste(&mut self, name: &str) {
        self.dot_message("on_paste", name, "Pasted");
    }

    pub fn on_option_hovered(&mut self, name: &str) {
        self.dot_message("on_option_hovered", name, "OnOptionHovered");
    }

    pub fn on_open(&mut self, name: &str) {
        self.dot_message("on_open", name, "OnOpen");
    }

    pub fn on_close(&mut self, name: &str) {
        self.dot_message("on_close", name, "OnClose");
    }
}

pub fn format_length(length: Length) -> String {
    match length {
        Length::Fill => "Length::Fill".to_string(),
        Length::Shrink => "Length::Shrink".to_string(),
        Length::Fixed(px) => format!("Length::Fixed({:.1})", px),
        Length::FillPortion(p) => format!("Length::FillPortion({})", p),
        _ => "Length::Shrink".to_string(),
    }
}

pub fn format_alignment(alignment: Alignment) -> String {
    match alignment {
        Alignment::Start => "Alignment::Start".to_string(),
        Alignment::Center => "Alignment::Center".to_string(),
        Alignment::End => "Alignment::End".to_string(),
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

pub fn sanitize_name(name: &str) -> String {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return "widget".to_string();
    }

    let sanitized = trimmed
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' })
        .collect::<String>()
        .to_lowercase();

    if sanitized.chars().next().map_or(false, |c| c.is_numeric()) {
        format!("_{}", sanitized)
    } else if sanitized.is_empty() {
        "widget".to_string()
    } else {
        sanitized
    }
}

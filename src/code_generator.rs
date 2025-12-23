use iced::{Alignment, Color, Element, Length, Padding, widget::{column, container, space::horizontal, row, scrollable, text}, Background, Border, Theme};
//use crate::widget_helper::*;
use crate::{enum_builder::{EnumDef, TypeSystem}, views};
use std::collections::{HashMap, BTreeMap};
use uuid::Uuid;

use crate::data_structures::types::types::*;
use crate::data_structures::types::type_implementations::*;
use crate::data_structures::properties::properties::*;
use crate::data_structures::widget_hierarchy::WidgetHierarchy;
pub mod tokens;
pub mod writer;
pub mod view;
pub mod import;
mod generate;
use tokens::{Token, TokenBuilder};
use writer::{CodeWriter, to_snake_case, to_pascal_case, handle_whitespace};


/// Code generator for creating Iced code from widget hierarchy
pub struct CodeGenerator<'a> {
    views: Option<&'a BTreeMap<Uuid, AppView>>, 
    current_hierarchy: Option<&'a WidgetHierarchy>,
    app_name: String,
    app_window_title: String,
    widget_counts: HashMap<String, usize>,  // Track duplicate widgets
    widget_names: HashMap<WidgetId, String>,
    type_system: &'a TypeSystem,
    theme: &'a Theme,
}

impl<'a> CodeGenerator<'a> {
    pub fn new(views: &'a BTreeMap<Uuid, AppView>,  theme: &'a Theme, type_system: &'a TypeSystem) -> Self {
        Self {
            views: Some(views),
            current_hierarchy: None,
            app_name: "App".to_string(),
            app_window_title: "App Window".to_string(),
            widget_counts: HashMap::new(),
            widget_names: HashMap::new(),
            type_system: type_system,
            theme: theme,
        }
    }

    pub fn new_single(hierarchy: &'a WidgetHierarchy, theme: &'a Theme, type_system: &'a TypeSystem) -> Self {
        let mut generation = Self {
            views: None,
            current_hierarchy: Some(hierarchy),
            app_name: "App".to_string(),
            app_window_title: "App Window".to_string(),
            widget_counts: HashMap::new(),
            widget_names: HashMap::new(),
            type_system: type_system,
            theme: theme,
        };
        
        // Pre-calculate names so the preview shows the correct variable names (e.g. "button_2")
        generation.collect_widget_names(hierarchy.root());
        
        generation
    }

    /// Set App name for code generation
    pub fn set_app_name(&mut self, name: String) {
        self.app_name = if name.trim().is_empty() { 
            "App".to_string() 
        } else { 
            // Ensure it's a valid Rust identifier
            name.chars()
                .map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' })
                .collect::<String>()
        };
    }

    /// Set Window Title for code generation
    pub fn set_window_title(&mut self, name: String) {
        self.app_window_title = if name.trim().is_empty() { 
            "App".to_string() 
        } else { 
            // Ensure it's a valid Rust identifier
            name.chars()
                .map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' })
                .collect::<String>()
        };
    }

/*     fn generate_struct_definitions(&mut self) {
        if self.type_system.is_none() { return }
        
        // Assuming your TypeSystem has a .structs field similar to .enums
        for struct_def in self.type_system.unwrap().structs.values() {
            self.generate_struct_code(struct_def);
            self.writer.add_newline();
            self.writer.add_newline();
        }
    }

    // You'll need to ensure your TypeSystem crate exports StructDef 
    // or replace StructDef with the actual type from your data_structures
    fn generate_struct_code(&mut self, struct_def: &crate::data_structures::types::types::StructDef) {
        self.writer.add_comment(&format!("// {} struct", struct_def.name));
        self.writer.add_newline();
        self.writer.add_plain("#[derive(Debug, Clone, PartialEq)]"); // Add Default if applicable
        self.writer.add_newline();
        self.writer.add_keyword("pub struct");
        self.writer.add_plain(" ");
        self.writer.add_type(&struct_def.name);
        self.writer.add_plain(" {");
        self.writer.add_newline();
        self.writer.increase_indent();
        
        for field in &struct_def.fields {
            self.writer.add_indent();
            self.writer.add_keyword("pub");
            self.writer.add_plain(" ");
            self.writer.add_identifier(&field.name);
            self.writer.add_operator(":");
            self.writer.add_plain(" ");
            // Assuming field.type_name is a string like "String", "i32", or "MyEnum"
            self.writer.add_type(&field.type_name); 
            self.writer.add_plain(",");
            self.writer.add_newline();
        }
        
        self.writer.decrease_indent();
        self.writer.add_plain("}");
        self.writer.add_newline();
    } */

/*     pub fn generate_standalone_component(&mut self, widget_id: WidgetId) -> Vec<Token> {
        self.tokens.clear();
        self.indent_level = 0;
        self.used_widgets.clear();
        
        // 1. Locate the widget
        let widget = match self.hierarchy.get_widget_by_id(widget_id) {
            Some(w) => w.clone(),
            None => return vec![],
        };

        // 2. Initialize tracking for just this subtree
        self.widget_counts.clear();
        self.widget_names.clear();
        self.collect_widget_names(&widget); // Only collect names for this subtree
        self.collect_used_widgets(&widget); // Only collect imports for this subtree

        // 3. Generate Imports
//        self.generate_imports();
        self.writer.add_newline();

        // 4. Generate Mini-App State
        self.writer.add_comment("// Component State");
        self.writer.add_newline();
        self.writer.add_keyword("struct");
        self.writer.add_plain(" ");
        self.writer.add_type("Component"); // Generic name for snippet
        self.writer.add_plain(" {");
        self.writer.add_newline();
        self.writer.increase_indent();
        self.generate_state_fields(&widget); // Recursive for subtree
        self.writer.decrease_indent();
        self.writer.add_plain("}");
        self.writer.add_newline();
        self.writer.add_newline();

        // 5. Generate Mini-App Messages
        self.writer.add_comment("// Component Messages");
        self.writer.add_newline();
        self.add_derive("Debug, Clone");
        self.writer.add_newline();
        self.writer.add_keyword("enum");
        self.writer.add_plain(" ");
        self.writer.add_type("Message");
        self.writer.add_plain(" {");
        self.writer.add_newline();
        self.writer.increase_indent();
        self.generate_message_variants(&widget); // Recursive for subtree
        // Add a Noop just in case
        self.writer.add_indent();
        self.writer.add_plain("Noop,");
        self.writer.add_newline();
        self.writer.decrease_indent();
        self.writer.add_plain("}");
        self.writer.add_newline();
        self.writer.add_newline();

        // 6. Generate Logic Impl
        self.writer.add_keyword("impl");
        self.writer.add_plain(" ");
        self.writer.add_type("Component");
        self.writer.add_plain(" {");
        self.writer.add_newline();
        self.writer.increase_indent();

        // Update
        self.writer.add_indent();
        self.writer.add_keyword("fn");
        self.writer.add_plain(" ");
        self.writer.add_function("update");
        self.writer.add_plain("(&");
        self.writer.add_keyword("mut");
        self.writer.add_plain(" ");
        self.writer.add_keyword("self");
        self.writer.add_plain(", message: Message) {");
        self.writer.add_newline();
        self.writer.increase_indent();
        self.writer.add_indent();
        self.writer.add_keyword("match");
        self.writer.add_plain(" message {");
        self.writer.add_newline();
        self.writer.increase_indent();
        self.generate_update_match_arms(&widget);
        self.writer.add_indent();
        self.writer.add_plain("_ => {}"); // Handle Noop
        self.writer.add_newline();
        self.writer.decrease_indent();
        self.writer.add_indent();
        self.writer.add_plain("}");
        self.writer.decrease_indent();
        self.writer.add_newline();
        self.writer.add_indent();
        self.writer.add_plain("}");
        self.writer.add_newline();
        self.writer.add_newline();

        // View
        self.writer.add_indent();
        self.writer.add_keyword("fn");
        self.writer.add_plain(" ");
        self.writer.add_function("view");
        self.writer.add_plain("(&");
        self.writer.add_keyword("self");
        self.writer.add_plain(") -> Element<Message> {");
        self.writer.add_newline();
        self.writer.increase_indent();
        
        // Determine if we need to wrap the widget to make it look right
        // e.g., if it's a single button, center it so it doesn't stretch weirdly in preview
        let needs_container = !matches!(widget.widget_type, WidgetType::Container | WidgetType::Column | WidgetType::Row);
        
        if needs_container {
            self.writer.add_indent();
            self.writer.add_function("container");
            self.writer.add_plain("(");
            self.writer.add_newline();
            self.writer.increase_indent();
        }

        self.generate_widget_creation(&widget, true);

        if needs_container {
            self.writer.add_newline();
            self.writer.decrease_indent();
            self.writer.add_indent();
            self.writer.add_plain(")");
            self.writer.add_newline();
            self.writer.add_indent();
            self.writer.add_operator(".");
            self.writer.add_function("center_x");
            self.writer.add_plain("(Length::Fill)");
            self.writer.add_newline();
            self.writer.add_indent();
            self.writer.add_operator(".");
            self.writer.add_function("center_y");
            self.writer.add_plain("(Length::Fill)");
        }

        self.writer.add_newline();
        self.writer.add_indent();
        self.writer.add_operator(".");
        self.writer.add_function("into");
        self.writer.add_plain("()");
        self.writer.add_newline();
        self.writer.decrease_indent();
        self.writer.add_indent();
        self.writer.add_plain("}");

        self.writer.decrease_indent();
        self.writer.add_newline();
        self.writer.add_plain("}");

        // Helper to add derive macro
        self.tokens.clone()
    } */

    /// Generates a multi-file project structure
    pub fn generate_project_structure(&mut self) -> HashMap<String, Vec<Token>> {
        let mut files = HashMap::new();
        
        // 1. Generate types.rs (Shared Enums)
        let mut writer = CodeWriter::new();
        generate::types::generate_enum_definitions(&mut writer, self.type_system);
        let tokens = writer.tokens();
        if !tokens.is_empty() {
            files.insert("types.rs".to_string(), tokens);
        }

        // 2. Identify generated modules for main.rs to import
        let mut generated_modules = Vec::new();

        // 3. Iterate through all views
        let views = self.views.expect("Cannot generate project structure without views map");
        for view in views.values() {
            self.prepare_for_view(&view.hierarchy);
            
            let file_name = if view.is_main {
                "main.rs".to_string()
            } else {
                // components are typically named by their view name
                format!("{}.rs", to_snake_case(&view.name))
            };

            if !view.is_main {
                generated_modules.push(to_snake_case(&view.name));
            }

            let struct_name = if view.is_main {
                self.app_name.clone()
            } else {
                handle_whitespace(
                    &to_pascal_case(&view.name)
                )
            };

            // Generate the content
            let tokens = self.generate_single_view_content(view, &struct_name, files.contains_key("types.rs"), &generated_modules);
            files.insert(file_name, tokens);
        }

        files
    }

    /// Reset internal counters and set current hierarchy context
    fn prepare_for_view(&mut self, hierarchy: &'a WidgetHierarchy) {
        self.current_hierarchy = Some(hierarchy);
        self.widget_counts.clear();
        self.widget_names.clear();
        self.collect_widget_names(hierarchy.root());
    }

    fn generate_single_view_content(
        &mut self, 
        view: &AppView, 
        struct_name: &str, 
        has_types: bool,
        modules_to_mod: &[String]
    ) -> Vec<Token> {
        let mut writer = CodeWriter::new();
        let root = view.hierarchy.root();
        let names = &self.widget_names;

        // -- Imports --
        generate::events::generate_imports(&mut writer, root);
        
        if has_types {
            writer.add_keyword("mod");
            writer.add_plain(" types;");
            writer.add_newline();
            writer.add_keyword("use");
            writer.add_plain(" types::*;");
            writer.add_newline();
        }

        // If this is Main, we need to mod the other generated component files
        if view.is_main {
            for module in modules_to_mod {
                writer.add_keyword("mod");
                writer.add_plain(&format!(" {};", module));
                writer.add_newline();
            }
            writer.add_newline();
        }

        // -- Message Enum --
        generate::events::generate_message_enum(&mut writer, root, names, self.type_system);
        writer.add_newline();

        // -- Struct --
        generate::app::generate_app_struct(&mut writer, root, names, struct_name, self.type_system);
        writer.add_newline();

        // -- Impl --
        generate::app::generate_impl(
            &mut writer, 
            root, 
            names, 
            struct_name, 
            // Only pass title/theme if it's the main view
            if view.is_main { Some((&self.app_name, self.theme)) } else { None }, 
            self.type_system
        );
        writer.add_newline();

        // -- Main Function (Only for Main View) --
        if view.is_main {
            generate::app::generate_main_function(&mut writer, struct_name);
        }

        writer.tokens()
    }

    fn generate_widget_creation(&mut self, widget: &Widget, use_self: bool) {
        let props = &widget.properties;
        
        match widget.widget_type {
            WidgetType::Container => {}
            WidgetType::Row => {}
            WidgetType::Column => {}
            WidgetType::Button => {}
            WidgetType::Text => {}
            WidgetType::TextInput => {}
            WidgetType::Checkbox => {}
            WidgetType::Radio => {}
            WidgetType::Slider => {}
            WidgetType::VerticalSlider => {}
            WidgetType::ProgressBar => {}
            WidgetType::Toggler => {}
            WidgetType::PickList => {}
            WidgetType::Scrollable => {}
            WidgetType::Space => {}
            WidgetType::Rule => {}
            WidgetType::Image => {}
            WidgetType::Svg => {}
            WidgetType::Tooltip => {}
            WidgetType::ComboBox => {}
            WidgetType::Markdown => {
/*                self.writer.add_indent();
                self.writer.add_keyword("let");
                self.writer.add_plain(" items = ");
                self.writer.add_function("markdown::parse");
                self.writer.add_plain("(");
                self.writer.add_string(&format!("\"{}\"", props.markdown_source.replace("\"", "\\\"")));
                self.writer.add_plain(");");
                self.writer.add_newline();
                self.writer.add_indent();
                self.writer.add_function("markdown::view");
                self.writer.add_plain("(");
                self.writer.add_operator("&");
                self.writer.add_plain("items, ");
                self.writer.add_newline();
                self.writer.increase_indent();
                self.writer.add_indent();
                self.writer.add_type("markdown::Settings");
                self.writer.add_plain(" {");
                self.writer.add_newline();
                 self.writer.increase_indent();
                
                self.writer.add_indent();
                self.writer.add_plain("link_color: Some(");
                self.add_color(props.markdown_link_color);
                self.writer.add_plain("),");
                self.writer.add_newline();
                
                self.writer.add_indent();
                self.writer.add_plain("link_underline: ");
                self.writer.add_keyword(if props.markdown_link_underline { "true" } else { "false" });
                self.writer.add_plain(",");
                self.writer.add_newline();
                
                self.writer.add_indent();
                self.writer.add_plain("code_color: Some(");
                self.add_color(props.markdown_code_color);
                self.writer.add_plain("),");
                self.writer.add_newline();
                
                self.writer.add_indent();
                self.writer.add_plain("block_spacing: ");
                self.writer.add_number(&format!("{}", props.markdown_block_spacing));
                self.writer.add_plain(",");
                self.writer.add_newline();
                
                self.writer.decrease_indent();
                self.writer.add_indent();
                self.writer.add_plain("}");
                self.writer.add_newline();
                self.writer.decrease_indent();
                self.writer.add_indent();
                self.writer.add_plain(")");
                self.generate_markdown_properties(props);
*/
            }
            WidgetType::MouseArea => {}
            WidgetType::QRCode => {}     
            WidgetType::Stack => {}         
            WidgetType::Themer => {} 
            WidgetType::Pin => {
            //    todo!("implement code gen for pin");
            }
            WidgetType::ViewReference => {}
        }
    }

    fn collect_widget_names(&mut self, widget: &Widget) {
        // Generate unique name for this widget
        let mut derp = String::new();
        let base_name = if !widget.properties.widget_name.trim().is_empty() {
            self.sanitize_name(&widget.properties.widget_name)
        } else {
            match widget.widget_type {
                WidgetType::Button => "button",
                WidgetType::Text => "text",
                WidgetType::TextInput => "text_input",
                WidgetType::Checkbox => "checkbox",
                WidgetType::Radio => "radio",
                WidgetType::Slider => "slider",
                WidgetType::VerticalSlider => "vertical_slider",
                WidgetType::ProgressBar => "progress_bar",
                WidgetType::Toggler => "toggler",
                WidgetType::PickList => "pick_list",
                _ => {
                    derp = format!("{:?}", widget.widget_type).to_lowercase();

                    &derp
                }
            }.to_string()
        };

        // Get count for this widget type
        let type_key = format!("{:?}", widget.widget_type).to_lowercase();
        let count = self.widget_counts.entry(type_key).or_insert(0);
        *count += 1;

        let final_name = if *count > 1 {
            format!("{}_{}", base_name, count)
        } else {
            base_name
        };

        self.widget_names.insert(widget.id, final_name);

        // Process children
        for child in &widget.children {
            self.collect_widget_names(child);
        }
    }

    fn get_widget_name(&self, widget_id: WidgetId) -> String {
        self.widget_names.get(&widget_id)
            .cloned()
            .unwrap_or_else(|| "widget".to_string())
    }
/* 
    fn generate_markdown_properties(&mut self, props: &Properties) {
        if !matches!(props.width, Length::Fill) {
            self.writer.add_newline();
            self.writer.increase_indent();
            self.writer.add_indent();
            self.writer.add_operator(".");
            self.writer.add_function("width");
            self.writer.add_plain("(");
            self.add_length(props.width);
            self.writer.add_plain(")");
            self.writer.decrease_indent();
        }
        
        if !matches!(props.height, Length::Shrink) {
            self.writer.add_newline();
            self.writer.increase_indent();
            self.writer.add_indent();
            self.writer.add_operator(".");
            self.writer.add_function("height");
            self.writer.add_plain("(");
            self.add_length(props.height);
            self.writer.add_plain(")");
            self.writer.decrease_indent();
        }
    }

    fn generate_padding(&mut self, padding: &Padding, padding_mode: PaddingMode) {
        self.writer.add_newline();
        self.writer.add_indent();
        self.writer.add_operator(".");
        self.writer.add_function("padding");
        self.writer.add_plain("(");
        
        match padding_mode {
            PaddingMode::Uniform => {
                // .padding(10.0)
                self.writer.add_number(&format!("{:.1}", padding.top));
            }
            PaddingMode::Symmetric => {
                // .padding([vertical, horizontal])
                self.writer.add_plain("[");
                self.writer.add_number(&format!("{:.1}", padding.top));
                self.writer.add_plain(", ");
                self.writer.add_number(&format!("{:.1}", padding.left));
                self.writer.add_plain("]");
            }
            PaddingMode::Individual => {
                // .padding(Padding { top, right, bottom, left })
                self.writer.add_newline();
                self.writer.add_type("Padding");
                self.writer.add_plain(" { ");
                self.writer.add_plain("top: ");
                self.writer.add_number(&format!("{:.1}", padding.top));
                self.writer.add_plain(", ");
                self.writer.add_plain("right: ");
                self.writer.add_number(&format!("{:.1}", padding.right));
                self.writer.add_plain(", ");
                self.writer.add_newline();
                self.writer.add_plain("bottom: ");
                self.writer.add_number(&format!("{:.1}", padding.bottom));
                self.writer.add_plain(", ");
                self.writer.add_plain("left: ");
                self.writer.add_number(&format!("{:.1}", padding.left));
                self.writer.add_newline();
                self.writer.add_plain(" }");
            }
        }
        
        self.writer.add_plain(")");
    }

    fn add_length(&mut self, length: Length) {
        match length {
            Length::Fill => {
                self.writer.add_type("Length");
                self.writer.add_operator("::");
                self.writer.add_plain("Fill");
            }
            Length::Shrink => {
                self.writer.add_type("Length");
                self.writer.add_operator("::");
                self.writer.add_plain("Shrink");
            }
            Length::Fixed(px) => {
                self.writer.add_type("Length");
                self.writer.add_operator("::");
                self.writer.add_plain("Fixed");
                self.writer.add_plain("(");
                self.writer.add_number(&format!("{:.1}", px));
                self.writer.add_plain(")");
            }
            Length::FillPortion(p) => {
                self.writer.add_type("Length");
                self.writer.add_operator("::");
                self.writer.add_plain("FillPortion");
                self.writer.add_plain("(");
                self.writer.add_number(&format!("{}", p));
                self.writer.add_plain(")");
            }
            _ => {
                self.writer.add_type("Length");
                self.writer.add_operator("::");
                self.writer.add_plain("Shrink");
            }
        }
    } */

    fn sanitize_name(&self, name: &str) -> String {
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

    pub fn generate_widget_code_rewrite(&mut self, widget_id: WidgetId) -> Vec<Token> {
        let hierarchy = self.current_hierarchy.expect("No hierarchy set for code generation");
        let widget = match hierarchy.get_widget_by_id(widget_id) {
            Some(w) => w,
            None => return Vec::new(),
        };

        if self.widget_names.is_empty() {
             self.collect_widget_names(hierarchy.root());
        }
        
        let mut writer = CodeWriter::new();
        crate::code_generator::generate::widgets::generate_widget_code(&mut writer, widget, &self.widget_names, false);

        writer.tokens()
    }
}

pub fn build_code_view_with_height<'a>(
    tokens: &[Token], 
    height: f32,
    theme: &Theme
) -> Element<'a, views::widget_tree::Message> {
    // Group tokens by lines
    let mut lines: Vec<Vec<Token>> = vec![vec![]];
    
    for token in tokens {
        if token.text.contains('\n') {
            // Handle tokens that contain newlines
            let parts: Vec<&str> = token.text.split('\n').collect();
            for (i, part) in parts.iter().enumerate() {
                if !part.is_empty() {
                    lines.last_mut().unwrap().push(Token {
                        text: part.to_string(),
                        token_type: token.token_type,
                    });
                }
                // Add new line for all but the last part
                if i < parts.len() - 1 {
                    lines.push(vec![]);
                }
            }
        } else {
            lines.last_mut().unwrap().push(token.clone());
        }
    }

    let bg_color = match theme {
        Theme::Light => Color::from_rgb8(248, 248, 248),  // Very light gray
        Theme::Dark => Color::from_rgb8(30, 30, 30),       // Dark gray
        _ => Color::from_rgb8(40, 40, 40),                 // Default dark
    };

    let border_color = match theme {
        Theme::Light => Color::from_rgb8(200, 200, 200),   // Light gray border
        Theme::Dark => Color::from_rgb8(60, 60, 60),        // Dark gray border
        _ => Color::from_rgb8(80, 80, 80),
    };
    
    // Build the content as a column of rows
    let content = column(
        lines.into_iter().map(|line| {
            if line.is_empty() {
                row![text(" ").size(14).font(iced::Font::MONOSPACE)].into()
            } else {
                row(
                    line.into_iter().map(|token| {
                        text(token.text)
                            .size(14)
                            .font(iced::Font::MONOSPACE)
                            .color(token.token_type.color_for_theme(&theme))
                            .into()
                    }).collect::<Vec<Element<'a, views::widget_tree::Message>>>()
                ).into()
            }
        }).collect::<Vec<Element<'a, views::widget_tree::Message>>>()
    )
    .spacing(2);
    
    container(
        scrollable(
            container(content)
                .width(Length::Fill)
                .padding(15)
                .style(move |_| container::Style {
                    background: Some(Background::Color(bg_color)),
                    border: Border {
                        color: border_color,
                        width: 1.0,
                        radius: 4.0.into(),
                    },
                    ..Default::default()
                })
        )
        .width(Length::Fill)
        .height(
            if height == 0.0 {
                Length::Fill
            }
            else {
                Length::Fixed(height)
            }
        )
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

/// Build a syntax-highlighted code view
pub fn build_code_view<'a>(tokens: &[Token], theme: &Theme) -> Element<'a, views::widget_tree::Message> {
    build_code_view_with_height(tokens, 300.0, theme)
}

/// Build a syntax-highlighted code view - generic so I can use it outside of widget_helper::Messages
pub fn build_code_view_with_height_generic<'a, Message: 'a>(
    tokens: &[Token], 
    height: f32,
    theme: Theme
) -> Element<'a, Message> {
    // Group tokens by lines
    let mut lines: Vec<Vec<Token>> = vec![vec![]];
    
    for token in tokens {
        if token.text.contains('\n') {
            let parts: Vec<&str> = token.text.split('\n').collect();
            for (i, part) in parts.iter().enumerate() {
                if !part.is_empty() {
                    lines.last_mut().unwrap().push(Token {
                        text: part.to_string(),
                        token_type: token.token_type,
                    });
                }
                if i < parts.len() - 1 {
                    lines.push(vec![]);
                }
            }
        } else {
            lines.last_mut().unwrap().push(token.clone());
        }
    }

    let bg_color = match theme {
        Theme::Light => Color::from_rgb8(248, 248, 248),
        Theme::Dark => Color::from_rgb8(30, 30, 30),
        _ => Color::from_rgb8(40, 40, 40),
    };

    let border_color = match theme {
        Theme::Light => Color::from_rgb8(200, 200, 200),
        Theme::Dark => Color::from_rgb8(60, 60, 60),
        _ => Color::from_rgb8(80, 80, 80),
    };
    
    let content = column(
        lines.into_iter().map(|line| {
            if line.is_empty() {
                row![text(" ").size(14).font(iced::Font::MONOSPACE)].into()
            } else {
                row(
                    line.into_iter().map(|token| {
                        text(token.text)
                            .size(14)
                            .font(iced::Font::MONOSPACE)
                            .color(token.token_type.color_for_theme(&theme))
                            .into()
                    }).collect::<Vec<Element<'a, Message>>>()
                ).into()
            }
        }).collect::<Vec<Element<'a, Message>>>()
    )
    .spacing(2);
    
    container(
        scrollable(
            container(content)
                .width(Length::Fill)
                .padding(15)
                .style(move |_| container::Style {
                    background: Some(Background::Color(bg_color)),
                    border: Border {
                        color: border_color,
                        width: 1.0,
                        radius: 4.0.into(),
                    },
                    ..Default::default()
                })
        )
        .width(Length::Fill)
        .height(
            if height == 0.0 {
                Length::Fill
            } else {
                Length::Fixed(height)
            }
        )
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}



/// StyleFn Generators

/// Generate tokens for container style code
pub fn generate_container_style_tokens(
    text_color: Color,
    background_color: Color,
    border_color: Color,
    border_width: f32,
    border_radius_top_left: f32,
    border_radius_top_right: f32,
    border_radius_bottom_right: f32,
    border_radius_bottom_left: f32,
    shadow_enabled: bool,
    shadow_color: Color,
    shadow_offset_x: f32,
    shadow_offset_y: f32,
    shadow_blur_radius: f32,
    snap: bool,
) -> Vec<Token> {
    let mut builder = TokenBuilder::new();

    builder.add_plain("container");
    builder.add_operator("::");
    builder.add_type("Style");
    builder.add_space();
    builder.add_plain("{");
    builder.add_newline();
    builder.increase_indent();

    // text_color field
    builder.add_field("text_color", |b| {
        b.add_plain("Some(");
        b.add_color(text_color);
        b.add_plain(")");
    });

    // background field
    builder.add_field("background", |b| {
        b.add_plain("Some(");
        b.add_type("Background");
        b.add_operator("::");
        b.add_type("Color");
        b.add_plain("(");
        b.add_color(background_color);
        b.add_plain("))");
    });

    // border field
    builder.add_field("border", |b| {
        b.add_struct("Border", |b| {
            b.add_field("color", |b| b.add_color(border_color));
            b.add_field("width", |b| b.add_number(&format!("{:.1}", border_width)));
            b.add_field("radius", |b| {
                b.add_struct("Radius", |b| {
                    b.add_field("top_left", |b| b.add_number(&format!("{:.1}", border_radius_top_left)));
                    b.add_field("top_right", |b| b.add_number(&format!("{:.1}", border_radius_top_right)));
                    b.add_field("bottom_right", |b| b.add_number(&format!("{:.1}", border_radius_bottom_right)));
                    b.add_field("bottom_left", |b| b.add_number(&format!("{:.1}", border_radius_bottom_left)));
                });
            });
        });
    });

    // shadow field
    builder.add_field("shadow", |b| {
        if shadow_enabled {
            b.add_struct("Shadow", |b| {
                b.add_field("color", |b| b.add_color(shadow_color));
                b.add_field("offset", |b| {
                    b.add_struct("Vector", |b| {
                        b.add_field("x", |b| b.add_number(&format!("{:.1}", shadow_offset_x)));
                        b.add_field("y", |b| b.add_number(&format!("{:.1}", shadow_offset_y)));
                    });
                });
                b.add_field("blur_radius", |b| b.add_number(&format!("{:.1}", shadow_blur_radius)));
            });
        } else {
            b.add_type("Shadow");
            b.add_operator("::");
            b.add_function("default");
            b.add_plain("()");
        }
    });

    // snap field
    builder.add_field("snap", |b| {
        b.add_keyword(if snap { "true" } else { "false" });
    });

    builder.decrease_indent();
    builder.add_plain("}");

    builder.into_tokens()
}
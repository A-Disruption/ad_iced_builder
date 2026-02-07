use iced::widget::{Column, text_editor};
use iced::{Element, Length, Task, Theme, window};
use tree_sitter_highlighter::{
    TsSettings, TreeSitterIcedHighlighter, SyntaxHighlight, code_gen_text_editor_style
};
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;
use uuid::Uuid;


use crate::data_structures::types::types::*;
use crate::data_structures::widget_hierarchy::WidgetHierarchy;
use crate::views::theme_and_stylefn_builder::CustomThemes;
use crate::enum_builder::TypeSystem;
use crate::{Window, WindowEnum};
use builder::{CodeBuilder, to_pascal_case, to_snake_case, handle_whitespace, sanitize_name};

// Sub-modules
pub(crate) mod helpers;
pub mod builder;
pub mod styles;
pub mod widgets;
pub mod types;
pub mod events;
pub mod app;
pub mod view;
pub mod window_settings;

pub use styles::{
    generate_button_style_code,
    generate_container_style_code,
    generate_checkbox_style_code,
    generate_combo_box_style_code,
};

pub struct CodeGeneratorV2<'a> {
    views: Option<&'a BTreeMap<Uuid, AppView>>,
    windows: Option<&'a BTreeMap<window::Id, Window>>,
    current_hierarchy: Option<&'a WidgetHierarchy>,
    app_name: String,
    widget_counts: HashMap<String, usize>,
    widget_names: HashMap<WidgetId, String>,
    type_system: &'a TypeSystem,
    theme: &'a Theme,
}

impl<'a> CodeGeneratorV2<'a> {
    pub fn new(
        views: &'a BTreeMap<Uuid, AppView>,
        windows: &'a BTreeMap<window::Id, Window>,
        theme: &'a Theme,
        type_system: &'a TypeSystem,
    ) -> Self {
        Self {
            views: Some(views),
            windows: Some(windows),
            current_hierarchy: None,
            app_name: "App".to_string(),
            widget_counts: HashMap::new(),
            widget_names: HashMap::new(),
            type_system,
            theme,
        }
    }

    pub fn new_single(
        hierarchy: &'a WidgetHierarchy,
        theme: &'a Theme,
        type_system: &'a TypeSystem,
    ) -> Self {
        let mut generator = Self {
            views: None,
            windows: None,
            current_hierarchy: Some(hierarchy),
            app_name: "App".to_string(),
            widget_counts: HashMap::new(),
            widget_names: HashMap::new(),
            type_system,
            theme,
        };
        generator.collect_widget_names(hierarchy.root());
        generator
    }

    pub fn set_app_name(&mut self, name: String) {
        self.app_name = if name.trim().is_empty() {
            "App".to_string()
        } else {
            name.chars()
                .map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' })
                .collect::<String>()
        };
    }

    pub fn generate_widget_code_rewrite(
        &mut self,
        widget_id: WidgetId,
        custom_styles: &CustomThemes,
    ) -> String {
        let hierarchy = self
            .current_hierarchy
            .expect("No hierarchy set for code generation");
        let widget = match hierarchy.get_widget_by_id(widget_id) {
            Some(w) => w,
            None => return String::new(),
        };

        if self.widget_names.is_empty() {
            self.collect_widget_names(hierarchy.root());
        }

        let mut b = CodeBuilder::new();
        widgets::generate_widget_code(&mut b, widget, &self.widget_names, false, custom_styles);
        b.build()
    }

    /// Generate a multi-file project structure. Returns filename -> code string.
    pub fn generate_project_structure(
        &mut self,
        custom_styles: &CustomThemes,
    ) -> HashMap<String, String> {
        let mut files = HashMap::new();

        // 1. Generate types.rs (shared enums)
        let mut b = CodeBuilder::new();
        types::generate_enum_definitions(&mut b, self.type_system);
        let types_code = b.build();
        if !types_code.is_empty() {
            files.insert("types.rs".to_string(), types_code);
        }

        // 2. Track generated modules for main.rs imports
        let mut generated_modules = Vec::new();

        // 3. Iterate through all views
        let views = self
            .views
            .expect("Cannot generate project structure without views map");
        for view_entry in views.values() {
            self.prepare_for_view(&view_entry.hierarchy);

            let file_name = if view_entry.is_main {
                "main.rs".to_string()
            } else {
                format!("{}.rs", to_snake_case(&view_entry.name))
            };

            if !view_entry.is_main {
                generated_modules.push(to_snake_case(&view_entry.name));
            }

            let struct_name = if view_entry.is_main {
                self.app_name.clone()
            } else {
                handle_whitespace(&to_pascal_case(&view_entry.name))
            };

            let code = self.generate_single_view_content(
                view_entry,
                &struct_name,
                files.contains_key("types.rs"),
                &generated_modules,
                custom_styles,
            );
            files.insert(file_name, code);
        }

        files
    }

    /// Reset internal counters and set current hierarchy context.
    fn prepare_for_view(&mut self, hierarchy: &'a WidgetHierarchy) {
        self.current_hierarchy = Some(hierarchy);
        self.widget_counts.clear();
        self.widget_names.clear();
        self.collect_widget_names(hierarchy.root());
    }

    fn generate_single_view_content(
        &mut self,
        view_entry: &AppView,
        struct_name: &str,
        has_types: bool,
        modules_to_mod: &[String],
        custom_styles: &CustomThemes,
    ) -> String {
        let mut b = CodeBuilder::new();
        let root = view_entry.hierarchy.root();
        let names = &self.widget_names;

        let window_config = match self.windows {
            Some(windows) => {
                let mut config =
                    WindowConfig::new("Visualizer".to_string(), window::Settings::default());
                for (_id, window) in windows {
                    if window.windowtype == WindowEnum::Visualizer {
                        config = window.config.clone();
                    }
                }
                config
            }
            None => WindowConfig::new("Visualizer".to_string(), window::Settings::default()),
        };

        // -- Imports --
        events::generate_imports(&mut b, root);

        if has_types {
            b.line("mod types;");
            b.line("use types::*;");
        }

        // If this is Main, mod the other generated component files
        if view_entry.is_main {
            for module in modules_to_mod {
                b.line(&format!("mod {};", module));
            }
            b.newline();
        }

        // -- Message Enum --
        events::generate_message_enum(&mut b, root, names, self.type_system);
        b.newline();

        // -- Struct --
        app::generate_app_struct(&mut b, root, names, struct_name, self.type_system);
        b.newline();

        // -- Impl --
        app::generate_impl(
            &mut b,
            root,
            names,
            struct_name,
            if view_entry.is_main {
                Some((&window_config, self.theme))
            } else {
                None
            },
            self.type_system,
            custom_styles,
        );
        b.newline();

        // -- Main Function (only for main view) --
        if view_entry.is_main {
            app::generate_main_function(
                &mut b,
                struct_name,
                if view_entry.is_main {
                    Some(&window_config)
                } else {
                    None
                },
            );
        }

        b.build()
    }

    fn collect_widget_names(&mut self, widget: &Widget) {
        let mut derp = String::new();
        let base_name = if !widget.properties.widget_name.trim().is_empty() {
            sanitize_name(&widget.properties.widget_name)
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
            }
            .to_string()
        };

        let type_key = format!("{:?}", widget.widget_type).to_lowercase();
        let count = self.widget_counts.entry(type_key).or_insert(0);
        *count += 1;

        let final_name = if *count > 1 {
            format!("{}_{}", base_name, count)
        } else {
            base_name
        };

        self.widget_names.insert(widget.id, final_name);

        for child in &widget.children {
            self.collect_widget_names(child);
        }
    }
}

// ── String-based code view helpers ────────────────────────────────────

/// Build a syntax-highlighted code view from a plain source string.
/// Uses tree_sitter for automatic syntax highlighting instead of manual tokens.
pub fn build_code_view_from_string<'a>(
    code: &str,
    height: f32,
    theme: &Theme,
) -> Element<'a, crate::views::widget_tree::Message> {
    build_code_view_from_string_generic::<crate::views::widget_tree::Message>(code, height, theme.clone())
}

/// Generic version of `build_code_view_from_string` for use with any Message type.
pub fn build_code_view_from_string_generic<'a, M: 'a>(
    code: &str,
    height: f32,
    theme: Theme,
) -> Element<'a, M> {
    use iced::widget::{column, row, container, scrollable, text};
    use iced::{Background, Border, Font};
    use tree_sitter_highlighter::{Highlighter, TreeSitterIcedHighlighter, TokenKind};

    let palette = theme.extended_palette();
    let bg_color = palette.background.weakest.color;
    let border_color = palette.background.weak.color;
    let default_color = palette.background.base.text;

    // Parse and highlight with tree_sitter
    let lines_text: Vec<&str> = code.split('\n').collect();
    let mut highlighted_lines: Vec<Vec<(String, iced::Color)>> = Vec::new();

    match Highlighter::new() {
        Ok(mut highlighter) => {
            if highlighter.set_text(code).is_ok() {
                let spans = highlighter
                    .highlight_range(code, 0..code.len())
                    .unwrap_or_default();

                // Group spans by line
                let mut spans_by_line: Vec<Vec<&tree_sitter_highlighter::Span>> =
                    vec![Vec::new(); lines_text.len()];
                for span in &spans {
                    if span.line < spans_by_line.len() {
                        spans_by_line[span.line].push(span);
                    }
                }

                for (line_idx, line_text) in lines_text.iter().enumerate() {
                    let line_spans = &spans_by_line[line_idx];
                    let mut segments: Vec<(String, iced::Color)> = Vec::new();
                    let mut cursor = 0usize;

                    for span in line_spans {
                        let start = span.start_col.min(line_text.len());
                        let end = span.end_col.min(line_text.len());

                        // Add any gap before this span as default-colored text
                        if cursor < start {
                            let gap = &line_text[cursor..start];
                            if !gap.is_empty() {
                                segments.push((gap.to_string(), default_color));
                            }
                        }

                        // Add the highlighted span
                        if start < end {
                            let segment_text = &line_text[start..end];
                            let format =
                                TreeSitterIcedHighlighter::to_format(&span.kind, &theme);
                            let color = format.color.unwrap_or(default_color);
                            segments.push((segment_text.to_string(), color));
                        }

                        cursor = end;
                    }

                    // Add remaining text after last span
                    if cursor < line_text.len() {
                        let remaining = &line_text[cursor..];
                        if !remaining.is_empty() {
                            segments.push((remaining.to_string(), default_color));
                        }
                    }

                    // If the line was empty, push a space for proper line height
                    if segments.is_empty() {
                        segments.push((" ".to_string(), default_color));
                    }

                    highlighted_lines.push(segments);
                }
            } else {
                // Fallback: plain text
                for line in &lines_text {
                    let display = if line.is_empty() { " " } else { line };
                    highlighted_lines.push(vec![(display.to_string(), default_color)]);
                }
            }
        }
        Err(_) => {
            // Fallback: plain text without highlighting
            for line in &lines_text {
                let display = if line.is_empty() { " " } else { line };
                highlighted_lines.push(vec![(display.to_string(), default_color)]);
            }
        }
    }

    // Build UI from highlighted segments
    let content = column(
        highlighted_lines
            .into_iter()
            .map(|segments| {
                row(segments
                    .into_iter()
                    .map(|(txt, color)| {
                        text(txt)
                            .size(14)
                            .font(Font::MONOSPACE)
                            .color(color)
                            .into()
                    })
                    .collect::<Vec<Element<'a, M>>>())
                .into()
            })
            .collect::<Vec<Element<'a, M>>>(),
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
                }),
        )
        .width(Length::Fill)
        .height(if height == 0.0 {
            Length::Fill
        } else {
            Length::Fixed(height)
        }),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

pub struct CodeView {
    content: text_editor::Content,
    theme: Theme,
}

#[derive(Debug, Clone)]
pub enum Message {
    Edit(text_editor::Action),
    ChangeTheme(Theme),
}

impl CodeView {
    pub fn new(content: text_editor::Content, theme: Theme) -> Self {
        Self {
            content,
            theme
        }
    }

    pub fn view(state: &CodeView) -> Element<'_, Message> {
        let settings = TsSettings {
            text: Arc::<str>::from(state.content.text()),
        };
        let extended = state.theme.extended_palette();
        let _syntax = SyntaxHighlight::generate(extended);

        Column::new()
            .push(
                text_editor(&state.content)
                    .placeholder("Type something here...")
                    .highlight_with::<TreeSitterIcedHighlighter>(
                        settings,
                        TreeSitterIcedHighlighter::to_format,
                    )
                    .on_action(Message::Edit)
                    .height(Length::Fill)
                    .style(code_gen_text_editor_style)
                    .size(14.0),
            )
            .into()
    }

    pub fn update(state: &mut CodeView, message: Message) -> Task<Message>  {
        match message {
            Message::Edit(action) => {
                state.content.perform(action);
            }

            Message::ChangeTheme(theme) => {
                state.theme = theme;
            }
        }
        Task::none()
    }
}
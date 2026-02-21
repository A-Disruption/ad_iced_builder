use iced::{Theme, window};
use std::collections::{BTreeMap, HashMap};
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
    generate_rule_style_code,
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
        widgets::generate_widget_code(&mut b, widget, &self.widget_names, false, custom_styles, self.type_system, &[]);
        b.build()
    }

    /// Generate a multi-file project structure. Returns filename -> code string.
    pub fn generate_project_structure(
        &mut self,
        custom_styles: &CustomThemes,
    ) -> HashMap<String, String> {
        let mut files = HashMap::new();

        // 1. Generate types.rs (shared enums and structs)
        let mut b = CodeBuilder::new();
        types::generate_enum_definitions(&mut b, self.type_system);
        types::generate_struct_definitions(&mut b, self.type_system);
        let types_code = b.build();
        if !types_code.is_empty() {
            files.insert("types.rs".to_string(), types_code);
        }

        // 2. Collect icon names and widget features across all views before generating files
        let views = self
            .views
            .expect("Cannot generate project structure without views map");

        let mut all_icon_names: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
        let mut any_icons = false;
        let mut needs_image = false;
        let mut needs_svg = false;
        let mut needs_markdown = false;
        let mut needs_qr_code = false;

        for view_entry in views.values() {
            let root = view_entry.hierarchy.root();
            let icon_names = events::collect_all_icon_names(root);
            if !icon_names.is_empty() {
                any_icons = true;
                all_icon_names.extend(icon_names);
            }
            Self::scan_cargo_features(root, &mut needs_image, &mut needs_svg, &mut needs_markdown, &mut needs_qr_code);
        }

        // 3. Generate Cargo.toml, build.rs, icons.toml if icons are used
        if any_icons {
            files.insert("build.rs".to_string(), generate_build_rs());
            files.insert("icons.toml".to_string(), generate_icons_toml(&all_icon_names));
        }
        files.insert("Cargo.toml".to_string(), generate_cargo_toml(
            &self.app_name,
            any_icons,
            needs_image,
            needs_svg,
            needs_markdown,
            needs_qr_code,
        ));

        // 4. Pre-scan all views to find which modules are actually referenced by ViewReference widgets
        let mut all_referenced_modules: std::collections::HashSet<String> = std::collections::HashSet::new();
        for view_entry in views.values() {
            let root = view_entry.hierarchy.root();
            let refs = events::collect_view_refs(root, views);
            for vr in refs {
                all_referenced_modules.insert(vr.module_name);
            }
        }

        // 5. Pre-collect all non-main module names so main.rs sees them regardless of iteration order
        let all_non_main_modules: Vec<String> = views.values()
            .filter(|v| !v.is_main)
            .map(|v| to_snake_case(&v.name))
            .collect();

        // 6. Iterate through all views
        for view_entry in views.values() {
            self.prepare_for_view(&view_entry.hierarchy);

            let file_name = if view_entry.is_main {
                "main.rs".to_string()
            } else {
                format!("{}.rs", to_snake_case(&view_entry.name))
            };

            let struct_name = if view_entry.is_main {
                self.app_name.clone()
            } else {
                handle_whitespace(&to_pascal_case(&view_entry.name))
            };

            let code = self.generate_single_view_content(
                view_entry,
                &struct_name,
                files.contains_key("types.rs"),
                &all_non_main_modules,
                &all_referenced_modules,
                custom_styles,
            );
            files.insert(file_name, code);
        }

        files
    }

    fn scan_cargo_features(
        widget: &Widget,
        needs_image: &mut bool,
        needs_svg: &mut bool,
        needs_markdown: &mut bool,
        needs_qr_code: &mut bool,
    ) {
        match widget.widget_type {
            WidgetType::Image => *needs_image = true,
            WidgetType::Svg => *needs_svg = true,
            WidgetType::Markdown => *needs_markdown = true,
            WidgetType::QRCode => *needs_qr_code = true,
            _ => {}
        }
        for child in &widget.children {
            Self::scan_cargo_features(child, needs_image, needs_svg, needs_markdown, needs_qr_code);
        }
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
        all_referenced_modules: &std::collections::HashSet<String>,
        custom_styles: &CustomThemes,
    ) -> String {
        let mut b = CodeBuilder::new();
        let root = view_entry.hierarchy.root();
        let names = &self.widget_names;

        // Resolve ViewReference widgets for this view
        let view_refs = if let Some(views) = self.views {
            events::collect_view_refs(root, views)
        } else {
            Vec::new()
        };

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
        let tracker = events::generate_imports(&mut b, root);

        if has_types {
            if view_entry.is_main {
                b.line("mod types;");
                b.line("use types::*;");
            } else {
                b.line("use crate::types::*;");
            }
        }

        // If this is Main, mod only the referenced component files and add use aliases
        if view_entry.is_main {
            for module in modules_to_mod {
                if all_referenced_modules.contains(module) {
                    b.line(&format!("mod {};", module));
                }
            }
            for vr in &view_refs {
                b.line(&format!("use {}::{};", vr.module_name, vr.struct_name));
            }
            if !modules_to_mod.is_empty() || !view_refs.is_empty() {
                b.newline();
            }
        }

        // -- Message Enum --
        events::generate_message_enum(&mut b, root, names, self.type_system, &view_refs);
        b.newline();

        // -- Struct --
        app::generate_app_struct(&mut b, root, names, struct_name, self.type_system, &view_refs, view_entry.is_main);
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
            &view_refs,
        );
        b.newline();

        // -- Main Function (only for main view) --
        if view_entry.is_main {
            app::generate_main_function(
                &mut b,
                struct_name,
                Some(&window_config),
                tracker.uses_icon,
            );
        }

        b.build()
    }

    fn collect_widget_names(&mut self, widget: &Widget) {
        let derp: String;
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

fn generate_build_rs() -> String {
    r#"pub fn main() {
    println!("cargo::rerun-if-changed=fonts/icons.toml");
    iced_lucide::build("fonts/icons.toml").expect("Build icon module");
}
"#.to_string()
}

fn generate_icons_toml(icon_names: &std::collections::BTreeSet<String>) -> String {
    let mut out = String::from("module = \"icon\"\n\n[icons]\n");
    for name in icon_names {
        // Rust identifier uses underscores; Lucide icon names use hyphens
        let lucide_name = name.replace('_', "-");
        out.push_str(&format!("{} = \"{}\"\n", name, lucide_name));
    }
    out
}

fn generate_cargo_toml(
    app_name: &str,
    uses_icons: bool,
    needs_image: bool,
    needs_svg: bool,
    needs_markdown: bool,
    needs_qr_code: bool,
) -> String {
    let mut features = vec!["\"tokio\"", "\"lazy\"", "\"advanced\"", "\"debug\""];
    if needs_image    { features.push("\"image\""); }
    if needs_svg      { features.push("\"svg\""); }
    if needs_markdown { features.push("\"markdown\""); }
    if needs_qr_code  { features.push("\"qr_code\""); }

    let features_str = features
        .iter()
        .map(|f| format!("    {},", f))
        .collect::<Vec<_>>()
        .join("\n");

    let build_deps = if uses_icons {
        "\n[build-dependencies]\niced_lucide = { git = \"https://github.com/A-Disruption/iced_lucide\", branch = \"master\" }\n"
    } else {
        ""
    };

    format!(
        "[package]\nname = \"{}\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\niced = {{ version = \"0.14.0\", features = [\n{}\n] }}\n{}",
        to_snake_case(app_name),
        features_str,
        build_deps,
    )
}
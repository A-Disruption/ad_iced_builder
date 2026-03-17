use iced::{Theme, window};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use uuid::Uuid;

use crate::action_system::semantic::build_view_reference_index;
use crate::data_structures::types::types::*;
use crate::data_structures::widget_hierarchy::WidgetHierarchy;
use crate::enum_builder::TypeSystem;
use crate::views::theme_and_stylefn_builder::CustomThemes;
use crate::{Window, WindowEnum};
use builder::{CodeBuilder, handle_whitespace, sanitize_name, to_pascal_case, to_snake_case};

// Sub-modules
pub mod action_codegen;
pub mod app;
pub mod builder;
pub mod events;
pub(crate) mod helpers;
pub mod styles;
pub mod types;
pub mod view;
pub mod widgets;
pub mod window_settings;

pub use styles::{
    generate_all_styles_file, generate_button_style_code, generate_checkbox_style_code,
    generate_combo_box_style_code, generate_container_style_code, generate_menu_style_code,
    generate_pick_list_style_code, generate_progress_bar_style_code, generate_radio_style_code,
    generate_rule_style_code, generate_slider_style_code, generate_text_input_style_code,
    generate_toggler_style_code,
};

fn generated_view_type_name(view_name: &str) -> String {
    to_pascal_case(&handle_whitespace(view_name))
}

pub struct CodeGeneratorV2<'a> {
    views: Option<&'a BTreeMap<Uuid, AppView>>,
    windows: Option<&'a BTreeMap<window::Id, Window>>,
    current_hierarchy: Option<&'a WidgetHierarchy>,
    flows: &'a [crate::action_system::flow::AppFlow],
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
        flows: &'a [crate::action_system::flow::AppFlow],
    ) -> Self {
        Self {
            views: Some(views),
            windows: Some(windows),
            current_hierarchy: None,
            flows,
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
            flows: &[],
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
                .map(|c| {
                    if c.is_alphanumeric() || c == '_' {
                        c
                    } else {
                        '_'
                    }
                })
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
        widgets::generate_widget_code(
            &mut b,
            widget,
            &self.widget_names,
            false,
            custom_styles,
            self.type_system,
            &[],
        );
        b.build()
    }

    /// Generate a multi-file project structure. Returns filename -> code string.
    pub fn generate_project_structure(
        &mut self,
        custom_styles: &CustomThemes,
    ) -> HashMap<String, String> {
        let mut files = HashMap::new();

        // 2. Collect icon names and widget features across all views before generating files
        let views = self
            .views
            .expect("Cannot generate project structure without views map");

        // Detect whether any flow has NavigateToView nodes.
        let has_navigation = views.len() > 1
            && self.flows.iter().any(|f| {
                f.graph.nodes.iter().any(|n| {
                    matches!(
                        &n.kind,
                        crate::action_system::node_kinds::ActionNodeKind::NavigateToView { .. }
                    )
                })
            });

        // Build sorted view list for View enum (ordered by view.order).
        let view_variants: Vec<(Uuid, String)> = if has_navigation {
            let mut vs: Vec<_> = views.values().collect();
            vs.sort_by_key(|v| v.order);
            vs.iter().map(|v| (v.id, v.name.clone())).collect()
        } else {
            Vec::new()
        };

        // 1. Generate types.rs (shared enums and structs + View enum if navigation is used)
        let mut b = CodeBuilder::new();
        if has_navigation {
            types::generate_view_enum(&mut b, &view_variants);
            b.newline();
        }
        types::generate_enum_definitions(&mut b, self.type_system);
        types::generate_struct_definitions(&mut b, self.type_system);
        let types_code = b.build();
        if !types_code.is_empty() {
            files.insert("types.rs".to_string(), types_code);
        }

        let mut all_icon_names: std::collections::BTreeSet<String> =
            std::collections::BTreeSet::new();
        let mut any_icons = false;
        let mut needs_image = false;
        let mut needs_svg = false;
        let mut needs_markdown = false;
        let mut needs_qr_code = false;
        let mut widget_features = BTreeSet::new();

        for view_entry in views.values() {
            let root = view_entry.hierarchy.root();
            let icon_names = events::collect_all_icon_names(root);
            if !icon_names.is_empty() {
                any_icons = true;
                all_icon_names.extend(icon_names);
            }
            Self::scan_cargo_features(
                root,
                &mut needs_image,
                &mut needs_svg,
                &mut needs_markdown,
                &mut needs_qr_code,
                &mut widget_features,
            );
        }

        // 3. Generate Cargo.toml, build.rs, icons.toml if icons are used
        if any_icons {
            files.insert("build.rs".to_string(), generate_build_rs());
            files.insert(
                "icons.toml".to_string(),
                generate_icons_toml(&all_icon_names),
            );
        }
        files.insert(
            "Cargo.toml".to_string(),
            generate_cargo_toml(
                &self.app_name,
                any_icons,
                needs_image,
                needs_svg,
                needs_markdown,
                needs_qr_code,
                &widget_features,
            ),
        );

        // 3b. Generate styles.rs if any custom styles are defined
        let has_styles = if let Some(styles_code) = styles::generate_all_styles_file(custom_styles)
        {
            files.insert("styles.rs".to_string(), styles_code);
            true
        } else {
            false
        };

        // 4. Pre-scan all views to find which modules are actually referenced by ViewReference widgets
        let mut all_referenced_modules: std::collections::HashSet<String> =
            std::collections::HashSet::new();
        for view_entry in views.values() {
            let root = view_entry.hierarchy.root();
            let refs = events::collect_view_refs(root, views);
            for vr in refs {
                all_referenced_modules.insert(vr.module_name.clone());
                // Also include extra views (non-primary alternatives in multi-view ViewReference)
                for (_, em, _) in &vr.extra_views {
                    all_referenced_modules.insert(em.clone());
                }
            }
        }

        // 5. Pre-collect all non-main module names so main.rs sees them regardless of iteration order
        let all_non_main_modules: Vec<String> = views
            .values()
            .filter(|v| !v.is_main)
            .map(|v| to_snake_case(&v.name))
            .collect();

        // 5b. Pre-compute widget name maps for all views (needed for cross-view intercept codegen).
        let mut all_view_widget_names: std::collections::HashMap<
            uuid::Uuid,
            std::collections::HashMap<WidgetId, String>,
        > = std::collections::HashMap::new();
        for view_entry in views.values() {
            self.prepare_for_view(&view_entry.hierarchy);
            for (wid_u64, override_name) in &view_entry.widget_state_names {
                let wid = WidgetId(*wid_u64 as usize);
                if self.widget_names.contains_key(&wid) {
                    self.widget_names.insert(wid, override_name.clone());
                }
            }
            all_view_widget_names.insert(view_entry.id, self.widget_names.clone());
        }

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
                generated_view_type_name(&view_entry.name)
            };

            let code = self.generate_single_view_content(
                view_entry,
                &struct_name,
                files.contains_key("types.rs"),
                has_styles,
                &all_non_main_modules,
                &all_referenced_modules,
                custom_styles,
                &view_variants,
                has_navigation,
                &all_view_widget_names,
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
        widget_features: &mut BTreeSet<&'static str>,
    ) {
        match widget.widget_type {
            WidgetType::Image => *needs_image = true,
            WidgetType::Svg => *needs_svg = true,
            WidgetType::Markdown => *needs_markdown = true,
            WidgetType::QRCode => *needs_qr_code = true,
            WidgetType::Collapsible | WidgetType::CollapsibleGroup => {
                widget_features.insert("collapsible");
            }
            WidgetType::GenericOverlay => {
                widget_features.insert("generic_overlay");
            }
            WidgetType::DatePicker => {
                widget_features.insert("date_picker");
            }
            _ => {}
        }
        for child in &widget.children {
            Self::scan_cargo_features(
                child,
                needs_image,
                needs_svg,
                needs_markdown,
                needs_qr_code,
                widget_features,
            );
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
        has_styles: bool,
        modules_to_mod: &[String],
        all_referenced_modules: &std::collections::HashSet<String>,
        custom_styles: &CustomThemes,
        view_variants: &[(Uuid, String)],
        has_navigation: bool,
        all_view_widget_names: &std::collections::HashMap<
            uuid::Uuid,
            std::collections::HashMap<WidgetId, String>,
        >,
    ) -> String {
        // Apply widget_state_names overrides: user-renamed auto-state widget fields.
        for (wid_u64, override_name) in &view_entry.widget_state_names {
            let wid = WidgetId(*wid_u64 as usize);
            if self.widget_names.contains_key(&wid) {
                self.widget_names.insert(wid, override_name.clone());
            }
        }

        let mut b = CodeBuilder::new();
        let root = view_entry.hierarchy.root();
        let names = &self.widget_names;

        // Resolve ViewReference widgets for this view
        let view_refs = if let Some(views) = self.views {
            events::collect_view_refs(root, views)
        } else {
            Vec::new()
        };
        let semantic_view_reference_index =
            self.views.map(|views| build_view_reference_index(views));

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
        let tracker = events::generate_imports(&mut b, root, view_entry.is_main);

        if has_types {
            if view_entry.is_main {
                b.line("mod types;");
                b.line("use types::*;");
            } else {
                b.line("use crate::types::*;");
            }
        }

        if has_styles {
            if view_entry.is_main {
                b.line("mod styles;");
            } else {
                b.line("use crate::styles;");
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
                for (_, em, es) in &vr.extra_views {
                    b.line(&format!("use {}::{};", em, es));
                }
            }
            if !modules_to_mod.is_empty() || !view_refs.is_empty() {
                b.newline();
            }
        }

        // Build all_flows ref slice — app-level flows are now stored directly on self.flows.
        let all_flows_early: Vec<&crate::action_system::flow::AppFlow> =
            self.flows.iter().collect();
        let view_names: std::collections::HashMap<uuid::Uuid, String> =
            if let Some(views) = self.views {
                views.iter().map(|(id, v)| (*id, v.name.clone())).collect()
            } else {
                std::collections::HashMap::new()
            };
        let known_view_ids: std::collections::HashSet<uuid::Uuid> =
            view_names.keys().copied().collect();
        let emits_bubbled_navigation_message = !view_entry.is_main
            && events::view_requires_bubbled_navigation_message(
                view_entry.id,
                &all_flows_early,
                &known_view_ids,
                semantic_view_reference_index.as_ref(),
            );
        let bubbled_navigation_subviews: std::collections::HashSet<uuid::Uuid> = if view_entry
            .is_main
        {
            view_refs
                .iter()
                .flat_map(|vr| {
                    std::iter::once(vr.referenced_view_id).chain(vr.extra_view_ids.iter().copied())
                })
                .filter(|view_id| {
                    events::view_requires_bubbled_navigation_message(
                        *view_id,
                        &all_flows_early,
                        &known_view_ids,
                        semantic_view_reference_index.as_ref(),
                    )
                })
                .collect()
        } else {
            std::collections::HashSet::new()
        };

        // For the main view: compute intercepts for each sub-view's cross-parent SetState actions.
        // These let main intercept the original trigger message and set parent state directly.
        let sub_view_intercepts: std::collections::HashMap<
            uuid::Uuid,
            Vec<events::CrossViewIntercept>,
        > = if view_entry.is_main {
            view_refs
                .iter()
                .flat_map(|vr| {
                    let all_ids = std::iter::once(vr.referenced_view_id)
                        .chain(vr.extra_view_ids.iter().copied());
                    all_ids
                        .filter_map(|vid| {
                            let sv = self.views?.get(&vid)?;
                            let names_map = all_view_widget_names.get(&vid)?;
                            let intercepts = events::collect_cross_view_intercepts(
                                &all_flows_early,
                                vid,
                                sv.hierarchy.root(),
                                names_map,
                                names,
                                view_entry.id,
                                semantic_view_reference_index.as_ref(),
                                &events::build_view_reference_selection_index(
                                    view_entry.id,
                                    &view_refs,
                                ),
                            );
                            if intercepts.is_empty() {
                                None
                            } else {
                                Some((vid, intercepts))
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .collect()
        } else {
            std::collections::HashMap::new()
        };

        // -- Message Enum --
        events::generate_message_enum(
            &mut b,
            root,
            names,
            self.type_system,
            &view_refs,
            self.flows,
            view_entry.is_main,
            emits_bubbled_navigation_message,
        );
        // -- Selection Enums for multi-view ViewReference widgets --
        events::generate_view_selection_enums(&mut b, &view_refs);
        b.newline();

        // current_view field + initializer only in the main view's App struct.
        let initial_view_variant: Option<String> = if has_navigation && view_entry.is_main {
            Some(generated_view_type_name(&view_entry.name))
        } else {
            None
        };

        // -- Struct --
        app::generate_app_struct(
            &mut b,
            root,
            names,
            struct_name,
            self.type_system,
            &view_refs,
            view_entry.is_main,
            &view_entry.custom_state,
            initial_view_variant.as_deref(),
        );
        b.newline();

        // all_flows was already computed above as all_flows_early (needed for sub_view_intercepts).
        let all_flows = &all_flows_early;

        // -- Impl --
        app::generate_impl(
            &mut b,
            root,
            names,
            struct_name,
            view_entry.id,
            if view_entry.is_main {
                Some((&window_config, self.theme))
            } else {
                None
            },
            self.type_system,
            custom_styles,
            &view_refs,
            &view_names,
            all_view_widget_names,
            &view_entry.custom_state,
            self.flows,
            &all_flows,
            semantic_view_reference_index.as_ref(),
            initial_view_variant.as_deref(),
            emits_bubbled_navigation_message,
            &bubbled_navigation_subviews,
            &sub_view_intercepts,
        );
        b.newline();

        // -- Main Function (only for main view) --
        if view_entry.is_main {
            app::generate_main_function(
                &mut b,
                struct_name,
                Some(&window_config),
                tracker.uses_icon,
                self.flows,
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
                WidgetType::DatePicker => "date_picker",
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

#[cfg(test)]
mod tests {
    use super::generated_view_type_name;

    #[test]
    fn generated_view_type_name_handles_spaces_and_numbers() {
        assert_eq!(generated_view_type_name("view 2"), "View2");
        assert_eq!(generated_view_type_name("view 3"), "View3");
        assert_eq!(generated_view_type_name("view four"), "ViewFour");
        assert_eq!(generated_view_type_name("Login Form"), "LoginForm");
    }
}

fn generate_build_rs() -> String {
    r#"pub fn main() {
    println!("cargo::rerun-if-changed=fonts/icons.toml");
    iced_lucide::build("fonts/icons.toml").expect("Build icon module");
}
"#
    .to_string()
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
    widget_features: &BTreeSet<&'static str>,
) -> String {
    let mut features = vec!["\"tokio\"", "\"lazy\"", "\"advanced\"", "\"debug\""];
    if needs_image {
        features.push("\"image\"");
    }
    if needs_svg {
        features.push("\"svg\"");
    }
    if needs_markdown {
        features.push("\"markdown\"");
    }
    if needs_qr_code {
        features.push("\"qr_code\"");
    }

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

    let widgets_dep = if widget_features.is_empty() {
        String::new()
    } else {
        let features = widget_features
            .iter()
            .map(|feature| format!("\"{}\"", feature))
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "widgets = {{ git = \"https://github.com/A-Disruption/widgets.git\", features = [{}] }}\n",
            features
        )
    };

    format!(
        "[package]\nname = \"{}\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\niced = {{ version = \"0.14.0\", features = [\n{}\n] }}\n{}{}",
        to_snake_case(app_name),
        features_str,
        widgets_dep,
        build_deps,
    )
}

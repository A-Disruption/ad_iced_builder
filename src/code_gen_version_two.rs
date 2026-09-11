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
    generate_rule_style_code, generate_slider_style_code, generate_text_editor_style_code,
    generate_text_input_style_code, generate_toggler_style_code,
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

    /// The window the generated application opens.
    ///
    /// The builder's own Visualizer window stands in for it: that is the one
    /// showing the project, so its settings are the project's settings.
    fn generated_window_config(&self) -> WindowConfig {
        let fallback =
            || WindowConfig::new("Visualizer".to_string(), window::Settings::default());

        let Some(windows) = self.windows else {
            return fallback();
        };

        windows
            .values()
            .find(|window| window.windowtype == WindowEnum::Visualizer)
            .map_or_else(fallback, |window| window.config.clone())
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

        let mut all_icons: std::collections::BTreeSet<(String, String)> =
            std::collections::BTreeSet::new();
        let mut any_icons = false;
        let mut needs_image = false;
        let mut needs_svg = false;
        let mut needs_markdown = false;
        let mut needs_qr_code = false;
        let mut needs_iced_split = false;
        let mut widget_features = BTreeSet::new();

        for view_entry in views.values() {
            let root = view_entry.hierarchy.root();
            let icons = events::collect_all_icons(root);
            if !icons.is_empty() {
                any_icons = true;
                all_icons.extend(icons);
            }
            Self::scan_cargo_features(
                root,
                &mut needs_image,
                &mut needs_svg,
                &mut needs_markdown,
                &mut needs_qr_code,
                &mut needs_iced_split,
                &mut widget_features,
            );
        }

        // 3. Generate Cargo.toml, build.rs, icons.toml if icons are used
        if any_icons {
            files.insert("build.rs".to_string(), generate_build_rs());
            files.insert("icons.toml".to_string(), generate_icons_toml(&all_icons));
        }
        files.insert(
            "Cargo.toml".to_string(),
            generate_cargo_toml(
                &self.app_name,
                any_icons,
                &icon_features(&all_icons),
                self.generated_window_config().use_native_frame,
                needs_image,
                needs_svg,
                needs_markdown,
                needs_qr_code,
                needs_iced_split,
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
        needs_iced_split: &mut bool,
        widget_features: &mut BTreeSet<&'static str>,
    ) {
        match widget.widget_type {
            WidgetType::Image => *needs_image = true,
            WidgetType::Svg => *needs_svg = true,
            WidgetType::Markdown => *needs_markdown = true,
            WidgetType::QRCode => *needs_qr_code = true,
            WidgetType::Split => *needs_iced_split = true,
            WidgetType::Collapsible | WidgetType::CollapsibleGroup => {
                widget_features.insert("collapsible");
            }
            WidgetType::Popover => {
                widget_features.insert("popover");
            }
            WidgetType::Menu => {
                widget_features.insert("menu");
            }
            WidgetType::Layer => {
                widget_features.insert("layer");
            }
            WidgetType::Toast => {
                widget_features.insert("toast");
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
                needs_iced_split,
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

        let window_config = self.generated_window_config();

        // Only the main view is an application, so only it owns the window
        // the frame decorates.
        let uses_native_frame = view_entry.is_main && window_config.use_native_frame;

        // -- Imports --
        let tracker =
            events::generate_imports(&mut b, root, view_entry.is_main, uses_native_frame);

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
            uses_native_frame,
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
            uses_native_frame,
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
                WidgetType::TextEditor => "text_editor",
                WidgetType::TextInput => "text_input",
                WidgetType::Checkbox => "checkbox",
                WidgetType::Radio => "radio",
                WidgetType::Slider => "slider",
                WidgetType::VerticalSlider => "vertical_slider",
                WidgetType::Split => "split",
                WidgetType::Float => "float",
                WidgetType::Responsive => "responsive",
                WidgetType::Sensor => "sensor",
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
    use super::{generate_icons_toml, generated_view_type_name, icon_fn_name, icon_features};

    fn icons(entries: &[(&str, &str)]) -> std::collections::BTreeSet<(String, String)> {
        entries
            .iter()
            .map(|(family, name)| (family.to_string(), name.to_string()))
            .collect()
    }

    #[test]
    fn icon_fn_name_leaves_lucide_bare_and_prefixes_the_rest() {
        assert_eq!(icon_fn_name("lucide", "house"), "house");
        assert_eq!(icon_fn_name("lucide", "trash-2"), "trash_2");
        assert_eq!(icon_fn_name("fa-brands", "github"), "fa_brands_github");
        assert_eq!(
            icon_fn_name("simple-icons", "github"),
            "simple_icons_github"
        );
    }

    #[test]
    fn icons_toml_groups_by_family() {
        let toml = generate_icons_toml(&icons(&[
            ("lucide", "house"),
            ("lucide", "trash-2"),
            ("fa-brands", "github"),
        ]));

        assert!(toml.contains("module = \"icon\""));
        assert!(toml.contains("[icons.lucide]"));
        assert!(toml.contains("house = \"house\""));
        assert!(toml.contains("trash_2 = \"trash-2\""));
        assert!(toml.contains("[icons.fa-brands]"));
        assert!(toml.contains("fa_brands_github = \"github\""));
    }

    #[test]
    fn icon_features_omit_the_default_family() {
        assert!(icon_features(&icons(&[("lucide", "house")])).is_empty());

        let features = icon_features(&icons(&[
            ("lucide", "house"),
            ("fa-brands", "github"),
            ("fa-solid", "gear"),
            ("tabler", "home"),
        ]));

        // Font Awesome's three faces come from one feature.
        assert_eq!(
            features.into_iter().collect::<Vec<_>>(),
            vec!["fontawesome", "tabler"]
        );
    }

    /// Writes a generated project to `AD_ICED_BUILDER_DUMP_DIR`, when set.
    ///
    /// Set that and run this test to get a project laid out on disk that
    /// `cargo check` can be pointed at — which is the only way to know the
    /// generated code really compiles.
    fn dump_project(files: &std::collections::HashMap<String, String>) {
        let Ok(dir) = std::env::var("AD_ICED_BUILDER_DUMP_DIR") else {
            return;
        };

        let dir = std::path::Path::new(&dir);
        let src = dir.join("src");
        std::fs::create_dir_all(&src).unwrap();

        for (name, contents) in files {
            let path = match name.as_str() {
                "Cargo.toml" | "build.rs" => dir.join(name),
                "icons.toml" => {
                    std::fs::create_dir_all(dir.join("fonts")).unwrap();
                    dir.join("fonts").join(name)
                }
                _ => src.join(name),
            };

            std::fs::write(path, contents).unwrap();
        }
    }

    /// Generates a project drawing icons from two families.
    ///
    /// One `icon` widget and one `text_input` carrying an icon, which between
    /// them cover every place an icon reaches the generated code: the call
    /// site, the composed row that replaced `text_input::Icon`, `icons.toml`,
    /// and the `iced_lucide` features in `Cargo.toml`.
    fn generate_icon_project() -> std::collections::HashMap<String, String> {
        use super::CodeGeneratorV2;
        use crate::data_structures::types::types::{AppView, WidgetType};
        use crate::views::theme_and_stylefn_builder::CustomThemes;
        use crate::{Window, WindowEnum};

        let view_id = uuid::Uuid::new_v4();
        let mut view = AppView::with_id(view_id, "Main View".to_string(), 0);

        let root = view.hierarchy.root().id;
        let column = view
            .hierarchy
            .add_child(root, WidgetType::Column)
            .expect("a column goes in the root container");

        let icon = view
            .hierarchy
            .add_child(column, WidgetType::Icon)
            .expect("an icon goes in a column");
        let input = view
            .hierarchy
            .add_child(column, WidgetType::TextInput)
            .expect("a text input goes in a column");

        {
            let icon = view
                .hierarchy
                .get_widget_by_id_mut(icon)
                .expect("the icon was just added");

            icon.properties.icon_family = "fa-brands".to_string();
            icon.properties.icon_name = "github".to_string();
        }

        {
            let input = view
                .hierarchy
                .get_widget_by_id_mut(input)
                .expect("the input was just added");

            input.properties.text_input_icon_enabled = true;
            input.properties.text_input_icon_family = "lucide".to_string();
            input.properties.text_input_icon_name = "search".to_string();
            input.properties.text_input_icon_size = 16.0;
        }

        let views = std::collections::BTreeMap::from([(view_id, view)]);
        let windows = std::collections::BTreeMap::from([(
            iced::window::Id::unique(),
            Window::new(iced::window::Id::unique(), WindowEnum::Visualizer),
        )]);

        let theme = iced::Theme::Dark;
        let type_system = crate::enum_builder::TypeSystem::new();

        let mut generator = CodeGeneratorV2::new(&views, &windows, &theme, &type_system, &[]);
        let files = generator.generate_project_structure(&CustomThemes::new(&theme));

        dump_project(&files);
        files
    }

    #[test]
    fn icons_reach_every_corner_of_a_generated_project() {
        let files = generate_icon_project();
        let main = files.get("main.rs").expect("main.rs is generated");

        // The call sites, family-prefixed where the family is not Lucide.
        assert!(main.contains("icon::fa_brands_github()"));
        assert!(main.contains("icon::search().size(16.0)"));

        // iced master has no `text_input::Icon`, so the icon is composed into
        // a row beside the field instead.
        assert!(!main.contains("text_input::Icon"));
        assert!(main.contains("row!["));
        assert!(main.contains(".align_y(Alignment::Center)"));

        // Every font, rather than the single `FONT` a one-family module has.
        assert!(main.contains("icon::FONTS"));

        let icons_toml = files.get("icons.toml").expect("icons.toml is generated");
        assert!(icons_toml.contains("[icons.fa-brands]"));
        assert!(icons_toml.contains("fa_brands_github = \"github\""));
        assert!(icons_toml.contains("[icons.lucide]"));
        assert!(icons_toml.contains("search = \"search\""));

        let cargo = files.get("Cargo.toml").expect("Cargo.toml is generated");
        assert!(cargo.contains("iced_lucide = { git ="));
        assert!(cargo.contains("features = [\"fontawesome\"]"));
    }

    /// Generates a project using the two lengths iced master added.
    fn generate_length_project() -> std::collections::HashMap<String, String> {
        use super::CodeGeneratorV2;
        use crate::data_structures::types::types::{AppView, WidgetType};
        use crate::views::theme_and_stylefn_builder::CustomThemes;
        use crate::{Window, WindowEnum};
        use iced::Length;
        use iced::advanced::graphics::core::length::{Bounds, Constraint, Sizing};

        let view_id = uuid::Uuid::new_v4();
        let mut view = AppView::with_id(view_id, "Main View".to_string(), 0);

        let root = view.hierarchy.root().id;
        let column = view
            .hierarchy
            .add_child(root, WidgetType::Column)
            .expect("a column goes in the root container");

        let bounded = view
            .hierarchy
            .add_child(column, WidgetType::Text)
            .expect("text goes in a column");
        let fluid = view
            .hierarchy
            .add_child(column, WidgetType::Text)
            .expect("text goes in a column");

        {
            let bounded = view
                .hierarchy
                .get_widget_by_id_mut(bounded)
                .expect("just added");

            bounded.properties.width = Length::Bounded {
                bounds: Bounds::Both {
                    min: 100.0,
                    max: 400.0,
                },
                sizing: Sizing::Fill(2),
            };
            bounded.properties.height = Length::Bounded {
                bounds: Bounds::Max(50.0),
                sizing: Sizing::Fit,
            };
        }

        {
            let fluid = view
                .hierarchy
                .get_widget_by_id_mut(fluid)
                .expect("just added");

            fluid.properties.width = Length::Fluid(Constraint::Min(120.0));
            fluid.properties.height = Length::Fluid(Constraint::Max);
        }

        let views = std::collections::BTreeMap::from([(view_id, view)]);
        let windows = std::collections::BTreeMap::from([(
            iced::window::Id::unique(),
            Window::new(iced::window::Id::unique(), WindowEnum::Visualizer),
        )]);

        let theme = iced::Theme::Dark;
        let type_system = crate::enum_builder::TypeSystem::new();

        let mut generator = CodeGeneratorV2::new(&views, &windows, &theme, &type_system, &[]);
        let files = generator.generate_project_structure(&CustomThemes::new(&theme));

        dump_project(&files);
        files
    }

    #[test]
    fn bounded_and_fluid_lengths_generate() {
        let files = generate_length_project();
        let main = files.get("main.rs").expect("main.rs is generated");

        // `Bounded` goes through the `.min()` / `.max()` builders, so nothing
        // has to import `Bounds` or `Sizing`.
        assert!(main.contains("Length::FillPortion(2).min(100.0).max(400.0)"));
        assert!(main.contains("Length::Fit.max(50.0)"));

        // `Fluid` has no builder behind it and is written out as the variant,
        // which is what the extra import is for.
        assert!(main.contains("use iced::advanced::graphics::core::length::Constraint;"));
        assert!(main.contains("Length::Fluid(Constraint::Min(120.0))"));
        assert!(main.contains("Length::Fluid(Constraint::Max)"));
    }

    /// Generates a whole one-view project, with the custom frame on or off.
    fn generate_project(use_native_frame: bool) -> std::collections::HashMap<String, String> {
        use super::CodeGeneratorV2;
        use crate::data_structures::types::types::AppView;
        use crate::views::theme_and_stylefn_builder::CustomThemes;
        use crate::{Window, WindowEnum};

        let view_id = uuid::Uuid::new_v4();
        let views = std::collections::BTreeMap::from([(
            view_id,
            AppView::with_id(view_id, "Main View".to_string(), 0),
        )]);

        let mut window = Window::new(iced::window::Id::unique(), WindowEnum::Visualizer);
        window.config.use_native_frame = use_native_frame;

        let windows = std::collections::BTreeMap::from([(iced::window::Id::unique(), window)]);

        let theme = iced::Theme::Dark;
        let type_system = crate::enum_builder::TypeSystem::new();

        let mut generator = CodeGeneratorV2::new(&views, &windows, &theme, &type_system, &[]);

        generator.generate_project_structure(&CustomThemes::new(&theme))
    }

    #[test]
    fn native_frame_off_leaves_the_project_alone() {
        let files = generate_project(false);
        let main = files.get("main.rs").expect("main.rs is generated");

        assert!(!main.contains("NativeFrame"));
        assert!(!main.contains("Message::Frame"));
        assert!(
            !files
                .get("Cargo.toml")
                .expect("Cargo.toml is generated")
                .contains("iced-native-frame")
        );
    }

    #[test]
    fn native_frame_on_wires_every_end() {
        let files = generate_project(true);
        let main = files.get("main.rs").expect("main.rs is generated");

        assert!(main.contains("use iced_native_frame::{FrameAction, NativeFrame, NativeFrameConfig};"));
        assert!(main.contains("frame: NativeFrame,"));
        assert!(main.contains("frame: NativeFrame::new(NativeFrameConfig::platform_default()),"));
        assert!(main.contains("Frame(FrameAction),"));
        assert!(main.contains("Message::Frame(action) =>"));
        assert!(main.contains("let installed = app.frame.install_latest().discard();"));
        dump_project(&files);
        assert!(main.contains("self.frame.subscription().map(Message::Frame)"));
        assert!(main.contains("self.frame.decorate("));

        // `window_settings` is what turns the platform decorations off, so it
        // has to be generated and wired even when every other setting is
        // still at its default.
        assert!(main.contains("frame.window_settings(iced::window::Settings {"));
        assert!(main.contains(".window(App::window_settings())"));
        assert!(main.contains(".subscription(App::subscription)"));

        assert!(
            files
                .get("Cargo.toml")
                .expect("Cargo.toml is generated")
                .contains("iced-native-frame = { git =")
        );
    }

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

/// The Rust function name `iced_lucide` generates for an icon.
///
/// Lucide is the crate's default family and by far the common case, so its
/// icons keep their bare name — `icon::house()`. Everything else is prefixed
/// with its family, which is what keeps Font Awesome's `github` apart from
/// Simple Icons' in one module.
///
/// This has to be derivable from `(family, name)` alone: the widget writing
/// the call site and the code writing `icons.toml` never meet.
pub(crate) fn icon_fn_name(family: &str, name: &str) -> String {
    if family == "lucide" {
        iced_lucide::function_name(name)
    } else {
        format!(
            "{}_{}",
            iced_lucide::function_name(family),
            iced_lucide::function_name(name)
        )
    }
}

/// The `iced_lucide` definition file for a project's icons.
///
/// Icons are grouped under `[icons.<family>]`, which is how a definition names
/// a family for the bare names beneath it.
fn generate_icons_toml(icons: &std::collections::BTreeSet<(String, String)>) -> String {
    let mut by_family: BTreeMap<&str, Vec<&str>> = BTreeMap::new();

    for (family, name) in icons {
        by_family
            .entry(family.as_str())
            .or_default()
            .push(name.as_str());
    }

    let mut out = String::from("# Generated by ad_iced_builder.\nmodule = \"icon\"\n");

    for (family, names) in by_family {
        let license = crate::icon_lucide::FAMILIES
            .iter()
            .find(|candidate| candidate.id == family)
            .map(|family| format!(" — {}, {}", family.name, family.license))
            .unwrap_or_default();

        out.push_str(&format!("\n# {family}{license}\n[icons.{family}]\n"));

        for name in names {
            out.push_str(&format!("{} = \"{}\"\n", icon_fn_name(family, name), name));
        }
    }

    out
}

/// The `iced_lucide` Cargo features a project's icons need.
///
/// Lucide is on by default, so it is never listed.
fn icon_features(icons: &std::collections::BTreeSet<(String, String)>) -> BTreeSet<&'static str> {
    icons
        .iter()
        .filter_map(|(family, _)| {
            crate::icon_lucide::FAMILIES
                .iter()
                .find(|candidate| candidate.id == family)
        })
        .map(|family| family.feature)
        .filter(|feature| *feature != "lucide")
        .collect()
}

fn generate_cargo_toml(
    app_name: &str,
    uses_icons: bool,
    icon_features: &BTreeSet<&'static str>,
    uses_native_frame: bool,
    needs_image: bool,
    needs_svg: bool,
    needs_markdown: bool,
    needs_qr_code: bool,
    needs_iced_split: bool,
    widget_features: &BTreeSet<&'static str>,
) -> String {
    // `lazy` is not on this list: iced dropped the feature on the way to 0.15,
    // and asking for it fails resolution outright.
    let mut features = vec!["\"tokio\"", "\"advanced\"", "\"debug\""];
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

    // Only the families this project actually draws from are enabled, so the
    // build subsets those fonts and no others.
    let build_deps = if uses_icons {
        let features = if icon_features.is_empty() {
            String::new()
        } else {
            let features = icon_features
                .iter()
                .map(|feature| format!("\"{feature}\""))
                .collect::<Vec<_>>()
                .join(", ");

            format!(", features = [{features}]")
        };

        format!(
            "\n[build-dependencies]\niced_lucide = {{ git = \"https://github.com/A-Disruption/iced_lucide\", branch = \"master\"{features} }}\n"
        )
    } else {
        String::new()
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
    // The published `iced_split` is pinned to `iced_core` 0.14 and only exists
    // at 0.1.0, so generated projects take the same fork the builder does.
    let iced_split_dep = if needs_iced_split {
        "iced_split = { git = \"https://github.com/A-Disruption/iced_split.git\" }\n"
    } else {
        ""
    };

    let native_frame_dep = if uses_native_frame {
        "iced-native-frame = { git = \"https://github.com/A-Disruption/iced-window-decorations.git\" }\n"
    } else {
        ""
    };

    // iced 0.15 is unreleased, so a `[patch.crates-io]` entry is not optional:
    // cargo cannot resolve `0.15.0-dev` from crates.io without it, and every
    // crate in the graph asking for that version has to resolve to the same
    // checkout for `Element` and `Theme` to be one type across all of them.
    //
    // The revision is pinned rather than tracking `branch = "master"`, and has
    // to be: master moves, and a project that follows it will one day resolve
    // an iced the widget crates no longer compile against. `ICED_REVISION` is
    // whatever this builder itself resolved — see `build.rs`.
    let patch = format!(
        "\n[patch.crates-io]\n\
         # The revision this project was generated against. Bump it when the \
         crates below move.\n\
         iced = {{ git = \"https://github.com/iced-rs/iced.git\", rev = \"{}\" }}\n",
        env!("ICED_REVISION"),
    );

    format!(
        "[package]\nname = \"{}\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\niced = {{ version = \"0.15.0-dev\", features = [\n{}\n] }}\n{}{}{}{}{}",
        to_snake_case(app_name),
        features_str,
        iced_split_dep,
        native_frame_dep,
        widgets_dep,
        build_deps,
        patch,
    )
}

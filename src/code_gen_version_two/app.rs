use super::builder::{CodeBuilder, to_pascal_case, to_snake_case};
use super::{events, view};
use super::window_settings::{generate_window_settings, window_settings_are_default};
use crate::data_structures::types::types::{Widget, WidgetType, WidgetId, WindowConfig};
use crate::views::theme_and_stylefn_builder::CustomThemes;
use crate::enum_builder::TypeSystem;
use iced::Theme;
use std::collections::HashMap;

pub fn generate_app_struct(
    b: &mut CodeBuilder,
    root: &Widget,
    names: &HashMap<WidgetId, String>,
    struct_name: &str,
    type_system: &TypeSystem,
) {
    b.newline();
    b.line(&format!("struct {} {{", struct_name));
    b.increase_indent();
    generate_state_fields(b, root, names, type_system);
    b.decrease_indent();
    b.line("}");
}

pub fn generate_impl(
    b: &mut CodeBuilder,
    root: &Widget,
    names: &HashMap<WidgetId, String>,
    struct_name: &str,
    main_config: Option<(&WindowConfig, &Theme)>,
    type_system: &TypeSystem,
    custom_styles: &CustomThemes,
) {
    b.line(&format!("impl {} {{", struct_name));
    b.increase_indent();

    generate_new_method(b, root, names, type_system);
    b.newline();

    // Only generate title, theme, and window settings if this is the Main App
    if let Some((window_config, theme)) = main_config {
        generate_title_method(b, &window_config.title);
        b.newline();
        generate_theme_method(b, theme);
        b.newline();
        generate_window_settings(b, window_config);
    }

    events::generate_update(b, root, names);
    b.newline();
    view::generate_view_method(b, root, names, custom_styles);

    b.newline();
    b.decrease_indent();
    b.line("}");
}

fn generate_state_fields(
    b: &mut CodeBuilder,
    widget: &Widget,
    names: &HashMap<WidgetId, String>,
    type_system: &TypeSystem,
) {
    let name = names
        .get(&widget.id)
        .unwrap_or(&"widget".to_string())
        .clone();
    let props = &widget.properties;

    match widget.widget_type {
        WidgetType::TextInput => {
            b.line(&format!("{}_value: String,", to_snake_case(&name)));
        }
        WidgetType::Checkbox => {
            b.line(&format!("{}_checked: bool,", to_snake_case(&name)));
        }
        WidgetType::Radio => {
            b.line(&format!("{}_selected: usize,", to_snake_case(&name)));
        }
        WidgetType::Slider | WidgetType::VerticalSlider => {
            b.line(&format!("{}_value: f32,", to_snake_case(&name)));
        }
        WidgetType::Toggler => {
            b.line(&format!("{}_active: bool,", to_snake_case(&name)));
        }
        WidgetType::PickList => {
            b.line(&format!(
                "{}_selected: Option<String>,",
                to_snake_case(&name)
            ));
        }
        WidgetType::ComboBox => {
            if let Some(ref enum_id) = props.referenced_enum {
                if let Some(enum_def) = type_system.get_enum(enum_id.clone()) {
                    b.line(&format!(
                        "{}_value: {},",
                        to_snake_case(&name),
                        enum_def.name
                    ));
                    b.line(&format!(
                        "{}_state: combo_box::State<{}>,",
                        to_snake_case(&name),
                        enum_def.name
                    ));
                    return;
                }
            }
            b.line(&format!("{}_value: String,", to_snake_case(&name)));
            b.line(&format!(
                "{}_state: combo_box::State<String>,",
                to_snake_case(&name)
            ));
        }
        WidgetType::QRCode => {
            b.line("qr_data: qr_code::Data,");
        }
        _ => {}
    }

    for child in &widget.children {
        generate_state_fields(b, child, names, type_system);
    }
}

fn generate_new_method(
    b: &mut CodeBuilder,
    root: &Widget,
    names: &HashMap<WidgetId, String>,
    type_system: &TypeSystem,
) {
    b.line("pub fn new() -> (Self, Task<Message>) {");
    b.increase_indent();

    b.line("(");
    b.increase_indent();
    b.line("Self {");
    b.increase_indent();

    generate_state_initializers(b, root, names, type_system);

    b.decrease_indent();
    b.line("},");
    b.line("Task::none()");

    b.decrease_indent();
    b.line(")");

    b.decrease_indent();
    b.line("}");
}

fn generate_state_initializers(
    b: &mut CodeBuilder,
    widget: &Widget,
    names: &HashMap<WidgetId, String>,
    type_system: &TypeSystem,
) {
    let props = &widget.properties;
    let name = names
        .get(&widget.id)
        .unwrap_or(&"widget".to_string())
        .clone();

    match widget.widget_type {
        WidgetType::TextInput => {
            b.line(&format!(
                "{}_value: String::new(),",
                to_snake_case(&name)
            ));
        }
        WidgetType::Checkbox => {
            b.line(&format!(
                "{}_checked: {},",
                to_snake_case(&name),
                if props.checkbox_checked { "true" } else { "false" }
            ));
        }
        WidgetType::Radio => {
            b.line(&format!(
                "{}_selected: {},",
                to_snake_case(&name),
                props.radio_selected_index
            ));
        }
        WidgetType::Slider | WidgetType::VerticalSlider => {
            b.line(&format!(
                "{}_value: {:.1},",
                to_snake_case(&name),
                props.slider_value
            ));
        }
        WidgetType::Toggler => {
            b.line(&format!(
                "{}_active: {},",
                to_snake_case(&name),
                if props.toggler_active { "true" } else { "false" }
            ));
        }
        WidgetType::PickList => {
            b.line(&format!("{}_selected: None,", to_snake_case(&name)));
        }
        WidgetType::ComboBox => {
            if let Some(ref enum_id) = props.referenced_enum {
                if let Some(enum_def) = type_system.get_enum(enum_id.clone()) {
                    b.line(&format!(
                        "{}_value: {}::{},",
                        to_snake_case(&name),
                        enum_def.name,
                        enum_def.variants[0].name
                    ));
                    b.line(&format!(
                        "{}_state: combo_box::State::new({}::ALL.to_vec()),",
                        to_snake_case(&name),
                        enum_def.name
                    ));
                    return;
                }
            }
            // String-based combo box
            b.line(&format!(
                "{}_value: String::new(),",
                to_snake_case(&name)
            ));
            let options: Vec<String> = props
                .combobox_options
                .iter()
                .map(|o| format!("\"{}\".to_string()", o))
                .collect();
            b.line(&format!(
                "{}_state: combo_box::State::new(vec![{}]),",
                to_snake_case(&name),
                options.join(", ")
            ));
        }
        WidgetType::QRCode => {
            b.line(&format!(
                "qr_data: qr_code::Data::new(\"{}\").unwrap(),",
                props.qrcode_link
            ));
        }
        _ => {}
    }

    for child in &widget.children {
        generate_state_initializers(b, child, names, type_system);
    }
}

fn generate_title_method(b: &mut CodeBuilder, window_title: &str) {
    b.line("fn title(&self) -> String {");
    b.increase_indent();
    b.line(&format!("String::from(\"{}\")", window_title));
    b.decrease_indent();
    b.line("}");
}

fn generate_theme_method(b: &mut CodeBuilder, theme: &Theme) {
    b.line("fn theme(&self) -> Theme {");
    b.increase_indent();

    let variant = match theme {
        Theme::Light => "Light",
        Theme::Dark => "Dark",
        Theme::Dracula => "Dracula",
        Theme::Nord => "Nord",
        Theme::SolarizedLight => "SolarizedLight",
        Theme::SolarizedDark => "SolarizedDark",
        Theme::GruvboxLight => "GruvboxLight",
        Theme::GruvboxDark => "GruvboxDark",
        Theme::CatppuccinLatte => "CatppuccinLatte",
        Theme::CatppuccinFrappe => "CatppuccinFrappe",
        Theme::CatppuccinMacchiato => "CatppuccinMacchiato",
        Theme::CatppuccinMocha => "CatppuccinMocha",
        Theme::TokyoNight => "TokyoNight",
        Theme::TokyoNightStorm => "TokyoNightStorm",
        Theme::TokyoNightLight => "TokyoNightLight",
        Theme::KanagawaWave => "KanagawaWave",
        Theme::KanagawaDragon => "KanagawaDragon",
        Theme::KanagawaLotus => "KanagawaLotus",
        Theme::Moonfly => "Moonfly",
        Theme::Nightfly => "Nightfly",
        Theme::Oxocarbon => "Oxocarbon",
        Theme::Ferra => "Ferra",
        _ => "Dark",
    };

    b.line(&format!("Theme::{}", variant));
    b.decrease_indent();
    b.line("}");
}

pub fn generate_main_function(
    b: &mut CodeBuilder,
    struct_name: &str,
    window_config: Option<&WindowConfig>,
) {
    b.line("pub fn main() -> iced::Result {");
    b.increase_indent();

    b.indent();
    b.push(&format!(
        "iced::application({}::new, {}::update, {}::view)",
        struct_name, struct_name, struct_name
    ));
    b.newline();

    b.increase_indent();
    if let Some(window_config) = window_config {
        if !window_settings_are_default(&window_config.settings) {
            b.indent();
            b.push(&format!(".window({}::window_settings())", struct_name));
            b.newline();
        }
    }

    b.indent();
    b.push(&format!(".theme({}::theme)", struct_name));
    b.newline();

    b.indent();
    b.push(&format!(".title({}::title)", struct_name));
    b.newline();

    b.indent();
    b.push(".run()");
    b.newline();

    b.decrease_indent();
    b.decrease_indent();
    b.line("}");
}

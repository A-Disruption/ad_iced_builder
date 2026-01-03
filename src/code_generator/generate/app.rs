use iced::Theme;
use crate::code_generator::writer::{CodeWriter, to_snake_case};
use crate::code_generator::generate::{view, events};
use crate::data_structures::types::types::{Widget, WidgetType, WidgetId};
use crate::views::theme_and_stylefn_builder::CustomThemes;
use crate::enum_builder::TypeSystem;
use std::collections::HashMap;

pub fn generate_app_struct(
    writer: &mut CodeWriter, 
    root: &Widget, 
    names: &HashMap<WidgetId, String>,
    struct_name: &str,
    type_system: &TypeSystem,
) {
    writer.add_newline();
    writer.add_keyword("struct");
    writer.add_plain(" ");
    writer.add_type(struct_name);
    writer.add_plain(" {");
    writer.add_newline();
    
    writer.increase_indent();
    generate_state_fields(writer, root, names, type_system);
    writer.decrease_indent();
    
    writer.add_plain("}");
    writer.add_newline();
}

pub fn generate_impl(
    writer: &mut CodeWriter, 
    root: &Widget, 
    names: &HashMap<WidgetId, String>,
    struct_name: &str,
    // Optional tuple: (window_title, theme). If None, standard component impl.
    main_config: Option<(&str, &Theme)>, 
    type_system: &TypeSystem,
    custom_styles: &CustomThemes,
) {
    writer.add_keyword("impl");
    writer.add_plain(" ");
    writer.add_type(struct_name);
    writer.add_plain(" {");
    writer.add_newline();
    writer.increase_indent();

    generate_new_method(writer, root, names, type_system);

    // Only generate title and theme if this is the Main App
    if let Some((window_title, theme)) = main_config {
        generate_title_method(writer, window_title);
        generate_theme_method(writer, theme);
    }

    events::generate_update(writer, root, names);
    view::generate_view_method(writer, root, names, custom_styles);

    writer.add_newline();
    writer.decrease_indent();
    writer.add_plain("}");
    writer.add_newline();
}

fn generate_state_fields(writer: &mut CodeWriter, widget: &Widget, names: &HashMap<WidgetId, String>, type_system: &TypeSystem) {
    let name = names.get(&widget.id).unwrap_or(&"widget".to_string()).clone();
    let props = &widget.properties;

    match widget.widget_type {
        WidgetType::TextInput => {
            writer.add_indent();
            writer.add_identifier(&format!("{}_value", to_snake_case(&name)));
            writer.add_operator(":");
            writer.add_plain(" ");
            writer.add_type("String");
            writer.add_plain(",");
            writer.add_newline();
        }
        WidgetType::Checkbox => {
            writer.add_indent();
            writer.add_identifier(&format!("{}_checked", to_snake_case(&name)));
            writer.add_operator(":");
            writer.add_plain(" ");
            writer.add_type("bool");
            writer.add_plain(",");
            writer.add_newline();
        }
        WidgetType::Radio => {
            writer.add_indent();
            writer.add_identifier(&format!("{}_selected", to_snake_case(&name)));
            writer.add_operator(":");
            writer.add_plain(" ");
            writer.add_type("usize");
            writer.add_plain(",");
            writer.add_newline();
        }
        WidgetType::Slider | WidgetType::VerticalSlider => {
            writer.add_indent();
            writer.add_identifier(&format!("{}_value", to_snake_case(&name)));
            writer.add_operator(":");
            writer.add_plain(" ");
            writer.add_type("f32");
            writer.add_plain(",");
            writer.add_newline();
        }
        WidgetType::Toggler => {
            writer.add_indent();
            writer.add_identifier(&format!("{}_active", to_snake_case(&name)));
            writer.add_operator(":");
            writer.add_plain(" ");
            writer.add_type("bool");
            writer.add_plain(",");
            writer.add_newline();
        }
        WidgetType::PickList => {
            writer.add_indent();
            writer.add_identifier(&format!("{}_selected", to_snake_case(&name)));
            writer.add_operator(":");
            writer.add_plain(" ");
            writer.add_type("Option");
            writer.add_operator("<");
            writer.add_type("String");
            writer.add_operator(">");
            writer.add_plain(",");
            writer.add_newline();
        }
        WidgetType::ComboBox => {
            if let Some(ref enum_id) = props.referenced_enum {
                if let Some(enum_def) = type_system.get_enum(enum_id.clone()) {
                    // Enum-based combo box
                    writer.add_indent();
                    writer.add_identifier(&format!("{}_value", to_snake_case(&name)));
                    writer.add_operator(":");
                    writer.add_plain(" ");
                    writer.add_type(&enum_def.name);
                    writer.add_plain(",");
                    writer.add_newline();
                    
                    writer.add_indent();
                    writer.add_identifier(&format!("{}_state", to_snake_case(&name)));
                    writer.add_operator(":");
                    writer.add_plain(" ");
                    writer.add_type("combo_box::State");
                    writer.add_operator("<");
                    writer.add_type(&enum_def.name);
                    writer.add_operator(">");
                    writer.add_plain(",");
                    writer.add_newline();
                    return;
                }
            }
            
            // String-based combo box (existing code)
            writer.add_indent();
            writer.add_identifier(&format!("{}_value", to_snake_case(&name)));
            writer.add_operator(":");
            writer.add_plain(" ");
            writer.add_type("String");
            writer.add_plain(",");
            writer.add_newline();
            
            writer.add_indent();
            writer.add_identifier(&format!("{}_state", to_snake_case(&name)));
            writer.add_operator(":");
            writer.add_plain(" ");
            writer.add_type("combo_box::State");
            writer.add_operator("<");
            writer.add_type("String");
            writer.add_operator(">");
            writer.add_plain(",");
            writer.add_newline();
        }
        WidgetType::QRCode => {
            writer.add_indent();
            writer.add_identifier("qr_data");
            writer.add_operator(":");
            writer.add_plain(" ");
            writer.add_type("qr_code::Data");
            writer.add_plain(",");
            writer.add_newline();            
        }
        _ => {}
    }

    for child in &widget.children {
        generate_state_fields(writer, child, names, type_system);
    }
}

fn generate_new_method(writer: &mut CodeWriter, root: &Widget, names: &HashMap<WidgetId, String>, type_system: &TypeSystem) {
    writer.add_indent();
    writer.add_keyword("pub fn"); // Make new pub so main.rs can call it
    writer.add_plain(" ");
    writer.add_function("new");
    writer.add_plain("() ");
    writer.add_operator("->");
    writer.add_plain(" (");
    writer.add_keyword("Self");
    writer.add_plain( ", " );
    writer.add_type("Task");
    writer.add_plain("<");
    writer.add_type("Message");
    writer.add_plain( ">) {" );
    writer.add_newline();
    writer.increase_indent();
    
    writer.add_indent();
    writer.add_plain("(");
    writer.add_newline();
    writer.increase_indent();
    
    writer.add_indent();
    writer.add_keyword("Self");
    writer.add_plain(" {");
    writer.add_newline();
    writer.increase_indent();
    
    generate_state_initializers(writer, root, names, type_system);
    
    writer.decrease_indent();
    writer.add_indent();
    writer.add_plain("},");
    writer.add_newline();
    
    writer.add_indent();
    writer.add_type("Task");
    writer.add_operator("::");
    writer.add_plain("none()");
    writer.add_newline();
    
    writer.decrease_indent();
    writer.add_indent();
    writer.add_plain(")");
    writer.add_newline();
    
    writer.decrease_indent();
    writer.add_indent();
    writer.add_plain("}");
    writer.add_newline();
}

fn generate_state_initializers(writer: &mut CodeWriter, widget: &Widget, names: &HashMap<WidgetId, String>, type_system: &TypeSystem) {
    let props = &widget.properties;
    let name = names.get(&widget.id).unwrap_or(&"widget".to_string()).clone();

    match widget.widget_type {
        WidgetType::TextInput => {
            writer.add_indent();
            writer.add_identifier(&format!("{}_value", to_snake_case(&name)));
            writer.add_operator(":");
            writer.add_plain(" String::new(),");
            writer.add_newline();
        }
        WidgetType::Checkbox => {
            writer.add_indent();
            writer.add_identifier(&format!("{}_checked", to_snake_case(&name)));
            writer.add_operator(":");
            writer.add_plain(" ");
            writer.add_keyword(if props.checkbox_checked { "true" } else { "false" });
            writer.add_plain(",");
            writer.add_newline();
        }
        WidgetType::Radio => {
            writer.add_indent();
            writer.add_identifier(&format!("{}_selected", to_snake_case(&name)));
            writer.add_operator(":");
            writer.add_plain(" ");
            writer.add_number(&format!("{}", props.radio_selected_index));
            writer.add_plain(",");
            writer.add_newline();
        }
        WidgetType::Slider | WidgetType::VerticalSlider => {
            writer.add_indent();
            writer.add_identifier(&format!("{}_value", to_snake_case(&name)));
            writer.add_operator(":");
            writer.add_plain(" ");
            writer.add_number(&format!("{:.1}", props.slider_value));
            writer.add_plain(",");
            writer.add_newline();
        }
        WidgetType::Toggler => {
            writer.add_indent();
            writer.add_identifier(&format!("{}_active", to_snake_case(&name)));
            writer.add_operator(":");
            writer.add_plain(" ");
            writer.add_keyword(if props.toggler_active { "true" } else { "false" });
            writer.add_plain(",");
            writer.add_newline();
        }
        WidgetType::PickList => {
            writer.add_indent();
            writer.add_identifier(&format!("{}_selected", to_snake_case(&name)));
            writer.add_operator(":");
            writer.add_plain(" None,");
            writer.add_newline();
        }
        WidgetType::ComboBox => {
            // Get the enum definition and initialize properly
            if let Some(ref enum_id) = props.referenced_enum {
                if let Some(enum_def) = type_system.get_enum(enum_id.clone()) {
                    writer.add_indent();
                    writer.add_identifier(&format!("{}_value", to_snake_case(&name)));
                    writer.add_operator(":");
                    writer.add_plain(" ");
                    writer.add_type(&enum_def.name);
                    writer.add_operator("::");
                    writer.add_plain(&enum_def.variants[0].name);
                    writer.add_plain(",");
                    writer.add_newline();
                    
                    // Initialize state with all variants
                    writer.add_indent();
                    writer.add_identifier(&format!("{}_state", to_snake_case(&name)));
                    writer.add_operator(":");
                    writer.add_plain(" ");
                    writer.add_type("combo_box::State");
                    writer.add_operator("::");
                    writer.add_function("new");
                    writer.add_plain("(");
                    writer.add_type(&enum_def.name);
                    writer.add_operator("::");
                    writer.add_plain("ALL.to_vec()");
                    writer.add_plain("),");
                    writer.add_newline();  
                }
            } else {
                writer.add_indent();
                writer.add_identifier(&format!("{}_value", to_snake_case(&name)));
                writer.add_operator(":");
                writer.add_plain(" String::new(),");
                writer.add_newline();
                
                writer.add_indent();
                writer.add_identifier(&format!("{}_state", to_snake_case(&name)));
                writer.add_operator(":");
                writer.add_plain(" ");
                writer.add_type("combo_box::State");
                writer.add_operator("::");
                writer.add_function("new");
                writer.add_plain("(vec![");
                for (i, option) in props.combobox_options.iter().enumerate() {
                    writer.add_string(&format!("\"{}\"", option));
                    writer.add_operator(".");
                    writer.add_function("to_string");
                    writer.add_plain("()");
                    if i < props.combobox_options.len() - 1 {
                        writer.add_plain(", ");
                    }
                }
                writer.add_plain("]),");
                writer.add_newline();
            }
        }
        WidgetType::QRCode => {
            writer.add_indent();
            writer.add_identifier("qr_data");
            writer.add_operator(":");
            writer.add_plain(" ");
            writer.add_type("qr_code::Data");
            writer.add_operator("::");
            writer.add_function("new");
            writer.add_plain("(");
            writer.add_string(&format!("\"{}\"", props.qrcode_link));
            writer.add_plain(")");
            writer.add_operator(".");
            writer.add_function("unwrap");
            writer.add_plain("()");
            writer.add_plain(",");
            writer.add_newline();                 
        }
        _ => {}
    }
    
    for child in &widget.children {
        generate_state_initializers(writer, child, names, type_system);
    }
}

fn generate_title_method(writer: &mut CodeWriter, window_title: &str) {
    writer.add_indent();
    writer.add_keyword("fn");
    writer.add_plain(" ");
    writer.add_function("title");
    writer.add_plain("(");
    writer.add_operator("&");
    writer.add_keyword("self");
    writer.add_plain(") ");
    writer.add_operator("->");
    writer.add_plain(" ");
    writer.add_type("String");
    writer.add_plain(" {");
    writer.add_newline();
    writer.increase_indent();
    
    writer.add_indent();
    writer.add_type("String");
    writer.add_operator("::");
    writer.add_function("from");
    writer.add_plain("(\"");
    writer.add_string(window_title);
    writer.add_plain("\")");
    writer.add_newline();
    
    writer.decrease_indent();
    writer.add_indent();
    writer.add_plain("}");
    writer.add_newline();
}

fn generate_theme_method(writer: &mut CodeWriter, theme: &Theme) {
    writer.add_indent();
    writer.add_keyword("fn");
    writer.add_plain(" ");
    writer.add_function("theme");
    writer.add_plain("(");
    writer.add_operator("&");
    writer.add_keyword("self");
    writer.add_plain(") ");
    writer.add_operator("->");
    writer.add_plain(" ");
    writer.add_type("Theme");
    writer.add_plain(" {");
    writer.add_newline();
    writer.increase_indent();
    
    writer.add_indent();
    writer.add_type("Theme");
    writer.add_operator("::");
    // Use the actual theme passed in
    match theme {
        Theme::Light => writer.add_plain("Light"),
        Theme::Dark => writer.add_plain("Dark"),
        Theme::Dracula => writer.add_plain("Dracula"),
        Theme::Nord => writer.add_plain("Nord"),
        Theme::SolarizedLight => writer.add_plain("SolarizedLight"),
        Theme::SolarizedDark => writer.add_plain("SolarizedDark"),
        Theme::GruvboxLight => writer.add_plain("GruvboxLight"),
        Theme::GruvboxDark => writer.add_plain("GruvboxDark"),
        Theme::CatppuccinLatte => writer.add_plain("CatppuccinLatte"),
        Theme::CatppuccinFrappe => writer.add_plain("CatppuccinFrappe"),
        Theme::CatppuccinMacchiato => writer.add_plain("CatppuccinMacchiato"),
        Theme::CatppuccinMocha => writer.add_plain("CatppuccinMocha"),
        Theme::TokyoNight => writer.add_plain("TokyoNight"),
        Theme::TokyoNightStorm => writer.add_plain("TokyoNightStorm"),
        Theme::TokyoNightLight => writer.add_plain("TokyoNightLight"),
        Theme::KanagawaWave => writer.add_plain("KanagawaWave"),
        Theme::KanagawaDragon => writer.add_plain("KanagawaDragon"),
        Theme::KanagawaLotus => writer.add_plain("KanagawaLotus"),
        Theme::Moonfly => writer.add_plain("Moonfly"),
        Theme::Nightfly => writer.add_plain("Nightfly"),
        Theme::Oxocarbon => writer.add_plain("Oxocarbon"),
        Theme::Ferra => writer.add_plain("Ferra"),
        _ => writer.add_plain("Dark"), // Default fallback for custom themes
    }
    writer.add_newline();
    
    writer.decrease_indent();
    writer.add_indent();
    writer.add_plain("}");
    writer.add_newline();
}

pub fn generate_main_function(writer: &mut CodeWriter, struct_name: &str) {
    writer.add_keyword("pub fn");
    writer.add_plain(" ");
    writer.add_function("main");
    writer.add_plain("() ");
    writer.add_operator("->");
    writer.add_plain(" iced::Result {");
    writer.add_newline();
    writer.increase_indent();
    
    writer.add_indent();
    writer.add_plain("iced::application(");
    writer.add_type(struct_name);
    writer.add_operator("::");
    writer.add_plain("new, ");
    writer.add_type(struct_name);
    writer.add_operator("::");
    writer.add_plain("update, ");
    writer.add_type(struct_name);
    writer.add_operator("::");
    writer.add_plain("view)");
    writer.add_newline();
    
    writer.increase_indent();
    writer.add_indent();
    writer.add_operator(".");
    writer.add_function("theme");
    writer.add_plain("(");
    writer.add_type(struct_name);
    writer.add_operator("::");
    writer.add_plain("theme)");
    writer.add_newline();
    
    writer.add_indent();
    writer.add_operator(".");
    writer.add_function("title");
    writer.add_plain("(");
    writer.add_type(struct_name);
    writer.add_operator("::");
    writer.add_plain("title)");
    writer.add_newline();
    
    writer.add_indent();
    writer.add_operator(".");
    writer.add_function("run");
    writer.add_plain("()");
    writer.add_newline();
    
    writer.decrease_indent();
    writer.decrease_indent();
    writer.add_plain("}");
}
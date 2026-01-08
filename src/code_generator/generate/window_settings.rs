use crate::code_generator::writer::CodeWriter;
use crate::data_structures::types::types::WindowConfig;
use iced::window;

pub fn generate_window_settings(writer: &mut CodeWriter, config: &WindowConfig) {
    let default_settings = window::Settings::default();
    let mut fill_defaults = false;

    if window_settings_are_default(&config.settings) {
        return
    }

    writer.add_indent();
    writer.add_keyword("pub fn ");
    writer.add_function("window_settings");
    writer.add_plain("()");
    writer.add_operator(" -> ");
    writer.add_number("iced");
    writer.add_operator("::");
    writer.add_number("window");
    writer.add_operator("::");
    writer.add_type("Settings");
    writer.add_plain(" {");
    writer.add_newline();
    writer.increase_indent();
    writer.add_indent();

    writer.add_number("iced");
    writer.add_operator("::");
    writer.add_number("window");
    writer.add_operator("::");
    writer.add_type("Settings");
    writer.add_plain(" {");
    writer.add_newline();
    writer.increase_indent();


    // Size
    if !(config.settings.size == default_settings.size) {
        writer.add_indent();
        writer.add_plain("size");
        writer.add_operator(":");
        writer.add_number(" iced");
        writer.add_operator("::");
        writer.add_type("Size");
        writer.add_operator("::");
        writer.add_function("new");
        writer.add_type("(");
        writer.add_number(format!("{:.1}", config.settings.size.width).as_str());
        writer.add_plain(", ");
        writer.add_number(format!("{:.1}", config.settings.size.height).as_str());
        writer.add_type(")");
        writer.add_plain(",");
        writer.add_newline();
    } else { fill_defaults = true; }

    // Maximized
    if !(config.settings.maximized == default_settings.maximized) {
        writer.add_indent();
        writer.add_plain("maximized");
        writer.add_operator(":");
        writer.add_number(format!("{}", config.settings.maximized).as_str());
        writer.add_plain(",");
        writer.add_newline();
    } else { fill_defaults = true; }

    // Fullscreen
    if !(config.settings.fullscreen == default_settings.fullscreen) {
        writer.add_indent();
        writer.add_plain("fullscreen");
        writer.add_operator(":");
        writer.add_number(format!("{}", config.settings.fullscreen).as_str());
        writer.add_plain(",");
        writer.add_newline();
    } else { fill_defaults = true; }

    // Window Position
    match config.settings.position {
        window::Position::Centered => {
            writer.add_indent();
            writer.add_plain("settings.position = ");
            writer.add_number("window");
            writer.add_operator("::");
            writer.add_type("Position");
            writer.add_operator("::");
            writer.add_plain("Centered");
            writer.add_plain(";");
            writer.add_newline();
        }
        window::Position::Specific(position) => {
            writer.add_indent();
            writer.add_plain("settings.position = ");
            writer.add_number("window");
            writer.add_operator("::");
            writer.add_type("Position");
            writer.add_operator("::");
            writer.add_plain("Specific");
            writer.add_plain("(");
            writer.add_type("Point::new");
            writer.add_plain("(");
            writer.add_number(&position.x.to_string());
            writer.add_number(&position.y.to_string());
            writer.add_plain("));");
            writer.add_newline();
        }
        _ => { fill_defaults = true; } // Default is fine     
    }
    
    // Min Size
    if !(config.settings.min_size == default_settings.min_size) {
        writer.add_indent();
        writer.add_plain("min_size");
        writer.add_operator(":");
        writer.add_plain(" Some");
        writer.add_plain("(");
        writer.add_type(" Size");
        writer.add_operator("::");
        writer.add_function("new");
        writer.add_type("(");
        writer.add_number(format!("{:.1}", config.settings.min_size.unwrap().width).as_str());
        writer.add_plain(", ");
        writer.add_number(format!("{:.1}", config.settings.min_size.unwrap().height).as_str());
        writer.add_type(")");
        writer.add_plain(" )");
        writer.add_plain(",");
        writer.add_newline();
    } else { fill_defaults = true; }


    // Max Size
    if !(config.settings.max_size == default_settings.max_size) {
        writer.add_indent();
        writer.add_plain("max_size");
        writer.add_operator(":");
        writer.add_plain(" Some");
        writer.add_plain("(");
        writer.add_type(" Size");
        writer.add_operator("::");
        writer.add_function("new");
        writer.add_type("(");
        writer.add_number(format!("{:.1}", config.settings.max_size.unwrap().width).as_str());
        writer.add_plain(", ");
        writer.add_number(format!("{:.1}", config.settings.max_size.unwrap().height).as_str());
        writer.add_type(")");
        writer.add_plain(" )");
        writer.add_plain(",");
        writer.add_newline();
    } else { fill_defaults = true; }

    // Visible
    if !(config.settings.visible == default_settings.visible) {
        writer.add_indent();
        writer.add_plain("visible");
        writer.add_operator(": ");
        writer.add_number(config.settings.visible.to_string().as_str());
        writer.add_plain(",");
        writer.add_newline();
    } else { fill_defaults = true; }

    // Resizable
    if !(config.settings.resizable == default_settings.resizable) {
        writer.add_indent();
        writer.add_plain("resizable");
        writer.add_operator(": ");
        writer.add_number(config.settings.resizable.to_string().as_str());
        writer.add_plain(",");
        writer.add_newline();
    } else { fill_defaults = true; }

    // Minimizable
    if !(config.settings.minimizable == default_settings.minimizable) {
        writer.add_indent();
        writer.add_plain("minimizable");
        writer.add_operator(": ");
        writer.add_number(config.settings.minimizable.to_string().as_str());
        writer.add_plain(",");
        writer.add_newline();
    } else { fill_defaults = true; }

    // Closeable
    if !(config.settings.closeable == default_settings.closeable) {
        writer.add_indent();
        writer.add_plain("closeable");
        writer.add_operator(": ");
        writer.add_number(config.settings.closeable.to_string().as_str());
        writer.add_plain(",");
        writer.add_newline();
    } else { fill_defaults = true; }

    // Decorations
    if !(config.settings.decorations == default_settings.decorations) {
        writer.add_indent();
        writer.add_plain("decorations");
        writer.add_operator(": ");
        writer.add_number(config.settings.decorations.to_string().as_str());
        writer.add_plain(",");
        writer.add_newline();
    } else { fill_defaults = true; }

    // Transparent
    if !(config.settings.transparent == default_settings.transparent) {
        writer.add_indent();
        writer.add_plain("transparent");
        writer.add_operator(": ");
        writer.add_number(config.settings.transparent.to_string().as_str());
        writer.add_plain(",");
        writer.add_newline();
    } else { fill_defaults = true; }

    // Blur
    if !(config.settings.blur == default_settings.blur) {
        writer.add_indent();
        writer.add_plain("blur");
        writer.add_operator(": ");
        writer.add_number(config.settings.blur.to_string().as_str());
        writer.add_plain(",");
        writer.add_newline();
    } else { fill_defaults = true; }

    // Level
    if !(config.settings.level == default_settings.level) {

    }  else { fill_defaults = true; }

    // Icon
    if !config.settings.icon.is_none() {

    }  else { fill_defaults = true; }

    // Exit on close request
    if !(config.settings.exit_on_close_request == default_settings.exit_on_close_request) {
        writer.add_indent();
        writer.add_plain("exit_on_close_request");
        writer.add_operator(": ");
        writer.add_number(config.settings.exit_on_close_request.to_string().as_str());
        writer.add_plain(",");
        writer.add_newline();
    } else { fill_defaults = true; }

    // Platform Specific
    if !(config.settings.platform_specific == default_settings.platform_specific) {

    }  else { fill_defaults = true; }

    // Use Defaults if any are not set
    if fill_defaults {
        writer.add_indent();
        writer.add_operator("..");
        writer.add_plain("iced");
        writer.add_operator("::");
        writer.add_number("window");
        writer.add_operator("::");
        writer.add_type("Settings");
        writer.add_operator("::");
        writer.add_function("default");
        writer.add_type("()");
        writer.add_newline();
    }

    writer.decrease_indent();
    writer.add_indent();
    writer.add_plain("}");
    writer.add_newline();

    writer.decrease_indent();
    writer.add_indent();
    writer.add_plain("}");
    writer.add_newline();
    writer.add_newline();
}


pub fn window_settings_are_default(config: &window::Settings) -> bool {
    let default_settings = window::Settings::default();

    if config.size == default_settings.size &&
        config.maximized == default_settings.maximized &&
        config.fullscreen == default_settings.fullscreen &&
        config.min_size == default_settings.min_size &&
        config.max_size == default_settings.max_size &&
        config.visible == default_settings.visible &&
        config.resizable == default_settings.resizable &&
        config.minimizable == default_settings.minimizable &&
        config.closeable == default_settings.closeable &&
        config.decorations == default_settings.decorations &&
        config.transparent == default_settings.transparent &&
        config.blur == default_settings.blur &&
        config.level == default_settings.level &&
        config.icon.is_none() &&
        config.exit_on_close_request == default_settings.exit_on_close_request &&
        config.platform_specific == default_settings.platform_specific
    { 
        match config.position {
            window::Position::Default  => true,
            _ => { false }
    } 

    } else { false }
}
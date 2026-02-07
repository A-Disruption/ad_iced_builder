use super::builder::CodeBuilder;
use crate::data_structures::types::types::WindowConfig;
use iced::window;

pub fn generate_window_settings(b: &mut CodeBuilder, config: &WindowConfig) {
    let default_settings = window::Settings::default();
    let mut fill_defaults = false;

    if window_settings_are_default(&config.settings) {
        return;
    }

    b.line("pub fn window_settings() -> iced::window::Settings {");
    b.increase_indent();
    b.line("iced::window::Settings {");
    b.increase_indent();

    if config.settings.size != default_settings.size {
        b.line(&format!(
            "size: iced::Size::new({:.1}, {:.1}),",
            config.settings.size.width, config.settings.size.height
        ));
    } else {
        fill_defaults = true;
    }

    if config.settings.maximized != default_settings.maximized {
        b.line(&format!("maximized: {},", config.settings.maximized));
    } else {
        fill_defaults = true;
    }

    if config.settings.fullscreen != default_settings.fullscreen {
        b.line(&format!("fullscreen: {},", config.settings.fullscreen));
    } else {
        fill_defaults = true;
    }

    match config.settings.position {
        window::Position::Centered => {
            b.line("position: window::Position::Centered,");
        }
        window::Position::Specific(position) => {
            b.line(&format!(
                "position: window::Position::Specific(Point::new({}, {})),",
                position.x, position.y
            ));
        }
        _ => {
            fill_defaults = true;
        }
    }

    if config.settings.min_size != default_settings.min_size {
        if let Some(min) = config.settings.min_size {
            b.line(&format!(
                "min_size: Some(Size::new({:.1}, {:.1})),",
                min.width, min.height
            ));
        }
    } else {
        fill_defaults = true;
    }

    if config.settings.max_size != default_settings.max_size {
        if let Some(max) = config.settings.max_size {
            b.line(&format!(
                "max_size: Some(Size::new({:.1}, {:.1})),",
                max.width, max.height
            ));
        }
    } else {
        fill_defaults = true;
    }

    if config.settings.visible != default_settings.visible {
        b.line(&format!("visible: {},", config.settings.visible));
    } else {
        fill_defaults = true;
    }

    if config.settings.resizable != default_settings.resizable {
        b.line(&format!("resizable: {},", config.settings.resizable));
    } else {
        fill_defaults = true;
    }

    if config.settings.minimizable != default_settings.minimizable {
        b.line(&format!("minimizable: {},", config.settings.minimizable));
    } else {
        fill_defaults = true;
    }

    if config.settings.closeable != default_settings.closeable {
        b.line(&format!("closeable: {},", config.settings.closeable));
    } else {
        fill_defaults = true;
    }

    if config.settings.decorations != default_settings.decorations {
        b.line(&format!("decorations: {},", config.settings.decorations));
    } else {
        fill_defaults = true;
    }

    if config.settings.transparent != default_settings.transparent {
        b.line(&format!("transparent: {},", config.settings.transparent));
    } else {
        fill_defaults = true;
    }

    if config.settings.blur != default_settings.blur {
        b.line(&format!("blur: {},", config.settings.blur));
    } else {
        fill_defaults = true;
    }

    // Level (not yet implemented)
    if config.settings.level != default_settings.level {
        // TODO: level support
    } else {
        fill_defaults = true;
    }

    // Icon (not yet implemented)
    if config.settings.icon.is_some() {
        // TODO: icon support
    } else {
        fill_defaults = true;
    }

    if config.settings.exit_on_close_request != default_settings.exit_on_close_request {
        b.line(&format!(
            "exit_on_close_request: {},",
            config.settings.exit_on_close_request
        ));
    } else {
        fill_defaults = true;
    }

    // Platform Specific (not yet implemented)
    if config.settings.platform_specific != default_settings.platform_specific {
        // TODO: platform_specific support
    } else {
        fill_defaults = true;
    }

    // Use defaults if any fields are not set
    if fill_defaults {
        b.line("..iced::window::Settings::default()");
    }

    b.decrease_indent();
    b.line("}");
    b.decrease_indent();
    b.line("}");
    b.newline();
}

pub fn window_settings_are_default(config: &window::Settings) -> bool {
    let default_settings = window::Settings::default();

    if config.size == default_settings.size
        && config.maximized == default_settings.maximized
        && config.fullscreen == default_settings.fullscreen
        && config.min_size == default_settings.min_size
        && config.max_size == default_settings.max_size
        && config.visible == default_settings.visible
        && config.resizable == default_settings.resizable
        && config.minimizable == default_settings.minimizable
        && config.closeable == default_settings.closeable
        && config.decorations == default_settings.decorations
        && config.transparent == default_settings.transparent
        && config.blur == default_settings.blur
        && config.level == default_settings.level
        && config.icon.is_none()
        && config.exit_on_close_request == default_settings.exit_on_close_request
        && config.platform_specific == default_settings.platform_specific
    {
        matches!(config.position, window::Position::Default)
    } else {
        false
    }
}

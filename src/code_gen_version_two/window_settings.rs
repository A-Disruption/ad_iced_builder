use super::builder::CodeBuilder;
use crate::data_structures::types::types::{NativeFrameSettings, WindowConfig};
use iced::window;

pub fn generate_window_settings(b: &mut CodeBuilder, config: &WindowConfig) {
    let default_settings = window::Settings::default();
    let mut fill_defaults = false;

    if window_settings_are_default(&config.settings) && !config.use_native_frame {
        return;
    }

    b.line("pub fn window_settings() -> iced::window::Settings {");
    b.increase_indent();

    // `NativeFrame::window_settings` is the one helper that replaces every
    // platform `#[cfg]` a consumer would otherwise write: undecorated on
    // Windows and Linux, decorated on macOS where the traffic lights stay.
    if config.use_native_frame {
        b.line("let frame = NativeFrame::new(Self::frame_config());");
        b.newline();
        b.line("frame.window_settings(iced::window::Settings {");
    } else {
        b.line("iced::window::Settings {");
    }

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
    b.line(if config.use_native_frame { "})" } else { "}" });
    b.decrease_indent();
    b.line("}");
    b.newline();
}

/// The custom title bar's configuration, as a method on the app.
///
/// Both `window_settings` and the frame itself need it, and it is long enough
/// that writing it out twice would be worse than naming it once. Only the
/// fields that differ from `platform_default` are written, so the generated
/// code says what the project chose rather than restating the platform.
pub fn generate_frame_config(b: &mut CodeBuilder, config: &WindowConfig) {
    let frame = config.native_frame;
    let base = NativeFrameSettings::default();

    b.line("fn frame_config() -> NativeFrameConfig {");
    b.increase_indent();
    b.line("NativeFrameConfig {");
    b.increase_indent();
    b.line("decoration_mode: DecorationMode::Custom,");

    let mut field = |name: &str, value: String, differs: bool| {
        if differs {
            b.line(&format!("{name}: {value},"));
        }
    };

    field(
        "title_bar_height",
        format!("{:.1}", frame.title_bar_height),
        frame.title_bar_height != base.title_bar_height,
    );
    field(
        "caption_button_width",
        format!("{:.1}", frame.caption_button_width),
        frame.caption_button_width != base.caption_button_width,
    );
    field(
        "caption_buttons",
        frame.caption_buttons.to_string(),
        frame.caption_buttons != base.caption_buttons,
    );
    field(
        "resize_border",
        format!("{:.1}", frame.resize_border),
        frame.resize_border != base.resize_border,
    );
    field(
        "resizable",
        frame.resizable.to_string(),
        frame.resizable != base.resizable,
    );
    field(
        "corner_radius",
        format!("{:.1}", frame.corner_radius),
        frame.corner_radius != base.corner_radius,
    );
    field(
        "frame_border",
        frame.frame_border.to_string(),
        frame.frame_border != base.frame_border,
    );
    field(
        "client_shadow",
        frame.client_shadow.to_string(),
        frame.client_shadow != base.client_shadow,
    );
    field(
        "outer_padding",
        format!("{:.1}", frame.outer_padding),
        frame.outer_padding != base.outer_padding,
    );
    field(
        "native_rounding",
        frame.native_rounding.to_string(),
        frame.native_rounding != base.native_rounding,
    );
    field(
        "native_shadow",
        frame.native_shadow.to_string(),
        frame.native_shadow != base.native_shadow,
    );
    field(
        "window_icon_size",
        format!("{:.1}", frame.window_icon_size),
        frame.window_icon_size != base.window_icon_size,
    );
    field(
        "title_spacing",
        format!("{:.1}", frame.title_spacing),
        frame.title_spacing != base.title_spacing,
    );
    field(
        "title_padding",
        format!("{:.1}", frame.title_padding),
        frame.title_padding != base.title_padding,
    );
    field(
        "leading_inset",
        format!("{:.1}", frame.leading_inset),
        frame.leading_inset != base.leading_inset,
    );
    field(
        "show_title",
        frame.show_title.to_string(),
        frame.show_title != base.show_title,
    );

    // The platform defaults differ per target, so this is the rest of them
    // rather than the struct's own `Default`.
    b.line("..NativeFrameConfig::platform_default()");
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

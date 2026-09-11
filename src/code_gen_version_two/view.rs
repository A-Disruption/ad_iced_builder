use super::builder::CodeBuilder;
use super::events::ViewRefInfo;
use super::widgets::generate_widget_code;
use crate::data_structures::types::types::{Widget, WidgetId};
use crate::enum_builder::TypeSystem;
use crate::views::theme_and_stylefn_builder::CustomThemes;
use std::collections::HashMap;

pub fn generate_view_method(
    b: &mut CodeBuilder,
    root: &Widget,
    names: &HashMap<WidgetId, String>,
    custom_styles: &CustomThemes,
    type_system: &TypeSystem,
    view_refs: &[ViewRefInfo],
    native_frame_title: Option<&str>,
) {
    b.line("pub fn view<'a>(&'a self) -> Element<'a, Message> {");
    b.increase_indent();

    if native_frame_title.is_some() {
        b.line("let content: Element<'a, Message> =");
        b.increase_indent();
    }

    // The root is always a Container widget with user-configured properties (width, height, style, etc.)
    // Generate it directly so all its properties are applied.
    generate_widget_code(b, root, names, true, custom_styles, type_system, view_refs);

    b.push(".into()");

    if let Some(title) = native_frame_title {
        b.push(";");
        b.newline();
        b.decrease_indent();
        b.newline();

        // `decorate` is the single-window form of `view`: it looks up the one
        // window the frame was installed on, so no `window::Id` is named here.
        b.line(&format!(
            "self.frame.decorate(\"{}\", content, Message::Frame)",
            super::app::escape_rust_string_literal(title)
        ));
    } else {
        b.newline();
    }

    b.decrease_indent();
    b.line("}");
}

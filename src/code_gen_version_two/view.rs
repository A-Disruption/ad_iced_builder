use super::builder::CodeBuilder;
use super::events::{TitleBarInfo, ViewRefInfo};
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
    title_bar: Option<&TitleBarInfo>,
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

        let title = super::app::escape_rust_string_literal(title);

        match title_bar {
            // `decorate` is the single-window shorthand, but it passes no
            // title-bar content — so a project with content takes the long
            // form, looking the window up the same way `decorate` does.
            Some(title_bar) => {
                b.line(&format!(
                    "let title_bar = self.{}.view().map(Message::{});",
                    title_bar.field_name,
                    title_bar.message_variant()
                ));
                b.newline();
                b.line("match self.frame.primary_window() {");
                b.increase_indent();
                b.line("Some(window) => self.frame.view(");
                b.increase_indent();
                b.line("window,");
                b.line(&format!("\"{title}\","));
                b.line("None,");
                b.line("Some(title_bar),");
                b.line("content,");
                b.line("Message::Frame,");
                b.decrease_indent();
                b.line("),");
                b.line("None => content,");
                b.decrease_indent();
                b.line("}");
            }

            None => {
                b.line(&format!(
                    "self.frame.decorate(\"{title}\", content, Message::Frame)"
                ));
            }
        }
    } else {
        b.newline();
    }

    b.decrease_indent();
    b.line("}");
}

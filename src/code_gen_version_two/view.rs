use super::builder::CodeBuilder;
use super::events::ViewRefInfo;
use super::widgets::generate_widget_code;
use crate::data_structures::types::types::{Widget, WidgetId};
use crate::views::theme_and_stylefn_builder::CustomThemes;
use crate::enum_builder::TypeSystem;
use std::collections::HashMap;

pub fn generate_view_method(
    b: &mut CodeBuilder,
    root: &Widget,
    names: &HashMap<WidgetId, String>,
    custom_styles: &CustomThemes,
    type_system: &TypeSystem,
    view_refs: &[ViewRefInfo],
) {
    b.line("pub fn view<'a>(&'a self) -> Element<'a, Message> {");
    b.increase_indent();

    // The root is always a Container widget with user-configured properties (width, height, style, etc.)
    // Generate it directly so all its properties are applied.
    generate_widget_code(b, root, names, true, custom_styles, type_system, view_refs);

    b.push(".into()");
    b.newline();

    b.decrease_indent();
    b.line("}");
}

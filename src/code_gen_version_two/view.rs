use super::builder::CodeBuilder;
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
) {
    b.line("pub fn view<'a>(&'a self) -> Element<'a, Message> {");
    b.increase_indent();

    if root.children.is_empty() {
        b.indent();
        b.push("container(text(\"Empty\"))");
    } else {
        b.indent();
        b.push("container(");
        b.newline();
        b.increase_indent();

        root.children.iter().for_each(|widget|
            generate_widget_code(b, widget, names, true, custom_styles, type_system)
        );

        b.decrease_indent();
        b.newline();
        b.indent();
        b.push(")");
    }

    b.push(".into()");
    b.newline();

    b.decrease_indent();
    b.line("}");
}

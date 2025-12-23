use crate::code_generator::writer::{CodeWriter, to_pascal_case, to_snake_case};
use crate::data_structures::types::types::{Widget, WidgetType, WidgetId};
use crate::code_generator::generate::widgets::generate_widget_code;
use std::collections::HashMap;

pub fn generate_view_method(writer: &mut CodeWriter, root: &Widget, names: &HashMap<WidgetId, String>, ) {
    writer.add_indent();
    writer.add_keyword("pub fn");
    writer.add_plain(" ");
    writer.add_function("view");
    writer.add_plain("<");
    writer.add_operator("'a");
    writer.add_plain(">");
    writer.add_plain("(");
    writer.add_operator("&'a ");
    writer.add_type("self");
    writer.add_plain(")");
    writer.add_operator(" -> ");
    writer.add_type("Element");
    writer.add_plain("<");
    writer.add_operator("'a");
    writer.add_plain(", ");
    writer.add_operator("Message");
    writer.add_plain("> {");
    writer.add_newline();
    writer.increase_indent();

    if root.children.is_empty() {
        writer.add_indent();
        writer.add_function("container");
        writer.add_plain("(");
        writer.add_function("text");
        writer.add_plain("(");
        writer.add_string("\"Empty\"");
        writer.add_plain("))");
    } else {
        writer.add_indent();
        writer.add_function("container");
        writer.add_plain("(");
        writer.add_newline();
        writer.increase_indent();

        root.children.iter().for_each(|widget|
            generate_widget_code(writer, widget, names, true)
        );
        
        writer.decrease_indent();
        writer.add_newline();
        writer.add_indent();
        writer.add_plain(")");
    }

    writer.add_operator(".");
    writer.add_function("into");
    writer.add_plain("()");
    writer.add_newline();

    writer.decrease_indent();
    writer.add_indent();
    writer.add_plain("}");
}
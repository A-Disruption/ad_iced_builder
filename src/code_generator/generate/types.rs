use crate::code_generator::writer::{CodeWriter, to_pascal_case, to_snake_case};
use crate::data_structures::types::types::{Widget, WidgetType, WidgetId};
use crate::enum_builder::EnumDef;
use crate::enum_builder::TypeSystem;

pub fn generate_enum_definitions(writer: &mut CodeWriter, type_system: &TypeSystem) {
    for enum_def in type_system.enums.values() {
        generate_enum_code(writer, enum_def);
        writer.add_newline();
        writer.add_newline();
    }
}

fn generate_enum_code(writer: &mut CodeWriter, enum_def: &EnumDef) {
    writer.add_comment(&format!("// {} enum", enum_def.name));
    writer.add_newline();
    writer.add_plain("#[derive(Debug, Clone, Copy, PartialEq, Eq)]");
    writer.add_newline();
    writer.add_keyword("pub enum");
    writer.add_plain(" ");
    writer.add_type(&enum_def.name);
    writer.add_plain(" {");
    writer.add_newline();
    writer.increase_indent();
    
    for variant in &enum_def.variants {
        writer.add_indent();
        writer.add_plain(&variant.name);
        writer.add_plain(",");
        writer.add_newline();
    }
    
    writer.decrease_indent();
    writer.add_plain("}");
    writer.add_newline();
    writer.add_newline();
    
    // Generate Display impl
    generate_enum_display_impl(writer, enum_def);
    writer.add_newline();
    
    // Generate ALL constant for combo_box
    generate_enum_all_const(writer, enum_def);
}

fn generate_enum_display_impl(writer: &mut CodeWriter, enum_def: &EnumDef) {
    writer.add_keyword("impl");
    writer.add_plain(" std::fmt::Display ");
    writer.add_keyword("for");
    writer.add_plain(" ");
    writer.add_type(&enum_def.name);
    writer.add_plain(" {");
    writer.add_newline();
    writer.increase_indent();
    
    writer.add_indent();
    writer.add_keyword("fn");
    writer.add_plain(" ");
    writer.add_function("fmt");
    writer.add_plain("(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {");
    writer.add_newline();
    writer.increase_indent();
    
    writer.add_indent();
    writer.add_keyword("match");
    writer.add_plain(" self {");
    writer.add_newline();
    writer.increase_indent();
    
    for variant in &enum_def.variants {
        writer.add_indent();
        writer.add_type(&enum_def.name);
        writer.add_operator("::");
        writer.add_plain(&variant.name);
        writer.add_plain(" => write!(f, ");
        writer.add_string(&format!("\"{}\"", &variant.name));
        writer.add_plain("),");
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
    
    writer.decrease_indent();
    writer.add_plain("}");
}

fn generate_enum_all_const(writer: &mut CodeWriter, enum_def: &EnumDef) {
    writer.add_keyword("impl");
    writer.add_plain(" ");
    writer.add_type(&enum_def.name);
    writer.add_plain(" {");
    writer.add_newline();
    writer.increase_indent();
    
    writer.add_indent();
    writer.add_keyword("pub const");
    writer.add_plain(" ALL: &'static [Self] = &[");
    writer.add_newline();
    writer.increase_indent();
    
    for variant in &enum_def.variants {
        writer.add_indent();
        writer.add_plain("Self::");
        writer.add_plain(&variant.name);
        writer.add_plain(",");
        writer.add_newline();
    }
    
    writer.decrease_indent();
    writer.add_indent();
    writer.add_plain("];");
    writer.add_newline();
    
    writer.decrease_indent();
    writer.add_plain("}");
}
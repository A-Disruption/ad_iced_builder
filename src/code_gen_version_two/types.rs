use super::builder::CodeBuilder;
use crate::enum_builder::{EnumDef, TypeSystem};

pub fn generate_enum_definitions(b: &mut CodeBuilder, type_system: &TypeSystem) {
    for enum_def in type_system.enums.values() {
        generate_enum_code(b, enum_def);
        b.newline();
        b.newline();
    }
}

fn generate_enum_code(b: &mut CodeBuilder, enum_def: &EnumDef) {
    b.line(&format!("// {} enum", enum_def.name));
    b.line("#[derive(Debug, Clone, Copy, PartialEq, Eq)]");
    b.line(&format!("pub enum {} {{", enum_def.name));
    b.increase_indent();

    for variant in &enum_def.variants {
        b.line(&format!("{},", variant.name));
    }

    b.decrease_indent();
    b.line("}");
    b.newline();

    generate_enum_display_impl(b, enum_def);
    b.newline();

    generate_enum_all_const(b, enum_def);
}

fn generate_enum_display_impl(b: &mut CodeBuilder, enum_def: &EnumDef) {
    b.line(&format!("impl std::fmt::Display for {} {{", enum_def.name));
    b.increase_indent();

    b.line("fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {");
    b.increase_indent();

    b.line("match self {");
    b.increase_indent();

    for variant in &enum_def.variants {
        b.line(&format!(
            "{}::{} => write!(f, \"{}\"),",
            enum_def.name, variant.name, variant.name
        ));
    }

    b.decrease_indent();
    b.line("}");

    b.decrease_indent();
    b.line("}");

    b.decrease_indent();
    b.line("}");
}

fn generate_enum_all_const(b: &mut CodeBuilder, enum_def: &EnumDef) {
    b.line(&format!("impl {} {{", enum_def.name));
    b.increase_indent();

    b.line("pub const ALL: &'static [Self] = &[");
    b.increase_indent();

    for variant in &enum_def.variants {
        b.line(&format!("Self::{},", variant.name));
    }

    b.decrease_indent();
    b.line("];");

    b.decrease_indent();
    b.line("}");
}

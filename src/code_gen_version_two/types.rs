use super::builder::{CodeBuilder, handle_whitespace, to_pascal_case};
use crate::enum_builder::{EnumDef, EnumVariant, StructDef, TypeSystem};
use uuid::Uuid;

/// Generates the `View` routing enum with one variant per app view.
pub fn generate_view_enum(b: &mut CodeBuilder, view_variants: &[(Uuid, String)]) {
    b.line("#[derive(Debug, Clone, Copy, PartialEq, Eq)]");
    b.line("pub enum View {");
    b.increase_indent();
    for (_, name) in view_variants {
        let variant = to_pascal_case(&handle_whitespace(name));
        b.line(&format!("{},", variant));
    }
    b.decrease_indent();
    b.line("}");
}

pub fn generate_enum_definitions(b: &mut CodeBuilder, type_system: &TypeSystem) {
    for enum_def in type_system.enums.values() {
        generate_enum_code(b, enum_def, type_system);
        b.newline();
        b.newline();
    }
}

fn enum_variant_declaration(variant: &EnumVariant, type_system: &TypeSystem) -> String {
    if variant.fields.is_empty() {
        variant.name.clone()
    } else {
        let fields = variant
            .fields
            .iter()
            .map(|field| {
                format!(
                    "{}: {}",
                    field.name,
                    field.field_type.rust_type(type_system)
                )
            })
            .collect::<Vec<_>>()
            .join(", ");
        format!("{} {{ {} }}", variant.name, fields)
    }
}

fn enum_variant_pattern(enum_name: &str, variant: &EnumVariant) -> String {
    if variant.fields.is_empty() {
        format!("{}::{}", enum_name, variant.name)
    } else {
        format!("{}::{} {{ .. }}", enum_name, variant.name)
    }
}

fn enum_variant_constructor(
    variant: &EnumVariant,
    prefix: &str,
    type_system: &TypeSystem,
) -> String {
    if variant.fields.is_empty() {
        format!("{prefix}::{}", variant.name)
    } else {
        let field_values = variant
            .fields
            .iter()
            .map(|field| {
                format!(
                    "{}: {}",
                    field.name,
                    field.field_type.default_value(type_system)
                )
            })
            .collect::<Vec<_>>()
            .join(", ");
        format!("{prefix}::{} {{ {} }}", variant.name, field_values)
    }
}

fn generate_enum_code(b: &mut CodeBuilder, enum_def: &EnumDef, type_system: &TypeSystem) {
    b.line(&format!("// {} enum", enum_def.name));
    b.line("#[derive(Debug, Clone, PartialEq)]");
    b.line(&format!("pub enum {} {{", enum_def.name));
    b.increase_indent();

    for variant in &enum_def.variants {
        b.line(&format!(
            "{},",
            enum_variant_declaration(variant, type_system)
        ));
    }

    b.decrease_indent();
    b.line("}");
    b.newline();

    generate_enum_display_impl(b, enum_def);
    b.newline();

    generate_enum_impl(b, enum_def, type_system);
    b.newline();

    generate_enum_default_impl(b, enum_def);
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
            "{} => write!(f, \"{}\"),",
            enum_variant_pattern(&enum_def.name, variant),
            variant.name
        ));
    }

    b.decrease_indent();
    b.line("}");

    b.decrease_indent();
    b.line("}");

    b.decrease_indent();
    b.line("}");
}

fn generate_enum_impl(b: &mut CodeBuilder, enum_def: &EnumDef, type_system: &TypeSystem) {
    b.line(&format!("impl {} {{", enum_def.name));
    b.increase_indent();

    if let Some(first_variant) = enum_def.variants.first() {
        b.line("pub fn new() -> Self {");
        b.increase_indent();
        b.line(&enum_variant_constructor(
            first_variant,
            "Self",
            type_system,
        ));
        b.decrease_indent();
        b.line("}");
        b.newline();
    }

    b.line("pub fn all() -> Vec<Self> {");
    b.increase_indent();
    b.line("vec![");
    b.increase_indent();
    for variant in &enum_def.variants {
        b.line(&format!(
            "{},",
            enum_variant_constructor(variant, "Self", type_system)
        ));
    }
    b.decrease_indent();
    b.line("]");
    b.decrease_indent();
    b.line("}");

    b.decrease_indent();
    b.line("}");
}

fn generate_enum_default_impl(b: &mut CodeBuilder, enum_def: &EnumDef) {
    b.line(&format!("impl Default for {} {{", enum_def.name));
    b.increase_indent();
    b.line("fn default() -> Self {");
    b.increase_indent();
    b.line("Self::new()");
    b.decrease_indent();
    b.line("}");
    b.decrease_indent();
    b.line("}");
}

pub fn generate_struct_definitions(b: &mut CodeBuilder, type_system: &TypeSystem) {
    for struct_def in type_system.structs.values() {
        generate_struct_code(b, struct_def, type_system);
        b.newline();
        b.newline();
    }
}

fn generate_struct_code(b: &mut CodeBuilder, struct_def: &StructDef, type_system: &TypeSystem) {
    b.line(&format!("// {} struct", struct_def.name));
    b.line("#[derive(Debug, Clone)]");
    b.line(&format!("pub struct {} {{", struct_def.name));
    b.increase_indent();

    for field in &struct_def.fields {
        b.line(&format!(
            "pub {}: {},",
            field.name,
            field.field_type.rust_type(type_system)
        ));
    }

    b.decrease_indent();
    b.line("}");
    b.newline();

    generate_struct_display_impl(b, struct_def);
    b.newline();

    generate_struct_impl(b, struct_def, type_system);
    b.newline();

    generate_struct_default_impl(b, struct_def);
}

fn generate_struct_display_impl(b: &mut CodeBuilder, struct_def: &StructDef) {
    b.line(&format!(
        "impl std::fmt::Display for {} {{",
        struct_def.name
    ));
    b.increase_indent();
    b.line("fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {");
    b.increase_indent();
    b.line("write!(f, \"{:?}\", self)");
    b.decrease_indent();
    b.line("}");
    b.decrease_indent();
    b.line("}");
}

fn generate_struct_impl(b: &mut CodeBuilder, struct_def: &StructDef, type_system: &TypeSystem) {
    b.line(&format!("impl {} {{", struct_def.name));
    b.increase_indent();
    b.line("pub fn new() -> Self {");
    b.increase_indent();
    b.line("Self {");
    b.increase_indent();

    for field in &struct_def.fields {
        b.line(&format!(
            "{}: {},",
            field.name,
            field.field_type.default_value(type_system)
        ));
    }

    b.decrease_indent();
    b.line("}");
    b.decrease_indent();
    b.line("}");
    b.decrease_indent();
    b.line("}");
}

fn generate_struct_default_impl(b: &mut CodeBuilder, struct_def: &StructDef) {
    b.line(&format!("impl Default for {} {{", struct_def.name));
    b.increase_indent();
    b.line("fn default() -> Self {");
    b.increase_indent();
    b.line("Self::new()");
    b.decrease_indent();
    b.line("}");
    b.decrease_indent();
    b.line("}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enum_builder::FieldType;

    #[test]
    fn generated_named_types_include_new_and_default_impls() {
        let mut type_system = TypeSystem::new();
        let enum_id = type_system
            .add_enum(
                "Status".to_string(),
                vec!["Ready".to_string(), "Busy".to_string()],
            )
            .expect("enum");
        let struct_id = type_system
            .add_struct("Payload".to_string())
            .expect("struct");
        type_system
            .add_struct_field(struct_id, "title".to_string(), FieldType::String)
            .expect("field");
        type_system
            .add_struct_field(
                struct_id,
                "status".to_string(),
                FieldType::CustomEnum(enum_id),
            )
            .expect("field");

        let mut b = CodeBuilder::new();
        generate_enum_definitions(&mut b, &type_system);
        generate_struct_definitions(&mut b, &type_system);
        let code = b.build();

        assert!(code.contains("impl Status {"));
        assert!(code.contains("pub fn new() -> Self {"));
        assert!(code.contains("Self::Ready"));
        assert!(code.contains("pub fn all() -> Vec<Self> {"));
        assert!(code.contains("impl Default for Status {"));
        assert!(code.contains("impl std::fmt::Display for Payload {"));
        assert!(code.contains("impl Payload {"));
        assert!(code.contains("title: String::new(),"));
        assert!(code.contains("status: Status::Ready,"));
        assert!(code.contains("impl Default for Payload {"));
    }

    #[test]
    fn generated_advanced_enums_and_nested_structs_use_defaults() {
        let mut type_system = TypeSystem::new();
        let status_id = type_system
            .add_enum(
                "Status".to_string(),
                vec!["Ready".to_string(), "Busy".to_string()],
            )
            .expect("enum");
        let busy_variant_id = type_system
            .get_enum(status_id)
            .expect("enum")
            .get_variant_by_name("Busy")
            .expect("variant")
            .id;
        type_system
            .add_variant_field(
                status_id,
                busy_variant_id,
                "message".to_string(),
                FieldType::String,
            )
            .expect("variant field");
        type_system
            .add_variant_field(
                status_id,
                busy_variant_id,
                "attempts".to_string(),
                FieldType::Usize,
            )
            .expect("variant field");

        let meta_id = type_system.add_struct("Meta".to_string()).expect("struct");
        type_system
            .add_struct_field(meta_id, "label".to_string(), FieldType::String)
            .expect("field");

        let payload_id = type_system
            .add_struct("Payload".to_string())
            .expect("struct");
        type_system
            .add_struct_field(
                payload_id,
                "status".to_string(),
                FieldType::CustomEnum(status_id),
            )
            .expect("field");
        type_system
            .add_struct_field(
                payload_id,
                "meta".to_string(),
                FieldType::CustomStruct(meta_id),
            )
            .expect("field");

        let mut b = CodeBuilder::new();
        generate_enum_definitions(&mut b, &type_system);
        generate_struct_definitions(&mut b, &type_system);
        let code = b.build();

        assert!(code.contains("Busy { message: String, attempts: usize }"));
        assert!(code.contains("Status::Busy { .. } => write!(f, \"Busy\")"));
        assert!(code.contains("Self::Busy { message: String::new(), attempts: 0 }"));
        assert!(code.contains("meta: Meta::new(),"));
    }
}

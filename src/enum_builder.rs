use std::collections::{HashMap, HashSet};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

/// A single variant within an enum
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnumVariant {
    pub id: Uuid,
    pub name: String,
}

impl EnumVariant {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
        }
    }
}

/// User-defined enum for use with widgets
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnumDef {
    pub id: Uuid,
    pub name: String,
    pub variants: Vec<EnumVariant>,
}

impl EnumDef {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            variants: Vec::new(),
        }
    }
    
    pub fn with_variants(name: String, variant_names: Vec<String>) -> Result<Self, String> {
        let mut enum_def = Self::new(name);
        for variant_name in variant_names {
            enum_def.add_variant(variant_name)?;
        }
        Ok(enum_def)
    }
    
    pub fn add_variant(&mut self, name: String) -> Result<Uuid, String> {
        validate_variant_name(&name)?;
        
        // Check for duplicate variant names
        if self.variants.iter().any(|v| v.name == name) {
            return Err(format!("Variant '{}' already exists in enum '{}'", name, self.name));
        }
        
        let variant = EnumVariant::new(name);
        let id = variant.id;
        self.variants.push(variant);
        Ok(id)
    }
    
    pub fn remove_variant(&mut self, variant_id: Uuid) -> Result<(), String> {
        let initial_len = self.variants.len();
        self.variants.retain(|v| v.id != variant_id);
        
        if self.variants.len() == initial_len {
            return Err("Variant not found".to_string());
        }
        
        if self.variants.is_empty() {
            return Err("Cannot remove last variant - enum must have at least one variant".to_string());
        }
        
        Ok(())
    }
    
    pub fn update_variant(&mut self, variant_id: Uuid, new_name: String) -> Result<(), String> {
        validate_variant_name(&new_name)?;
        
        // Check for duplicate (excluding the one we're updating)
        if self.variants.iter().any(|v| v.id != variant_id && v.name == new_name) {
            return Err(format!("Variant '{}' already exists in enum '{}'", new_name, self.name));
        }
        
        if let Some(variant) = self.variants.iter_mut().find(|v| v.id == variant_id) {
            variant.name = new_name;
            Ok(())
        } else {
            Err("Variant not found".to_string())
        }
    }
    
    pub fn get_variant(&self, variant_id: Uuid) -> Option<&EnumVariant> {
        self.variants.iter().find(|v| v.id == variant_id)
    }
    
    pub fn get_variant_by_name(&self, name: &str) -> Option<&EnumVariant> {
        self.variants.iter().find(|v| v.name == name)
    }
}

/// Validation for Rust identifiers
pub fn validate_variant_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("Variant name cannot be empty".to_string());
    }

    let first_char = name.chars().next().unwrap();
    if !first_char.is_alphabetic() && first_char != '_' {
        return Err(format!(
            "Variant name '{}' must start with a letter or underscore",
            name
        ));
    }

    if !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err(format!(
            "Variant name '{}' can only contain letters, numbers, and underscores. Use 'CPlusPlus' instead of 'C++'",
            name
        ));
    }

    const RUST_KEYWORDS: &[&str] = &[
        "as", "break", "const", "continue", "crate", "else", "enum", "extern",
        "false", "fn", "for", "if", "impl", "in", "let", "loop", "match",
        "mod", "move", "mut", "pub", "ref", "return", "self", "Self", "static",
        "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
        "while", "async", "await", "dyn", "abstract", "become", "box", "do",
        "final", "macro", "override", "priv", "typeof", "unsized", "virtual", "yield",
    ];

    if RUST_KEYWORDS.contains(&name) {
        return Err(format!("'{}' is a Rust keyword and cannot be used", name));
    }

    Ok(())
}

pub fn validate_enum_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("Enum name cannot be empty".to_string());
    }

    let first_char = name.chars().next().unwrap();
    if !first_char.is_alphabetic() && first_char != '_' {
        return Err("Enum name must start with a letter or underscore".to_string());
    }

    if !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err("Enum name can only contain letters, numbers, and underscores".to_string());
    }

    Ok(())
}

// ==================== STRUCT DEFINITIONS ====================

/// Supported field types for struct fields
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FieldType {
    String,
    F32,
    F64,
    I32,
    I64,
    U32,
    U64,
    Usize,
    Bool,
    CustomEnum(Uuid),
}

impl FieldType {
    pub fn display_name(&self) -> String {
        match self {
            FieldType::String => "String".to_string(),
            FieldType::F32 => "f32".to_string(),
            FieldType::F64 => "f64".to_string(),
            FieldType::I32 => "i32".to_string(),
            FieldType::I64 => "i64".to_string(),
            FieldType::U32 => "u32".to_string(),
            FieldType::U64 => "u64".to_string(),
            FieldType::Usize => "usize".to_string(),
            FieldType::Bool => "bool".to_string(),
            FieldType::CustomEnum(_) => "Enum".to_string(),
        }
    }

    pub fn rust_type(&self, type_system: &TypeSystem) -> String {
        match self {
            FieldType::String => "String".to_string(),
            FieldType::F32 => "f32".to_string(),
            FieldType::F64 => "f64".to_string(),
            FieldType::I32 => "i32".to_string(),
            FieldType::I64 => "i64".to_string(),
            FieldType::U32 => "u32".to_string(),
            FieldType::U64 => "u64".to_string(),
            FieldType::Usize => "usize".to_string(),
            FieldType::Bool => "bool".to_string(),
            FieldType::CustomEnum(id) => {
                type_system.get_enum(*id)
                    .map(|e| e.name.clone())
                    .unwrap_or_else(|| "UnknownEnum".to_string())
            }
        }
    }

    pub fn default_value(&self, type_system: &TypeSystem) -> String {
        match self {
            FieldType::String => "String::new()".to_string(),
            FieldType::F32 => "0.0".to_string(),
            FieldType::F64 => "0.0".to_string(),
            FieldType::I32 | FieldType::I64 => "0".to_string(),
            FieldType::U32 | FieldType::U64 | FieldType::Usize => "0".to_string(),
            FieldType::Bool => "false".to_string(),
            FieldType::CustomEnum(id) => {
                type_system.get_enum(*id)
                    .and_then(|e| e.variants.first())
                    .map(|v| {
                        let enum_name = type_system.get_enum(*id).unwrap().name.clone();
                        format!("{}::{}", enum_name, v.name)
                    })
                    .unwrap_or_else(|| "Default::default()".to_string())
            }
        }
    }

    /// All primitive field types (for UI pick lists)
    pub fn primitives() -> Vec<FieldType> {
        vec![
            FieldType::String,
            FieldType::F32,
            FieldType::F64,
            FieldType::I32,
            FieldType::I64,
            FieldType::U32,
            FieldType::U64,
            FieldType::Usize,
            FieldType::Bool,
        ]
    }
}

impl std::fmt::Display for FieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// A single field within a struct definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructFieldDef {
    pub id: Uuid,
    pub name: String,
    pub field_type: FieldType,
}

impl StructFieldDef {
    pub fn new(name: String, field_type: FieldType) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            field_type,
        }
    }
}

/// User-defined struct for use with table widget and data binding
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructDef {
    pub id: Uuid,
    pub name: String,
    pub fields: Vec<StructFieldDef>,
}

impl StructDef {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            fields: Vec::new(),
        }
    }

    pub fn add_field(&mut self, name: String, field_type: FieldType) -> Result<Uuid, String> {
        validate_field_name(&name)?;

        if self.fields.iter().any(|f| f.name == name) {
            return Err(format!("Field '{}' already exists in struct '{}'", name, self.name));
        }

        let field = StructFieldDef::new(name, field_type);
        let id = field.id;
        self.fields.push(field);
        Ok(id)
    }

    pub fn remove_field(&mut self, field_id: Uuid) -> Result<(), String> {
        let initial_len = self.fields.len();
        self.fields.retain(|f| f.id != field_id);

        if self.fields.len() == initial_len {
            return Err("Field not found".to_string());
        }

        Ok(())
    }

    pub fn update_field_name(&mut self, field_id: Uuid, new_name: String) -> Result<(), String> {
        validate_field_name(&new_name)?;

        if self.fields.iter().any(|f| f.id != field_id && f.name == new_name) {
            return Err(format!("Field '{}' already exists in struct '{}'", new_name, self.name));
        }

        if let Some(field) = self.fields.iter_mut().find(|f| f.id == field_id) {
            field.name = new_name;
            Ok(())
        } else {
            Err("Field not found".to_string())
        }
    }

    pub fn update_field_type(&mut self, field_id: Uuid, new_type: FieldType) -> Result<(), String> {
        if let Some(field) = self.fields.iter_mut().find(|f| f.id == field_id) {
            field.field_type = new_type;
            Ok(())
        } else {
            Err("Field not found".to_string())
        }
    }

    pub fn get_field(&self, field_id: Uuid) -> Option<&StructFieldDef> {
        self.fields.iter().find(|f| f.id == field_id)
    }

    pub fn get_field_by_name(&self, name: &str) -> Option<&StructFieldDef> {
        self.fields.iter().find(|f| f.name == name)
    }
}

pub fn validate_field_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("Field name cannot be empty".to_string());
    }

    let first_char = name.chars().next().unwrap();
    if !first_char.is_alphabetic() && first_char != '_' {
        return Err(format!("Field name '{}' must start with a letter or underscore", name));
    }

    if !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err(format!("Field name '{}' can only contain letters, numbers, and underscores", name));
    }

    // Should be snake_case
    if name.chars().any(|c| c.is_uppercase()) {
        return Err(format!("Field name '{}' should be snake_case (lowercase with underscores)", name));
    }

    Ok(())
}

pub fn validate_struct_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("Struct name cannot be empty".to_string());
    }

    let first_char = name.chars().next().unwrap();
    if !first_char.is_alphabetic() && first_char != '_' {
        return Err("Struct name must start with a letter or underscore".to_string());
    }

    if !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err("Struct name can only contain letters, numbers, and underscores".to_string());
    }

    Ok(())
}

/// A snapshot of the TypeSystem state for undo/redo
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TypeSystemSnapshot {
    enums: HashMap<Uuid, EnumDef>,
    structs: HashMap<Uuid, StructDef>,
    dependencies: HashMap<Uuid, HashSet<String>>,
}

/// Action performed on the TypeSystem (for debugging/logging)
#[derive(Debug, Clone)]
pub enum TypeSystemAction {
    AddEnum { enum_id: Uuid, name: String },
    RemoveEnum { enum_id: Uuid, name: String },
    UpdateEnumName { enum_id: Uuid, old_name: String, new_name: String },
    AddVariant { enum_id: Uuid, variant_id: Uuid, name: String },
    RemoveVariant { enum_id: Uuid, variant_id: Uuid, name: String },
    UpdateVariant { enum_id: Uuid, variant_id: Uuid, old_name: String, new_name: String },
}

/// Manager for user-defined types with undo/redo and dependency tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeSystem {
    /// All enum definitions, keyed by stable UUID
    pub enums: HashMap<Uuid, EnumDef>,

    /// All struct definitions, keyed by stable UUID
    pub structs: HashMap<Uuid, StructDef>,

    /// Dependency tracking: enum_id -> set of widget_ids that use this enum
    pub dependencies: HashMap<Uuid, HashSet<String>>,

    /// Undo/redo history
    pub history: Vec<TypeSystemSnapshot>,

    /// Current position in history
    pub current_index: usize,

    /// Maximum history size (to prevent unbounded growth)
    pub max_history_size: usize,
}

impl Default for TypeSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeSystem {
    pub fn new() -> Self {
        let initial_snapshot = TypeSystemSnapshot {
            enums: HashMap::new(),
            structs: HashMap::new(),
            dependencies: HashMap::new(),
        };

        Self {
            enums: HashMap::new(),
            structs: HashMap::new(),
            dependencies: HashMap::new(),
            history: vec![initial_snapshot],
            current_index: 0,
            max_history_size: 100,
        }
    }
    
    /// Create a snapshot of the current state for undo/redo
    fn create_snapshot(&self) -> TypeSystemSnapshot {
        TypeSystemSnapshot {
            enums: self.enums.clone(),
            structs: self.structs.clone(),
            dependencies: self.dependencies.clone(),
        }
    }
    
    /// Save current state to history (called after each modifying operation)
    fn save_to_history(&mut self) {
        // Remove any "future" history if we're not at the end
        self.history.truncate(self.current_index + 1);
        
        // Add new snapshot
        let snapshot = self.create_snapshot();
        self.history.push(snapshot);
        self.current_index += 1;
        
        // Limit history size
        if self.history.len() > self.max_history_size {
            self.history.remove(0);
            self.current_index = self.current_index.saturating_sub(1);
        }
    }
    
    /// Restore from a snapshot
    fn restore_snapshot(&mut self, snapshot: &TypeSystemSnapshot) {
        self.enums = snapshot.enums.clone();
        self.structs = snapshot.structs.clone();
        self.dependencies = snapshot.dependencies.clone();
    }
    
    // ==================== UNDO/REDO ====================
    
    pub fn can_undo(&self) -> bool {
        self.current_index > 0
    }
    
    pub fn can_redo(&self) -> bool {
        self.current_index < self.history.len() - 1
    }
    
    pub fn undo(&mut self) -> Result<(), String> {
        if !self.can_undo() {
            return Err("Nothing to undo".to_string());
        }
        
        self.current_index -= 1;
        let cloned_history = self.history[self.current_index].clone();
        self.restore_snapshot(&cloned_history);
        Ok(())
    }
    
    pub fn redo(&mut self) -> Result<(), String> {
        if !self.can_redo() {
            return Err("Nothing to redo".to_string());
        }
        
        self.current_index += 1;
        let cloned_history = self.history[self.current_index].clone();
        self.restore_snapshot(&cloned_history);
        Ok(())
    }
    
    pub fn clear_history(&mut self) {
        let current_snapshot = self.create_snapshot();
        self.history = vec![current_snapshot];
        self.current_index = 0;
    }
    
    // ==================== ENUM OPERATIONS ====================
    
    pub fn add_enum(&mut self, name: String, variants: Vec<String>) -> Result<Uuid, String> {
        validate_enum_name(&name)?;
        
        // Check for duplicate names
        if self.enums.values().any(|e| e.name == name) {
            return Err(format!("Enum '{}' already exists", name));
        }
        
        if variants.is_empty() {
            return Err("Enum must have at least one variant".to_string());
        }
        
        let enum_def = EnumDef::with_variants(name, variants)?;
        let enum_id = enum_def.id;
        
        self.enums.insert(enum_id, enum_def);
        self.save_to_history();
        
        Ok(enum_id)
    }
    
    pub fn remove_enum(&mut self, enum_id: Uuid) -> Result<(), String> {
        // Check dependencies before removing
        if let Some(dependents) = self.dependencies.get(&enum_id) {
            if !dependents.is_empty() {
                return Err(format!(
                    "Cannot remove enum - it is used by {} widget(s): {}",
                    dependents.len(),
                    dependents.iter().take(3).cloned().collect::<Vec<_>>().join(", ")
                ));
            }
        }
        
        if self.enums.remove(&enum_id).is_some() {
            self.dependencies.remove(&enum_id);
            self.save_to_history();
            Ok(())
        } else {
            Err("Enum not found".to_string())
        }
    }
    
    pub fn update_enum_name(&mut self, enum_id: Uuid, new_name: String) -> Result<(), String> {
        validate_enum_name(&new_name)?;
        
        // Check for duplicate names (excluding the one we're updating)
        if self.enums.values().any(|e| e.id != enum_id && e.name == new_name) {
            return Err(format!("Enum '{}' already exists", new_name));
        }
        
        if let Some(enum_def) = self.enums.get_mut(&enum_id) {
            enum_def.name = new_name;
            self.save_to_history();
            Ok(())
        } else {
            Err("Enum not found".to_string())
        }
    }
    
    pub fn add_variant(&mut self, enum_id: Uuid, variant_name: String) -> Result<Uuid, String> {
        if let Some(enum_def) = self.enums.get_mut(&enum_id) {
            let variant_id = enum_def.add_variant(variant_name)?;
            self.save_to_history();
            Ok(variant_id)
        } else {
            Err("Enum not found".to_string())
        }
    }
    
    pub fn remove_variant(&mut self, enum_id: Uuid, variant_id: Uuid) -> Result<(), String> {
        if let Some(enum_def) = self.enums.get_mut(&enum_id) {
            enum_def.remove_variant(variant_id)?;
            self.save_to_history();
            Ok(())
        } else {
            Err("Enum not found".to_string())
        }
    }
    
    pub fn update_variant(&mut self, enum_id: Uuid, variant_id: Uuid, new_name: String) -> Result<(), String> {
        if let Some(enum_def) = self.enums.get_mut(&enum_id) {
            enum_def.update_variant(variant_id, new_name)?;
            self.save_to_history();
            Ok(())
        } else {
            Err("Enum not found".to_string())
        }
    }
    
    // ==================== STRUCT OPERATIONS ====================

    pub fn add_struct(&mut self, name: String) -> Result<Uuid, String> {
        validate_struct_name(&name)?;

        if self.structs.values().any(|s| s.name == name) {
            return Err(format!("Struct '{}' already exists", name));
        }

        let struct_def = StructDef::new(name);
        let struct_id = struct_def.id;

        self.structs.insert(struct_id, struct_def);
        self.save_to_history();

        Ok(struct_id)
    }

    pub fn remove_struct(&mut self, struct_id: Uuid) -> Result<(), String> {
        if self.structs.remove(&struct_id).is_some() {
            self.save_to_history();
            Ok(())
        } else {
            Err("Struct not found".to_string())
        }
    }

    pub fn update_struct_name(&mut self, struct_id: Uuid, new_name: String) -> Result<(), String> {
        validate_struct_name(&new_name)?;

        if self.structs.values().any(|s| s.id != struct_id && s.name == new_name) {
            return Err(format!("Struct '{}' already exists", new_name));
        }

        if let Some(struct_def) = self.structs.get_mut(&struct_id) {
            struct_def.name = new_name;
            self.save_to_history();
            Ok(())
        } else {
            Err("Struct not found".to_string())
        }
    }

    pub fn add_struct_field(&mut self, struct_id: Uuid, name: String, field_type: FieldType) -> Result<Uuid, String> {
        if let Some(struct_def) = self.structs.get_mut(&struct_id) {
            let field_id = struct_def.add_field(name, field_type)?;
            self.save_to_history();
            Ok(field_id)
        } else {
            Err("Struct not found".to_string())
        }
    }

    pub fn remove_struct_field(&mut self, struct_id: Uuid, field_id: Uuid) -> Result<(), String> {
        if let Some(struct_def) = self.structs.get_mut(&struct_id) {
            struct_def.remove_field(field_id)?;
            self.save_to_history();
            Ok(())
        } else {
            Err("Struct not found".to_string())
        }
    }

    pub fn update_struct_field_name(&mut self, struct_id: Uuid, field_id: Uuid, new_name: String) -> Result<(), String> {
        if let Some(struct_def) = self.structs.get_mut(&struct_id) {
            struct_def.update_field_name(field_id, new_name)?;
            self.save_to_history();
            Ok(())
        } else {
            Err("Struct not found".to_string())
        }
    }

    pub fn update_struct_field_type(&mut self, struct_id: Uuid, field_id: Uuid, new_type: FieldType) -> Result<(), String> {
        if let Some(struct_def) = self.structs.get_mut(&struct_id) {
            struct_def.update_field_type(field_id, new_type)?;
            self.save_to_history();
            Ok(())
        } else {
            Err("Struct not found".to_string())
        }
    }

    pub fn get_struct(&self, struct_id: Uuid) -> Option<&StructDef> {
        self.structs.get(&struct_id)
    }

    pub fn get_struct_by_name(&self, name: &str) -> Option<&StructDef> {
        self.structs.values().find(|s| s.name == name)
    }

    pub fn all_structs(&self) -> Vec<&StructDef> {
        let mut structs: Vec<_> = self.structs.values().collect();
        structs.sort_by(|a, b| a.name.cmp(&b.name));
        structs
    }

    pub fn struct_count(&self) -> usize {
        self.structs.len()
    }

    // ==================== QUERY OPERATIONS ====================

    pub fn get_enum(&self, enum_id: Uuid) -> Option<&EnumDef> {
        self.enums.get(&enum_id)
    }
    
    pub fn get_enum_by_name(&self, name: &str) -> Option<&EnumDef> {
        self.enums.values().find(|e| e.name == name)
    }
    
    pub fn all_enums(&self) -> Vec<&EnumDef> {
        let mut enums: Vec<_> = self.enums.values().collect();
        enums.sort_by(|a, b| a.name.cmp(&b.name));
        enums
    }
    
    pub fn enum_names(&self) -> Vec<String> {
        let mut names: Vec<String> = self.enums.values().map(|e| e.name.clone()).collect();
        names.sort();
        names
    }
    
    pub fn enum_count(&self) -> usize {
        self.enums.len()
    }
    
    // ==================== DEPENDENCY TRACKING ====================
    
    /// Register that a widget is using an enum
    pub fn add_dependency(&mut self, enum_id: Uuid, widget_id: String) {
        self.dependencies
            .entry(enum_id)
            .or_insert_with(HashSet::new)
            .insert(widget_id);
    }
    
    /// Remove a widget's dependency on an enum
    pub fn remove_dependency(&mut self, enum_id: Uuid, widget_id: &str) {
        if let Some(deps) = self.dependencies.get_mut(&enum_id) {
            deps.remove(widget_id);
            if deps.is_empty() {
                self.dependencies.remove(&enum_id);
            }
        }
    }
    
    /// Remove all dependencies for a widget (when widget is deleted)
    pub fn remove_widget_dependencies(&mut self, widget_id: &str) {
        for deps in self.dependencies.values_mut() {
            deps.remove(widget_id);
        }
        self.dependencies.retain(|_, deps| !deps.is_empty());
    }
    
    /// Get all widgets that depend on an enum
    pub fn get_dependents(&self, enum_id: Uuid) -> Vec<String> {
        self.dependencies
            .get(&enum_id)
            .map(|deps| {
                let mut list: Vec<_> = deps.iter().cloned().collect();
                list.sort();
                list
            })
            .unwrap_or_default()
    }
    
    /// Get all enums used by a widget
    pub fn get_widget_enum_dependencies(&self, widget_id: &str) -> Vec<Uuid> {
        self.dependencies
            .iter()
            .filter(|(_, deps)| deps.contains(widget_id))
            .map(|(enum_id, _)| *enum_id)
            .collect()
    }
    
    /// Check if an enum is in use
    pub fn is_enum_in_use(&self, enum_id: Uuid) -> bool {
        self.dependencies
            .get(&enum_id)
            .map(|deps| !deps.is_empty())
            .unwrap_or(false)
    }
    
    /// Get dependency summary for UI display
    pub fn get_dependency_info(&self, enum_id: Uuid) -> Option<DependencyInfo> {
        self.enums.get(&enum_id).map(|enum_def| {
            let dependents = self.get_dependents(enum_id);
            DependencyInfo {
                enum_id,
                enum_name: enum_def.name.clone(),
                dependent_count: dependents.len(),
                dependents,
            }
        })
    }
}

/// Information about enum dependencies for UI display
#[derive(Debug, Clone)]
pub struct DependencyInfo {
    pub enum_id: Uuid,
    pub enum_name: String,
    pub dependent_count: usize,
    pub dependents: Vec<String>,
}

// ==================== MIGRATION HELPER ====================

/// Helper to migrate from old string-based system to UUID-based system
pub fn migrate_from_old_system(
    old_enums: HashMap<String, (String, Vec<String>)> // name -> (name, variants)
) -> (TypeSystem, HashMap<String, Uuid>) {
    let mut type_system = TypeSystem::new();
    let mut name_to_id_map = HashMap::new();
    
    for (name, (_, variants)) in old_enums {
        if let Ok(enum_id) = type_system.add_enum(name.clone(), variants) {
            name_to_id_map.insert(name, enum_id);
        }
    }
    
    (type_system, name_to_id_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uuid_stability() {
        let mut ts = TypeSystem::new();
        let enum_id = ts.add_enum("Languages".to_string(), vec!["Rust".to_string()]).unwrap();
        
        // Rename the enum
        ts.update_enum_name(enum_id, "ProgrammingLanguages".to_string()).unwrap();
        
        // ID should be the same
        let enum_def = ts.get_enum(enum_id).unwrap();
        assert_eq!(enum_def.name, "ProgrammingLanguages");
        assert_eq!(enum_def.id, enum_id);
    }

    #[test]
    fn test_validation() {
        let mut ts = TypeSystem::new();
        
        // Valid
        assert!(ts.add_enum("Languages".to_string(), vec!["Rust".to_string()]).is_ok());
        
        // Invalid - C++
        let result = ts.add_enum("Languages2".to_string(), vec!["C++".to_string()]);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("can only contain"));
    }

    #[test]
    fn test_undo_redo() {
        let mut ts = TypeSystem::new();
        
        // Add enum
        let enum_id = ts.add_enum("Languages".to_string(), vec!["Rust".to_string()]).unwrap();
        assert_eq!(ts.enum_count(), 1);
        
        // Undo
        ts.undo().unwrap();
        assert_eq!(ts.enum_count(), 0);
        
        // Redo
        ts.redo().unwrap();
        assert_eq!(ts.enum_count(), 1);
        assert!(ts.get_enum(enum_id).is_some());
    }

    #[test]
    fn test_dependency_tracking() {
        let mut ts = TypeSystem::new();
        let enum_id = ts.add_enum("Languages".to_string(), vec!["Rust".to_string()]).unwrap();
        
        // Add dependency
        ts.add_dependency(enum_id, "combo_box_1".to_string());
        ts.add_dependency(enum_id, "combo_box_2".to_string());
        
        assert!(ts.is_enum_in_use(enum_id));
        assert_eq!(ts.get_dependents(enum_id).len(), 2);
        
        // Try to remove - should fail
        assert!(ts.remove_enum(enum_id).is_err());
        
        // Remove dependencies
        ts.remove_dependency(enum_id, "combo_box_1");
        ts.remove_dependency(enum_id, "combo_box_2");
        
        // Now should succeed
        assert!(ts.remove_enum(enum_id).is_ok());
    }

    #[test]
    fn test_variant_operations() {
        let mut ts = TypeSystem::new();
        let enum_id = ts.add_enum("Languages".to_string(), vec!["Rust".to_string()]).unwrap();
        
        // Add variant
        let variant_id = ts.add_variant(enum_id, "Python".to_string()).unwrap();
        
        let enum_def = ts.get_enum(enum_id).unwrap();
        assert_eq!(enum_def.variants.len(), 2);
        
        // Update variant
        ts.update_variant(enum_id, variant_id, "Go".to_string()).unwrap();
        let enum_def = ts.get_enum(enum_id).unwrap();
        assert_eq!(enum_def.get_variant(variant_id).unwrap().name, "Go");
        
        // Undo
        ts.undo().unwrap();
        let enum_def = ts.get_enum(enum_id).unwrap();
        assert_eq!(enum_def.get_variant(variant_id).unwrap().name, "Python");
    }
}
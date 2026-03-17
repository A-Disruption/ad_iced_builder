use iced::{
    Alignment, Element, Length, Padding, Task, Theme, padding,
    widget::{button, column, container, pick_list, row, scrollable, space, text, text_input},
};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

use crate::enum_builder::*;
use crate::icon;
use crate::icon_lucide;
use crate::styles;
use widgets::collapsible::{CollapsibleGroup, collapsible};

// ==================== STATE ====================

#[derive(Debug, Clone)]
struct FieldTypeOption {
    pub field_type: FieldType,
    pub label: String,
}

impl FieldTypeOption {
    fn from_field_type(ft: &FieldType, type_system: &TypeSystem) -> Self {
        let label = match ft {
            FieldType::CustomEnum(id) => {
                let name = type_system
                    .get_enum(*id)
                    .map(|e| e.name.clone())
                    .unwrap_or_else(|| "Unknown".to_string());
                format!("{} (enum)", name)
            }
            FieldType::CustomStruct(id) => {
                let name = type_system
                    .get_struct(*id)
                    .map(|s| s.name.clone())
                    .unwrap_or_else(|| "Unknown".to_string());
                format!("{} (struct)", name)
            }
            other => other.display_name(),
        };

        Self {
            field_type: ft.clone(),
            label,
        }
    }
}

impl std::fmt::Display for FieldTypeOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label)
    }
}

impl PartialEq for FieldTypeOption {
    fn eq(&self, other: &Self) -> bool {
        self.field_type == other.field_type
    }
}

fn field_type_options(
    type_system: &TypeSystem,
    owning_enum_id: Uuid,
    selected_type: Option<&FieldType>,
) -> Vec<FieldTypeOption> {
    let mut options: Vec<FieldTypeOption> = FieldType::primitives()
        .iter()
        .map(|ft| FieldTypeOption::from_field_type(ft, type_system))
        .collect();

    for enum_def in type_system.all_enums() {
        if enum_def.id != owning_enum_id
            || matches!(selected_type, Some(FieldType::CustomEnum(id)) if *id == enum_def.id)
        {
            options.push(FieldTypeOption::from_field_type(
                &FieldType::CustomEnum(enum_def.id),
                type_system,
            ));
        }
    }

    for struct_def in type_system.all_structs() {
        options.push(FieldTypeOption::from_field_type(
            &FieldType::CustomStruct(struct_def.id),
            type_system,
        ));
    }

    if let Some(selected_type) = selected_type {
        if !options
            .iter()
            .any(|option| option.field_type == *selected_type)
        {
            options.push(FieldTypeOption::from_field_type(selected_type, type_system));
        }
    }

    options
}

#[derive(Debug, Clone)]
pub struct EnumEditorState {
    /// The enum being edited
    pub enum_id: Uuid,

    /// Whether this enum's details are expanded
    pub is_expanded: bool,

    /// Input field for enum name
    pub name_input: String,

    /// Input field for new variant
    pub new_variant_input: String,

    /// New payload field names keyed by variant
    pub new_variant_field_names: HashMap<Uuid, String>,

    /// New payload field types keyed by variant
    pub new_variant_field_types: HashMap<Uuid, FieldType>,

    /// Any validation errors to display
    pub validation_error: Option<String>,
}

impl EnumEditorState {
    pub fn new(enum_id: Uuid, enum_name: String) -> Self {
        Self {
            enum_id,
            is_expanded: false,
            name_input: enum_name,
            new_variant_input: String::new(),
            new_variant_field_names: HashMap::new(),
            new_variant_field_types: HashMap::new(),
            validation_error: None,
        }
    }
}

pub struct EnumEditorView {
    /// Reference to the TypeSystem (lives in WidgetVisualizer)
    /// We don't own it, just view it

    /// Editor states for each enum
    pub editor_states: Vec<EnumEditorState>,
}

impl EnumEditorView {
    pub fn new() -> Self {
        Self {
            editor_states: Vec::new(),
        }
    }

    /// Sync editor states with TypeSystem
    /// Call this whenever TypeSystem changes (after undo/redo, load, etc.)
    pub fn sync_with_type_system(&mut self, type_system: &TypeSystem) {
        // Remove states for deleted enums
        self.editor_states
            .retain(|state| type_system.get_enum(state.enum_id).is_some());

        // Add states for new enums
        for enum_def in type_system.all_enums() {
            if !self.editor_states.iter().any(|s| s.enum_id == enum_def.id) {
                self.editor_states
                    .push(EnumEditorState::new(enum_def.id, enum_def.name.clone()));
            }
        }

        // Update names for existing states
        for state in &mut self.editor_states {
            if let Some(enum_def) = type_system.get_enum(state.enum_id) {
                // Only update if not currently being edited
                if !state.is_expanded {
                    state.name_input = enum_def.name.clone();
                }

                let variant_ids: HashSet<Uuid> =
                    enum_def.variants.iter().map(|variant| variant.id).collect();
                state
                    .new_variant_field_names
                    .retain(|variant_id, _| variant_ids.contains(variant_id));
                state
                    .new_variant_field_types
                    .retain(|variant_id, _| variant_ids.contains(variant_id));

                for variant in &enum_def.variants {
                    state
                        .new_variant_field_names
                        .entry(variant.id)
                        .or_insert_with(String::new);
                    state
                        .new_variant_field_types
                        .entry(variant.id)
                        .or_insert(FieldType::String);
                }
            }
        }
    }
}

// ==================== MESSAGES ====================

#[derive(Debug, Clone)]
pub enum Message {
    // Enum operations
    CreateNewEnum,
    DeleteEnum(Uuid),
    RenameEnum {
        enum_id: Uuid,
        new_name: String,
    },

    // Variant operations
    AddVariant {
        enum_id: Uuid,
        name: String,
    },
    RemoveVariant {
        enum_id: Uuid,
        variant_id: Uuid,
    },
    UpdateVariant {
        enum_id: Uuid,
        variant_id: Uuid,
        new_name: String,
    },
    AddVariantField {
        enum_id: Uuid,
        variant_id: Uuid,
        name: String,
        field_type: FieldType,
    },
    RemoveVariantField {
        enum_id: Uuid,
        variant_id: Uuid,
        field_id: Uuid,
    },
    UpdateVariantFieldName {
        enum_id: Uuid,
        variant_id: Uuid,
        field_id: Uuid,
        new_name: String,
    },
    UpdateVariantFieldType {
        enum_id: Uuid,
        variant_id: Uuid,
        field_id: Uuid,
        new_type: FieldType,
    },

    // UI state
    ToggleExpanded(Uuid),
    EnumNameInputChanged {
        enum_id: Uuid,
        value: String,
    },
    NewVariantInputChanged {
        enum_id: Uuid,
        value: String,
    },
    NewVariantFieldNameInputChanged {
        enum_id: Uuid,
        variant_id: Uuid,
        value: String,
    },
    NewVariantFieldTypeChanged {
        enum_id: Uuid,
        variant_id: Uuid,
        field_type: FieldType,
    },
    SaveEnum(Uuid),

    // Undo/Redo
    Undo,
    Redo,
}

// ==================== UPDATE ====================

pub fn update(
    message: Message,
    type_system: &mut TypeSystem,
    editor_view: &mut EnumEditorView,
) -> Task<Message> {
    match message {
        Message::CreateNewEnum => {
            let count = type_system.enum_count() + 1;
            match type_system.add_enum(format!("NewEnum{}", count), vec!["Variant1".to_string()]) {
                Ok(enum_id) => {
                    editor_view.sync_with_type_system(type_system);
                    // Expand the new enum
                    if let Some(state) = editor_view
                        .editor_states
                        .iter_mut()
                        .find(|s| s.enum_id == enum_id)
                    {
                        state.is_expanded = true;
                    }
                }
                Err(e) => eprintln!("Error creating enum: {}", e),
            }
        }

        Message::DeleteEnum(enum_id) => {
            match type_system.remove_enum(enum_id) {
                Ok(()) => {
                    editor_view.sync_with_type_system(type_system);
                }
                Err(e) => {
                    // Show error in the UI
                    if let Some(state) = editor_view
                        .editor_states
                        .iter_mut()
                        .find(|s| s.enum_id == enum_id)
                    {
                        state.validation_error = Some(e);
                    }
                }
            }
        }

        Message::RenameEnum { enum_id, new_name } => {
            if let Some(state) = editor_view
                .editor_states
                .iter_mut()
                .find(|s| s.enum_id == enum_id)
            {
                match type_system.update_enum_name(enum_id, new_name) {
                    Ok(()) => {
                        state.validation_error = None;
                        state.is_expanded = false; // Collapse after save
                    }
                    Err(e) => {
                        state.validation_error = Some(e);
                    }
                }
            }
        }

        Message::AddVariant { enum_id, name } => match type_system.add_variant(enum_id, name) {
            Ok(_variant_id) => {
                if let Some(state) = editor_view
                    .editor_states
                    .iter_mut()
                    .find(|s| s.enum_id == enum_id)
                {
                    state.new_variant_input.clear();
                    state.validation_error = None;
                }
                editor_view.sync_with_type_system(type_system);
            }
            Err(e) => {
                if let Some(state) = editor_view
                    .editor_states
                    .iter_mut()
                    .find(|s| s.enum_id == enum_id)
                {
                    state.validation_error = Some(e);
                }
            }
        },

        Message::RemoveVariant {
            enum_id,
            variant_id,
        } => match type_system.remove_variant(enum_id, variant_id) {
            Ok(()) => editor_view.sync_with_type_system(type_system),
            Err(e) => {
                if let Some(state) = editor_view
                    .editor_states
                    .iter_mut()
                    .find(|s| s.enum_id == enum_id)
                {
                    state.validation_error = Some(e);
                }
            }
        },

        Message::UpdateVariant {
            enum_id,
            variant_id,
            new_name,
        } => match type_system.update_variant(enum_id, variant_id, new_name) {
            Ok(()) => {
                if let Some(state) = editor_view
                    .editor_states
                    .iter_mut()
                    .find(|s| s.enum_id == enum_id)
                {
                    state.validation_error = None;
                }
            }
            Err(e) => {
                if let Some(state) = editor_view
                    .editor_states
                    .iter_mut()
                    .find(|s| s.enum_id == enum_id)
                {
                    state.validation_error = Some(e);
                }
            }
        },

        Message::AddVariantField {
            enum_id,
            variant_id,
            name,
            field_type,
        } => {
            if let Some(state) = editor_view
                .editor_states
                .iter_mut()
                .find(|s| s.enum_id == enum_id)
            {
                match type_system.add_variant_field(enum_id, variant_id, name, field_type) {
                    Ok(_field_id) => {
                        state.validation_error = None;
                        state
                            .new_variant_field_names
                            .insert(variant_id, String::new());
                        state
                            .new_variant_field_types
                            .insert(variant_id, FieldType::String);
                    }
                    Err(e) => {
                        state.validation_error = Some(e);
                    }
                }
            }
        }

        Message::RemoveVariantField {
            enum_id,
            variant_id,
            field_id,
        } => match type_system.remove_variant_field(enum_id, variant_id, field_id) {
            Ok(()) => {
                if let Some(state) = editor_view
                    .editor_states
                    .iter_mut()
                    .find(|s| s.enum_id == enum_id)
                {
                    state.validation_error = None;
                }
            }
            Err(e) => {
                if let Some(state) = editor_view
                    .editor_states
                    .iter_mut()
                    .find(|s| s.enum_id == enum_id)
                {
                    state.validation_error = Some(e);
                }
            }
        },

        Message::UpdateVariantFieldName {
            enum_id,
            variant_id,
            field_id,
            new_name,
        } => match type_system.update_variant_field_name(enum_id, variant_id, field_id, new_name) {
            Ok(()) => {
                if let Some(state) = editor_view
                    .editor_states
                    .iter_mut()
                    .find(|s| s.enum_id == enum_id)
                {
                    state.validation_error = None;
                }
            }
            Err(e) => {
                if let Some(state) = editor_view
                    .editor_states
                    .iter_mut()
                    .find(|s| s.enum_id == enum_id)
                {
                    state.validation_error = Some(e);
                }
            }
        },

        Message::UpdateVariantFieldType {
            enum_id,
            variant_id,
            field_id,
            new_type,
        } => match type_system.update_variant_field_type(enum_id, variant_id, field_id, new_type) {
            Ok(()) => {
                if let Some(state) = editor_view
                    .editor_states
                    .iter_mut()
                    .find(|s| s.enum_id == enum_id)
                {
                    state.validation_error = None;
                }
            }
            Err(e) => {
                if let Some(state) = editor_view
                    .editor_states
                    .iter_mut()
                    .find(|s| s.enum_id == enum_id)
                {
                    state.validation_error = Some(e);
                }
            }
        },

        Message::ToggleExpanded(enum_id) => {
            if let Some(state) = editor_view
                .editor_states
                .iter_mut()
                .find(|s| s.enum_id == enum_id)
            {
                state.is_expanded = !state.is_expanded;
                state.validation_error = None;
            }
        }

        Message::EnumNameInputChanged { enum_id, value } => {
            if let Some(state) = editor_view
                .editor_states
                .iter_mut()
                .find(|s| s.enum_id == enum_id)
            {
                state.name_input = value;
                state.validation_error = None;
            }
        }

        Message::NewVariantInputChanged { enum_id, value } => {
            if let Some(state) = editor_view
                .editor_states
                .iter_mut()
                .find(|s| s.enum_id == enum_id)
            {
                state.new_variant_input = value;
                state.validation_error = None;
            }
        }

        Message::NewVariantFieldNameInputChanged {
            enum_id,
            variant_id,
            value,
        } => {
            if let Some(state) = editor_view
                .editor_states
                .iter_mut()
                .find(|s| s.enum_id == enum_id)
            {
                state.new_variant_field_names.insert(variant_id, value);
                state.validation_error = None;
            }
        }

        Message::NewVariantFieldTypeChanged {
            enum_id,
            variant_id,
            field_type,
        } => {
            if let Some(state) = editor_view
                .editor_states
                .iter_mut()
                .find(|s| s.enum_id == enum_id)
            {
                state.new_variant_field_types.insert(variant_id, field_type);
                state.validation_error = None;
            }
        }

        Message::SaveEnum(enum_id) => {
            if let Some(state) = editor_view
                .editor_states
                .iter_mut()
                .find(|s| s.enum_id == enum_id)
            {
                let new_name = state.name_input.clone();
                match type_system.update_enum_name(enum_id, new_name) {
                    Ok(()) => {
                        state.validation_error = None;
                        state.is_expanded = false;
                    }
                    Err(e) => {
                        state.validation_error = Some(e);
                    }
                }
            }
        }

        Message::Undo => {
            if let Err(e) = type_system.undo() {
                eprintln!("Undo failed: {}", e);
            }
            editor_view.sync_with_type_system(type_system);
        }

        Message::Redo => {
            if let Err(e) = type_system.redo() {
                eprintln!("Redo failed: {}", e);
            }
            editor_view.sync_with_type_system(type_system);
        }
    }

    Task::none()
}

// ==================== VIEW ====================
pub fn view<'a>(
    type_system: &'a TypeSystem,
    editor_view: &'a EnumEditorView,
) -> Element<'a, Message> {
    let header = row![
        space::horizontal(),
        text("Custom Enums").size(18),
        space::horizontal(),
        button(icon_lucide::plus())
            .style(button::text)
            .on_press(Message::CreateNewEnum)
    ]
    .align_y(Alignment::Center)
    .padding(padding::right(10));

    let enum_vec = type_system
        .all_enums()
        .iter()
        .map(|enum_def| {
            let enum_name = enum_def.name.clone();
            let enum_id = enum_def.id;
            let editor_state = editor_view
                .editor_states
                .iter()
                .find(|state| state.enum_id == enum_id);
            let new_variant_name = editor_state
                .map(|state| state.new_variant_input.clone())
                .unwrap_or_default();
            let validation_error = editor_state.and_then(|state| state.validation_error.clone());

            let variant_rows = enum_def
                .variants
                .iter()
                .map(|variant| {
                    let variant_name = variant.name.clone();
                    let variant_id = variant.id;
                    let new_field_name = editor_state
                        .and_then(|state| state.new_variant_field_names.get(&variant_id).cloned())
                        .unwrap_or_default();
                    let new_field_type = editor_state
                        .and_then(|state| state.new_variant_field_types.get(&variant_id).cloned())
                        .unwrap_or(FieldType::String);
                    let new_field_type_option =
                        FieldTypeOption::from_field_type(&new_field_type, type_system);
                    let add_type_options =
                        field_type_options(type_system, enum_id, Some(&new_field_type));

                    let field_rows = variant
                        .fields
                        .iter()
                        .map(|field| {
                            let field_id = field.id;
                            let field_name = field.name.clone();
                            let field_type_option =
                                FieldTypeOption::from_field_type(&field.field_type, type_system);
                            let type_options =
                                field_type_options(type_system, enum_id, Some(&field.field_type));

                            row![
                                text_input::<Message, Theme, iced::Renderer>(
                                    "field_name",
                                    &field_name,
                                )
                                .on_input(move |new_name| {
                                    Message::UpdateVariantFieldName {
                                        enum_id,
                                        variant_id,
                                        field_id,
                                        new_name,
                                    }
                                }),
                                pick_list(
                                    type_options,
                                    Some(field_type_option),
                                    move |selected: FieldTypeOption| {
                                        Message::UpdateVariantFieldType {
                                            enum_id,
                                            variant_id,
                                            field_id,
                                            new_type: selected.field_type,
                                        }
                                    }
                                )
                                .width(150),
                                button(icon_lucide::trash_2())
                                    .style(styles::button::cancel)
                                    .on_press(Message::RemoveVariantField {
                                        enum_id,
                                        variant_id,
                                        field_id,
                                    }),
                            ]
                            .spacing(10)
                            .align_y(Alignment::Center)
                            .into()
                        })
                        .collect::<Vec<_>>();

                    column![
                        row![
                            text_input::<Message, Theme, iced::Renderer>(
                                "Variant Name",
                                &variant_name,
                            )
                            .on_input(move |new_name| {
                                Message::UpdateVariant {
                                    enum_id,
                                    variant_id,
                                    new_name,
                                }
                            }),
                            button(icon_lucide::trash_2())
                                .style(styles::button::cancel)
                                .on_press(Message::RemoveVariant {
                                    enum_id,
                                    variant_id
                                }),
                        ]
                        .spacing(10)
                        .padding(5)
                        .align_y(Alignment::Center),
                        container(
                            column![
                                text("Payload Fields").size(11),
                                column(field_rows).spacing(6),
                                row![
                                    text_input("field_name", &new_field_name).on_input(
                                        move |value| {
                                            Message::NewVariantFieldNameInputChanged {
                                                enum_id,
                                                variant_id,
                                                value,
                                            }
                                        }
                                    ),
                                    pick_list(
                                        add_type_options,
                                        Some(new_field_type_option),
                                        move |selected: FieldTypeOption| {
                                            Message::NewVariantFieldTypeChanged {
                                                enum_id,
                                                variant_id,
                                                field_type: selected.field_type,
                                            }
                                        }
                                    )
                                    .width(150),
                                    button(icon_lucide::plus().center())
                                        .style(button::text)
                                        .on_press(Message::AddVariantField {
                                            enum_id,
                                            variant_id,
                                            name: new_field_name.clone(),
                                            field_type: new_field_type.clone(),
                                        }),
                                ]
                                .spacing(10)
                                .align_y(Alignment::Center),
                            ]
                            .spacing(6),
                        )
                        .padding(Padding {
                            top: 0.0,
                            right: 5.0,
                            bottom: 5.0,
                            left: 20.0,
                        }),
                    ]
                    .spacing(4)
                    .into()
                })
                .collect::<Vec<_>>();

            let error_row = validation_error
                .map(|error| container(text(error).size(11)).padding([0, 10]))
                .unwrap_or_else(|| container(text("")).padding(0));

            collapsible(
                &enum_name,
                column![
                    column![
                        text("Enum Name"),
                        row![
                            text_input("Enum Name", &enum_name).on_input(move |name| {
                                Message::RenameEnum {
                                    enum_id: enum_id,
                                    new_name: name,
                                }
                            }),
                            button(icon_lucide::trash_2()).style(styles::button::invisible)
                        ]
                        .spacing(10)
                        .padding(5)
                    ]
                    .spacing(5)
                    .padding([5, 10]),
                    container(text("Enum Variants"),).padding(Padding {
                        top: 10.0,
                        right: 10.0,
                        bottom: 0.0,
                        left: 10.0
                    }),
                    column(variant_rows).padding([5, 10]),
                    container(
                        row![
                            text_input("variant_name", &new_variant_name).on_input(move |value| {
                                Message::NewVariantInputChanged { enum_id, value }
                            }),
                            button(icon_lucide::plus().center())
                                .style(button::text)
                                .on_press(Message::AddVariant {
                                    enum_id,
                                    name: new_variant_name.clone(),
                                }),
                        ]
                        .spacing(10)
                        .align_y(Alignment::Center)
                    )
                    .padding([5, 10]),
                    error_row,
                ],
            )
            .expand_icon(icon::collapsed())
            .collapse_icon(icon::expanded())
            .action_icon(
                button(icon_lucide::trash_2())
                    .style(styles::button::cancel)
                    .on_press(Message::DeleteEnum(enum_id))
                    .padding(padding::right(10.0)),
            )
            .title_alignment(Alignment::Center)
            .into()
        })
        .collect();

    container(
        scrollable(
            container(column![
                header,
                CollapsibleGroup::new(enum_vec).spacing(5.0)
            ])
            .padding(padding::right(10)),
        )
        .height(Length::Fill),
    )
    .height(Length::Fill)
    .width(400.0)
    .into()
}

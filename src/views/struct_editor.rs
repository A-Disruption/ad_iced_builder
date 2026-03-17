use iced::{
    Alignment, Element, Length, Padding, Task, Theme, padding,
    widget::{button, column, container, pick_list, row, scrollable, space, text, text_input},
};
use uuid::Uuid;

use crate::enum_builder::*;
use crate::icon;
use crate::icon_lucide;
use crate::styles;
use widgets::collapsible::{CollapsibleGroup, collapsible};

// Wrapper for FieldType that carries a display label for pick_list rendering
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
    owning_struct_id: Uuid,
    selected_type: Option<&FieldType>,
) -> Vec<FieldTypeOption> {
    let mut options: Vec<FieldTypeOption> = FieldType::primitives()
        .iter()
        .map(|ft| FieldTypeOption::from_field_type(ft, type_system))
        .collect();

    for enum_def in type_system.all_enums() {
        options.push(FieldTypeOption::from_field_type(
            &FieldType::CustomEnum(enum_def.id),
            type_system,
        ));
    }

    for struct_def in type_system.all_structs() {
        if struct_def.id != owning_struct_id
            || matches!(selected_type, Some(FieldType::CustomStruct(id)) if *id == struct_def.id)
        {
            options.push(FieldTypeOption::from_field_type(
                &FieldType::CustomStruct(struct_def.id),
                type_system,
            ));
        }
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

// ==================== STATE ====================

#[derive(Debug, Clone)]
pub struct StructEditorState {
    pub struct_id: Uuid,
    pub is_expanded: bool,
    pub name_input: String,
    pub new_field_name_input: String,
    pub new_field_type: FieldType,
    pub validation_error: Option<String>,
}

impl StructEditorState {
    pub fn new(struct_id: Uuid, struct_name: String) -> Self {
        Self {
            struct_id,
            is_expanded: false,
            name_input: struct_name,
            new_field_name_input: String::new(),
            new_field_type: FieldType::String,
            validation_error: None,
        }
    }
}

pub struct StructEditorView {
    pub editor_states: Vec<StructEditorState>,
}

impl StructEditorView {
    pub fn new() -> Self {
        Self {
            editor_states: Vec::new(),
        }
    }

    pub fn sync_with_type_system(&mut self, type_system: &TypeSystem) {
        // Remove states for deleted structs
        self.editor_states
            .retain(|state| type_system.get_struct(state.struct_id).is_some());

        // Add states for new structs
        for struct_def in type_system.all_structs() {
            if !self
                .editor_states
                .iter()
                .any(|s| s.struct_id == struct_def.id)
            {
                self.editor_states.push(StructEditorState::new(
                    struct_def.id,
                    struct_def.name.clone(),
                ));
            }
        }

        // Update names for existing states
        for state in &mut self.editor_states {
            if let Some(struct_def) = type_system.get_struct(state.struct_id) {
                if !state.is_expanded {
                    state.name_input = struct_def.name.clone();
                }
            }
        }
    }
}

// ==================== MESSAGES ====================

#[derive(Debug, Clone)]
pub enum Message {
    // Struct operations
    CreateNewStruct,
    DeleteStruct(Uuid),
    RenameStruct {
        struct_id: Uuid,
        new_name: String,
    },

    // Field operations
    AddField {
        struct_id: Uuid,
        name: String,
        field_type: FieldType,
    },
    RemoveField {
        struct_id: Uuid,
        field_id: Uuid,
    },
    UpdateFieldName {
        struct_id: Uuid,
        field_id: Uuid,
        new_name: String,
    },
    UpdateFieldType {
        struct_id: Uuid,
        field_id: Uuid,
        new_type: FieldType,
    },

    // UI state
    ToggleExpanded(Uuid),
    StructNameInputChanged {
        struct_id: Uuid,
        value: String,
    },
    NewFieldNameInputChanged {
        struct_id: Uuid,
        value: String,
    },
    NewFieldTypeChanged {
        struct_id: Uuid,
        field_type: FieldType,
    },
    SaveStruct(Uuid),

    // Undo/Redo
    Undo,
    Redo,
}

// ==================== UPDATE ====================

pub fn update(
    message: Message,
    type_system: &mut TypeSystem,
    editor_view: &mut StructEditorView,
) -> Task<Message> {
    match message {
        Message::CreateNewStruct => {
            let count = type_system.struct_count() + 1;
            match type_system.add_struct(format!("NewStruct{}", count)) {
                Ok(struct_id) => {
                    editor_view.sync_with_type_system(type_system);
                    if let Some(state) = editor_view
                        .editor_states
                        .iter_mut()
                        .find(|s| s.struct_id == struct_id)
                    {
                        state.is_expanded = true;
                    }
                }
                Err(e) => eprintln!("Error creating struct: {}", e),
            }
        }

        Message::DeleteStruct(struct_id) => match type_system.remove_struct(struct_id) {
            Ok(()) => {
                editor_view.sync_with_type_system(type_system);
            }
            Err(e) => {
                if let Some(state) = editor_view
                    .editor_states
                    .iter_mut()
                    .find(|s| s.struct_id == struct_id)
                {
                    state.validation_error = Some(e);
                }
            }
        },

        Message::RenameStruct {
            struct_id,
            new_name,
        } => {
            if let Some(state) = editor_view
                .editor_states
                .iter_mut()
                .find(|s| s.struct_id == struct_id)
            {
                match type_system.update_struct_name(struct_id, new_name) {
                    Ok(()) => {
                        state.validation_error = None;
                    }
                    Err(e) => {
                        state.validation_error = Some(e);
                    }
                }
            }
        }

        Message::AddField {
            struct_id,
            name,
            field_type,
        } => {
            if let Some(state) = editor_view
                .editor_states
                .iter_mut()
                .find(|s| s.struct_id == struct_id)
            {
                match type_system.add_struct_field(struct_id, name, field_type) {
                    Ok(_field_id) => {
                        state.new_field_name_input.clear();
                        state.new_field_type = FieldType::String;
                        state.validation_error = None;
                    }
                    Err(e) => {
                        state.validation_error = Some(e);
                    }
                }
            }
        }

        Message::RemoveField {
            struct_id,
            field_id,
        } => {
            if let Err(e) = type_system.remove_struct_field(struct_id, field_id) {
                if let Some(state) = editor_view
                    .editor_states
                    .iter_mut()
                    .find(|s| s.struct_id == struct_id)
                {
                    state.validation_error = Some(e);
                }
            }
        }

        Message::UpdateFieldName {
            struct_id,
            field_id,
            new_name,
        } => {
            if let Err(e) = type_system.update_struct_field_name(struct_id, field_id, new_name) {
                if let Some(state) = editor_view
                    .editor_states
                    .iter_mut()
                    .find(|s| s.struct_id == struct_id)
                {
                    state.validation_error = Some(e);
                }
            }
        }

        Message::UpdateFieldType {
            struct_id,
            field_id,
            new_type,
        } => {
            if let Err(e) = type_system.update_struct_field_type(struct_id, field_id, new_type) {
                if let Some(state) = editor_view
                    .editor_states
                    .iter_mut()
                    .find(|s| s.struct_id == struct_id)
                {
                    state.validation_error = Some(e);
                }
            }
        }

        Message::ToggleExpanded(struct_id) => {
            if let Some(state) = editor_view
                .editor_states
                .iter_mut()
                .find(|s| s.struct_id == struct_id)
            {
                state.is_expanded = !state.is_expanded;
                state.validation_error = None;
            }
        }

        Message::StructNameInputChanged { struct_id, value } => {
            if let Some(state) = editor_view
                .editor_states
                .iter_mut()
                .find(|s| s.struct_id == struct_id)
            {
                state.name_input = value;
                state.validation_error = None;
            }
        }

        Message::NewFieldNameInputChanged { struct_id, value } => {
            if let Some(state) = editor_view
                .editor_states
                .iter_mut()
                .find(|s| s.struct_id == struct_id)
            {
                state.new_field_name_input = value;
                state.validation_error = None;
            }
        }

        Message::NewFieldTypeChanged {
            struct_id,
            field_type,
        } => {
            if let Some(state) = editor_view
                .editor_states
                .iter_mut()
                .find(|s| s.struct_id == struct_id)
            {
                state.new_field_type = field_type;
            }
        }

        Message::SaveStruct(struct_id) => {
            if let Some(state) = editor_view
                .editor_states
                .iter_mut()
                .find(|s| s.struct_id == struct_id)
            {
                let new_name = state.name_input.clone();
                match type_system.update_struct_name(struct_id, new_name) {
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
    editor_view: &'a StructEditorView,
) -> Element<'a, Message> {
    let header = row![
        space::horizontal(),
        text("Custom Structs").size(18),
        space::horizontal(),
        button(icon_lucide::plus())
            .style(button::text)
            .on_press(Message::CreateNewStruct)
    ]
    .align_y(Alignment::Center)
    .padding(padding::right(10));

    let struct_vec = type_system
        .all_structs()
        .iter()
        .map(|struct_def| {
            let struct_name = struct_def.name.clone();
            let struct_id = struct_def.id;

            let editor_state = editor_view
                .editor_states
                .iter()
                .find(|s| s.struct_id == struct_id);

            let new_field_name = editor_state
                .map(|s| s.new_field_name_input.clone())
                .unwrap_or_default();
            let new_field_type = editor_state
                .map(|s| s.new_field_type.clone())
                .unwrap_or(FieldType::String);
            let new_field_type_option =
                FieldTypeOption::from_field_type(&new_field_type, type_system);
            let add_field_type_options =
                field_type_options(type_system, struct_id, Some(&new_field_type));

            // Build field rows
            let field_rows = struct_def
                .fields
                .iter()
                .map(|field| {
                    let field_name = field.name.clone();
                    let field_id = field.id;
                    let field_type_option =
                        FieldTypeOption::from_field_type(&field.field_type, type_system);
                    let type_opts_inner =
                        field_type_options(type_system, struct_id, Some(&field.field_type));

                    row![
                        text_input::<Message, Theme, iced::Renderer>("field_name", &field_name,)
                            .on_input(move |new_name| {
                                Message::UpdateFieldName {
                                    struct_id,
                                    field_id,
                                    new_name,
                                }
                            }),
                        pick_list(
                            type_opts_inner,
                            Some(field_type_option),
                            move |selected: FieldTypeOption| {
                                Message::UpdateFieldType {
                                    struct_id,
                                    field_id,
                                    new_type: selected.field_type,
                                }
                            }
                        )
                        .width(150),
                        button(icon_lucide::trash_2())
                            .style(styles::button::cancel)
                            .on_press(Message::RemoveField {
                                struct_id,
                                field_id
                            })
                    ]
                    .spacing(10)
                    .padding(5)
                    .align_y(Alignment::Center)
                    .into()
                })
                .collect::<Vec<_>>();

            collapsible(
                &struct_name,
                column![
                    column![
                        text("Struct Name"),
                        row![
                            text_input("Struct Name", &struct_name).on_input(move |name| {
                                Message::RenameStruct {
                                    struct_id,
                                    new_name: name,
                                }
                            }),
                            space().width(150),
                            button(icon_lucide::trash_2()).style(styles::button::invisible)
                        ]
                        .spacing(10)
                        .padding(5)
                    ]
                    .spacing(5)
                    .padding([5, 10]),
                    container(text("Fields"),).padding(Padding {
                        top: 10.0,
                        right: 10.0,
                        bottom: 0.0,
                        left: 10.0
                    }),
                    column(field_rows).padding([5, 10]),
                    // Add new field row
                    container(
                        row![
                            text_input("field_name", &new_field_name).on_input(move |v| {
                                Message::NewFieldNameInputChanged {
                                    struct_id,
                                    value: v,
                                }
                            }),
                            pick_list(
                                add_field_type_options,
                                Some(new_field_type_option),
                                move |selected: FieldTypeOption| Message::NewFieldTypeChanged {
                                    struct_id,
                                    field_type: selected.field_type,
                                }
                            )
                            .width(150),
                            button(icon_lucide::plus().center())
                                .style(button::text)
                                .on_press(Message::AddField {
                                    struct_id,
                                    name: new_field_name.clone(),
                                    field_type: new_field_type.clone(),
                                })
                        ]
                        .spacing(10)
                        .padding(5)
                        .align_y(Alignment::Center)
                    )
                    .padding([5, 10]),
                ],
            )
            .expand_icon(icon::collapsed())
            .collapse_icon(icon::expanded())
            .action_icon(
                button(icon_lucide::trash_2())
                    .style(styles::button::cancel)
                    .on_press(Message::DeleteStruct(struct_id))
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
                CollapsibleGroup::new(struct_vec).spacing(5.0)
            ])
            .padding(padding::right(10)),
        )
        .height(Length::Fill),
    )
    .height(Length::Fill)
    .width(400.0)
    .into()
}

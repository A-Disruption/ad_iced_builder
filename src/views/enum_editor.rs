use iced::{
    widget::{button, column, container, row, text, text_input, scrollable, space},
    Alignment, Element, Length, Task, Theme, Background, Border, Color, padding, Padding
};
use uuid::Uuid;

use crate::icon;
use crate::enum_builder::*;
use crate::styles;
use crate::styles::container::*;
use widgets::collapsible::{Collapsible, CollapsibleGroup, collapsible};

// ==================== STATE ====================

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
        self.editor_states.retain(|state| {
            type_system.get_enum(state.enum_id).is_some()
        });
        
        // Add states for new enums
        for enum_def in type_system.all_enums() {
            if !self.editor_states.iter().any(|s| s.enum_id == enum_def.id) {
                self.editor_states.push(EnumEditorState::new(
                    enum_def.id,
                    enum_def.name.clone(),
                ));
            }
        }
        
        // Update names for existing states
        for state in &mut self.editor_states {
            if let Some(enum_def) = type_system.get_enum(state.enum_id) {
                // Only update if not currently being edited
                if !state.is_expanded {
                    state.name_input = enum_def.name.clone();
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
    RenameEnum { enum_id: Uuid, new_name: String },
    
    // Variant operations
    AddVariant { enum_id: Uuid, name: String },
    RemoveVariant { enum_id: Uuid, variant_id: Uuid },
    UpdateVariant { enum_id: Uuid, variant_id: Uuid, new_name: String },
    
    // UI state
    ToggleExpanded(Uuid),
    EnumNameInputChanged { enum_id: Uuid, value: String },
    NewVariantInputChanged { enum_id: Uuid, value: String },
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
            match type_system.add_enum(
                format!("NewEnum{}", count),
                vec!["Variant1".to_string()]
            ) {
                Ok(enum_id) => {
                    editor_view.sync_with_type_system(type_system);
                    // Expand the new enum
                    if let Some(state) = editor_view.editor_states.iter_mut()
                        .find(|s| s.enum_id == enum_id) {
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
                    if let Some(state) = editor_view.editor_states.iter_mut()
                        .find(|s| s.enum_id == enum_id) {
                        state.validation_error = Some(e);
                    }
                }
            }
        }
        
        Message::RenameEnum { enum_id, new_name } => {
            if let Some(state) = editor_view.editor_states.iter_mut()
                .find(|s| s.enum_id == enum_id) {
                
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
        
        Message::AddVariant { enum_id, name } => {
            if let Some(state) = editor_view.editor_states.iter_mut()
                .find(|s| s.enum_id == enum_id) {
                
                match type_system.add_variant(enum_id, name) {
                    Ok(_variant_id) => {
                        state.new_variant_input.clear();
                        state.validation_error = None;
                    }
                    Err(e) => {
                        state.validation_error = Some(e);
                    }
                }
            }
        }
        
        Message::RemoveVariant { enum_id, variant_id } => {
            if let Err(e) = type_system.remove_variant(enum_id, variant_id) {
                if let Some(state) = editor_view.editor_states.iter_mut()
                    .find(|s| s.enum_id == enum_id) {
                    state.validation_error = Some(e);
                }
            }
        }
        
        Message::UpdateVariant { enum_id, variant_id, new_name } => {
            if let Err(e) = type_system.update_variant(enum_id, variant_id, new_name) {
                if let Some(state) = editor_view.editor_states.iter_mut()
                    .find(|s| s.enum_id == enum_id) {
                    state.validation_error = Some(e);
                }
            }
        }
        
        Message::ToggleExpanded(enum_id) => {
            if let Some(state) = editor_view.editor_states.iter_mut()
                .find(|s| s.enum_id == enum_id) {
                state.is_expanded = !state.is_expanded;
                state.validation_error = None;
            }
        }
        
        Message::EnumNameInputChanged { enum_id, value } => {
            if let Some(state) = editor_view.editor_states.iter_mut()
                .find(|s| s.enum_id == enum_id) {
                state.name_input = value;
                state.validation_error = None;
            }
        }
        
        Message::NewVariantInputChanged { enum_id, value } => {
            if let Some(state) = editor_view.editor_states.iter_mut()
                .find(|s| s.enum_id == enum_id) {
                state.new_variant_input = value;
                state.validation_error = None;
            }
        }
        
        Message::SaveEnum(enum_id) => {
            if let Some(state) = editor_view.editor_states.iter_mut()
                .find(|s| s.enum_id == enum_id) {
                
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
        button(icon::plus()).style(button::text).on_press(Message::CreateNewEnum)
    ].align_y(Alignment::Center).padding(padding::right(10));

    let enum_vec = type_system.all_enums()
        .iter()
        .map(|enum_def| {
            let enum_name = enum_def.name.clone();
            let enum_id = enum_def.id;
            // Build the variant rows for this enum
            let variant_rows = enum_def.variants
                .iter()
                .map(move |variant| {
                    let variant_name = variant.name.clone();
                    let variant_id = variant.id;
                    row![
                        text_input::<Message, Theme, iced::Renderer>(
                            "Variant Name",
                            &variant_name,
                        )
                        .on_input(move |new_name| {
                            Message::UpdateVariant {
                                enum_id: enum_id,
                                variant_id: variant_id,
                                new_name,
                            }
                        }),
                        button(icon::trash()).style(styles::button::cancel).on_press(Message::RemoveVariant { enum_id: enum_id, variant_id: variant_id})
                    ]
                    .spacing(10)
                    .padding(5)
                    .into()
                })
                .collect::<Vec<_>>();

            collapsible(
                &enum_name,
                column![
                    column![
                        text("Enum Name"),
                        text_input("Enum Name", &enum_name)
                            .on_input(move |name| Message::RenameEnum {
                                enum_id: enum_id,
                                new_name: name
                            })
                    ]
                    .spacing(5)
                    .padding([5, 10]),
                    
                    container(
                        text("Enum Variants"),
                    ).padding(Padding { top: 10.0, right: 10.0, bottom: 0.0, left: 10.0}),

                    // Spread all the variant rows here
                    column(variant_rows).padding(5),
                    container(
                        button(icon::plus().center()).style(button::text).on_press(Message::AddVariant { enum_id, name: "Variant".to_string() })
                    ).padding(5)
                ],
            )
            .expand_icon(icon::collapsed())
            .collapse_icon(icon::expanded())
            .into()
        })
        .collect();

    container(
        scrollable(
            container(
                column![
                    header,
                    CollapsibleGroup::new(enum_vec).spacing(5.0)
                ]
            ).padding(padding::right(10))
        ).height(Length::Fill)
    )
    .height(Length::Fill)
    .width(400.0)
    .into()   
}
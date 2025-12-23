use iced::widget::{button, column, container, markdown, row, rule, space, text};
use iced::{Alignment, Element, Length, Task, Theme};
use widgets::tree::{tree_handle, branch, DropInfo, DropPosition, Branch};
use std::collections::{HashSet, BTreeMap};
use arboard::Clipboard;
use uuid::Uuid;

use crate::icon;
use crate::styles;
use crate::controls::*;
use crate::data_structures::widget_hierarchy::WidgetHierarchy;
use crate::data_structures::properties::messages::PropertyChange;
use crate::data_structures::types::types::{WidgetId, WidgetType, Widget, AppView};
use crate::data_structures::types::type_implementations::Orientation;
use widgets::generic_overlay::overlay_button;
use crate::enum_builder::TypeSystem;
use crate::views::enum_editor::{self, EnumEditorView};

#[derive(Debug, Clone)]
pub enum Message {
    // Tree Hierarchy
    TreeMove(DropInfo),

    // Widget Operations
    SelectWidgets(HashSet<usize>),
    DeleteWidget(WidgetId),
    AddChild(WidgetId, WidgetType),
    PropertyChanged(WidgetId, PropertyChange),
    SwapKind(WidgetId),

    // Interactive widget messages
    ButtonPressed(WidgetId),
    TextInputChanged(WidgetId, String),
    Submitted(WidgetId),
    TextPasted(WidgetId, String),
    CheckboxToggled(WidgetId, bool),
    RadioSelected(WidgetId, usize),
    SliderChanged(WidgetId, f32),
    TogglerToggled(WidgetId, bool),
    PickListSelected(WidgetId, String),
    ComboBoxOnInput(WidgetId, String),
    ComboBoxSelected(WidgetId, String),
    ComboBoxOnOptionHovered(WidgetId, String),
    ComboBoxOnClose(WidgetId),
    ComboBoxOnOpen(WidgetId),
    Noop,

    OpenEnumEditor,
    EnumEditor(enum_editor::Message),

    // Wrapping operations
    WrapSelectedInContainer(WidgetType),  // Wraps selection in Row/Column/MouseArea/Tooltip
    
    // Batch editing operations  
    BatchPropertyChanged(PropertyChange), // Applies property to all selected widgets

    OverlayOpened(iced::Point, iced::Size),

    CodeTabSelected(String),
    CopyCode(String),
}



pub fn update(
    message: Message,
    hierarchy: &mut WidgetHierarchy,
    type_system: &mut TypeSystem,
    enum_editor: &mut EnumEditorView,
) -> Task<Message> {
    match message {
        Message::TreeMove(drop_info) => {
            if let Some(target_external_id) = drop_info.target_id {
                let target_id = WidgetId(target_external_id);
                
                // Convert all dragged external IDs to WidgetIds
                let dragged_ids: Vec<WidgetId> = drop_info.dragged_ids.iter()
                    .map(|&id| WidgetId(id))
                    .collect();
                
                // Process moves one at a time to handle index adjustments properly
                match drop_info.position {
                    DropPosition::Into => {
                        // Moving into a target - append each item
                        for dragged_id in dragged_ids {
                            if dragged_id != target_id {
                                // When moving into, get the target's current child count for proper indexing
                                let target_child_count = hierarchy.get_widget_by_id(target_id)
                                    .map(|w| w.children.len())
                                    .unwrap_or(0);
                                let _ = hierarchy.move_widget(
                                    dragged_id, 
                                    target_id, 
                                    target_child_count
                                );
                            }
                        }
                    }
                    DropPosition::Before | DropPosition::After => {
                        if let Some(parent_id) = hierarchy.find_parent_id(target_id) {
                            for dragged_id in dragged_ids {
                                if dragged_id == target_id {
                                    continue;
                                }
                                
                                // Get fresh target position each time since moves can shift indices
                                let target_index = hierarchy.get_widget_by_id(parent_id)
                                    .and_then(|parent| {
                                        parent.children.iter()
                                            .position(|c| c.id == target_id)
                                    })
                                    .unwrap_or(0);
                                
                                let insert_index = match drop_info.position {
                                    DropPosition::Before => target_index,
                                    DropPosition::After => target_index + 1,
                                    _ => target_index,
                                };
                                
                                let _ = hierarchy.move_widget(dragged_id, parent_id, insert_index);
                            }
                        }
                    }
                }
            }
        }

        Message::SelectWidgets(external_ids) => {
            let widget_ids: HashSet<WidgetId> = external_ids.iter()
                .map(|&id| WidgetId(id))
                .collect();
            hierarchy.set_selected_ids(widget_ids);
        }
        
        Message::DeleteWidget(id) => {
            let _ = hierarchy.delete_widget(id);
        }
        
        Message::AddChild(parent_id, widget_type) => {
            println!("Adding {:?} to parent {:?}", widget_type, parent_id);
            if let Ok(new_id) = hierarchy.add_child(parent_id, widget_type) {
                println!("Successfully added with id {:?}", new_id);
                // Debug print the tree
                debug_print_widget(&hierarchy.root(), 0);
            } else {
                println!("Failed to add child");
            }
        }
        
        Message::PropertyChanged(id, change) => {
            hierarchy.apply_property_change(id, change.clone(), type_system);

            match hierarchy.get_widget_by_id(id) {
                Some(widget) => { 
                    if widget.widget_type == WidgetType::Space {
                        match change {
                            PropertyChange::Orientation(Orientation::Horizontal) => {
                                hierarchy.apply_property_change(id, PropertyChange::Width(Length::Fill), type_system);
                                hierarchy.apply_property_change(id, PropertyChange::Height(Length::Shrink), type_system);
                            }
                            PropertyChange::Orientation(Orientation::Vertical) => {
                                hierarchy.apply_property_change(id, PropertyChange::Width(Length::Shrink), type_system);
                                hierarchy.apply_property_change(id, PropertyChange::Height(Length::Fill), type_system);
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        Message::SwapKind(id) => {
            hierarchy.swap_kind(id);
        }

        // Interactive widget messages
        Message::ButtonPressed(id) => {
            println!("{:?}, button pressed", id);
        }
        
        Message::TextInputChanged(id, value) => {
            hierarchy.apply_property_change(id, PropertyChange::TextInputValue(value), type_system);
        }

        Message::Submitted(id) => { println!("{:?}, text_input submitted.", id); }

        Message::TextPasted(id, value) => {
            println!("{:?}, text pasted.", id);
            hierarchy.apply_property_change(id, PropertyChange::TextInputValue(value), type_system)
        }
        
        Message::CheckboxToggled(id, checked) => {
            hierarchy.apply_property_change(id, PropertyChange::CheckboxChecked(checked), type_system);
        }
        
        Message::RadioSelected(id, index) => {
            hierarchy.apply_property_change(id, PropertyChange::RadioSelectedIndex(index), type_system);
        }
        
        Message::SliderChanged(id, value) => {
            hierarchy.apply_property_change(id, PropertyChange::SliderValue(value), type_system);
        }
        
        Message::TogglerToggled(id, active) => {
            hierarchy.apply_property_change(id, PropertyChange::TogglerActive(active), type_system);
        }
        
        Message::PickListSelected(id, index) => {
            hierarchy.apply_property_change(id, PropertyChange::PickListSelected(Some(index)), type_system);
        }

        Message::ComboBoxOnInput(id, value) => {
            let props = &hierarchy.get_widget_by_id(id).unwrap().properties;
            if props.combobox_use_on_input {
                println!("combobox {:?} input text: {}", id, value);
            }
        }
        Message::ComboBoxSelected(id, value) => {
            println!("combobox selected: {:?}", value);
            hierarchy.apply_property_change(id, PropertyChange::ComboBoxSelected(Some(value)), type_system);
        }
        Message::ComboBoxOnOpen(id) => {
            let props = &hierarchy.get_widget_by_id(id).unwrap().properties;
            if props.combobox_use_on_open {
                println!("combobox {:?} opened!", id);
            }
        }
        Message::ComboBoxOnClose(id) => {
            let props = &hierarchy.get_widget_by_id(id).unwrap().properties;
            if props.combobox_use_on_close {
                println!("combobox {:?} closed!", id);
            }
        }
        Message::ComboBoxOnOptionHovered(id, options) => {
            let props = &hierarchy.get_widget_by_id(id).unwrap().properties;
            if props.combobox_use_on_option_hovered {
                println!("combobox option hovered: {:?}", options);
            }
        }
        Message::Noop => {
            // Do nothing - for preview-only interactions
        }

        Message::OpenEnumEditor => {
            // open the enum editor view
        }

        Message::EnumEditor(msg) => {
            let task = enum_editor::update(msg, type_system, enum_editor)
                .map(Message::EnumEditor);

            return task
        }

        Message::WrapSelectedInContainer(container_type) => {
            match hierarchy.wrap_selected_in_container(container_type) {
                Ok(wrapper_id) => {
                    println!("Successfully wrapped widgets in {:?} with id {:?}", 
                                container_type, wrapper_id);
                }
                Err(e) => {
                    println!("Failed to wrap widgets: {}", e);
                    // TODO: Show error to user (could add a status message field)
                }
            }
        }
        
        Message::BatchPropertyChanged(change) => {
            hierarchy.apply_property_to_all_selected(change, type_system);
        }
        Message::OverlayOpened(overlay_position, overlay_size) => {
            println!("Overlay was opened. \n\tPosition: {}, \n\tSize: {:?}", overlay_position, overlay_size)
        },
        Message::CopyCode(code) => {
            if let Ok(mut clipboard) = Clipboard::new() {
                let _ = clipboard.set_text(code);
            }
        }
        Message::CodeTabSelected(_) => {} // Handled in main.rs
    }
    
    Task::none()
}

pub fn view<'a>(
    hierarchy: &'a WidgetHierarchy,
    type_system: &'a TypeSystem,
    theme: &'a Theme,
    views: &'a BTreeMap<Uuid, AppView>,
) -> Element<'a, Message> {
    let widget = hierarchy.root();
    let overlay_content = build_editor_for_widget(hierarchy, type_system, theme, widget, widget.id, views);

    // Determine if this widget can be swapped and the button label
    let swap_label: Option<iced::advanced::widget::Text<'_, Theme, iced::Renderer>> = match widget.widget_type {
        WidgetType::Row        => Some(icon::swap()), 
        WidgetType::Column     => Some(icon::swap()),
        WidgetType::Container  => Some(icon::swap()),
        WidgetType::Scrollable => Some(icon::swap()),
        _ => None,
    };

    // Optional Swap button element
    let swap_button: Option<Element<Message>> = swap_label.map(|label| {
        button(label)
            .on_press(Message::SwapKind(widget.id))
            .style(button::text)
            .into()
    });

    let disabled_delete_button: Element<Message> = { // Don't allow deleting root
                button(icon::trash())
                    .style(styles::button::cancel)
                    .into()
            };

    //let place_holder = button("  ").style(button::text);

    let mut children = Vec::new();

    for child in &widget.children {
        children.push(build_tree_item(hierarchy, type_system, theme, child, views));
    }

    let root = branch(
        row![
            container(text(format!("{}", widget.name))).padding(5),
            space::horizontal(),
            swap_button,

            // Create overlay button with this widget's specific content
            overlay_button(
                icon::edit(),
                format!("Editing {}", widget.name),
                overlay_content
            )
            .close_on_click_outside()
            .resizable(widgets::generic_overlay::ResizeMode::Always)
            .overlay_width(500.0)
            .overlay_height(750.0)
            .style(styles::button::edit),

            disabled_delete_button
            //place_holder

        ].spacing(5)
    ).block_dragging()
    .with_children(children)
    .with_id(widget.id.0);

    let mut tree = tree_handle(
        vec!(root)
    )
//    .expand_icon(icon::collapsed())
//    .collapse_icon(icon::expanded())
    .on_drop(Message::TreeMove)
    .on_select(|selected_ids| Message::SelectWidgets(selected_ids));

    tree = tree.reset_order_state();

    tree.into()

}

fn build_tree_item<'a>(
    hierarchy: &'a WidgetHierarchy,
    type_system: &'a TypeSystem,
    theme: &'a Theme,
    widget: &'a Widget,
    views: &'a BTreeMap<Uuid, AppView>,
) -> Branch<'a, Message, Theme, iced::Renderer> {     

    let is_selected = hierarchy.selected_ids().contains(&widget.id);
    let selection_count = hierarchy.selected_ids().len();   

    let is_first_child_of_root = hierarchy.root().children.first()
    .map(|c| c.id == widget.id)
    .unwrap_or(false);

    // Create the overlay content for this specific widget
    let overlay_content = build_editor_for_widget(hierarchy, type_system, theme, widget, widget.id, views);
    // Determine if this widget can be swapped and the button label
    let swap_label: Option<iced::advanced::widget::Text<'_, Theme, iced::Renderer>> = match widget.widget_type {
        WidgetType::Row        => Some(icon::swap()), 
        WidgetType::Column     => Some(icon::swap()),
        WidgetType::Container  => Some(icon::swap()),
        WidgetType::Scrollable => Some(icon::swap()),
        _ => None,
    };

    // Optional Swap button element
    let swap_button: Option<Element<Message>> = swap_label.map(|label| {
        button(label)
            .on_press(Message::SwapKind(widget.id))
            .style(button::text)
            .into()
    });

    let delete_button: Option<Element<Message>> = if widget.id.0 != 0 { // Don't allow deleting root
                Some(button(icon::trash())
                    .on_press(Message::DeleteWidget(widget.id))
                    .style(styles::button::cancel)
                    .into())
            } else {
                None
            };

    let edit_button: Element<Message> = if selection_count == 1 {
        // Original single-widget edit overlay
        Some(overlay_button(
            icon::edit(),
            format!("Editing {}", widget.name),
            build_editor_for_widget(hierarchy, type_system, theme, widget, widget.id, views)
        )
        .close_on_click_outside()
        .overlay_width(500.0)
        .overlay_height(750.0)
        .resizable(widgets::generic_overlay::ResizeMode::Always)
        .on_open(|overlay_position, overlay_size| Message::OverlayOpened(overlay_position, overlay_size))
        .style(styles::button::edit)).into()
    } else if is_selected {
        // Show indicator that this is in batch selection
        Some(button(text("Selected"))
            .style(button::secondary)).into()
    } else {
        Some(button(text("Edit"))
            .style(button::text)).into()
    };

    let mut children = Vec::new();

    for child in &widget.children {
        children.push(build_tree_item(hierarchy, type_system, theme, child, views));
    }

    let branch = match widget.widget_type {
        WidgetType::Row | WidgetType::Column | WidgetType::Container | WidgetType::Scrollable | WidgetType::Tooltip | WidgetType::MouseArea => {

            let content = row![
                    container(text(format!("{}", widget.name))).padding(5),

                    space::horizontal(),

                    swap_button,

                    // Create overlay button with this widget's specific content
                    edit_button,

                    delete_button
            ].spacing(5);

            if !is_first_child_of_root {
                branch(
                    content
                ).with_id(widget.id.0)
                .with_children(children)
                .accepts_drops()
            } else {
                branch(
                    content
                ).with_id(widget.id.0)
                .with_children(children)
                .accepts_drops()
                .block_dragging()
            }

        }
        _ => {
            let content = row![
                    container(text(format!("{}", widget.name))).padding(5),

                    space::horizontal(),

                    swap_button,

                    // Create overlay button with this widget's specific content
                    overlay_button(
                        icon::edit(),
                        format!("Editing {}", widget.name),
                        overlay_content
                    )
                    .overlay_width(500.0)
                    .overlay_height(750.0)
                    .close_on_click_outside()
                    .resizable(widgets::generic_overlay::ResizeMode::Always)
                    .on_open(|overlay_position, overlay_size| Message::OverlayOpened(overlay_position, overlay_size))
                    .style(styles::button::edit),

                    delete_button
            ].spacing(5);
            
            if !is_first_child_of_root {
                branch(
                    content
                ).with_id(widget.id.0)
            } else { // Block dragging for first child of root
                branch(
                    content
                ).with_id(widget.id.0).block_dragging()
            }

        }
    };

    branch
}

fn build_editor_for_widget<'a>(
    hierarchy: &'a WidgetHierarchy,
    type_system: &'a TypeSystem,
    theme: &'a Theme,
    widget: &Widget, 
    widget_id: WidgetId,
    views: &'a BTreeMap<Uuid, AppView>,
) -> Element<'a, Message> {
    let controls_view: Element<Message> = match widget.widget_type {
        WidgetType::Container       => container_controls(&hierarchy, widget_id, theme, &type_system),
        WidgetType::Scrollable      => scrollable_controls(&hierarchy, widget_id, theme, &type_system),
        WidgetType::Row             => row_controls(&hierarchy, widget_id, theme, &type_system),
        WidgetType::Column          => column_controls(&hierarchy, widget_id, theme, &type_system),
        WidgetType::Button          => button_controls(&hierarchy, widget_id, theme, &type_system),
        WidgetType::Text            => text_controls(&hierarchy, widget_id, theme, &type_system),
        WidgetType::TextInput       => text_input_controls(&hierarchy, widget_id, theme, &type_system),
        WidgetType::Checkbox        => checkbox_controls(&hierarchy, widget_id, theme, &type_system),
        WidgetType::Radio           => radio_controls(&hierarchy, widget_id, theme, &type_system),
        WidgetType::Toggler         => toggler_controls(&hierarchy, widget_id, theme, &type_system),
        WidgetType::PickList        => picklist_controls(&hierarchy, widget_id, theme, &type_system),
        WidgetType::Slider          => slider_controls(&hierarchy, widget_id, theme, &type_system),
        WidgetType::VerticalSlider  => vertical_slider_controls(&hierarchy, widget_id, theme, &type_system),
        WidgetType::Rule            => rule_controls(&hierarchy, widget_id, theme, &type_system),
        WidgetType::Space           => space_controls(&hierarchy, widget_id, theme, &type_system),
        WidgetType::ProgressBar     => progress_controls(&hierarchy, widget_id, theme, &type_system),
        WidgetType::Image           => image_controls(&hierarchy, widget_id, theme, &type_system),
        WidgetType::Svg             => svg_controls(&hierarchy, widget_id, theme, &type_system),
        WidgetType::Tooltip         => tooltip_controls(&hierarchy, widget_id, theme, &type_system),
        WidgetType::ComboBox        => combobox_controls(&hierarchy, widget_id, theme, &type_system),
        WidgetType::Markdown        => markdown_controls(&hierarchy, widget_id, theme, &type_system),
        WidgetType::MouseArea       => mousearea_controls(&hierarchy, widget_id, theme, &type_system),
        WidgetType::QRCode          => qrcode_controls(&hierarchy, widget_id, theme, &type_system),
        WidgetType::Stack           => stack_controls(&hierarchy, widget_id, theme, &type_system),
        WidgetType::Themer          => themer_controls(&hierarchy, widget_id, theme, &type_system),
        WidgetType::Pin             => pin_controls(&hierarchy, widget_id, theme, &type_system),
        WidgetType::ViewReference   => view_reference_controls(&hierarchy, widget_id, theme, &type_system, views),
        _ => column![text("Editor not implemented for this widget type")].into(),
    };

    column![
        row![
            text(format!("Editing: {}", widget.name)).size(20),
            text(format!("{}", widget.widget_type)).size(10).align_y(Alignment::End),
        ],
        rule::horizontal(5),
        controls_view,
    ]
    .spacing(10)
    .padding(20)
    .into()
}

pub fn debug_print_widget(widget: &Widget, depth: usize) {
    println!("{}- {:?} (id: {:?}, children: {})", 
        "  ".repeat(depth), 
        widget.widget_type, 
        widget.id,
        widget.children.len()
    );
    for child in &widget.children {
        debug_print_widget(child, depth + 1);
    }
}
use iced::widget::{button, container, row, space, text, text_editor};
use iced::{Element, Length, Task, Theme};
use widgets::tree::{tree_handle, branch, DropInfo, DropPosition, Branch};
use std::collections::{HashSet, BTreeMap};
use iced::clipboard;
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
use crate::views::enum_editor::EnumEditorView;
use crate::views::theme_and_stylefn_builder::CustomThemes;

#[derive(Debug, Clone)]
pub enum Message {
    // Tree Hierarchy
    TreeMove(DropInfo),

    // Widget Operations
    SelectWidgets(HashSet<usize>),
    DeleteWidget(WidgetId),
    PropertyChanged(WidgetId, PropertyChange),
    SwapKind(WidgetId),

    OpenEnumEditor,
//    EnumEditor(enum_editor::Message),

    OverlayOpened(WidgetId, iced::Point, iced::Size),
    OverlayClosed(WidgetId),

    CodeTabSelected(String),
    CopyCode(String),

    // text_editor actions for code views
    CodeViewEdit(text_editor::Action),
    WidgetPreviewEdit(text_editor::Action),
}



pub fn update(
    message: Message,
    hierarchy: &mut WidgetHierarchy,
    type_system: &mut TypeSystem,
    _enum_editor: &mut EnumEditorView,
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
        Message::OpenEnumEditor => {
            // open the enum editor view
        }
        Message::OverlayOpened(_id, _point, _size) => {} // Handled in main.rs
        Message::OverlayClosed(_id) => {}       // Handled in main.rs
        Message::CopyCode(code) => {
            return clipboard::write(code)
        }
        Message::CodeTabSelected(_) => {} // Handled in main.rs
        Message::CodeViewEdit(_) => {}   // Handled in main.rs
        Message::WidgetPreviewEdit(_) => {} // Handled in main.rs
    }
    
    Task::none()
}

pub fn view<'a>(
    hierarchy: &'a WidgetHierarchy,
    type_system: &'a TypeSystem,
    theme: &'a Theme,
    views: &'a BTreeMap<Uuid, AppView>,
    custom_themes: &'a CustomThemes,
    widget_preview_content: &'a text_editor::Content,
    open_editor_widget_id: Option<WidgetId>,
) -> Element<'a, Message> {
    let widget = hierarchy.root();
    let overlay_content = build_editor_for_widget(hierarchy, type_system, theme, widget, widget.id, views, custom_themes, widget_preview_content, open_editor_widget_id);

    let display_name = if widget.properties.widget_name.is_empty() {
        &widget.name
    } else {
        &widget.properties.widget_name
    };

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

    let mut children = Vec::new();

    for child in &widget.children {
        children.push(build_tree_item(hierarchy, type_system, theme, child, views, custom_themes, widget_preview_content, open_editor_widget_id));
    }

    let root_widget_id = widget.id;
    let root = branch(
        row![
            container(text(format!("{}", display_name))).padding(5),
            space::horizontal(),
            swap_button,

            // Create overlay button with this widget's specific content
            overlay_button(
                icon::edit(),
                format!("Editing {}", display_name),
                overlay_content
            )
            .close_on_click_outside()
            .resizable(widgets::generic_overlay::ResizeMode::Always)
            .overlay_width(600.0)
            .overlay_height(750.0)
            .on_open(move |pos, size| Message::OverlayOpened(root_widget_id, pos, size))
            .on_close(move || Message::OverlayClosed(root_widget_id))
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
    .expand_icon(icon::collapsed())
    .collapse_icon(icon::expanded())
    .quick()
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
    custom_themes: &'a CustomThemes,
    widget_preview_content: &'a text_editor::Content,
    open_editor_widget_id: Option<WidgetId>,
) -> Branch<'a, Message, Theme, iced::Renderer> {     

    let is_selected = hierarchy.selected_ids().contains(&widget.id);
    let selection_count = hierarchy.selected_ids().len();

    let display_name = if widget.properties.widget_name.is_empty() {
        &widget.name
    } else {
        &widget.properties.widget_name
    };

    let is_first_child_of_root = hierarchy.root().children.first()
    .map(|c| c.id == widget.id)
    .unwrap_or(false);

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

    let widget_id = widget.id;
    let edit_button: Element<Message> = if selection_count == 1 {
        // Original single-widget edit overlay
        Some(overlay_button(
            icon::edit(),
            format!("Editing {}", display_name),
            build_editor_for_widget(hierarchy, type_system, theme, widget, widget.id, views, custom_themes, widget_preview_content, open_editor_widget_id)
        )
        .close_on_click_outside()
        .overlay_width(600.0)
        .overlay_height(750.0)
        .resizable(widgets::generic_overlay::ResizeMode::Always)
        .on_open(move |pos, size| Message::OverlayOpened(widget_id, pos, size))
        .on_close(move || Message::OverlayClosed(widget_id))
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
        children.push(build_tree_item(hierarchy, type_system, theme, child, views, custom_themes, widget_preview_content, open_editor_widget_id));
    }

    let branch = match widget.widget_type {
        WidgetType::Row | WidgetType::Column | WidgetType::Container | WidgetType::Scrollable | WidgetType::Tooltip | WidgetType::MouseArea | WidgetType::Pin | WidgetType::Button | WidgetType::Stack | WidgetType::Themer | WidgetType::Grid => {

            let content = row![
                    container(text(format!("{}", display_name))).padding(5),

                    space::horizontal(),

                    swap_button,

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
                    container(text(format!("{}",display_name))).padding(5),

                    space::horizontal(),

                    swap_button,

                    edit_button,

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
    custom_styles: &'a CustomThemes,
    preview_content: &'a text_editor::Content,
    open_editor_widget_id: Option<WidgetId>,
) -> Element<'a, Message> {
    // Return empty element if this widget's overlay isn't open
    // This avoids expensive text_editor + tree-sitter construction for closed overlays
    if Some(widget_id) != open_editor_widget_id {
        return space::horizontal().width(0).into();
    }

    let controls_view: Element<Message> = match widget.widget_type {
        WidgetType::Container       => container_controls(&hierarchy, widget_id, theme, &type_system, custom_styles, preview_content),
        WidgetType::Scrollable      => scrollable_controls(&hierarchy, widget_id, theme, &type_system, custom_styles, preview_content),
        WidgetType::Row             => row_controls(&hierarchy, widget_id, theme, &type_system, custom_styles, preview_content),
        WidgetType::Column          => column_controls(&hierarchy, widget_id, theme, &type_system, custom_styles, preview_content),
        WidgetType::Button          => button_controls(&hierarchy, widget_id, theme, &type_system, custom_styles, preview_content),
        WidgetType::Text            => text_controls(&hierarchy, widget_id, theme, &type_system, custom_styles, preview_content),
        WidgetType::TextInput       => text_input_controls(&hierarchy, widget_id, theme, &type_system, custom_styles, preview_content),
        WidgetType::Checkbox        => checkbox_controls(&hierarchy, widget_id, theme, &type_system, custom_styles, preview_content),
        WidgetType::Radio           => radio_controls(&hierarchy, widget_id, theme, &type_system, custom_styles, preview_content),
        WidgetType::Toggler         => toggler_controls(&hierarchy, widget_id, theme, &type_system, custom_styles, preview_content),
        WidgetType::PickList        => picklist_controls(&hierarchy, widget_id, theme, &type_system, custom_styles, preview_content),
        WidgetType::Slider          => slider_controls(&hierarchy, widget_id, theme, &type_system, custom_styles, preview_content),
        WidgetType::VerticalSlider  => vertical_slider_controls(&hierarchy, widget_id, theme, &type_system, custom_styles, preview_content),
        WidgetType::Rule            => rule_controls(&hierarchy, widget_id, theme, &type_system, custom_styles, preview_content),
        WidgetType::Space           => space_controls(&hierarchy, widget_id, theme, &type_system, custom_styles, preview_content),
        WidgetType::ProgressBar     => progress_controls(&hierarchy, widget_id, theme, &type_system, custom_styles, preview_content),
        WidgetType::Image           => image_controls(&hierarchy, widget_id, theme, &type_system, custom_styles, preview_content),
        WidgetType::Svg             => svg_controls(&hierarchy, widget_id, theme, &type_system, custom_styles, preview_content),
        WidgetType::Tooltip         => tooltip_controls(&hierarchy, widget_id, theme, &type_system, custom_styles, preview_content),
        WidgetType::ComboBox        => combobox_controls(&hierarchy, widget_id, theme, &type_system, custom_styles, preview_content),
        WidgetType::Markdown        => markdown_controls(&hierarchy, widget_id, theme, &type_system, custom_styles, preview_content),
        WidgetType::MouseArea       => mousearea_controls(&hierarchy, widget_id, theme, &type_system, custom_styles, preview_content),
        WidgetType::QRCode          => qrcode_controls(&hierarchy, widget_id, theme, &type_system, custom_styles, preview_content),
        WidgetType::Stack           => stack_controls(&hierarchy, widget_id, theme, &type_system, custom_styles, preview_content),
        WidgetType::Themer          => themer_controls(&hierarchy, widget_id, theme, &type_system, custom_styles, preview_content),
        WidgetType::Grid            => grid_controls(&hierarchy, widget_id, theme, &type_system, custom_styles, preview_content),
        WidgetType::Pin             => pin_controls(&hierarchy, widget_id, theme, &type_system, custom_styles, preview_content),
        WidgetType::Table           => table_controls(&hierarchy, widget_id, theme, &type_system, custom_styles, preview_content),
        WidgetType::ViewReference   => view_reference_controls(&hierarchy, widget_id, theme, &type_system, views, custom_styles, preview_content),
        WidgetType::Icon            => icon_controls(&hierarchy, widget_id, theme, &type_system, custom_styles, preview_content),
    };

    controls_view.into()
}
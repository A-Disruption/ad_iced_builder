use iced::{Element, Settings, Theme, Length, Task,
    widget::{ button, column, container, row, rule, scrollable, space, text },
};
use crate::data_structures::types::types::{WidgetId, WidgetType};
use crate::data_structures::widget_hierarchy::WidgetHierarchy;
use crate::data_structures::properties::messages::PropertyChange;
use crate::enum_builder::TypeSystem;
use widgets::generic_overlay::overlay_button;
use crate::widget_tree::debug_print_widget;
use crate::controls::batch_edit::batch_editor_controls;

// Application messages
#[derive(Debug, Clone)]
pub enum Message {
    //SelectWidgetType(WidgetType),
    AddChild(WidgetId, WidgetType),

    // Wrapping operations
    WrapSelectedInContainer(WidgetType),  // Wraps selection in Row/Column/MouseArea/Tooltip

    // Batch editing operations  
    BatchPropertyChanged(PropertyChange), // Applies property to all selected widgets
}

pub fn update<'a>(hierarchy: &'a mut WidgetHierarchy, type_system: &'a mut TypeSystem, message: Message) -> Task<Message>{
    match message {
        //Message::SelectWidgetType(widget_type) => {}

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
            hierarchy.apply_property_to_all_selected(change, &type_system);
        }
    }
    Task::none()
}

pub fn view<'a>(
    hierarchy: &'a WidgetHierarchy,
    parent_id: &'a WidgetId,
//    available_types: &[WidgetType],
) -> Element<'a, Message> {
    let available_types = get_available_types(hierarchy, parent_id);
    let selected_count = hierarchy.selected_ids().len();

    let multi_selection_menu = if selected_count > 1 {
        multi_selection_controls(hierarchy, selected_count)
    } else {
        column![].height(0).width(0).into()
    };

    // Helper to create a button if the type is available
    let widget_button = |widget_type: WidgetType, label: &'a str| -> Element<'a, Message> {
        if available_types.contains(&widget_type) {
            button(text(label).center())
                .on_press(Message::AddChild(*parent_id, widget_type))
                .style(button::secondary)
                .width(Length::FillPortion(1))
                .into()
        } else {
            button(text(label).center())
                .style(button::secondary)
                .width(Length::FillPortion(1))
                .into()
        }
    };

    scrollable(
        column![
/*             if available_types.contains(&WidgetType::Container) 
                || available_types.contains(&WidgetType::Scrollable) { */
                column![
                    text("Containers").size(18),
                    rule::horizontal(2),
                    row![
                        widget_button(WidgetType::Container, "Container"),
                        widget_button(WidgetType::Scrollable, "Scrollable"),
                        widget_button(WidgetType::ViewReference, "View"),
                    ]
                    .spacing(10)
                    .padding(5),
                ],
//            } else {
//                column![].height(0).width(0)
//            },
            
//            if available_types.contains(&WidgetType::Row) 
//                || available_types.contains(&WidgetType::Column) {
                column![
                    text("Layout").size(18),
                    rule::horizontal(2),
                    row![
                        widget_button(WidgetType::Row, "Row"),
                        widget_button(WidgetType::Column, "Column")
                    ]
                    .spacing(10)
                    .padding(5),
                ],
//            } else {
//                column![].height(0).width(0)
//            },

//            if !available_types.is_empty(){
                column![
                    text("Widgets").size(18),
                    rule::horizontal(2),
                    
                    row![
                        widget_button(WidgetType::Text, "Text"),
                        widget_button(WidgetType::TextInput, "Text Input"),
                        widget_button(WidgetType::Button, "Button")
                    ]
                    .spacing(10)
                    .padding(5),
                    
                    row![
                        widget_button(WidgetType::Checkbox, "Checkbox"),
                        widget_button(WidgetType::Radio, "Radio"),
                        widget_button(WidgetType::Toggler, "Toggler")
                    ]
                    .spacing(10)
                    .padding(5),
                    
                    row![
                        widget_button(WidgetType::Slider, "Slider"),
                        widget_button(WidgetType::VerticalSlider, "Vert. Slider"),
                        widget_button(WidgetType::ProgressBar, "Progress")
                    ]
                    .spacing(10)
                    .padding(5),
                    
                    row![
                        widget_button(WidgetType::PickList, "Pick List"),
                        widget_button(WidgetType::Space, "Space"),
                        widget_button(WidgetType::Rule, "Rule")
                    ]
                    .spacing(10)
                    .padding(5),
                    
                    row![
                        widget_button(WidgetType::Image, "Image"),
                        widget_button(WidgetType::Svg, "SVG"),
                        widget_button(WidgetType::Tooltip, "Tooltip")
                    ]
                    .spacing(10)
                    .padding(5),

                    row![
                        widget_button(WidgetType::ComboBox, "ComboBox"),
                        widget_button(WidgetType::Markdown, "Markdown"),
                        widget_button(WidgetType::MouseArea, "MouseArea")
                    ]
                    .spacing(10)
                    .padding(5),

                    row![
                        widget_button(WidgetType::Pin, "Pin"),
                        widget_button(WidgetType::QRCode, "QRCode"),
                    ]
                    .spacing(10)
                    .padding(5),
                ],
//            } else {
//                column![].height(0).width(0)
//            }, 


            
            if available_types.is_empty() && selected_count > 1 {
                multi_selection_menu
            } 
/*             else if available_types.is_empty() {
                column![
                    text("No widgets can be added to this parent").size(14)
                        .color(iced::Color::from_rgb(0.6, 0.6, 0.6)),
                ]
                .padding(10)
                .into()
            }  */
            else {
                column![].height(0).width(0).into()
            }
        ]
        .spacing(10)
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}


/// Returns list of widget's that can be added to the currently selected widget
pub fn get_available_types<'a>(hierarchy: &'a WidgetHierarchy, parent_id: &'a WidgetId) -> Vec<WidgetType> {
   let parent = hierarchy.get_widget_by_id(*parent_id);
    if parent.is_none() {
        return Vec::new();
    }
    let parent = parent.unwrap();
    
    let available_types = if *parent_id == hierarchy.root().id {
        if hierarchy.root().children.is_empty() {
            vec![WidgetType::Column, WidgetType::Row]
        } else {
            vec![]
        }
    } else if parent.widget_type == WidgetType::Scrollable {
        if parent.children.is_empty() {
            vec![WidgetType::Container, WidgetType::Column, WidgetType::Row, WidgetType::ViewReference]
        } else {
            vec![]
        }
    } else if parent.widget_type == WidgetType::Container {
        if parent.children.is_empty() {
            vec![
                WidgetType::Container,
                WidgetType::Scrollable,
                WidgetType::Row,
                WidgetType::Column,
                WidgetType::Button,
                WidgetType::Text,
                WidgetType::TextInput,
                WidgetType::Checkbox,
                WidgetType::Radio,
                WidgetType::Slider,
                WidgetType::VerticalSlider,
                WidgetType::ProgressBar,
                WidgetType::Toggler,
                WidgetType::PickList,
                WidgetType::Space,
                WidgetType::Rule,
                WidgetType::Image,
                WidgetType::Svg,
                WidgetType::Tooltip,
                WidgetType::ComboBox,
                WidgetType::Markdown,
                WidgetType::MouseArea,
                WidgetType::Pin,
                WidgetType::QRCode,
                WidgetType::ViewReference
            ]
        } else {
            vec![]
        }
    } else if parent.widget_type == WidgetType::MouseArea {
        if parent.children.len() < 1 {
            vec![
                WidgetType::Container,
                WidgetType::Scrollable,
                WidgetType::Row,
                WidgetType::Column,
                WidgetType::Button,
                WidgetType::Text,
                WidgetType::TextInput,
                WidgetType::Checkbox,
                WidgetType::Radio,
                WidgetType::Slider,
                WidgetType::VerticalSlider,
                WidgetType::ProgressBar,
                WidgetType::Toggler,
                WidgetType::PickList,
                WidgetType::Space,
                WidgetType::Rule,
                WidgetType::Image,
                WidgetType::Svg,
                WidgetType::Tooltip,
                WidgetType::ComboBox,
                WidgetType::Markdown,
                WidgetType::Pin,
                WidgetType::QRCode,
            ]
        } else {
            vec![]
        }
    } else if parent.widget_type == WidgetType::Tooltip {
        if parent.children.len() < 2 {
            vec![
                WidgetType::Container,
                WidgetType::Scrollable,
                WidgetType::Row,
                WidgetType::Column,
                WidgetType::Button,
                WidgetType::Text,
                WidgetType::TextInput,
                WidgetType::Checkbox,
                WidgetType::Radio,
                WidgetType::Slider,
                WidgetType::VerticalSlider,
                WidgetType::ProgressBar,
                WidgetType::Toggler,
                WidgetType::PickList,
                WidgetType::Image,
                WidgetType::Svg,
                WidgetType::ComboBox,
                WidgetType::Markdown,
                WidgetType::MouseArea,
                WidgetType::Pin,
                WidgetType::QRCode,
            ]
        } else {
            vec![]
        }
    } else if parent.widget_type == WidgetType::Row || parent.widget_type == WidgetType::Column {
        vec![
            WidgetType::Container,
            WidgetType::Scrollable,
            WidgetType::Row,
            WidgetType::Column,
            WidgetType::Button,
            WidgetType::Text,
            WidgetType::TextInput,
            WidgetType::Checkbox,
            WidgetType::Radio,
            WidgetType::Slider,
            WidgetType::VerticalSlider,
            WidgetType::ProgressBar,
            WidgetType::Toggler,
            WidgetType::PickList,
            WidgetType::Space,
            WidgetType::Rule,
            WidgetType::Image,
            WidgetType::Svg,
            WidgetType::Tooltip,
            WidgetType::ComboBox,
            WidgetType::Markdown,
            WidgetType::MouseArea,
            WidgetType::Pin,
            WidgetType::QRCode,
            WidgetType::ViewReference
        ]
    } else {
        vec![]
    };

    available_types
}


/// Builds controls that appear when multiple widgets are selected
fn multi_selection_controls<'a>(hierarchy: &'a WidgetHierarchy, selected_count: usize) -> Element<'a, Message> {
    
    // Validate if wrapping is possible
    let can_wrap = hierarchy.validate_wrapping().is_ok();
    
    column![
        // Header showing selection count
        text(format!("{} widgets selected", selected_count))
            .size(16),
        
        rule::horizontal(2),
        
        // Wrapping controls
        text("Wrap in:").size(14),
        
        column![
            // Row wrapping button
            button(text("Row"))
                .on_press_maybe(
                    if can_wrap {
                        Some(Message::WrapSelectedInContainer(WidgetType::Row))
                    } else {
                        None
                    }
                )
                .width(Length::Fill),
            
            // Column wrapping button
            button(text("Column"))
                .on_press_maybe(
                    if can_wrap {
                        Some(Message::WrapSelectedInContainer(WidgetType::Column))
                    } else {
                        None
                    }
                )
                .width(Length::Fill),
            
            // MouseArea wrapping button
            button(text("MouseArea"))
                .on_press_maybe(
                    if can_wrap {
                        Some(Message::WrapSelectedInContainer(WidgetType::MouseArea))
                    } else {
                        None
                    }
                )
                .width(Length::Fill),
            
            // Container wrapping (for single widget only)
            button(text("Container"))
                .on_press_maybe(
                    if can_wrap && selected_count == 1 {
                        Some(Message::WrapSelectedInContainer(WidgetType::Container))
                    } else {
                        None
                    }
                )
                .width(Length::Fill),
            
            // Tooltip wrapping (requires exactly 2 widgets)
            button(text("Tooltip (2 widgets only)"))
                .on_press_maybe(
                    if can_wrap && selected_count == 2 {
                        Some(Message::WrapSelectedInContainer(WidgetType::Tooltip))
                    } else {
                        None
                    }
                )
                .width(Length::Fill),
        ]
        .spacing(5),
        
        rule::horizontal(2),
        
        // Batch edit button
        text("Batch Edit:").size(14),
        
        // Create overlay for batch editing
        overlay_button(
            "Edit All Properties",
            format!("Editing {} widgets", selected_count),
            batch_editor_controls(&hierarchy)
        )
        .overlay_width(500.0)
        .overlay_height(750.0)
        .style(button::primary),
        
    ]
    .spacing(10)
    .into()
}
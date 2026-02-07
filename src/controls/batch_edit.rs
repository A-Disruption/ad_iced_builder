use iced::{ Alignment, Color, Element, Length, Padding, Theme };
use iced::widget::{ container, button, column, pick_list, radio, row, rule, scrollable, slider, space, text, text_input};
use crate::data_structures::widget_hierarchy::WidgetHierarchy;
use crate::data_structures::types::type_implementations::*;
use crate::data_structures::properties::properties::*;
use crate::data_structures::properties::messages::*;
use crate::styles::container::*;
use crate::views::add_widgets::Message;
use super::control_styling::*;

pub fn batch_editor_controls<'a>(
    hierarchy: &'a WidgetHierarchy,
) -> Element<'a, Message> {
    let selected = hierarchy.get_selected_widgets();
    let selected_count = selected.len();
    let common = hierarchy.common_properties.as_ref().expect("Should have Some() Common Properties");
    

    let mut content = column![
        text(format!("Batch Editing {} Widgets", selected_count))
            .size(20),
        
        text("Only common properties are shown. Changes apply to all selected widgets.")
            .size(12)
            .color(Color::from_rgb(0.6, 0.6, 0.6)),
        
        rule::horizontal(2),
    ]
    .spacing(10);
    
    // Width and Height (all widgets have these)
    if common.has_width_height {
        content = content.push(
            batch_size_controls(common, hierarchy)
        );
    }
    
    // Padding (only for widgets that support it)
    if common.has_padding {
        content = content.push(
            batch_padding_controls(common)
        );
    }
    
    // Spacing (only for Row/Column)
    if common.has_spacing {
        content = content.push(
            batch_spacing_controls(common)
        );
    }
    
    // Text properties (for Text, Button, TextInput)
    if common.has_text_properties {
        content = content.push(
            batch_text_size_controls(common)
        );
    }
    
    // List the widgets being edited (for clarity)
    content = content.push(rule::horizontal(2));
    content = content.push(text("Editing:").size(SECTION_SIZE));
    
    for widget in selected.iter().take(10) {
        // Check if widget has a custom name
        let display_name = if !widget.properties.widget_name.is_empty() {
            // Custom name - show it prominently
            format!("  • {} ({})", widget.properties.widget_name, widget.widget_type)
        } else {
            // Default name - show the type and id
            format!("  • {} ({})", widget.widget_type, widget.name)
        };
        
        content = content.push(
            text(display_name)
                .size(LABEL_SIZE)
        );
    }
    
    if selected.len() > 10 {
        content = content.push(
            text(format!("  ... and {} more", selected.len() - 10))
                .size(LABEL_SIZE)
                .color(Color::from_rgb(0.6, 0.6, 0.6))
        );
    }
    
    scrollable(content.padding(15)).into()
}

/// Batch width/height controls - mirrors size_controls_scrollable_aware
fn batch_size_controls<'a>(
    common: &'a CommonProperties,
    hierarchy: &'a WidgetHierarchy,
) -> Element<'a, Message> {

    column![
        text("Size").size(SECTION_SIZE),
        
        // Show current values if uniform
        if let Some(width) = common.uniform_width {
            text(format!("Current width: {}", length_to_string(width)))
                .size(LABEL_SIZE)
                .color(Color::from_rgb(0.5, 0.5, 0.5))
        } else {
            text("Width: (mixed values)")
                .size(LABEL_SIZE)
                .color(Color::from_rgb(0.8, 0.6, 0.3))
        },
        
        if let Some(height) = common.uniform_height {
            text(format!("Current height: {}", length_to_string(height)))
                .size(LABEL_SIZE)
                .color(Color::from_rgb(0.5, 0.5, 0.5))
        } else {
            text("Height: (mixed values)")
                .size(LABEL_SIZE)
                .color(Color::from_rgb(0.8, 0.6, 0.3))
        },
        
        // Width picker with draft support
        batch_length_picker(
            "Width", 
            common.uniform_width, 
            &common.draft_fixed_width,
            &common.draft_fill_portion_width,
            false, 
            hierarchy
        ),
        
        // Height picker with draft support
        batch_length_picker(
            "Height", 
            common.uniform_height, 
            &common.draft_fixed_height,
            &common.draft_fill_portion_height,
            true, 
            hierarchy
        ),
    ]
    .spacing(SECTION_SPACING)
    .into()
}

/// Batch length picker - mirrors length_picker_scrollable_aware but for batch mode
fn batch_length_picker<'a>(
    label: &'a str,
    current: Option<Length>,
    draft_fixed: &'a str,
    draft_fill_portion: &'a str,
    is_height: bool,
    hierarchy: &'a WidgetHierarchy,
) -> Element<'a, Message> {
    const DEFAULT_PX: f32 = 120.0;
    const DEFAULT_PORTION: u16 = 1;
    
    // Static slices for pick_list (must outlive the function)
    const ALL_CHOICES: &[LengthChoice] = &[
        LengthChoice::Fill,
        LengthChoice::FillPortion,
        LengthChoice::Shrink,
        LengthChoice::Fixed,
    ];
    
    const CONSTRAINED_CHOICES: &[LengthChoice] = &[
        LengthChoice::Shrink,
        LengthChoice::Fixed,
    ];
    
    // Check if ANY selected widget is under a scrollable
    // For batch editing, we need to check all selected widgets
    let selected_ids = hierarchy.get_selected_widgets();
    let any_scrollable_conflicts = selected_ids.iter().any(|widget| {
        if let Some((_, scroll_dir)) = hierarchy.get_scrollable_ancestor_info(widget.id) {
            match scroll_dir {
                iced::widget::scrollable::Direction::Vertical(_) => is_height,
                iced::widget::scrollable::Direction::Horizontal(_) => !is_height,
                iced::widget::scrollable::Direction::Both { .. } => true,
            }
        } else {
            false
        }
    });
    
    // Determine current choice
    let choice_now = current.map(|len| LengthChoice::from_length(len))
        .unwrap_or(LengthChoice::Fill);
    
    // Select the appropriate static slice based on constraints
    let available_choices = if any_scrollable_conflicts {
        CONSTRAINED_CHOICES
    } else {
        ALL_CHOICES
    };
    
    // Show warning if current choice is incompatible
    let warning = if any_scrollable_conflicts && 
                     (choice_now == LengthChoice::Fill || choice_now == LengthChoice::FillPortion) {
        Some(
            container(
                text("⚠ Some widgets are in scrollables and cannot use Fill")
                    .size(11)
                    .color(Color::from_rgb(0.9, 0.6, 0.2))
            )
            .padding(Padding::from([2, 5]))
            .style(error_box)
        )
    } else {
        None
    };
    
    let picker = column![
        text(label).size(LABEL_SIZE),
        pick_list(
            available_choices,
            Some(choice_now),
            move |choice| {
                // When changing choice, use default values
                let new_len = match choice {
                    LengthChoice::Fill => Length::Fill,
                    LengthChoice::FillPortion => Length::FillPortion(DEFAULT_PORTION),
                    LengthChoice::Shrink => Length::Shrink,
                    LengthChoice::Fixed => Length::Fixed(DEFAULT_PX),
                };
                
                if is_height {
                    Message::BatchPropertyChanged(PropertyChange::Height(new_len))
                } else {
                    Message::BatchPropertyChanged(PropertyChange::Width(new_len))
                }
            }
        )
        .width(250)
    ]
    .spacing(LABEL_SPACING);
    
    // Add extra input fields based on current choice
    let extra: Element<'a, Message> = match choice_now {
        LengthChoice::Fixed => {
            
            column![
                text("Pixels").size(LABEL_SIZE),
                text_input::<Message, Theme, iced::Renderer>("e.g. 120", &draft_fixed)
                    .on_input(move |text| {
                        // Update draft field
                        if is_height {
                            Message::BatchPropertyChanged(PropertyChange::DraftFixedHeight(text))
                        } else {
                            Message::BatchPropertyChanged(PropertyChange::DraftFixedWidth(text))
                        }
                    })
                    .width(250)
            ]
            .spacing(LABEL_SPACING)
            .into()
        }
        LengthChoice::FillPortion => {
            
            column![
                text("Portion").size(LABEL_SIZE),
                text_input::<Message, Theme, iced::Renderer>("e.g. 1", &draft_fill_portion)
                    .on_input(move |text| {
                        // Update draft field
                        if is_height {
                            Message::BatchPropertyChanged(PropertyChange::DraftFillPortionHeight(text))
                        } else {
                            Message::BatchPropertyChanged(PropertyChange::DraftFillPortionWidth(text))
                        }
                    })
                    .width(250)
            ]
            .spacing(LABEL_SPACING)
            .into()
        }
        _ => space::horizontal().into(),
    };
    
    let content = row![picker, extra].spacing(SECTION_SPACING);
    
    // Add warning if present
    if let Some(warn) = warning {
        column![content, warn].spacing(5).into()
    } else {
        content.into()
    }
}


/// Batch padding controls - mirrors padding_controls
fn batch_padding_controls<'a>(
    common: &'a CommonProperties,
) -> Element<'a, Message> {
    let current_padding = common.uniform_padding.unwrap_or(Padding::new(10.0));
    let current_padding_mode = common.uniform_padding_mode.unwrap_or(PaddingMode::Uniform);
    
    column![
        text("Padding").size(SECTION_SIZE),


        // Mode selection
        column![
            text("Padding Mode").size(LABEL_SIZE),
            column![
                radio(
                    "Uniform - All sides equal",
                    PaddingMode::Uniform,
                    Some(current_padding_mode),
                    move |mode| Message::BatchPropertyChanged(
                        PropertyChange::PaddingMode(mode)
                    )
                ),
                radio(
                    "Symmetric - Vertical/Horizontal pairs",
                    PaddingMode::Symmetric,
                    Some(current_padding_mode),
                    move |mode| Message::BatchPropertyChanged(
                        PropertyChange::PaddingMode(mode)
                    )
                ),
                radio(
                    "Individual - Each side separate",
                    PaddingMode::Individual,
                    Some(current_padding_mode),
                    move |mode| Message::BatchPropertyChanged(
                        PropertyChange::PaddingMode(mode)
                    )
                ),
            ]
            .spacing(LABEL_SPACING)
        ]
        .spacing(LABEL_SPACING),


        // Controls based on mode
        match current_padding_mode {
            PaddingMode::Uniform => {
                // Single slider controls all sides
                column![
                    text("All Sides").size(LABEL_SIZE),
                    row![
                        slider(0.0..=50.0, current_padding.top, |v| {
                            Message::BatchPropertyChanged(PropertyChange::PaddingUniform(v))
                        })
                        .step(1.0)
                        .width(250),
                        text(format!("{:.0}px", current_padding.top))
                            .size(LABEL_SIZE)
                            .width(50),
                    ]
                    .spacing(SECTION_SPACING)
                    .align_y(Alignment::Center),
                ]
                .spacing(LABEL_SPACING)
            }
            
            PaddingMode::Symmetric => {
                // Two sliders: vertical and horizontal
                column![
                    row![
                        column![
                            text("Vertical (Top/Bottom)").size(LABEL_SIZE),
                            row![
                                slider(0.0..=50.0, current_padding.top, |v| {
                                    Message::BatchPropertyChanged(PropertyChange::PaddingVertical(v))
                                })
                                .step(1.0)
                                .width(200),
                                text(format!("{:.0}px", current_padding.top))
                                    .size(LABEL_SIZE)
                                    .width(50),
                            ]
                            .spacing(SECTION_SPACING)
                            .align_y(Alignment::Center),
                        ]
                        .spacing(LABEL_SPACING)
                        .width(Length::Fill),
                    ],
                    row![
                        column![
                            text("Horizontal (Left/Right)").size(LABEL_SIZE),
                            row![
                                slider(0.0..=50.0, current_padding.left, |v| {
                                    Message::BatchPropertyChanged(PropertyChange::PaddingHorizontal(v))
                                })
                                .step(1.0)
                                .width(200),
                                text(format!("{:.0}px", current_padding.left))
                                    .size(LABEL_SIZE)
                                    .width(50),
                            ]
                            .spacing(SECTION_SPACING)
                            .align_y(Alignment::Center),
                        ]
                        .spacing(LABEL_SPACING)
                        .width(Length::Fill),
                    ],
                ]
                .spacing(SECTION_SPACING)
            }
            
            PaddingMode::Individual => {
                // Four separate sliders in a 2x2 grid
                column![
                    row![
                        column![
                            text("Top").size(LABEL_SIZE),
                                slider(0.0..=50.0, current_padding.top, |v| {
                                    Message::BatchPropertyChanged(PropertyChange::PaddingTop(v))
                                })
                            .step(1.0),
                            text(format!("{:.0}px", current_padding.top))
                                .size(LABEL_SIZE)
                                .center(),
                        ]
                        .spacing(LABEL_SPACING)
                        .width(Length::Fill),
                        
                        column![
                            text("Right").size(LABEL_SIZE),
                                slider(0.0..=50.0, current_padding.right, |v| {
                                    Message::BatchPropertyChanged(PropertyChange::PaddingRight(v))
                                })
                            .step(1.0),
                            text(format!("{:.0}px", current_padding.right))
                                .size(LABEL_SIZE)
                                .center(),
                        ]
                        .spacing(LABEL_SPACING)
                        .width(Length::Fill),
                    ]
                    .spacing(MAIN_SPACING),
                    
                    row![
                        column![
                            text("Bottom").size(LABEL_SIZE),
                                slider(0.0..=50.0, current_padding.bottom, |v| {
                                    Message::BatchPropertyChanged(PropertyChange::PaddingBottom(v))
                                })
                            .step(1.0),
                            text(format!("{:.0}px", current_padding.bottom))
                                .size(LABEL_SIZE)
                                .center(),
                        ]
                        .spacing(LABEL_SPACING)
                        .width(Length::Fill),
                        
                        column![
                            text("Left").size(LABEL_SIZE),
                                slider(0.0..=50.0, current_padding.left, |v| {
                                    Message::BatchPropertyChanged(PropertyChange::PaddingLeft(v))
                                })
                            .step(1.0),
                            text(format!("{:.0}px", current_padding.left))
                                .size(LABEL_SIZE)
                                .center(),
                        ]
                        .spacing(LABEL_SPACING)
                        .width(Length::Fill),
                    ]
                    .spacing(MAIN_SPACING),
                ]
                .spacing(SECTION_SPACING)
            }
        },
    ]
    .spacing(SECTION_SPACING)
    .into()
}

/// Batch spacing controls - mirrors spacing controls in row/column editors
fn batch_spacing_controls<'a>(
    common: &'a CommonProperties,
) -> Element<'a, Message> {
    let current_spacing = common.uniform_spacing.unwrap_or(0.0);
    
    column![
        text("Spacing").size(SECTION_SIZE),
        
        if common.uniform_spacing.is_some() {
            text(format!("Current: {:.0}px", current_spacing))
                .size(LABEL_SIZE)
                .color(Color::from_rgb(0.5, 0.5, 0.5))
        } else {
            text("(mixed values)")
                .size(LABEL_SIZE)
                .color(Color::from_rgb(0.8, 0.6, 0.3))
        },
        
        column![
            text("Element spacing:").size(LABEL_SIZE),
            row![
                slider(0.0..=50.0, current_spacing, |v| {
                    Message::BatchPropertyChanged(PropertyChange::Spacing(v))
                })
                .step(1.0)
                .width(250),
                text(format!("{:.0}px", current_spacing))
                    .size(LABEL_SIZE)
                    .width(50),
            ]
            .spacing(SECTION_SPACING)
            .align_y(Alignment::Center),
            
            // Quick preset buttons
            row![
                button(text("0px")).on_press(
                    Message::BatchPropertyChanged(PropertyChange::Spacing(0.0))
                ).padding(5),
                button(text("5px")).on_press(
                    Message::BatchPropertyChanged(PropertyChange::Spacing(5.0))
                ).padding(5),
                button(text("10px")).on_press(
                    Message::BatchPropertyChanged(PropertyChange::Spacing(10.0))
                ).padding(5),
                button(text("20px")).on_press(
                    Message::BatchPropertyChanged(PropertyChange::Spacing(20.0))
                ).padding(5),
            ]
            .spacing(5),
        ]
        .spacing(LABEL_SPACING),
    ]
    .spacing(SECTION_SPACING)
    .into()
}

/// Batch text size controls
fn batch_text_size_controls<'a>(
    common: &'a CommonProperties,
) -> Element<'a, Message> {
    let current_size = common.uniform_text_size.unwrap_or(16.0);
    
    column![
        text("Text Properties").size(SECTION_SIZE),
        
        if common.uniform_text_size.is_some() {
            text(format!("Current size: {:.0}px", current_size))
                .size(LABEL_SIZE)
                .color(Color::from_rgb(0.5, 0.5, 0.5))
        } else {
            text("(mixed values)")
                .size(LABEL_SIZE)
                .color(Color::from_rgb(0.8, 0.6, 0.3))
        },
        
        column![
            text("Font size:").size(LABEL_SIZE),
            row![
                slider(8.0..=72.0, current_size, |v| {
                    Message::BatchPropertyChanged(PropertyChange::TextSize(v))
                })
                .step(1.0)
                .width(250),
                text(format!("{:.0}px", current_size))
                    .size(LABEL_SIZE)
                    .width(50),
            ]
            .spacing(SECTION_SPACING)
            .align_y(Alignment::Center),
            
            // Quick preset buttons
            row![
                button(text("12px")).on_press(
                    Message::BatchPropertyChanged(PropertyChange::TextSize(12.0))
                ).padding(5),
                button(text("16px")).on_press(
                    Message::BatchPropertyChanged(PropertyChange::TextSize(16.0))
                ).padding(5),
                button(text("20px")).on_press(
                    Message::BatchPropertyChanged(PropertyChange::TextSize(20.0))
                ).padding(5),
                button(text("24px")).on_press(
                    Message::BatchPropertyChanged(PropertyChange::TextSize(24.0))
                ).padding(5),
            ]
            .spacing(5),
        ]
        .spacing(LABEL_SPACING),
    ]
    .spacing(SECTION_SPACING)
    .into()
}
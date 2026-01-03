pub mod batch_edit;
pub mod single_edit;
pub mod control_styling;

// controls.rs
use iced::{ Alignment, Color, Element, Length, Padding, Theme, mouse::Interaction, padding };
use iced::widget::{ Space, button, checkbox, column, combo_box, container, pick_list, radio, row, rule, scrollable, slider, space, text, text_editor, text_input, toggler, tooltip};
use uuid::Uuid;
use std::collections::BTreeMap;
//use crate::code_generator::{CodeGenerator, build_code_view_with_height};
use crate::code_generator::CodeGenerator;
use crate::code_generator::build_code_view_with_height;
use crate::enum_builder::TypeSystem;
use crate::data_structures::widget_hierarchy::WidgetHierarchy;
use crate::data_structures::types::types::*;
use crate::data_structures::types::type_implementations::*;
use crate::data_structures::properties::properties::*;
use crate::data_structures::properties::messages::*;
use crate::styles::container::*;
use crate::icon;

use crate::views::widget_tree::Message;
use crate::views::theme_and_stylefn_builder::{CustomThemes, ThemePaneEnum};
use control_styling::*;



pub fn container_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    theme: &Theme,
    type_system: &'a TypeSystem,
    custom_styles: &'a CustomThemes,
) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    let content = column![
        // Title
        text("Container Properties").size(TITLE_SIZE),

        // Widget Name
        row![
            widget_name(widget_id, &props.widget_name),
            custom_style_picker(custom_styles, widget_id, props, ThemePaneEnum::Container),
        ].spacing(MAIN_SPACING),        

        column![
            text("Sizing Mode").size(SECTION_SIZE),
            pick_list(
                vec![
                    ContainerSizingMode::Manual,
                    ContainerSizingMode::CenterX,
                    ContainerSizingMode::CenterY,
                    ContainerSizingMode::Center,
                ],
                Some(props.container_sizing_mode),
                move |mode| Message::PropertyChanged(
                    widget_id,
                    PropertyChange::ContainerSizingMode(mode)
                )
            ),
            text(match props.container_sizing_mode {
                ContainerSizingMode::Manual => "Set width and height separately",
                ContainerSizingMode::CenterX => "Set width and center content horizontally",
                ContainerSizingMode::CenterY => "Set height and center content vertically",
                ContainerSizingMode::Center => "Set size and center content in both directions",
            })
            .size(LABEL_SIZE - 1.0)
            .color(Color::from_rgb(0.5, 0.5, 0.5)),
        ]
        .spacing(LABEL_SPACING),

        // Size Controls - conditional based on mode
        match props.container_sizing_mode {
            ContainerSizingMode::Manual => {
                // Regular width/height controls
                size_controls_scrollable_aware(
                    props.width,
                    move |l| Message::PropertyChanged(widget_id, PropertyChange::Width(l)),
                    props.height,
                    move |l| Message::PropertyChanged(widget_id, PropertyChange::Height(l)),
                    h,
                    widget_id,
                )
            }
            ContainerSizingMode::CenterX => {
                column![
                    length_picker_scrollable_aware(
                        "Width (centers content horizontally)",
                        props.container_center_length,
                        move |l| Message::PropertyChanged(widget_id, PropertyChange::ContainerCenterLength(l)),
                        h,
                        widget_id,
                        false
                    ),
                    text("Height will be determined by content")
                        .size(LABEL_SIZE - 1.0)
                        .color(Color::from_rgb(0.5, 0.5, 0.5)),
                ]
                .spacing(LABEL_SPACING)
                .into()
            }
            ContainerSizingMode::CenterY => {
                column![
                    length_picker_scrollable_aware(
                        "Height (centers content vertically)",
                        props.container_center_length,
                        move |l| Message::PropertyChanged(widget_id, PropertyChange::ContainerCenterLength(l)),
                        h,
                        widget_id,
                        true
                    ),
                    text("Width will be determined by content")
                        .size(LABEL_SIZE - 1.0)
                        .color(Color::from_rgb(0.5, 0.5, 0.5)),
                ]
                .spacing(LABEL_SPACING)
                .into()
            }
            ContainerSizingMode::Center => {
                column![
                    length_picker_scrollable_aware(
                        "Size (centers content in both directions)",
                        props.container_center_length,
                        move |l| Message::PropertyChanged(widget_id, PropertyChange::ContainerCenterLength(l)),
                        h,
                        widget_id,
                        false
                    ),
                ]
                .spacing(LABEL_SPACING)
                .into()
            }
        },

        // Only show alignment controls in Manual mode
        if matches!(props.container_sizing_mode, ContainerSizingMode::Manual) {
            row![
                column![
                    text("Horizontal Align").size(LABEL_SIZE),
                    pick_list(
                        vec![ContainerAlignX::Left, ContainerAlignX::Center, ContainerAlignX::Right],
                        Some(props.align_x),
                        move |v| Message::PropertyChanged(widget_id, PropertyChange::AlignX(v)),
                    ).width(160),
                ]
                .spacing(LABEL_SPACING)
                .width(Length::Fill),
                
                column![
                    text("Vertical Align").size(LABEL_SIZE),
                    pick_list(
                        vec![ContainerAlignY::Top, ContainerAlignY::Center, ContainerAlignY::Bottom],
                        Some(props.align_y),
                        move |v| Message::PropertyChanged(widget_id, PropertyChange::AlignY(v)),
                    ).width(160),
                ]
                .spacing(LABEL_SPACING)
                .width(Length::Fill),
            ]
            .spacing(MAIN_SPACING)
        } else {
            row![]
        },

        // Padding Controls
        padding_controls(
            props.padding,
            widget_id,
            props.padding_mode,
            theme,
        ),

        // Set a Widget Id
        widget_id_control(widget_id, props.widget_id.clone(), theme),

        // Max Width control
        max_width_control(widget_id, props.max_width),

        // Max Height control
        max_height_control(widget_id, props.max_height),
        
        //Clip control
        clip_control(widget_id, props.clip, theme),

    ]
    .spacing(MAIN_SPACING)
    .into();

    scrollable( 
        container(
            add_code_preview(content, h, widget_id, theme, type_system, custom_styles)
        ).padding(padding::right(10.0))
    ).into()
}

pub fn row_controls<'a>(h: &'a WidgetHierarchy, widget_id: WidgetId, theme: &Theme, type_system: &'a TypeSystem, custom_styles: &CustomThemes,) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    let content = column![
        text("Row Properties").size(TITLE_SIZE),

        widget_name(widget_id, &props.widget_name),

        column![
            text("Layout Mode").size(SECTION_SIZE),
            row![
                toggler(
                    props.is_wrapping_row,
                )
                .on_toggle(move |v| Message::PropertyChanged(widget_id, PropertyChange::IsWrappingRow(v))),
                text("Enable Wrapping"),
                information(theme, "Items wrap to next line when row width is exceeded.")
            ]
            .spacing(LABEL_SPACING)
            .align_y(Alignment::End),
        ]
        .spacing(LABEL_SPACING),

        column![
            text("Spacing between items").size(LABEL_SIZE),
            row![
                slider(0.0..=50.0, props.spacing, move |v| {
                    Message::PropertyChanged(widget_id, PropertyChange::Spacing(v))
                })
                .step(1.0)
                .width(200),
                text(format!("{:.0}px", props.spacing)).size(LABEL_SIZE).width(50),
            ]
            .spacing(SECTION_SPACING)
            .align_y(Alignment::Center),
        ]
        .spacing(LABEL_SPACING),

        // NEW: Wrapping-specific controls (only show when wrapping enabled)
        if props.is_wrapping_row {
            column![
                column![
                    row![
                        text("Vertical Spacing").size(SECTION_SIZE),
                        information(theme, "Spacing between wrapped rows")
                    ]
                    .spacing(LABEL_SPACING)
                    .align_y(Alignment::End),
                    
                    row![
                        checkbox(
                            props.match_horizontal_spacing,
                        )
                        .label("match horizontal spacing")
                        .on_toggle(move |use_same| {
                            Message::PropertyChanged(
                                widget_id,
                                PropertyChange::WrappingSpacingMatchToggle(use_same)
                            )
                        }),
                        if props.match_horizontal_spacing {
                            row![
                                slider(0.0..=50.0, props.wrapping_vertical_spacing, move |v| {
                                    Message::PropertyChanged(
                                        widget_id,
                                        PropertyChange::WrappingVerticalSpacing(v)
                                    )
                                })
                                .step(1.0)
                                .width(180),
                                text(format!("{:.0}px", props.wrapping_vertical_spacing)).size(LABEL_SIZE).width(50),
                            ]
                            .spacing(SECTION_SPACING)
                            .align_y(Alignment::Center)
                        } else {
                            row![].into()
                        }
                    ]
                    .spacing(SECTION_SPACING)
                    .align_y(Alignment::Center),
                ]
                .spacing(LABEL_SPACING),
                
                column![
                    row![
                        text("Horizontal Alignment").size(LABEL_SIZE),
                        information(theme, "Aligns wrapped lines within the row"),
                    ]
                    .spacing(LABEL_SPACING)
                    .align_y(Alignment::End),
                    
                    pick_list(
                        vec![
                            ContainerAlignX::Left,
                            ContainerAlignX::Center,
                            ContainerAlignX::Right,
                        ],
                        Some(props.wrapping_align_x),
                        move |align| Message::PropertyChanged(
                            widget_id,
                            PropertyChange::WrappingAlignX(align)
                        ),
                    ).width(160),
                ]
                .spacing(LABEL_SPACING),
            ]
            .spacing(SECTION_SPACING)
        } else {
            column![].into()
        },

        // Vertical alignment (only for non-wrapping rows)
        if !props.is_wrapping_row {
            column![
                text("Vertical Alignment").size(LABEL_SIZE),
                row![
                    pick_list(
                        vec![AlignmentXOption::Start, AlignmentXOption::Center, AlignmentXOption::End],
                        Some(AlignmentXOption::from_alignment(props.align_items)),
                        move |sel| Message::PropertyChanged(
                            widget_id,
                            PropertyChange::AlignItems(sel.to_alignment())
                        ),
                    ),
                    information(theme, "Aligns children vertically within the row"),
                ]
                .spacing(LABEL_SPACING)
                .align_y(Alignment::End),
            ]
            .spacing(LABEL_SPACING)
        } else {
            column![].into()
        },

        size_controls_scrollable_aware(
            props.width,
            move |l| Message::PropertyChanged(widget_id, PropertyChange::Width(l)),
            props.height,
            move |l| Message::PropertyChanged(widget_id, PropertyChange::Height(l)),
            h,
            widget_id,
        ),

        padding_controls(
            props.padding,
            widget_id,
            props.padding_mode,
            theme,
        ),

        clip_control(widget_id, props.clip, theme),
    ]
    .spacing(MAIN_SPACING)
    .into();

    scrollable( 
        container(
            add_code_preview(content, h, widget_id, theme, type_system, custom_styles)
        ).padding(padding::right(10.0))
    ).into()
}

pub fn column_controls<'a>(h: &'a WidgetHierarchy, widget_id: WidgetId, theme: &Theme, type_system: &'a TypeSystem, custom_styles: &CustomThemes,) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    let content = column![
        text("Column Properties").size(TITLE_SIZE),

        widget_name(widget_id, &props.widget_name),

        column![
            text("Spacing between items").size(LABEL_SIZE),
            row![
                slider(0.0..=50.0, props.spacing, move |v| {
                    Message::PropertyChanged(widget_id, PropertyChange::Spacing(v))
                })
                .step(1.0)
                .width(200),
                text(format!("{:.0}px", props.spacing)).size(LABEL_SIZE).width(50),
            ]
            .spacing(SECTION_SPACING)
            .align_y(Alignment::Center),
        ]
        .spacing(LABEL_SPACING),

        column![
            text("Horizontal Alignment").size(LABEL_SIZE),
            pick_list(
                vec![AlignmentXOption::Start, AlignmentXOption::Center, AlignmentXOption::End],
                Some(AlignmentXOption::from_alignment(props.align_items)),
                move |sel| Message::PropertyChanged(widget_id, PropertyChange::AlignItems(sel.to_alignment())),
            ),
        ]
        .spacing(LABEL_SPACING),

        size_controls_scrollable_aware(
            props.width,
            move |l| Message::PropertyChanged(widget_id, PropertyChange::Width(l)),
            props.height,
            move |l| Message::PropertyChanged(widget_id, PropertyChange::Height(l)),
            h,
            widget_id,
        ),

        padding_controls(
            props.padding,
            widget_id,
            props.padding_mode,
            theme,
        ),

        // Max Width control
        max_width_control(widget_id, props.max_width),
        
        //Clip control
        clip_control(widget_id, props.clip, theme),
    ]
    .spacing(MAIN_SPACING)
    .into();

    scrollable( 
        container(
            add_code_preview(content, h, widget_id, theme, type_system, custom_styles)
        ).padding(padding::right(10.0))
    ).into()
}

pub fn button_controls<'a>(
    h: &'a WidgetHierarchy, 
    widget_id: WidgetId, 
    theme: &Theme, 
    type_system: &'a TypeSystem, 
    custom_styles: &'a CustomThemes,
) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;
    let palette = theme.extended_palette();

    // Determine which handler is currently selected
    let selected_handler = if props.button_on_press_enabled {
        0
    } else if props.button_on_press_with_enabled {
        1
    } else if props.button_on_press_maybe_enabled {
        2
    } else {
        3 // None selected
    };

    let content = column![
        text("Button Properties").size(TITLE_SIZE),

        widget_name(widget_id, &props.widget_name),

        column![
            text("Button Text").size(LABEL_SIZE),
            text_input("Text", &props.text_content)
                .on_input(move |v| Message::PropertyChanged(widget_id, PropertyChange::TextContent(v)))
                .width(250),
        ]
        .spacing(LABEL_SPACING),

        custom_style_picker(custom_styles, widget_id, props, ThemePaneEnum::Button),

        column![
            text("Event Handler").size(SECTION_SIZE),
            text("Choose which press handler pattern to use:")
                .size(LABEL_SIZE)
                .color(palette.background.strong.color),
            
            // None option
            row![
                radio(
                    "None (button disabled)",
                    3,
                    Some(selected_handler),
                    move |_| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::ButtonPressHandler(OnHandler::None)
                    )
                ),
                information(theme, "Button will not respond to clicks"),
            ]
            .spacing(LABEL_SPACING)
            .align_y(Alignment::Center),
            

            // on_press option
            row![
                radio(
                    "on_press",
                    0,
                    Some(selected_handler),
                    move |_| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::ButtonPressHandler(OnHandler::OnAction)
                    )
                ),
                information(theme, "Direct message dispatch - use when message is always the same"),
            ]
            .spacing(LABEL_SPACING)
            .align_y(Alignment::Center),
            
            // on_press_with option
            row![
                radio(
                    "on_press_with",
                    1,
                    Some(selected_handler),
                    move |_| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::ButtonPressHandler(OnHandler::OnActionWith)
                    )
                ),
                information(theme, "Closure returns message - use when message needs runtime data"),
            ]
            .spacing(LABEL_SPACING)
            .align_y(Alignment::Center),
            
            // on_press_maybe option
            row![
                radio(
                    "on_press_maybe",
                    2,
                    Some(selected_handler),
                    move |_| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::ButtonPressHandler(OnHandler::OnActionMaybe)
                    )
                ),
                information(theme, "Optional message - use when button should be conditionally enabled"),
            ]
            .spacing(LABEL_SPACING)
            .align_y(Alignment::Center),
        ]
        .spacing(SECTION_SPACING),

        size_controls_scrollable_aware(
            props.width,
            move |l| Message::PropertyChanged(widget_id, PropertyChange::Width(l)),
            props.height,
            move |l| Message::PropertyChanged(widget_id, PropertyChange::Height(l)),
            h,
            widget_id,
        ),

        padding_controls(
            props.padding,
            widget_id,
            props.padding_mode,
            theme,
        ),

        clip_control(widget_id, props.clip, theme),
    ]
    .spacing(MAIN_SPACING)
    .into();

    scrollable( 
        container(
            add_code_preview(content, h, widget_id, theme, type_system, custom_styles)
        ).padding(padding::right(10.0))
    ).into()
}


pub fn text_controls<'a>(h: &'a WidgetHierarchy, widget_id: WidgetId, theme: &Theme, type_system: &'a TypeSystem, custom_styles: &CustomThemes,) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    let content = column![
        text("Text Properties").size(TITLE_SIZE),

        widget_name(widget_id, &props.widget_name),

        column![
            text("Text Content").size(LABEL_SIZE),
            text_input("Content", &props.text_content)
                .on_input(move |v| Message::PropertyChanged(widget_id, PropertyChange::TextContent(v)))
                .width(300),
        ]
        .spacing(LABEL_SPACING),

        column![
            text("Font Size").size(LABEL_SIZE),
            row![
                slider(8.0..=72.0, props.text_size, move |v| {
                    Message::PropertyChanged(widget_id, PropertyChange::TextSize(v))
                })
                .step(1.0)
                .width(200),
                text(format!("{:.0}px", props.text_size)).size(LABEL_SIZE).width(50),
            ]
            .spacing(SECTION_SPACING)
            .align_y(Alignment::Center),
        ]
        .spacing(LABEL_SPACING),

        column![
            text("Font").size(LABEL_SIZE),
            pick_list(
                vec![FontType::Default, FontType::Monospace],
                Some(props.font),
                move |v| Message::PropertyChanged(widget_id, PropertyChange::Font(v)),
            )
            .width(200),
        ]
        .spacing(LABEL_SPACING),

        size_controls_scrollable_aware(
            props.width,
            move |l| Message::PropertyChanged(widget_id, PropertyChange::Width(l)),
            props.height,
            move |l| Message::PropertyChanged(widget_id, PropertyChange::Height(l)),
            h,
            widget_id,
        ),

        color_hex_input("Text Color", props.text_color, move |c| {
            Message::PropertyChanged(widget_id, PropertyChange::TextColor(c))
        }),

        column![
            text("Wrapping").size(LABEL_SIZE),
            pick_list(
                vec![TextWrapping::None, TextWrapping::Word, TextWrapping::Glyph, TextWrapping::WordOrGlyph],
                Some(TextWrapping::from(props.wrap)),
                move |w| Message::PropertyChanged(widget_id, PropertyChange::TextWrap(w))
            )
        ]
        .spacing(LABEL_SPACING),

        column![
            text("Shaping").size(LABEL_SIZE),
            pick_list(
                vec![TextShaping::Basic, TextShaping::Advanced, TextShaping::Auto],
                Some(TextShaping::from(props.shaping)),
                move |s| Message::PropertyChanged(widget_id, PropertyChange::TextShaping(s))
            )
        ]
        .spacing(LABEL_SPACING),

        column![
            text("Line Height").size(LABEL_SIZE),
            row![
                slider(0.8..=2.0, match props.line_height { text::LineHeight::Relative(v) => v, _ => 1.0 }, move |v| {
                    Message::PropertyChanged(widget_id, PropertyChange::TextLineHeight(text::LineHeight::Relative((v*100.0).round()/100.0)))
                })
                .step(0.05)
                .width(220),
                text(match props.line_height { text::LineHeight::Relative(v) => format!("{:.2}", v), _ => "1.00".into() }).size(LABEL_SIZE)
            ]
            .spacing(SECTION_SPACING)
            .align_y(Alignment::Center)
        ]
        .spacing(LABEL_SPACING),

        row![
            column![
                text("Align X").size(LABEL_SIZE),
                pick_list(
                    vec![AlignText::Default, AlignText::Left, AlignText::Center, AlignText::Right, AlignText::Justified],
                    Some(AlignText::from(props.text_align_x)),
                    move |a| Message::PropertyChanged(widget_id, PropertyChange::TextAlignX(a))
                )
            ]
            .spacing(LABEL_SPACING)
            .width(Length::Fill),
            
            column![
                text("Align Y").size(LABEL_SIZE),
                pick_list(
                    vec![AlignmentYOption::Top, AlignmentYOption::Center, AlignmentYOption::Bottom],
                    Some(AlignmentYOption::from(props.text_align_y)),
                    move |a| Message::PropertyChanged(widget_id, PropertyChange::TextAlignY(a))
                )
            ]
            .spacing(LABEL_SPACING)
            .width(Length::Fill),
        ]
        .spacing(SECTION_SPACING),
    ]
    .spacing(MAIN_SPACING)
    .into();

    scrollable( 
        container(
            add_code_preview(content, h, widget_id, theme, type_system, custom_styles)
        ).padding(padding::right(10.0))
    ).into()
}

pub fn text_input_controls<'a>(h: &'a WidgetHierarchy, widget_id: WidgetId, theme: &Theme, type_system: &'a TypeSystem, custom_styles: &CustomThemes,) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    let content = column![
        text("Text Input Properties").size(TITLE_SIZE),

        widget_name(widget_id, &props.widget_name),

        column![
            text("Placeholder Text").size(LABEL_SIZE),
            text_input("Placeholder", &props.text_input_placeholder)
                .on_input(move |v| Message::PropertyChanged(widget_id, PropertyChange::TextInputPlaceholder(v)))
                .width(250),
        ]
        .spacing(LABEL_SPACING),

        column![
            text("Font Size").size(LABEL_SIZE),
            row![
                slider(8.0..=32.0, props.text_input_size, move |v| {
                    Message::PropertyChanged(widget_id, PropertyChange::TextInputSize(v))
                })
                .step(1.0)
                .width(200),
                text(format!("{:.0}px", props.text_input_size)).size(LABEL_SIZE).width(50),
            ]
            .spacing(SECTION_SPACING)
            .align_y(Alignment::Center),
        ]
        .spacing(LABEL_SPACING),

        column![
            text("Internal Padding").size(LABEL_SIZE),
            row![
                slider(0.0..=30.0, props.text_input_padding, move |v| {
                    Message::PropertyChanged(widget_id, PropertyChange::TextInputPadding(v))
                })
                .step(1.0)
                .width(200),
                text(format!("{:.0}px", props.text_input_padding)).size(LABEL_SIZE).width(50),
            ]
            .spacing(SECTION_SPACING)
            .align_y(Alignment::Center),
        ]
        .spacing(LABEL_SPACING),

        column![
            text("Font").size(LABEL_SIZE),
            pick_list(
                vec![FontType::Default, FontType::Monospace],
                Some(props.text_input_font),
                move |v| Message::PropertyChanged(widget_id, PropertyChange::TextInputFont(v.into()))
            ),
        ]
        .spacing(LABEL_SPACING),

        column![
            text("Horizontal Alignment").size(LABEL_SIZE),
            pick_list(
                vec![
                    ContainerAlignX::Left,
                    ContainerAlignX::Center,
                    ContainerAlignX::Right,
                ],
                Some(props.text_input_alignment),
                move |v| Message::PropertyChanged(widget_id, PropertyChange::TextInputAlignment(v))
            ),
        ]
        .spacing(LABEL_SPACING),

        column![
            text("Security & Behavior").size(SECTION_SIZE),
            
            checkbox(props.is_secure)
                .label("Secure Input (Password)")
                .on_toggle(move |v| Message::PropertyChanged(widget_id, PropertyChange::IsSecure(v))),
        ]
        .spacing(SECTION_SPACING),

        column![
            text("Event Handlers").size(SECTION_SIZE),
            text("Enable additional event handlers:")
                .size(LABEL_SIZE)
                .color(Color::from_rgb(0.6, 0.6, 0.6)),
            
            checkbox(props.text_input_on_submit)
                .label("on_submit - Fires when Enter key is pressed")
                .on_toggle(move |v| Message::PropertyChanged(
                    widget_id, 
                    PropertyChange::TextInputOnSubmit(v)
                )),
            
            checkbox(props.text_input_on_paste)
                .label("on_paste - Fires when text is pasted")
                .on_toggle(move |v| Message::PropertyChanged(
                    widget_id, 
                    PropertyChange::TextInputOnPaste(v)
                )),
        ]
        .spacing(SECTION_SPACING),

        size_controls_scrollable_aware(
            props.width,
            move |l| Message::PropertyChanged(widget_id, PropertyChange::Width(l)),
            props.height,
            move |l| Message::PropertyChanged(widget_id, PropertyChange::Height(l)),
            h,
            widget_id,
        ),
    ]
    .spacing(MAIN_SPACING)
    .into();

    scrollable( 
        container(
            add_code_preview(content, h, widget_id, theme, type_system, custom_styles)
        ).padding(padding::right(10.0))
    ).into()
}

pub fn checkbox_controls<'a>(h: &'a WidgetHierarchy, widget_id: WidgetId, theme: &Theme, type_system: &'a TypeSystem, custom_styles: &CustomThemes,) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    let content = column![
        text("Checkbox Properties").size(TITLE_SIZE),

        widget_name(widget_id, &props.widget_name),

        column![
            text("Label Text").size(LABEL_SIZE),
            text_input("Label", &props.checkbox_label)
                .on_input(move |v| Message::PropertyChanged(widget_id, PropertyChange::CheckboxLabel(v)))
                .width(250),
        ]
        .spacing(LABEL_SPACING),

        column![
            text("Checkbox Size").size(LABEL_SIZE),
            row![
                slider(12.0..=40.0, props.checkbox_size, move |v| {
                    Message::PropertyChanged(widget_id, PropertyChange::CheckboxSize(v))
                })
                .step(1.0)
                .width(200),
                text(format!("{:.0}px", props.checkbox_size)).size(LABEL_SIZE).width(50),
            ]
            .spacing(SECTION_SPACING)
            .align_y(Alignment::Center),
        ]
        .spacing(LABEL_SPACING),

        column![
            text("Label Spacing").size(LABEL_SIZE),
            row![
                slider(0.0..=30.0, props.checkbox_spacing, move |v| {
                    Message::PropertyChanged(widget_id, PropertyChange::CheckboxSpacing(v))
                })
                .step(1.0)
                .width(200),
                text(format!("{:.0}px", props.checkbox_spacing)).size(LABEL_SIZE).width(50),
            ]
            .spacing(SECTION_SPACING)
            .align_y(Alignment::Center),
        ]
        .spacing(LABEL_SPACING),

        checkbox(props.checkbox_checked)
            .label("Default Checked State")
            .on_toggle(move |v| Message::PropertyChanged(widget_id, PropertyChange::CheckboxChecked(v))),

        size_controls_scrollable_aware(
            props.width,
            move |l| Message::PropertyChanged(widget_id, PropertyChange::Width(l)),
            props.height,
            move |l| Message::PropertyChanged(widget_id, PropertyChange::Height(l)),
            h,
            widget_id,
        ),
    ]
    .spacing(MAIN_SPACING)
    .into();

    scrollable( 
        container(
            add_code_preview(content, h, widget_id, theme, type_system, custom_styles)
        ).padding(padding::right(10.0))
    ).into()
}

pub fn toggler_controls<'a>(h: &'a WidgetHierarchy, widget_id: WidgetId, theme: &Theme, type_system: &'a TypeSystem, custom_styles: &CustomThemes,) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    let content = column![
        text("Toggler Properties").size(TITLE_SIZE),

        widget_name(widget_id, &props.widget_name),

        column![
            text("Label Text").size(LABEL_SIZE),
            text_input("Label", &props.toggler_label)
                .on_input(move |v| Message::PropertyChanged(widget_id, PropertyChange::TogglerLabel(v)))
                .width(250),
        ]
        .spacing(LABEL_SPACING),

        column![
            text("Toggler Size").size(LABEL_SIZE),
            row![
                slider(12.0..=40.0, props.toggler_size, move |v| {
                    Message::PropertyChanged(widget_id, PropertyChange::TogglerSize(v))
                })
                .step(1.0)
                .width(200),
                text(format!("{:.0}px", props.toggler_size)).size(LABEL_SIZE).width(50),
            ]
            .spacing(SECTION_SPACING)
            .align_y(Alignment::Center),
        ]
        .spacing(LABEL_SPACING),

        column![
            text("Label Spacing").size(LABEL_SIZE),
            row![
                slider(0.0..=30.0, props.toggler_spacing, move |v| {
                    Message::PropertyChanged(widget_id, PropertyChange::TogglerSpacing(v))
                })
                .step(1.0)
                .width(200),
                text(format!("{:.0}px", props.toggler_spacing)).size(LABEL_SIZE).width(50),
            ]
            .spacing(SECTION_SPACING)
            .align_y(Alignment::Center),
        ]
        .spacing(LABEL_SPACING),

        checkbox(props.toggler_active)
            .label("Default Active State")
            .on_toggle(move |v| Message::PropertyChanged(widget_id, PropertyChange::TogglerActive(v))),

        size_controls_scrollable_aware(
            props.width,
            move |l| Message::PropertyChanged(widget_id, PropertyChange::Width(l)),
            props.height,
            move |l| Message::PropertyChanged(widget_id, PropertyChange::Height(l)),
            h,
            widget_id,
        ),
    ]
    .spacing(MAIN_SPACING)
    .into();

    scrollable( 
        container(
            add_code_preview(content, h, widget_id, theme, type_system, custom_styles)
        ).padding(padding::right(10.0))
    ).into()
}

pub fn radio_controls<'a>(hierarchy: &'a WidgetHierarchy, widget_id: WidgetId, theme: &Theme, type_system: &'a TypeSystem, custom_styles: &CustomThemes,) -> Element<'a, Message> {
    let widget = hierarchy.get_widget_by_id(widget_id).unwrap();
    let props = &widget.properties;

    let content = column![
        text("Radio Button Properties").size(TITLE_SIZE),

        widget_name(widget_id, &props.widget_name),

        column![
            text("Label Text").size(LABEL_SIZE),
            text_input("Label", &props.radio_label)
                .on_input(move |v| Message::PropertyChanged(widget_id, PropertyChange::RadioLabel(v)))
                .width(250),
        ]
        .spacing(LABEL_SPACING),

        row![
            column![
                text("Radio Size").size(LABEL_SIZE),
                row![
                    slider(12.0..=40.0, props.radio_size, move |v| {
                        Message::PropertyChanged(widget_id, PropertyChange::RadioSize(v))
                    })
                    .step(1.0)
                    .width(200),
                    text(format!("{:.0}px", props.radio_size)).size(LABEL_SIZE).width(50),
                ]
                .spacing(SECTION_SPACING)
                .align_y(Alignment::Center),
            ]
            .spacing(LABEL_SPACING),
            
            column![
                text("Label Spacing").size(LABEL_SIZE),
                row![
                    slider(0.0..=30.0, props.radio_spacing, move |v| {
                        Message::PropertyChanged(widget_id, PropertyChange::RadioSpacing(v))
                    })
                    .step(1.0)
                    .width(200),
                    text(format!("{:.0}px", props.radio_spacing)).size(LABEL_SIZE).width(50),
                ]
                .spacing(SECTION_SPACING)
                .align_y(Alignment::Center),
            ]
            .spacing(LABEL_SPACING),
        ]
        .spacing(SECTION_SPACING),

        column![
            text("Options").size(SECTION_SIZE),
            column(
                props.radio_options
                    .iter()
                    .enumerate()
                    .map(|(i, option)| {
                        let label = format!("Option {}", i + 1);
                        row![
                            text_input(&label, option)
                                .on_input({
                                    let index = i;
                                    let existing = props.radio_options.clone();
                                    move |v| {
                                        let mut next = existing.clone();
                                        if index < next.len() { next[index] = v; }
                                        Message::PropertyChanged(widget_id, PropertyChange::RadioOptions(next))
                                    }
                                })
                                .width(220),
                            button("Remove")
                                .on_press({
                                    let index = i;
                                    let mut next = props.radio_options.clone();
                                    if index < next.len() && next.len() > 1 { next.remove(index); }
                                    Message::PropertyChanged(widget_id, PropertyChange::RadioOptions(next))
                                }),
                        ]
                        .spacing(SECTION_SPACING)
                        .align_y(Alignment::Center)
                        .into()
                    })
                    .collect::<Vec<Element<'a, Message>>>()
            )
            .spacing(LABEL_SPACING),
            button("Add Option")
                .on_press({
                    let mut next = props.radio_options.clone();
                    next.push(format!("Option {}", next.len() + 1));
                    Message::PropertyChanged(widget_id, PropertyChange::RadioOptions(next))
                })
        ]
        .spacing(SECTION_SPACING),

        column![
            text("Default Selection").size(LABEL_SIZE),
            pick_list(
                props.radio_options.clone(),
                props.radio_options.get(props.radio_selected_index).cloned(),
                move |selected| {
                    let current = props.radio_options.clone();
                    if let Some(ix) = current.iter().position(|s| s == &selected) {
                        Message::PropertyChanged(widget_id, PropertyChange::RadioSelectedIndex(ix))
                    } else {
                        Message::PropertyChanged(widget_id, PropertyChange::RadioSelectedIndex(0))
                    }
                }
            )
            .width(220),
        ]
        .spacing(LABEL_SPACING),

        size_controls_scrollable_aware(
            props.width,
            move |l| Message::PropertyChanged(widget_id, PropertyChange::Width(l)),
            props.height,
            move |l| Message::PropertyChanged(widget_id, PropertyChange::Height(l)),
            hierarchy,
            widget_id,
        ),
    ]
    .spacing(MAIN_SPACING)
    .into();

    scrollable( 
        container(
            add_code_preview(content, hierarchy, widget_id, theme, type_system, custom_styles)
        ).padding(padding::right(10.0))
    ).into()
}

pub fn picklist_controls<'a>(h: &'a WidgetHierarchy, widget_id: WidgetId, theme: &Theme, type_system: &'a TypeSystem, custom_styles: &CustomThemes,) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    let content = column![
        text("Pick List Properties").size(TITLE_SIZE),

        widget_name(widget_id, &props.widget_name),

        column![
            text("Placeholder Text").size(LABEL_SIZE),
            text_input("Placeholder", &props.picklist_placeholder)
                .on_input(move |v| Message::PropertyChanged(widget_id, PropertyChange::PickListPlaceholder(v)))
                .width(250),
        ]
        .spacing(LABEL_SPACING),

        column![
            text("Default Selection").size(LABEL_SIZE),
            pick_list(
                props.picklist_options.clone(),
                props.picklist_selected.clone(),
                move |selection| Message::PropertyChanged(widget_id, PropertyChange::PickListSelected(Some(selection)))
            ),
        ]
        .spacing(LABEL_SPACING),

        column![
            text("Options").size(SECTION_SIZE),
            column(
                props.picklist_options
                    .iter()
                    .enumerate()
                    .map(|(i, option)| {
                        row![
                            text_input(&format!("Option {}", i + 1), option)
                                .on_input({
                                    let index = i;
                                    let current = props.picklist_options.clone();
                                    move |v| {
                                        let mut new_options = current.clone();
                                        if index < new_options.len() {
                                            new_options[index] = v;
                                        }
                                        Message::PropertyChanged(widget_id, PropertyChange::PickListOptions(new_options))
                                    }
                                })
                                .width(200),
                            button("Remove")
                                .on_press({
                                    let index = i;
                                    let mut new_options = props.picklist_options.clone();
                                    if index < new_options.len() {
                                        new_options.remove(index);
                                    }
                                    Message::PropertyChanged(widget_id, PropertyChange::PickListOptions(new_options))
                                })
                                .style(button::danger)
                                .padding(Padding::new(5.0)),
                        ]
                        .spacing(SECTION_SPACING)
                        .align_y(Alignment::Center)
                        .into()
                    })
                    .collect::<Vec<Element<'a, Message>>>()
            )
            .spacing(LABEL_SPACING),

            button("Add Option")
                .on_press({
                    let mut new_options = props.picklist_options.clone();
                    new_options.push(format!("Option {}", new_options.len() + 1));
                    Message::PropertyChanged(widget_id, PropertyChange::PickListOptions(new_options))
                })
                .style(button::success)
                .padding(Padding::new(5.0)),
        ]
        .spacing(SECTION_SPACING),

        size_controls_scrollable_aware(
            props.width,
            move |l| Message::PropertyChanged(widget_id, PropertyChange::Width(l)),
            props.height,
            move |l| Message::PropertyChanged(widget_id, PropertyChange::Height(l)),
            h,
            widget_id,
        ),
    ]
    .spacing(MAIN_SPACING)
    .into();

    scrollable( 
        container(
            add_code_preview(content, h, widget_id, theme, type_system, custom_styles)
        ).padding(padding::right(10.0))
    ).into()
}

pub fn slider_controls<'a>(hierarchy: &'a WidgetHierarchy, widget_id: WidgetId, theme: &Theme, type_system: &'a TypeSystem, custom_styles: &CustomThemes,) -> Element<'a, Message> {
    let widget = hierarchy.get_widget_by_id(widget_id).unwrap();
    let props = &widget.properties;

    let min_str = format!("{:.3}", props.slider_min);
    let max_str = format!("{:.3}", props.slider_max);
    let step_str = format!("{:.3}", props.slider_step);
    let slider_height = format!("{:.0}", props.slider_height);

    let content = column![
        text("Slider Properties").size(TITLE_SIZE),

        widget_name(widget_id, &props.widget_name),

        row![
            column![
                text("Min").size(LABEL_SIZE),
                text_input("min", &min_str).on_input(move |s| {
                    let v = parse_f32(&s, props.slider_min);
                    Message::PropertyChanged(widget_id, PropertyChange::SliderMin(v))
                }).width(120),
            ]
            .spacing(LABEL_SPACING),
            
            column![
                text("Max").size(LABEL_SIZE),
                text_input("max", &max_str).on_input(move |s| {
                    let v = parse_f32(&s, props.slider_max);
                    Message::PropertyChanged(widget_id, PropertyChange::SliderMax(v))
                }).width(120),
            ]
            .spacing(LABEL_SPACING),
            
            column![
                text("Step").size(LABEL_SIZE),
                text_input("step", &step_str).on_input(move |s| {
                    let v = parse_f32(&s, props.slider_step.max(0.000_001));
                    Message::PropertyChanged(widget_id, PropertyChange::SliderStep(v.max(0.000_001)))
                }).width(120),
            ]
            .spacing(LABEL_SPACING),
        ]
        .spacing(SECTION_SPACING),

        column![
            text("Value").size(LABEL_SIZE),
            row![
                slider(props.slider_min..=props.slider_max, props.slider_value, move |val| {
                    Message::PropertyChanged(widget_id, PropertyChange::SliderValue(val))
                })
                .step(props.slider_step.max(0.000_001))
                .width(300),
                text(format!("{:.3}", props.slider_value)).size(LABEL_SIZE),
            ]
            .spacing(SECTION_SPACING)
            .align_y(Alignment::Center),
        ]
        .spacing(LABEL_SPACING),

        column![
            length_picker_scrollable_aware(
                "Width (Length)",
                props.width,
                move |l| Message::PropertyChanged(widget_id, PropertyChange::Width(l)),
                hierarchy,
                widget_id,
                false
            ),
            column![
                text("Height (Thickness)").size(LABEL_SIZE),
                text_input("px", &slider_height).on_input(move |s| {
                    Message::PropertyChanged(widget_id, PropertyChange::SliderHeight(parse_f32(&s, props.slider_height)))
                }).width(120)
            ]
            .spacing(LABEL_SPACING),
        ]
        .spacing(SECTION_SPACING),
    ]
    .spacing(MAIN_SPACING)
    .into();

    scrollable( 
        container(
            add_code_preview(content, hierarchy, widget_id, theme, type_system, custom_styles)
        ).padding(padding::right(10.0))
    ).into()
}

pub fn vertical_slider_controls<'a>(hierarchy: &'a WidgetHierarchy, widget_id: WidgetId, theme: &Theme, type_system: &'a TypeSystem, custom_styles: &CustomThemes,) -> Element<'a, Message> {
    let widget = hierarchy.get_widget_by_id(widget_id).unwrap();
    let props = &widget.properties;

    let min_str = format!("{:.3}", props.slider_min);
    let max_str = format!("{:.3}", props.slider_max);
    let step_str = format!("{:.3}", props.slider_step);
    let slider_width = format!("{:.0}", props.slider_width);

    let content = column![
        text("Vertical Slider Properties").size(TITLE_SIZE),

        widget_name(widget_id, &props.widget_name),

        row![
            column![
                text("Min").size(LABEL_SIZE),
                text_input("min", &min_str).on_input(move |s| {
                    let v = parse_f32(&s, props.slider_min);
                    Message::PropertyChanged(widget_id, PropertyChange::SliderMin(v))
                }).width(120),
            ]
            .spacing(LABEL_SPACING),
            
            column![
                text("Max").size(LABEL_SIZE),
                text_input("max", &max_str).on_input(move |s| {
                    let v = parse_f32(&s, props.slider_max);
                    Message::PropertyChanged(widget_id, PropertyChange::SliderMax(v))
                }).width(120),
            ]
            .spacing(LABEL_SPACING),
            
            column![
                text("Step").size(LABEL_SIZE),
                text_input("step", &step_str).on_input(move |s| {
                    let v = parse_f32(&s, props.slider_step.max(0.000_001));
                    Message::PropertyChanged(widget_id, PropertyChange::SliderStep(v.max(0.000_001)))
                }).width(120),
            ]
            .spacing(LABEL_SPACING),
        ]
        .spacing(SECTION_SPACING),

        column![
            text("Value").size(LABEL_SIZE),
            row![
                slider(props.slider_min..=props.slider_max, props.slider_value, move |val| {
                    Message::PropertyChanged(widget_id, PropertyChange::SliderValue(val))
                })
                .step(props.slider_step.max(0.000_001))
                .width(300),
                text(format!("{:.3}", props.slider_value)).size(LABEL_SIZE),
            ]
            .spacing(SECTION_SPACING)
            .align_y(Alignment::Center),
        ]
        .spacing(LABEL_SPACING),

        column![
            length_picker_scrollable_aware(
                "Height (Length)",
                props.height,
                move |l| Message::PropertyChanged(widget_id, PropertyChange::Height(l)),
                hierarchy,
                widget_id,
                true
            ),
            column![
                text("Width (Thickness)").size(LABEL_SIZE),
                text_input("px", &slider_width).on_input(move |s| {
                    Message::PropertyChanged(widget_id, PropertyChange::SliderWidth(parse_f32(&s, props.slider_width)))
                }).width(120)
            ]
            .spacing(LABEL_SPACING),
        ]
        .spacing(SECTION_SPACING),
    ]
    .spacing(MAIN_SPACING)
    .into();

    scrollable( 
        container(
            add_code_preview(content, hierarchy, widget_id, theme, type_system, custom_styles)
        ).padding(padding::right(10.0))
    ).into()
}

pub fn rule_controls<'a>(h: &'a WidgetHierarchy, widget_id: WidgetId, theme: &Theme, type_system: &'a TypeSystem, custom_styles: &CustomThemes,) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).unwrap();
    let p = &widget.properties;

    let content = column![
        text("Rule Properties").size(TITLE_SIZE),

        column![
            text("Orientation").size(LABEL_SIZE),
            pick_list(
                vec![Orientation::Horizontal, Orientation::Vertical],
                Some(p.orientation),
                move |o| Message::PropertyChanged(widget_id, PropertyChange::Orientation(o))
            )
        ]
        .spacing(LABEL_SPACING),

        column![
            text("Thickness").size(LABEL_SIZE),
            row![
                slider(1.0..=20.0, p.rule_thickness as f32, move |v| {
                    Message::PropertyChanged(widget_id, PropertyChange::RuleThickness(v.round()))
                })
                .step(1.0)
                .width(200),
                text(format!("{:.0}px", p.rule_thickness)).size(LABEL_SIZE).width(50),
            ]
            .spacing(SECTION_SPACING)
            .align_y(Alignment::Center),
        ]
        .spacing(LABEL_SPACING),

        column![
            text("Quick Presets").size(LABEL_SIZE),
            row([1.0_f32, 2.0, 3.0, 4.0, 6.0, 8.0, 12.0].into_iter().map(|px| {
                button(text(format!("{px}px")))
                    .on_press(Message::PropertyChanged(widget_id, PropertyChange::RuleThickness(px)))
                    .padding(6)
                    .into()
            }).collect::<Vec<_>>())
            .spacing(LABEL_SPACING)
        ]
        .spacing(LABEL_SPACING),
    ]
    .spacing(MAIN_SPACING)
    .into();

    scrollable( 
        container(
            add_code_preview(content, h, widget_id, theme, type_system, custom_styles)
        ).padding(padding::right(10.0))
    ).into()
}

pub fn scrollable_controls<'a>(hierarchy: &'a WidgetHierarchy, widget_id: WidgetId, theme: &Theme, type_system: &'a TypeSystem, custom_styles: &CustomThemes,) -> Element<'a, Message> {
    let widget = hierarchy.get_widget_by_id(widget_id).unwrap();
    let props = &widget.properties;

    let content = column![
        text("Scrollable Properties").size(TITLE_SIZE),

        widget_name(widget_id, &props.widget_name),

        size_controls_scrollable_aware(
            props.width,
            move |l| Message::PropertyChanged(widget_id, PropertyChange::Width(l)),
            props.height,
            move |l| Message::PropertyChanged(widget_id, PropertyChange::Height(l)),
            hierarchy,
            widget_id,
        ),

        column![
            text("Direction").size(LABEL_SIZE),
            pick_list(
                vec![DirChoice::Vertical, DirChoice::Horizontal, DirChoice::Both],
                Some(DirChoice::to_choice(props.scroll_dir)),
                move |c| Message::PropertyChanged(widget_id, PropertyChange::ScrollableDirection(DirChoice::from_choice(c)))
            )
        ]
        .spacing(LABEL_SPACING),

        row![
            column![
                text("Anchor X").size(LABEL_SIZE),
                pick_list(
                    vec![AnchorChoice::Start, AnchorChoice::End],
                    Some(AnchorChoice::from(props.anchor_x)),
                    move |a| Message::PropertyChanged(widget_id, PropertyChange::ScrollableAnchorX(AnchorChoice::from_anchor(a)))
                )
            ]
            .spacing(LABEL_SPACING)
            .width(Length::Fill),
            
            column![
                text("Anchor Y").size(LABEL_SIZE),
                pick_list(
                    vec![AnchorChoice::Start, AnchorChoice::End],
                    Some(AnchorChoice::from(props.anchor_y)),
                    move |a| Message::PropertyChanged(widget_id, PropertyChange::ScrollableAnchorY(AnchorChoice::from_anchor(a)))
                )
            ]
            .spacing(LABEL_SPACING)
            .width(Length::Fill),
        ]
        .spacing(SECTION_SPACING),
    ]
    .spacing(MAIN_SPACING)
    .into();

    scrollable( 
        container(
            add_code_preview(content, hierarchy, widget_id, theme, type_system, custom_styles)
        ).padding(padding::right(10.0))
    ).into()
}

pub fn space_controls<'a>(hierarchy: &'a WidgetHierarchy, widget_id: WidgetId, theme: &Theme, type_system: &'a TypeSystem, custom_styles: &CustomThemes,) -> Element<'a, Message> {
    let widget = hierarchy.get_widget_by_id(widget_id).unwrap();
    let props = &widget.properties;

    let content = column![
        text("Space Properties").size(TITLE_SIZE),

        widget_name(widget_id, &props.widget_name),

        column![
            text("Orientation").size(LABEL_SIZE),
            pick_list(
                vec![Orientation::Horizontal, Orientation::Vertical],
                Some(props.orientation),
                move |o| Message::PropertyChanged(widget_id, PropertyChange::Orientation(o))
            ),

            checkbox(props.show_widget_bounds).label("Show Bounds")
        ]
        .spacing(LABEL_SPACING),

        size_controls_scrollable_aware(
            props.width,
            move |l| Message::PropertyChanged(widget_id, PropertyChange::Width(l)),
            props.height,
            move |l| Message::PropertyChanged(widget_id, PropertyChange::Height(l)),
            hierarchy,
            widget_id,
        ),
    ]
    .spacing(MAIN_SPACING)
    .into();

    scrollable( 
        container(
            add_code_preview(content, hierarchy, widget_id, theme, type_system, custom_styles)
        ).padding(padding::right(10.0))
    ).into()
}

pub fn progress_controls<'a>(h: &'a WidgetHierarchy, widget_id: WidgetId, theme: &Theme, type_system: &'a TypeSystem, custom_styles: &CustomThemes,) -> Element<'a, Message> {
    let w = h.get_widget_by_id(widget_id).unwrap();
    let p = &w.properties;
    let girth_str = format!("{:.0}", p.progress_girth);

    let clamp_step = ((p.progress_max - p.progress_min) / 100.0).abs().max(0.001);

    let content = column![
        text("Progress Bar Properties").size(TITLE_SIZE),

        widget_name(widget_id, &p.widget_name),

        row![
            text("Orientation").size(LABEL_SIZE).width(Length::Fixed(80.0)),
            radio("Horizontal", false, Some(p.progress_vertical), move |_|
                Message::PropertyChanged(widget_id, PropertyChange::ProgressVertical(false))
            ),
            radio("Vertical", true, Some(p.progress_vertical), move |_|
                Message::PropertyChanged(widget_id, PropertyChange::ProgressVertical(true))
            ),
        ]
        .spacing(SECTION_SPACING)
        .align_y(Alignment::Center),

        if p.progress_vertical {
            column![
                length_picker_scrollable_aware( 
                    "Length", 
                    p.progress_length, 
                    move |len| Message::PropertyChanged(widget_id, PropertyChange::ProgressLength(len)), 
                    h, 
                    widget_id, 
                    true
                ),
                column![
                    text("Girth (Width)").size(LABEL_SIZE),
                    text_input("px", &girth_str).on_input(move |s| {
                        Message::PropertyChanged(widget_id, PropertyChange::ProgressGirth(parse_f32(&s, p.progress_girth)))
                    }).width(120)
                ]
                .spacing(LABEL_SPACING),
            ]
            .spacing(SECTION_SPACING)
        } else {
            column![
                length_picker_scrollable_aware( 
                    "Length", 
                    p.progress_length, 
                    move |len| Message::PropertyChanged(widget_id, PropertyChange::ProgressLength(len)), 
                    h, 
                    widget_id, 
                    false
                ),
                column![
                    text("Girth (Height)").size(LABEL_SIZE),
                    text_input("px", &girth_str).on_input(move |s| {
                        Message::PropertyChanged(widget_id, PropertyChange::ProgressGirth(parse_f32(&s, p.progress_girth)))
                    }).width(120)
                ]
                .spacing(LABEL_SPACING),
            ]
            .spacing(SECTION_SPACING)
        },

        column![
            text("Range").size(SECTION_SIZE),
            row![
                column![
                    text("Min").size(LABEL_SIZE),
                    text_input("min", &format!("{}", p.progress_min)).on_input(move |s| {
                        let v = s.trim().parse::<f32>().unwrap_or(p.progress_min);
                        Message::PropertyChanged(widget_id, PropertyChange::ProgressMin(v))
                    })
                    .width(120)
                ]
                .spacing(LABEL_SPACING),
                
                column![
                    text("Max").size(LABEL_SIZE),
                    text_input("max", &format!("{}", p.progress_max)).on_input(move |s| {
                        let v = s.trim().parse::<f32>().unwrap_or(p.progress_max);
                        Message::PropertyChanged(widget_id, PropertyChange::ProgressMax(v))
                    })
                    .width(120)
                ]
                .spacing(LABEL_SPACING),
            ]
            .spacing(SECTION_SPACING)
            .align_y(Alignment::Center),
        ]
        .spacing(LABEL_SPACING),

        column![
            text("Value").size(LABEL_SIZE),
            row![
                slider(p.progress_min..=p.progress_max, p.progress_value, move |v| {
                    Message::PropertyChanged(widget_id, PropertyChange::ProgressValue(v))
                })
                .step(clamp_step)
                .width(250),
                text(format!("{:.02}", p.progress_value)).size(LABEL_SIZE).width(60),
            ]
            .spacing(SECTION_SPACING)
            .align_y(Alignment::Center),
        ]
        .spacing(LABEL_SPACING),
    ]
    .spacing(MAIN_SPACING)
    .into();

    scrollable( 
        container(
            add_code_preview(content, h, widget_id, theme, type_system, custom_styles)
        ).padding(padding::right(10.0))
    ).into()
}

pub fn image_controls<'a>(h: &'a WidgetHierarchy, widget_id: WidgetId, theme: &Theme, type_system: &'a TypeSystem, custom_styles: &CustomThemes,) -> Element<'a, Message> {
    let w = h.get_widget_by_id(widget_id).unwrap();
    let props = &w.properties;

    let content = column![
        text("Image Properties").size(TITLE_SIZE),

        widget_name(widget_id, &props.widget_name),
        
        row![
            text("Path").size(LABEL_SIZE).width(Length::Fixed(80.0)),
            text_input("assets/pic.png", &props.image_path)
                .on_input(move |s| Message::PropertyChanged(widget_id, PropertyChange::ImagePath(s)))
                .width(Length::Fill),
        ]
        .spacing(SECTION_SPACING),
        
        row![
            text("Fit").size(LABEL_SIZE).width(Length::Fixed(80.0)),
            pick_list(
                vec![
                    ContentFitChoice::Contain,
                    ContentFitChoice::Cover,
                    ContentFitChoice::Fill,
                    ContentFitChoice::ScaleDown,
                    ContentFitChoice::None,
                ],
                Some(props.image_fit),
                move |v| Message::PropertyChanged(widget_id, PropertyChange::ImageFit(v))
            )
        ]
        .spacing(SECTION_SPACING),

        size_controls_scrollable_aware(
            props.width,
            move |l| Message::PropertyChanged(widget_id, PropertyChange::Width(l)),
            props.height,
            move |l| Message::PropertyChanged(widget_id, PropertyChange::Height(l)),
            h,
            widget_id,
        ),
    ]
    .spacing(MAIN_SPACING)
    .into();

    scrollable( 
        container(
            add_code_preview(content, h, widget_id, theme, type_system, custom_styles)
        ).padding(padding::right(10.0))
    ).into()
}

pub fn svg_controls<'a>(h: &'a WidgetHierarchy, widget_id: WidgetId, theme: &Theme, type_system: &'a TypeSystem, custom_styles: &CustomThemes,) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).unwrap();
    let props = &widget.properties;

    let content = column![
        text("SVG Properties").size(TITLE_SIZE),

        widget_name(widget_id, &props.widget_name),
        
        row![
            text("Path").size(LABEL_SIZE).width(Length::Fixed(80.0)),
            text_input("assets/icon.svg", &props.svg_path)
                .on_input(move |s| Message::PropertyChanged(widget_id, PropertyChange::SvgPath(s)))
                .width(Length::Fill),
        ]
        .spacing(SECTION_SPACING),
        
        row![
            text("Fit").size(LABEL_SIZE).width(Length::Fixed(80.0)),
            pick_list(
                vec![
                    ContentFitChoice::Contain,
                    ContentFitChoice::Cover,
                    ContentFitChoice::Fill,
                    ContentFitChoice::ScaleDown,
                    ContentFitChoice::None,
                ],
                Some(props.svg_fit),
                move |v| Message::PropertyChanged(widget_id, PropertyChange::SvgFit(v))
            )
        ]
        .spacing(SECTION_SPACING),

        size_controls_scrollable_aware(
            props.width,
            move |l| Message::PropertyChanged(widget_id, PropertyChange::Width(l)),
            props.height,
            move |l| Message::PropertyChanged(widget_id, PropertyChange::Height(l)),
            h,
            widget_id,
        ),
    ]
    .spacing(MAIN_SPACING)
    .into();

    scrollable( 
        container(
            add_code_preview(content, h, widget_id, theme, type_system, custom_styles)
        ).padding(padding::right(10.0))
    ).into()
}

pub fn tooltip_controls<'a>(h: &'a WidgetHierarchy, widget_id: WidgetId, theme: &Theme, type_system: &'a TypeSystem, custom_styles: &CustomThemes,) -> Element<'a, Message> {
    let w = h.get_widget_by_id(widget_id).unwrap();
    let p = &w.properties;

    let content = column![
        text("Tooltip Properties").size(TITLE_SIZE),

        widget_name(widget_id, &p.widget_name),
        
        row![
            text("Text").size(LABEL_SIZE).width(Length::Fixed(80.0)),
            text_input("Tooltip text", &p.tooltip_text)
                .on_input(move |s| Message::PropertyChanged(widget_id, PropertyChange::TooltipText(s)))
                .width(Length::Fill),
        ]
        .spacing(SECTION_SPACING),
        
        row![
            text("Position").size(LABEL_SIZE).width(Length::Fixed(80.0)),
            pick_list(
                vec![TooltipPosition::Top, TooltipPosition::Bottom, TooltipPosition::Left, TooltipPosition::Right],
                Some(p.tooltip_position),
                move |pos| Message::PropertyChanged(widget_id, PropertyChange::TooltipPosition(pos))
            )
        ]
        .spacing(SECTION_SPACING),
        
        column![
            text("Tip: Tooltip wraps two children. Add them under it in the tree.")
                .size(LABEL_SIZE)
                .color(Color::from_rgb(0.6, 0.6, 0.6)),
            text("1st child is the element you hover")
                .size(LABEL_SIZE)
                .color(Color::from_rgb(0.6, 0.6, 0.6)),
            text("2nd child is the tooltip content")
                .size(LABEL_SIZE)
                .color(Color::from_rgb(0.6, 0.6, 0.6)),
        ]
        .spacing(LABEL_SPACING),
    ]
    .spacing(MAIN_SPACING)
    .into();

    scrollable( 
        container(
            add_code_preview(content, h, widget_id, theme, type_system, custom_styles)
        ).padding(padding::right(10.0))
    ).into()
}

pub fn combobox_controls<'a>(
    h: &'a WidgetHierarchy, 
    widget_id: WidgetId, 
    theme: &Theme,
    type_system: &'a TypeSystem, 
    custom_styles: &CustomThemes,
) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    let selected = if let Some(referenced_enum) = props.referenced_enum {
        let enum_id = type_system.get_enum(referenced_enum);
        match enum_id {
            Some(enum_def) => enum_def.name.clone(),
            None => String::from("Choose an enum...")
        }
    } else { 
        String::from("Choose an enum...") 
    };

    let content = column![
        text("ComboBox Properties").size(TITLE_SIZE),

        widget_name(widget_id, &props.widget_name),

        column![
            text("Placeholder Text").size(LABEL_SIZE),
            text_input("Placeholder", &props.combobox_placeholder)
                .on_input(move |v| Message::PropertyChanged(widget_id, PropertyChange::ComboBoxPlaceholder(v)))
                .width(300),
        ]
        .spacing(LABEL_SPACING),

        column![
            text("Font Size").size(LABEL_SIZE),
            row![
                slider(8.0..=32.0, props.combobox_size, move |v| {
                    Message::PropertyChanged(widget_id, PropertyChange::ComboBoxSize(v))
                })
                .step(1.0)
                .width(200),
                text(format!("{:.0}px", props.combobox_size)).size(LABEL_SIZE).width(50),
            ]
            .spacing(SECTION_SPACING)
            .align_y(Alignment::Center),
        ]
        .spacing(LABEL_SPACING),
        
        column![
            text("Data Source").size(SECTION_SIZE),
            
            column![
                radio(
                    "Custom Options (strings)",
                    0,
                    Some(props.radio_selected_index),
                    |selected| Message::PropertyChanged(widget_id, PropertyChange::RadioSelectedIndex(selected))
                ),
                radio(
                    "Use Enum",
                    1,
                    Some(props.radio_selected_index),
                    |selected| Message::PropertyChanged(widget_id, PropertyChange::RadioSelectedIndex(selected))
                )
            ]
            .spacing(LABEL_SPACING)
        ]
        .spacing(LABEL_SPACING),

        if props.radio_selected_index == 1 {
            column![
                row![
                    text("Select Enum").size(LABEL_SIZE).width(100),
                    if type_system.enums.is_empty() {
                        column![
                            text("No enums defined yet")
                                .size(LABEL_SIZE)
                                .style(text::warning),
                            button("Create Enum")
                                .on_press(Message::OpenEnumEditor)
                                .style(button::primary)
                        ]
                        .spacing(LABEL_SPACING)
                    } else {
                        column![
                            pick_list(
                                type_system.enum_names(),
                                Some(selected),
                                move |enum_name| {
                                    let enum_id = type_system.get_enum_by_name(&enum_name).expect("MissingEnumDef").id;
                                    Message::PropertyChanged(
                                        widget_id, 
                                        PropertyChange::ComboBoxEnumId(Some(enum_id))
                                    )
                                }
                            )
                            .placeholder("Choose an enum...")
                            .width(200)
                        ]
                    }
                ]
                .spacing(SECTION_SPACING)
                .align_y(Alignment::Center),
                
                if let Some(ref enum_name) = props.referenced_enum {
                    if let Some(enum_def) = type_system.get_enum(enum_name.clone()) {
                        column![
                            text(format!("Variants: {}", enum_def.variants.len()))
                                .size(LABEL_SIZE)
                                .color(Color::from_rgb(0.5, 0.5, 0.5)),
                            scrollable(
                                column(
                                    enum_def.variants.iter().map(|variant| {
                                        text(format!("• {}", variant.name))
                                            .size(LABEL_SIZE)
                                            .into()
                                    }).collect::<Vec<Element<'a, Message>>>()
                                )
                                .spacing(LABEL_SPACING)
                            )
                            .width(Length::Fill)
                            .height(Length::Fixed(100.0))
                        ]
                        .width(Length::Fill)
                        .spacing(LABEL_SPACING)
                    } else {
                        column![
                            text(format!("Enum '{}' not found", enum_name))
                                .size(LABEL_SIZE)
                                .color(Color::from_rgb(0.7, 0.3, 0.3))
                        ]
                    }
                } else {
                    column![]
                }
            ]
            .spacing(SECTION_SPACING)
        } else {
            column![
                text("Custom Options").size(SECTION_SIZE),
                column(
                    props.combobox_options
                        .iter()
                        .enumerate()
                        .map(|(i, option)| {
                            row![
                                text_input(&format!("Option {}", i + 1), option)
                                    .on_input({
                                        let index = i;
                                        let current = props.combobox_options.clone();
                                        move |v| {
                                            let mut new_options = current.clone();
                                            if index < new_options.len() {
                                                new_options[index] = v;
                                            }
                                            Message::PropertyChanged(widget_id, PropertyChange::ComboBoxState(new_options))
                                        }
                                    })
                                    .width(200),
                                button("Remove")
                                    .on_press({
                                        let index = i;
                                        let mut new_options = props.combobox_options.clone();
                                        if index < new_options.len() && new_options.len() > 1 {
                                            new_options.remove(index);
                                        }
                                        Message::PropertyChanged(widget_id, PropertyChange::ComboBoxState(new_options))
                                    })
                                    .style(button::danger),
                            ]
                            .spacing(SECTION_SPACING)
                            .align_y(Alignment::Center)
                            .into()
                        })
                        .collect::<Vec<Element<'a, Message>>>()
                )
                .spacing(LABEL_SPACING),

                button("Add Option")
                    .on_press({
                        let mut new_options = props.combobox_options.clone();
                        new_options.push(format!("Option {}", new_options.len() + 1));
                        Message::PropertyChanged(widget_id, PropertyChange::ComboBoxState(new_options))
                    })
                    .style(button::success),
            ]
            .spacing(SECTION_SPACING)
        },

        column![
            text("ComboBox Event Handlers").size(SECTION_SIZE),
            text("Enable optional event handlers for advanced interactions")
                .size(LABEL_SIZE)
                .color(Color::from_rgb(0.6, 0.6, 0.6)),
            
            column![
                checkbox(props.combobox_use_on_input)
                    .label("on_input - Triggers when typing")
                    .on_toggle(move |v| Message::PropertyChanged(
                        widget_id, 
                        PropertyChange::ComboBoxUseOnInput(v)
                    )),
                
                checkbox(props.combobox_use_on_option_hovered)
                    .label("on_option_hovered - Triggers on arrow key navigation or hover")
                    .on_toggle(move |v| Message::PropertyChanged(
                        widget_id, 
                        PropertyChange::ComboBoxUseOnOptionHovered(v)
                    )),
                
                checkbox(props.combobox_use_on_open)
                    .label("on_open - Triggers when dropdown opens")
                    .on_toggle(move |v| Message::PropertyChanged(
                        widget_id, 
                        PropertyChange::ComboBoxUseOnOpen(v)
                    )),
                
                checkbox(props.combobox_use_on_close)
                    .label("on_close - Triggers when dropdown closes")
                    .on_toggle(move |v| Message::PropertyChanged(
                        widget_id, 
                        PropertyChange::ComboBoxUseOnClose(v)
                    )),
            ]
            .spacing(LABEL_SPACING)
        ]
        .spacing(SECTION_SPACING),

        size_controls_scrollable_aware(
            props.width,
            move |l| Message::PropertyChanged(widget_id, PropertyChange::Width(l)),
            props.height,
            move |l| Message::PropertyChanged(widget_id, PropertyChange::Height(l)),
            h,
            widget_id,
        ),
    ]
    .spacing(MAIN_SPACING)
    .into();

    scrollable( 
        container(
            add_code_preview(content, h, widget_id, theme, type_system, custom_styles)
        ).padding(padding::right(10.0))
    ).into()
}

pub fn markdown_controls<'a>(h: &'a WidgetHierarchy, widget_id: WidgetId, theme: &Theme, type_system: &'a TypeSystem, custom_styles: &CustomThemes,) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    let content = column![
        text("Markdown Properties").size(TITLE_SIZE),

        widget_name(widget_id, &props.widget_name),

        column![
            text("Markdown Content").size(LABEL_SIZE),
            text_editor(&props.markdown_source)
                .placeholder("Markdown text here")
                .on_action(move |act| Message::PropertyChanged(widget_id, PropertyChange::MarkdownContent(act)))
                .height(Length::Fixed(180.0))
                .width(350.0),
        ]
        .spacing(LABEL_SPACING),

        column![
            text("Text Size").size(LABEL_SIZE),
            row![
                slider(8.0..=32.0, props.markdown_text_size, move |v| {
                    Message::PropertyChanged(widget_id, PropertyChange::MarkdownTextSize(v))
                })
                .step(1.0)
                .width(200),
                text(format!("{:.0}px", props.markdown_text_size)).size(LABEL_SIZE).width(50),
            ]
            .spacing(SECTION_SPACING)
            .align_y(Alignment::Center),
        ]
        .spacing(LABEL_SPACING),

        size_controls_scrollable_aware(
            props.width,
            move |l| Message::PropertyChanged(widget_id, PropertyChange::Width(l)),
            props.height,
            move |l| Message::PropertyChanged(widget_id, PropertyChange::Height(l)),
            h,
            widget_id,
        ),
    ]
    .spacing(MAIN_SPACING)
    .into();

    scrollable( 
        container(
            add_code_preview(content, h, widget_id, theme, type_system, custom_styles)
        ).padding(padding::right(10.0))
    ).into()
}

pub fn qrcode_controls<'a>(h: &'a WidgetHierarchy, widget_id: WidgetId, theme: &Theme, type_system: &'a TypeSystem, custom_styles: &CustomThemes,) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    let content = column![
        text("QR Code Properties").size(TITLE_SIZE),

        widget_name(widget_id, &props.widget_name),

        column![
            text("Data to Encode").size(LABEL_SIZE),
            text_input("Data", &props.qrcode_link)
                .on_input(move |v| Message::PropertyChanged(widget_id, PropertyChange::QRCodeData(v)))
                .width(350),
        ]
        .spacing(LABEL_SPACING),

        column![
            text("Cell Size").size(LABEL_SIZE),
            row![
                slider(1.0..=20.0, props.qrcode_cell_size as f32, move |v| {
                    Message::PropertyChanged(widget_id, PropertyChange::QRCodeCellSize(v))
                })
                .step(1.0)
                .width(200),
                text(format!("{}px", props.qrcode_cell_size)).size(LABEL_SIZE).width(50),
            ]
            .spacing(SECTION_SPACING)
            .align_y(Alignment::Center),
        ]
        .spacing(LABEL_SPACING),
    ]
    .spacing(MAIN_SPACING)
    .into();

    scrollable( 
        container(
            add_code_preview(content, h, widget_id, theme, type_system, custom_styles)
        ).padding(padding::right(10.0))
    ).into()
}

pub fn stack_controls<'a>(h: &'a WidgetHierarchy, widget_id: WidgetId, theme: &Theme, type_system: &'a TypeSystem, custom_styles: &CustomThemes,) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    let content = column![
        text("Stack Properties").size(TITLE_SIZE),

        widget_name(widget_id, &props.widget_name),

        text("Stack overlays its children on top of each other.")
            .size(LABEL_SIZE)
            .color(Color::from_rgb(0.6, 0.6, 0.6)),
        
        text("The first child is at the bottom, last child is on top.")
            .size(LABEL_SIZE)
            .color(Color::from_rgb(0.6, 0.6, 0.6)),

        size_controls_scrollable_aware(
            props.width,
            move |l| Message::PropertyChanged(widget_id, PropertyChange::Width(l)),
            props.height,
            move |l| Message::PropertyChanged(widget_id, PropertyChange::Height(l)),
            h,
            widget_id,
        ),
    ]
    .spacing(MAIN_SPACING)
    .into();

    scrollable( 
        container(
            add_code_preview(content, h, widget_id, theme, type_system, custom_styles)
        ).padding(padding::right(10.0))
    ).into()
}

pub fn mousearea_controls<'a>(h: &'a WidgetHierarchy, widget_id: WidgetId, theme: &Theme, type_system: &'a TypeSystem, custom_styles: &CustomThemes,) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    let content = column![
        text("Mouse Area Properties").size(TITLE_SIZE),

        widget_name(widget_id, &props.widget_name),

        text("Mouse Area captures mouse events over its child widget.")
            .size(LABEL_SIZE)
            .color(Color::from_rgb(0.6, 0.6, 0.6)),

        // Event Handlers Section
        column![
            text("Event Handlers").size(SECTION_SIZE),
            
            // Left button events
            column![
                text("Left Mouse Button:").size(LABEL_SIZE),
                checkbox(props.mousearea_on_press)
                    .label("on_press")
                    .on_toggle(move |v| Message::PropertyChanged(widget_id, PropertyChange::MouseAreaOnPress(v))),
                checkbox(props.mousearea_on_release)
                    .label("on_release")
                    .on_toggle(move |v| Message::PropertyChanged(widget_id, PropertyChange::MouseAreaOnRelease(v))),
                checkbox(props.mousearea_on_double_click)
                    .label("on_double_click")
                    .on_toggle(move |v| Message::PropertyChanged(widget_id, PropertyChange::MouseAreaOnDoubleClick(v))),
            ].spacing(LABEL_SPACING),
            
            // Right button events
            column![
                text("Right Mouse Button:").size(LABEL_SIZE),
                checkbox(props.mousearea_on_right_press)
                    .label("on_right_press")
                    .on_toggle(move |v| Message::PropertyChanged(widget_id, PropertyChange::MouseAreaOnRightPress(v))),
                checkbox(props.mousearea_on_right_release)
                    .label("on_right_release")
                    .on_toggle(move |v| Message::PropertyChanged(widget_id, PropertyChange::MouseAreaOnRightRelease(v))),
            ].spacing(LABEL_SPACING),
            
            // Middle button events
            column![
                text("Middle Mouse Button:").size(LABEL_SIZE),
                checkbox(props.mousearea_on_middle_press)
                    .label("on_middle_press")
                    .on_toggle(move |v| Message::PropertyChanged(widget_id, PropertyChange::MouseAreaOnMiddlePress(v))),
                checkbox(props.mousearea_on_middle_release)
                    .label("on_middle_release")
                    .on_toggle(move |v| Message::PropertyChanged(widget_id, PropertyChange::MouseAreaOnMiddleRelease(v))),
            ].spacing(LABEL_SPACING),
            
            // Other events
            column![
                text("Other Events:").size(LABEL_SIZE),
                checkbox(props.mousearea_on_scroll)
                    .label("on_scroll (with ScrollDelta)")
                    .on_toggle(move |v| Message::PropertyChanged(widget_id, PropertyChange::MouseAreaOnScroll(v))),
                checkbox(props.mousearea_on_enter)
                    .label("on_enter")
                    .on_toggle(move |v| Message::PropertyChanged(widget_id, PropertyChange::MouseAreaOnEnter(v))),
                checkbox(props.mousearea_on_move)
                    .label("on_move (with Point)")
                    .on_toggle(move |v| Message::PropertyChanged(widget_id, PropertyChange::MouseAreaOnMove(v))),
                checkbox(props.mousearea_on_exit)
                    .label("on_exit")
                    .on_toggle(move |v| Message::PropertyChanged(widget_id, PropertyChange::MouseAreaOnExit(v))),
            ].spacing(LABEL_SPACING),
            
            // Mouse interaction picker
            column![
                text("Mouse Cursor:").size(LABEL_SIZE),
                pick_list(
                    MouseInteraction::ALL,
                    props.mousearea_interaction,
                    move |v| Message::PropertyChanged(widget_id, PropertyChange::MouseAreaInteraction(Some(v)))
                )
                .placeholder("Default cursor"),
            ].spacing(LABEL_SPACING),
        ].spacing(SECTION_SPACING),

        size_controls_scrollable_aware(
            props.width,
            move |l| Message::PropertyChanged(widget_id, PropertyChange::Width(l)),
            props.height,
            move |l| Message::PropertyChanged(widget_id, PropertyChange::Height(l)),
            h,
            widget_id,
        ),
    ]
    .spacing(MAIN_SPACING)
    .into();

    scrollable( 
        container(
            add_code_preview(content, h, widget_id, theme, type_system, custom_styles)
        ).padding(padding::right(10.0))
    ).into()
}

pub fn themer_controls<'a>(h: &'a WidgetHierarchy, widget_id: WidgetId, theme: &Theme, type_system: &'a TypeSystem, custom_styles: &CustomThemes,) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    let content = column![
        text("Themer Properties").size(TITLE_SIZE),

        widget_name(widget_id, &props.widget_name),

        column![
            text("Theme").size(LABEL_SIZE),
            pick_list(
                Theme::ALL,
                props.themer_theme.clone(),
                move |theme| Message::PropertyChanged(widget_id, PropertyChange::ThemerTheme(Some(theme)))
            )
            .placeholder("Inherit from parent"),
        ]
        .spacing(LABEL_SPACING),

        text("Themer applies a theme to all its children.")
            .size(LABEL_SIZE)
            .color(Color::from_rgb(0.6, 0.6, 0.6)),

        size_controls_scrollable_aware(
            props.width,
            move |l| Message::PropertyChanged(widget_id, PropertyChange::Width(l)),
            props.height,
            move |l| Message::PropertyChanged(widget_id, PropertyChange::Height(l)),
            h,
            widget_id,
        ),
    ]
    .spacing(MAIN_SPACING)
    .into();

    scrollable( 
        container(
            add_code_preview(content, h, widget_id, theme, type_system, custom_styles)
        ).padding(padding::right(10.0))
    ).into()
}

pub fn pin_controls<'a>(h: &'a WidgetHierarchy, widget_id: WidgetId, theme: &Theme, type_system: &'a TypeSystem, custom_styles: &CustomThemes,) -> Element<'a, Message> {
    let w = h.get_widget_by_id(widget_id).unwrap();
    let props = &w.properties;

    let content = column![
        text("Pin Properties").size(TITLE_SIZE),

        widget_name(widget_id, &props.widget_name),
        
        row![
            text("Position").size(LABEL_SIZE).width(Length::Fixed(80.0)),
            column![],
        ]
        .spacing(SECTION_SPACING),

        size_controls_scrollable_aware(
            props.width,
            move |l| Message::PropertyChanged(widget_id, PropertyChange::Width(l)),
            props.height,
            move |l| Message::PropertyChanged(widget_id, PropertyChange::Height(l)),
            h,
            widget_id,
        ),
    ]
    .spacing(MAIN_SPACING)
    .into();

    scrollable( 
        container(
            add_code_preview(content, h, widget_id, theme, type_system, custom_styles)
        ).padding(padding::right(10.0))
    ).into()
}

pub fn view_reference_controls<'a>(
    h: &'a WidgetHierarchy, 
    widget_id: WidgetId, 
    theme: &Theme, 
    type_system: &'a TypeSystem, 
    views: &'a BTreeMap<Uuid, AppView>,
    custom_styles: &CustomThemes,
) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    // Create a list of (Uuid, Name) for the pick_list
    let view_options: Vec<(Uuid, String)> = views
        .iter()
        .map(|(id, view)| (*id, view.name.clone()))
        .collect();
    
    // Get the currently selected view name (if any)
    let selected_view_name = props.referenced_view_id
        .and_then(|view_id| views.get(&view_id))
        .map(|view| view.name.clone());

    let content = column![
        text("View Reference Properties").size(TITLE_SIZE),

        widget_name(widget_id, &props.widget_name),

        column![
            text("Referenced View").size(SECTION_SIZE),
            text("Select which view to display here:")
                .size(LABEL_SIZE)
                .color(Color::from_rgb(0.6, 0.6, 0.6)),
            
            pick_list(
                view_options.iter().map(|(_, name)| name.clone()).collect::<Vec<_>>(),
                selected_view_name,
                move |selected_name| {
                    // Find the UUID for the selected name
                    let view_id = view_options
                        .iter()
                        .find(|(_, name)| name == &selected_name)
                        .map(|(id, _)| *id);
                    
                    Message::PropertyChanged(
                        widget_id,
                        PropertyChange::ViewReferenceId(view_id, selected_name)
                    )
                }
            )
            .placeholder("Select a view...")
            .width(300),
        ]
        .spacing(LABEL_SPACING),

        // Show warning if current view is selected (would be circular)
        if let Some(view_id) = props.referenced_view_id {
            if let Some(view) = views.get(&view_id) {
                column![
                    rule::horizontal(2),
                    text(format!("Selected: {}", view.name))
                        .size(LABEL_SIZE)
                        .color(theme.extended_palette().success.base.color),
                    text("This view will be embedded here")
                        .size(LABEL_SIZE - 2.0)
                        .color(Color::from_rgb(0.5, 0.5, 0.5)),
                ]
                .spacing(LABEL_SPACING)
            } else {
                column![
                    text("Selected view no longer exists")
                        .size(LABEL_SIZE)
                        .color(theme.extended_palette().danger.base.color),
                ]
            }
        } else {
            column![]
        },

        size_controls_scrollable_aware(
            props.width,
            move |l| Message::PropertyChanged(widget_id, PropertyChange::Width(l)),
            props.height,
            move |l| Message::PropertyChanged(widget_id, PropertyChange::Height(l)),
            h,
            widget_id,
        ),
    ]
    .spacing(MAIN_SPACING)
    .into();

    scrollable( 
        container(
            add_code_preview(content, h, widget_id, theme, type_system, custom_styles)
        ).padding(padding::right(10.0))
    ).into()
}

fn parse_f32(s: &str, default: f32) -> f32 {
    s.trim().parse::<f32>().unwrap_or(default)
}

fn parse_color_hex(s: &str, default: Color) -> Color {
    let t = s.trim().trim_start_matches('#');
    let hex = |i: usize| u8::from_str_radix(&t[i..i+2], 16).ok();
    match t.len() {
        6 => {
            if let (Some(r), Some(g), Some(b)) = (hex(0), hex(2), hex(4)) {
                return Color::from_rgba8(r, g, b, 255.0);
            }
            default
        }
        8 => {
            if let (Some(r), Some(g), Some(b), Some(a)) = (hex(0), hex(2), hex(4), hex(6)) {
                return Color::from_rgba8(r, g, b, a.into());
            }
            default
        }
        _ => default,
    }
}

fn color_to_hex(c: Color) -> String {
    let [r,g,b,a] = [
        (c.r * 255.0).round().clamp(0.0,255.0) as u8,
        (c.g * 255.0).round().clamp(0.0,255.0) as u8,
        (c.b * 255.0).round().clamp(0.0,255.0) as u8,
        (c.a * 255.0).round().clamp(0.0,255.0) as u8,
    ];
    if a == 255 { format!("#{:02X}{:02X}{:02X}", r,g,b) }
    else { format!("#{:02X}{:02X}{:02X}{:02X}", r,g,b,a) }
}

fn color_hex_input<'a, F>(label: &'a str, current: Color, on_change: F) -> Element<'a, Message>
where F: Fn(Color) -> Message + 'a + Copy {
    let cur = color_to_hex(current);
    column![
        text(label),
        text_input("#RRGGBB or #RRGGBBAA", &cur)
            .on_input(move |s| on_change(parse_color_hex(&s, current)))
            .width(160)
    ]
    .spacing(5)
    .into()
}

/// Helper for scrollable-aware size controls
pub fn size_controls_scrollable_aware<'a>(
    width_now: Length,
    on_width: impl Fn(Length) -> Message + 'a + Copy,
    height_now: Length,
    on_height: impl Fn(Length) -> Message + 'a + Copy,
    hierarchy: &'a WidgetHierarchy,
    widget_id: WidgetId,
) -> Element<'a, Message> {
    let widget = hierarchy.get_widget_by_id(widget_id);
    let props = widget.map(|w| &w.properties);
    
    row![
        length_picker_with_draft(
            "Width",
            width_now,
            props.map(|p| {
                match width_now {
                    Length::Fixed(_) => &p.draft_fixed_width,
                    Length::FillPortion(_) => &p.draft_fill_portion_width,
                    _ => &p.draft_fixed_width,
                }
            }),
            on_width,
            move |text| Message::PropertyChanged(
                widget_id,
                match width_now {
                    Length::Fixed(_) => PropertyChange::DraftFixedWidth(text),
                    Length::FillPortion(_) => PropertyChange::DraftFillPortionWidth(text),
                    _ => PropertyChange::Noop,
                }
            ),
            hierarchy,
            widget_id,
            false,
        ),
        
        length_picker_with_draft(
            "Height",
            height_now,
            props.map(|p| {
                match height_now {
                    Length::Fixed(_) => &p.draft_fixed_height,
                    Length::FillPortion(_) => &p.draft_fill_portion_height,
                    _ => &p.draft_fixed_height
                }
            }),
            on_height,
            move |text| Message::PropertyChanged(
                widget_id,
                match height_now {
                    Length::Fixed(_) => PropertyChange::DraftFixedHeight(text),
                    Length::FillPortion(_) => PropertyChange::DraftFillPortionHeight(text),
                    _ => PropertyChange::Noop,
                }
            ),
            hierarchy,
            widget_id,
            true,
        ),
    ]
    .spacing(MAIN_SPACING)
    .into()
}

/// A single labeled Length selector that's aware of scrollable constraints
pub fn length_picker_scrollable_aware<'a, F>(
    label: &'a str,
    current: Length,
    on_change: F,
    hierarchy: &WidgetHierarchy,
    widget_id: WidgetId,
    is_height: bool, // true for height, false for width
) -> Element<'a, Message>
where
    F: Fn(Length) -> Message + 'a + Copy,
{
    const DEFAULT_PX: f32 = 120.0;
    const DEFAULT_PORTION: u16 = 1;

    // Check if under scrollable and what dimensions are constrained
    let (can_fill, saved_value) = if let Some((_, scroll_dir)) = hierarchy.get_scrollable_ancestor_info(widget_id) {
        let height_blocked = match scroll_dir {
            iced::widget::scrollable::Direction::Vertical(_) => true,
            iced::widget::scrollable::Direction::Both { .. } => true,
            iced::widget::scrollable::Direction::Horizontal(_) => false,
        };
        
        let width_blocked = match scroll_dir {
            iced::widget::scrollable::Direction::Horizontal(_) => true,
            iced::widget::scrollable::Direction::Both { .. } => true,
            iced::widget::scrollable::Direction::Vertical(_) => false,
        };
        
        let blocked = if is_height { height_blocked } else { width_blocked };
        
        if blocked {
            // Get the saved value if it exists
            let saved = if let Some(widget) = hierarchy.get_widget_by_id(widget_id) {
                if is_height {
                    widget.properties.saved_height_before_scrollable
                } else {
                    widget.properties.saved_width_before_scrollable
                }
            } else {
                None
            };
            (!blocked, saved)
        } else {
            (true, None)
        }
    } else {
        (true, None)
    };

    let choice_now = LengthChoice::from_length(current);

    // Build available choices based on scrollable constraints
    let mut available_choices = vec![LengthChoice::Shrink, LengthChoice::Fixed];
    if can_fill {
        available_choices.insert(0, LengthChoice::Fill);
        available_choices.insert(1, LengthChoice::FillPortion);
    }

    let picker = column![
        if !can_fill && saved_value.is_some() {
            column![
                text(label),
                text(format!("(was: {})", length_to_string(saved_value.unwrap())))
                    .size(10)
                    .color(Color::from_rgb(0.6, 0.6, 0.6))
            ]
        } else {
            column![text(label)]
        },
        pick_list(
            available_choices,
            Some(choice_now),
            move |choice| {
                let new_len = match choice {
                    LengthChoice::Fill => Length::Fill,
                    LengthChoice::FillPortion => match current {
                        Length::FillPortion(p) => Length::FillPortion(p),
                        _ => Length::FillPortion(DEFAULT_PORTION),
                    },
                    LengthChoice::Shrink => Length::Shrink,
                    LengthChoice::Fixed => {
                        match current {
                            Length::Fixed(px) => Length::Fixed(px),
                            _ => Length::Fixed(DEFAULT_PX),
                        }
                    }
                };
                on_change(new_len)
            }
        )
        .width(160)
    ]
    .spacing(5)
    .width(Length::Shrink);

    // Secondary control for Fixed and FillPortion
    let extra: Element<_> = match choice_now {
        LengthChoice::Fixed => {
            let value_str = match current {
                Length::Fixed(px) => format!("{px}"),
                _ => format!("{DEFAULT_PX}"),
            };
            column![
                text("Pixels"),
                text_input("e.g. 120.0", &value_str)
                    .on_input(move |v| on_change(parse_length(&v)))
                    .width(120)
            ]
            .spacing(5)
            .into()
        }
        LengthChoice::FillPortion if can_fill => {
            let portion_now = match current {
                Length::FillPortion(p) => p,
                _ => DEFAULT_PORTION,
            };
            let value_str = portion_now.to_string();
            column![
                text("Portion"),
                text_input("e.g. 1", &value_str)
                    .on_input(move |v| {
                        let p = v.trim().parse::<u16>().ok().map(|x| x.max(1)).unwrap_or(DEFAULT_PORTION);
                        on_change(Length::FillPortion(p))
                    })
                    .width(120)
            ]
            .spacing(5)
            .into()
        }
        _ => Space::new().width(0).into(),
    };

    row![picker, extra].spacing(15).into()
}

pub fn length_picker_with_draft<'a>(
    label: &'a str,
    current: Length,
    draft_text: Option<&'a String>,
    on_change: impl Fn(Length) -> Message + 'a + Copy,
    on_draft_change: impl Fn(String) -> Message + 'a + Copy,
    hierarchy: &WidgetHierarchy,
    widget_id: WidgetId,
    is_height: bool,
) -> Element<'a, Message> {
    const DEFAULT_PX: f32 = 120.0;
    const DEFAULT_PORTION: u16 = 1;

    let (can_fill, saved_value) = if let Some((_, scroll_dir)) = hierarchy.get_scrollable_ancestor_info(widget_id) {
        let height_blocked = matches!(
            scroll_dir,
            iced::widget::scrollable::Direction::Vertical(_) 
            | iced::widget::scrollable::Direction::Both { .. }
        );
        
        let width_blocked = matches!(
            scroll_dir,
            iced::widget::scrollable::Direction::Horizontal(_) 
            | iced::widget::scrollable::Direction::Both { .. }
        );
        
        let blocked = if is_height { height_blocked } else { width_blocked };
        
        if blocked {
            let saved = if let Some(widget) = hierarchy.get_widget_by_id(widget_id) {
                if is_height {
                    widget.properties.saved_height_before_scrollable
                } else {
                    widget.properties.saved_width_before_scrollable
                }
            } else {
                None
            };
            (!blocked, saved)
        } else {
            (true, None)
        }
    } else {
        (true, None)
    };

    let choice_now = LengthChoice::from_length(current);

    let mut available_choices = vec![LengthChoice::Shrink, LengthChoice::Fixed];
    if can_fill {
        available_choices.insert(0, LengthChoice::Fill);
        available_choices.insert(1, LengthChoice::FillPortion);
    }

    let picker = column![
        if !can_fill && saved_value.is_some() {
            column![
                text(label).size(LABEL_SIZE),
                text(format!("(was: {})", length_to_string(saved_value.unwrap())))
                    .size(LABEL_SIZE)
            ]
        } else {
            column![text(label).size(LABEL_SIZE)]
        },
        pick_list(
            available_choices,
            Some(choice_now),
            move |choice| {
                let new_len = match choice {
                    LengthChoice::Fill => Length::Fill,
                    LengthChoice::FillPortion => match current {
                        Length::FillPortion(p) => Length::FillPortion(p),
                        _ => Length::FillPortion(DEFAULT_PORTION),
                    },
                    LengthChoice::Shrink => Length::Shrink,
                    LengthChoice::Fixed => match current {
                        Length::Fixed(px) => Length::Fixed(px),
                        _ => Length::Fixed(DEFAULT_PX),
                    },
                };
                on_change(new_len)
            }
        )
        .width(160)
    ]
    .spacing(LABEL_SPACING)
    .width(Length::Shrink);

    let extra: Element<_> = match choice_now {
        LengthChoice::Fixed => {
            let committed_value = match current {
                Length::Fixed(px) => format!("{px}"),
                _ => format!("{DEFAULT_PX}"),
            };
            
            let display_text = draft_text.map(|s| s.as_str()).unwrap_or("");
            
            column![
                text("Pixels").size(LABEL_SIZE),
                text_input(&committed_value, display_text)
                    .on_input(move |v| {
                        on_draft_change(v)
                    })
                    .width(75)
            ]
            .spacing(LABEL_SPACING)
            .into()
        }
        LengthChoice::FillPortion if can_fill => {
            let committed_value = match current {
                Length::FillPortion(p) => p.to_string(),
                _ => DEFAULT_PORTION.to_string(),
            };
            
            let display_text = draft_text.map(|s| s.as_str()).unwrap_or("");
            
            column![
                text("Portion").size(LABEL_SIZE),
                text_input(&committed_value, display_text)
                    .on_input(move |v| {
                        on_draft_change(v)
                    })
                    .width(75)
            ]
            .spacing(LABEL_SPACING)
            .into()
        }
        _ => space::horizontal().into(),
    };

    row![picker, extra].width(Length::Fill).spacing(SECTION_SPACING).into()
}

pub fn padding_controls<'a>(
    current_padding: Padding,
    widget_id: WidgetId,
    padding_mode: PaddingMode,
    theme: &Theme
) -> Element<'a, Message> {
    column![
        text("Padding").size(SECTION_SIZE),
        
        // Mode selection
        column![
            row![
                row![
                    radio(
                        "Uniform",
                        PaddingMode::Uniform,
                        Some(padding_mode),
                        move |mode| Message::PropertyChanged(
                            widget_id,
                            PropertyChange::PaddingMode(mode)
                        )
                    ),
                    information(theme, "Pad all sides equally")
                ]
                .spacing(LABEL_SPACING)
                .align_y(Alignment::End),

                row![
                    radio(
                        "Symmetric",
                        PaddingMode::Symmetric,
                        Some(padding_mode),
                        move |mode| Message::PropertyChanged(
                            widget_id,
                            PropertyChange::PaddingMode(mode)
                        )
                    ),
                    information(theme, "Pad vertical and horizontal pairs evenly")
                ]
                .spacing(LABEL_SPACING)
                .align_y(Alignment::End),

                row![
                    radio(
                        "Individual",
                        PaddingMode::Individual,
                        Some(padding_mode),
                        move |mode| Message::PropertyChanged(
                            widget_id,
                            PropertyChange::PaddingMode(mode)
                        )
                    ),
                    information(theme, "Pad each size Individually")
                ]
                .spacing(LABEL_SPACING)
                .align_y(Alignment::End),
            ]
            .spacing(15.0)
        ]
        .spacing(LABEL_SPACING),
        
        match padding_mode {
            PaddingMode::Uniform => {
                column![
                    text("All Sides").size(LABEL_SIZE),
                    row![
                        slider(0.0..=50.0, current_padding.top, move |v| {
                            Message::PropertyChanged(
                                widget_id,
                                PropertyChange::PaddingUniform(v)
                            )
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
                column![
                    row![
                        row![
                            column![
                                text("Vertical (Top/Bottom)").size(LABEL_SIZE),
                                slider(0.0..=50.0, current_padding.top, move |v| {
                                    Message::PropertyChanged(
                                        widget_id,
                                        PropertyChange::PaddingVertical(v)
                                    )
                                })
                                .step(1.0)
                                .width(250),
                                text(format!("{:.0}px", current_padding.top))
                                    .size(LABEL_SIZE)
                                    .width(50),
                            ]
                            .spacing(LABEL_SPACING)
                            .width(Length::Fill),
                        ],
                        row![
                            column![
                                text("Horizontal (Left/Right)").size(LABEL_SIZE),
                                slider(0.0..=50.0, current_padding.left, move |v| {
                                    Message::PropertyChanged(
                                        widget_id,
                                        PropertyChange::PaddingHorizontal(v)
                                    )
                                })
                                .step(1.0)
                                .width(250),
                                text(format!("{:.0}px", current_padding.left))
                                    .size(LABEL_SIZE)
                                    .width(50),
                            ]
                            .spacing(LABEL_SPACING)
                            .width(Length::Fill),
                        ],
                    ]
                ]
                .spacing(SECTION_SPACING)
            }
            
            PaddingMode::Individual => {
                column![
                    row![
                        column![
                            text("Top").size(LABEL_SIZE),
                            slider(0.0..=50.0, current_padding.top, move |v| {
                                Message::PropertyChanged(
                                    widget_id,
                                    PropertyChange::PaddingTop(v)
                                )
                            })
                            .step(1.0)
                            .width(250),
                            text(format!("{:.0}px", current_padding.top))
                                .size(LABEL_SIZE)
                                .center(),
                        ]
                        .spacing(LABEL_SPACING)
                        .width(Length::Fill),
                        
                        column![
                            text("Right").size(LABEL_SIZE),
                            slider(0.0..=50.0, current_padding.right, move |v| {
                                Message::PropertyChanged(
                                    widget_id,
                                    PropertyChange::PaddingRight(v)
                                )
                            })
                            .step(1.0)
                            .width(250),
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
                            slider(0.0..=50.0, current_padding.bottom, move |v| {
                                Message::PropertyChanged(
                                    widget_id,
                                    PropertyChange::PaddingBottom(v)
                                )
                            })
                            .step(1.0)
                            .width(250),
                            text(format!("{:.0}px", current_padding.bottom))
                                .size(LABEL_SIZE)
                                .center(),
                        ]
                        .spacing(LABEL_SPACING)
                        .width(Length::Fill),
                        
                        column![
                            text("Left").size(LABEL_SIZE),
                            slider(0.0..=50.0, current_padding.left, move |v| {
                                Message::PropertyChanged(
                                    widget_id,
                                    PropertyChange::PaddingLeft(v)
                                )
                            })
                            .step(1.0)
                            .width(250),
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

pub fn information<'a>(theme: &Theme, info: &'a str) -> Element<'a, Message> {
    let palette = theme.extended_palette();
    tooltip(
        icon::info().center().size(14).color(palette.background.stronger.color),
        container(
            text(info)
                .size(12)
        ).style(container::rounded_box).padding([5, 10]),
        tooltip::Position::Top
    ).into()
}

pub fn clip_control<'a>(widget_id: WidgetId, clipped: bool, theme: &Theme) -> Element<'a, Message>{
        row![
            checkbox(clipped,)
                .label("Enable Clipping")
                .on_toggle(move |v| Message::PropertyChanged(widget_id, PropertyChange::Clip(v))),
            information(theme, "When enabled, child content that exceeds bounds will be clipped"),
        ]
        .spacing(LABEL_SPACING)
        .align_y(Alignment::End)
        .into()
}

pub fn max_width_control<'a>(widget_id: WidgetId, max_width: Option<f32>) -> Element<'a, Message> {
        column![
            row![
                checkbox(max_width.is_some())
                    .label("Set max width")
                    .on_toggle(move |enabled| Message::PropertyChanged(widget_id, PropertyChange::MaxWidth(if enabled { Some(800.0) } else { None }))),
                if let Some(max_w) = max_width {
                    row![
                        slider(100.0..=2000.0, max_w, move |v| {
                            Message::PropertyChanged(widget_id, PropertyChange::MaxWidth(Some(v)))
                        })
                        .step(10.0)
                        .width(200),
                        text(format!("{:.0}px", max_w)).size(LABEL_SIZE).width(60),
                    ]
                    .spacing(SECTION_SPACING)
                    .align_y(Alignment::Center)
                } else {
                    row![]
                }
            ]
            .spacing(SECTION_SPACING)
            .align_y(Alignment::Center),
        ]
        .spacing(LABEL_SPACING)
        .into()   
}

pub fn max_height_control<'a>(widget_id: WidgetId, max_height: Option<f32>) -> Element<'a, Message> {
        column![
            row![
                checkbox(max_height.is_some())
                    .label("Set max height")
                    .on_toggle(move |enabled| Message::PropertyChanged(widget_id, PropertyChange::MaxHeight(if enabled { Some(800.0) } else { None }))),
                if let Some(max_h) = max_height {
                    row![
                        slider(100.0..=2000.0, max_h, move |v| {
                            Message::PropertyChanged(widget_id, PropertyChange::MaxHeight(Some(v)))
                        })
                        .step(10.0)
                        .width(200),
                        text(format!("{:.0}px", max_h)).size(LABEL_SIZE).width(60),
                    ]
                    .spacing(SECTION_SPACING)
                    .align_y(Alignment::Center)
                } else {
                    row![].into()
                }
            ]
            .spacing(SECTION_SPACING)
            .align_y(Alignment::Center),
        ]
        .spacing(LABEL_SPACING)
        .into()
}

pub fn widget_id_control<'a>(widget_id: WidgetId, id: Option<String>, theme: &Theme ) -> Element<'a, Message> {
    let id_clone = id.clone();

    row![
        row![
            checkbox(id_clone.is_some())
                .label("Set Widget Id")
                .on_toggle(move |enabled| Message::PropertyChanged(widget_id, PropertyChange::WidgetId(if enabled { Some(String::new()) } else { None }))),
            information(theme, "Use for programmatic access via widget::Id"),
        ]        
        .spacing(LABEL_SPACING)
        .align_y(Alignment::End),

        if let Some(ref id_val) = id {
            row![
                text_input("widget_id", *&id_val)
                    .on_input(move |v| {
                        Message::PropertyChanged(widget_id, PropertyChange::WidgetId(Some(v)))
                    })
                    .width(200)
            ]

        } else {
            row![]
        }
    ]
    .spacing(SECTION_SPACING)
    .align_y(Alignment::Center)
    .into()
}

pub fn widget_name<'a>(widget_id: WidgetId, name: &'a str) -> Element<'a, Message> {
        column![
            text("Widget Name").size(LABEL_SIZE),
            text_input("Name", name)
                .on_input(move |v| Message::PropertyChanged(widget_id, PropertyChange::WidgetName(v)))
                .width(250),
        ]
        .spacing(LABEL_SPACING)
        .into()
}

fn custom_style_picker<'a>(
    custom_themes: &'a CustomThemes,
    widget_id: WidgetId,
    props: &'a Properties,
    widget_type_enum: ThemePaneEnum,
) -> Element<'a, Message> {
    let custom_styles = custom_themes.styles().get(&widget_type_enum);

    let mut style_options: Vec<String> = vec![];
    if let Some(styles) = custom_styles {
        if !styles.is_empty() {
             style_options.extend(styles.keys().map(|f| f.to_string()));
        }
    }

    match widget_type_enum {
        ThemePaneEnum::Container => {
            for name in ContainerStyleType::all() {
                style_options.push(name);
            }
        }
        ThemePaneEnum::Button => {
            for name in ButtonStyleType::all() {
                style_options.push(name);
            }
        }
        _ => {}
    }
      
    if style_options.len() < 1 { // Don't show picker if there are no styles
        return column![].into();
    }

    column![
        text("Style").size(LABEL_SIZE),
        pick_list(
            style_options,
            props.custom_style_name.clone(),
            move |selection| {
                Message::PropertyChanged(widget_id, PropertyChange::CustomStyle(Some(selection.to_string())))
            }
        )
        .placeholder("default")
    ]
    .spacing(LABEL_SPACING)
    .into()
}

pub fn add_code_preview<'a>(content: Element<'a, Message>, hierarchy: &'a WidgetHierarchy, widget_id: WidgetId, theme: &Theme, type_system: &'a TypeSystem, custom_styles: &CustomThemes,) -> Element<'a, Message> {
    let mut generator = CodeGenerator::new_single(hierarchy, theme, type_system);
    let tokens = generator.generate_widget_code_rewrite(widget_id, custom_styles);
    
    column![
        column![
            scrollable(
                content,
            ),
        ]
        .height(400),

        container(rule::horizontal(2)).padding(padding::right(10)),
        text("Widget Code").size(16),

        scrollable(
            container(
                build_code_view_with_height(&tokens, 0.0, theme)
            )
            .padding(padding::right(10))
        ),
    ]
    .padding(5)
    .spacing(10)
    .into()
}


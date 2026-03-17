pub mod batch_edit;
pub mod control_styling;
pub mod single_edit;

use crate::action_system::custom_state::CustomFieldType;
use crate::data_structures::properties::messages::*;
use crate::data_structures::properties::properties::*;
use crate::data_structures::types::type_implementations::*;
use crate::data_structures::types::types::*;
use crate::data_structures::widget_hierarchy::WidgetHierarchy;
use crate::enum_builder::TypeSystem;
use crate::icon;
use crate::icon_lucide;
use iced::widget::{
    Row, Space, button, checkbox, column, container, pick_list, radio, row, rule, scrollable,
    slider, space, text, text_editor, text_input, toggler, tooltip,
};
use iced::{Alignment, Color, Element, Length, Padding, Theme, padding};
use std::collections::BTreeMap;
use uuid::Uuid;
use widgets::generic_overlay::overlay_button;

use crate::views::theme_and_stylefn_builder::{CustomThemes, ThemePaneEnum};
use crate::views::widget_tree::Message;
use control_styling::*;

pub fn container_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    theme: &Theme,
    type_system: &'a TypeSystem,
    custom_styles: &'a CustomThemes,
    preview_content: &'a text_editor::Content,
    view_id: Uuid,
    views: &'a BTreeMap<Uuid, AppView>,
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
        ]
        .spacing(MAIN_SPACING),
        conditional_style_section(
            widget_id,
            props,
            theme,
            view_id,
            h.root(),
            views,
            type_system
        ),
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
                        move |l| Message::PropertyChanged(
                            widget_id,
                            PropertyChange::ContainerCenterLength(l)
                        ),
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
                        move |l| Message::PropertyChanged(
                            widget_id,
                            PropertyChange::ContainerCenterLength(l)
                        ),
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
                column![length_picker_scrollable_aware(
                    "Size (centers content in both directions)",
                    props.container_center_length,
                    move |l| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::ContainerCenterLength(l)
                    ),
                    h,
                    widget_id,
                    false
                ),]
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
                        vec![
                            ContainerAlignX::Left,
                            ContainerAlignX::Center,
                            ContainerAlignX::Right
                        ],
                        Some(props.align_x),
                        move |v| Message::PropertyChanged(widget_id, PropertyChange::AlignX(v)),
                    )
                    .width(160),
                ]
                .spacing(LABEL_SPACING)
                .width(Length::Fill),
                column![
                    text("Vertical Align").size(LABEL_SIZE),
                    pick_list(
                        vec![
                            ContainerAlignY::Top,
                            ContainerAlignY::Center,
                            ContainerAlignY::Bottom
                        ],
                        Some(props.align_y),
                        move |v| Message::PropertyChanged(widget_id, PropertyChange::AlignY(v)),
                    )
                    .width(160),
                ]
                .spacing(LABEL_SPACING)
                .width(Length::Fill),
            ]
            .spacing(MAIN_SPACING)
        } else {
            row![]
        },
        // Padding Controls
        padding_controls(props.padding, widget_id, props.padding_mode, theme,),
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

    scrollable(container(add_code_preview(content, preview_content)).padding(padding::right(10.0)))
        .into()
}

pub fn row_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    theme: &Theme,
    _type_system: &'a TypeSystem,
    _custom_styles: &CustomThemes,
    preview_content: &'a text_editor::Content,
) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    let content = column![
        text("Row Properties").size(TITLE_SIZE),
        widget_name(widget_id, &props.widget_name),
        column![
            text("Layout Mode").size(SECTION_SIZE),
            row![
                toggler(props.is_wrapping_row,).on_toggle(move |v| Message::PropertyChanged(
                    widget_id,
                    PropertyChange::IsWrappingRow(v)
                )),
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
                text(format!("{:.0}px", props.spacing))
                    .size(LABEL_SIZE)
                    .width(50),
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
                        checkbox(props.match_horizontal_spacing,)
                            .label("match horizontal spacing")
                            .on_toggle(move |use_same| {
                                Message::PropertyChanged(
                                    widget_id,
                                    PropertyChange::WrappingSpacingMatchToggle(use_same),
                                )
                            }),
                        if props.match_horizontal_spacing {
                            row![
                                slider(0.0..=50.0, props.wrapping_vertical_spacing, move |v| {
                                    Message::PropertyChanged(
                                        widget_id,
                                        PropertyChange::WrappingVerticalSpacing(v),
                                    )
                                })
                                .step(1.0)
                                .width(180),
                                text(format!("{:.0}px", props.wrapping_vertical_spacing))
                                    .size(LABEL_SIZE)
                                    .width(50),
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
                    )
                    .width(160),
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
                        vec![
                            AlignmentXOption::Start,
                            AlignmentXOption::Center,
                            AlignmentXOption::End
                        ],
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
        padding_controls(props.padding, widget_id, props.padding_mode, theme,),
        clip_control(widget_id, props.clip, theme),
    ]
    .spacing(MAIN_SPACING)
    .into();

    scrollable(container(add_code_preview(content, preview_content)).padding(padding::right(10.0)))
        .into()
}

pub fn column_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    theme: &Theme,
    _type_system: &'a TypeSystem,
    _custom_styles: &CustomThemes,
    preview_content: &'a text_editor::Content,
) -> Element<'a, Message> {
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
                text(format!("{:.0}px", props.spacing))
                    .size(LABEL_SIZE)
                    .width(50),
            ]
            .spacing(SECTION_SPACING)
            .align_y(Alignment::Center),
        ]
        .spacing(LABEL_SPACING),
        column![
            text("Horizontal Alignment").size(LABEL_SIZE),
            pick_list(
                vec![
                    AlignmentXOption::Start,
                    AlignmentXOption::Center,
                    AlignmentXOption::End
                ],
                Some(AlignmentXOption::from_alignment(props.align_items)),
                move |sel| Message::PropertyChanged(
                    widget_id,
                    PropertyChange::AlignItems(sel.to_alignment())
                ),
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
        padding_controls(props.padding, widget_id, props.padding_mode, theme,),
        // Max Width control
        max_width_control(widget_id, props.max_width),
        //Clip control
        clip_control(widget_id, props.clip, theme),
    ]
    .spacing(MAIN_SPACING)
    .into();

    scrollable(container(add_code_preview(content, preview_content)).padding(padding::right(10.0)))
        .into()
}

pub fn button_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    theme: &Theme,
    type_system: &'a TypeSystem,
    custom_styles: &'a CustomThemes,
    preview_content: &'a text_editor::Content,
    views: &'a BTreeMap<Uuid, AppView>,
    view_id: Uuid,
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

    let has_child = !widget.children.is_empty();

    let content =
        column![
        text("Button Properties").size(TITLE_SIZE),
        widget_name(widget_id, &props.widget_name),
        column![
            text("Button Text").size(LABEL_SIZE),
            text_input("Text", &props.text_content)
                .on_input(move |v| Message::PropertyChanged(
                    widget_id,
                    PropertyChange::TextContent(v)
                ))
                .width(250),
            if has_child {
                text("Ignored — button is using its child widget as content.")
                    .size(LABEL_SIZE - 2.0)
                    .color(palette.primary.base.color)
            } else {
                text("")
            },
        ]
        .spacing(LABEL_SPACING),
        custom_style_picker(custom_styles, widget_id, props, ThemePaneEnum::Button),
        conditional_style_section(widget_id, props, theme, view_id, h.root(), views, type_system),
        column![
            text("Events").size(SECTION_SIZE),
            with_event_info(
                radio(
                    "None (button disabled)",
                    3,
                    Some(selected_handler),
                    move |_| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::ButtonPressHandler(OnHandler::None)
                    )
                )
                .into(),
                theme,
                "Button will not respond to clicks",
            ),
            with_event_info(
                radio("on_press", 0, Some(selected_handler), move |_| {
                    Message::PropertyChanged(
                        widget_id,
                        PropertyChange::ButtonPressHandler(OnHandler::OnAction),
                    )
                })
                .into(),
                theme,
                "Direct message dispatch - use when the action is always the same",
            ),
            with_event_info(
                radio("on_press_with", 1, Some(selected_handler), move |_| {
                    Message::PropertyChanged(
                        widget_id,
                        PropertyChange::ButtonPressHandler(OnHandler::OnActionWith),
                    )
                })
                .into(),
                theme,
                "Closure returns the action - use when the action depends on runtime data",
            ),
            with_event_info(
                radio("on_press_maybe", 2, Some(selected_handler), move |_| {
                    Message::PropertyChanged(
                        widget_id,
                        PropertyChange::ButtonPressHandler(OnHandler::OnActionMaybe),
                    )
                })
                .into(),
                theme,
                "Optional action - use when the button should only be enabled in certain states",
            ),
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
        padding_controls(props.padding, widget_id, props.padding_mode, theme,),
        clip_control(widget_id, props.clip, theme),
    ]
        .spacing(MAIN_SPACING)
        .into();

    scrollable(container(add_code_preview(content, preview_content)).padding(padding::right(10.0)))
        .into()
}

pub fn text_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    _theme: &Theme,
    _type_system: &'a TypeSystem,
    _custom_styles: &CustomThemes,
    preview_content: &'a text_editor::Content,
) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    let content = column![
        text("Text Properties").size(TITLE_SIZE),
        widget_name(widget_id, &props.widget_name),
        column![
            text("Text Content").size(LABEL_SIZE),
            text_input("Content", &props.text_content)
                .on_input(move |v| Message::PropertyChanged(
                    widget_id,
                    PropertyChange::TextContent(v)
                ))
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
                text(format!("{:.0}px", props.text_size))
                    .size(LABEL_SIZE)
                    .width(50),
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
        color_hex_input("Text Color", &props.draft_text_color, move |c| {
            Message::PropertyChanged(widget_id, PropertyChange::DraftTextColor(c))
        }),
        column![
            text("Wrapping").size(LABEL_SIZE),
            pick_list(
                vec![
                    TextWrapping::None,
                    TextWrapping::Word,
                    TextWrapping::Glyph,
                    TextWrapping::WordOrGlyph
                ],
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
                slider(
                    0.8..=2.0,
                    match props.line_height {
                        text::LineHeight::Relative(v) => v,
                        _ => 1.0,
                    },
                    move |v| {
                        Message::PropertyChanged(
                            widget_id,
                            PropertyChange::TextLineHeight(text::LineHeight::Relative(
                                (v * 100.0).round() / 100.0,
                            )),
                        )
                    }
                )
                .step(0.05)
                .width(220),
                text(match props.line_height {
                    text::LineHeight::Relative(v) => format!("{:.2}", v),
                    _ => "1.00".into(),
                })
                .size(LABEL_SIZE)
            ]
            .spacing(SECTION_SPACING)
            .align_y(Alignment::Center)
        ]
        .spacing(LABEL_SPACING),
        row![
            column![
                text("Align X").size(LABEL_SIZE),
                pick_list(
                    vec![
                        AlignText::Default,
                        AlignText::Left,
                        AlignText::Center,
                        AlignText::Right,
                        AlignText::Justified
                    ],
                    Some(AlignText::from(props.text_align_x)),
                    move |a| Message::PropertyChanged(widget_id, PropertyChange::TextAlignX(a))
                )
            ]
            .spacing(LABEL_SPACING)
            .width(Length::Fill),
            column![
                text("Align Y").size(LABEL_SIZE),
                pick_list(
                    vec![
                        AlignmentYOption::Top,
                        AlignmentYOption::Center,
                        AlignmentYOption::Bottom
                    ],
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

    scrollable(container(add_code_preview(content, preview_content)).padding(padding::right(10.0)))
        .into()
}

pub fn text_input_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    theme: &Theme,
    _type_system: &'a TypeSystem,
    custom_styles: &'a CustomThemes,
    preview_content: &'a text_editor::Content,
) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    let content = column![
        text("Text Input Properties").size(TITLE_SIZE),
        widget_name(widget_id, &props.widget_name),
        named_style_picker(
            custom_styles,
            widget_id,
            "Style",
            props.custom_style_name.clone(),
            ThemePaneEnum::TextInput,
            PropertyChange::CustomStyle,
        ),
        column![
            text("Placeholder Text").size(LABEL_SIZE),
            text_input("Placeholder", &props.text_input_placeholder)
                .on_input(move |v| Message::PropertyChanged(
                    widget_id,
                    PropertyChange::TextInputPlaceholder(v)
                ))
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
                text(format!("{:.0}px", props.text_input_size))
                    .size(LABEL_SIZE)
                    .width(50),
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
                text(format!("{:.0}px", props.text_input_padding))
                    .size(LABEL_SIZE)
                    .width(50),
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
                move |v| Message::PropertyChanged(
                    widget_id,
                    PropertyChange::TextInputFont(v.into())
                )
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
                .on_toggle(move |v| Message::PropertyChanged(
                    widget_id,
                    PropertyChange::IsSecure(v)
                )),
        ]
        .spacing(SECTION_SPACING),
        column![
            text("Events").size(SECTION_SIZE),
            event_name_row(theme, "on_input", "Fires whenever the text value changes"),
            event_checkbox_row(
                theme,
                props.text_input_on_submit,
                "on_submit",
                "Fires when the user submits the field, usually by pressing Enter",
                move |v| Message::PropertyChanged(widget_id, PropertyChange::TextInputOnSubmit(v)),
            ),
            event_checkbox_row(
                theme,
                props.text_input_on_paste,
                "on_paste",
                "Fires when text is pasted into the field",
                move |v| Message::PropertyChanged(widget_id, PropertyChange::TextInputOnPaste(v)),
            ),
        ]
        .spacing(SECTION_SPACING),
        {
            // Icon section
            let filter = props.text_input_icon_picker_filter.to_lowercase();
            let icon_buttons: Vec<Element<'a, Message>> = icon_lucide::ALL_ICONS
                .iter()
                .filter(|(name, _)| filter.is_empty() || name.contains(filter.as_str()))
                .map(|(name, codepoint)| {
                    let cp_u32 = codepoint.chars().next().map(|c| c as u32).unwrap_or(0xFFFD);
                    let name_clone = name.to_string();
                    tooltip(
                        button(icon_lucide::render(codepoint))
                            .on_press(Message::PropertyChanged(
                                widget_id,
                                PropertyChange::TextInputIconSelected(name_clone, cp_u32),
                            ))
                            .style(button::text),
                        container(text(*name))
                            .style(container::bordered_box)
                            .padding(5),
                        tooltip::Position::Top,
                    )
                    .into()
                })
                .collect();

            let search = text_input("Search icons…", &props.text_input_icon_picker_filter)
                .on_input(move |v| {
                    Message::PropertyChanged(
                        widget_id,
                        PropertyChange::TextInputIconPickerFilter(v),
                    )
                });

            let picker_content = column![
                container(search).padding(padding::horizontal(5.0)),
                scrollable(
                    Row::with_children(icon_buttons)
                        .spacing(4)
                        .padding(10)
                        .wrap(),
                ),
            ]
            .spacing(4);

            let current_cp_str: &'static str = icon_lucide::ALL_ICONS
                .iter()
                .find(|(name, _)| *name == props.text_input_icon_name.as_str())
                .map(|(_, cp)| *cp)
                .unwrap_or("\u{FFFD}");

            let trigger = row![
                icon_lucide::render(current_cp_str).size(20),
                text(&props.text_input_icon_name),
            ]
            .spacing(6)
            .align_y(Alignment::Center);

            let picker = overlay_button(trigger, "Select Icon", picker_content)
                .overlay_width(650.0)
                .overlay_height(450.0)
                .hide_header()
                .close_on_click_outside()
                .hover_positions_on_click()
                .hover_position(widgets::generic_overlay::Position::Right)
                .hover_gap(5.0)
                .hover_alignment(Alignment::Start);

            column![
                text("Icon").size(SECTION_SIZE),
                checkbox(props.text_input_icon_enabled)
                    .label("Enable Icon")
                    .on_toggle(move |v| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::TextInputIconEnabled(v)
                    )),
                if props.text_input_icon_enabled {
                    column![
                        column![text("Icon").size(LABEL_SIZE), picker,].spacing(LABEL_SPACING),
                        column![
                            text("Side").size(LABEL_SIZE),
                            pick_list(
                                vec![TextInputIconSide::Left, TextInputIconSide::Right],
                                Some(props.text_input_icon_side),
                                move |v| Message::PropertyChanged(
                                    widget_id,
                                    PropertyChange::TextInputIconSide(v)
                                )
                            ),
                        ]
                        .spacing(LABEL_SPACING),
                        column![
                            text("Icon Size (0 = auto)").size(LABEL_SIZE),
                            row![
                                slider(0.0..=64.0, props.text_input_icon_size, move |v| {
                                    Message::PropertyChanged(
                                        widget_id,
                                        PropertyChange::TextInputIconSize(v),
                                    )
                                })
                                .step(1.0)
                                .width(180),
                                text(if props.text_input_icon_size > 0.0 {
                                    format!("{:.0}px", props.text_input_icon_size)
                                } else {
                                    "auto".to_string()
                                })
                                .size(LABEL_SIZE)
                                .width(50),
                            ]
                            .spacing(SECTION_SPACING)
                            .align_y(Alignment::Center),
                        ]
                        .spacing(LABEL_SPACING),
                        column![
                            text("Spacing").size(LABEL_SIZE),
                            row![
                                slider(0.0..=30.0, props.text_input_icon_spacing, move |v| {
                                    Message::PropertyChanged(
                                        widget_id,
                                        PropertyChange::TextInputIconSpacing(v),
                                    )
                                })
                                .step(1.0)
                                .width(180),
                                text(format!("{:.0}px", props.text_input_icon_spacing))
                                    .size(LABEL_SIZE)
                                    .width(50),
                            ]
                            .spacing(SECTION_SPACING)
                            .align_y(Alignment::Center),
                        ]
                        .spacing(LABEL_SPACING),
                    ]
                    .spacing(SECTION_SPACING)
                } else {
                    column![]
                },
            ]
            .spacing(SECTION_SPACING)
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

    scrollable(container(add_code_preview(content, preview_content)).padding(padding::right(10.0)))
        .into()
}

pub fn checkbox_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    theme: &Theme,
    type_system: &'a TypeSystem,
    custom_styles: &'a CustomThemes,
    preview_content: &'a text_editor::Content,
    views: &'a BTreeMap<Uuid, AppView>,
    view_id: Uuid,
) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    let content = column![
        text("Checkbox Properties").size(TITLE_SIZE),
        widget_name(widget_id, &props.widget_name),
        custom_style_picker(custom_styles, widget_id, props, ThemePaneEnum::Checkbox),
        conditional_style_section(
            widget_id,
            props,
            theme,
            view_id,
            h.root(),
            views,
            type_system
        ),
        column![
            text("Label Text").size(LABEL_SIZE),
            text_input("Label", &props.checkbox_label)
                .on_input(move |v| Message::PropertyChanged(
                    widget_id,
                    PropertyChange::CheckboxLabel(v)
                ))
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
                text(format!("{:.0}px", props.checkbox_size))
                    .size(LABEL_SIZE)
                    .width(50),
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
                text(format!("{:.0}px", props.checkbox_spacing))
                    .size(LABEL_SIZE)
                    .width(50),
            ]
            .spacing(SECTION_SPACING)
            .align_y(Alignment::Center),
        ]
        .spacing(LABEL_SPACING),
        checkbox(props.checkbox_checked)
            .label("Default Checked State")
            .on_toggle(move |v| Message::PropertyChanged(
                widget_id,
                PropertyChange::CheckboxChecked(v)
            )),
        column![
            text("Events").size(SECTION_SIZE),
            event_name_row(
                theme,
                "on_toggle",
                "Fires when the checkbox checked state changes",
            ),
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

    scrollable(container(add_code_preview(content, preview_content)).padding(padding::right(10.0)))
        .into()
}

pub fn toggler_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    theme: &Theme,
    type_system: &'a TypeSystem,
    custom_styles: &'a CustomThemes,
    preview_content: &'a text_editor::Content,
    views: &'a BTreeMap<Uuid, AppView>,
    view_id: Uuid,
) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    let content = column![
        text("Toggler Properties").size(TITLE_SIZE),
        widget_name(widget_id, &props.widget_name),
        custom_style_picker(custom_styles, widget_id, props, ThemePaneEnum::Toggler),
        conditional_style_section(
            widget_id,
            props,
            theme,
            view_id,
            h.root(),
            views,
            type_system
        ),
        column![
            text("Label Text").size(LABEL_SIZE),
            text_input("Label", &props.toggler_label)
                .on_input(move |v| Message::PropertyChanged(
                    widget_id,
                    PropertyChange::TogglerLabel(v)
                ))
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
                text(format!("{:.0}px", props.toggler_size))
                    .size(LABEL_SIZE)
                    .width(50),
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
                text(format!("{:.0}px", props.toggler_spacing))
                    .size(LABEL_SIZE)
                    .width(50),
            ]
            .spacing(SECTION_SPACING)
            .align_y(Alignment::Center),
        ]
        .spacing(LABEL_SPACING),
        checkbox(props.toggler_active)
            .label("Default Active State")
            .on_toggle(move |v| Message::PropertyChanged(
                widget_id,
                PropertyChange::TogglerActive(v)
            )),
        column![
            text("Events").size(SECTION_SIZE),
            event_name_row(
                theme,
                "on_toggle",
                "Fires when the toggler switches on or off",
            ),
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

    scrollable(container(add_code_preview(content, preview_content)).padding(padding::right(10.0)))
        .into()
}

pub fn radio_controls<'a>(
    hierarchy: &'a WidgetHierarchy,
    widget_id: WidgetId,
    theme: &Theme,
    type_system: &'a TypeSystem,
    custom_styles: &'a CustomThemes,
    preview_content: &'a text_editor::Content,
    views: &'a BTreeMap<Uuid, AppView>,
    view_id: Uuid,
) -> Element<'a, Message> {
    let widget = hierarchy.get_widget_by_id(widget_id).unwrap();
    let props = &widget.properties;

    let content = column![
        text("Radio Button Properties").size(TITLE_SIZE),
        widget_name(widget_id, &props.widget_name),
        custom_style_picker(custom_styles, widget_id, props, ThemePaneEnum::Radio),
        conditional_style_section(
            widget_id,
            props,
            theme,
            view_id,
            hierarchy.root(),
            views,
            type_system,
        ),
        column![
            text("Label Text").size(LABEL_SIZE),
            text_input("Label", &props.radio_label)
                .on_input(move |v| Message::PropertyChanged(
                    widget_id,
                    PropertyChange::RadioLabel(v)
                ))
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
                    text(format!("{:.0}px", props.radio_size))
                        .size(LABEL_SIZE)
                        .width(50),
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
                    text(format!("{:.0}px", props.radio_spacing))
                        .size(LABEL_SIZE)
                        .width(50),
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
                props
                    .radio_options
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
                                        if index < next.len() {
                                            next[index] = v;
                                        }
                                        Message::PropertyChanged(
                                            widget_id,
                                            PropertyChange::RadioOptions(next),
                                        )
                                    }
                                })
                                .width(220),
                            button("Remove").on_press({
                                let index = i;
                                let mut next = props.radio_options.clone();
                                if index < next.len() && next.len() > 1 {
                                    next.remove(index);
                                }
                                Message::PropertyChanged(
                                    widget_id,
                                    PropertyChange::RadioOptions(next),
                                )
                            }),
                        ]
                        .spacing(SECTION_SPACING)
                        .align_y(Alignment::Center)
                        .into()
                    })
                    .collect::<Vec<Element<'a, Message>>>()
            )
            .spacing(LABEL_SPACING),
            button("Add Option").on_press({
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

    scrollable(container(add_code_preview(content, preview_content)).padding(padding::right(10.0)))
        .into()
}

pub fn picklist_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    theme: &Theme,
    type_system: &'a TypeSystem,
    custom_styles: &'a CustomThemes,
    preview_content: &'a text_editor::Content,
    views: &'a BTreeMap<Uuid, AppView>,
    view_id: Uuid,
) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    let content = column![
        text("Pick List Properties").size(TITLE_SIZE),
        widget_name(widget_id, &props.widget_name),
        custom_style_picker(custom_styles, widget_id, props, ThemePaneEnum::Picklist),
        conditional_style_section(
            widget_id,
            props,
            theme,
            view_id,
            h.root(),
            views,
            type_system
        ),
        named_style_picker(
            custom_styles,
            widget_id,
            "Menu Style",
            props.menu_style_name.clone(),
            ThemePaneEnum::Menu,
            PropertyChange::MenuStyle,
        ),
        column![
            text("Placeholder Text").size(LABEL_SIZE),
            text_input("Placeholder", &props.picklist_placeholder)
                .on_input(move |v| Message::PropertyChanged(
                    widget_id,
                    PropertyChange::PickListPlaceholder(v)
                ))
                .width(250),
        ]
        .spacing(LABEL_SPACING),
        column![
            text("Default Selection").size(LABEL_SIZE),
            pick_list(
                props.picklist_options.clone(),
                props.picklist_selected.clone(),
                move |selection| Message::PropertyChanged(
                    widget_id,
                    PropertyChange::PickListSelected(Some(selection))
                )
            ),
        ]
        .spacing(LABEL_SPACING),
        column![
            text("Options").size(SECTION_SIZE),
            column(
                props
                    .picklist_options
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
                                        Message::PropertyChanged(
                                            widget_id,
                                            PropertyChange::PickListOptions(new_options),
                                        )
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
                                    Message::PropertyChanged(
                                        widget_id,
                                        PropertyChange::PickListOptions(new_options),
                                    )
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
                    Message::PropertyChanged(
                        widget_id,
                        PropertyChange::PickListOptions(new_options),
                    )
                })
                .style(button::success)
                .padding(Padding::new(5.0)),
        ]
        .spacing(SECTION_SPACING),
        column![
            text("Events").size(SECTION_SIZE),
            event_name_row(theme, "on_select", "Fires when the user picks an option"),
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

    scrollable(container(add_code_preview(content, preview_content)).padding(padding::right(10.0)))
        .into()
}

pub fn slider_controls<'a>(
    hierarchy: &'a WidgetHierarchy,
    widget_id: WidgetId,
    theme: &Theme,
    type_system: &'a TypeSystem,
    custom_styles: &'a CustomThemes,
    preview_content: &'a text_editor::Content,
    views: &'a BTreeMap<Uuid, AppView>,
    view_id: Uuid,
) -> Element<'a, Message> {
    let widget = hierarchy.get_widget_by_id(widget_id).unwrap();
    let props = &widget.properties;

    let min_str = format!("{:.3}", props.slider_min);
    let max_str = format!("{:.3}", props.slider_max);
    let step_str = format!("{:.3}", props.slider_step);
    let slider_height = format!("{:.0}", props.slider_height);

    let content = column![
        text("Slider Properties").size(TITLE_SIZE),
        widget_name(widget_id, &props.widget_name),
        custom_style_picker(custom_styles, widget_id, props, ThemePaneEnum::Slider),
        conditional_style_section(
            widget_id,
            props,
            theme,
            view_id,
            hierarchy.root(),
            views,
            type_system,
        ),
        row![
            column![
                text("Min").size(LABEL_SIZE),
                text_input("min", &min_str)
                    .on_input(move |s| {
                        let v = parse_f32(&s, props.slider_min);
                        Message::PropertyChanged(widget_id, PropertyChange::SliderMin(v))
                    })
                    .width(120),
            ]
            .spacing(LABEL_SPACING),
            column![
                text("Max").size(LABEL_SIZE),
                text_input("max", &max_str)
                    .on_input(move |s| {
                        let v = parse_f32(&s, props.slider_max);
                        Message::PropertyChanged(widget_id, PropertyChange::SliderMax(v))
                    })
                    .width(120),
            ]
            .spacing(LABEL_SPACING),
            column![
                text("Step").size(LABEL_SIZE),
                text_input("step", &step_str)
                    .on_input(move |s| {
                        let v = parse_f32(&s, props.slider_step.max(0.000_001));
                        Message::PropertyChanged(
                            widget_id,
                            PropertyChange::SliderStep(v.max(0.000_001)),
                        )
                    })
                    .width(120),
            ]
            .spacing(LABEL_SPACING),
        ]
        .spacing(SECTION_SPACING),
        column![
            text("Value").size(LABEL_SIZE),
            row![
                slider(
                    props.slider_min..=props.slider_max,
                    props.slider_value,
                    move |val| {
                        Message::PropertyChanged(widget_id, PropertyChange::SliderValue(val))
                    }
                )
                .step(props.slider_step.max(0.000_001))
                .width(300),
                text(format!("{:.3}", props.slider_value)).size(LABEL_SIZE),
            ]
            .spacing(SECTION_SPACING)
            .align_y(Alignment::Center),
        ]
        .spacing(LABEL_SPACING),
        column![
            text("Events").size(SECTION_SIZE),
            event_name_row(
                theme,
                "on_change",
                "Fires whenever the slider value changes"
            ),
        ]
        .spacing(SECTION_SPACING),
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
                text_input("px", &slider_height)
                    .on_input(move |s| {
                        Message::PropertyChanged(
                            widget_id,
                            PropertyChange::SliderHeight(parse_f32(&s, props.slider_height)),
                        )
                    })
                    .width(120)
            ]
            .spacing(LABEL_SPACING),
        ]
        .spacing(SECTION_SPACING),
    ]
    .spacing(MAIN_SPACING)
    .into();

    scrollable(container(add_code_preview(content, preview_content)).padding(padding::right(10.0)))
        .into()
}

pub fn vertical_slider_controls<'a>(
    hierarchy: &'a WidgetHierarchy,
    widget_id: WidgetId,
    theme: &Theme,
    type_system: &'a TypeSystem,
    custom_styles: &'a CustomThemes,
    preview_content: &'a text_editor::Content,
    views: &'a BTreeMap<Uuid, AppView>,
    view_id: Uuid,
) -> Element<'a, Message> {
    let widget = hierarchy.get_widget_by_id(widget_id).unwrap();
    let props = &widget.properties;

    let min_str = format!("{:.3}", props.slider_min);
    let max_str = format!("{:.3}", props.slider_max);
    let step_str = format!("{:.3}", props.slider_step);
    let slider_width = format!("{:.0}", props.slider_width);

    let content = column![
        text("Vertical Slider Properties").size(TITLE_SIZE),
        widget_name(widget_id, &props.widget_name),
        custom_style_picker(custom_styles, widget_id, props, ThemePaneEnum::Slider),
        conditional_style_section(
            widget_id,
            props,
            theme,
            view_id,
            hierarchy.root(),
            views,
            type_system,
        ),
        row![
            column![
                text("Min").size(LABEL_SIZE),
                text_input("min", &min_str)
                    .on_input(move |s| {
                        let v = parse_f32(&s, props.slider_min);
                        Message::PropertyChanged(widget_id, PropertyChange::SliderMin(v))
                    })
                    .width(120),
            ]
            .spacing(LABEL_SPACING),
            column![
                text("Max").size(LABEL_SIZE),
                text_input("max", &max_str)
                    .on_input(move |s| {
                        let v = parse_f32(&s, props.slider_max);
                        Message::PropertyChanged(widget_id, PropertyChange::SliderMax(v))
                    })
                    .width(120),
            ]
            .spacing(LABEL_SPACING),
            column![
                text("Step").size(LABEL_SIZE),
                text_input("step", &step_str)
                    .on_input(move |s| {
                        let v = parse_f32(&s, props.slider_step.max(0.000_001));
                        Message::PropertyChanged(
                            widget_id,
                            PropertyChange::SliderStep(v.max(0.000_001)),
                        )
                    })
                    .width(120),
            ]
            .spacing(LABEL_SPACING),
        ]
        .spacing(SECTION_SPACING),
        column![
            text("Value").size(LABEL_SIZE),
            row![
                slider(
                    props.slider_min..=props.slider_max,
                    props.slider_value,
                    move |val| {
                        Message::PropertyChanged(widget_id, PropertyChange::SliderValue(val))
                    }
                )
                .step(props.slider_step.max(0.000_001))
                .width(300),
                text(format!("{:.3}", props.slider_value)).size(LABEL_SIZE),
            ]
            .spacing(SECTION_SPACING)
            .align_y(Alignment::Center),
        ]
        .spacing(LABEL_SPACING),
        column![
            text("Events").size(SECTION_SIZE),
            event_name_row(
                theme,
                "on_change",
                "Fires whenever the slider value changes"
            ),
        ]
        .spacing(SECTION_SPACING),
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
                text_input("px", &slider_width)
                    .on_input(move |s| {
                        Message::PropertyChanged(
                            widget_id,
                            PropertyChange::SliderWidth(parse_f32(&s, props.slider_width)),
                        )
                    })
                    .width(120)
            ]
            .spacing(LABEL_SPACING),
        ]
        .spacing(SECTION_SPACING),
    ]
    .spacing(MAIN_SPACING)
    .into();

    scrollable(container(add_code_preview(content, preview_content)).padding(padding::right(10.0)))
        .into()
}

pub fn rule_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    _theme: &Theme,
    _type_system: &'a TypeSystem,
    custom_styles: &'a CustomThemes,
    preview_content: &'a text_editor::Content,
) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).unwrap();
    let p = &widget.properties;

    let content = column![
        text("Rule Properties").size(TITLE_SIZE),
        custom_style_picker(custom_styles, widget_id, p, ThemePaneEnum::Rule),
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
                slider(0.5..=20.0, p.rule_thickness as f32, move |v| {
                    Message::PropertyChanged(widget_id, PropertyChange::RuleThickness(v))
                })
                .step(0.5)
                .width(200),
                text(format!("{:.1}px", p.rule_thickness))
                    .size(LABEL_SIZE)
                    .width(50),
            ]
            .spacing(SECTION_SPACING)
            .align_y(Alignment::Center),
        ]
        .spacing(LABEL_SPACING),
        column![
            text("Quick Presets").size(LABEL_SIZE),
            row([1.0_f32, 2.0, 3.0, 4.0, 6.0, 8.0, 12.0]
                .into_iter()
                .map(|px| {
                    button(text(format!("{px}px")))
                        .on_press(Message::PropertyChanged(
                            widget_id,
                            PropertyChange::RuleThickness(px),
                        ))
                        .padding(6)
                        .into()
                })
                .collect::<Vec<_>>())
            .spacing(LABEL_SPACING)
        ]
        .spacing(LABEL_SPACING),
    ]
    .spacing(MAIN_SPACING)
    .into();

    scrollable(container(add_code_preview(content, preview_content)).padding(padding::right(10.0)))
        .into()
}

pub fn scrollable_controls<'a>(
    hierarchy: &'a WidgetHierarchy,
    widget_id: WidgetId,
    _theme: &Theme,
    _type_system: &'a TypeSystem,
    _custom_styles: &CustomThemes,
    preview_content: &'a text_editor::Content,
) -> Element<'a, Message> {
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
                move |c| Message::PropertyChanged(
                    widget_id,
                    PropertyChange::ScrollableDirection(DirChoice::from_choice(c))
                )
            )
        ]
        .spacing(LABEL_SPACING),
        row![
            column![
                text("Anchor X").size(LABEL_SIZE),
                pick_list(
                    vec![AnchorChoice::Start, AnchorChoice::End],
                    Some(AnchorChoice::from(props.anchor_x)),
                    move |a| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::ScrollableAnchorX(AnchorChoice::from_anchor(a))
                    )
                )
            ]
            .spacing(LABEL_SPACING)
            .width(Length::Fill),
            column![
                text("Anchor Y").size(LABEL_SIZE),
                pick_list(
                    vec![AnchorChoice::Start, AnchorChoice::End],
                    Some(AnchorChoice::from(props.anchor_y)),
                    move |a| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::ScrollableAnchorY(AnchorChoice::from_anchor(a))
                    )
                )
            ]
            .spacing(LABEL_SPACING)
            .width(Length::Fill),
        ]
        .spacing(SECTION_SPACING),
    ]
    .spacing(MAIN_SPACING)
    .into();

    scrollable(container(add_code_preview(content, preview_content)).padding(padding::right(10.0)))
        .into()
}

pub fn space_controls<'a>(
    hierarchy: &'a WidgetHierarchy,
    widget_id: WidgetId,
    _theme: &Theme,
    _type_system: &'a TypeSystem,
    _custom_styles: &CustomThemes,
    preview_content: &'a text_editor::Content,
) -> Element<'a, Message> {
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

    scrollable(container(add_code_preview(content, preview_content)).padding(padding::right(10.0)))
        .into()
}

pub fn progress_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    theme: &Theme,
    type_system: &'a TypeSystem,
    custom_styles: &'a CustomThemes,
    preview_content: &'a text_editor::Content,
    views: &'a BTreeMap<Uuid, AppView>,
    view_id: Uuid,
) -> Element<'a, Message> {
    let w = h.get_widget_by_id(widget_id).unwrap();
    let p = &w.properties;
    let girth_str = format!("{:.0}", p.progress_girth);

    let clamp_step = ((p.progress_max - p.progress_min) / 100.0).abs().max(0.001);

    let content = column![
        text("Progress Bar Properties").size(TITLE_SIZE),
        widget_name(widget_id, &p.widget_name),
        custom_style_picker(custom_styles, widget_id, p, ThemePaneEnum::Progressbar),
        conditional_style_section(widget_id, p, theme, view_id, h.root(), views, type_system),
        row![
            text("Orientation")
                .size(LABEL_SIZE)
                .width(Length::Fixed(80.0)),
            radio("Horizontal", false, Some(p.progress_vertical), move |_| {
                Message::PropertyChanged(widget_id, PropertyChange::ProgressVertical(false))
            }),
            radio("Vertical", true, Some(p.progress_vertical), move |_| {
                Message::PropertyChanged(widget_id, PropertyChange::ProgressVertical(true))
            }),
        ]
        .spacing(SECTION_SPACING)
        .align_y(Alignment::Center),
        if p.progress_vertical {
            column![
                length_picker_scrollable_aware(
                    "Length",
                    p.progress_length,
                    move |len| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::ProgressLength(len)
                    ),
                    h,
                    widget_id,
                    true
                ),
                column![
                    text("Girth (Width)").size(LABEL_SIZE),
                    text_input("px", &girth_str)
                        .on_input(move |s| {
                            Message::PropertyChanged(
                                widget_id,
                                PropertyChange::ProgressGirth(parse_f32(&s, p.progress_girth)),
                            )
                        })
                        .width(120)
                ]
                .spacing(LABEL_SPACING),
            ]
            .spacing(SECTION_SPACING)
        } else {
            column![
                length_picker_scrollable_aware(
                    "Length",
                    p.progress_length,
                    move |len| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::ProgressLength(len)
                    ),
                    h,
                    widget_id,
                    false
                ),
                column![
                    text("Girth (Height)").size(LABEL_SIZE),
                    text_input("px", &girth_str)
                        .on_input(move |s| {
                            Message::PropertyChanged(
                                widget_id,
                                PropertyChange::ProgressGirth(parse_f32(&s, p.progress_girth)),
                            )
                        })
                        .width(120)
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
                    text_input("min", &format!("{}", p.progress_min))
                        .on_input(move |s| {
                            let v = s.trim().parse::<f32>().unwrap_or(p.progress_min);
                            Message::PropertyChanged(widget_id, PropertyChange::ProgressMin(v))
                        })
                        .width(120)
                ]
                .spacing(LABEL_SPACING),
                column![
                    text("Max").size(LABEL_SIZE),
                    text_input("max", &format!("{}", p.progress_max))
                        .on_input(move |s| {
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
                slider(
                    p.progress_min..=p.progress_max,
                    p.progress_value,
                    move |v| {
                        Message::PropertyChanged(widget_id, PropertyChange::ProgressValue(v))
                    }
                )
                .step(clamp_step)
                .width(250),
                text(format!("{:.02}", p.progress_value))
                    .size(LABEL_SIZE)
                    .width(60),
            ]
            .spacing(SECTION_SPACING)
            .align_y(Alignment::Center),
        ]
        .spacing(LABEL_SPACING),
    ]
    .spacing(MAIN_SPACING)
    .into();

    scrollable(container(add_code_preview(content, preview_content)).padding(padding::right(10.0)))
        .into()
}

pub fn image_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    _theme: &Theme,
    _type_system: &'a TypeSystem,
    _custom_styles: &CustomThemes,
    preview_content: &'a text_editor::Content,
) -> Element<'a, Message> {
    let w = h.get_widget_by_id(widget_id).unwrap();
    let props = &w.properties;

    let content = column![
        text("Image Properties").size(TITLE_SIZE),
        widget_name(widget_id, &props.widget_name),
        row![
            text("Path").size(LABEL_SIZE).width(Length::Fixed(80.0)),
            text_input("assets/pic.png", &props.image_path)
                .on_input(move |s| Message::PropertyChanged(
                    widget_id,
                    PropertyChange::ImagePath(s)
                ))
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

    scrollable(container(add_code_preview(content, preview_content)).padding(padding::right(10.0)))
        .into()
}

pub fn svg_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    _theme: &Theme,
    _type_system: &'a TypeSystem,
    _custom_styles: &CustomThemes,
    preview_content: &'a text_editor::Content,
) -> Element<'a, Message> {
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

    scrollable(container(add_code_preview(content, preview_content)).padding(padding::right(10.0)))
        .into()
}

pub fn tooltip_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    _theme: &Theme,
    _type_system: &'a TypeSystem,
    _custom_styles: &CustomThemes,
    preview_content: &'a text_editor::Content,
) -> Element<'a, Message> {
    let w = h.get_widget_by_id(widget_id).unwrap();
    let p = &w.properties;

    let content = column![
        text("Tooltip Properties").size(TITLE_SIZE),
        widget_name(widget_id, &p.widget_name),
        row![
            text("Text").size(LABEL_SIZE).width(Length::Fixed(80.0)),
            text_input("Tooltip text", &p.tooltip_text)
                .on_input(move |s| Message::PropertyChanged(
                    widget_id,
                    PropertyChange::TooltipText(s)
                ))
                .width(Length::Fill),
        ]
        .spacing(SECTION_SPACING),
        row![
            text("Position").size(LABEL_SIZE).width(Length::Fixed(80.0)),
            pick_list(
                vec![
                    TooltipPosition::Top,
                    TooltipPosition::Bottom,
                    TooltipPosition::Left,
                    TooltipPosition::Right
                ],
                Some(p.tooltip_position),
                move |pos| Message::PropertyChanged(
                    widget_id,
                    PropertyChange::TooltipPosition(pos)
                )
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

    scrollable(container(add_code_preview(content, preview_content)).padding(padding::right(10.0)))
        .into()
}

pub fn combobox_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    theme: &Theme,
    type_system: &'a TypeSystem,
    custom_styles: &'a CustomThemes,
    preview_content: &'a text_editor::Content,
) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    let selected = if let Some(referenced_enum) = props.referenced_enum {
        let enum_id = type_system.get_enum(referenced_enum);
        match enum_id {
            Some(enum_def) => enum_def.name.clone(),
            None => String::from("Choose an enum..."),
        }
    } else {
        String::from("Choose an enum...")
    };

    let content = column![
        text("ComboBox Properties").size(TITLE_SIZE),
        widget_name(widget_id, &props.widget_name),
        row![
            named_style_picker(
                custom_styles,
                widget_id,
                "Input Style",
                props.custom_style_name.clone(),
                ThemePaneEnum::TextInput,
                PropertyChange::CustomStyle,
            ),
            named_style_picker(
                custom_styles,
                widget_id,
                "Menu Style",
                props.menu_style_name.clone(),
                ThemePaneEnum::Menu,
                PropertyChange::MenuStyle,
            ),
        ]
        .spacing(MAIN_SPACING),
        column![
            text("Placeholder Text").size(LABEL_SIZE),
            text_input("Placeholder", &props.combobox_placeholder)
                .on_input(move |v| Message::PropertyChanged(
                    widget_id,
                    PropertyChange::ComboBoxPlaceholder(v)
                ))
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
                text(format!("{:.0}px", props.combobox_size))
                    .size(LABEL_SIZE)
                    .width(50),
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
                    |selected| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::RadioSelectedIndex(selected)
                    )
                ),
                radio(
                    "Use Enum",
                    1,
                    Some(props.radio_selected_index),
                    |selected| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::RadioSelectedIndex(selected)
                    )
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
                            pick_list(type_system.enum_names(), Some(selected), move |enum_name| {
                                let enum_id = type_system
                                    .get_enum_by_name(&enum_name)
                                    .expect("MissingEnumDef")
                                    .id;
                                Message::PropertyChanged(
                                    widget_id,
                                    PropertyChange::ComboBoxEnumId(Some(enum_id)),
                                )
                            })
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
                                    enum_def
                                        .variants
                                        .iter()
                                        .map(|variant| {
                                            text(format!("• {}", variant.name))
                                                .size(LABEL_SIZE)
                                                .into()
                                        })
                                        .collect::<Vec<Element<'a, Message>>>()
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
                    props
                        .combobox_options
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
                                            Message::PropertyChanged(
                                                widget_id,
                                                PropertyChange::ComboBoxState(new_options),
                                            )
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
                                        Message::PropertyChanged(
                                            widget_id,
                                            PropertyChange::ComboBoxState(new_options),
                                        )
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
                        Message::PropertyChanged(
                            widget_id,
                            PropertyChange::ComboBoxState(new_options),
                        )
                    })
                    .style(button::success),
            ]
            .spacing(SECTION_SPACING)
        },
        column![
            text("Events").size(SECTION_SIZE),
            column![
                event_checkbox_row(
                    theme,
                    props.combobox_use_on_input,
                    "on_input",
                    "Fires when the user types into the combo box search input",
                    move |v| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::ComboBoxUseOnInput(v)
                    ),
                ),
                event_checkbox_row(
                    theme,
                    props.combobox_use_on_option_hovered,
                    "on_option_hovered",
                    "Fires when an option is highlighted by hover or keyboard navigation",
                    move |v| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::ComboBoxUseOnOptionHovered(v)
                    ),
                ),
                event_checkbox_row(
                    theme,
                    props.combobox_use_on_open,
                    "on_open",
                    "Fires when the dropdown opens",
                    move |v| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::ComboBoxUseOnOpen(v)
                    ),
                ),
                event_checkbox_row(
                    theme,
                    props.combobox_use_on_close,
                    "on_close",
                    "Fires when the dropdown closes",
                    move |v| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::ComboBoxUseOnClose(v)
                    ),
                ),
            ]
            .spacing(LABEL_SPACING)
        ]
        .spacing(SECTION_SPACING),
        {
            // ComboBox icon section
            let filter = props.combobox_icon_picker_filter.to_lowercase();
            let icon_buttons: Vec<Element<'a, Message>> = icon_lucide::ALL_ICONS
                .iter()
                .filter(|(name, _)| filter.is_empty() || name.contains(filter.as_str()))
                .map(|(name, codepoint)| {
                    let cp_u32 = codepoint.chars().next().map(|c| c as u32).unwrap_or(0xFFFD);
                    let name_clone = name.to_string();
                    tooltip(
                        button(icon_lucide::render(codepoint))
                            .on_press(Message::PropertyChanged(
                                widget_id,
                                PropertyChange::ComboBoxIconSelected(name_clone, cp_u32),
                            ))
                            .style(button::text),
                        container(text(*name))
                            .style(container::bordered_box)
                            .padding(5),
                        tooltip::Position::Top,
                    )
                    .into()
                })
                .collect();

            let search = text_input("Search icons…", &props.combobox_icon_picker_filter).on_input(
                move |v| {
                    Message::PropertyChanged(widget_id, PropertyChange::ComboBoxIconPickerFilter(v))
                },
            );

            let picker_content = column![
                container(search).padding(padding::horizontal(5.0)),
                scrollable(
                    Row::with_children(icon_buttons)
                        .spacing(4)
                        .padding(10)
                        .wrap(),
                ),
            ]
            .spacing(4);

            let current_cp_str: &'static str = icon_lucide::ALL_ICONS
                .iter()
                .find(|(name, _)| *name == props.combobox_icon_name.as_str())
                .map(|(_, cp)| *cp)
                .unwrap_or("\u{FFFD}");

            let trigger = row![
                icon_lucide::render(current_cp_str).size(20),
                text(&props.combobox_icon_name),
            ]
            .spacing(6)
            .align_y(Alignment::Center);

            let picker = overlay_button(trigger, "Select Icon", picker_content)
                .overlay_width(650.0)
                .overlay_height(450.0)
                .hide_header()
                .close_on_click_outside()
                .hover_positions_on_click()
                .hover_position(widgets::generic_overlay::Position::Right)
                .hover_gap(5.0)
                .hover_alignment(Alignment::Start);

            column![
                text("Icon").size(SECTION_SIZE),
                checkbox(props.combobox_icon_enabled)
                    .label("Enable Icon")
                    .on_toggle(move |v| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::ComboBoxIconEnabled(v)
                    )),
                if props.combobox_icon_enabled {
                    column![
                        column![text("Icon").size(LABEL_SIZE), picker,].spacing(LABEL_SPACING),
                        column![
                            text("Side").size(LABEL_SIZE),
                            pick_list(
                                vec![TextInputIconSide::Left, TextInputIconSide::Right],
                                Some(props.combobox_icon_side),
                                move |v| Message::PropertyChanged(
                                    widget_id,
                                    PropertyChange::ComboBoxIconSide(v)
                                )
                            ),
                        ]
                        .spacing(LABEL_SPACING),
                        column![
                            text("Icon Size (0 = auto)").size(LABEL_SIZE),
                            row![
                                slider(0.0..=64.0, props.combobox_icon_size, move |v| {
                                    Message::PropertyChanged(
                                        widget_id,
                                        PropertyChange::ComboBoxIconSize(v),
                                    )
                                })
                                .step(1.0)
                                .width(180),
                                text(if props.combobox_icon_size > 0.0 {
                                    format!("{:.0}px", props.combobox_icon_size)
                                } else {
                                    "auto".to_string()
                                })
                                .size(LABEL_SIZE)
                                .width(50),
                            ]
                            .spacing(SECTION_SPACING)
                            .align_y(Alignment::Center),
                        ]
                        .spacing(LABEL_SPACING),
                        column![
                            text("Spacing").size(LABEL_SIZE),
                            row![
                                slider(0.0..=30.0, props.combobox_icon_spacing, move |v| {
                                    Message::PropertyChanged(
                                        widget_id,
                                        PropertyChange::ComboBoxIconSpacing(v),
                                    )
                                })
                                .step(1.0)
                                .width(180),
                                text(format!("{:.0}px", props.combobox_icon_spacing))
                                    .size(LABEL_SIZE)
                                    .width(50),
                            ]
                            .spacing(SECTION_SPACING)
                            .align_y(Alignment::Center),
                        ]
                        .spacing(LABEL_SPACING),
                    ]
                    .spacing(SECTION_SPACING)
                } else {
                    column![]
                },
            ]
            .spacing(SECTION_SPACING)
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

    scrollable(container(add_code_preview(content, preview_content)).padding(padding::right(10.0)))
        .into()
}

pub fn markdown_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    _theme: &Theme,
    _type_system: &'a TypeSystem,
    _custom_styles: &CustomThemes,
    preview_content: &'a text_editor::Content,
) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    let content = column![
        text("Markdown Properties").size(TITLE_SIZE),
        widget_name(widget_id, &props.widget_name),
        column![
            text("Markdown Content").size(LABEL_SIZE),
            text_editor(&props.markdown_source)
                .placeholder("Markdown text here")
                .on_action(move |act| Message::PropertyChanged(
                    widget_id,
                    PropertyChange::MarkdownContent(act)
                ))
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
                text(format!("{:.0}px", props.markdown_text_size))
                    .size(LABEL_SIZE)
                    .width(50),
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

    scrollable(container(add_code_preview(content, preview_content)).padding(padding::right(10.0)))
        .into()
}

pub fn qrcode_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    _theme: &Theme,
    _type_system: &'a TypeSystem,
    _custom_styles: &CustomThemes,
    preview_content: &'a text_editor::Content,
) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    let content = column![
        text("QR Code Properties").size(TITLE_SIZE),
        widget_name(widget_id, &props.widget_name),
        column![
            text("Data to Encode").size(LABEL_SIZE),
            text_input("Data", &props.qrcode_link)
                .on_input(move |v| Message::PropertyChanged(
                    widget_id,
                    PropertyChange::QRCodeData(v)
                ))
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
                text(format!("{}px", props.qrcode_cell_size))
                    .size(LABEL_SIZE)
                    .width(50),
            ]
            .spacing(SECTION_SPACING)
            .align_y(Alignment::Center),
        ]
        .spacing(LABEL_SPACING),
    ]
    .spacing(MAIN_SPACING)
    .into();

    scrollable(container(add_code_preview(content, preview_content)).padding(padding::right(10.0)))
        .into()
}

pub fn stack_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    _theme: &Theme,
    _type_system: &'a TypeSystem,
    _custom_styles: &CustomThemes,
    preview_content: &'a text_editor::Content,
) -> Element<'a, Message> {
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

    scrollable(container(add_code_preview(content, preview_content)).padding(padding::right(10.0)))
        .into()
}

fn collapsible_style_options() -> Vec<String> {
    ["Default", "Primary", "Success", "Danger", "Warning"]
        .into_iter()
        .map(str::to_string)
        .collect()
}

fn generic_overlay_style_options() -> Vec<String> {
    ["Primary", "Success", "Danger", "Warning", "Blank"]
        .into_iter()
        .map(str::to_string)
        .collect()
}

pub fn collapsible_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    theme: &Theme,
    type_system: &'a TypeSystem,
    _custom_styles: &'a CustomThemes,
    preview_content: &'a text_editor::Content,
    views: &'a BTreeMap<Uuid, AppView>,
    view_id: Uuid,
) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;
    let style_options = collapsible_style_options();

    let content = column![
        text("Collapsible Properties").size(TITLE_SIZE),
        widget_name(widget_id, &props.widget_name),
        text(
            "Collapsible uses the real custom widget and renders a single expandable content child."
        )
        .size(LABEL_SIZE)
        .color(Color::from_rgb(0.6, 0.6, 0.6)),
        column![
            text("Title").size(LABEL_SIZE),
            text_input("Collapsible", &props.collapsible_title)
                .on_input(move |value| Message::PropertyChanged(
                    widget_id,
                    PropertyChange::CollapsibleTitle(value)
                ))
                .width(300),
        ]
        .spacing(LABEL_SPACING),
        column![
            text("Header Height").size(LABEL_SIZE),
            row![
                slider(24.0..=80.0, props.collapsible_header_height, move |value| {
                    Message::PropertyChanged(
                        widget_id,
                        PropertyChange::CollapsibleHeaderHeight(value),
                    )
                })
                .step(1.0)
                .width(220),
                text(format!("{:.0}px", props.collapsible_header_height))
                    .size(LABEL_SIZE)
                    .width(60),
            ]
            .spacing(SECTION_SPACING)
            .align_y(Alignment::Center),
        ]
        .spacing(LABEL_SPACING),
        column![
            text("Title Alignment").size(LABEL_SIZE),
            pick_list(
                vec![
                    ContainerAlignX::Left,
                    ContainerAlignX::Center,
                    ContainerAlignX::Right,
                ],
                Some(props.align_x),
                move |value| Message::PropertyChanged(widget_id, PropertyChange::AlignX(value)),
            )
            .width(200),
        ]
        .spacing(LABEL_SPACING),
        column![
            text("Title Font Size").size(LABEL_SIZE),
            row![
                slider(8.0..=32.0, props.text_size, move |value| {
                    Message::PropertyChanged(widget_id, PropertyChange::TextSize(value))
                })
                .step(1.0)
                .width(220),
                text(format!("{:.0}px", props.text_size))
                    .size(LABEL_SIZE)
                    .width(60),
            ]
            .spacing(SECTION_SPACING)
            .align_y(Alignment::Center),
        ]
        .spacing(LABEL_SPACING),
        column![
            text("Title Font").size(LABEL_SIZE),
            pick_list(
                vec![FontType::Default, FontType::Monospace],
                Some(props.font),
                move |value| Message::PropertyChanged(widget_id, PropertyChange::Font(value)),
            )
            .width(200),
        ]
        .spacing(LABEL_SPACING),
        checkbox(props.collapsible_header_clickable)
            .label("Header is clickable")
            .on_toggle(move |value| Message::PropertyChanged(
                widget_id,
                PropertyChange::CollapsibleHeaderClickable(value)
            )),
        checkbox(props.collapsible_expanded)
            .label("Start expanded")
            .on_toggle(move |value| Message::PropertyChanged(
                widget_id,
                PropertyChange::CollapsibleExpanded(value)
            )),
        size_controls_scrollable_aware(
            props.width,
            move |l| Message::PropertyChanged(widget_id, PropertyChange::Width(l)),
            props.height,
            move |l| Message::PropertyChanged(widget_id, PropertyChange::Height(l)),
            h,
            widget_id,
        ),
        padding_controls(props.padding, widget_id, props.padding_mode, theme,),
        column![
            text("Style").size(LABEL_SIZE),
            pick_list(
                style_options.clone(),
                props.custom_style_name.clone(),
                move |selection| {
                    Message::PropertyChanged(
                        widget_id,
                        PropertyChange::CustomStyle(Some(selection)),
                    )
                }
            )
            .placeholder("default"),
            text("Alternate style").size(LABEL_SIZE),
            pick_list(
                style_options,
                props.active_style_name.clone(),
                move |selection| {
                    Message::PropertyChanged(
                        widget_id,
                        PropertyChange::ActiveStyle(Some(selection)),
                    )
                }
            )
            .placeholder("none"),
        ]
        .spacing(LABEL_SPACING),
        conditional_style_section(
            widget_id,
            props,
            theme,
            view_id,
            h.root(),
            views,
            type_system
        ),
    ]
    .spacing(MAIN_SPACING)
    .into();

    scrollable(container(add_code_preview(content, preview_content)).padding(padding::right(10.0)))
        .into()
}

pub fn collapsible_group_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    _theme: &Theme,
    _type_system: &'a TypeSystem,
    _custom_styles: &CustomThemes,
    preview_content: &'a text_editor::Content,
) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    let content = column![
        text("Collapsible Group Properties").size(TITLE_SIZE),
        widget_name(widget_id, &props.widget_name),
        text("Collapsible Group uses the real accordion widget and only accepts Collapsible children.")
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
        column![
            text("Spacing").size(LABEL_SIZE),
            row![
                slider(0.0..=32.0, props.spacing, move |value| {
                    Message::PropertyChanged(widget_id, PropertyChange::Spacing(value))
                })
                .step(1.0)
                .width(220),
                text(format!("{:.0}px", props.spacing))
                    .size(LABEL_SIZE)
                    .width(60),
            ]
            .spacing(SECTION_SPACING)
            .align_y(Alignment::Center),
        ]
        .spacing(LABEL_SPACING),
    ]
    .spacing(MAIN_SPACING)
    .into();

    scrollable(container(add_code_preview(content, preview_content)).padding(padding::right(10.0)))
        .into()
}

pub fn generic_overlay_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    theme: &Theme,
    type_system: &'a TypeSystem,
    custom_styles: &'a CustomThemes,
    preview_content: &'a text_editor::Content,
    views: &'a BTreeMap<Uuid, AppView>,
    view_id: Uuid,
) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;
    let has_trigger_child = widget.children.get(0).is_some();
    let uses_hover_placement =
        props.generic_overlay_on_hover || props.generic_overlay_hover_positions_on_click;

    let overlay_style_options = generic_overlay_style_options();
    let overlay_style_picker = column![
        text("Overlay panel style").size(LABEL_SIZE),
        pick_list(
            overlay_style_options,
            props.generic_overlay_overlay_style_name.clone(),
            move |selection| Message::PropertyChanged(
                widget_id,
                PropertyChange::GenericOverlayOverlayStyle(Some(selection))
            )
        )
        .placeholder("default"),
    ]
    .spacing(LABEL_SPACING);

    let opaque_alpha_controls: Element<'a, Message> = if props.generic_overlay_opaque {
        column![
            text("Backdrop Alpha").size(LABEL_SIZE),
            row![
                slider(
                    0.1..=1.0,
                    props.generic_overlay_opaque_alpha,
                    move |value| {
                        Message::PropertyChanged(
                            widget_id,
                            PropertyChange::GenericOverlayOpaqueAlpha(value),
                        )
                    }
                )
                .step(0.05)
                .width(220),
                text(format!("{:.2}", props.generic_overlay_opaque_alpha))
                    .size(LABEL_SIZE)
                    .width(60),
            ]
            .spacing(SECTION_SPACING)
            .align_y(Alignment::Center),
        ]
        .spacing(LABEL_SPACING)
        .into()
    } else {
        column![].into()
    };

    let animation_controls: Element<'a, Message> = if props.generic_overlay_animate {
        column![
            text("Animation Speed").size(LABEL_SIZE),
            pick_list(
                vec![
                    GenericOverlayAnimationPreset::Default,
                    GenericOverlayAnimationPreset::Quick,
                    GenericOverlayAnimationPreset::Slow,
                ],
                Some(props.generic_overlay_animation_preset),
                move |value| Message::PropertyChanged(
                    widget_id,
                    PropertyChange::GenericOverlayAnimationPreset(value)
                ),
            )
            .width(160),
        ]
        .spacing(LABEL_SPACING)
        .into()
    } else {
        column![].into()
    };

    let trigger_controls: Element<'a, Message> = if has_trigger_child {
        column![
            text(
                "The trigger wrapper stays shrink-sized with zero padding and text-button styling so the trigger child renders unchanged."
            )
            .size(LABEL_SIZE)
            .color(Color::from_rgb(0.6, 0.6, 0.6)),
        ]
        .spacing(LABEL_SPACING)
        .into()
    } else {
        column![
            custom_style_picker(custom_styles, widget_id, props, ThemePaneEnum::Button),
            conditional_style_section(
                widget_id,
                props,
                theme,
                view_id,
                h.root(),
                views,
                type_system
            ),
            size_controls_scrollable_aware(
                props.width,
                move |l| Message::PropertyChanged(widget_id, PropertyChange::Width(l)),
                props.height,
                move |l| Message::PropertyChanged(widget_id, PropertyChange::Height(l)),
                h,
                widget_id,
            ),
            padding_controls(props.padding, widget_id, props.padding_mode, theme,),
            clip_control(widget_id, props.clip, theme),
        ]
        .spacing(MAIN_SPACING)
        .into()
    };

    let placement_controls: Element<'a, Message> = if uses_hover_placement {
        column![
            row![
                column![
                    text("Hover Position").size(LABEL_SIZE),
                    pick_list(
                        vec![
                            GenericOverlayPosition::Top,
                            GenericOverlayPosition::Right,
                            GenericOverlayPosition::Bottom,
                            GenericOverlayPosition::Left,
                        ],
                        Some(props.generic_overlay_hover_position),
                        move |value| Message::PropertyChanged(
                            widget_id,
                            PropertyChange::GenericOverlayHoverPosition(value)
                        ),
                    )
                    .width(160),
                ]
                .spacing(LABEL_SPACING),
                column![
                    text("Alignment").size(LABEL_SIZE),
                    pick_list(
                        vec![
                            ContainerAlignX::Left,
                            ContainerAlignX::Center,
                            ContainerAlignX::Right,
                        ],
                        Some(props.generic_overlay_hover_alignment),
                        move |value| Message::PropertyChanged(
                            widget_id,
                            PropertyChange::GenericOverlayHoverAlignment(value)
                        ),
                    )
                    .width(160),
                ]
                .spacing(LABEL_SPACING),
                column![
                    text("Mode").size(LABEL_SIZE),
                    pick_list(
                        vec![
                            GenericOverlayPositionMode::Outside,
                            GenericOverlayPositionMode::Inside,
                        ],
                        Some(props.generic_overlay_hover_mode),
                        move |value| Message::PropertyChanged(
                            widget_id,
                            PropertyChange::GenericOverlayHoverMode(value)
                        ),
                    )
                    .width(160),
                ]
                .spacing(LABEL_SPACING),
            ]
            .spacing(MAIN_SPACING),
            column![
                text("Hover Gap").size(LABEL_SIZE),
                row![
                    slider(0.0..=40.0, props.generic_overlay_hover_gap, move |value| {
                        Message::PropertyChanged(
                            widget_id,
                            PropertyChange::GenericOverlayHoverGap(value),
                        )
                    })
                    .step(1.0)
                    .width(220),
                    text(format!("{:.0}px", props.generic_overlay_hover_gap))
                        .size(LABEL_SIZE)
                        .width(60),
                ]
                .spacing(SECTION_SPACING)
                .align_y(Alignment::Center),
            ]
            .spacing(LABEL_SPACING),
            checkbox(props.generic_overlay_hover_snap)
                .label("Snap inside viewport")
                .on_toggle(move |value| Message::PropertyChanged(
                    widget_id,
                    PropertyChange::GenericOverlayHoverSnap(value)
                )),
            checkbox(props.generic_overlay_safe_triangle)
                .label("Use safe triangle")
                .on_toggle(move |value| Message::PropertyChanged(
                    widget_id,
                    PropertyChange::GenericOverlaySafeTriangle(value)
                )),
        ]
        .spacing(MAIN_SPACING)
        .into()
    } else {
        column![].into()
    };

    let content = column![
        text("Generic Overlay Properties").size(TITLE_SIZE),
        widget_name(widget_id, &props.widget_name),
        text(
            "Generic Overlay uses the real custom widget. Child 1 is the trigger content and child 2 is the overlay content."
        )
        .size(LABEL_SIZE)
        .color(Color::from_rgb(0.6, 0.6, 0.6)),
        column![
            text("Header Title").size(LABEL_SIZE),
            text_input("Overlay", &props.generic_overlay_title)
                .on_input(move |value| Message::PropertyChanged(
                    widget_id,
                    PropertyChange::GenericOverlayTitle(value)
                ))
                .width(300),
        ]
        .spacing(LABEL_SPACING),
        text("Trigger").size(SECTION_SIZE),
        trigger_controls,
        text("Overlay Panel").size(SECTION_SIZE),
        overlay_style_picker,
        row![
            generic_overlay_size_picker(
                "Overlay Width",
                props.generic_overlay_overlay_width,
                props.generic_overlay_overlay_width_dynamic,
                props.generic_overlay_overlay_width_dynamic_factor,
                Some(&props.draft_generic_overlay_overlay_width_fixed),
                Some(&props.draft_generic_overlay_overlay_width_fill_portion),
                Some(&props.draft_generic_overlay_overlay_width_dynamic),
                move |choice| Message::PropertyChanged(
                    widget_id,
                    PropertyChange::GenericOverlayOverlayWidthChoice(choice)
                ),
                move |text| Message::PropertyChanged(
                    widget_id,
                    PropertyChange::DraftGenericOverlayOverlayWidthFixed(text)
                ),
                move |text| Message::PropertyChanged(
                    widget_id,
                    PropertyChange::DraftGenericOverlayOverlayWidthFillPortion(text)
                ),
                move |text| Message::PropertyChanged(
                    widget_id,
                    PropertyChange::DraftGenericOverlayOverlayWidthDynamic(text)
                ),
            ),
            generic_overlay_size_picker(
                "Overlay Height",
                props.generic_overlay_overlay_height,
                props.generic_overlay_overlay_height_dynamic,
                props.generic_overlay_overlay_height_dynamic_factor,
                Some(&props.draft_generic_overlay_overlay_height_fixed),
                Some(&props.draft_generic_overlay_overlay_height_fill_portion),
                Some(&props.draft_generic_overlay_overlay_height_dynamic),
                move |choice| Message::PropertyChanged(
                    widget_id,
                    PropertyChange::GenericOverlayOverlayHeightChoice(choice)
                ),
                move |text| Message::PropertyChanged(
                    widget_id,
                    PropertyChange::DraftGenericOverlayOverlayHeightFixed(text)
                ),
                move |text| Message::PropertyChanged(
                    widget_id,
                    PropertyChange::DraftGenericOverlayOverlayHeightFillPortion(text)
                ),
                move |text| Message::PropertyChanged(
                    widget_id,
                    PropertyChange::DraftGenericOverlayOverlayHeightDynamic(text)
                ),
            ),
        ]
        .spacing(MAIN_SPACING),
        column![
            text("Overlay Padding").size(LABEL_SIZE),
            row![
                slider(0.0..=40.0, props.generic_overlay_overlay_padding, move |value| {
                    Message::PropertyChanged(
                        widget_id,
                        PropertyChange::GenericOverlayOverlayPadding(value)
                    )
                })
                .step(1.0)
                .width(220),
                text(format!("{:.0}px", props.generic_overlay_overlay_padding))
                    .size(LABEL_SIZE)
                    .width(60),
            ]
            .spacing(SECTION_SPACING)
            .align_y(Alignment::Center),
        ]
        .spacing(LABEL_SPACING),
        column![
            text("Overlay Radius").size(LABEL_SIZE),
            row![
                slider(0.0..=32.0, props.generic_overlay_overlay_radius, move |value| {
                    Message::PropertyChanged(
                        widget_id,
                        PropertyChange::GenericOverlayOverlayRadius(value)
                    )
                })
                .step(1.0)
                .width(220),
                text(format!("{:.0}px", props.generic_overlay_overlay_radius))
                    .size(LABEL_SIZE)
                    .width(60),
            ]
            .spacing(SECTION_SPACING)
            .align_y(Alignment::Center),
        ]
        .spacing(LABEL_SPACING),
        text("Placement").size(SECTION_SIZE),
        checkbox(props.generic_overlay_on_hover)
            .label("Open on hover")
            .on_toggle(move |value| Message::PropertyChanged(
                widget_id,
                PropertyChange::GenericOverlayOnHover(value)
            )),
        checkbox(props.generic_overlay_hover_positions_on_click)
            .label("Use hover placement on click")
            .on_toggle(move |value| Message::PropertyChanged(
                widget_id,
                PropertyChange::GenericOverlayHoverPositionsOnClick(value)
            )),
        placement_controls,
        text("Behavior").size(SECTION_SIZE),
        checkbox(props.generic_overlay_initially_open)
            .label("Start open")
            .on_toggle(move |value| Message::PropertyChanged(
                widget_id,
                PropertyChange::GenericOverlayInitiallyOpen(value)
            )),
        checkbox(props.generic_overlay_close_on_click_outside)
            .label("Close on click outside")
            .on_toggle(move |value| Message::PropertyChanged(
                widget_id,
                PropertyChange::GenericOverlayCloseOnClickOutside(value)
            )),
        checkbox(props.generic_overlay_opaque)
            .label("Opaque backdrop")
            .on_toggle(move |value| Message::PropertyChanged(
                widget_id,
                PropertyChange::GenericOverlayOpaque(value)
            )),
        opaque_alpha_controls,
        checkbox(props.generic_overlay_hide_header)
            .label("Hide header")
            .on_toggle(move |value| Message::PropertyChanged(
                widget_id,
                PropertyChange::GenericOverlayHideHeader(value)
            )),
        checkbox(props.generic_overlay_hide_close_button)
            .label("Hide close button")
            .on_toggle(move |value| Message::PropertyChanged(
                widget_id,
                PropertyChange::GenericOverlayHideCloseButton(value)
            )),
        checkbox(props.generic_overlay_block_dragging)
            .label("Block dragging")
            .on_toggle(move |value| Message::PropertyChanged(
                widget_id,
                PropertyChange::GenericOverlayBlockDragging(value)
            )),
        column![
            text("Resize Mode").size(LABEL_SIZE),
            pick_list(
                vec![
                    GenericOverlayResizeMode::None,
                    GenericOverlayResizeMode::Always,
                    GenericOverlayResizeMode::WithCtrl,
                ],
                Some(props.generic_overlay_resizable),
                move |value| Message::PropertyChanged(
                    widget_id,
                    PropertyChange::GenericOverlayResizable(value)
                ),
            )
            .width(180),
        ]
        .spacing(LABEL_SPACING),
        checkbox(props.generic_overlay_reset_on_close)
            .label("Reset position and size on close")
            .on_toggle(move |value| Message::PropertyChanged(
                widget_id,
                PropertyChange::GenericOverlayResetOnClose(value)
            )),
        checkbox(props.generic_overlay_animate)
            .label("Animate open and close")
            .on_toggle(move |value| Message::PropertyChanged(
                widget_id,
                PropertyChange::GenericOverlayAnimate(value)
            )),
        animation_controls,
    ]
    .spacing(MAIN_SPACING)
    .into();

    scrollable(container(add_code_preview(content, preview_content)).padding(padding::right(10.0)))
        .into()
}

pub fn date_picker_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    theme: &Theme,
    type_system: &'a TypeSystem,
    custom_styles: &'a CustomThemes,
    preview_content: &'a text_editor::Content,
    views: &'a BTreeMap<Uuid, AppView>,
    view_id: Uuid,
) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    let mode_controls = column![
        text("Selection Mode").size(LABEL_SIZE),
        pick_list(
            vec![
                DatePickerSelectionMode::Single,
                DatePickerSelectionMode::Range,
            ],
            Some(props.date_picker_mode),
            move |value| Message::PropertyChanged(
                widget_id,
                PropertyChange::DatePickerSelectionMode(value),
            )
        )
        .width(180),
    ]
    .spacing(LABEL_SPACING);

    let initial_selection_controls: Element<'a, Message> = match props.date_picker_mode {
        DatePickerSelectionMode::Single => column![
            text("Initial Date").size(LABEL_SIZE),
            text_input("YYYY-MM-DD", &props.date_picker_initial_single_date)
                .on_input(move |value| Message::PropertyChanged(
                    widget_id,
                    PropertyChange::DatePickerInitialSingleDate(value),
                ))
                .width(220),
        ]
        .spacing(LABEL_SPACING)
        .into(),
        DatePickerSelectionMode::Range => row![
            column![
                text("Range Start").size(LABEL_SIZE),
                text_input("YYYY-MM-DD", &props.date_picker_initial_range_start)
                    .on_input(move |value| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::DatePickerInitialRangeStart(value),
                    ))
                    .width(180),
            ]
            .spacing(LABEL_SPACING)
            .width(Length::Fill),
            column![
                text("Range End").size(LABEL_SIZE),
                text_input("YYYY-MM-DD", &props.date_picker_initial_range_end)
                    .on_input(move |value| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::DatePickerInitialRangeEnd(value),
                    ))
                    .width(180),
            ]
            .spacing(LABEL_SPACING)
            .width(Length::Fill),
        ]
        .spacing(MAIN_SPACING)
        .into(),
    };

    let time_controls: Element<'a, Message> = if props.date_picker_show_time {
        column![
            text("Initial Time").size(SECTION_SIZE),
            column![
                text("Hour").size(LABEL_SIZE),
                row![
                    slider(
                        0.0..=23.0,
                        props.date_picker_initial_hour as f32,
                        move |value| {
                            Message::PropertyChanged(
                                widget_id,
                                PropertyChange::DatePickerInitialHour(value.round() as u32),
                            )
                        }
                    )
                    .step(1.0)
                    .width(220),
                    text(format!("{:02}", props.date_picker_initial_hour))
                        .size(LABEL_SIZE)
                        .width(40),
                ]
                .spacing(SECTION_SPACING)
                .align_y(Alignment::Center),
            ]
            .spacing(LABEL_SPACING),
            column![
                text("Minute").size(LABEL_SIZE),
                row![
                    slider(
                        0.0..=59.0,
                        props.date_picker_initial_minute as f32,
                        move |value| {
                            Message::PropertyChanged(
                                widget_id,
                                PropertyChange::DatePickerInitialMinute(value.round() as u32),
                            )
                        }
                    )
                    .step(1.0)
                    .width(220),
                    text(format!("{:02}", props.date_picker_initial_minute))
                        .size(LABEL_SIZE)
                        .width(40),
                ]
                .spacing(SECTION_SPACING)
                .align_y(Alignment::Center),
            ]
            .spacing(LABEL_SPACING),
        ]
        .spacing(MAIN_SPACING)
        .into()
    } else {
        column![].into()
    };

    let content = column![
        text("Date Picker Properties").size(TITLE_SIZE),
        widget_name(widget_id, &props.widget_name),
        column![
            text("Trigger Text").size(LABEL_SIZE),
            text_input("Select a date", &props.text_content)
                .on_input(move |value| Message::PropertyChanged(
                    widget_id,
                    PropertyChange::TextContent(value),
                ))
                .width(260),
            text("Used until the picker has a selected date.")
                .size(LABEL_SIZE - 1.0)
                .color(Color::from_rgb(0.6, 0.6, 0.6)),
        ]
        .spacing(LABEL_SPACING),
        custom_style_picker(custom_styles, widget_id, props, ThemePaneEnum::Button),
        conditional_style_section(
            widget_id,
            props,
            theme,
            view_id,
            h.root(),
            views,
            type_system
        ),
        mode_controls,
        checkbox(props.date_picker_show_time)
            .label("Include time selection")
            .on_toggle(move |value| Message::PropertyChanged(
                widget_id,
                PropertyChange::DatePickerShowTime(value),
            )),
        checkbox(props.date_picker_initially_open)
            .label("Start open")
            .on_toggle(move |value| Message::PropertyChanged(
                widget_id,
                PropertyChange::DatePickerInitiallyOpen(value),
            )),
        initial_selection_controls,
        time_controls,
        size_controls_scrollable_aware(
            props.width,
            move |l| Message::PropertyChanged(widget_id, PropertyChange::Width(l)),
            props.height,
            move |l| Message::PropertyChanged(widget_id, PropertyChange::Height(l)),
            h,
            widget_id,
        ),
        padding_controls(props.padding, widget_id, props.padding_mode, theme,),
        clip_control(widget_id, props.clip, theme),
    ]
    .spacing(MAIN_SPACING)
    .into();

    scrollable(container(add_code_preview(content, preview_content)).padding(padding::right(10.0)))
        .into()
}

pub fn mousearea_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    theme: &Theme,
    _type_system: &'a TypeSystem,
    _custom_styles: &CustomThemes,
    preview_content: &'a text_editor::Content,
) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    let content = column![
        text("Mouse Area Properties").size(TITLE_SIZE),
        widget_name(widget_id, &props.widget_name),
        text("Mouse Area captures mouse events over its child widget.")
            .size(LABEL_SIZE)
            .color(Color::from_rgb(0.6, 0.6, 0.6)),
        column![
            text("Events").size(SECTION_SIZE),
            column![
                text("Left Mouse Button:").size(LABEL_SIZE),
                event_checkbox_row(
                    theme,
                    props.mousearea_on_press,
                    "on_press",
                    "Fires when the left mouse button is pressed over the widget",
                    move |v| Message::PropertyChanged(widget_id, PropertyChange::MouseAreaOnPress(v)),
                ),
                event_checkbox_row(
                    theme,
                    props.mousearea_on_release,
                    "on_release",
                    "Fires when the left mouse button is released over the widget",
                    move |v| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::MouseAreaOnRelease(v)
                    ),
                ),
                event_checkbox_row(
                    theme,
                    props.mousearea_on_double_click,
                    "on_double_click",
                    "Fires when the widget is double-clicked with the left mouse button",
                    move |v| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::MouseAreaOnDoubleClick(v)
                    ),
                ),
            ]
            .spacing(LABEL_SPACING),
            column![
                text("Right Mouse Button:").size(LABEL_SIZE),
                event_checkbox_row(
                    theme,
                    props.mousearea_on_right_press,
                    "on_right_press",
                    "Fires when the right mouse button is pressed over the widget",
                    move |v| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::MouseAreaOnRightPress(v)
                    ),
                ),
                event_checkbox_row(
                    theme,
                    props.mousearea_on_right_release,
                    "on_right_release",
                    "Fires when the right mouse button is released over the widget",
                    move |v| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::MouseAreaOnRightRelease(v)
                    ),
                ),
            ]
            .spacing(LABEL_SPACING),
            column![
                text("Middle Mouse Button:").size(LABEL_SIZE),
                event_checkbox_row(
                    theme,
                    props.mousearea_on_middle_press,
                    "on_middle_press",
                    "Fires when the middle mouse button is pressed over the widget",
                    move |v| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::MouseAreaOnMiddlePress(v)
                    ),
                ),
                event_checkbox_row(
                    theme,
                    props.mousearea_on_middle_release,
                    "on_middle_release",
                    "Fires when the middle mouse button is released over the widget",
                    move |v| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::MouseAreaOnMiddleRelease(v)
                    ),
                ),
            ]
            .spacing(LABEL_SPACING),
            column![
                text("Other Events:").size(LABEL_SIZE),
                event_checkbox_row(
                    theme,
                    props.mousearea_on_scroll,
                    "on_scroll",
                    "Fires when the user scrolls over the widget and provides the scroll delta",
                    move |v| Message::PropertyChanged(widget_id, PropertyChange::MouseAreaOnScroll(v)),
                ),
                event_checkbox_row(
                    theme,
                    props.mousearea_on_enter,
                    "on_enter",
                    "Fires when the pointer enters the widget bounds",
                    move |v| Message::PropertyChanged(widget_id, PropertyChange::MouseAreaOnEnter(v)),
                ),
                event_checkbox_row(
                    theme,
                    props.mousearea_on_move,
                    "on_move",
                    "Fires when the pointer moves over the widget and provides the pointer position",
                    move |v| Message::PropertyChanged(widget_id, PropertyChange::MouseAreaOnMove(v)),
                ),
                event_checkbox_row(
                    theme,
                    props.mousearea_on_exit,
                    "on_exit",
                    "Fires when the pointer leaves the widget bounds",
                    move |v| Message::PropertyChanged(widget_id, PropertyChange::MouseAreaOnExit(v)),
                ),
            ]
            .spacing(LABEL_SPACING),
            column![
                text("Mouse Cursor:").size(LABEL_SIZE),
                pick_list(
                    MouseInteraction::ALL,
                    props.mousearea_interaction,
                    move |v| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::MouseAreaInteraction(Some(v))
                    )
                )
                .placeholder("Default cursor"),
            ]
            .spacing(LABEL_SPACING),
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

    scrollable(container(add_code_preview(content, preview_content)).padding(padding::right(10.0)))
        .into()
}

pub fn themer_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    _theme: &Theme,
    _type_system: &'a TypeSystem,
    _custom_styles: &CustomThemes,
    preview_content: &'a text_editor::Content,
) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    let content = column![
        text("Themer Properties").size(TITLE_SIZE),
        widget_name(widget_id, &props.widget_name),
        column![
            text("Theme").size(LABEL_SIZE),
            pick_list(Theme::ALL, props.themer_theme.clone(), move |theme| {
                Message::PropertyChanged(widget_id, PropertyChange::ThemerTheme(Some(theme)))
            })
            .placeholder("Inherit from parent"),
        ]
        .spacing(LABEL_SPACING),
        text("Themer applies a theme to all its children.")
            .size(LABEL_SIZE)
            .color(Color::from_rgb(0.6, 0.6, 0.6)),
    ]
    .spacing(MAIN_SPACING)
    .into();

    scrollable(container(add_code_preview(content, preview_content)).padding(padding::right(10.0)))
        .into()
}

pub fn grid_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    _theme: &Theme,
    _type_system: &'a TypeSystem,
    _custom_styles: &CustomThemes,
    preview_content: &'a text_editor::Content,
) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    let columns_str = props.grid_columns.to_string();
    let spacing_str = format!("{}", props.grid_spacing);
    let fixed_width_str = match props.grid_fixed_width {
        Some(w) => format!("{}", w),
        None => String::new(),
    };
    let fluid_max_str = format!("{}", props.grid_fluid_max_width);

    let layout_section: Element<Message> = if props.grid_use_fluid {
        column![
            text("Fluid Max Column Width").size(LABEL_SIZE),
            text_input("200", &fluid_max_str)
                .on_input(move |s| {
                    let v = parse_f32(&s, props.grid_fluid_max_width);
                    Message::PropertyChanged(widget_id, PropertyChange::GridFluidMaxWidth(v))
                })
                .width(100),
            text("Columns fill available space up to this width.")
                .size(LABEL_SIZE - 1.0)
                .color(Color::from_rgb(0.5, 0.5, 0.5)),
        ]
        .spacing(LABEL_SPACING)
        .into()
    } else {
        column![
            text("Columns").size(LABEL_SIZE),
            text_input("3", &columns_str)
                .on_input(move |s| {
                    let v = s
                        .trim()
                        .parse::<usize>()
                        .unwrap_or(props.grid_columns)
                        .max(1);
                    Message::PropertyChanged(widget_id, PropertyChange::GridColumns(v))
                })
                .width(100),
        ]
        .spacing(LABEL_SPACING)
        .into()
    };

    let content = column![
        text("Grid Properties").size(TITLE_SIZE),
        widget_name(widget_id, &props.widget_name),
        text("Grid distributes children in a uniform column layout.")
            .size(LABEL_SIZE)
            .color(Color::from_rgb(0.6, 0.6, 0.6)),
        column![
            text("Layout Mode").size(LABEL_SIZE),
            toggler(props.grid_use_fluid)
                .label("Fluid (auto-column) mode")
                .on_toggle(move |v| Message::PropertyChanged(
                    widget_id,
                    PropertyChange::GridUseFluid(v)
                )),
        ]
        .spacing(LABEL_SPACING),
        layout_section,
        column![
            text("Spacing").size(LABEL_SIZE),
            text_input("0", &spacing_str)
                .on_input(move |s| {
                    let v = parse_f32(&s, props.grid_spacing);
                    Message::PropertyChanged(widget_id, PropertyChange::GridSpacing(v))
                })
                .width(100),
        ]
        .spacing(LABEL_SPACING),
        column![
            text("Fixed Width (px)").size(LABEL_SIZE),
            text_input("none", &fixed_width_str)
                .on_input(move |s| {
                    let v = if s.trim().is_empty() {
                        None
                    } else {
                        parse_f32(&s, props.grid_fixed_width.unwrap_or(0.0)).into()
                    };
                    Message::PropertyChanged(widget_id, PropertyChange::GridFixedWidth(v))
                })
                .width(100),
            text("Pixel width for the entire grid widget. Leave empty for Fill.")
                .size(LABEL_SIZE - 1.0)
                .color(Color::from_rgb(0.5, 0.5, 0.5)),
        ]
        .spacing(LABEL_SPACING),
    ]
    .spacing(MAIN_SPACING)
    .into();

    scrollable(container(add_code_preview(content, preview_content)).padding(padding::right(10.0)))
        .into()
}

pub fn pin_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    _theme: &Theme,
    _type_system: &'a TypeSystem,
    _custom_styles: &CustomThemes,
    preview_content: &'a text_editor::Content,
) -> Element<'a, Message> {
    let w = h.get_widget_by_id(widget_id).unwrap();
    let props = &w.properties;

    let x_str = if props.draft_pin_x.is_empty() {
        format!("{}", props.pin_point.x)
    } else {
        props.draft_pin_x.clone()
    };
    let y_str = if props.draft_pin_y.is_empty() {
        format!("{}", props.pin_point.y)
    } else {
        props.draft_pin_y.clone()
    };

    let content = column![
        text("Pin Properties").size(TITLE_SIZE),
        widget_name(widget_id, &props.widget_name),
        text("Position").size(SECTION_SIZE),
        row![
            text("X").size(LABEL_SIZE).width(Length::Fixed(20.0)),
            text_input("0", &x_str)
                .on_input(move |s| {
                    let v = parse_f32(&s, props.pin_point.x);
                    Message::PropertyChanged(widget_id, PropertyChange::PinX(v))
                })
                .width(120),
        ]
        .spacing(SECTION_SPACING)
        .align_y(Alignment::Center),
        row![
            text("Y").size(LABEL_SIZE).width(Length::Fixed(20.0)),
            text_input("0", &y_str)
                .on_input(move |s| {
                    let v = parse_f32(&s, props.pin_point.y);
                    Message::PropertyChanged(widget_id, PropertyChange::PinY(v))
                })
                .width(120),
        ]
        .spacing(SECTION_SPACING)
        .align_y(Alignment::Center),
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

    scrollable(container(add_code_preview(content, preview_content)).padding(padding::right(10.0)))
        .into()
}

pub fn table_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    _theme: &Theme,
    type_system: &'a TypeSystem,
    _custom_styles: &CustomThemes,
    preview_content: &'a text_editor::Content,
) -> Element<'a, Message> {
    let w = h.get_widget_by_id(widget_id).unwrap();
    let props = &w.properties;

    let selected_struct_name = if let Some(struct_id) = props.table_referenced_struct {
        type_system
            .get_struct(struct_id)
            .map(|s| s.name.clone())
            .unwrap_or_else(|| String::from("Choose a struct..."))
    } else {
        String::from("Choose a struct...")
    };

    let struct_names: Vec<String> = type_system
        .all_structs()
        .iter()
        .map(|s| s.name.clone())
        .collect();

    let content = column![
        text("Table Properties").size(TITLE_SIZE),
        widget_name(widget_id, &props.widget_name),
        column![
            text("Data Source (Struct)").size(SECTION_SIZE),
            if struct_names.is_empty() {
                column![
                    text("No structs defined yet")
                        .size(LABEL_SIZE)
                        .style(text::warning),
                ]
                .spacing(LABEL_SPACING)
            } else {
                column![
                    pick_list(
                        struct_names,
                        Some(selected_struct_name),
                        move |struct_name| {
                            let struct_id = type_system
                                .get_struct_by_name(&struct_name)
                                .expect("MissingStructDef")
                                .id;
                            Message::PropertyChanged(
                                widget_id,
                                PropertyChange::TableReferencedStruct(Some(struct_id)),
                            )
                        }
                    )
                    .placeholder("Choose a struct...")
                    .width(200),
                ]
            },
        ]
        .spacing(LABEL_SPACING),
        // Show struct fields if a struct is selected
        if let Some(struct_id) = props.table_referenced_struct {
            if let Some(struct_def) = type_system.get_struct(struct_id) {
                column![
                    text(format!("Columns ({})", struct_def.fields.len()))
                        .size(LABEL_SIZE)
                        .color(Color::from_rgb(0.5, 0.5, 0.5)),
                    scrollable(
                        column(
                            struct_def
                                .fields
                                .iter()
                                .map(|field| {
                                    text(format!(
                                        "  {} : {}",
                                        field.name,
                                        field.field_type.display_name()
                                    ))
                                    .size(LABEL_SIZE)
                                    .into()
                                })
                                .collect::<Vec<Element<'a, Message>>>()
                        )
                        .spacing(LABEL_SPACING)
                    )
                    .width(Length::Fill)
                    .height(Length::Fixed(120.0))
                ]
                .width(Length::Fill)
                .spacing(LABEL_SPACING)
            } else {
                column![]
            }
        } else {
            column![]
        },
        text("Table Settings").size(SECTION_SIZE),
        column![
            text("Padding X").size(LABEL_SIZE),
            row![
                slider(0.0..=30.0, props.table_padding_x, move |v| {
                    Message::PropertyChanged(widget_id, PropertyChange::TablePaddingX(v))
                })
                .step(1.0)
                .width(200),
                text(format!("{:.0}", props.table_padding_x))
                    .size(LABEL_SIZE)
                    .width(30),
            ]
            .spacing(SECTION_SPACING)
            .align_y(Alignment::Center),
        ]
        .spacing(LABEL_SPACING),
        column![
            text("Padding Y").size(LABEL_SIZE),
            row![
                slider(0.0..=30.0, props.table_padding_y, move |v| {
                    Message::PropertyChanged(widget_id, PropertyChange::TablePaddingY(v))
                })
                .step(1.0)
                .width(200),
                text(format!("{:.0}", props.table_padding_y))
                    .size(LABEL_SIZE)
                    .width(30),
            ]
            .spacing(SECTION_SPACING)
            .align_y(Alignment::Center),
        ]
        .spacing(LABEL_SPACING),
        column![
            text("Separator X").size(LABEL_SIZE),
            row![
                slider(0.0..=10.0, props.table_separator_x, move |v| {
                    Message::PropertyChanged(widget_id, PropertyChange::TableSeparatorX(v))
                })
                .step(0.5)
                .width(200),
                text(format!("{:.1}", props.table_separator_x))
                    .size(LABEL_SIZE)
                    .width(30),
            ]
            .spacing(SECTION_SPACING)
            .align_y(Alignment::Center),
        ]
        .spacing(LABEL_SPACING),
        column![
            text("Separator Y").size(LABEL_SIZE),
            row![
                slider(0.0..=10.0, props.table_separator_y, move |v| {
                    Message::PropertyChanged(widget_id, PropertyChange::TableSeparatorY(v))
                })
                .step(0.5)
                .width(200),
                text(format!("{:.1}", props.table_separator_y))
                    .size(LABEL_SIZE)
                    .width(30),
            ]
            .spacing(SECTION_SPACING)
            .align_y(Alignment::Center),
        ]
        .spacing(LABEL_SPACING),
        checkbox(props.table_bold_headers)
            .label("Bold Headers")
            .on_toggle(move |v| Message::PropertyChanged(
                widget_id,
                PropertyChange::TableBoldHeaders(v)
            )),
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

    scrollable(container(add_code_preview(content, preview_content)).padding(padding::right(10.0)))
        .into()
}

pub fn view_reference_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    theme: &Theme,
    _type_system: &'a TypeSystem,
    views: &'a BTreeMap<Uuid, AppView>,
    _custom_styles: &CustomThemes,
    preview_content: &'a text_editor::Content,
) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    // Create a list of (Uuid, Name) for the pick_list
    let view_options: Vec<(Uuid, String)> = views
        .iter()
        .map(|(id, view)| (*id, view.name.clone()))
        .collect();

    // Get the currently selected view name (if any)
    let selected_view_name = props
        .referenced_view_id
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
                view_options
                    .iter()
                    .map(|(_, name)| name.clone())
                    .collect::<Vec<_>>(),
                selected_view_name,
                move |selected_name| {
                    // Find the UUID for the selected name
                    let view_id = view_options
                        .iter()
                        .find(|(_, name)| name == &selected_name)
                        .map(|(id, _)| *id);

                    Message::PropertyChanged(
                        widget_id,
                        PropertyChange::ViewReferenceId(view_id, selected_name),
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
        // Extra view selections (generates a {Field}Selection enum when any exist)
        if !props.extra_view_ids.is_empty() {
            let extra_rows: Vec<Element<'_, Message>> = props
                .extra_view_ids
                .iter()
                .enumerate()
                .map(|(i, extra_id)| {
                    let selected_name = views.get(extra_id).map(|v| v.name.clone());
                    let view_options2: Vec<(Uuid, String)> =
                        views.iter().map(|(id, v)| (*id, v.name.clone())).collect();
                    row![
                        pick_list(
                            view_options2
                                .iter()
                                .map(|(_, name)| name.clone())
                                .collect::<Vec<_>>(),
                            selected_name,
                            move |n| {
                                let vid = view_options2
                                    .iter()
                                    .find(|(_, name)| name == &n)
                                    .map(|(id, _)| *id)
                                    .unwrap();
                                Message::PropertyChanged(
                                    widget_id,
                                    PropertyChange::SetExtraViewRef(i, vid),
                                )
                            }
                        )
                        .placeholder("Select view...")
                        .width(220),
                        button(text("×").size(12)).on_press(Message::PropertyChanged(
                            widget_id,
                            PropertyChange::RemoveExtraViewRef(i)
                        )),
                    ]
                    .spacing(4)
                    .align_y(Alignment::Center)
                    .into()
                })
                .collect();
            column(extra_rows).spacing(4)
        } else {
            column![]
        },
        button(text("+ Add view").size(12)).on_press(Message::PropertyChanged(
            widget_id,
            PropertyChange::AddExtraViewRef
        )),
        if !props.extra_view_ids.is_empty() {
            column![
                text("Generates a {Field}Selection enum.")
                    .size(LABEL_SIZE - 1.0)
                    .color(Color::from_rgb(0.5, 0.5, 0.5)),
                text("Use a SetState action to switch the active view.")
                    .size(LABEL_SIZE - 1.0)
                    .color(Color::from_rgb(0.5, 0.5, 0.5)),
            ]
            .spacing(2)
        } else {
            column![]
        },
    ]
    .spacing(MAIN_SPACING)
    .into();

    scrollable(container(add_code_preview(content, preview_content)).padding(padding::right(10.0)))
        .into()
}

pub fn icon_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    _theme: &Theme,
    _type_system: &'a TypeSystem,
    _custom_styles: &CustomThemes,
    preview_content: &'a text_editor::Content,
) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    // Filter the full icon list for the picker grid
    let filter = props.icon_picker_filter.to_lowercase();
    let icon_buttons: Vec<Element<'a, Message>> = icon_lucide::ALL_ICONS
        .iter()
        .filter(|(name, _)| filter.is_empty() || name.contains(filter.as_str()))
        .map(|(name, codepoint)| {
            let cp_u32 = codepoint.chars().next().map(|c| c as u32).unwrap_or(0xFFFD);
            let name_clone = name.to_string();
            tooltip(
                button(icon_lucide::render(codepoint))
                    .on_press(Message::PropertyChanged(
                        widget_id,
                        PropertyChange::IconSelected(name_clone, cp_u32),
                    ))
                    .style(button::text),
                container(text(*name))
                    .style(container::bordered_box)
                    .padding(5),
                tooltip::Position::Top,
            )
            .into()
        })
        .collect();

    let search = text_input("Search icons\u{2026}", &props.icon_picker_filter).on_input(move |v| {
        Message::PropertyChanged(widget_id, PropertyChange::IconPickerFilter(v))
    });

    let picker_content = column![
        container(search).padding(padding::horizontal(5.0)),
        scrollable(
            Row::with_children(icon_buttons)
                .spacing(4)
                .padding(10)
                .wrap(),
        ),
    ]
    .spacing(4);

    // Trigger button shows the current icon + its name
    let current_cp_str: &'static str = icon_lucide::ALL_ICONS
        .iter()
        .find(|(name, _)| *name == props.icon_name.as_str())
        .map(|(_, cp)| *cp)
        .unwrap_or("\u{FFFD}");

    let trigger = row![
        icon_lucide::render(current_cp_str).size(20),
        text(&props.icon_name),
    ]
    .spacing(6)
    .align_y(Alignment::Center);

    let picker = overlay_button(trigger, "Select Icon", picker_content)
        .overlay_width(650.0)
        .overlay_height(450.0)
        .hide_header()
        .close_on_click_outside()
        .hover_positions_on_click()
        .hover_position(widgets::generic_overlay::Position::Right)
        .hover_gap(5.0)
        .hover_alignment(Alignment::Start);

    let content = column![
        text("Icon Properties").size(TITLE_SIZE),
        widget_name(widget_id, &props.widget_name),
        column![text("Icon").size(LABEL_SIZE), picker,].spacing(LABEL_SPACING),
        column![
            text("Icon Size").size(LABEL_SIZE),
            row![
                slider(8.0..=128.0, props.icon_size, move |v| {
                    Message::PropertyChanged(widget_id, PropertyChange::IconSize(v))
                })
                .step(1.0)
                .width(200),
                text(format!("{:.0}px", props.icon_size))
                    .size(LABEL_SIZE)
                    .width(50),
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

    scrollable(container(add_code_preview(content, preview_content)).padding(padding::right(10.0)))
        .into()
}

fn parse_f32(s: &str, default: f32) -> f32 {
    s.trim().parse::<f32>().unwrap_or(default)
}

fn color_hex_input<'a, F>(label: &'a str, current: &'a str, on_change: F) -> Element<'a, Message>
where
    F: Fn(String) -> Message + 'a + Copy,
{
    //let cur = color_to_hex(current);
    column![
        text(label),
        text_input("#RRGGBB or #RRGGBBAA", &current)
            .on_input(move |s| on_change(s))
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
                    _ => &p.draft_fixed_height,
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
    let (can_fill, saved_value) =
        if let Some((_, scroll_dir)) = hierarchy.get_scrollable_ancestor_info(widget_id) {
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

            let blocked = if is_height {
                height_blocked
            } else {
                width_blocked
            };

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
        pick_list(available_choices, Some(choice_now), move |choice| {
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
        })
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
                        let p = v
                            .trim()
                            .parse::<u16>()
                            .ok()
                            .map(|x| x.max(1))
                            .unwrap_or(DEFAULT_PORTION);
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

    let (can_fill, saved_value) =
        if let Some((_, scroll_dir)) = hierarchy.get_scrollable_ancestor_info(widget_id) {
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

            let blocked = if is_height {
                height_blocked
            } else {
                width_blocked
            };

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
                text(format!("(was: {})", length_to_string(saved_value.unwrap()))).size(LABEL_SIZE)
            ]
        } else {
            column![text(label).size(LABEL_SIZE)]
        },
        pick_list(available_choices, Some(choice_now), move |choice| {
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
        })
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
                    .on_input(move |v| { on_draft_change(v) })
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
                    .on_input(move |v| { on_draft_change(v) })
                    .width(75)
            ]
            .spacing(LABEL_SPACING)
            .into()
        }
        _ => space::horizontal().into(),
    };

    row![picker, extra]
        .width(Length::Fill)
        .spacing(SECTION_SPACING)
        .into()
}

pub fn generic_overlay_size_picker<'a>(
    label: &'a str,
    current: Length,
    is_dynamic: bool,
    dynamic_factor: f32,
    fixed_draft: Option<&'a String>,
    fill_portion_draft: Option<&'a String>,
    dynamic_draft: Option<&'a String>,
    on_choice_change: impl Fn(GenericOverlayLengthChoice) -> Message + 'a + Copy,
    on_fixed_draft_change: impl Fn(String) -> Message + 'a + Copy,
    on_fill_portion_draft_change: impl Fn(String) -> Message + 'a + Copy,
    on_dynamic_draft_change: impl Fn(String) -> Message + 'a + Copy,
) -> Element<'a, Message> {
    const DEFAULT_PX: f32 = 120.0;
    const DEFAULT_PORTION: u16 = 1;

    let choice_now = GenericOverlayLengthChoice::from_state(current, is_dynamic);
    let available_choices = vec![
        GenericOverlayLengthChoice::Fill,
        GenericOverlayLengthChoice::FillPortion,
        GenericOverlayLengthChoice::Shrink,
        GenericOverlayLengthChoice::Fixed,
        GenericOverlayLengthChoice::Dynamic,
    ];

    let picker = column![
        text(label).size(LABEL_SIZE),
        pick_list(available_choices, Some(choice_now), move |choice| {
            on_choice_change(choice)
        })
        .width(160),
    ]
    .spacing(LABEL_SPACING)
    .width(Length::Shrink);

    let extra: Element<_> = match choice_now {
        GenericOverlayLengthChoice::Fixed => {
            let committed_value = match current {
                Length::Fixed(px) => format!("{px}"),
                _ => format!("{DEFAULT_PX}"),
            };
            let display_text = fixed_draft.map(|s| s.as_str()).unwrap_or("");

            column![
                text("Pixels").size(LABEL_SIZE),
                text_input(&committed_value, display_text)
                    .on_input(move |value| on_fixed_draft_change(value))
                    .width(75),
            ]
            .spacing(LABEL_SPACING)
            .into()
        }
        GenericOverlayLengthChoice::FillPortion => {
            let committed_value = match current {
                Length::FillPortion(portion) => portion.to_string(),
                _ => DEFAULT_PORTION.to_string(),
            };
            let display_text = fill_portion_draft.map(|s| s.as_str()).unwrap_or("");

            column![
                text("Portion").size(LABEL_SIZE),
                text_input(&committed_value, display_text)
                    .on_input(move |value| on_fill_portion_draft_change(value))
                    .width(75),
            ]
            .spacing(LABEL_SPACING)
            .into()
        }
        GenericOverlayLengthChoice::Dynamic => {
            let committed_value = format!("{dynamic_factor}");
            let display_text = dynamic_draft.map(|s| s.as_str()).unwrap_or("");

            column![
                text("Factor").size(LABEL_SIZE),
                text_input(&committed_value, display_text)
                    .on_input(move |value| on_dynamic_draft_change(value))
                    .width(75),
            ]
            .spacing(LABEL_SPACING)
            .into()
        }
        _ => space::horizontal().into(),
    };

    row![picker, extra]
        .width(Length::Fill)
        .spacing(SECTION_SPACING)
        .into()
}

pub fn padding_controls<'a>(
    current_padding: Padding,
    widget_id: WidgetId,
    padding_mode: PaddingMode,
    theme: &Theme,
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
                            Message::PropertyChanged(widget_id, PropertyChange::PaddingUniform(v))
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
                column![row![
                    row![
                        column![
                            text("Vertical (Top/Bottom)").size(LABEL_SIZE),
                            slider(0.0..=50.0, current_padding.top, move |v| {
                                Message::PropertyChanged(
                                    widget_id,
                                    PropertyChange::PaddingVertical(v),
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
                                    PropertyChange::PaddingHorizontal(v),
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
                ]]
                .spacing(SECTION_SPACING)
            }

            PaddingMode::Individual => {
                column![
                    row![
                        column![
                            text("Top").size(LABEL_SIZE),
                            slider(0.0..=50.0, current_padding.top, move |v| {
                                Message::PropertyChanged(widget_id, PropertyChange::PaddingTop(v))
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
                                Message::PropertyChanged(widget_id, PropertyChange::PaddingRight(v))
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
                                    PropertyChange::PaddingBottom(v),
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
                                Message::PropertyChanged(widget_id, PropertyChange::PaddingLeft(v))
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
        icon::info()
            .center()
            .size(14)
            .color(palette.background.stronger.color),
        container(text(info).size(12))
            .style(container::rounded_box)
            .padding([5, 10]),
        tooltip::Position::Top,
    )
    .into()
}

fn with_event_info<'a>(
    control: Element<'a, Message>,
    theme: &Theme,
    info: &'a str,
) -> Element<'a, Message> {
    row![control, information(theme, info)]
        .spacing(LABEL_SPACING)
        .align_y(Alignment::Center)
        .into()
}

fn event_name_row<'a>(theme: &Theme, name: &'a str, info: &'a str) -> Element<'a, Message> {
    with_event_info(text(name).size(LABEL_SIZE).into(), theme, info)
}

fn event_checkbox_row<'a, F>(
    theme: &Theme,
    enabled: bool,
    name: &'a str,
    info: &'a str,
    on_toggle: F,
) -> Element<'a, Message>
where
    F: 'a + Fn(bool) -> Message,
{
    with_event_info(
        checkbox(enabled).label(name).on_toggle(on_toggle).into(),
        theme,
        info,
    )
}

pub fn clip_control<'a>(widget_id: WidgetId, clipped: bool, theme: &Theme) -> Element<'a, Message> {
    row![
        checkbox(clipped,)
            .label("Enable Clipping")
            .on_toggle(move |v| Message::PropertyChanged(widget_id, PropertyChange::Clip(v))),
        information(
            theme,
            "When enabled, child content that exceeds bounds will be clipped"
        ),
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
                .on_toggle(move |enabled| Message::PropertyChanged(
                    widget_id,
                    PropertyChange::MaxWidth(if enabled { Some(800.0) } else { None })
                )),
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

pub fn max_height_control<'a>(
    widget_id: WidgetId,
    max_height: Option<f32>,
) -> Element<'a, Message> {
    column![
        row![
            checkbox(max_height.is_some())
                .label("Set max height")
                .on_toggle(move |enabled| Message::PropertyChanged(
                    widget_id,
                    PropertyChange::MaxHeight(if enabled { Some(800.0) } else { None })
                )),
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

pub fn widget_id_control<'a>(
    widget_id: WidgetId,
    id: Option<String>,
    theme: &Theme,
) -> Element<'a, Message> {
    let id_clone = id.clone();

    row![
        row![
            checkbox(id_clone.is_some())
                .label("Set Widget Id")
                .on_toggle(move |enabled| Message::PropertyChanged(
                    widget_id,
                    PropertyChange::WidgetId(if enabled { Some(String::new()) } else { None })
                )),
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

/// Collects state fields available for conditional style: (display_label, field_path, variants).
/// `variants` is non-empty for enum/bool fields (use a pick_list), empty means use text_input.
fn collect_condition_fields(
    root: &Widget,
    view_id: Uuid,
    views: &BTreeMap<Uuid, AppView>,
    type_system: &TypeSystem,
) -> Vec<(String, String, Vec<String>)> {
    let mut out = Vec::new();
    if let Some(view) = views.get(&view_id) {
        for field in &view.custom_state {
            out.push((
                format!("{} ({})", field.name, field.field_type.rust_type()),
                field.name.clone(),
                condition_variants_for_custom_field(&field.field_type, type_system),
            ));
        }
    }
    collect_condition_fields_rec(root, views, &mut out);
    out
}

fn collect_condition_fields_rec(
    widget: &Widget,
    views: &BTreeMap<Uuid, AppView>,
    out: &mut Vec<(String, String, Vec<String>)>,
) {
    if widget.widget_type == WidgetType::ViewReference {
        if let Some(ref_id) = widget.properties.referenced_view_id {
            if !widget.properties.extra_view_ids.is_empty() {
                if let Some(primary_view) = views.get(&ref_id) {
                    let field_base = if !widget.properties.widget_name.trim().is_empty() {
                        cond_snake_case(&widget.properties.widget_name)
                    } else {
                        cond_snake_case(&primary_view.name)
                    };
                    let sel_field = format!("{}_selection", field_base);
                    let type_name = format!("{}Selection", cond_pascal_case(&field_base));
                    let mut variants = vec![format!(
                        "{}::{}",
                        type_name,
                        cond_pascal_case(&cond_snake_case(&primary_view.name))
                    )];
                    for eid in &widget.properties.extra_view_ids {
                        if let Some(ev) = views.get(eid) {
                            variants.push(format!(
                                "{}::{}",
                                type_name,
                                cond_pascal_case(&cond_snake_case(&ev.name))
                            ));
                        }
                    }
                    out.push((
                        format!("{} ({})", sel_field, type_name),
                        sel_field,
                        variants,
                    ));
                }
            }
        }
    }
    for child in &widget.children {
        collect_condition_fields_rec(child, views, out);
    }
}

fn cond_snake_case(s: &str) -> String {
    s.chars()
        .enumerate()
        .flat_map(|(i, c)| {
            if c.is_uppercase() && i > 0 {
                vec!['_', c.to_lowercase().next().unwrap_or(c)]
            } else {
                vec![c.to_lowercase().next().unwrap_or(c)]
            }
        })
        .collect::<String>()
        .replace(' ', "_")
}

fn cond_pascal_case(s: &str) -> String {
    s.split(|c: char| c == '_' || c == ' ')
        .filter(|p| !p.is_empty())
        .map(|p| {
            let mut chars = p.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}

fn conditional_style_section<'a>(
    widget_id: WidgetId,
    props: &'a Properties,
    theme: &Theme,
    view_id: Uuid,
    root: &Widget,
    views: &'a BTreeMap<Uuid, AppView>,
    type_system: &TypeSystem,
) -> Element<'a, Message> {
    if props.active_style_name.is_none() {
        return column![].into();
    }

    let fields = collect_condition_fields(root, view_id, views, type_system);
    let field_options: Vec<String> = fields.iter().map(|(d, _, _)| d.clone()).collect();

    // Find which field is currently selected
    let selected_field_display: Option<String> =
        props.style_condition_field.as_ref().and_then(|f| {
            fields
                .iter()
                .find(|(_, path, _)| path == f)
                .map(|(d, _, _)| d.clone())
        });

    // Get value options for the selected field
    let value_variants: Vec<String> = props
        .style_condition_field
        .as_ref()
        .and_then(|f| fields.iter().find(|(_, path, _)| path == f))
        .map(|(_, _, v)| v.clone())
        .unwrap_or_default();

    let has_condition = props.style_condition_field.is_some();
    let condition_value = props.style_condition_value.clone().unwrap_or_default();

    let condition_body: Element<'a, Message> = if !has_condition {
        column![].into()
    } else if !field_options.is_empty() {
        let value_section: Element<'a, Message> = if !value_variants.is_empty() {
            column![
                text("When equals").size(LABEL_SIZE),
                pick_list(
                    value_variants,
                    props.style_condition_value.clone(),
                    move |v| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::StyleConditionValue(Some(v))
                    ),
                )
                .placeholder("select value"),
            ]
            .spacing(LABEL_SPACING)
            .into()
        } else {
            column![
                text("When equals").size(LABEL_SIZE),
                text_input("value", &condition_value)
                    .on_input(move |v| {
                        let val = if v.is_empty() { None } else { Some(v) };
                        Message::PropertyChanged(
                            widget_id,
                            PropertyChange::StyleConditionValue(val),
                        )
                    })
                    .size(LABEL_SIZE),
            ]
            .spacing(LABEL_SPACING)
            .into()
        };
        let fields_for_closure = fields.clone();
        column![
            text("State field").size(LABEL_SIZE),
            pick_list(field_options, selected_field_display, move |d: String| {
                // Map display label back to field path
                let path = fields_for_closure
                    .iter()
                    .find(|(display, _, _)| display == &d)
                    .map(|(_, p, _)| p.clone())
                    .unwrap_or(d);
                Message::PropertyChanged(widget_id, PropertyChange::StyleConditionField(Some(path)))
            },)
            .placeholder("select field"),
            value_section,
        ]
        .spacing(LABEL_SPACING)
        .into()
    } else {
        column![
            text("State field").size(LABEL_SIZE),
            text_input(
                "self.field_name",
                &props.style_condition_field.clone().unwrap_or_default()
            )
            .on_input(move |v| {
                let val = if v.is_empty() { None } else { Some(v) };
                Message::PropertyChanged(widget_id, PropertyChange::StyleConditionField(val))
            })
            .size(LABEL_SIZE),
            text("When equals").size(LABEL_SIZE),
            text_input("value", &condition_value)
                .on_input(move |v| {
                    let val = if v.is_empty() { None } else { Some(v) };
                    Message::PropertyChanged(widget_id, PropertyChange::StyleConditionValue(val))
                })
                .size(LABEL_SIZE),
        ]
        .spacing(LABEL_SPACING)
        .into()
    };

    column![
        with_event_info(
            checkbox(has_condition)
                .label("Use state condition")
                .on_toggle(move |checked| {
                    if checked {
                        Message::PropertyChanged(
                            widget_id,
                            PropertyChange::StyleConditionField(Some(String::new())),
                        )
                    } else {
                        Message::PropertyChanged(
                            widget_id,
                            PropertyChange::StyleConditionField(None),
                        )
                    }
                })
                .size(LABEL_SIZE)
                .into(),
            theme,
            "Use a view state field here, then update that state from the action system to switch to the alternate style.",
        ),
        condition_body,
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
    let style_options = style_options_for_widget_type(custom_themes, widget_type_enum);
    let supports_active_style = matches!(
        widget_type_enum,
        ThemePaneEnum::Container
            | ThemePaneEnum::Button
            | ThemePaneEnum::Checkbox
            | ThemePaneEnum::Picklist
            | ThemePaneEnum::Progressbar
            | ThemePaneEnum::Radio
            | ThemePaneEnum::Slider
            | ThemePaneEnum::Toggler
    );

    if style_options.len() < 1 {
        // Don't show picker if there are no styles
        return column![].into();
    }

    let mut content = column![
        text("Style").size(LABEL_SIZE),
        pick_list(
            style_options.clone(),
            props.custom_style_name.clone(),
            move |selection| {
                Message::PropertyChanged(
                    widget_id,
                    PropertyChange::CustomStyle(Some(selection.to_string())),
                )
            }
        )
        .placeholder("default"),
    ]
    .spacing(LABEL_SPACING);

    if supports_active_style {
        content = content.push(text("Alternate style").size(LABEL_SIZE)).push(
            pick_list(
                style_options,
                props.active_style_name.clone(),
                move |selection| {
                    Message::PropertyChanged(
                        widget_id,
                        PropertyChange::ActiveStyle(Some(selection.to_string())),
                    )
                },
            )
            .placeholder("none"),
        );
    }

    content.into()
}

fn condition_variants_for_custom_field(
    field_type: &CustomFieldType,
    type_system: &TypeSystem,
) -> Vec<String> {
    match field_type {
        CustomFieldType::Bool => vec!["true".to_string(), "false".to_string()],
        CustomFieldType::Enum(name) => type_system
            .get_enum_by_name(name)
            .map(|enum_def| {
                enum_def
                    .variants
                    .iter()
                    .map(|variant| format!("{}::{}", enum_def.name, variant.name))
                    .collect()
            })
            .unwrap_or_default(),
        _ => Vec::new(),
    }
}

fn style_options_for_widget_type(
    custom_themes: &CustomThemes,
    widget_type_enum: ThemePaneEnum,
) -> Vec<String> {
    let mut style_options: Vec<String> = custom_themes
        .styles()
        .get(&widget_type_enum)
        .map(|styles| styles.keys().cloned().collect())
        .unwrap_or_default();
    let push_style_option = |style_options: &mut Vec<String>, name: &str| {
        if !style_options.iter().any(|existing| existing == name) {
            style_options.push(name.to_string());
        }
    };

    match widget_type_enum {
        ThemePaneEnum::Container => {
            for name in ContainerStyleType::all() {
                push_style_option(&mut style_options, &name);
            }
        }
        ThemePaneEnum::Button => {
            for name in ButtonStyleType::all() {
                push_style_option(&mut style_options, &name);
            }
        }
        ThemePaneEnum::Checkbox => {
            for name in ["Primary", "Secondary", "Success", "Danger"] {
                push_style_option(&mut style_options, name);
            }
        }
        ThemePaneEnum::Picklist
        | ThemePaneEnum::Slider
        | ThemePaneEnum::Radio
        | ThemePaneEnum::Toggler => {
            push_style_option(&mut style_options, "Default");
        }
        ThemePaneEnum::Progressbar => {
            for name in ["Primary", "Secondary", "Success", "Warning", "Danger"] {
                push_style_option(&mut style_options, name);
            }
        }
        _ => {}
    }

    style_options
}

fn named_style_picker<'a>(
    custom_themes: &'a CustomThemes,
    widget_id: WidgetId,
    label: &'a str,
    selected_style: Option<String>,
    widget_type_enum: ThemePaneEnum,
    change: fn(Option<String>) -> PropertyChange,
) -> Element<'a, Message> {
    let style_options = style_options_for_widget_type(custom_themes, widget_type_enum);

    if style_options.is_empty() {
        return column![].into();
    }

    column![
        text(label).size(LABEL_SIZE),
        pick_list(style_options, selected_style, move |selection| {
            Message::PropertyChanged(widget_id, change(Some(selection)))
        })
        .placeholder("default"),
    ]
    .spacing(LABEL_SPACING)
    .into()
}

pub fn add_code_preview<'a>(
    content: Element<'a, Message>,
    preview_content: &'a text_editor::Content,
) -> Element<'a, Message> {
    use std::sync::Arc;
    use tree_sitter_highlighter::{
        TreeSitterIcedHighlighter, TsSettings, code_gen_text_editor_style,
    };

    let settings = TsSettings {
        text: Arc::<str>::from(preview_content.text().as_str()),
    };

    column![
        column![scrollable(content,),].height(400),
        container(rule::horizontal(2)).padding(padding::right(10)),
        text("Widget Code").size(16),
        text_editor(preview_content)
            .highlight_with::<TreeSitterIcedHighlighter>(
                settings,
                TreeSitterIcedHighlighter::to_format,
            )
            .on_action(Message::WidgetPreviewEdit)
            .style(code_gen_text_editor_style)
            .size(14.0),
    ]
    .padding(5)
    .spacing(10)
    .into()
}

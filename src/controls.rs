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
    Column, Row, Space, button, checkbox, column, container, pick_list, radio, row, rule, scrollable,
    slider, space, text, text_editor, text_input, toggler, tooltip,
};
use iced::advanced::graphics::core::length::{Bounds, Constraint, Sizing};
use iced::{Alignment, Color, Element, Length, Padding, Theme, padding};
use std::collections::BTreeMap;
use uuid::Uuid;
use widgets::anchor;
use widgets::popover::popover;

use crate::views::theme_and_stylefn_builder::{CustomThemes, ThemePaneEnum};
use crate::views::widget_tree::Message;
use control_styling::*;

/// Looks up an icon by the family and name stored in a project.
///
/// iced_lucide 0.2 renamed icons from `snake_case` to the upstream
/// `kebab-case`, so a name saved before that migration is normalised before
/// the lookup. Falls back to the replacement glyph when nothing matches.
pub fn stored_icon(family: &str, name: &str) -> icon_lucide::Icon {
    icon::find_in(family, name)
        .or_else(|| icon::find_in(family, &name.replace('_', "-")))
        .unwrap_or(icon_lucide::Icon {
            name: "",
            codepoint: "\u{FFFD}",
            font: iced::Font::DEFAULT,
            family: "",
        })
}

/// How many icons the picker grid draws before asking for a narrower search.
///
/// Every family is enabled, which is eighty thousand icons; Nerd Fonts alone
/// is eleven thousand. Building a button for each would stall the frame, and
/// nobody scrolls that far anyway.
const ICON_PICKER_LIMIT: usize = 400;

/// The icon picker: a family selector, a search box, and a grid of hits.
///
/// The three widgets that carry an icon — `text_input`, `combo_box`, and the
/// `icon` widget itself — each have their own property messages, so the two
/// choices a caller owns arrive as closures.
pub fn icon_picker<'a>(
    family: &'a str,
    name: &'a str,
    filter: &'a str,
    on_family: impl Fn(String) -> Message + 'a,
    on_filter: impl Fn(String) -> Message + 'a,
    on_select: impl Fn(String, String, u32) -> Message + 'a,
) -> Element<'a, Message> {
    let needle = filter.to_lowercase();

    let matches = icon::in_family(family)
        .filter(|icon| needle.is_empty() || icon.name.contains(needle.as_str()));

    let total = matches.clone().count();

    let icon_buttons: Vec<Element<'a, Message>> = matches
        .take(ICON_PICKER_LIMIT)
        .map(|icon| {
            tooltip(
                button(icon_lucide::render(*icon))
                    .on_press(on_select(
                        icon.family.to_string(),
                        icon.name.to_string(),
                        icon::codepoint_of(*icon),
                    ))
                    .style(button::text),
                container(text(icon.name))
                    .style(container::bordered_box)
                    .padding(5),
                tooltip::Position::Top,
            )
            .into()
        })
        .collect();

    let families: Vec<&'static str> = icon_lucide::FAMILIES
        .iter()
        .map(|family| family.name)
        .collect();

    let selected_family = icon_lucide::FAMILIES
        .iter()
        .find(|candidate| candidate.id == family)
        .map(|family| family.name);

    let family_picker = pick_list(selected_family, families, ToString::to_string)
        .on_select(move |label| {
            let id = icon_lucide::FAMILIES
                .iter()
                .find(|family| family.name == label)
                .map_or("lucide", |family| family.id);

            on_family(id.to_string())
        })
        .width(Length::Fill);

    let search = text_input("Search icons\u{2026}", filter).on_input(on_filter);

    // Saying how many were left out is the only cue that the grid is capped.
    let tally = if total > ICON_PICKER_LIMIT {
        format!("Showing {ICON_PICKER_LIMIT} of {total} — narrow the search")
    } else {
        format!("{total} icons")
    };

    let picker_content = column![
        container(column![family_picker, search].spacing(4))
            .padding(padding::horizontal(5.0).top(5.0)),
        container(text(tally).size(LABEL_SIZE)).padding(padding::horizontal(5.0)),
        scroll_y(
            Row::with_children(icon_buttons)
                .spacing(4)
                .padding(10)
                .wrap(),
        ),
    ]
    .spacing(4);

    let trigger = row![
        icon_lucide::render(stored_icon(family, name)).size(20),
        text(name),
    ]
    .spacing(6)
    .align_y(Alignment::Center);

    popover(trigger, container(picker_content).width(650).height(450))
        .side(anchor::Side::Right)
        .align(anchor::Align::Start)
        .gap(5.0)
        .max_width(650.0)
        .dismiss_on_outside_press(true)
        .into()
}

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
        section(
            "Sizing",
            column![
                field_grid(vec![
                    pick_field(
                        "Mode",
                        props.container_sizing_mode,
                        vec![
                            ContainerSizingMode::Manual,
                            ContainerSizingMode::CenterX,
                            ContainerSizingMode::CenterY,
                            ContainerSizingMode::Center,
                        ],
                        move |mode| Message::PropertyChanged(
                            widget_id,
                            PropertyChange::ContainerSizingMode(mode),
                        ),
                    ),
                    // The alignment pick lists belong to Manual mode: the
                    // centring modes do this themselves.
                    if matches!(props.container_sizing_mode, ContainerSizingMode::Manual) {
                        pick_field(
                            "Align X",
                            props.align_x,
                            vec![
                                ContainerAlignX::Left,
                                ContainerAlignX::Center,
                                ContainerAlignX::Right,
                            ],
                            move |v| Message::PropertyChanged(
                                widget_id,
                                PropertyChange::AlignX(v)
                            ),
                        )
                    } else {
                        nothing()
                    },
                    if matches!(props.container_sizing_mode, ContainerSizingMode::Manual) {
                        pick_field(
                            "Align Y",
                            props.align_y,
                            vec![
                                ContainerAlignY::Top,
                                ContainerAlignY::Center,
                                ContainerAlignY::Bottom,
                            ],
                            move |v| Message::PropertyChanged(
                                widget_id,
                                PropertyChange::AlignY(v)
                            ),
                        )
                    } else {
                        nothing()
                    },
                ]),
                text(match props.container_sizing_mode {
                    ContainerSizingMode::Manual => "Set width and height separately",
                    ContainerSizingMode::CenterX => "Set width and center content horizontally",
                    ContainerSizingMode::CenterY => "Set height and center content vertically",
                    ContainerSizingMode::Center =>
                        "Set size and center content in both directions",
                })
                .size(LABEL_SIZE - 1.0)
                .color(Color::from_rgb(0.5, 0.5, 0.5)),
            ]
            .spacing(LABEL_SPACING),
        ),
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
        // Padding Controls
        padding_controls(props.padding, widget_id, props.padding_mode, theme,),
        advanced_section(widget_id, props, theme),
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
        section(
            "Layout",
            field_grid(vec![
                field(
                    "Wrapping",
                    row![
                        toggler(props.is_wrapping_row).on_toggle(move |v| {
                            Message::PropertyChanged(widget_id, PropertyChange::IsWrappingRow(v))
                        }),
                        information(theme, "Items wrap onto a new line when the row runs out"),
                    ]
                    .spacing(LABEL_SPACING)
                    .align_y(Alignment::Center),
                ),
                slider_field("Spacing", 0.0..=50.0, props.spacing, move |v| {
                    Message::PropertyChanged(widget_id, PropertyChange::Spacing(v))
                }),
                // A wrapping row aligns its lines; a plain one aligns its
                // children across the axis it does not lay out along.
                if props.is_wrapping_row {
                    toggle_field(
                        "Match spacing",
                        props.match_horizontal_spacing,
                        move |use_same| Message::PropertyChanged(
                            widget_id,
                            PropertyChange::WrappingSpacingMatchToggle(use_same),
                        ),
                    )
                } else {
                    nothing()
                },
                if props.is_wrapping_row && props.match_horizontal_spacing {
                    slider_field(
                        "Line spacing",
                        0.0..=50.0,
                        props.wrapping_vertical_spacing,
                        move |v| Message::PropertyChanged(
                            widget_id,
                            PropertyChange::WrappingVerticalSpacing(v),
                        ),
                    )
                } else {
                    nothing()
                },
                if props.is_wrapping_row {
                    pick_field(
                        "Align lines",
                        props.wrapping_align_x,
                        vec![
                            ContainerAlignX::Left,
                            ContainerAlignX::Center,
                            ContainerAlignX::Right,
                        ],
                        move |align| Message::PropertyChanged(
                            widget_id,
                            PropertyChange::WrappingAlignX(align),
                        ),
                    )
                } else {
                    pick_field(
                        "Align children",
                        AlignmentXOption::from_alignment(props.align_items),
                        vec![
                            AlignmentXOption::Start,
                            AlignmentXOption::Center,
                            AlignmentXOption::End,
                        ],
                        move |sel| Message::PropertyChanged(
                            widget_id,
                            PropertyChange::AlignItems(sel.to_alignment()),
                        ),
                    )
                },
            ]),
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
        advanced_section(widget_id, props, theme),
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
        section(
            "Layout",
            field_grid(vec![
                slider_field("Spacing", 0.0..=50.0, props.spacing, move |v| {
                    Message::PropertyChanged(widget_id, PropertyChange::Spacing(v))
                }),
                pick_field(
                    "Align children",
                    AlignmentXOption::from_alignment(props.align_items),
                    vec![
                        AlignmentXOption::Start,
                        AlignmentXOption::Center,
                        AlignmentXOption::End,
                    ],
                    move |sel| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::AlignItems(sel.to_alignment()),
                    ),
                ),
            ]),
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
        advanced_section(widget_id, props, theme),
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
    let palette = theme.palette();

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
            rule::horizontal(1),
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
        advanced_section(widget_id, props, theme),
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
        section(
            "Text",
            field_grid(vec![
                text_field("Content", "Content", &props.text_content, move |v| {
                    Message::PropertyChanged(widget_id, PropertyChange::TextContent(v))
                }),
                slider_field("Font size", 8.0..=72.0, props.text_size, move |v| {
                    Message::PropertyChanged(widget_id, PropertyChange::TextSize(v))
                }),
                pick_field(
                    "Font",
                    props.font,
                    vec![FontType::Default, FontType::Monospace],
                    move |v| Message::PropertyChanged(widget_id, PropertyChange::Font(v)),
                ),
                field(
                    "Colour",
                    color_hex_input("", &props.draft_text_color, move |c| {
                        Message::PropertyChanged(widget_id, PropertyChange::DraftTextColor(c))
                    }),
                ),
                pick_field(
                    "Wrapping",
                    TextWrapping::from(props.wrap),
                    vec![
                        TextWrapping::None,
                        TextWrapping::Word,
                        TextWrapping::Glyph,
                        TextWrapping::WordOrGlyph,
                    ],
                    move |w| Message::PropertyChanged(widget_id, PropertyChange::TextWrap(w)),
                ),
                pick_field(
                    "Shaping",
                    TextShaping::from(props.shaping),
                    vec![TextShaping::Basic, TextShaping::Advanced, TextShaping::Auto],
                    move |s| Message::PropertyChanged(widget_id, PropertyChange::TextShaping(s)),
                ),
                // Relative line height runs in hundredths, which the shared
                // whole-number readout would round away.
                field(
                    "Line height",
                    row![
                        slider(
                            0.8..=2.0,
                            match props.line_height {
                                text::LineHeight::Relative(v) => v,
                                _ => 1.0,
                            },
                            move |v| Message::PropertyChanged(
                                widget_id,
                                PropertyChange::TextLineHeight(text::LineHeight::Relative(
                                    (v * 100.0).round() / 100.0,
                                )),
                            ),
                        )
                        .step(0.05)
                        .width(FIELD_WIDTH - 55.0),
                        text(match props.line_height {
                            text::LineHeight::Relative(v) => format!("{v:.2}"),
                            _ => "1.00".into(),
                        })
                        .size(LABEL_SIZE)
                        .width(50),
                    ]
                    .spacing(LABEL_SPACING)
                    .align_y(Alignment::Center),
                ),
                pick_field(
                    "Align X",
                    AlignText::from(props.text_align_x),
                    vec![
                        AlignText::Default,
                        AlignText::Left,
                        AlignText::Center,
                        AlignText::Right,
                        AlignText::Justified,
                    ],
                    move |a| Message::PropertyChanged(widget_id, PropertyChange::TextAlignX(a)),
                ),
                pick_field(
                    "Align Y",
                    AlignmentYOption::from(props.text_align_y),
                    vec![
                        AlignmentYOption::Top,
                        AlignmentYOption::Center,
                        AlignmentYOption::Bottom,
                    ],
                    move |a| Message::PropertyChanged(widget_id, PropertyChange::TextAlignY(a)),
                ),
            ]),
        ),
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
        section(
            "Field",
            field_grid(vec![
                text_field(
                    "Placeholder",
                    "Placeholder",
                    &props.text_input_placeholder,
                    move |v| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::TextInputPlaceholder(v),
                    ),
                ),
                slider_field("Font size", 8.0..=32.0, props.text_input_size, move |v| {
                    Message::PropertyChanged(widget_id, PropertyChange::TextInputSize(v))
                }),
                slider_field("Padding", 0.0..=30.0, props.text_input_padding, move |v| {
                    Message::PropertyChanged(widget_id, PropertyChange::TextInputPadding(v))
                }),
                pick_field(
                    "Font",
                    props.text_input_font,
                    vec![FontType::Default, FontType::Monospace],
                    move |v| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::TextInputFont(v.into()),
                    ),
                ),
                pick_field(
                    "Align X",
                    props.text_input_alignment,
                    vec![
                        ContainerAlignX::Left,
                        ContainerAlignX::Center,
                        ContainerAlignX::Right,
                    ],
                    move |v| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::TextInputAlignment(v),
                    ),
                ),
                toggle_field("Secure (password)", props.is_secure, move |v| {
                    Message::PropertyChanged(widget_id, PropertyChange::IsSecure(v))
                }),
            ]),
        ),
        column![
            text("Events").size(SECTION_SIZE),
            rule::horizontal(1),
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
            let picker = icon_picker(
                &props.text_input_icon_family,
                &props.text_input_icon_name,
                &props.text_input_icon_picker_filter,
                move |family| {
                    Message::PropertyChanged(
                        widget_id,
                        PropertyChange::TextInputIconFamily(family),
                    )
                },
                move |v| {
                    Message::PropertyChanged(
                        widget_id,
                        PropertyChange::TextInputIconPickerFilter(v),
                    )
                },
                move |family, name, codepoint| {
                    Message::PropertyChanged(
                        widget_id,
                        PropertyChange::TextInputIconSelected(family, name, codepoint),
                    )
                },
            );

            section(
                "Icon",
                column![
                    checkbox(props.text_input_icon_enabled)
                        .label("Show an icon")
                        .on_toggle(move |v| Message::PropertyChanged(
                            widget_id,
                            PropertyChange::TextInputIconEnabled(v)
                        )),
                    if props.text_input_icon_enabled {
                        field_grid(vec![
                            field("Icon", picker),
                            pick_field(
                                "Side",
                                props.text_input_icon_side,
                                vec![TextInputIconSide::Left, TextInputIconSide::Right],
                                move |v| Message::PropertyChanged(
                                    widget_id,
                                    PropertyChange::TextInputIconSide(v),
                                ),
                            ),
                            // Zero means "take the field's own size", which a
                            // bare number would not say.
                            field(
                                "Size",
                                row![
                                    slider(0.0..=64.0, props.text_input_icon_size, move |v| {
                                        Message::PropertyChanged(
                                            widget_id,
                                            PropertyChange::TextInputIconSize(v),
                                        )
                                    })
                                    .step(1.0)
                                    .width(FIELD_WIDTH - 55.0),
                                    text(if props.text_input_icon_size > 0.0 {
                                        format!("{:.0}", props.text_input_icon_size)
                                    } else {
                                        "auto".to_string()
                                    })
                                    .size(LABEL_SIZE)
                                    .width(50),
                                ]
                                .spacing(LABEL_SPACING)
                                .align_y(Alignment::Center),
                            ),
                            slider_field(
                                "Spacing",
                                0.0..=30.0,
                                props.text_input_icon_spacing,
                                move |v| Message::PropertyChanged(
                                    widget_id,
                                    PropertyChange::TextInputIconSpacing(v),
                                ),
                            ),
                        ])
                    } else {
                        nothing()
                    },
                ]
                .spacing(SECTION_SPACING),
            )
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

pub fn text_editor_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    theme: &Theme,
    _type_system: &'a TypeSystem,
    custom_styles: &'a CustomThemes,
    preview_content: &'a text_editor::Content,
    _views: &'a BTreeMap<Uuid, AppView>,
    _view_id: Uuid,
) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    let mut editor = text_editor(&props.text_editor_content)
        .placeholder(&props.text_editor_placeholder)
        .on_action(move |action| {
            Message::PropertyChanged(widget_id, PropertyChange::TextEditorContent(action))
        })
        .height(Length::Fixed(180.0))
        .width(350.0)
        .size(props.text_editor_size)
        .padding(props.text_editor_padding)
        .wrapping(props.text_editor_wrapping);

    if props.text_editor_font != FontType::Default {
        editor = editor.font(props.text_editor_font);
    }

    if props.text_editor_line_height != text::LineHeight::default() {
        editor = editor.line_height(props.text_editor_line_height);
    }

    let content = column![
        text("Text Editor Properties").size(TITLE_SIZE),
        widget_name(widget_id, &props.widget_name),
        custom_style_picker(custom_styles, widget_id, props, ThemePaneEnum::TextEditor),
        section(
            "Editor",
            column![
                field_grid(vec![
                    text_field(
                        "Placeholder",
                        "Placeholder",
                        &props.text_editor_placeholder,
                        move |value| Message::PropertyChanged(
                            widget_id,
                            PropertyChange::TextEditorPlaceholder(value),
                        ),
                    ),
                    slider_field("Font size", 8.0..=32.0, props.text_editor_size, move |value| {
                        Message::PropertyChanged(widget_id, PropertyChange::TextEditorSize(value))
                    }),
                    slider_field(
                        "Padding",
                        0.0..=30.0,
                        props.text_editor_padding,
                        move |value| Message::PropertyChanged(
                            widget_id,
                            PropertyChange::TextEditorPadding(value),
                        ),
                    ),
                    pick_field(
                        "Font",
                        props.text_editor_font,
                        vec![FontType::Default, FontType::Monospace],
                        move |font| Message::PropertyChanged(
                            widget_id,
                            PropertyChange::TextEditorFont(font),
                        ),
                    ),
                    pick_field(
                        "Wrapping",
                        TextWrapping::from(props.text_editor_wrapping),
                        vec![
                            TextWrapping::None,
                            TextWrapping::Word,
                            TextWrapping::Glyph,
                            TextWrapping::WordOrGlyph,
                        ],
                        move |wrapping| Message::PropertyChanged(
                            widget_id,
                            PropertyChange::TextEditorWrapping(wrapping),
                        ),
                    ),
                    // Relative line height runs in hundredths.
                    field(
                        "Line height",
                        row![
                            slider(
                                0.8..=2.0,
                                match props.text_editor_line_height {
                                    text::LineHeight::Relative(value) => value,
                                    _ => 1.0,
                                },
                                move |value| Message::PropertyChanged(
                                    widget_id,
                                    PropertyChange::TextEditorLineHeight(
                                        text::LineHeight::Relative(
                                            (value * 100.0).round() / 100.0,
                                        ),
                                    ),
                                ),
                            )
                            .step(0.05)
                            .width(FIELD_WIDTH - 55.0),
                            text(match props.text_editor_line_height {
                                text::LineHeight::Relative(value) => format!("{value:.2}"),
                                _ => "1.00".into(),
                            })
                            .size(LABEL_SIZE)
                            .width(50),
                        ]
                        .spacing(LABEL_SPACING)
                        .align_y(Alignment::Center),
                    ),
                ]),
                // The editor itself is full width: it is the content, not a
                // setting, and a grid cell would make it unusable.
                column![text("Content").size(LABEL_SIZE), editor].spacing(LABEL_SPACING),
            ]
            .spacing(SECTION_SPACING),
        ),
        column![
            text("Events").size(SECTION_SIZE),
            rule::horizontal(1),
            event_name_row(
                theme,
                "on_edit",
                "Fires when the editor content changes and exposes the full text as `text`.",
            ),
        ]
        .spacing(SECTION_SPACING),
        size_controls_scrollable_aware(
            props.width,
            move |length| Message::PropertyChanged(widget_id, PropertyChange::Width(length)),
            props.height,
            move |length| Message::PropertyChanged(widget_id, PropertyChange::Height(length)),
            h,
            widget_id,
        ),
        advanced_section(widget_id, props, theme),
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
        section(
            "Checkbox",
            field_grid(vec![
                text_field("Label", "Label", &props.checkbox_label, move |v| {
                    Message::PropertyChanged(widget_id, PropertyChange::CheckboxLabel(v))
                }),
                slider_field("Size", 12.0..=40.0, props.checkbox_size, move |v| {
                    Message::PropertyChanged(widget_id, PropertyChange::CheckboxSize(v))
                }),
                slider_field("Label spacing", 0.0..=30.0, props.checkbox_spacing, move |v| {
                    Message::PropertyChanged(widget_id, PropertyChange::CheckboxSpacing(v))
                }),
                toggle_field("Checked", props.checkbox_checked, move |v| {
                    Message::PropertyChanged(widget_id, PropertyChange::CheckboxChecked(v))
                }),
            ]),
        ),
        section(
            "Events",
            event_name_row(
                theme,
                "on_toggle",
                "Fires when the checkbox checked state changes",
            ),
        ),
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
        section(
            "Toggler",
            field_grid(vec![
                text_field("Label", "Label", &props.toggler_label, move |v| {
                    Message::PropertyChanged(widget_id, PropertyChange::TogglerLabel(v))
                }),
                slider_field("Size", 12.0..=40.0, props.toggler_size, move |v| {
                    Message::PropertyChanged(widget_id, PropertyChange::TogglerSize(v))
                }),
                slider_field("Label spacing", 0.0..=30.0, props.toggler_spacing, move |v| {
                    Message::PropertyChanged(widget_id, PropertyChange::TogglerSpacing(v))
                }),
                toggle_field("Active", props.toggler_active, move |v| {
                    Message::PropertyChanged(widget_id, PropertyChange::TogglerActive(v))
                }),
            ]),
        ),
        section(
            "Events",
            event_name_row(theme, "on_toggle", "Fires when the toggler switches on or off"),
        ),
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
        section(
            "Radio",
            field_grid(vec![
                text_field("Label", "Label", &props.radio_label, move |v| {
                    Message::PropertyChanged(widget_id, PropertyChange::RadioLabel(v))
                }),
                slider_field("Size", 12.0..=40.0, props.radio_size, move |v| {
                    Message::PropertyChanged(widget_id, PropertyChange::RadioSize(v))
                }),
                slider_field("Label spacing", 0.0..=30.0, props.radio_spacing, move |v| {
                    Message::PropertyChanged(widget_id, PropertyChange::RadioSpacing(v))
                }),
            ]),
        ),
        column![
            text("Options").size(SECTION_SIZE),
            rule::horizontal(1),
            column(
                props
                    .radio_options
                    .iter()
                    .enumerate()
                    .map(|(i, option)| {
                        let label = format!("Option {}", i + 1);
                        row![
                            text_input(label.clone(), option)
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
                props.radio_options.get(props.radio_selected_index).cloned(),
                props.radio_options.clone(),
                ToString::to_string,
            )
            .on_select(move |selected| {
                    let current = props.radio_options.clone();
                    if let Some(ix) = current.iter().position(|s| s == &selected) {
                        Message::PropertyChanged(widget_id, PropertyChange::RadioSelectedIndex(ix))
                    } else {
                        Message::PropertyChanged(widget_id, PropertyChange::RadioSelectedIndex(0))
                    }
                })
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
        section(
            "Pick list",
            field_grid(vec![
                text_field(
                    "Placeholder",
                    "Placeholder",
                    &props.picklist_placeholder,
                    move |v| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::PickListPlaceholder(v),
                    ),
                ),
                field(
                    "Default selection",
                    pick_list(
                        props.picklist_selected.clone(),
                        props.picklist_options.clone(),
                        ToString::to_string,
                    )
                    .on_select(move |selection| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::PickListSelected(Some(selection)),
                    ))
                    .width(FIELD_WIDTH),
                ),
            ]),
        ),
        column![
            text("Options").size(SECTION_SIZE),
            rule::horizontal(1),
            column(
                props
                    .picklist_options
                    .iter()
                    .enumerate()
                    .map(|(i, option)| {
                        row![
                            text_input(format!("Option {}", i + 1), option)
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
            rule::horizontal(1),
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
        section(
            "Range",
            field_grid(vec![
                text_field("Min", "min", min_str, move |s| {
                    let v = parse_f32(&s, props.slider_min);
                    Message::PropertyChanged(widget_id, PropertyChange::SliderMin(v))
                }),
                text_field("Max", "max", max_str, move |s| {
                    let v = parse_f32(&s, props.slider_max);
                    Message::PropertyChanged(widget_id, PropertyChange::SliderMax(v))
                }),
                text_field("Step", "step", step_str, move |s| {
                    let v = parse_f32(&s, props.slider_step.max(0.000_001));
                    Message::PropertyChanged(
                        widget_id,
                        PropertyChange::SliderStep(v.max(0.000_001)),
                    )
                }),
                // Not `slider_field`: the range and step are the project's, and
                // a value of 0.001 wants more than the whole numbers that
                // readout shows.
                field(
                    "Value",
                    row![
                        slider(
                            props.slider_min..=props.slider_max,
                            props.slider_value,
                            move |val| Message::PropertyChanged(
                                widget_id,
                                PropertyChange::SliderValue(val)
                            ),
                        )
                        .step(props.slider_step.max(0.000_001))
                        .width(FIELD_WIDTH - 55.0),
                        text(format!("{:.3}", props.slider_value))
                            .size(LABEL_SIZE)
                            .width(50),
                    ]
                    .spacing(LABEL_SPACING)
                    .align_y(Alignment::Center),
                ),
            ]),
        ),
        section(
            "Events",
            event_name_row(theme, "on_change", "Fires whenever the slider value changes"),
        ),
        section(
            "Size",
            column![
                length_picker_scrollable_aware(
                    "Width (Length)",
                    props.width,
                    move |l| Message::PropertyChanged(widget_id, PropertyChange::Width(l)),
                    hierarchy,
                    widget_id,
                    false
                ),
                field_grid(vec![text_field(
                    "Height (thickness)",
                    "px",
                    slider_height,
                    move |s| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::SliderHeight(parse_f32(&s, props.slider_height)),
                    ),
                )]),
            ]
            .spacing(SECTION_SPACING),
        ),
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
        section(
            "Range",
            field_grid(vec![
                text_field("Min", "min", min_str, move |s| {
                    let v = parse_f32(&s, props.slider_min);
                    Message::PropertyChanged(widget_id, PropertyChange::SliderMin(v))
                }),
                text_field("Max", "max", max_str, move |s| {
                    let v = parse_f32(&s, props.slider_max);
                    Message::PropertyChanged(widget_id, PropertyChange::SliderMax(v))
                }),
                text_field("Step", "step", step_str, move |s| {
                    let v = parse_f32(&s, props.slider_step.max(0.000_001));
                    Message::PropertyChanged(
                        widget_id,
                        PropertyChange::SliderStep(v.max(0.000_001)),
                    )
                }),
                field(
                    "Value",
                    row![
                        slider(
                            props.slider_min..=props.slider_max,
                            props.slider_value,
                            move |val| Message::PropertyChanged(
                                widget_id,
                                PropertyChange::SliderValue(val)
                            ),
                        )
                        .step(props.slider_step.max(0.000_001))
                        .width(FIELD_WIDTH - 55.0),
                        text(format!("{:.3}", props.slider_value))
                            .size(LABEL_SIZE)
                            .width(50),
                    ]
                    .spacing(LABEL_SPACING)
                    .align_y(Alignment::Center),
                ),
            ]),
        ),
        section(
            "Events",
            event_name_row(theme, "on_change", "Fires whenever the slider value changes"),
        ),
        section(
            "Size",
            column![
                length_picker_scrollable_aware(
                    "Height (Length)",
                    props.height,
                    move |l| Message::PropertyChanged(widget_id, PropertyChange::Height(l)),
                    hierarchy,
                    widget_id,
                    true
                ),
                field_grid(vec![text_field(
                    "Width (thickness)",
                    "px",
                    slider_width,
                    move |s| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::SliderWidth(parse_f32(&s, props.slider_width)),
                    ),
                )]),
            ]
            .spacing(SECTION_SPACING),
        ),
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
        section(
            "Rule",
            column![
                field_grid(vec![
                    pick_field(
                        "Orientation",
                        p.orientation,
                        vec![Orientation::Horizontal, Orientation::Vertical],
                        move |o| Message::PropertyChanged(
                            widget_id,
                            PropertyChange::Orientation(o)
                        ),
                    ),
                    // Half-pixel steps, so the shared whole-number readout of
                    // `slider_field` would not say enough.
                    field(
                        "Thickness",
                        row![
                            slider(0.5..=20.0, p.rule_thickness, move |v| {
                                Message::PropertyChanged(
                                    widget_id,
                                    PropertyChange::RuleThickness(v),
                                )
                            })
                            .step(0.5)
                            .width(FIELD_WIDTH - 55.0),
                            text(format!("{:.1}", p.rule_thickness))
                                .size(LABEL_SIZE)
                                .width(50),
                        ]
                        .spacing(LABEL_SPACING)
                        .align_y(Alignment::Center),
                    ),
                ]),
                column![
                    text("Quick presets").size(LABEL_SIZE),
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
                    .wrap(),
                ]
                .spacing(LABEL_SPACING),
            ]
            .spacing(SECTION_SPACING),
        ),
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
        section(
            "Scrolling",
            field_grid(vec![
                pick_field(
                    "Direction",
                    DirChoice::to_choice(props.scroll_dir),
                    vec![DirChoice::Vertical, DirChoice::Horizontal, DirChoice::Both],
                    move |c| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::ScrollableDirection(DirChoice::from_choice(c)),
                    ),
                ),
                pick_field(
                    "Anchor X",
                    AnchorChoice::from(props.anchor_x),
                    vec![AnchorChoice::Start, AnchorChoice::End],
                    move |a| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::ScrollableAnchorX(AnchorChoice::from_anchor(a)),
                    ),
                ),
                pick_field(
                    "Anchor Y",
                    AnchorChoice::from(props.anchor_y),
                    vec![AnchorChoice::Start, AnchorChoice::End],
                    move |a| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::ScrollableAnchorY(AnchorChoice::from_anchor(a)),
                    ),
                ),
            ]),
        ),
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
        section(
            "Space",
            field_grid(vec![pick_field(
                "Orientation",
                props.orientation,
                vec![Orientation::Horizontal, Orientation::Vertical],
                move |o| Message::PropertyChanged(widget_id, PropertyChange::Orientation(o)),
            )]),
        ),
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
        section(
            "Bar",
            column![
                field_grid(vec![
                    toggle_field("Vertical", p.progress_vertical, move |vertical| {
                        Message::PropertyChanged(
                            widget_id,
                            PropertyChange::ProgressVertical(vertical),
                        )
                    }),
                    // Girth is the axis the bar does *not* run along, so which
                    // dimension it is follows the orientation.
                    text_field(
                        if p.progress_vertical {
                            "Girth (width)"
                        } else {
                            "Girth (height)"
                        },
                        "px",
                        girth_str,
                        move |s| Message::PropertyChanged(
                            widget_id,
                            PropertyChange::ProgressGirth(parse_f32(&s, p.progress_girth)),
                        ),
                    ),
                ]),
                length_picker_scrollable_aware(
                    "Length",
                    p.progress_length,
                    move |len| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::ProgressLength(len)
                    ),
                    h,
                    widget_id,
                    p.progress_vertical,
                ),
            ]
            .spacing(SECTION_SPACING),
        ),
        section(
            "Range",
            field_grid(vec![
                text_field("Min", "min", format!("{}", p.progress_min), move |s| {
                    let v = s.trim().parse::<f32>().unwrap_or(p.progress_min);
                    Message::PropertyChanged(widget_id, PropertyChange::ProgressMin(v))
                }),
                text_field("Max", "max", format!("{}", p.progress_max), move |s| {
                    let v = s.trim().parse::<f32>().unwrap_or(p.progress_max);
                    Message::PropertyChanged(widget_id, PropertyChange::ProgressMax(v))
                }),
                field(
                    "Value",
                    row![
                        slider(p.progress_min..=p.progress_max, p.progress_value, move |v| {
                            Message::PropertyChanged(widget_id, PropertyChange::ProgressValue(v))
                        })
                        .step(clamp_step)
                        .width(FIELD_WIDTH - 55.0),
                        text(format!("{:.02}", p.progress_value))
                            .size(LABEL_SIZE)
                            .width(50),
                    ]
                    .spacing(LABEL_SPACING)
                    .align_y(Alignment::Center),
                ),
            ]),
        ),
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
        section(
            "Image",
            field_grid(vec![
                text_field("Path", "assets/pic.png", &props.image_path, move |s| {
                    Message::PropertyChanged(widget_id, PropertyChange::ImagePath(s))
                }),
                pick_field("Fit", props.image_fit, ContentFitChoice::ALL, move |v| {
                    Message::PropertyChanged(widget_id, PropertyChange::ImageFit(v))
                }),
            ]),
        ),
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
        section(
            "SVG",
            field_grid(vec![
                text_field("Path", "assets/icon.svg", &props.svg_path, move |s| {
                    Message::PropertyChanged(widget_id, PropertyChange::SvgPath(s))
                }),
                pick_field("Fit", props.svg_fit, ContentFitChoice::ALL, move |v| {
                    Message::PropertyChanged(widget_id, PropertyChange::SvgFit(v))
                }),
            ]),
        ),
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
        section(
            "Tooltip",
            column![
                field_grid(vec![
                    text_field("Text", "Tooltip text", &p.tooltip_text, move |s| {
                        Message::PropertyChanged(widget_id, PropertyChange::TooltipText(s))
                    }),
                    pick_field(
                        "Position",
                        p.tooltip_position,
                        vec![
                            TooltipPosition::Top,
                            TooltipPosition::Bottom,
                            TooltipPosition::Left,
                            TooltipPosition::Right,
                        ],
                        move |pos| Message::PropertyChanged(
                            widget_id,
                            PropertyChange::TooltipPosition(pos)
                        ),
                    ),
                ]),
                text(
                    "Tooltip wraps two children, added under it in the tree: the \n                     first is what you hover, the second is what it shows."
                )
                .size(LABEL_SIZE)
                .color(Color::from_rgb(0.6, 0.6, 0.6)),
            ]
            .spacing(LABEL_SPACING),
        ),
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
        section(
            "Combo box",
            field_grid(vec![
                text_field(
                    "Placeholder",
                    "Placeholder",
                    &props.combobox_placeholder,
                    move |v| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::ComboBoxPlaceholder(v),
                    ),
                ),
                slider_field("Font size", 8.0..=32.0, props.combobox_size, move |v| {
                    Message::PropertyChanged(widget_id, PropertyChange::ComboBoxSize(v))
                }),
            ]),
        ),
        column![
            text("Data Source").size(SECTION_SIZE),
            rule::horizontal(1),
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
                            pick_list(
                                Some(selected),
                                type_system.enum_names(),
                                ToString::to_string,
                            )
                            .on_select(move |enum_name| {
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
                            scroll_y(
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
                rule::horizontal(1),
                column(
                    props
                        .combobox_options
                        .iter()
                        .enumerate()
                        .map(|(i, option)| {
                            row![
                                text_input(format!("Option {}", i + 1), option)
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
            rule::horizontal(1),
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
            let picker = icon_picker(
                &props.combobox_icon_family,
                &props.combobox_icon_name,
                &props.combobox_icon_picker_filter,
                move |family| {
                    Message::PropertyChanged(widget_id, PropertyChange::ComboBoxIconFamily(family))
                },
                move |v| {
                    Message::PropertyChanged(widget_id, PropertyChange::ComboBoxIconPickerFilter(v))
                },
                move |family, name, codepoint| {
                    Message::PropertyChanged(
                        widget_id,
                        PropertyChange::ComboBoxIconSelected(family, name, codepoint),
                    )
                },
            );

            section(
                "Icon",
                column![
                    checkbox(props.combobox_icon_enabled)
                        .label("Show an icon")
                        .on_toggle(move |v| Message::PropertyChanged(
                            widget_id,
                            PropertyChange::ComboBoxIconEnabled(v)
                        )),
                    if props.combobox_icon_enabled {
                        field_grid(vec![
                            field("Icon", picker),
                            pick_field(
                                "Side",
                                props.combobox_icon_side,
                                vec![TextInputIconSide::Left, TextInputIconSide::Right],
                                move |v| Message::PropertyChanged(
                                    widget_id,
                                    PropertyChange::ComboBoxIconSide(v),
                                ),
                            ),
                            // Zero means "take the field's own size", which a
                            // bare number would not say.
                            field(
                                "Size",
                                row![
                                    slider(0.0..=64.0, props.combobox_icon_size, move |v| {
                                        Message::PropertyChanged(
                                            widget_id,
                                            PropertyChange::ComboBoxIconSize(v),
                                        )
                                    })
                                    .step(1.0)
                                    .width(FIELD_WIDTH - 55.0),
                                    text(if props.combobox_icon_size > 0.0 {
                                        format!("{:.0}", props.combobox_icon_size)
                                    } else {
                                        "auto".to_string()
                                    })
                                    .size(LABEL_SIZE)
                                    .width(50),
                                ]
                                .spacing(LABEL_SPACING)
                                .align_y(Alignment::Center),
                            ),
                            slider_field(
                                "Spacing",
                                0.0..=30.0,
                                props.combobox_icon_spacing,
                                move |v| Message::PropertyChanged(
                                    widget_id,
                                    PropertyChange::ComboBoxIconSpacing(v),
                                ),
                            ),
                        ])
                    } else {
                        nothing()
                    },
                ]
                .spacing(SECTION_SPACING),
            )
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
        section(
            "Markdown",
            field_grid(vec![slider_field(
                "Text size",
                8.0..=32.0,
                props.markdown_text_size,
                move |v| Message::PropertyChanged(
                    widget_id,
                    PropertyChange::MarkdownTextSize(v)
                ),
            )]),
        ),
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
        section(
            "QR Code",
            field_grid(vec![
                text_field("Data", "Data", &props.qrcode_link, move |v| {
                    Message::PropertyChanged(widget_id, PropertyChange::QRCodeData(v))
                }),
                slider_field("Cell size", 1.0..=20.0, props.qrcode_cell_size, move |v| {
                    Message::PropertyChanged(widget_id, PropertyChange::QRCodeCellSize(v))
                }),
            ]),
        ),
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
        section(
            "Stack",
            text(
                "Stack overlays its children on top of each other: the first \n                 child is at the bottom, the last is on top."
            )
            .size(LABEL_SIZE)
            .color(Color::from_rgb(0.6, 0.6, 0.6)),
        ),
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

fn overlay_style_options() -> Vec<String> {
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
        section(
            "Header",
            field_grid(vec![
                text_field("Title", "Collapsible", &props.collapsible_title, move |value| {
                    Message::PropertyChanged(widget_id, PropertyChange::CollapsibleTitle(value))
                }),
                slider_field(
                    "Height",
                    24.0..=80.0,
                    props.collapsible_header_height,
                    move |value| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::CollapsibleHeaderHeight(value),
                    ),
                ),
                pick_field(
                    "Title alignment",
                    props.align_x,
                    vec![
                        ContainerAlignX::Left,
                        ContainerAlignX::Center,
                        ContainerAlignX::Right,
                    ],
                    move |value| Message::PropertyChanged(widget_id, PropertyChange::AlignX(value)),
                ),
                slider_field("Title size", 8.0..=32.0, props.text_size, move |value| {
                    Message::PropertyChanged(widget_id, PropertyChange::TextSize(value))
                }),
                pick_field(
                    "Title font",
                    props.font,
                    vec![FontType::Default, FontType::Monospace],
                    move |value| Message::PropertyChanged(widget_id, PropertyChange::Font(value)),
                ),
                toggle_field(
                    "Header clickable",
                    props.collapsible_header_clickable,
                    move |value| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::CollapsibleHeaderClickable(value),
                    ),
                ),
                toggle_field("Start expanded", props.collapsible_expanded, move |value| {
                    Message::PropertyChanged(
                        widget_id,
                        PropertyChange::CollapsibleExpanded(value),
                    )
                }),
            ]),
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
        section(
            "Style",
            field_grid(vec![
                field(
                    "Style",
                    pick_list(
                        props.custom_style_name.clone(),
                        style_options.clone(),
                        ToString::to_string,
                    )
                    .on_select(move |selection| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::CustomStyle(Some(selection)),
                    ))
                    .placeholder("default")
                    .width(FIELD_WIDTH),
                ),
                field(
                    "Alternate style",
                    pick_list(
                        props.active_style_name.clone(),
                        style_options,
                        ToString::to_string,
                    )
                    .on_select(move |selection| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::ActiveStyle(Some(selection)),
                    ))
                    .placeholder("none")
                    .width(FIELD_WIDTH),
                ),
            ]),
        ),
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
        section(
            "Group",
            column![
                text(
                    "An accordion of Collapsible children; it accepts no other \n                     kind of child."
                )
                .size(LABEL_SIZE)
                .color(Color::from_rgb(0.6, 0.6, 0.6)),
                field_grid(vec![slider_field(
                    "Spacing",
                    0.0..=32.0,
                    props.spacing,
                    move |value| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::Spacing(value)
                    ),
                )]),
            ]
            .spacing(LABEL_SPACING),
        ),
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

pub fn overlay_controls<'a>(
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

    // The four widgets that replaced `generic_overlay` share this panel, but
    // not its shape: each composes its own sections below, grouped the way that
    // widget reads rather than forced into one common set. A Layer in
    // particular has a title bar, a backdrop and drag/resize handles that the
    // others have nothing to do with.
    let kind = widget.widget_type;
    let is_popover = kind == WidgetType::Popover;
    let is_menu = kind == WidgetType::Menu;
    let is_layer = kind == WidgetType::Layer;
    let is_toast = kind == WidgetType::Toast;
    let is_anchored = is_popover || is_menu;
    let has_trigger_child = widget.children.first().is_some();

    let (title, blurb) = match kind {
        WidgetType::Popover => (
            "Popover",
            "Child 1 is the trigger, child 2 is the panel it opens.",
        ),
        WidgetType::Menu => (
            "Menu",
            "Child 1 is the trigger. Every child after it is one row of the panel.",
        ),
        WidgetType::Layer => (
            "Layer",
            "The single child is the panel content. A layer is positioned against \
             the window rather than a trigger, so the application decides when it \
             is open.",
        ),
        _ => (
            "Toasts",
            "Each child is one notification. The stack draws over the window, so \
             the application decides when it is showing.",
        ),
    };

    // ── Header, with the preset picker ───────────────────────────────────────
    let presets = OverlayPreset::all_for(kind);

    let preset_picker: Element<'a, Message> = if presets.is_empty() {
        nothing()
    } else {
        field(
            "Start From",
            pick_list(None::<OverlayPreset>, presets, ToString::to_string)
                .on_select(move |preset| {
                    Message::PropertyChanged(
                        widget_id,
                        PropertyChange::ApplyOverlayPreset(preset),
                    )
                })
                .placeholder("preset…")
                .width(FIELD_WIDTH),
        )
    };

    let header = column![
        row![
            column![
                text(title).size(TITLE_SIZE),
                text(blurb)
                    .size(LABEL_SIZE)
                    .color(Color::from_rgb(0.6, 0.6, 0.6)),
            ]
            .spacing(LABEL_SPACING)
            .width(Length::Fill),
            preset_picker,
        ]
        .spacing(SECTION_SPACING)
        .align_y(Alignment::Start),
        widget_name(widget_id, &props.widget_name),
        // All four take an `id`, and the generated code emits one when it is
        // set, so the panel has to be able to set it.
        widget_id_control(widget_id, props.widget_id.clone(), theme),
    ]
    .spacing(SECTION_SPACING);

    // ── Appearance, shared by all four ───────────────────────────────────────
    let mut appearance = vec![
        field(
            "Panel Style",
            pick_list(
                props.overlay_style_name.clone(),
                overlay_style_options(),
                ToString::to_string,
            )
            .on_select(move |selection| {
                Message::PropertyChanged(widget_id, PropertyChange::OverlayStyle(Some(selection)))
            })
            .placeholder("default")
            .width(FIELD_WIDTH),
        ),
        field(
            "Padding",
            float_input(props.overlay_padding, move |value| {
                Message::PropertyChanged(widget_id, PropertyChange::OverlayPadding(value))
            }),
        ),
        field(
            "Corner Radius",
            float_input(props.overlay_radius, move |value| {
                Message::PropertyChanged(widget_id, PropertyChange::OverlayRadius(value))
            }),
        ),
    ];

    // ── Size ─────────────────────────────────────────────────────────────────
    // A toast stack gives every toast in it one fixed width; the other three
    // size themselves to their contents between bounds.
    let mut size = Vec::new();

    if is_toast {
        appearance.push(field(
            "Toast Width",
            float_input(props.toast_width, move |value| {
                Message::PropertyChanged(widget_id, PropertyChange::ToastWidth(value))
            }),
        ));
    } else {
        size.push(field(
            "Min Width",
            float_input(props.overlay_min_width, move |value| {
                Message::PropertyChanged(widget_id, PropertyChange::OverlayMinWidth(value))
            }),
        ));
        size.push(toggle_field(
            "Limit width",
            props.overlay_max_width.is_some(),
            move |value| {
                Message::PropertyChanged(
                    widget_id,
                    PropertyChange::OverlayMaxWidthEnabled(value),
                )
            },
        ));

        if let Some(max) = props.overlay_max_width {
            size.push(field(
                "Max Width",
                float_input(max, move |value| {
                    Message::PropertyChanged(widget_id, PropertyChange::OverlayMaxWidth(value))
                }),
            ));
        }

        if is_layer {
            size.push(field(
                "Min Height",
                float_input(props.layer_min_height, move |value| {
                    Message::PropertyChanged(widget_id, PropertyChange::LayerMinHeight(value))
                }),
            ));
        }
    }

    // ── Placement ────────────────────────────────────────────────────────────
    let mut placement: Vec<Element<'a, Message>> = Vec::new();

    if is_anchored {
        placement.push(field(
            "Side",
            pick_list(
                Some(props.overlay_hover_position),
                vec![
                    OverlayPosition::Top,
                    OverlayPosition::Bottom,
                    OverlayPosition::Left,
                    OverlayPosition::Right,
                ],
                ToString::to_string,
            )
            .on_select(move |value| {
                Message::PropertyChanged(widget_id, PropertyChange::OverlayHoverPosition(value))
            })
            .width(FIELD_WIDTH),
        ));
        placement.push(field(
            "Align",
            pick_list(
                Some(props.overlay_hover_alignment),
                vec![
                    ContainerAlignX::Left,
                    ContainerAlignX::Center,
                    ContainerAlignX::Right,
                ],
                ToString::to_string,
            )
            .on_select(move |value| {
                Message::PropertyChanged(widget_id, PropertyChange::OverlayHoverAlignment(value))
            })
            .width(FIELD_WIDTH),
        ));
        placement.push(field(
            "Gap",
            float_input(props.overlay_hover_gap, move |value| {
                Message::PropertyChanged(widget_id, PropertyChange::OverlayHoverGap(value))
            }),
        ));
        placement.push(field(
            "Overlap",
            pick_list(
                Some(props.overlay_hover_mode),
                vec![OverlayPositionMode::Outside, OverlayPositionMode::Inside],
                ToString::to_string,
            )
            .on_select(move |value| {
                Message::PropertyChanged(widget_id, PropertyChange::OverlayHoverMode(value))
            })
            .width(FIELD_WIDTH),
        ));
        placement.push(toggle_field(
            "Flip to fit",
            props.overlay_hover_snap,
            move |value| {
                Message::PropertyChanged(widget_id, PropertyChange::OverlayHoverSnap(value))
            },
        ));
    }

    if is_layer {
        // An exact point overrides the anchor, so the two are never both live.
        if props.layer_position_enabled {
            placement.push(field(
                "X",
                float_input(props.layer_position_x, move |value| {
                    Message::PropertyChanged(widget_id, PropertyChange::LayerPositionX(value))
                }),
            ));
            placement.push(field(
                "Y",
                float_input(props.layer_position_y, move |value| {
                    Message::PropertyChanged(widget_id, PropertyChange::LayerPositionY(value))
                }),
            ));
        } else {
            placement.push(field(
                "Anchor",
                pick_list(
                    Some(props.layer_anchor),
                    OverlayAnchor::ALL,
                    ToString::to_string,
                )
                .on_select(move |value| {
                    Message::PropertyChanged(widget_id, PropertyChange::LayerAnchor(value))
                })
                .width(FIELD_WIDTH),
            ));
            placement.push(field(
                "Margin",
                float_input(props.overlay_hover_gap, move |value| {
                    Message::PropertyChanged(widget_id, PropertyChange::OverlayHoverGap(value))
                }),
            ));
            placement.push(toggle_field(
                "Stretch to edge",
                props.layer_stretch,
                move |value| {
                    Message::PropertyChanged(widget_id, PropertyChange::LayerStretch(value))
                },
            ));
        }

        placement.push(toggle_field(
            "Exact point",
            props.layer_position_enabled,
            move |value| {
                Message::PropertyChanged(widget_id, PropertyChange::LayerPositionEnabled(value))
            },
        ));
    }

    if is_toast {
        placement.push(field(
            "Corner",
            pick_list(
                Some(props.toast_anchor),
                OverlayAnchor::ALL,
                ToString::to_string,
            )
            .on_select(move |value| {
                Message::PropertyChanged(widget_id, PropertyChange::ToastAnchor(value))
            })
            .width(FIELD_WIDTH),
        ));
        placement.push(field(
            "Margin",
            float_input(props.toast_margin, move |value| {
                Message::PropertyChanged(widget_id, PropertyChange::ToastMargin(value))
            }),
        ));
        placement.push(field(
            "Gap Between",
            float_input(props.toast_spacing, move |value| {
                Message::PropertyChanged(widget_id, PropertyChange::ToastSpacing(value))
            }),
        ));
    }

    // ── Window chrome, Layer only ────────────────────────────────────────────
    // The title bar, the handles you move and resize it by: the things that
    // make a layer feel like a window rather than a panel.
    let mut window = Vec::new();

    if is_layer {
        window.push(toggle_field(
            "Title bar",
            !props.overlay_hide_header,
            move |value| {
                Message::PropertyChanged(widget_id, PropertyChange::OverlayHideHeader(!value))
            },
        ));

        // Nothing else in this group exists without the bar itself.
        if !props.overlay_hide_header {
            window.push(field(
                "Title",
                text_input("Overlay", &props.overlay_title)
                    .on_input(move |value| {
                        Message::PropertyChanged(widget_id, PropertyChange::OverlayTitle(value))
                    })
                    .width(FIELD_WIDTH),
            ));
            window.push(toggle_field(
                "Close button",
                !props.overlay_hide_close_button,
                move |value| {
                    Message::PropertyChanged(
                        widget_id,
                        PropertyChange::OverlayHideCloseButton(!value),
                    )
                },
            ));
        }

        window.push(toggle_field(
            "Draggable",
            !props.overlay_block_dragging,
            move |value| {
                Message::PropertyChanged(widget_id, PropertyChange::OverlayBlockDragging(!value))
            },
        ));
        window.push(field(
            "Resize",
            pick_list(
                Some(props.overlay_resizable),
                vec![
                    OverlayResizeMode::None,
                    OverlayResizeMode::Always,
                    OverlayResizeMode::WithCtrl,
                ],
                ToString::to_string,
            )
            .on_select(move |value| {
                Message::PropertyChanged(widget_id, PropertyChange::OverlayResizable(value))
            })
            .width(FIELD_WIDTH),
        ));
    }

    // ── Backdrop and dismissal ───────────────────────────────────────────────
    let mut dismissal = Vec::new();

    if is_layer {
        dismissal.push(toggle_field(
            "Dim the page",
            props.overlay_opaque,
            move |value| Message::PropertyChanged(widget_id, PropertyChange::OverlayOpaque(value)),
        ));

        // The alpha only reaches the screen when there is a backdrop for it.
        if props.overlay_opaque {
            dismissal.push(field(
                "Dim Amount",
                slider(0.1..=1.0, props.overlay_opaque_alpha, move |value| {
                    Message::PropertyChanged(widget_id, PropertyChange::OverlayOpaqueAlpha(value))
                })
                .step(0.05)
                .width(FIELD_WIDTH),
            ));
        }

        dismissal.push(toggle_field(
            "Close on backdrop",
            props.overlay_close_on_click_outside,
            move |value| {
                Message::PropertyChanged(
                    widget_id,
                    PropertyChange::OverlayCloseOnClickOutside(value),
                )
            },
        ));
        dismissal.push(toggle_field(
            "Close on Escape",
            props.layer_dismiss_on_escape,
            move |value| {
                Message::PropertyChanged(widget_id, PropertyChange::LayerDismissOnEscape(value))
            },
        ));
    } else if is_popover {
        dismissal.push(toggle_field(
            "Close on outside press",
            props.overlay_close_on_click_outside,
            move |value| {
                Message::PropertyChanged(
                    widget_id,
                    PropertyChange::OverlayCloseOnClickOutside(value),
                )
            },
        ));
    }

    // ── Menu panel ───────────────────────────────────────────────────────────
    let menu_panel: Element<'a, Message> = if is_menu {
        section(
            "Rows",
            column![
                field_grid(vec![
                    field(
                        "Submenu Gap",
                        float_input(props.menu_submenu_gap, move |value| {
                            Message::PropertyChanged(
                                widget_id,
                                PropertyChange::MenuSubmenuGap(value),
                            )
                        }),
                    ),
                    field(
                        "Gutter Spacing",
                        float_input(props.menu_gutter_spacing, move |value| {
                            Message::PropertyChanged(
                                widget_id,
                                PropertyChange::MenuGutterSpacing(value),
                            )
                        }),
                    ),
                    field(
                        "Check Side",
                        pick_list(
                            Some(props.menu_check_side),
                            MenuCheckSide::ALL,
                            ToString::to_string
                        )
                        .on_select(move |value| Message::PropertyChanged(
                            widget_id,
                            PropertyChange::MenuCheckSide(value)
                        ))
                        .width(FIELD_WIDTH),
                    ),
                ]),
                padding_field("Row Padding", props.menu_item_padding, move |padding| {
                    Message::PropertyChanged(widget_id, PropertyChange::MenuItemPadding(padding))
                }),
            ]
            .spacing(SECTION_SPACING),
        )
    } else {
        nothing()
    };

    // ── Trigger ──────────────────────────────────────────────────────────────
    // Without a trigger child the widget draws a plain button in its place, and
    // the widget's own size, padding and style describe that button.
    let trigger: Element<'a, Message> = if !is_anchored {
        nothing()
    } else {
        let mut behaviour = Vec::new();

        if is_popover {
            behaviour.push(toggle_field(
                "Open on hover",
                props.overlay_on_hover,
                move |value| {
                    Message::PropertyChanged(widget_id, PropertyChange::OverlayOnHover(value))
                },
            ));

            // The corridor is the forgiving path between trigger and surface,
            // so it only means anything when hovering is what opens it.
            if props.overlay_on_hover {
                behaviour.push(toggle_field(
                    "Safe corridor",
                    props.overlay_safe_triangle,
                    move |value| {
                        Message::PropertyChanged(
                            widget_id,
                            PropertyChange::OverlaySafeTriangle(value),
                        )
                    },
                ));
            }
        }

        if is_menu {
            behaviour.push(toggle_field(
                "Plain trigger",
                props.menu_plain_trigger,
                move |value| {
                    Message::PropertyChanged(widget_id, PropertyChange::MenuPlainTrigger(value))
                },
            ));
        }

        let body: Element<'a, Message> = if has_trigger_child {
            let mut rows = column![
                text(
                    "The trigger child renders unchanged: the widget wraps it without \
                     adding padding or styling of its own.",
                )
                .size(LABEL_SIZE)
                .color(Color::from_rgb(0.6, 0.6, 0.6)),
            ]
            .spacing(SECTION_SPACING);

            if !behaviour.is_empty() {
                rows = rows.push(field_grid(behaviour));
            }

            if is_menu {
                rows = rows.push(padding_field(
                    "Trigger Padding",
                    props.menu_trigger_padding,
                    move |padding| {
                        Message::PropertyChanged(
                            widget_id,
                            PropertyChange::MenuTriggerPadding(padding),
                        )
                    },
                ));
            }

            rows.into()
        } else {
            column![
                text("No trigger child, so the widget draws a button in its place.")
                    .size(LABEL_SIZE)
                    .color(Color::from_rgb(0.6, 0.6, 0.6)),
                field_grid(behaviour),
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
                padding_controls(props.padding, widget_id, props.padding_mode, theme),
            ]
            .spacing(SECTION_SPACING)
            .into()
        };

        section("Trigger", body)
    };

    // ── Animation and visibility ─────────────────────────────────────────────
    let mut animation = vec![toggle_field(
        "Animate",
        props.overlay_animate,
        move |value| Message::PropertyChanged(widget_id, PropertyChange::OverlayAnimate(value)),
    )];

    if props.overlay_animate {
        animation.push(field(
            "Speed",
            pick_list(
                Some(props.overlay_animation_preset),
                vec![
                    OverlayAnimationPreset::Default,
                    OverlayAnimationPreset::Quick,
                    OverlayAnimationPreset::Slow,
                ],
                ToString::to_string,
            )
            .on_select(move |value| {
                Message::PropertyChanged(
                    widget_id,
                    PropertyChange::OverlayAnimationPreset(value),
                )
            })
            .width(FIELD_WIDTH),
        ));
    }

    // A menu owns its own open state, so there is nothing to start open.
    if !is_menu {
        animation.push(toggle_field(
            "Start open",
            props.overlay_initially_open,
            move |value| {
                Message::PropertyChanged(widget_id, PropertyChange::OverlayInitiallyOpen(value))
            },
        ));
    }

    // ── Assembly, in the order each widget reads ─────────────────────────────
    let mut sections: Vec<Element<'a, Message>> = vec![header.into()];

    let mut push = |title: &'a str, fields: Vec<Element<'a, Message>>| {
        if !fields.is_empty() {
            sections.push(section(title, field_grid(fields)));
        }
    };

    if is_layer {
        push("Window", window);
        push("Placement", placement);
        push("Size", size);
        push("Backdrop", dismissal);
        push("Appearance", appearance);
        push("Animation", animation);
    } else {
        push("Placement", placement);
        push("Size", size);
        push("Appearance", appearance);
        push("Behaviour", dismissal);
        push("Animation", animation);
    }

    if is_menu {
        sections.push(menu_panel);
    }

    if is_anchored {
        sections.push(trigger);
    }

    let content = Column::with_children(sections)
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

    // Single mode has one date; range mode has two.
    let initial_selection_fields: Vec<Element<'a, Message>> = match props.date_picker_mode {
        DatePickerSelectionMode::Single => vec![text_field(
            "Initial date",
            "YYYY-MM-DD",
            &props.date_picker_initial_single_date,
            move |value| Message::PropertyChanged(
                widget_id,
                PropertyChange::DatePickerInitialSingleDate(value),
            ),
        )],
        DatePickerSelectionMode::Range => vec![
            text_field(
                "Range start",
                "YYYY-MM-DD",
                &props.date_picker_initial_range_start,
                move |value| Message::PropertyChanged(
                    widget_id,
                    PropertyChange::DatePickerInitialRangeStart(value),
                ),
            ),
            text_field(
                "Range end",
                "YYYY-MM-DD",
                &props.date_picker_initial_range_end,
                move |value| Message::PropertyChanged(
                    widget_id,
                    PropertyChange::DatePickerInitialRangeEnd(value),
                ),
            ),
        ],
    };

    let time_controls: Element<'a, Message> = if props.date_picker_show_time {
        section(
            "Initial time",
            field_grid(vec![
                slider_field(
                    "Hour",
                    0.0..=23.0,
                    props.date_picker_initial_hour as f32,
                    move |value| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::DatePickerInitialHour(value.round() as u32),
                    ),
                ),
                slider_field(
                    "Minute",
                    0.0..=59.0,
                    props.date_picker_initial_minute as f32,
                    move |value| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::DatePickerInitialMinute(value.round() as u32),
                    ),
                ),
            ]),
        )
    } else {
        nothing()
    };
    let content = column![
        text("Date Picker Properties").size(TITLE_SIZE),
        widget_name(widget_id, &props.widget_name),
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
        section(
            "Picker",
            column![
                field_grid(
                    [
                        vec![
                            text_field(
                                "Trigger text",
                                "Select a date",
                                &props.text_content,
                                move |value| Message::PropertyChanged(
                                    widget_id,
                                    PropertyChange::TextContent(value),
                                ),
                            ),
                            pick_field(
                                "Mode",
                                props.date_picker_mode,
                                vec![
                                    DatePickerSelectionMode::Single,
                                    DatePickerSelectionMode::Range,
                                ],
                                move |value| Message::PropertyChanged(
                                    widget_id,
                                    PropertyChange::DatePickerSelectionMode(value),
                                ),
                            ),
                        ],
                        initial_selection_fields,
                        vec![
                            toggle_field(
                                "Include time",
                                props.date_picker_show_time,
                                move |value| Message::PropertyChanged(
                                    widget_id,
                                    PropertyChange::DatePickerShowTime(value),
                                ),
                            ),
                            toggle_field(
                                "Start open",
                                props.date_picker_initially_open,
                                move |value| Message::PropertyChanged(
                                    widget_id,
                                    PropertyChange::DatePickerInitiallyOpen(value),
                                ),
                            ),
                        ],
                    ]
                    .into_iter()
                    .flatten()
                    .collect()
                ),
                text("The trigger text is used until the picker has a selected date.")
                    .size(LABEL_SIZE - 1.0)
                    .color(Color::from_rgb(0.6, 0.6, 0.6)),
            ]
            .spacing(LABEL_SPACING),
        ),
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
        advanced_section(widget_id, props, theme),
    ]
    .spacing(MAIN_SPACING)
    .into();

    scrollable(container(add_code_preview(content, preview_content)).padding(padding::right(10.0)))
        .into()
}

pub fn split_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    _theme: &Theme,
    _type_system: &'a TypeSystem,
    _custom_styles: &'a CustomThemes,
    preview_content: &'a text_editor::Content,
) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;
    let pane_hint = match props.orientation {
        Orientation::Vertical => "Child 1 is the left pane and child 2 is the right pane.",
        Orientation::Horizontal => "Child 1 is the top pane and child 2 is the bottom pane.",
    };
    let position_label = match props.split_strategy {
        SplitStrategy::Relative => "Split Position",
        SplitStrategy::Start => "Start Pane Size",
        SplitStrategy::End => "End Pane Size",
    };

    // Relative splits run 0..1 and want two decimals; the pixel strategies
    // run to the pane size and want none.
    let position_field: Element<'a, Message> = match props.split_strategy {
        SplitStrategy::Relative => field(
            position_label,
            row![
                slider(0.0..=1.0, props.split_at, move |value| {
                    Message::PropertyChanged(widget_id, PropertyChange::SplitAt(value))
                })
                .step(0.01)
                .width(FIELD_WIDTH - 55.0),
                text(format!("{:.2}", props.split_at))
                    .size(LABEL_SIZE)
                    .width(50),
            ]
            .spacing(LABEL_SPACING)
            .align_y(Alignment::Center),
        ),
        SplitStrategy::Start | SplitStrategy::End => slider_field(
            position_label,
            0.0..=props.split_at.max(800.0).ceil(),
            props.split_at,
            move |value| Message::PropertyChanged(widget_id, PropertyChange::SplitAt(value)),
        ),
    };

    let content = column![
        text("Split Properties").size(TITLE_SIZE),
        widget_name(widget_id, &props.widget_name),
        section(
            "Split",
            column![
                text(format!(
                    "Split lays out exactly two child panes with a draggable \
                     divider. {pane_hint}"
                ))
                .size(LABEL_SIZE)
                .color(Color::from_rgb(0.6, 0.6, 0.6)),
                field_grid(vec![
                    pick_field(
                        "Orientation",
                        props.orientation,
                        vec![Orientation::Vertical, Orientation::Horizontal],
                        move |value| Message::PropertyChanged(
                            widget_id,
                            PropertyChange::Orientation(value),
                        ),
                    ),
                    pick_field(
                        "Sizing",
                        props.split_strategy,
                        vec![
                            SplitStrategy::Relative,
                            SplitStrategy::Start,
                            SplitStrategy::End,
                        ],
                        move |value| Message::PropertyChanged(
                            widget_id,
                            PropertyChange::SplitStrategy(value),
                        ),
                    ),
                    position_field,
                    slider_field(
                        "Handle width",
                        1.0..=32.0,
                        props.split_handle_width,
                        move |value| Message::PropertyChanged(
                            widget_id,
                            PropertyChange::SplitHandleWidth(value),
                        ),
                    ),
                ]),
            ]
            .spacing(LABEL_SPACING),
        ),
        size_controls_scrollable_aware(
            props.width,
            move |l| Message::PropertyChanged(widget_id, PropertyChange::Width(l)),
            props.height,
            move |l| Message::PropertyChanged(widget_id, PropertyChange::Height(l)),
            h,
            widget_id,
        ),
        advanced_section(widget_id, props, _theme),
    ]
    .spacing(MAIN_SPACING)
    .into();

    scrollable(container(add_code_preview(content, preview_content)).padding(padding::right(10.0)))
        .into()
}

pub fn float_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    theme: &Theme,
    _type_system: &'a TypeSystem,
    _custom_styles: &'a CustomThemes,
    preview_content: &'a text_editor::Content,
) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    let content = column![
        text("Float Properties").size(TITLE_SIZE),
        widget_name(widget_id, &props.widget_name),
        section(
            "Float",
            column![
                text("Float renders its child as an overlay when you scale or offset it.")
                    .size(LABEL_SIZE)
                    .color(Color::from_rgb(0.6, 0.6, 0.6)),
                field_grid(vec![
                    // Scale is a multiplier in twentieths, so it keeps its own
                    // two-decimal readout.
                    field(
                        "Scale",
                        row![
                            slider(0.25..=3.0, props.float_scale, move |value| {
                                Message::PropertyChanged(
                                    widget_id,
                                    PropertyChange::FloatScale(value),
                                )
                            })
                            .step(0.05)
                            .width(FIELD_WIDTH - 55.0),
                            text(format!("{:.2}x", props.float_scale))
                                .size(LABEL_SIZE)
                                .width(50),
                        ]
                        .spacing(LABEL_SPACING)
                        .align_y(Alignment::Center),
                    ),
                    slider_field(
                        "Translate X",
                        -400.0..=400.0,
                        props.float_translate_x,
                        move |value| Message::PropertyChanged(
                            widget_id,
                            PropertyChange::FloatTranslateX(value),
                        ),
                    ),
                    slider_field(
                        "Translate Y",
                        -400.0..=400.0,
                        props.float_translate_y,
                        move |value| Message::PropertyChanged(
                            widget_id,
                            PropertyChange::FloatTranslateY(value),
                        ),
                    ),
                ]),
            ]
            .spacing(LABEL_SPACING),
        ),
        size_controls_scrollable_aware(
            props.width,
            move |l| Message::PropertyChanged(widget_id, PropertyChange::Width(l)),
            props.height,
            move |l| Message::PropertyChanged(widget_id, PropertyChange::Height(l)),
            h,
            widget_id,
        ),
        advanced_section(widget_id, props, theme),
    ]
    .spacing(MAIN_SPACING)
    .into();

    scrollable(container(add_code_preview(content, preview_content)).padding(padding::right(10.0)))
        .into()
}

pub fn responsive_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    theme: &Theme,
    _type_system: &'a TypeSystem,
    _custom_styles: &'a CustomThemes,
    preview_content: &'a text_editor::Content,
) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    let content = column![
        text("Responsive Properties").size(TITLE_SIZE),
        widget_name(widget_id, &props.widget_name),
        section(
            "Responsive",
            column![
                text(
                    "Child 1 is shown at or below the breakpoint width; child 2 \n                     is shown above it."
                )
                .size(LABEL_SIZE)
                .color(Color::from_rgb(0.6, 0.6, 0.6)),
                field_grid(vec![slider_field(
                    "Breakpoint",
                    240.0..=1600.0,
                    props.responsive_breakpoint,
                    move |value| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::ResponsiveBreakpoint(value),
                    ),
                )]),
            ]
            .spacing(LABEL_SPACING),
        ),
        size_controls_scrollable_aware(
            props.width,
            move |l| Message::PropertyChanged(widget_id, PropertyChange::Width(l)),
            props.height,
            move |l| Message::PropertyChanged(widget_id, PropertyChange::Height(l)),
            h,
            widget_id,
        ),
        advanced_section(widget_id, props, theme),
    ]
    .spacing(MAIN_SPACING)
    .into();

    scrollable(container(add_code_preview(content, preview_content)).padding(padding::right(10.0)))
        .into()
}

pub fn sensor_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    theme: &Theme,
    _type_system: &'a TypeSystem,
    _custom_styles: &'a CustomThemes,
    preview_content: &'a text_editor::Content,
) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    let content = column![
        text("Sensor Properties").size(TITLE_SIZE),
        widget_name(widget_id, &props.widget_name),
        text("Sensor fires actions when its child appears, resizes while visible, or leaves view.")
            .size(LABEL_SIZE)
            .color(Color::from_rgb(0.6, 0.6, 0.6)),
        column![
            text("Events").size(SECTION_SIZE),
            rule::horizontal(1),
            column![
                event_checkbox_row(
                    theme,
                    props.sensor_on_show,
                    "on_show",
                    "Fires when the widget enters the viewport and provides width and height",
                    move |value| Message::PropertyChanged(widget_id, PropertyChange::SensorOnShow(value)),
                ),
                event_checkbox_row(
                    theme,
                    props.sensor_on_resize,
                    "on_resize",
                    "Fires when the widget changes size while visible and provides width and height",
                    move |value| Message::PropertyChanged(widget_id, PropertyChange::SensorOnResize(value)),
                ),
                event_checkbox_row(
                    theme,
                    props.sensor_on_hide,
                    "on_hide",
                    "Fires when the widget leaves the viewport",
                    move |value| Message::PropertyChanged(widget_id, PropertyChange::SensorOnHide(value)),
                ),
            ]
            .spacing(LABEL_SPACING),
        ]
        .spacing(LABEL_SPACING),
        section(
            "Sensor",
            field_grid(vec![
                slider_field(
                    "Anticipate",
                    0.0..=600.0,
                    props.sensor_anticipate,
                    move |value| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::SensorAnticipate(value),
                    ),
                ),
                slider_field(
                    "Delay (ms)",
                    0.0..=5000.0,
                    props.sensor_delay_ms as f32,
                    move |value| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::SensorDelayMs(value.round() as u64),
                    ),
                ),
            ]),
        ),
        size_controls_scrollable_aware(
            props.width,
            move |l| Message::PropertyChanged(widget_id, PropertyChange::Width(l)),
            props.height,
            move |l| Message::PropertyChanged(widget_id, PropertyChange::Height(l)),
            h,
            widget_id,
        ),
        advanced_section(widget_id, props, theme),
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
            rule::horizontal(1),
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
                    props.mousearea_interaction,
                    MouseInteraction::ALL,
                    ToString::to_string,
                )
                .on_select(move |v| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::MouseAreaInteraction(Some(v))
                    ))
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
        section(
            "Themer",
            column![
                field_grid(vec![field(
                    "Theme",
                    pick_list(props.themer_theme.clone(), Theme::ALL, ToString::to_string)
                        .on_select(move |theme| Message::PropertyChanged(
                            widget_id,
                            PropertyChange::ThemerTheme(Some(theme)),
                        ))
                        .placeholder("Inherit from parent")
                        .width(FIELD_WIDTH),
                )]),
                text("Themer applies a theme to all its children.")
                    .size(LABEL_SIZE)
                    .color(Color::from_rgb(0.6, 0.6, 0.6)),
            ]
            .spacing(LABEL_SPACING),
        ),
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

    let content = column![
        text("Grid Properties").size(TITLE_SIZE),
        widget_name(widget_id, &props.widget_name),
        section(
            "Grid",
            column![
                text("Grid distributes children in a uniform column layout.")
                    .size(LABEL_SIZE)
                    .color(Color::from_rgb(0.6, 0.6, 0.6)),
                field_grid(vec![
                    toggle_field("Fluid columns", props.grid_use_fluid, move |v| {
                        Message::PropertyChanged(widget_id, PropertyChange::GridUseFluid(v))
                    }),
                    // Fluid mode derives the column count from a maximum
                    // column width; fixed mode is told the count outright.
                    if props.grid_use_fluid {
                        text_field("Max column width", "200", fluid_max_str, move |s| {
                            let v = parse_f32(&s, props.grid_fluid_max_width);
                            Message::PropertyChanged(
                                widget_id,
                                PropertyChange::GridFluidMaxWidth(v),
                            )
                        })
                    } else {
                        text_field("Columns", "3", columns_str, move |s| {
                            let v = s
                                .trim()
                                .parse::<usize>()
                                .unwrap_or(props.grid_columns)
                                .max(1);
                            Message::PropertyChanged(widget_id, PropertyChange::GridColumns(v))
                        })
                    },
                    text_field("Spacing", "0", spacing_str, move |s| {
                        let v = parse_f32(&s, props.grid_spacing);
                        Message::PropertyChanged(widget_id, PropertyChange::GridSpacing(v))
                    }),
                    text_field("Fixed width", "fill", fixed_width_str, move |s| {
                        let v = if s.trim().is_empty() {
                            None
                        } else {
                            parse_f32(&s, props.grid_fixed_width.unwrap_or(0.0)).into()
                        };
                        Message::PropertyChanged(widget_id, PropertyChange::GridFixedWidth(v))
                    }),
                ]),
                text(if props.grid_use_fluid {
                    "Columns fill the available space up to the maximum width. \
                     Fixed width sizes the whole grid; leave it empty to fill."
                } else {
                    "Fixed width sizes the whole grid; leave it empty to fill."
                })
                .size(LABEL_SIZE - 1.0)
                .color(Color::from_rgb(0.5, 0.5, 0.5)),
            ]
            .spacing(LABEL_SPACING),
        ),
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
        section(
            "Position",
            field_grid(vec![
                text_field("X", "0", x_str, move |s| {
                    let v = parse_f32(&s, props.pin_point.x);
                    Message::PropertyChanged(widget_id, PropertyChange::PinX(v))
                }),
                text_field("Y", "0", y_str, move |s| {
                    let v = parse_f32(&s, props.pin_point.y);
                    Message::PropertyChanged(widget_id, PropertyChange::PinY(v))
                }),
            ]),
        ),
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
            rule::horizontal(1),
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
                        Some(selected_struct_name),
                        struct_names,
                        ToString::to_string,
                    )
                    .on_select(move |struct_name| {
                            let struct_id = type_system
                                .get_struct_by_name(&struct_name)
                                .expect("MissingStructDef")
                                .id;
                            Message::PropertyChanged(
                                widget_id,
                                PropertyChange::TableReferencedStruct(Some(struct_id)),
                            )
                        })
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
                    scroll_y(
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
        section(
            "Table",
            field_grid(vec![
                slider_field("Padding X", 0.0..=30.0, props.table_padding_x, move |v| {
                    Message::PropertyChanged(widget_id, PropertyChange::TablePaddingX(v))
                }),
                slider_field("Padding Y", 0.0..=30.0, props.table_padding_y, move |v| {
                    Message::PropertyChanged(widget_id, PropertyChange::TablePaddingY(v))
                }),
                // Separators move in half pixels, so they keep a readout with
                // a decimal rather than the shared whole-number one.
                field(
                    "Separator X",
                    row![
                        slider(0.0..=10.0, props.table_separator_x, move |v| {
                            Message::PropertyChanged(widget_id, PropertyChange::TableSeparatorX(v))
                        })
                        .step(0.5)
                        .width(FIELD_WIDTH - 45.0),
                        text(format!("{:.1}", props.table_separator_x))
                            .size(LABEL_SIZE)
                            .width(40),
                    ]
                    .spacing(LABEL_SPACING)
                    .align_y(Alignment::Center),
                ),
                field(
                    "Separator Y",
                    row![
                        slider(0.0..=10.0, props.table_separator_y, move |v| {
                            Message::PropertyChanged(widget_id, PropertyChange::TableSeparatorY(v))
                        })
                        .step(0.5)
                        .width(FIELD_WIDTH - 45.0),
                        text(format!("{:.1}", props.table_separator_y))
                            .size(LABEL_SIZE)
                            .width(40),
                    ]
                    .spacing(LABEL_SPACING)
                    .align_y(Alignment::Center),
                ),
                toggle_field("Bold headers", props.table_bold_headers, move |v| {
                    Message::PropertyChanged(widget_id, PropertyChange::TableBoldHeaders(v))
                }),
            ]),
        ),
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
            rule::horizontal(1),
            text("Select which view to display here:")
                .size(LABEL_SIZE)
                .color(Color::from_rgb(0.6, 0.6, 0.6)),
            pick_list(
                selected_view_name,
                view_options
                    .iter()
                    .map(|(_, name)| name.clone())
                    .collect::<Vec<_>>(),
                ToString::to_string,
            )
            .on_select(move |selected_name| {
                    // Find the UUID for the selected name
                    let view_id = view_options
                        .iter()
                        .find(|(_, name)| name == &selected_name)
                        .map(|(id, _)| *id);

                    Message::PropertyChanged(
                        widget_id,
                        PropertyChange::ViewReferenceId(view_id, selected_name),
                    )
                })
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
                        .color(theme.palette().success.base.color),
                    text("This view will be embedded here")
                        .size(LABEL_SIZE - 2.0)
                        .color(Color::from_rgb(0.5, 0.5, 0.5)),
                ]
                .spacing(LABEL_SPACING)
            } else {
                column![
                    text("Selected view no longer exists")
                        .size(LABEL_SIZE)
                        .color(theme.palette().danger.base.color),
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
                            selected_name,
                            view_options2
                                .iter()
                                .map(|(_, name)| name.clone())
                                .collect::<Vec<_>>(),
                            ToString::to_string,
                        )
                        .on_select(move |n| {
                                let vid = view_options2
                                    .iter()
                                    .find(|(_, name)| name == &n)
                                    .map(|(id, _)| *id)
                                    .unwrap();
                                Message::PropertyChanged(
                                    widget_id,
                                    PropertyChange::SetExtraViewRef(i, vid),
                                )
                            })
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

    let picker = icon_picker(
        &props.icon_family,
        &props.icon_name,
        &props.icon_picker_filter,
        move |family| Message::PropertyChanged(widget_id, PropertyChange::IconFamily(family)),
        move |v| Message::PropertyChanged(widget_id, PropertyChange::IconPickerFilter(v)),
        move |family, name, codepoint| {
            Message::PropertyChanged(
                widget_id,
                PropertyChange::IconSelected(family, name, codepoint),
            )
        },
    );

    let content = column![
        text("Icon Properties").size(TITLE_SIZE),
        widget_name(widget_id, &props.widget_name),
        section(
            "Icon",
            field_grid(vec![
                field("Icon", picker),
                slider_field("Size", 8.0..=128.0, props.icon_size, move |v| {
                    Message::PropertyChanged(widget_id, PropertyChange::IconSize(v))
                }),
            ]),
        ),
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
        text_input("#RRGGBB or #RRGGBBAA", current)
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
/// The pixel value a length picker reaches for when it has none of its own.
const DEFAULT_LENGTH_PX: f32 = 120.0;
const DEFAULT_LENGTH_PORTION: u16 = 1;
const DEFAULT_LENGTH_MIN: f32 = 0.0;
const DEFAULT_LENGTH_MAX: f32 = 400.0;

/// Pulls a bounded length apart into the values its editors each own.
///
/// A length that is not bounded still answers, with defaults — that is what
/// makes switching *to* `Bounded` land somewhere sensible.
fn bounded_parts(current: Length) -> (SizingChoice, u16, BoundsChoice, f32, f32) {
    let Length::Bounded { bounds, sizing } = current else {
        let sizing = match current {
            Length::Fill => SizingChoice::Fill,
            Length::FillPortion(_) => SizingChoice::FillPortion,
            Length::Shrink => SizingChoice::Shrink,
            _ => SizingChoice::Fit,
        };

        let portion = match current {
            Length::FillPortion(portion) => portion,
            _ => DEFAULT_LENGTH_PORTION,
        };

        return (
            sizing,
            portion,
            BoundsChoice::Max,
            DEFAULT_LENGTH_MIN,
            DEFAULT_LENGTH_MAX,
        );
    };

    let portion = match sizing {
        Sizing::Fill(portion) => portion,
        _ => DEFAULT_LENGTH_PORTION,
    };

    let (min, max) = match bounds {
        Bounds::Min(min) => (min, DEFAULT_LENGTH_MAX.max(min)),
        Bounds::Max(max) => (DEFAULT_LENGTH_MIN, max),
        Bounds::Both { min, max } => (min, max),
    };

    (
        SizingChoice::from_sizing(sizing),
        portion,
        BoundsChoice::from_bounds(bounds),
        min,
        max,
    )
}

/// A labelled number field that keeps the current value while it is being
/// typed into.
///
/// These edit a `Length` straight through, so there is nowhere to park a
/// half-typed string: an unparseable field reads as "unchanged" rather than
/// snapping the widget to zero on the way to `12`.
fn pixel_field<'a, M: Clone + 'a>(
    label: &'a str,
    value: f32,
    on_change: impl Fn(f32) -> M + 'a,
) -> Element<'a, M> {
    column![
        text(label),
        text_input("e.g. 120.0", format!("{value}"))
            .on_input(move |input| on_change(input.trim().parse::<f32>().unwrap_or(value)))
            .width(120)
    ]
    .spacing(5)
    .into()
}

/// The controls that sit beside a length's kind, for whichever kind is chosen.
///
/// `Fixed` wants a pixel count, `FillPortion` a portion, and the two lengths
/// iced master added want rather more than that — so all of it lives here
/// rather than being written out at each picker.
fn length_detail_controls<'a, M: Clone + 'a>(
    current: Length,
    can_fill: bool,
    on_change: impl Fn(Length) -> M + 'a + Copy,
) -> Element<'a, M> {
    match LengthChoice::from_length(current) {
        LengthChoice::Fixed => {
            let pixels = match current {
                Length::Fixed(pixels) => pixels,
                _ => DEFAULT_LENGTH_PX,
            };

            pixel_field("Pixels", pixels, move |pixels| {
                on_change(Length::Fixed(pixels))
            })
        }

        LengthChoice::FillPortion if can_fill => {
            let portion = match current {
                Length::FillPortion(portion) => portion,
                _ => DEFAULT_LENGTH_PORTION,
            };

            column![
                text("Portion"),
                text_input("e.g. 1", portion.to_string())
                    .on_input(move |input| {
                        let portion = input
                            .trim()
                            .parse::<u16>()
                            .map(|portion| portion.max(1))
                            .unwrap_or(portion);

                        on_change(Length::FillPortion(portion))
                    })
                    .width(120)
            ]
            .spacing(5)
            .into()
        }

        LengthChoice::Bounded => {
            let (sizing, portion, bounds, min, max) = bounded_parts(current);

            let rebuild = move |sizing: SizingChoice, portion: u16, bounds: BoundsChoice, min, max| {
                on_change(Length::Bounded {
                    bounds: bounds.to_bounds(min, max),
                    sizing: sizing.to_sizing(portion),
                })
            };

            let sizings: Vec<SizingChoice> = SizingChoice::ALL
                .iter()
                .copied()
                .filter(|choice| {
                    can_fill
                        || !matches!(choice, SizingChoice::Fill | SizingChoice::FillPortion)
                })
                .collect();

            let mut controls = column![
                column![
                    text("Grows by"),
                    pick_list(Some(sizing), sizings, ToString::to_string)
                        .on_select(move |sizing| rebuild(sizing, portion, bounds, min, max))
                        .width(160),
                ]
                .spacing(5),
                column![
                    text("Bounded at"),
                    pick_list(Some(bounds), BoundsChoice::ALL.to_vec(), ToString::to_string)
                        .on_select(move |bounds| rebuild(sizing, portion, bounds, min, max))
                        .width(160),
                ]
                .spacing(5),
            ]
            .spacing(10);

            if sizing == SizingChoice::FillPortion {
                controls = controls.push(
                    column![
                        text("Portion"),
                        text_input("e.g. 1", portion.to_string())
                            .on_input(move |input| {
                                let portion = input
                                    .trim()
                                    .parse::<u16>()
                                    .map(|portion| portion.max(1))
                                    .unwrap_or(portion);

                                rebuild(sizing, portion, bounds, min, max)
                            })
                            .width(120),
                    ]
                    .spacing(5),
                );
            }

            if bounds != BoundsChoice::Max {
                controls = controls.push(pixel_field("Minimum px", min, move |min| {
                    rebuild(sizing, portion, bounds, min, max)
                }));
            }

            if bounds != BoundsChoice::Min {
                controls = controls.push(pixel_field("Maximum px", max, move |max| {
                    rebuild(sizing, portion, bounds, min, max)
                }));
            }

            controls.into()
        }

        LengthChoice::Fluid => {
            let constraint = match current {
                Length::Fluid(constraint) => constraint,
                _ => Constraint::Max,
            };

            let min = match constraint {
                Constraint::Min(min) => min,
                Constraint::Max => DEFAULT_LENGTH_MIN,
            };

            let mut controls = column![
                column![
                    text("Limit"),
                    pick_list(
                        Some(ConstraintChoice::from_constraint(constraint)),
                        ConstraintChoice::ALL.to_vec(),
                        ToString::to_string,
                    )
                    .on_select(move |choice| on_change(Length::Fluid(choice.to_constraint(min))))
                    .width(200),
                ]
                .spacing(5),
            ]
            .spacing(10);

            if matches!(constraint, Constraint::Min(_)) {
                controls = controls.push(pixel_field("Minimum px", min, move |min| {
                    on_change(Length::Fluid(Constraint::Min(min)))
                }));
            }

            controls.into()
        }

        _ => Space::new().width(0).into(),
    }
}

/// Turns a chosen kind into a length, keeping whatever the current one can
/// lend it.
fn length_for_choice(choice: LengthChoice, current: Length) -> Length {
    let (sizing, portion, bounds, min, max) = bounded_parts(current);

    match choice {
        LengthChoice::Fill => Length::Fill,
        LengthChoice::FillPortion => match current {
            Length::FillPortion(portion) => Length::FillPortion(portion),
            _ => Length::FillPortion(DEFAULT_LENGTH_PORTION),
        },
        LengthChoice::Fit => Length::Fit,
        LengthChoice::Shrink => Length::Shrink,
        LengthChoice::Fixed => match current {
            Length::Fixed(pixels) => Length::Fixed(pixels),
            _ => Length::Fixed(DEFAULT_LENGTH_PX),
        },
        LengthChoice::Bounded => Length::Bounded {
            bounds: bounds.to_bounds(min, max),
            sizing: sizing.to_sizing(portion),
        },
        LengthChoice::Fluid => match current {
            Length::Fluid(constraint) => Length::Fluid(constraint),
            _ => Length::Fluid(Constraint::Max),
        },
    }
}

/// The kinds a length may take here.
///
/// A scrollable gives its child an unbounded axis to scroll through, so
/// nothing that fills can be measured against it.
fn length_choices(can_fill: bool) -> Vec<LengthChoice> {
    LengthChoice::ALL
        .iter()
        .copied()
        .filter(|choice| can_fill || !choice.needs_fill())
        .collect()
}

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
        pick_list(Some(choice_now), length_choices(can_fill), ToString::to_string)
            .on_select(move |choice| on_change(length_for_choice(choice, current)))
            .width(160)
    ]
    .spacing(5)
    .width(Length::Shrink);

    row![picker, length_detail_controls(current, can_fill, on_change)]
        .spacing(15)
        .into()
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

    let picker = column![
        if !can_fill && saved_value.is_some() {
            column![
                text(label).size(LABEL_SIZE),
                text(format!("(was: {})", length_to_string(saved_value.unwrap()))).size(LABEL_SIZE)
            ]
        } else {
            column![text(label).size(LABEL_SIZE)]
        },
        pick_list(Some(choice_now), length_choices(can_fill), ToString::to_string)
            .on_select(move |choice| on_change(length_for_choice(choice, current)))
            .width(160)
    ]
    .spacing(LABEL_SPACING)
    .width(Length::Shrink);

    // `Fixed` and `FillPortion` are typed into, so they keep the draft string
    // that lets a field sit empty mid-edit. The lengths iced master added have
    // no draft of their own, and fall through to the shared editors.
    let extra: Element<_> = match choice_now {
        LengthChoice::Fixed => {
            let committed_value = match current {
                Length::Fixed(px) => format!("{px}"),
                _ => format!("{DEFAULT_PX}"),
            };

            let display_text = draft_text.map(|s| s.as_str()).unwrap_or("");

            column![
                text("Pixels").size(LABEL_SIZE),
                text_input(committed_value.clone(), display_text)
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
                text_input(committed_value.clone(), display_text)
                    .on_input(move |v| { on_draft_change(v) })
                    .width(75)
            ]
            .spacing(LABEL_SPACING)
            .into()
        }
        LengthChoice::Bounded | LengthChoice::Fluid => {
            length_detail_controls(current, can_fill, on_change)
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
        rule::horizontal(1),
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
    let palette = theme.palette();
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
    field(
        "Clipping",
        row![
            checkbox(clipped)
                .label("Clip")
                .on_toggle(move |v| Message::PropertyChanged(widget_id, PropertyChange::Clip(v))),
            information(theme, "Child content beyond the bounds is cut off"),
        ]
        .spacing(LABEL_SPACING)
        .align_y(Alignment::Center),
    )
}

pub fn max_width_control<'a>(widget_id: WidgetId, max_width: Option<f32>) -> Element<'a, Message> {
    bounded_field("Max width", max_width, move |value| {
        Message::PropertyChanged(widget_id, PropertyChange::MaxWidth(value))
    })
}

pub fn max_height_control<'a>(
    widget_id: WidgetId,
    max_height: Option<f32>,
) -> Element<'a, Message> {
    bounded_field("Max height", max_height, move |value| {
        Message::PropertyChanged(widget_id, PropertyChange::MaxHeight(value))
    })
}

/// An optional pixel bound: a switch that turns it on, and an entry for it.
///
/// Turning it on starts at a size rather than zero, so the widget does not
/// vanish the moment the box is ticked.
fn bounded_field<'a>(
    label: &'a str,
    value: Option<f32>,
    on_change: impl Fn(Option<f32>) -> Message + 'a + Copy,
) -> Element<'a, Message> {
    const DEFAULT: f32 = 800.0;

    let control: Element<'a, Message> = match value {
        Some(value) => text_input("px", format!("{value}"))
            .on_input(move |raw| on_change(Some(raw.trim().parse().unwrap_or(value))))
            .width(FIELD_WIDTH - 30.0)
            .into(),
        None => text("off").size(LABEL_SIZE).into(),
    };

    field(
        label,
        row![
            checkbox(value.is_some()).on_toggle(move |on| on_change(on.then_some(DEFAULT))),
            control,
        ]
        .spacing(LABEL_SPACING)
        .align_y(Alignment::Center),
    )
}

/// The settings every container-ish widget carries: an id, bounds, clipping.
///
/// They are the same four everywhere they appear, and none of them is what
/// anyone opens the panel for, so they travel together at the end.
pub fn advanced_section<'a>(
    widget_id: WidgetId,
    props: &'a Properties,
    theme: &Theme,
) -> Element<'a, Message> {
    section(
        "Advanced",
        field_grid(vec![
            widget_id_control(widget_id, props.widget_id.clone(), theme),
            max_width_control(widget_id, props.max_width),
            max_height_control(widget_id, props.max_height),
            clip_control(widget_id, props.clip, theme),
        ]),
    )
}

pub fn widget_id_control<'a>(
    widget_id: WidgetId,
    id: Option<String>,
    theme: &Theme,
) -> Element<'a, Message> {
    let is_set = id.is_some();

    let control: Element<'a, Message> = match id {
        Some(id) => text_input("widget_id", id)
            .on_input(move |v| {
                Message::PropertyChanged(widget_id, PropertyChange::WidgetId(Some(v)))
            })
            .width(FIELD_WIDTH - 30.0)
            .into(),
        // With no id to edit there is room for the explanation instead.
        None => information(theme, "Use for programmatic access via widget::Id"),
    };

    field(
        "Widget id",
        row![
            checkbox(is_set).on_toggle(move |enabled| {
                Message::PropertyChanged(
                    widget_id,
                    PropertyChange::WidgetId(enabled.then(String::new)),
                )
            }),
            control,
        ]
        .spacing(LABEL_SPACING)
        .align_y(Alignment::Center),
    )
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
                    props.style_condition_value.clone(),
                    value_variants,
                    ToString::to_string,
                )
                .on_select(move |v| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::StyleConditionValue(Some(v))
                    ))
                .placeholder("select value"),
            ]
            .spacing(LABEL_SPACING)
            .into()
        } else {
            column![
                text("When equals").size(LABEL_SIZE),
                text_input("value", condition_value.clone())
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
            pick_list(
                selected_field_display,
                field_options,
                ToString::to_string,
            )
            .on_select(move |d: String| {
                // Map display label back to field path
                let path = fields_for_closure
                    .iter()
                    .find(|(display, _, _)| display == &d)
                    .map(|(_, p, _)| p.clone())
                    .unwrap_or(d);
                Message::PropertyChanged(widget_id, PropertyChange::StyleConditionField(Some(path)))
            })
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
                props.style_condition_field.clone().unwrap_or_default()
            )
            .on_input(move |v| {
                let val = if v.is_empty() { None } else { Some(v) };
                Message::PropertyChanged(widget_id, PropertyChange::StyleConditionField(val))
            })
            .size(LABEL_SIZE),
            text("When equals").size(LABEL_SIZE),
            text_input("value", condition_value.clone())
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
            props.custom_style_name.clone(),
            style_options.clone(),
            ToString::to_string,
        )
        .on_select(move |selection| {
                Message::PropertyChanged(
                    widget_id,
                    PropertyChange::CustomStyle(Some(selection.to_string())),
                )
            })
        .placeholder("default"),
    ]
    .spacing(LABEL_SPACING);

    if supports_active_style {
        content = content.push(text("Alternate style").size(LABEL_SIZE)).push(
            pick_list(
                props.active_style_name.clone(),
                style_options,
                ToString::to_string,
            )
            .on_select(move |selection| {
                    Message::PropertyChanged(
                        widget_id,
                        PropertyChange::ActiveStyle(Some(selection.to_string())),
                    )
                })
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

pub(crate) fn style_options_for_widget_type(
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
        pick_list(
            selected_style,
            style_options,
            ToString::to_string,
        )
        .on_select(move |selection| {
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
        column![scroll_y(content)].height(400),
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

/// The role a widget plays inside an overlay parent, if any.
///
/// A row of a Menu or an item in a Toast carries settings that belong to that
/// role rather than to the widget's own type — any widget at all can be a menu
/// row. The trigger, which is a Menu's first child, is not a row.
pub fn overlay_child_role(h: &WidgetHierarchy, widget_id: WidgetId) -> Option<WidgetType> {
    let parent_id = h.find_parent_id(widget_id)?;
    let parent = h.get_widget_by_id(parent_id)?;

    match parent.widget_type {
        WidgetType::Toast => Some(WidgetType::Toast),
        WidgetType::Menu => {
            let is_trigger = parent
                .children
                .first()
                .is_some_and(|first| first.id == widget_id);

            (!is_trigger).then_some(WidgetType::Menu)
        }
        // A submenu's own children are rows of that submenu.
        WidgetType::Popover | WidgetType::Layer => None,
        _ => {
            let grandparent_is_menu = h
                .find_parent_id(parent_id)
                .and_then(|id| h.get_widget_by_id(id))
                .is_some_and(|w| w.widget_type == WidgetType::Menu);

            (grandparent_is_menu && parent.properties.menu_item_kind == MenuItemKind::Submenu)
                .then_some(WidgetType::Menu)
        }
    }
}

/// The settings a widget has by virtue of sitting inside a Menu or a Toast.
pub fn overlay_child_controls<'a>(
    role: WidgetType,
    widget_id: WidgetId,
    props: &'a Properties,
) -> Element<'a, Message> {
    match role {
        WidgetType::Menu => {
            let kind = props.menu_item_kind;

            let checked: Element<'a, Message> = if kind == MenuItemKind::Toggle {
                checkbox(props.menu_item_checked)
                    .label("Checked")
                    .on_toggle(move |value| {
                        Message::PropertyChanged(widget_id, PropertyChange::MenuItemChecked(value))
                    })
                    .into()
            } else {
                column![].into()
            };

            column![
                text("Menu Row").size(SECTION_SIZE),
                rule::horizontal(1),
                column![
                    text("Row Kind").size(LABEL_SIZE),
                    pick_list(Some(kind), MenuItemKind::ALL, ToString::to_string)
                        .on_select(move |value| Message::PropertyChanged(
                            widget_id,
                            PropertyChange::MenuItemKind(value)
                        ))
                        .width(180),
                ]
                .spacing(LABEL_SPACING),
                checked,
                checkbox(props.menu_item_enabled)
                    .label("Enabled")
                    .on_toggle(move |value| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::MenuItemEnabled(value)
                    )),
                rule::horizontal(1),
            ]
            .spacing(LABEL_SPACING)
            .into()
        }
        _ => column![
            text("Toast").size(SECTION_SIZE),
            rule::horizontal(1),
            column![
                text("Variant").size(LABEL_SIZE),
                pick_list(
                    Some(props.toast_variant),
                    ToastVariant::ALL,
                    ToString::to_string
                )
                .on_select(move |value| Message::PropertyChanged(
                    widget_id,
                    PropertyChange::ToastVariant(value)
                ))
                .width(180),
            ]
            .spacing(LABEL_SPACING),
            checkbox(props.toast_without_icon)
                .label("Hide status icon")
                .on_toggle(move |value| Message::PropertyChanged(
                    widget_id,
                    PropertyChange::ToastWithoutIcon(value)
                )),
            float_field(
                "Dismiss after (seconds, 0 keeps it)",
                props.toast_timeout_secs,
                move |value| Message::PropertyChanged(
                    widget_id,
                    PropertyChange::ToastTimeoutSecs(value)
                ),
            ),
            rule::horizontal(1),
        ]
        .spacing(LABEL_SPACING)
        .into(),
    }
}

/// A labelled numeric field.
///
/// The overlay widgets take most of their measurements as plain pixel values,
/// and a text field keeps an exact number enterable where a slider would not.
pub fn float_field<'a>(
    label: &'a str,
    value: f32,
    on_change: impl Fn(f32) -> Message + 'a,
) -> Element<'a, Message> {
    column![
        text(label).size(LABEL_SIZE),
        text_input("0", format!("{value}"))
            .on_input(move |raw| {
                let parsed = raw.trim().parse::<f32>().unwrap_or(0.0);
                on_change(parsed)
            })
            .width(120),
    ]
    .spacing(LABEL_SPACING)
    .into()
}

/// Four numeric fields for the sides of a `Padding`.
///
/// The menu takes padding for its rows and its trigger separately from the
/// panel's own, and neither goes through the shared padding control, which is
/// tied to a widget's single `padding` property and its mode.
pub fn padding_field<'a>(
    label: &'a str,
    padding: Padding,
    on_change: impl Fn(Padding) -> Message + Clone + 'a,
) -> Element<'a, Message> {
    // Four narrow entries on one line: a padding is one setting, so it reads
    // better as a single row than as four fields in the wrapping grid.
    let side = move |value: f32, set: fn(&mut Padding, f32)| {
        let on_change = on_change.clone();

        text_input("0", format!("{value}"))
            .on_input(move |raw| {
                let mut next = padding;
                set(&mut next, raw.trim().parse::<f32>().unwrap_or(0.0));
                on_change(next)
            })
            .width(56)
    };

    column![
        text(format!("{label} (top, right, bottom, left)")).size(LABEL_SIZE),
        row![
            side(padding.top, |p, v| p.top = v),
            side(padding.right, |p, v| p.right = v),
            side(padding.bottom, |p, v| p.bottom = v),
            side(padding.left, |p, v| p.left = v),
        ]
        .spacing(LABEL_SPACING),
    ]
    .spacing(LABEL_SPACING)
    .into()
}

// ── Property grid ────────────────────────────────────────────────────────────
//
// The overlay panels carry enough settings that a single column of full-width
// rows runs to several screens. These lay small controls out in a wrapping row
// instead, so the editor's width is actually used and related settings sit
// beside each other.

/// The width one control in the property grid occupies.
///
/// Wide enough for a pick list of anchor names, narrow enough that three fit
/// across the editor at its default width.
pub const FIELD_WIDTH: f32 = 168.0;

/// The height of one control in the property grid.
///
/// A pick list, a text entry and a checkbox are all different heights on their
/// own, which left a wrapped row looking ragged. Boxing each to the same height
/// keeps the rows level whatever mix of controls lands on them.
pub const CONTROL_HEIGHT: f32 = 32.0;

/// A labelled control, sized to the property grid.
pub fn field<'a, M: 'a>(
    label: &'a str,
    control: impl Into<Element<'a, M>>,
) -> Element<'a, M> {
    column![
        text(label).size(LABEL_SIZE),
        container(control.into())
            .height(CONTROL_HEIGHT)
            .align_y(Alignment::Center),
    ]
    .spacing(LABEL_SPACING)
    .width(FIELD_WIDTH)
    .into()
}

/// Lays fields out across the available width, wrapping onto further rows.
pub fn field_grid<'a, M: 'a>(fields: Vec<Element<'a, M>>) -> Element<'a, M> {
    Row::with_children(fields)
        .spacing(SECTION_SPACING)
        .wrap()
        .into()
}

/// A titled group of controls.
pub fn section<'a, M: 'a>(
    title: &'a str,
    body: impl Into<Element<'a, M>>,
) -> Element<'a, M> {
    column![
        text(title).size(SECTION_SIZE),
        rule::horizontal(1),
        body.into(),
    ]
    .spacing(LABEL_SPACING)
    .into()
}

/// Nothing at all, for a section that does not apply to this widget.
pub fn nothing<'a, M: 'a>() -> Element<'a, M> {
    column![].into()
}

/// How wide the widget editor opens.
///
/// The property grid is a wrapping row of `FIELD_WIDTH` columns, which asks
/// for whatever width it is given — so without a cap the layer opens as wide
/// as the viewport and the grid lays itself out in one long line. Four columns
/// across is the shape the sections were written for; the rest is the layer's
/// own 16px padding on each side, the editor's 5px, and the scrollbar gutter.
///
/// The editor is resizable, so this is a starting size rather than a limit:
/// `max_width` caps the width the layer *chooses*, not what the user can drag
/// it to.
pub const EDITOR_MODAL_WIDTH: f32 = editor_modal_width(4.0);

/// The narrowest the widget editor can be dragged.
///
/// Three columns: below that the grid starts wrapping pairs awkwardly.
pub const EDITOR_MODAL_MIN_WIDTH: f32 = editor_modal_width(3.0);

/// The width `columns` of the property grid occupy, with the chrome around
/// them: the layer's own 16px padding each side, the editor's 5px, and the
/// scrollbar gutter.
const fn editor_modal_width(columns: f32) -> f32 {
    columns * FIELD_WIDTH + (columns - 1.0) * SECTION_SPACING + 2.0 * (16.0 + 5.0)
        + SCROLLBAR_GUTTER
}

/// How tall the widget editor opens.
///
/// Tall enough for the code preview under the settings to be worth having,
/// short enough to fit a 768px-high display with the layer's margin around it.
pub const EDITOR_MODAL_HEIGHT: f32 = 720.0;

/// The room a vertical scrollbar needs beside the content it scrolls.
///
/// iced draws the bar inside the scrollable's own bounds and over whatever is
/// beneath it, so anything running to the full width ends up underneath it.
/// The bar is 10px wide by default; the rest is breathing room.
pub const SCROLLBAR_GUTTER: f32 = 14.0;

/// A vertical scrollable that leaves room for its own scrollbar.
///
/// Prefer this over a bare `scrollable` anywhere the content fills the width.
pub fn scroll_y<'a, M>(
    content: impl Into<Element<'a, M>>,
) -> iced::widget::Scrollable<'a, M>
where
    M: 'a,
{
    scrollable(container(content.into()).padding(padding::right(SCROLLBAR_GUTTER)))
}

/// A numeric entry sized for the property grid.
pub fn float_input<'a, M: Clone + 'a>(
    value: f32,
    on_change: impl Fn(f32) -> M + 'a,
) -> Element<'a, M> {
    text_input("0", format!("{value}"))
        .on_input(move |raw| on_change(raw.trim().parse::<f32>().unwrap_or(0.0)))
        .width(FIELD_WIDTH)
        .into()
}

/// A labelled text entry sized for the property grid.
pub fn text_field<'a, M: Clone + 'a>(
    label: &'a str,
    placeholder: &'a str,
    value: impl iced::advanced::text::IntoFragment<'a>,
    on_change: impl Fn(String) -> M + 'a,
) -> Element<'a, M> {
    field(
        label,
        text_input(placeholder, value)
            .on_input(on_change)
            .width(FIELD_WIDTH),
    )
}

/// A labelled pick list sized for the property grid.
pub fn pick_field<'a, T, L, M>(
    label: &'a str,
    selected: T,
    options: L,
    on_select: impl Fn(T) -> M + 'a,
) -> Element<'a, M>
where
    T: ToString + PartialEq + Clone + 'a,
    L: std::borrow::Borrow<[T]> + 'a,
    M: Clone + 'a,
{
    field(
        label,
        pick_list(Some(selected), options, ToString::to_string)
            .on_select(on_select)
            .width(FIELD_WIDTH),
    )
}

/// A labelled slider with its value beside it, sized for the property grid.
///
/// The readout is what makes a slider worth having over a number entry, so it
/// travels with it rather than being written out at each call.
pub fn slider_field<'a, M: Clone + 'a>(
    label: &'a str,
    range: std::ops::RangeInclusive<f32>,
    value: f32,
    on_change: impl Fn(f32) -> M + 'a,
) -> Element<'a, M> {
    // The readout is fixed-width so a row of sliders lines up whatever the
    // numbers happen to be.
    const READOUT_WIDTH: f32 = 38.0;

    field(
        label,
        row![
            slider(range, value, on_change)
                .step(1.0)
                .width(FIELD_WIDTH - READOUT_WIDTH - LABEL_SPACING),
            text(format!("{value:.0}"))
                .size(LABEL_SIZE)
                .width(READOUT_WIDTH),
        ]
        .spacing(LABEL_SPACING)
        .align_y(Alignment::Center),
    )
}

/// A checkbox laid out to line up with the labelled fields beside it.
///
/// The label sits inside the checkbox rather than above it, so an empty label
/// row keeps it on the same baseline as the controls it wraps alongside.
pub fn toggle_field<'a, M: Clone + 'a>(
    label: &'a str,
    checked: bool,
    on_toggle: impl Fn(bool) -> M + 'a,
) -> Element<'a, M> {
    column![
        text(" ").size(LABEL_SIZE),
        container(checkbox(checked).label(label).on_toggle(on_toggle))
            .height(CONTROL_HEIGHT)
            .align_y(Alignment::Center),
    ]
    .spacing(LABEL_SPACING)
    .width(FIELD_WIDTH)
    .into()
}

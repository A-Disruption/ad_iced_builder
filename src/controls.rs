use iced::Widget as _;
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
use iced::{Alignment, Border, Color, Element, Length, Padding, Theme, padding};
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

/// The most icons the picker grid draws at once.
///
/// Every family is enabled, which is eighty thousand icons; Nerd Fonts alone is
/// eleven thousand. Building a button for each would stall the frame — so the
/// grid draws a window of this size and *slides* it as you scroll, rather than
/// capping the results and asking for a narrower search. A search matching
/// twenty thousand icons costs the same as one matching sixteen hundred.
const ICON_WINDOW: usize = 1600;

/// How far the window slides each time a scroll reaches one of its ends.
const ICON_STEP: usize = 400;

/// How close to an end counts as asking for the next or previous step.
const ICON_LOAD_MORE_AT: f32 = 0.85;
const ICON_LOAD_BACK_AT: f32 = 0.15;

/// Identifies the grid, so a slide can put the view back where it was.
pub const ICON_GRID_ID: &str = "icon-picker-grid";

/// Reports a slide of one widget's icon grid.
///
/// The same for all three pickers: what moves is the one that is open.
pub fn icon_scroll_handler(
    widget_id: WidgetId,
) -> impl Fn(usize, Option<f32>) -> Message + 'static {
    move |start, snap| Message::IconGridScrolled(widget_id, start, snap)
}

/// Where the window lands after a scroll, and how far to nudge the view back.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IconSlide {
    pub start: usize,
    pub snap: Option<f32>,
}

/// Advances or retreats the icon window after a scroll to `offset`.
///
/// Because the window is a fixed size, moving it by `n` icons shifts everything
/// on screen by `n / ICON_WINDOW` of the scrollable's height. Snapping back by
/// that much leaves the icons under the cursor where they were, so a slide
/// reads as ordinary scrolling rather than a jump.
///
/// Separate from the view because a `scrollable::Viewport` cannot be built by
/// hand, and the arithmetic is the part worth testing.
pub fn slide_icon_window(start: usize, total: usize, offset: f32) -> IconSlide {
    if total <= ICON_WINDOW {
        return IconSlide {
            start: 0,
            snap: None,
        };
    }

    let shift = |from: usize, to: usize| {
        let moved = to.abs_diff(from) as f32 / ICON_WINDOW as f32;
        let snapped = if to > from {
            offset - moved
        } else {
            offset + moved
        };

        IconSlide {
            start: to,
            snap: Some(snapped.clamp(0.0, 1.0)),
        }
    };

    let last = total - ICON_WINDOW;

    if offset >= ICON_LOAD_MORE_AT && start < last {
        return shift(start, (start + ICON_STEP).min(last));
    }

    if offset <= ICON_LOAD_BACK_AT && start > 0 {
        return shift(start, start.saturating_sub(ICON_STEP));
    }

    IconSlide { start, snap: None }
}

/// The icon picker: a family selector, a search box, and a grid of hits.
///
/// The three widgets that carry an icon — `text_input`, `combo_box`, and the
/// `icon` widget itself — each have their own property messages, so the two
/// choices a caller owns arrive as closures.
pub fn icon_picker<'a>(
    family: &'a str,
    name: &'a str,
    filter: &'a str,
    window: usize,
    on_family: impl Fn(String) -> Message + 'a,
    on_filter: impl Fn(String) -> Message + 'a,
    on_scroll: impl Fn(usize, Option<f32>) -> Message + 'a,
    on_select: impl Fn(String, String, u32) -> Message + 'a,
) -> Element<'a, Message> {
    let needle = filter.to_lowercase();

    let matches = icon::in_family(family)
        .filter(|icon| needle.is_empty() || icon.name.contains(needle.as_str()));

    let total = matches.clone().count();

    // Clamped rather than trusted: the window is left where it was when the
    // family changes, and the new family may hold fewer icons than that.
    let start = window.min(total.saturating_sub(ICON_WINDOW));

    let icon_buttons: Vec<Element<'a, Message>> = matches
        .skip(start)
        .take(ICON_WINDOW)
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
            .boxed()
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

    // The window slides rather than capping, so this says where you are in the
    // results rather than what was thrown away.
    let shown = ICON_WINDOW.min(total.saturating_sub(start));

    let tally = if total > ICON_WINDOW {
        format!("{}\u{2013}{} of {total} icons", start + 1, start + shown)
    } else {
        format!("{total} icons")
    };

    let picker_content = column![
        container(column![family_picker, search].spacing(4))
            .padding(padding::horizontal(5.0).top(5.0)),
        container(text(tally).size(LABEL_SIZE)).padding(padding::horizontal(5.0)),
        scrollable(
            Row::with_children(icon_buttons)
                .spacing(4)
                .padding(padding::all(10.0).right(10.0 + SCROLLBAR_GUTTER))
                .wrap(),
        )
        .id(iced::advanced::widget::Id::new(ICON_GRID_ID))
        // Where the window should land is worked out here, because this is
        // where the result count is known — the handler only has the viewport.
        .on_scroll(move |viewport| {
            let slide = slide_icon_window(start, total, viewport.viewport.relative_offset().y);

            on_scroll(slide.start, slide.snap)
        }),
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
        .boxed()
}

pub fn container_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    theme: &Theme,
    type_system: &'a TypeSystem,
    custom_styles: &'a CustomThemes,
    _preview_content: &'a text_editor::Content,
    view_id: Uuid,
    views: &'a BTreeMap<Uuid, AppView>,
) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    let content = column![
        custom_style_picker(custom_styles, widget_id, props, ThemePaneEnum::Container),
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
                .boxed()
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
                .boxed()
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
                .boxed()
            }
        },
        // Padding Controls
        padding_controls(props.padding, widget_id, props.padding_mode, theme,),
        advanced_section(widget_id, props, theme),
    ]
    .spacing(MAIN_SPACING)
    .boxed();

    content
}

pub fn row_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    theme: &Theme,
    _type_system: &'a TypeSystem,
    _custom_styles: &CustomThemes,
    _preview_content: &'a text_editor::Content,
) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    let content = column![
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
    .boxed();

    content
}

pub fn column_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    theme: &Theme,
    _type_system: &'a TypeSystem,
    _custom_styles: &CustomThemes,
    _preview_content: &'a text_editor::Content,
) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    let content = column![
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
    .boxed();

    content
}

pub fn button_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    theme: &Theme,
    type_system: &'a TypeSystem,
    custom_styles: &'a CustomThemes,
    _preview_content: &'a text_editor::Content,
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
                .boxed(),
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
                .boxed(),
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
                .boxed(),
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
                .boxed(),
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
        .boxed();

    content
}

pub fn text_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    _theme: &Theme,
    _type_system: &'a TypeSystem,
    _custom_styles: &CustomThemes,
    _preview_content: &'a text_editor::Content,
) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    let content = column![
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
                        .width(Length::Fill),
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
    .boxed();

    content
}

pub fn text_input_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    theme: &Theme,
    _type_system: &'a TypeSystem,
    custom_styles: &'a CustomThemes,
    _preview_content: &'a text_editor::Content,
) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    let content = column![
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
                props.icon_picker_window,
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
                icon_scroll_handler(widget_id),
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
                                    .width(Length::Fill),
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
    .boxed();

    content
}

pub fn text_editor_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    theme: &Theme,
    _type_system: &'a TypeSystem,
    custom_styles: &'a CustomThemes,
    _preview_content: &'a text_editor::Content,
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
                            .width(Length::Fill),
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
    .boxed();

    content
}

pub fn checkbox_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    theme: &Theme,
    type_system: &'a TypeSystem,
    custom_styles: &'a CustomThemes,
    _preview_content: &'a text_editor::Content,
    views: &'a BTreeMap<Uuid, AppView>,
    view_id: Uuid,
) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    let content = column![
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
    .boxed();

    content
}

pub fn toggler_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    theme: &Theme,
    type_system: &'a TypeSystem,
    custom_styles: &'a CustomThemes,
    _preview_content: &'a text_editor::Content,
    views: &'a BTreeMap<Uuid, AppView>,
    view_id: Uuid,
) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    let content = column![
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
    .boxed();

    content
}

pub fn radio_controls<'a>(
    hierarchy: &'a WidgetHierarchy,
    widget_id: WidgetId,
    theme: &Theme,
    type_system: &'a TypeSystem,
    custom_styles: &'a CustomThemes,
    _preview_content: &'a text_editor::Content,
    views: &'a BTreeMap<Uuid, AppView>,
    view_id: Uuid,
) -> Element<'a, Message> {
    let widget = hierarchy.get_widget_by_id(widget_id).unwrap();
    let props = &widget.properties;

    let content = column![
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
                        .boxed()
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
    .boxed();

    content
}

pub fn picklist_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    theme: &Theme,
    type_system: &'a TypeSystem,
    custom_styles: &'a CustomThemes,
    _preview_content: &'a text_editor::Content,
    views: &'a BTreeMap<Uuid, AppView>,
    view_id: Uuid,
) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    let content = column![
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
                    .width(Length::Fill),
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
                        .boxed()
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
    .boxed();

    content
}

pub fn slider_controls<'a>(
    hierarchy: &'a WidgetHierarchy,
    widget_id: WidgetId,
    theme: &Theme,
    type_system: &'a TypeSystem,
    custom_styles: &'a CustomThemes,
    _preview_content: &'a text_editor::Content,
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
                        .width(Length::Fill),
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
    .boxed();

    content
}

pub fn vertical_slider_controls<'a>(
    hierarchy: &'a WidgetHierarchy,
    widget_id: WidgetId,
    theme: &Theme,
    type_system: &'a TypeSystem,
    custom_styles: &'a CustomThemes,
    _preview_content: &'a text_editor::Content,
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
                        .width(Length::Fill),
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
    .boxed();

    content
}

pub fn rule_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    _theme: &Theme,
    _type_system: &'a TypeSystem,
    custom_styles: &'a CustomThemes,
    _preview_content: &'a text_editor::Content,
) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).unwrap();
    let p = &widget.properties;

    let content = column![
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
                            .width(Length::Fill),
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
                                .boxed()
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
    .boxed();

    content
}

pub fn scrollable_controls<'a>(
    hierarchy: &'a WidgetHierarchy,
    widget_id: WidgetId,
    _theme: &Theme,
    _type_system: &'a TypeSystem,
    _custom_styles: &CustomThemes,
    _preview_content: &'a text_editor::Content,
) -> Element<'a, Message> {
    let widget = hierarchy.get_widget_by_id(widget_id).unwrap();
    let props = &widget.properties;

    let content = column![
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
    .boxed();

    content
}

pub fn space_controls<'a>(
    hierarchy: &'a WidgetHierarchy,
    widget_id: WidgetId,
    _theme: &Theme,
    _type_system: &'a TypeSystem,
    _custom_styles: &CustomThemes,
    _preview_content: &'a text_editor::Content,
) -> Element<'a, Message> {
    let widget = hierarchy.get_widget_by_id(widget_id).unwrap();
    let props = &widget.properties;

    let content = column![
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
    .boxed();

    content
}

pub fn progress_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    theme: &Theme,
    type_system: &'a TypeSystem,
    custom_styles: &'a CustomThemes,
    _preview_content: &'a text_editor::Content,
    views: &'a BTreeMap<Uuid, AppView>,
    view_id: Uuid,
) -> Element<'a, Message> {
    let w = h.get_widget_by_id(widget_id).unwrap();
    let p = &w.properties;
    let girth_str = format!("{:.0}", p.progress_girth);

    let clamp_step = ((p.progress_max - p.progress_min) / 100.0).abs().max(0.001);

    let content = column![
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
                        .width(Length::Fill),
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
    .boxed();

    content
}

pub fn image_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    _theme: &Theme,
    _type_system: &'a TypeSystem,
    _custom_styles: &CustomThemes,
    _preview_content: &'a text_editor::Content,
) -> Element<'a, Message> {
    let w = h.get_widget_by_id(widget_id).unwrap();
    let props = &w.properties;

    let content = column![
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
    .boxed();

    content
}

pub fn svg_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    _theme: &Theme,
    _type_system: &'a TypeSystem,
    _custom_styles: &CustomThemes,
    _preview_content: &'a text_editor::Content,
) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).unwrap();
    let props = &widget.properties;

    let content = column![
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
    .boxed();

    content
}

pub fn tooltip_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    _theme: &Theme,
    _type_system: &'a TypeSystem,
    _custom_styles: &CustomThemes,
    _preview_content: &'a text_editor::Content,
) -> Element<'a, Message> {
    let w = h.get_widget_by_id(widget_id).unwrap();
    let p = &w.properties;

    let content = column![
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
    .boxed();

    content
}

pub fn combobox_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    theme: &Theme,
    type_system: &'a TypeSystem,
    custom_styles: &'a CustomThemes,
    _preview_content: &'a text_editor::Content,
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
                                                .boxed()
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
                            .boxed()
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
                props.icon_picker_window,
                move |family| {
                    Message::PropertyChanged(widget_id, PropertyChange::ComboBoxIconFamily(family))
                },
                move |v| {
                    Message::PropertyChanged(widget_id, PropertyChange::ComboBoxIconPickerFilter(v))
                },
                icon_scroll_handler(widget_id),
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
                                    .width(Length::Fill),
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
    .boxed();

    content
}

pub fn markdown_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    _theme: &Theme,
    _type_system: &'a TypeSystem,
    _custom_styles: &CustomThemes,
    _preview_content: &'a text_editor::Content,
) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    let content = column![
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
    .boxed();

    content
}

pub fn qrcode_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    _theme: &Theme,
    _type_system: &'a TypeSystem,
    _custom_styles: &CustomThemes,
    _preview_content: &'a text_editor::Content,
) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    let content = column![
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
    .boxed();

    content
}

pub fn stack_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    _theme: &Theme,
    _type_system: &'a TypeSystem,
    _custom_styles: &CustomThemes,
    _preview_content: &'a text_editor::Content,
) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    let content = column![
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
    .boxed();

    content
}

/// The name of the widgets crate's Material Design 3 style.
///
/// Behind that crate's `m3_theme` feature, which the builder turns on. Unlike
/// the swatch presets beside it, an M3 style maps every field of the widget's
/// `Style` onto a *role*, so it tracks the user's seed colour and light/dark
/// mode rather than one palette entry.
pub(crate) const MATERIAL_STYLE: &str = "Material";

fn collapsible_style_options() -> Vec<String> {
    [
        "Default",
        "Primary",
        "Success",
        "Danger",
        "Warning",
        MATERIAL_STYLE,
    ]
    .into_iter()
    .map(str::to_string)
    .collect()
}

fn overlay_style_options() -> Vec<String> {
    ["Primary", "Success", "Danger", "Warning", "Blank", MATERIAL_STYLE]
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
    _preview_content: &'a text_editor::Content,
    views: &'a BTreeMap<Uuid, AppView>,
    view_id: Uuid,
) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;
    let style_options = collapsible_style_options();

    let content = column![
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
                    .width(Length::Fill),
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
                    .width(Length::Fill),
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
    .boxed();

    content
}

pub fn collapsible_group_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    _theme: &Theme,
    _type_system: &'a TypeSystem,
    _custom_styles: &CustomThemes,
    _preview_content: &'a text_editor::Content,
) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    let content = column![
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
    .boxed();

    content
}

pub fn overlay_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    theme: &Theme,
    type_system: &'a TypeSystem,
    custom_styles: &'a CustomThemes,
    _preview_content: &'a text_editor::Content,
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
                .width(Length::Fill),
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
        // All four take an `id`, and the generated code emits one when it is
        // set, so the panel has to be able to set it.
        widget_id_control(widget_id, props.widget_id.clone(), theme),
    ]
    .spacing(SECTION_SPACING);

    // ── Appearance, shared by all four ───────────────────────────────────────
    // A toast's style is per-variant, so the swatch presets cannot apply to
    // it — one colour for every toast would flatten error, success and
    // warning into each other. Only the M3 style, which keeps them apart,
    // is offered.
    let style_options = if is_toast {
        vec![MATERIAL_STYLE.to_string()]
    } else {
        overlay_style_options()
    };

    let mut appearance = vec![
        field(
            "Panel Style",
            pick_list(
                props.overlay_style_name.clone(),
                style_options,
                ToString::to_string,
            )
            .on_select(move |selection| {
                Message::PropertyChanged(widget_id, PropertyChange::OverlayStyle(Some(selection)))
            })
            .placeholder("default")
            .width(Length::Fill),
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
            .width(Length::Fill),
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
            .width(Length::Fill),
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
            .width(Length::Fill),
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
                .width(Length::Fill),
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
            .width(Length::Fill),
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
                    .width(Length::Fill),
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
            .width(Length::Fill),
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
                .width(Length::Fill),
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
                        .width(Length::Fill),
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

            rows.boxed()
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
            .boxed()
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
            .width(Length::Fill),
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
    let mut sections: Vec<Element<'a, Message>> = vec![header.boxed()];

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
        .boxed();

    content
}

pub fn date_picker_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    theme: &Theme,
    type_system: &'a TypeSystem,
    custom_styles: &'a CustomThemes,
    _preview_content: &'a text_editor::Content,
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
    .boxed();

    content
}

pub fn split_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    _theme: &Theme,
    _type_system: &'a TypeSystem,
    _custom_styles: &'a CustomThemes,
    _preview_content: &'a text_editor::Content,
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
                .width(Length::Fill),
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
    .boxed();

    content
}

pub fn float_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    theme: &Theme,
    _type_system: &'a TypeSystem,
    _custom_styles: &'a CustomThemes,
    _preview_content: &'a text_editor::Content,
) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    let content = column![
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
                            .width(Length::Fill),
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
    .boxed();

    content
}

pub fn responsive_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    theme: &Theme,
    _type_system: &'a TypeSystem,
    _custom_styles: &'a CustomThemes,
    _preview_content: &'a text_editor::Content,
) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    let content = column![
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
    .boxed();

    content
}

pub fn sensor_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    theme: &Theme,
    _type_system: &'a TypeSystem,
    _custom_styles: &'a CustomThemes,
    _preview_content: &'a text_editor::Content,
) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    let content = column![
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
    .boxed();

    content
}

pub fn mousearea_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    theme: &Theme,
    _type_system: &'a TypeSystem,
    _custom_styles: &CustomThemes,
    _preview_content: &'a text_editor::Content,
) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    let content = column![
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
    .boxed();

    content
}

pub fn themer_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    _theme: &Theme,
    _type_system: &'a TypeSystem,
    _custom_styles: &CustomThemes,
    _preview_content: &'a text_editor::Content,
) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    let content = column![
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
                        .width(Length::Fill),
                )]),
                text("Themer applies a theme to all its children.")
                    .size(LABEL_SIZE)
                    .color(Color::from_rgb(0.6, 0.6, 0.6)),
            ]
            .spacing(LABEL_SPACING),
        ),
    ]
    .spacing(MAIN_SPACING)
    .boxed();

    content
}

pub fn grid_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    _theme: &Theme,
    _type_system: &'a TypeSystem,
    _custom_styles: &CustomThemes,
    _preview_content: &'a text_editor::Content,
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
    .boxed();

    content
}

pub fn pin_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    _theme: &Theme,
    _type_system: &'a TypeSystem,
    _custom_styles: &CustomThemes,
    _preview_content: &'a text_editor::Content,
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
    .boxed();

    content
}


/// The `advanced-table` data grid.
///
/// Distinct from [`table_controls`], which is iced's own `table`: a static
/// layout with no scrolling, sorting or selection of its own. Everything here
/// that reports something back adds a field to the generated app and a message
/// that writes it — the widget holds no state, by design.
pub fn data_table_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    theme: &Theme,
    type_system: &'a TypeSystem,
    _custom_styles: &CustomThemes,
    _preview_content: &'a text_editor::Content,
) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    let struct_names: Vec<String> = type_system
        .all_structs()
        .iter()
        .map(|def| def.name.clone())
        .collect();

    let selected_struct = props
        .data_table_referenced_struct
        .and_then(|id| type_system.get_struct(id))
        .map(|def| def.name.clone());

    // ── Data source ──────────────────────────────────────────────────────────
    let source: Element<'a, Message> = if struct_names.is_empty() {
        text("Define a struct in the type editor first — its fields become the columns.")
            .size(LABEL_SIZE)
            .style(text::warning)
            .boxed()
    } else {
        field(
            "Struct",
            pick_list(selected_struct, struct_names, ToString::to_string)
                .on_select(move |name| {
                    let id = type_system.get_struct_by_name(&name).map(|def| def.id);

                    Message::PropertyChanged(widget_id, PropertyChange::DataTableStruct(id))
                })
                .placeholder("choose a struct")
                .width(Length::Fill),
        )
    };

    let mut sections = vec![
        text("Data Table Properties").size(TITLE_SIZE).boxed(),
        section("Data Source", source),
    ];

    // Everything below is per-column, so there is nothing to show until the
    // columns exist.
    if !props.data_table_columns.is_empty() {
        sections.push(section(
            "Columns",
            column(
                props
                    .data_table_columns
                    .iter()
                    .enumerate()
                    .map(|(index, spec)| {
                        data_table_column_row(widget_id, index, spec, props)
                    })
                    .collect::<Vec<_>>(),
            )
            .spacing(SECTION_SPACING),
        ));

        sections.push(section(
            "Header Groups",
            column![
                text(
                    "A group is a header cell spanning the columns assigned to it. \
                     Joining one moves the column beside the group's others, since \
                     a group needs an unbroken run to draw its label across."
                )
                .size(LABEL_SIZE)
                .style(text::secondary),
                column(
                    props
                        .data_table_groups
                        .iter()
                        .enumerate()
                        .map(|(index, group)| data_table_group_row(widget_id, index, group))
                        .collect::<Vec<_>>(),
                )
                .spacing(LABEL_SPACING),
                button(text("+ group").size(LABEL_SIZE))
                    .on_press(Message::PropertyChanged(
                        widget_id,
                        PropertyChange::DataTableGroupAdded
                    ))
                    .style(button::secondary),
            ]
            .spacing(LABEL_SPACING),
        ));
    }

    sections.push(section(
        "Appearance",
        field_grid(vec![
            float_field("Row spacing", props.data_table_spacing, move |value| {
                Message::PropertyChanged(widget_id, PropertyChange::DataTableSpacing(value))
            }),
            float_field("Cell padding X", props.data_table_padding_x, move |value| {
                Message::PropertyChanged(widget_id, PropertyChange::DataTablePaddingX(value))
            }),
            float_field("Cell padding Y", props.data_table_padding_y, move |value| {
                Message::PropertyChanged(widget_id, PropertyChange::DataTablePaddingY(value))
            }),
            float_field(
                "Min row height",
                props.data_table_min_row_height,
                move |value| Message::PropertyChanged(
                    widget_id,
                    PropertyChange::DataTableMinRowHeight(value)
                ),
            ),
            toggle_field("Bold headers", props.data_table_bold_headers, move |value| {
                Message::PropertyChanged(widget_id, PropertyChange::DataTableBoldHeaders(value))
            }),
        ]),
    ));

    // ── Columns and fit ──────────────────────────────────────────────────────
    let mut fit = vec![
        pick_field(
            "When columns overflow",
            props.data_table_overflow,
            DataTableOverflow::ALL,
            move |value| {
                Message::PropertyChanged(
                    widget_id,
                    PropertyChange::DataTableOverflowMode(value),
                )
            },
        ),
        toggle_field("Resizable columns", props.data_table_resizable, move |value| {
            Message::PropertyChanged(widget_id, PropertyChange::DataTableResizable(value))
        }),
    ];

    if props.data_table_resizable {
        fit.push(float_field(
            "Min column width",
            props.data_table_min_column_width,
            move |value| {
                Message::PropertyChanged(
                    widget_id,
                    PropertyChange::DataTableMinColumnWidth(value),
                )
            },
        ));
    }

    fit.push(toggle_field(
        "Measure every row",
        props.data_table_measure_all,
        move |value| {
            Message::PropertyChanged(widget_id, PropertyChange::DataTableMeasureAll(value))
        },
    ));

    // Measuring every row is exact and costs a pass over all of them; the
    // sample is the alternative, so the two are never both offered.
    if !props.data_table_measure_all {
        fit.push(float_field(
            "Rows measured",
            props.data_table_measure_sample,
            move |value| {
                Message::PropertyChanged(
                    widget_id,
                    PropertyChange::DataTableMeasureSample(value),
                )
            },
        ));
    }

    sections.push(section("Column Fit", field_grid(fit)));

    // ── Scrolling ────────────────────────────────────────────────────────────
    sections.push(section(
        "Scrolling",
        column![
            text(
                "The body scrolls under the sticky header only when the table has a \
                 bounded height — Fill, a fixed height, or a sized container. Same \
                 rule as `scrollable`."
            )
            .size(LABEL_SIZE)
            .style(text::secondary),
            field_grid(vec![
                pick_field(
                    "Vertical",
                    props.data_table_vertical_scroll,
                    DataTableScrollPolicy::ALL,
                    move |value| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::DataTableVerticalScroll(value)
                    ),
                ),
                pick_field(
                    "Horizontal",
                    props.data_table_horizontal_scroll,
                    DataTableScrollPolicy::ALL,
                    move |value| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::DataTableHorizontalScroll(value)
                    ),
                ),
                float_field(
                    "Scrollbar width",
                    props.data_table_scrollbar_width,
                    move |value| Message::PropertyChanged(
                        widget_id,
                        PropertyChange::DataTableScrollbarWidth(value)
                    ),
                ),
            ]),
        ]
        .spacing(LABEL_SPACING),
    ));

    // ── Selection and interaction ────────────────────────────────────────────
    let mut interaction = vec![
        pick_field(
            "Rows",
            props.data_table_row_selection,
            DataTableSelection::ALL,
            move |value| {
                Message::PropertyChanged(
                    widget_id,
                    PropertyChange::DataTableRowSelection(value),
                )
            },
        ),
        pick_field(
            "Columns",
            props.data_table_column_selection,
            DataTableSelection::ALL,
            move |value| {
                Message::PropertyChanged(
                    widget_id,
                    PropertyChange::DataTableColumnSelection(value),
                )
            },
        ),
        pick_field(
            "Cells",
            props.data_table_cell_selection,
            DataTableSelection::ALL,
            move |value| {
                Message::PropertyChanged(
                    widget_id,
                    PropertyChange::DataTableCellSelection(value),
                )
            },
        ),
    ];

    // With cell selection on, a body click means "this cell" — the gutter is
    // then the only place a click still means "this row", which is the whole
    // reason it exists.
    if props.data_table_cell_selection.is_on() && props.data_table_row_selection.is_on() {
        interaction.push(float_field(
            "Row gutter",
            props.data_table_gutter,
            move |value| {
                Message::PropertyChanged(widget_id, PropertyChange::DataTableGutter(value))
            },
        ));
    }

    interaction.push(toggle_field(
        "Sortable columns",
        props.data_table_sorting,
        move |value| Message::PropertyChanged(widget_id, PropertyChange::DataTableSorting(value)),
    ));

    interaction.push(toggle_field(
        "Drag to reorder",
        props.data_table_reorderable,
        move |value| {
            Message::PropertyChanged(widget_id, PropertyChange::DataTableReorderable(value))
        },
    ));

    interaction.push(toggle_field(
        "Report Ctrl+C",
        props.data_table_on_copy,
        move |value| Message::PropertyChanged(widget_id, PropertyChange::DataTableOnCopy(value)),
    ));

    interaction.push(toggle_field(
        "Report right-click",
        props.data_table_on_right_click,
        move |value| {
            Message::PropertyChanged(widget_id, PropertyChange::DataTableOnRightClick(value))
        },
    ));

    let sorting_note: Element<'a, Message> = if props.data_table_sorting {
        text(
            "The table draws the control and reports the click — it does not \
             reorder the rows. Sort your own data in update() and hand them \
             back in the new order."
        )
        .size(LABEL_SIZE)
        .style(text::secondary)
        .boxed()
    } else {
        nothing()
    };

    sections.push(section(
        "Selection",
        column![field_grid(interaction), sorting_note].spacing(LABEL_SPACING),
    ));

    sections.push(size_controls_scrollable_aware(
        props.width,
        move |length| Message::PropertyChanged(widget_id, PropertyChange::Width(length)),
        props.height,
        move |length| Message::PropertyChanged(widget_id, PropertyChange::Height(length)),
        h,
        widget_id,
    ));
    sections.push(advanced_section(widget_id, props, theme));

    column(sections).spacing(SECTION_SPACING).boxed()
}

/// One column: what it shows, how wide it is, and where it sits.
fn data_table_column_row<'a>(
    widget_id: WidgetId,
    index: usize,
    spec: &'a DataTableColumn,
    props: &'a Properties,
) -> Element<'a, Message> {
    let mut fields = vec![
        text_field(
            "Header",
            &spec.field,
            spec.header.clone(),
            move |value| {
                Message::PropertyChanged(
                    widget_id,
                    PropertyChange::DataTableColumnHeader(index, value),
                )
            },
        ),
        pick_field(
            "Width",
            spec.sizing,
            DataTableSizing::ALL,
            move |value| {
                Message::PropertyChanged(
                    widget_id,
                    PropertyChange::DataTableColumnSizing(index, value),
                )
            },
        ),
    ];

    // Each sizing mode reads a different pair of numbers, so only its own are
    // offered.
    match spec.sizing {
        DataTableSizing::Fixed => fields.push(float_field("Exactly", spec.width, move |value| {
            Message::PropertyChanged(
                widget_id,
                PropertyChange::DataTableColumnWidth(index, value),
            )
        })),
        DataTableSizing::Fit => {
            fields.push(float_field("At least", spec.min_width, move |value| {
                Message::PropertyChanged(
                    widget_id,
                    PropertyChange::DataTableColumnMinWidth(index, value),
                )
            }));
            fields.push(float_field(
                "At most (0 = no cap)",
                spec.max_width,
                move |value| {
                    Message::PropertyChanged(
                        widget_id,
                        PropertyChange::DataTableColumnMaxWidth(index, value),
                    )
                },
            ));
        }
        DataTableSizing::Fill => {
            fields.push(float_field("Share", f32::from(spec.weight), move |value| {
                Message::PropertyChanged(
                    widget_id,
                    PropertyChange::DataTableColumnWeight(index, value),
                )
            }));
            fields.push(float_field("Shrink floor", spec.min_width, move |value| {
                Message::PropertyChanged(
                    widget_id,
                    PropertyChange::DataTableColumnMinWidth(index, value),
                )
            }));
        }
    }

    fields.push(pick_field(
        "Align",
        spec.align,
        vec![
            ContainerAlignX::Left,
            ContainerAlignX::Center,
            ContainerAlignX::Right,
        ],
        move |value| {
            Message::PropertyChanged(
                widget_id,
                PropertyChange::DataTableColumnAlign(index, value),
            )
        },
    ));

    // Only offered when the table reports sort clicks at all; a sort control
    // nothing listens to would draw and do nothing.
    if props.data_table_sorting {
        fields.push(toggle_field("Sortable", spec.sortable, move |value| {
            Message::PropertyChanged(
                widget_id,
                PropertyChange::DataTableColumnSortable(index, value),
            )
        }));
    }

    // A grouped column takes its stickiness from the group — that is what
    // keeps a group from being split by the frozen boundary.
    if spec.group.is_none() {
        fields.push(toggle_field("Frozen", spec.sticky, move |value| {
            Message::PropertyChanged(
                widget_id,
                PropertyChange::DataTableColumnSticky(index, value),
            )
        }));
    }

    if !props.data_table_groups.is_empty() {
        let groups: Vec<String> = std::iter::once("—".to_string())
            .chain(
                props
                    .data_table_groups
                    .iter()
                    .enumerate()
                    .map(|(position, group)| data_table_group_label(position, group)),
            )
            .collect();

        let selected = spec
            .group
            .and_then(|group| props.data_table_groups.get(group).map(|def| (group, def)))
            .map(|(group, def)| data_table_group_label(group, def))
            .unwrap_or_else(|| "—".to_string());

        let labels = groups.clone();

        fields.push(field(
            "Group",
            pick_list(Some(selected), groups, ToString::to_string)
                .on_select(move |choice: String| {
                    // Position in the list, less the leading "no group".
                    let group = labels
                        .iter()
                        .position(|label| *label == choice)
                        .filter(|position| *position > 0)
                        .map(|position| position - 1);

                    Message::PropertyChanged(
                        widget_id,
                        PropertyChange::DataTableColumnGroup(index, group),
                    )
                })
                .width(Length::Fill),
        ));
    }

    let order = row![
        button(text("\u{2191}").size(LABEL_SIZE))
            .on_press(Message::PropertyChanged(
                widget_id,
                PropertyChange::DataTableColumnMoved(index, true)
            ))
            .style(button::secondary),
        button(text("\u{2193}").size(LABEL_SIZE))
            .on_press(Message::PropertyChanged(
                widget_id,
                PropertyChange::DataTableColumnMoved(index, false)
            ))
            .style(button::secondary),
    ]
    .spacing(LABEL_SPACING);

    container(
        column![
            row![
                text(format!("{}. {}", index + 1, spec.field)).size(LABEL_SIZE),
                space::horizontal(),
                order,
            ]
            .align_y(Alignment::Center),
            field_grid(fields),
        ]
        .spacing(LABEL_SPACING),
    )
    .padding(6)
    .style(container::bordered_box)
    .boxed()
}

/// One header group: its label, and whether it and its columns are frozen.
fn data_table_group_row<'a>(
    widget_id: WidgetId,
    index: usize,
    group: &'a DataTableGroup,
) -> Element<'a, Message> {
    row![
        text_field("Header", "Group", group.header.clone(), move |value| {
            Message::PropertyChanged(
                widget_id,
                PropertyChange::DataTableGroupHeader(index, value),
            )
        }),
        toggle_field("Frozen", group.sticky, move |value| {
            Message::PropertyChanged(
                widget_id,
                PropertyChange::DataTableGroupSticky(index, value),
            )
        }),
        button(text("\u{2715}").size(LABEL_SIZE))
            .on_press(Message::PropertyChanged(
                widget_id,
                PropertyChange::DataTableGroupRemoved(index)
            ))
            .style(button::danger),
    ]
    .spacing(SECTION_SPACING)
    .align_y(Alignment::End)
    .boxed()
}

/// A group as it reads in a picker — numbered, since two groups may share a
/// label and the picker matches on the text.
fn data_table_group_label(index: usize, group: &DataTableGroup) -> String {
    if group.header.trim().is_empty() {
        format!("{}. (unnamed)", index + 1)
    } else {
        format!("{}. {}", index + 1, group.header.trim())
    }
}
pub fn table_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    _theme: &Theme,
    type_system: &'a TypeSystem,
    _custom_styles: &CustomThemes,
    _preview_content: &'a text_editor::Content,
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
                                    .boxed()
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
    .boxed();

    content
}

pub fn view_reference_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    theme: &Theme,
    _type_system: &'a TypeSystem,
    views: &'a BTreeMap<Uuid, AppView>,
    _custom_styles: &CustomThemes,
    _preview_content: &'a text_editor::Content,
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
                    .boxed()
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
    .boxed();

    content
}

pub fn icon_controls<'a>(
    h: &'a WidgetHierarchy,
    widget_id: WidgetId,
    _theme: &Theme,
    _type_system: &'a TypeSystem,
    _custom_styles: &CustomThemes,
    _preview_content: &'a text_editor::Content,
) -> Element<'a, Message> {
    let widget = h.get_widget_by_id(widget_id).expect("widget exists");
    let props = &widget.properties;

    let picker = icon_picker(
        &props.icon_family,
        &props.icon_name,
        &props.icon_picker_filter,
                props.icon_picker_window,
        move |family| Message::PropertyChanged(widget_id, PropertyChange::IconFamily(family)),
        move |v| Message::PropertyChanged(widget_id, PropertyChange::IconPickerFilter(v)),
        icon_scroll_handler(widget_id),
        move |family, name, codepoint| {
            Message::PropertyChanged(
                widget_id,
                PropertyChange::IconSelected(family, name, codepoint),
            )
        },
    );

    let content = column![
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
    .boxed();

    content
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
    .boxed()
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
    .boxed()
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
    .boxed()
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
            .boxed()
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
                    .spacing(5)
                    .boxed(),
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

            controls.boxed()
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

            controls.boxed()
        }

        _ => Space::new().width(0).boxed(),
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
        .boxed()
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
            .boxed()
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
            .boxed()
        }
        LengthChoice::Bounded | LengthChoice::Fluid => {
            length_detail_controls(current, can_fill, on_change)
        }
        _ => space::horizontal().boxed(),
    };

    row![picker, extra]
        .width(Length::Fill)
        .spacing(SECTION_SPACING)
        .boxed()
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
    .boxed()
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
    .boxed()
}

fn with_event_info<'a>(
    control: Element<'a, Message>,
    theme: &Theme,
    info: &'a str,
) -> Element<'a, Message> {
    row![control, information(theme, info)]
        .spacing(LABEL_SPACING)
        .align_y(Alignment::Center)
        .boxed()
}

fn event_name_row<'a>(theme: &Theme, name: &'a str, info: &'a str) -> Element<'a, Message> {
    with_event_info(text(name).size(LABEL_SIZE).boxed(), theme, info)
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
        checkbox(enabled).label(name).on_toggle(on_toggle).boxed(),
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
            .boxed(),
        None => text("off").size(LABEL_SIZE).boxed(),
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
            .boxed(),
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

/// The widget's name, for the inspector's header.
///
/// It used to be a labelled field at the top of every property panel, under a
/// title that said the type — three lines to carry one string. In the header it
/// is the title: the type is the placeholder, so an unnamed widget reads as its
/// type and naming it replaces that in place.
pub fn widget_name<'a>(
    widget_id: WidgetId,
    name: &'a str,
    placeholder: &'a str,
) -> Element<'a, Message> {
    text_input(placeholder, name)
        .on_input(move |v| Message::PropertyChanged(widget_id, PropertyChange::WidgetName(v)))
        .size(16)
        .padding(padding::left(4).right(4).top(2).bottom(2))
        .style(name_field_style)
        .width(Length::Fill)
        .boxed()
}

/// A text field that reads as a title until you touch it.
fn name_field_style(theme: &Theme, status: text_input::Status) -> text_input::Style {
    let palette = theme.palette();
    let base = text_input::default(theme, status);

    match status {
        text_input::Status::Active => text_input::Style {
            background: iced::Background::Color(iced::Color::TRANSPARENT),
            border: Border {
                color: iced::Color::TRANSPARENT,
                ..base.border
            },
            placeholder: palette.background.strong.color,
            ..base
        },
        _ => base,
    }
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
        return Space::new().boxed();
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
        Space::new().boxed()
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
            .boxed()
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
            .boxed()
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
        .boxed()
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
        .boxed()
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
                .boxed(),
            theme,
            "Use a view state field here, then update that state from the action system to switch to the alternate style.",
        ),
        condition_body,
    ]
    .spacing(LABEL_SPACING)
    .boxed()
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
        return Space::new().boxed();
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
        content = content.push(text("Alternate style").size(LABEL_SIZE).boxed()).push(
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
            .placeholder("none")
            .boxed(),
        );
    }

    content.boxed()
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
        return Space::new().boxed();
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
    .boxed()
}

/// The generated code for the selected widget, syntax-highlighted.
///
/// This used to be glued onto the bottom of every `*_controls` panel, along
/// with a scrollable and a 400px-tall box around the properties — three nested
/// scrollables by the time the inspector had wrapped the result in its own.
/// The panel is now just the properties; [`crate::views::inspector`] decides
/// what sits underneath them and how much room it gets.
pub fn widget_code_view<'a>(
    preview_content: &'a text_editor::Content,
) -> Element<'a, Message> {
    use std::sync::Arc;
    use tree_sitter_highlighter::{
        TreeSitterIcedHighlighter, TsSettings, code_gen_text_editor_style,
    };

    let settings = TsSettings {
        text: Arc::<str>::from(preview_content.text().as_str()),
    };

    text_editor(preview_content)
        .highlight_with::<TreeSitterIcedHighlighter>(
            settings,
            TreeSitterIcedHighlighter::to_format,
        )
        .on_action(Message::WidgetPreviewEdit)
        .style(code_gen_text_editor_style)
        .size(14.0)
        .height(Length::Fill)
        .boxed()
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
                    .boxed()
            } else {
                Space::new().boxed()
            };

            // A separator has no content, so it has no gutter icon either.
            let icon: Element<'a, Message> = if kind == MenuItemKind::Separator {
                Space::new().boxed()
            } else {
                let picker = icon_picker(
                    &props.menu_item_icon_family,
                    &props.menu_item_icon_name,
                    &props.menu_item_icon_picker_filter,
                    props.icon_picker_window,
                    move |family| {
                        Message::PropertyChanged(
                            widget_id,
                            PropertyChange::MenuItemIconFamily(family),
                        )
                    },
                    move |value| {
                        Message::PropertyChanged(
                            widget_id,
                            PropertyChange::MenuItemIconPickerFilter(value),
                        )
                    },
                    icon_scroll_handler(widget_id),
                    move |family, name, codepoint| {
                        Message::PropertyChanged(
                            widget_id,
                            PropertyChange::MenuItemIconSelected(family, name, codepoint),
                        )
                    },
                );

                column![
                    checkbox(props.menu_item_icon_enabled)
                        .label("Gutter icon")
                        .on_toggle(move |value| Message::PropertyChanged(
                            widget_id,
                            PropertyChange::MenuItemIconEnabled(value)
                        )),
                    if props.menu_item_icon_enabled {
                        field_grid(vec![
                            field("Icon", picker),
                            float_field(
                                "Size (0 keeps the font's)",
                                props.menu_item_icon_size,
                                move |value| Message::PropertyChanged(
                                    widget_id,
                                    PropertyChange::MenuItemIconSize(value)
                                ),
                            ),
                        ])
                    } else {
                        nothing()
                    },
                ]
                .spacing(LABEL_SPACING)
                .boxed()
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
                icon,
                rule::horizontal(1),
            ]
            .spacing(LABEL_SPACING)
            .boxed()
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
        .boxed(),
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
    // Through `field` like every other labelled control: this used to build its
    // own column at a different width, so a numeric entry never lined up with
    // the pick list beside it.
    field(
        label,
        text_input("0", format!("{value}"))
            .on_input(move |raw| on_change(raw.trim().parse::<f32>().unwrap_or(0.0)))
            .width(Length::Fill),
    )
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
    .boxed()
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

/// How wide the label column is, down the left of every field.
///
/// Fixed, because a column that each field sized for itself would not be a
/// column. Wide enough for "Align children" and "Line spacing" at
/// `LABEL_SIZE`; anything longer wraps rather than pushing the controls out of
/// line.
pub const LABEL_COLUMN: f32 = 104.0;

/// The gap between one field and the next.
///
/// Tighter than `SECTION_SPACING`: fields within a section are a list, and the
/// card's own edge already says where the section stops.
pub const FIELD_SPACING: f32 = 4.0;

/// The height of one control in the property grid.
///
/// A pick list, a text entry and a checkbox are all different heights on their
/// own, which left a wrapped row looking ragged. Boxing each to the same height
/// keeps the rows level whatever mix of controls lands on them.
pub const CONTROL_HEIGHT: f32 = 32.0;

/// A labelled control, sized to the property grid.
pub fn field<'a, M: 'a>(
    label: &'a str,
    control: impl iced::Widget<M> + 'a,
) -> Element<'a, M> {
    row![
        container(text(label).size(LABEL_SIZE).style(text::secondary))
            .width(LABEL_COLUMN)
            .height(CONTROL_HEIGHT)
            .align_y(Alignment::Center),
        container(control.boxed())
            .width(Length::Fill)
            .height(CONTROL_HEIGHT)
            .align_y(Alignment::Center),
    ]
    .spacing(LABEL_SPACING)
    .boxed()
}

/// Lays fields out one to a line, labels lined up down the left.
///
/// These used to be a wrapping row of fixed-width columns, which in a pane
/// narrow enough for two of them left a ragged edge of dead space, and which
/// put each label above its own control — a line of vertical space per field,
/// and no column for the eye to run down. One field per line is what a settings
/// panel is for.
pub fn field_grid<'a, M: 'a>(fields: Vec<Element<'a, M>>) -> Element<'a, M> {
    Column::with_children(fields)
        .spacing(FIELD_SPACING)
        .width(Length::Fill)
        .boxed()
}

/// A titled group of controls, as a card.
///
/// The title is set small and upper-case rather than as a heading: it marks
/// where a group begins, and it should not compete with the widget's name in
/// the inspector's header. The card's own edge does the separating that a
/// hairline rule used to be asked to do on its own.
pub fn section<'a, M: 'a>(
    title: &'a str,
    body: impl iced::Widget<M> + 'a,
) -> Element<'a, M> {
    container(
        column![
            text(title.to_uppercase())
                .size(SECTION_SIZE - 3.0)
                .style(text::secondary),
            body.boxed(),
        ]
        .spacing(SECTION_SPACING),
    )
    .style(crate::styles::container::card)
    .padding(SECTION_PADDING)
    .width(Length::Fill)
    .boxed()
}

/// Nothing at all, for a section that does not apply to this widget.
pub fn nothing<'a, M: 'a>() -> Element<'a, M> {
    Space::new().boxed()
}

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
    content: impl iced::Widget<M> + 'a,
) -> iced::widget::Scrollable<'a, M>
where
    M: 'a,
{
    scrollable(container(content.boxed()).padding(padding::right(SCROLLBAR_GUTTER)).boxed())
}

/// A numeric entry sized for the property grid.
pub fn float_input<'a, M: Clone + 'a>(
    value: f32,
    on_change: impl Fn(f32) -> M + 'a,
) -> Element<'a, M> {
    text_input("0", format!("{value}"))
        .on_input(move |raw| on_change(raw.trim().parse::<f32>().unwrap_or(0.0)))
        .width(Length::Fill)
        .boxed()
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
            .width(Length::Fill),
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
            .width(Length::Fill),
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
                .width(Length::Fill),
            text(format!("{value:.0}"))
                .size(LABEL_SIZE)
                .width(READOUT_WIDTH),
        ]
        .spacing(LABEL_SPACING)
        .align_y(Alignment::Center),
    )
}

/// A checkbox in the control column, with its name in the label column.
///
/// The checkbox used to carry its own label, because the fields around it put
/// theirs above the control and a checkbox has nowhere to put one. With the
/// label in a column of its own there is somewhere, and a toggle that names
/// itself in a different place from everything else is the sort of thing that
/// makes a panel feel unplanned.
pub fn toggle_field<'a, M: Clone + 'a>(
    label: &'a str,
    checked: bool,
    on_toggle: impl Fn(bool) -> M + 'a,
) -> Element<'a, M> {
    field(label, checkbox(checked).on_toggle(on_toggle))
}

#[cfg(test)]
mod icon_window {
    use super::*;

    /// Everything fits, so there is nothing to slide.
    #[test]
    fn a_short_result_set_stays_at_the_top() {
        assert_eq!(
            slide_icon_window(0, ICON_WINDOW, 1.0),
            IconSlide {
                start: 0,
                snap: None
            }
        );
    }

    /// Scrolling around the middle is ordinary scrolling.
    #[test]
    fn the_window_only_moves_at_the_ends() {
        let total = ICON_WINDOW * 10;

        assert_eq!(
            slide_icon_window(ICON_WINDOW, total, 0.5),
            IconSlide {
                start: ICON_WINDOW,
                snap: None
            }
        );
    }

    /// A slide is compensated by exactly as much as it moved the icons, so it
    /// lands short of the edge that triggered it and cannot fire again at once.
    #[test]
    fn sliding_forward_nudges_the_view_back() {
        let total = ICON_WINDOW * 10;
        let slide = slide_icon_window(0, total, 0.9);

        assert_eq!(slide.start, ICON_STEP);

        let snap = slide.snap.expect("a slide snaps");
        let moved = ICON_STEP as f32 / ICON_WINDOW as f32;

        assert!((snap - (0.9 - moved)).abs() < f32::EPSILON);
        assert!(snap < ICON_LOAD_MORE_AT, "it would slide again immediately");
    }

    #[test]
    fn sliding_back_nudges_the_view_forward() {
        let total = ICON_WINDOW * 10;
        let slide = slide_icon_window(ICON_STEP, total, 0.1);

        assert_eq!(slide.start, 0);
        assert!(slide.snap.expect("a slide snaps") > ICON_LOAD_BACK_AT);
    }

    /// The window stops with its end against the end of the results.
    #[test]
    fn the_window_does_not_run_off_the_end() {
        let total = ICON_WINDOW * 2;
        let last = total - ICON_WINDOW;

        assert_eq!(slide_icon_window(last, total, 1.0).start, last);
        assert!(slide_icon_window(last - 1, total, 1.0).start <= last);
    }
}

//! The add-widget picker.
//!
//! This replaces the fixed palette that used to sit under the widget tree. That
//! palette drew a button for every [`WidgetType`] whether or not the selected
//! branch could take one, which cost the tree several hundred pixels of the
//! left column and grew every time a widget type was added.
//!
//! Here the same list is a popover hung off a `+` on the branch itself, so it
//! costs nothing until it is open, and it asks
//! [`get_available_types`](crate::views::add_widgets::get_available_types) — the
//! gate the palette already computed and then spent on greying buttons out — to
//! decide what to show. Nothing unavailable is drawn at all.

use iced::widget::{button, column, container, row, rule, scrollable, text, text_input};
use iced::{Alignment, Element, Length, padding};

use widgets::anchor::{Align, Side};
use widgets::popover::popover;

use crate::data_structures::types::type_implementations::WidgetCategory;
use crate::data_structures::types::types::{WidgetId, WidgetType};
use crate::icon;

/// The search box's identity, so opening the picker can focus it.
///
/// One id for every picker is enough: only one is ever open.
pub const SEARCH_INPUT_ID: iced::advanced::widget::Id =
    iced::advanced::widget::Id::new("widget-picker-search");

/// How wide the popover is, and how tall its list is allowed to grow.
const PICKER_WIDTH: f32 = 328.0;
const LIST_MAX_HEIGHT: f32 = 360.0;

/// Above this many hits the list is capped and scrolls.
const LIST_SCROLL_THRESHOLD: usize = 14;

/// Which branch's picker is open, and what has been typed into it.
#[derive(Debug, Clone, Default)]
pub struct PickerState {
    /// The branch whose `+` is showing its picker, if any.
    open_for: Option<WidgetId>,
    /// The current search text. Cleared whenever the picker opens.
    query: String,
}

impl PickerState {
    pub fn is_open_for(&self, id: WidgetId) -> bool {
        self.open_for == Some(id)
    }

    /// Opens the picker on a branch, clearing any previous search.
    pub fn open(&mut self, id: WidgetId) {
        if self.open_for == Some(id) {
            return;
        }

        self.open_for = Some(id);
        self.query.clear();
    }

    /// Closes the picker, but only if it is the one this branch opened.
    ///
    /// Pressing `+` on a second branch while the first is still open publishes
    /// both a close for the old one and an open for the new one in the same
    /// frame, in no guaranteed order. Naming the branch makes a late-arriving
    /// close harmless instead of leaving nothing open.
    pub fn close(&mut self, id: WidgetId) {
        if self.open_for != Some(id) {
            return;
        }

        self.dismiss();
    }

    /// Closes whatever is open, whoever asked.
    pub fn dismiss(&mut self) {
        self.open_for = None;
        self.query.clear();
    }

    pub fn set_query(&mut self, query: String) {
        self.query = query;
    }

    pub fn query(&self) -> &str {
        &self.query
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    /// The `+` was pressed. Deliberately ignored.
    ///
    /// A `button` with no `on_press` draws disabled, so the `+` needs one — but
    /// it must not be the thing that toggles the picker. An iced button fires
    /// `on_press` on *release*, while [`popover`] toggles on *press*: the press
    /// opened the picker, the view was rebuilt with the new state, and the
    /// release then published the freshly inverted message and closed it again.
    /// One source of truth, and it is [`Message::Toggled`].
    TriggerPressed,
    /// The popover opened or closed — pressing the `+`, pressing outside, or
    /// Escape.
    Toggled(WidgetId, bool),
    /// The search box changed.
    QueryChanged(String),
    /// A type was chosen. The child is added and the picker stays open.
    Picked(WidgetId, WidgetType),
    /// Return was pressed in the search box: take the first match.
    ///
    /// Carries nothing, because what the first match *is* depends on the
    /// hierarchy as it stands when the message is handled — and after adding a
    /// child that is not what it was when the box was drawn.
    SubmitFirstMatch(WidgetId),
}

/// What a branch can take right now, filtered by a search.
///
/// Public because Return in the search box is resolved when the message is
/// handled rather than when the box is drawn: after adding a child, what the
/// branch will accept has usually changed.
pub fn first_matches(available: &[WidgetType], query: &str) -> Vec<WidgetType> {
    available
        .iter()
        .copied()
        .filter(|ty| ty.matches_search(query))
        .collect()
}

/// The `+` button for a branch, with its picker hanging off it.
///
/// Returns `None` when the branch cannot take another child, so the caller can
/// leave the slot empty rather than draw a disabled control — the tree row has
/// no space to spend on a button that never does anything.
pub fn trigger<'a>(
    parent_id: WidgetId,
    available: &[WidgetType],
    state: &'a PickerState,
) -> Option<Element<'a, Message>> {
    if available.is_empty() {
        return None;
    }

    let is_open = state.is_open_for(parent_id);

    let trigger = button(icon::plus())
        .style(button::text)
        .on_press(Message::TriggerPressed);

    Some(
        popover(trigger, picker_content(available, state, parent_id))
            .open(is_open)
            .side(Side::Bottom)
            .align(Align::End)
            .min_width(PICKER_WIDTH)
            .max_width(PICKER_WIDTH)
            .dismiss_on_outside_press(true)
            .on_toggle(move |open| Message::Toggled(parent_id, open))
            .into(),
    )
}

/// The search box and the filtered list beneath it.
fn picker_content<'a>(
    available: &[WidgetType],
    state: &'a PickerState,
    parent_id: WidgetId,
) -> Element<'a, Message> {
    let search = row![
        icon::search().size(14),
        text_input("Search widgets", state.query())
            .id(SEARCH_INPUT_ID)
            .on_input(Message::QueryChanged)
            .on_submit(Message::SubmitFirstMatch(parent_id))
            .size(14)
            .width(Length::Fill),
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    let matches = first_matches(available, state.query());

    let body: Element<'a, Message> = if matches.is_empty() {
        container(
            text(format!("Nothing matches \u{201c}{}\u{201d}", state.query().trim()))
                .size(13)
                .style(text::secondary),
        )
        .padding(padding::top(14).bottom(6))
        .into()
    } else {
        // The first entry is what Return takes, so the order the sections are
        // built in is the order `first_matches` returns.
        let mut sections = column![].spacing(10);

        for category in WidgetCategory::ALL {
            let in_category: Vec<WidgetType> = matches
                .iter()
                .copied()
                .filter(|ty| ty.category() == category)
                .collect();

            if in_category.is_empty() {
                continue;
            }

            sections = sections.push(section(category, &in_category, parent_id));
        }

        // Short lists size to their content; only a long one is capped and
        // given a scrollbar, so the popover is never mostly empty.
        let height = if matches.len() > LIST_SCROLL_THRESHOLD {
            Length::Fixed(LIST_MAX_HEIGHT)
        } else {
            Length::Shrink
        };

        scrollable(sections).height(height).into()
    };

    container(
        column![search, rule::horizontal(1), body]
            .spacing(8)
            .width(Length::Fill),
    )
    .into()
}

/// One labelled group of types, two to a row.
fn section<'a>(
    category: WidgetCategory,
    types: &[WidgetType],
    parent_id: WidgetId,
) -> Element<'a, Message> {
    let mut rows = column![
        text(category.label())
            .size(11)
            .style(text::secondary),
    ]
    .spacing(4);

    for pair in types.chunks(2) {
        let mut line = row![].spacing(6);

        for ty in pair {
            line = line.push(
                button(
                    row![
                        icon::widget_type(*ty).size(13),
                        text(ty.label()).size(13),
                    ]
                    .spacing(6)
                    .align_y(Alignment::Center),
                )
                .style(button::secondary)
                .width(Length::FillPortion(1))
                .on_press(Message::Picked(parent_id, *ty)),
            );
        }

        // A lone button on the last line should not stretch to the full width
        // and break the grid.
        if pair.len() == 1 {
            line = line.push(iced::widget::space::horizontal().width(Length::FillPortion(1)));
        }

        rows = rows.push(line);
    }

    rows.into()
}

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

use iced::Widget as _;
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

/// The results list, so the arrow keys can scroll the highlight into view.
pub const LIST_ID: iced::advanced::widget::Id =
    iced::advanced::widget::Id::new("widget-picker-list");

/// How wide the popover is, and how tall its list is allowed to grow.
const PICKER_WIDTH: f32 = 328.0;
const LIST_MAX_HEIGHT: f32 = 360.0;

/// How wide the search box is.
///
/// Explicit rather than `Fill`, and that matters: a popover measures its
/// content at `Shrink` to find its intrinsic width before laying it out again
/// at the settled one. A `Fill` text input measures as nothing in that first
/// pass and caches a scroll offset against it, which left the caret parked off
/// the left edge — you could type and see nothing until an arrow key scrolled
/// the view back.
const SEARCH_WIDTH: f32 = PICKER_WIDTH - POPOVER_PADDING * 2.0 - SEARCH_ICON_WIDTH;

/// The popover's own padding, and the room the search icon and its gap take.
const POPOVER_PADDING: f32 = 12.0;
const SEARCH_ICON_WIDTH: f32 = 22.0;

/// Above this many hits the list is capped and scrolls.
const LIST_SCROLL_THRESHOLD: usize = 10;

/// Which branch's picker is open, and what has been typed into it.
#[derive(Debug, Clone, Default)]
pub struct PickerState {
    /// The branch whose `+` is showing its picker, if any.
    open_for: Option<WidgetId>,
    /// The current search text. Cleared whenever the picker opens.
    query: String,
    /// Which result the arrow keys are on.
    ///
    /// A second focus: the search box keeps the real one so typing never
    /// stops, and this rides alongside it so Return knows what to add.
    highlight: usize,
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
        // A different search is a different list; keeping the old index would
        // point the highlight at whatever happened to land in that slot.
        self.highlight = 0;
    }

    pub fn query(&self) -> &str {
        &self.query
    }

    pub fn open_for(&self) -> Option<WidgetId> {
        self.open_for
    }

    pub fn highlight(&self) -> usize {
        self.highlight
    }

    /// Moves the highlight, wrapping at both ends.
    ///
    /// Wrapping rather than clamping because the list is short and searched:
    /// one press past the last hit is more likely to mean "back to the top"
    /// than "stay here".
    pub fn move_highlight(&mut self, delta: i32, len: usize) {
        if len == 0 {
            self.highlight = 0;
            return;
        }

        let len = len as i32;
        let next = (self.highlight as i32 + delta).rem_euclid(len);

        self.highlight = next as usize;
    }

    /// Where the list has to be scrolled to for the highlight to be on screen.
    ///
    /// A ratio of the highlight's position through the list rather than a
    /// measured offset: the entries are a uniform height, so walking from the
    /// top to the bottom walks the scroll from one end to the other. The
    /// category headings make it approximate, by about one heading's height at
    /// worst, which is well inside the viewport.
    pub fn scroll_offset(&self, len: usize) -> f32 {
        if len <= 1 {
            return 0.0;
        }

        self.highlight as f32 / (len - 1) as f32
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
    /// Return was pressed in the search box: take the highlighted result.
    ///
    /// The result is resolved when the message is handled rather than when the
    /// box was drawn — after adding a child, what the branch will accept has
    /// usually changed.
    SubmitHighlighted(WidgetId),
}

/// What a branch can take right now, filtered by a search, in the order the
/// picker draws them.
///
/// The order matters twice over: the list is grouped by category, and the
/// highlight is an index into it. If the keyboard counted one order and the
/// list drew another, Return would add whatever happened to sit in that slot.
/// So this is the only definition of the order, and both use it.
///
/// Public because Return is resolved when the message is handled rather than
/// when the box was drawn: after adding a child, what the branch will accept
/// has usually changed.
pub fn first_matches(available: &[WidgetType], query: &str) -> Vec<WidgetType> {
    let mut matches: Vec<WidgetType> = available
        .iter()
        .copied()
        .filter(|ty| ty.matches_search(query))
        .collect();

    // Stable, so within a category the parent's own order survives.
    matches.sort_by_key(|ty| ty.category());

    matches
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
            .boxed(),
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
            .on_submit(Message::SubmitHighlighted(parent_id))
            .size(14)
            .width(SEARCH_WIDTH),
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
        .boxed()
    } else {
        // One entry per line, because the arrow keys walk this list and a grid
        // would make "down" mean two different things depending on where you
        // were. Walked in `first_matches` order — the same order the highlight
        // indexes into — with a heading emitted wherever the category turns
        // over, rather than regrouping and risking a different one.
        let mut list = column![].spacing(2);
        let mut current: Option<WidgetCategory> = None;

        for (index, &ty) in matches.iter().enumerate() {
            if current != Some(ty.category()) {
                current = Some(ty.category());

                list = list.push(
                    container(text(ty.category().label()).size(10).style(text::secondary))
                        .padding(padding::top(6).bottom(2))
                        .boxed(),
                );
            }

            list = list.push(entry(ty, parent_id, index == state.highlight()));
        }

        // Short lists size to their content; only a long one is capped and
        // given a scrollbar, so the popover is never mostly empty.
        let height = if matches.len() > LIST_SCROLL_THRESHOLD {
            Length::Fixed(LIST_MAX_HEIGHT)
        } else {
            Length::Shrink
        };

        scrollable(list).id(LIST_ID).height(height).boxed()
    };

    container(
        column![search, rule::horizontal(1), body]
            .spacing(8)
            .width(Length::Fill),
    )
    .boxed()
}

/// One widget type in the list.
///
/// The highlighted entry is drawn as the primary action rather than merely
/// tinted, because it is what Return will take.
fn entry<'a>(
    widget_type: WidgetType,
    parent_id: WidgetId,
    highlighted: bool,
) -> Element<'a, Message> {
    let style = if highlighted {
        button::primary
    } else {
        button::text
    };

    button(
        row![
            icon::widget_type(widget_type).size(13),
            text(widget_type.label()).size(13),
        ]
        .spacing(8)
        .align_y(Alignment::Center),
    )
    .style(style)
    .padding(padding::left(6).right(6).top(3).bottom(3))
    .width(Length::Fill)
    .on_press(Message::Picked(parent_id, widget_type))
    .boxed()
}

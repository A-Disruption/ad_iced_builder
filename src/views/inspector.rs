//! Properties of whatever is selected.
//!
//! Before this pane existed, editing a widget meant pressing a pencil on its
//! tree row, which opened a modal over the whole editor: you could not see the
//! tree, the preview or the code while changing the thing they were all showing.
//! Worse, the modal was built per branch, so a tree of fifty widgets constructed
//! fifty property panels — and fifty `text_editor`s — on every frame, guarded
//! only by an early return.
//!
//! Here the panel is built once, for the selection, in a pane that is always on
//! screen. Selecting several widgets shows the properties they have in common
//! rather than nothing at all.

use std::collections::BTreeMap;

use iced::widget::{button, column, container, row, rule, space, text};
use iced::{Alignment, Element, Length, Theme, padding};
use iced_split::horizontal_split;
use uuid::Uuid;

use crate::controls::batch_edit::batch_editor_controls;
use crate::data_structures::types::types::{AppView, WidgetType};
use crate::data_structures::widget_hierarchy::WidgetHierarchy;
use crate::enum_builder::TypeSystem;
use crate::icon;
use crate::controls::scroll_y;
use crate::views::theme_and_stylefn_builder::CustomThemes;
use crate::views::{add_widgets, widget_tree};

/// What the inspector reports.
///
/// Two vocabularies, because editing one widget and editing several are
/// genuinely different operations: one names a `WidgetId`, the other applies to
/// whatever is selected. Both already existed — this only gives them one place
/// to be emitted from.
#[derive(Debug, Clone)]
pub enum Message {
    /// A property of the one selected widget changed.
    Single(widget_tree::Message),
    /// Something that applies to the whole selection — a batch property, or
    /// wrapping it all in a new parent.
    Batch(add_widgets::Message),
    /// One of the panel's dividers was dragged.
    SplitMoved(Divider, f32),
}

/// The two dividers down the inspector.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Divider {
    /// Between the properties and everything under them.
    Body,
    /// Between the generated code and the wrapping controls.
    Code,
}

/// Where the inspector's dividers sit, as fractions of the space they split.
///
/// The properties used to be boxed at a fixed 400px with the code preview
/// nailed under them, so a widget with many settings scrolled in a short
/// window no matter how tall the pane was.
#[derive(Debug, Clone, Copy)]
pub struct Splits {
    pub body: f32,
    pub code: f32,
}

impl Default for Splits {
    fn default() -> Self {
        Self {
            // Properties get most of the panel; the code is a glance, and the
            // wrap buttons are two rows.
            body: 0.62,
            code: 0.74,
        }
    }
}

impl Splits {
    pub fn set(&mut self, divider: Divider, ratio: f32) {
        let ratio = ratio.clamp(0.05, 0.95);

        match divider {
            Divider::Body => self.body = ratio,
            Divider::Code => self.code = ratio,
        }
    }
}

/// The inspector's body for the current selection.
pub fn view<'a>(
    hierarchy: &'a WidgetHierarchy,
    type_system: &'a TypeSystem,
    theme: &'a Theme,
    views: &'a BTreeMap<Uuid, AppView>,
    custom_styles: &'a CustomThemes,
    preview_content: &'a iced::widget::text_editor::Content,
    view_id: Uuid,
    splits: Splits,
) -> Element<'a, Message> {
    let selected = hierarchy.selected_ids();

    match selected.len() {
        0 => empty(
            "Nothing selected",
            "Pick a widget in the tree to edit it. Select several to edit them together.",
        ),

        1 => {
            let Some(widget_id) = selected.iter().next().copied() else {
                return empty("Nothing selected", "Pick a widget in the tree to edit it.");
            };

            let Some(widget) = hierarchy.get_widget_by_id(widget_id) else {
                // The selection outlived the widget — a delete that has not yet
                // cleared the set. Say so rather than drawing a stale panel.
                return empty("Selection is gone", "That widget no longer exists.");
            };

            // The name is the title: an unnamed widget shows its type as
            // placeholder, and naming it replaces that in place.
            let header = row![
                icon::widget_type(widget.widget_type).size(15),
                crate::controls::widget_name(
                    widget_id,
                    &widget.properties.widget_name,
                    widget.widget_type.label(),
                )
                .map(Message::Single),
                text(widget.widget_type.label())
                    .size(12)
                    .style(text::secondary),
            ]
            .align_y(Alignment::Center)
            .spacing(8);

            let properties = widget_tree::controls_for_widget(
                hierarchy,
                type_system,
                theme,
                widget,
                widget_id,
                views,
                custom_styles,
                preview_content,
                view_id,
            )
            .map(Message::Single);

            // Wrapping used to be offered only alongside a multiple selection,
            // where its `Container` button was gated on exactly one widget being
            // selected — so that button could never fire. `validate_wrapping`
            // has always accepted a single widget.
            let under = horizontal_split(
                titled("Widget Code", crate::controls::widget_code_view(preview_content)
                    .map(Message::Single)),
                titled("Wrap in", wrap_controls(hierarchy)),
                splits.code,
                |ratio| Message::SplitMoved(Divider::Code, ratio),
            );

            let body = horizontal_split(
                scroll_y(properties).height(Length::Fill),
                under,
                splits.body,
                |ratio| Message::SplitMoved(Divider::Body, ratio),
            );

            panel(header.into(), body.into())
        }

        count => {
            let header = row![
                text(format!("{count} widgets selected")).size(16),
                space::horizontal(),
            ]
            .align_y(Alignment::Center)
            .spacing(8);

            // `common_properties` is recomputed on every selection change, so
            // it is there whenever anything is selected. Checked rather than
            // unwrapped, because a panic in `view` takes the window with it.
            let body: Element<'a, Message> = if hierarchy.common_properties.is_some() {
                column![
                    wrap_controls(hierarchy),
                    rule::horizontal(1),
                    batch_editor_controls(hierarchy, custom_styles).map(Message::Batch),
                ]
                .spacing(10)
                .into()
            } else {
                text("No properties in common.")
                    .size(13)
                    .style(text::secondary)
                    .into()
            };

            panel(header.into(), body)
        }
    }
}

/// Wrapping the selection in a new parent.
///
/// This lived under the tree, in the panel the widget palette used to share.
/// It belongs with the rest of what can be done to a selection.
fn wrap_controls<'a>(hierarchy: &'a WidgetHierarchy) -> Element<'a, Message> {
    let can_wrap = hierarchy.validate_wrapping().is_ok();

    let wrap_button = move |widget_type: WidgetType| {
        button(
            row![
                icon::widget_type(widget_type).size(13),
                text(widget_type.label()).size(13),
            ]
            .spacing(6)
            .align_y(Alignment::Center),
        )
        .style(button::secondary)
        .width(Length::FillPortion(1))
        .on_press_maybe(can_wrap.then(|| {
            Message::Batch(add_widgets::Message::WrapSelectedInContainer(widget_type))
        }))
    };

    column![
        text("Wrap in").size(11).style(text::secondary),
        row![
            wrap_button(WidgetType::Row),
            wrap_button(WidgetType::Column)
        ]
        .spacing(6),
        row![
            wrap_button(WidgetType::Container),
            wrap_button(WidgetType::MouseArea)
        ]
        .spacing(6),
    ]
    .spacing(6)
    .into()
}

/// A small caption over a region of the panel.
fn titled<'a>(label: &'a str, body: Element<'a, Message>) -> Element<'a, Message> {
    column![
        text(label).size(11).style(text::secondary),
        body,
    ]
    .spacing(4)
    .height(Length::Fill)
    .into()
}

/// A header, a rule, and whatever the selection calls for.
///
/// The panel pads its own edges and the scrollable inside it runs to that same
/// edge, so the scrollbar lands under the rule's right end rather than a
/// gutter's width inside it.
fn panel<'a>(header: Element<'a, Message>, body: Element<'a, Message>) -> Element<'a, Message> {
    container(
        column![header, rule::horizontal(1), body]
            .spacing(8)
            .width(Length::Fill)
            .height(Length::Fill),
    )
    .padding(padding::all(10))
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

/// The pane with nothing to show, said plainly.
fn empty<'a>(title: &'a str, detail: &'a str) -> Element<'a, Message> {
    container(
        column![
            text(title).size(15),
            text(detail).size(13).style(text::secondary),
        ]
        .spacing(6)
        .align_x(Alignment::Center),
    )
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .padding(20)
    .into()
}

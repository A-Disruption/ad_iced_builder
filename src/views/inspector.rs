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
use uuid::Uuid;

use crate::controls::batch_edit::batch_editor_controls;
use crate::data_structures::types::types::{AppView, WidgetType};
use crate::data_structures::widget_hierarchy::WidgetHierarchy;
use crate::enum_builder::TypeSystem;
use crate::icon;
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

            let display_name = if widget.properties.widget_name.is_empty() {
                widget.name.as_str()
            } else {
                widget.properties.widget_name.as_str()
            };

            let header = row![
                icon::widget_type(widget.widget_type).size(15),
                text(display_name).size(16),
                space::horizontal(),
                text(widget.widget_type.label())
                    .size(12)
                    .style(text::secondary),
            ]
            .align_y(Alignment::Center)
            .spacing(8);

            let controls = widget_tree::controls_for_widget(
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
            panel(
                header.into(),
                column![controls, rule::horizontal(1), wrap_controls(hierarchy)]
                    .spacing(10)
                    .into(),
            )
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

/// A header, a rule, and whatever the selection calls for.
fn panel<'a>(header: Element<'a, Message>, body: Element<'a, Message>) -> Element<'a, Message> {
    container(
        column![header, rule::horizontal(1), body]
            .spacing(8)
            .width(Length::Fill),
    )
    .padding(padding::all(10))
    .width(Length::Fill)
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

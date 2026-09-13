use iced::clipboard;
use iced::widget::{button, column, container, mouse_area, row, space, text, text_editor};
use iced::{Alignment, Element, Length, Task, Theme, padding};
use std::collections::{BTreeMap, HashSet};
use uuid::Uuid;
use widgets::tree::{Branch, DropInfo, DropPosition, branch, tree_handle};

use crate::controls::*;
use crate::data_structures::properties::messages::PropertyChange;
use crate::data_structures::types::type_implementations::Orientation;
use crate::data_structures::types::types::{AppView, Widget, WidgetId, WidgetType};
use crate::data_structures::widget_hierarchy::WidgetHierarchy;
use crate::enum_builder::TypeSystem;
use crate::icon;
use crate::styles;
use crate::views::enum_editor::EnumEditorView;
use crate::views::theme_and_stylefn_builder::CustomThemes;
use crate::views::add_widgets::get_available_types;
use crate::views::widget_picker::{self, PickerState};

#[derive(Debug, Clone)]
pub enum Message {
    // Tree Hierarchy
    TreeMove(DropInfo),

    // Widget Operations
    SelectWidgets(HashSet<usize>),
    DeleteWidget(WidgetId),
    PropertyChanged(WidgetId, PropertyChange),
    SwapKind(WidgetId),

    /// The cursor entered a row.
    ///
    /// Held by the application so the row under the cursor can show its
    /// actions and every other row can stay quiet.
    RowEntered(WidgetId),
    /// The cursor left a row.
    RowLeft(WidgetId),

    /// The keyboard moved to a different row.
    ///
    /// The tree owns focus — it knows the row order and which rows are on
    /// screen — so the application is told rather than asked.
    RowFocused(Option<WidgetId>),

    /// The add-widget picker hanging off a branch's `+`.
    ///
    /// Handled in `main.rs`: the picker's open state lives on the application,
    /// not on the hierarchy, and adding a child has to trigger regeneration.
    Picker(widget_picker::Message),

    OpenEnumEditor,

    CodeTabSelected(String),
    CopyCode(String),

    // text_editor actions for code views
    CodeViewEdit(text_editor::Action),
    WidgetPreviewEdit(text_editor::Action),
}

pub fn update(
    message: Message,
    hierarchy: &mut WidgetHierarchy,
    type_system: &mut TypeSystem,
    _enum_editor: &mut EnumEditorView,
) -> Task<Message> {
    match message {
        Message::TreeMove(drop_info) => {
            if let Some(target_external_id) = drop_info.target_id {
                let target_id = WidgetId(target_external_id);

                // Convert all dragged external IDs to WidgetIds
                let dragged_ids: Vec<WidgetId> = drop_info
                    .dragged_ids
                    .iter()
                    .map(|&id| WidgetId(id))
                    .collect();

                // Process moves one at a time to handle index adjustments properly
                match drop_info.position {
                    DropPosition::Into => {
                        // Moving into a target - append each item
                        for dragged_id in dragged_ids {
                            if dragged_id != target_id {
                                // When moving into, get the target's current child count for proper indexing
                                let target_child_count = hierarchy
                                    .get_widget_by_id(target_id)
                                    .map(|w| w.children.len())
                                    .unwrap_or(0);
                                let _ = hierarchy.move_widget(
                                    dragged_id,
                                    target_id,
                                    target_child_count,
                                );
                            }
                        }
                    }
                    DropPosition::Before | DropPosition::After => {
                        if let Some(parent_id) = hierarchy.find_parent_id(target_id) {
                            for dragged_id in dragged_ids {
                                if dragged_id == target_id {
                                    continue;
                                }

                                // Get fresh target position each time since moves can shift indices
                                let target_index = hierarchy
                                    .get_widget_by_id(parent_id)
                                    .and_then(|parent| {
                                        parent.children.iter().position(|c| c.id == target_id)
                                    })
                                    .unwrap_or(0);

                                let insert_index = match drop_info.position {
                                    DropPosition::Before => target_index,
                                    DropPosition::After => target_index + 1,
                                    _ => target_index,
                                };

                                let _ = hierarchy.move_widget(dragged_id, parent_id, insert_index);
                            }
                        }
                    }
                }
            }
        }

        Message::SelectWidgets(external_ids) => {
            let widget_ids: HashSet<WidgetId> =
                external_ids.iter().map(|&id| WidgetId(id)).collect();
            hierarchy.set_selected_ids(widget_ids);
        }

        Message::DeleteWidget(id) => {
            let _ = hierarchy.delete_widget(id);
        }

        Message::PropertyChanged(id, change) => {
            hierarchy.apply_property_change(id, change.clone(), type_system);

            match hierarchy.get_widget_by_id(id) {
                Some(widget) => {
                    if widget.widget_type == WidgetType::Space {
                        match change {
                            PropertyChange::Orientation(Orientation::Horizontal) => {
                                hierarchy.apply_property_change(
                                    id,
                                    PropertyChange::Width(Length::Fill),
                                    type_system,
                                );
                                hierarchy.apply_property_change(
                                    id,
                                    PropertyChange::Height(Length::Shrink),
                                    type_system,
                                );
                            }
                            PropertyChange::Orientation(Orientation::Vertical) => {
                                hierarchy.apply_property_change(
                                    id,
                                    PropertyChange::Width(Length::Shrink),
                                    type_system,
                                );
                                hierarchy.apply_property_change(
                                    id,
                                    PropertyChange::Height(Length::Fill),
                                    type_system,
                                );
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        Message::SwapKind(id) => {
            hierarchy.swap_kind(id);
        }
        Message::OpenEnumEditor => {
            // open the enum editor view
        }
        Message::Picker(_) => {}     // Handled in main.rs
        Message::RowEntered(_) => {} // Handled in main.rs
        Message::RowLeft(_) => {}    // Handled in main.rs
        Message::RowFocused(_) => {} // Handled in main.rs
        Message::CopyCode(code) => return clipboard::write(code).discard(),
        Message::CodeTabSelected(_) => {}   // Handled in main.rs
        Message::CodeViewEdit(_) => {}      // Handled in main.rs
        Message::WidgetPreviewEdit(_) => {} // Handled in main.rs
    }

    Task::none()
}

/// One view's widget tree.
///
/// Takes only the hierarchy and the picker now. It used to take the type
/// system, the themes, every other view and the preview's editor content as
/// well — not because a tree row needs any of them, but because each row built
/// a whole property modal that did. Those properties live in
/// [`crate::views::inspector`], which is handed the selection once.
pub fn view<'a>(
    hierarchy: &'a WidgetHierarchy,
    picker: &'a PickerState,
    hovered: Option<WidgetId>,
) -> Element<'a, Message> {
    let widget = hierarchy.root();

    let mut children = Vec::new();

    for child in &widget.children {
        children.push(build_tree_item(hierarchy, child, picker, hovered));
    }

    let root = branch(row_content(hierarchy, widget, picker, hovered))
        .block_dragging()
        .with_children(children)
        .with_id(widget.id.0);

    let mut tree = tree_handle(vec![root])
        .expand_icon(icon::chevron_right())
        .collapse_icon(icon::chevron_down())
        // The tree's own floor used to be 32, which beat the row's height and
        // left every row taller than it asked to be.
        .line_height(ROW_HEIGHT)
        .indent(16.0)
        .quick()
        .on_drop(Message::TreeMove)
        // Ask the hierarchy the same question the palette asks, so a drop the
        // move would reject is drawn as refused rather than silently doing
        // nothing when the mouse comes up.
        .on_can_drop(move |dragged, target, position| {
            let Some(target) = target else {
                return false;
            };

            let target = WidgetId(target);

            // Dropping *onto* a branch makes it the parent; dropping before or
            // after makes it a sibling, so the parent is the branch's own.
            let parent = match position {
                DropPosition::Into => Some(target),
                DropPosition::Before | DropPosition::After => hierarchy.find_parent_id(target),
            };

            let Some(parent) = parent else {
                return false;
            };

            dragged.iter().all(|&dragged| {
                let dragged = WidgetId(dragged);

                hierarchy
                    .get_widget_by_id(dragged)
                    .is_some_and(|widget| {
                        let reordering = hierarchy.find_parent_id(dragged) == Some(parent);

                        hierarchy
                            .check_accepts_child(parent, widget.widget_type, reordering)
                            .is_ok()
                    })
            })
        })
        .on_select(|selected_ids| Message::SelectWidgets(selected_ids))
        .on_focus(|id| Message::RowFocused(id.map(WidgetId)));

    tree = tree.reset_order_state();

    tree.into()
}

/// How wide one action button is, including the gap after it.
///
/// The actions only appear on the row under the cursor, and a row that changes
/// width when the cursor arrives is a row that shoves its neighbours' names
/// around. So the space is always there; only the buttons come and go.
const ACTION_SLOT_WIDTH: f32 = 26.0;

/// How tall every row is, whatever it happens to contain.
///
/// Pinned rather than left to the content, because the content changes: the
/// actions appear under the cursor and a bare `text` is shorter than a
/// `button`. Without this the row grew as the cursor arrived and everything
/// below it stepped down the screen.
const ROW_HEIGHT: f32 = 28.0;

/// One tree row: what the widget is, what it is called, and what you can do to
/// it.
///
/// The actions are drawn only for the row under the cursor. Three buttons on
/// every row of a deep tree is most of what you see, and none of them is what
/// you came to the tree to read — and drawing them for the selection too meant
/// the selected row never went quiet.
fn row_content<'a>(
    hierarchy: &'a WidgetHierarchy,
    widget: &'a Widget,
    picker: &'a PickerState,
    hovered: Option<WidgetId>,
) -> Element<'a, Message> {
    let is_named = !widget.properties.widget_name.is_empty();

    let display_name = if is_named {
        widget.properties.widget_name.as_str()
    } else {
        widget.name.as_str()
    };

    // A named widget's name hides its type, so say the type beside it. An
    // unnamed one is already called `Button`, and repeating that is noise.
    let type_label: Option<Element<'a, Message>> = is_named.then(|| {
        text(widget.widget_type.label())
            .size(11)
            .style(text::secondary)
            .into()
    });

    let available = get_available_types(hierarchy, &widget.id);

    let can_swap = matches!(
        widget.widget_type,
        WidgetType::Row | WidgetType::Column | WidgetType::Container | WidgetType::Scrollable
    );
    // Root is the one widget that cannot be deleted.
    let can_delete = widget.id.0 != 0;

    let slots = usize::from(!available.is_empty()) + usize::from(can_swap) + usize::from(can_delete);

    // The picker's own popover has to stay in the tree while it is open, or
    // the surface it is anchored to disappears from under it — so a row whose
    // picker is open keeps its actions even once the cursor has moved on.
    let show_actions = hovered == Some(widget.id) || picker.is_open_for(widget.id);

    let actions: Element<'a, Message> = if show_actions {
        // Padding kept tight so a button is no taller than the text beside it;
        // `ROW_HEIGHT` pins the row either way, but a button that overflows it
        // would be clipped rather than centred.
        let add = widget_picker::trigger(widget.id, &available, picker)
            .map(|element| element.map(Message::Picker));

        let swap = can_swap.then(|| {
            button(icon::arrow_right_left().size(13))
                .padding(3)
                .on_press(Message::SwapKind(widget.id))
                .style(button::text)
        });

        let delete = can_delete.then(|| {
            button(icon::trash_2().size(13))
                .padding(3)
                .on_press(Message::DeleteWidget(widget.id))
                .style(styles::button::cancel)
        });

        row![add, swap, delete]
            .spacing(2)
            .align_y(Alignment::Center)
            .into()
    } else {
        space::horizontal()
            .width(Length::Fixed(slots as f32 * ACTION_SLOT_WIDTH))
            .into()
    };

    mouse_area(
        container(
            row![
                icon::widget_type(widget.widget_type)
                    .size(14)
                    .style(text::secondary),
                text(display_name).size(13),
                type_label,
                space::horizontal(),
                actions,
            ]
            .spacing(6)
            .align_y(Alignment::Center)
            .padding(padding::left(2).right(4)),
        )
        .height(Length::Fixed(ROW_HEIGHT))
        .align_y(Alignment::Center),
    )
    .on_enter(Message::RowEntered(widget.id))
    .on_exit(Message::RowLeft(widget.id))
    .into()
}
fn build_tree_item<'a>(
    hierarchy: &'a WidgetHierarchy,
    widget: &'a Widget,
    picker: &'a PickerState,
    hovered: Option<WidgetId>,
) -> Branch<'a, Message, Theme, iced::Renderer> {
    // The view's outermost widget stays put: it is what the view *is*, and
    // dragging it out would leave the view with no root.
    let is_first_child_of_root = hierarchy
        .root()
        .children
        .first()
        .map(|c| c.id == widget.id)
        .unwrap_or(false);

    let mut children = Vec::new();

    for child in &widget.children {
        children.push(build_tree_item(hierarchy, child, picker, hovered));
    }

    // Whether a branch takes drops is a question about the widget type, not
    // about whether it happens to be full right now — a full branch still
    // accepts a drop that reorders its own children.
    let accepts_drops = matches!(
        widget.widget_type,
        WidgetType::Row
            | WidgetType::Column
            | WidgetType::Container
            | WidgetType::Scrollable
            | WidgetType::Split
            | WidgetType::Responsive
            | WidgetType::Tooltip
            | WidgetType::Float
            | WidgetType::Sensor
            | WidgetType::MouseArea
            | WidgetType::Pin
            | WidgetType::Button
            | WidgetType::Stack
            | WidgetType::Collapsible
            | WidgetType::CollapsibleGroup
            | WidgetType::Popover
            | WidgetType::Menu
            | WidgetType::Layer
            | WidgetType::Toast
            | WidgetType::Themer
            | WidgetType::Grid
    );

    let mut branch = branch(row_content(hierarchy, widget, picker, hovered))
        .with_id(widget.id.0)
        .with_children(children);

    if accepts_drops {
        branch = branch.accepts_drops();
    }

    if is_first_child_of_root {
        branch = branch.block_dragging();
    }

    branch
}

/// Every control for one widget, as one panel.
///
/// This used to be private, and guarded: it built nothing unless the widget's
/// modal was the open one, because a tree of fifty branches would otherwise
/// construct fifty `text_editor`s and run tree-sitter over all of them on every
/// frame. The guard is gone with the modals — [`crate::views::inspector`] calls
/// this exactly once, for the selection.
pub fn controls_for_widget<'a>(
    hierarchy: &'a WidgetHierarchy,
    type_system: &'a TypeSystem,
    theme: &'a Theme,
    widget: &'a Widget,
    widget_id: WidgetId,
    views: &'a BTreeMap<Uuid, AppView>,
    custom_styles: &'a CustomThemes,
    preview_content: &'a text_editor::Content,
    view_id: Uuid,
) -> Element<'a, Message> {
    let controls_view: Element<Message> = match widget.widget_type {
        WidgetType::Container => container_controls(
            &hierarchy,
            widget_id,
            theme,
            &type_system,
            custom_styles,
            preview_content,
            view_id,
            views,
        ),
        WidgetType::Scrollable => scrollable_controls(
            &hierarchy,
            widget_id,
            theme,
            &type_system,
            custom_styles,
            preview_content,
        ),
        WidgetType::Row => row_controls(
            &hierarchy,
            widget_id,
            theme,
            &type_system,
            custom_styles,
            preview_content,
        ),
        WidgetType::Column => column_controls(
            &hierarchy,
            widget_id,
            theme,
            &type_system,
            custom_styles,
            preview_content,
        ),
        WidgetType::Button => button_controls(
            &hierarchy,
            widget_id,
            theme,
            &type_system,
            custom_styles,
            preview_content,
            views,
            view_id,
        ),
        WidgetType::Text => text_controls(
            &hierarchy,
            widget_id,
            theme,
            &type_system,
            custom_styles,
            preview_content,
        ),
        WidgetType::TextEditor => text_editor_controls(
            &hierarchy,
            widget_id,
            theme,
            &type_system,
            custom_styles,
            preview_content,
            views,
            view_id,
        ),
        WidgetType::TextInput => text_input_controls(
            &hierarchy,
            widget_id,
            theme,
            &type_system,
            custom_styles,
            preview_content,
        ),
        WidgetType::Checkbox => checkbox_controls(
            &hierarchy,
            widget_id,
            theme,
            &type_system,
            custom_styles,
            preview_content,
            views,
            view_id,
        ),
        WidgetType::Radio => radio_controls(
            &hierarchy,
            widget_id,
            theme,
            &type_system,
            custom_styles,
            preview_content,
            views,
            view_id,
        ),
        WidgetType::Toggler => toggler_controls(
            &hierarchy,
            widget_id,
            theme,
            &type_system,
            custom_styles,
            preview_content,
            views,
            view_id,
        ),
        WidgetType::PickList => picklist_controls(
            &hierarchy,
            widget_id,
            theme,
            &type_system,
            custom_styles,
            preview_content,
            views,
            view_id,
        ),
        WidgetType::DatePicker => date_picker_controls(
            &hierarchy,
            widget_id,
            theme,
            &type_system,
            custom_styles,
            preview_content,
            views,
            view_id,
        ),
        WidgetType::Split => split_controls(
            &hierarchy,
            widget_id,
            theme,
            &type_system,
            custom_styles,
            preview_content,
        ),
        WidgetType::Float => float_controls(
            &hierarchy,
            widget_id,
            theme,
            &type_system,
            custom_styles,
            preview_content,
        ),
        WidgetType::Responsive => responsive_controls(
            &hierarchy,
            widget_id,
            theme,
            &type_system,
            custom_styles,
            preview_content,
        ),
        WidgetType::Sensor => sensor_controls(
            &hierarchy,
            widget_id,
            theme,
            &type_system,
            custom_styles,
            preview_content,
        ),
        WidgetType::Slider => slider_controls(
            &hierarchy,
            widget_id,
            theme,
            &type_system,
            custom_styles,
            preview_content,
            views,
            view_id,
        ),
        WidgetType::VerticalSlider => vertical_slider_controls(
            &hierarchy,
            widget_id,
            theme,
            &type_system,
            custom_styles,
            preview_content,
            views,
            view_id,
        ),
        WidgetType::Rule => rule_controls(
            &hierarchy,
            widget_id,
            theme,
            &type_system,
            custom_styles,
            preview_content,
        ),
        WidgetType::Space => space_controls(
            &hierarchy,
            widget_id,
            theme,
            &type_system,
            custom_styles,
            preview_content,
        ),
        WidgetType::ProgressBar => progress_controls(
            &hierarchy,
            widget_id,
            theme,
            &type_system,
            custom_styles,
            preview_content,
            views,
            view_id,
        ),
        WidgetType::Image => image_controls(
            &hierarchy,
            widget_id,
            theme,
            &type_system,
            custom_styles,
            preview_content,
        ),
        WidgetType::Svg => svg_controls(
            &hierarchy,
            widget_id,
            theme,
            &type_system,
            custom_styles,
            preview_content,
        ),
        WidgetType::Tooltip => tooltip_controls(
            &hierarchy,
            widget_id,
            theme,
            &type_system,
            custom_styles,
            preview_content,
        ),
        WidgetType::ComboBox => combobox_controls(
            &hierarchy,
            widget_id,
            theme,
            &type_system,
            custom_styles,
            preview_content,
        ),
        WidgetType::Markdown => markdown_controls(
            &hierarchy,
            widget_id,
            theme,
            &type_system,
            custom_styles,
            preview_content,
        ),
        WidgetType::MouseArea => mousearea_controls(
            &hierarchy,
            widget_id,
            theme,
            &type_system,
            custom_styles,
            preview_content,
        ),
        WidgetType::QRCode => qrcode_controls(
            &hierarchy,
            widget_id,
            theme,
            &type_system,
            custom_styles,
            preview_content,
        ),
        WidgetType::Stack => stack_controls(
            &hierarchy,
            widget_id,
            theme,
            &type_system,
            custom_styles,
            preview_content,
        ),
        WidgetType::Collapsible => collapsible_controls(
            &hierarchy,
            widget_id,
            theme,
            &type_system,
            custom_styles,
            preview_content,
            views,
            view_id,
        ),
        WidgetType::CollapsibleGroup => collapsible_group_controls(
            &hierarchy,
            widget_id,
            theme,
            &type_system,
            custom_styles,
            preview_content,
        ),
        WidgetType::Popover | WidgetType::Menu | WidgetType::Layer | WidgetType::Toast => overlay_controls(
            &hierarchy,
            widget_id,
            theme,
            &type_system,
            custom_styles,
            preview_content,
            views,
            view_id,
        ),
        WidgetType::Themer => themer_controls(
            &hierarchy,
            widget_id,
            theme,
            &type_system,
            custom_styles,
            preview_content,
        ),
        WidgetType::Grid => grid_controls(
            &hierarchy,
            widget_id,
            theme,
            &type_system,
            custom_styles,
            preview_content,
        ),
        WidgetType::Pin => pin_controls(
            &hierarchy,
            widget_id,
            theme,
            &type_system,
            custom_styles,
            preview_content,
        ),
        WidgetType::Table => table_controls(
            &hierarchy,
            widget_id,
            theme,
            &type_system,
            custom_styles,
            preview_content,
        ),
        WidgetType::DataTable => data_table_controls(
            &hierarchy,
            widget_id,
            theme,
            &type_system,
            custom_styles,
            preview_content,
        ),
        WidgetType::ViewReference => view_reference_controls(
            &hierarchy,
            widget_id,
            theme,
            &type_system,
            views,
            custom_styles,
            preview_content,
        ),
        WidgetType::Icon => icon_controls(
            &hierarchy,
            widget_id,
            theme,
            &type_system,
            custom_styles,
            preview_content,
        ),
    };

    // A widget that is a row of a Menu or an item in a Toast carries settings
    // belonging to that role rather than to its own type, so they sit above its
    // own panel whatever type it happens to be.
    match overlay_child_role(&hierarchy, widget_id) {
        Some(role) => column![
            overlay_child_controls(role, widget_id, &widget.properties),
            controls_view,
        ]
        .spacing(5)
        .into(),
        None => controls_view.into(),
    }
}

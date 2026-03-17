use iced::alignment::{Horizontal, Vertical};
use iced::widget::{
    Pin, button, checkbox, column, combo_box, container, grid, image, markdown, mouse_area,
    pick_list, progress_bar, radio, row, rule, scrollable, slider, space, stack, svg, table, text,
    text_input, themer, toggler, tooltip, vertical_slider,
};
use iced::{Alignment, Background, Border, Color, Element, Font, Length, Task, Theme, font};
use std::collections::{BTreeMap, HashMap};
use uuid::Uuid;

use crate::action_system::node_kinds::ActionValue;
use crate::action_system::state_ref::{
    date_picker_open_state_key, generic_overlay_open_state_key, view_reference_selection_state_key,
    view_selection_state_key,
};
use crate::data_structures::properties::messages::PropertyChange;
use crate::data_structures::types::type_implementations::*;
use crate::data_structures::types::types::*;
use crate::data_structures::widget_hierarchy::WidgetHierarchy;
use crate::enum_builder::TypeSystem;
use crate::preview_runtime::interpreter;
use crate::views::theme_and_stylefn_builder::{CustomThemes, ThemePaneEnum};
use widgets::collapsible::{self as collapsible_widget, CollapsibleGroup};
use widgets::{date_picker as date_picker_widget, generic_overlay};

#[derive(Debug, Clone)]
pub enum Message {
    // Widget Operations
    PropertyChanged(WidgetId, PropertyChange, Option<Uuid>),

    // Interactive widget messages
    TextInputChanged(WidgetId, String, Option<Uuid>),
    Submitted(WidgetId, Option<Uuid>),
    TextPasted(WidgetId, String, Option<Uuid>),
    CheckboxToggled(WidgetId, bool, Option<Uuid>),
    RadioSelected(WidgetId, usize, Option<Uuid>),
    SliderChanged(WidgetId, f32, Option<Uuid>),
    TogglerToggled(WidgetId, bool, Option<Uuid>),
    PickListSelected(WidgetId, String, Option<Uuid>),
    GenericOverlayToggled(WidgetId, bool, Option<Uuid>),
    DatePickerOpenRequested(WidgetId, Option<Uuid>),
    DatePickerChanged(WidgetId, date_picker_widget::DateSelection, Option<Uuid>),
    DatePickerChangedWithTime(
        WidgetId,
        date_picker_widget::DateSelection,
        date_picker_widget::TimeSelection,
        Option<Uuid>,
    ),
    DatePickerClosed(WidgetId, Option<Uuid>),
    ComboBoxOnInput(WidgetId, String, Option<Uuid>),
    ComboBoxOnOptionHovered(WidgetId, String, Option<Uuid>),
    ComboBoxOnClose(WidgetId, Option<Uuid>),
    ComboBoxOnOpen(WidgetId, Option<Uuid>),
    /// Emitted when a NavigateToView action node fires during preview.
    NavigatedToView(Uuid),
    /// Button pressed — checked against action graph before defaulting to Noop.
    ButtonPressed(WidgetId, Option<Uuid>),
    Noop,
}

fn custom_style_definition(
    custom_themes: &CustomThemes,
    pane: ThemePaneEnum,
    style_name: &str,
) -> Option<crate::styles::style_enum::SavedStyleDefinition> {
    custom_themes
        .styles()
        .get(&pane)
        .and_then(|styles| styles.get(style_name))
        .cloned()
}

fn resolve_container_style(
    custom_themes: &CustomThemes,
    theme: &Theme,
    style_name: &str,
) -> Option<container::Style> {
    if let Some(style_definition) =
        custom_style_definition(custom_themes, ThemePaneEnum::Container, style_name)
    {
        return Some(style_definition.to_container_style(theme));
    }

    ContainerStyleType::get(style_name).map(|style| match style {
        ContainerStyleType::Transparent => container::transparent(theme),
        ContainerStyleType::Background => {
            container::background(theme.extended_palette().background.base.color)
        }
        ContainerStyleType::RoundedBox => container::rounded_box(theme),
        ContainerStyleType::BorderedBox => container::bordered_box(theme),
        ContainerStyleType::Dark => container::dark(theme),
        ContainerStyleType::Primary => container::primary(theme),
        ContainerStyleType::Secondary => container::secondary(theme),
        ContainerStyleType::Success => container::success(theme),
        ContainerStyleType::Danger => container::danger(theme),
        ContainerStyleType::Warning => container::warning(theme),
    })
}

fn resolve_button_style(
    custom_themes: &CustomThemes,
    theme: &Theme,
    status: button::Status,
    style_name: &str,
) -> Option<button::Style> {
    if let Some(style_definition) =
        custom_style_definition(custom_themes, ThemePaneEnum::Button, style_name)
    {
        return Some(style_definition.to_button_style(theme, status));
    }

    ButtonStyleType::get(style_name).map(|style| match style {
        ButtonStyleType::Primary => button::primary(theme, status),
        ButtonStyleType::Secondary => button::secondary(theme, status),
        ButtonStyleType::Success => button::success(theme, status),
        ButtonStyleType::Danger => button::danger(theme, status),
        ButtonStyleType::Text => button::text(theme, status),
        ButtonStyleType::Background => button::background(theme, status),
        ButtonStyleType::Subtle => button::subtle(theme, status),
    })
}

fn resolve_checkbox_style(
    custom_themes: &CustomThemes,
    theme: &Theme,
    status: checkbox::Status,
    style_name: &str,
) -> Option<checkbox::Style> {
    if let Some(style_definition) =
        custom_style_definition(custom_themes, ThemePaneEnum::Checkbox, style_name)
    {
        return Some(style_definition.to_checkbox_style(theme, status));
    }

    match style_name {
        "Primary" => Some(checkbox::primary(theme, status)),
        "Secondary" => Some(checkbox::secondary(theme, status)),
        "Success" => Some(checkbox::success(theme, status)),
        "Danger" => Some(checkbox::danger(theme, status)),
        _ => None,
    }
}

fn resolve_radio_style(
    custom_themes: &CustomThemes,
    theme: &Theme,
    status: radio::Status,
    style_name: &str,
) -> Option<radio::Style> {
    if let Some(style_definition) =
        custom_style_definition(custom_themes, ThemePaneEnum::Radio, style_name)
    {
        return Some(style_definition.to_radio_style(theme, status));
    }

    match style_name {
        "Default" => Some(radio::default(theme, status)),
        _ => None,
    }
}

fn resolve_slider_style(
    custom_themes: &CustomThemes,
    theme: &Theme,
    status: slider::Status,
    style_name: &str,
) -> Option<slider::Style> {
    if let Some(style_definition) =
        custom_style_definition(custom_themes, ThemePaneEnum::Slider, style_name)
    {
        return Some(style_definition.to_slider_style(theme, status));
    }

    match style_name {
        "Default" => Some(slider::default(theme, status)),
        _ => None,
    }
}

fn resolve_progress_bar_style(
    custom_themes: &CustomThemes,
    theme: &Theme,
    style_name: &str,
) -> Option<progress_bar::Style> {
    if let Some(style_definition) =
        custom_style_definition(custom_themes, ThemePaneEnum::Progressbar, style_name)
    {
        return Some(style_definition.to_progress_bar_style(theme));
    }

    match style_name {
        "Primary" => Some(progress_bar::primary(theme)),
        "Secondary" => Some(progress_bar::secondary(theme)),
        "Success" => Some(progress_bar::success(theme)),
        "Warning" => Some(progress_bar::warning(theme)),
        "Danger" => Some(progress_bar::danger(theme)),
        _ => None,
    }
}

fn resolve_toggler_style(
    custom_themes: &CustomThemes,
    theme: &Theme,
    status: toggler::Status,
    style_name: &str,
) -> Option<toggler::Style> {
    if let Some(style_definition) =
        custom_style_definition(custom_themes, ThemePaneEnum::Toggler, style_name)
    {
        return Some(style_definition.to_toggler_style(theme, status));
    }

    match style_name {
        "Default" => Some(toggler::default(theme, status)),
        _ => None,
    }
}

fn resolve_pick_list_style(
    custom_themes: &CustomThemes,
    theme: &Theme,
    status: pick_list::Status,
    style_name: &str,
) -> Option<pick_list::Style> {
    if let Some(style_definition) =
        custom_style_definition(custom_themes, ThemePaneEnum::Picklist, style_name)
    {
        return Some(style_definition.to_pick_list_style(theme, status));
    }

    match style_name {
        "Default" => Some(pick_list::default(theme, status)),
        _ => None,
    }
}

fn resolve_collapsible_style(
    theme: &Theme,
    status: collapsible_widget::Status,
    style_name: &str,
) -> Option<collapsible_widget::Style> {
    match style_name {
        "Default" => Some(collapsible_widget::default(theme, status)),
        "Primary" => Some(collapsible_widget::primary(theme, status)),
        "Success" => Some(collapsible_widget::success(theme, status)),
        "Danger" => Some(collapsible_widget::danger(theme, status)),
        "Warning" => Some(collapsible_widget::warning(theme, status)),
        _ => None,
    }
}

fn resolve_generic_overlay_style(
    theme: &Theme,
    style_name: &str,
) -> Option<generic_overlay::Style> {
    match style_name {
        "Primary" => Some(generic_overlay::primary(theme)),
        "Success" => Some(generic_overlay::success(theme)),
        "Danger" => Some(generic_overlay::danger(theme)),
        "Warning" => Some(generic_overlay::warning(theme)),
        "Blank" => Some(generic_overlay::blank(theme)),
        _ => None,
    }
}

fn should_use_alternate_style(
    widget: &Widget,
    current_view_id: Option<Uuid>,
    all_views: &BTreeMap<Uuid, AppView>,
) -> bool {
    if widget.properties.active_style_name.is_none() {
        return false;
    }

    match widget.properties.style_condition_field.as_deref() {
        None => true,
        Some(field) if field.trim().is_empty() => false,
        Some(field) => {
            let Some(expected) = widget
                .properties
                .style_condition_value
                .as_deref()
                .map(str::trim)
                .filter(|value| !value.is_empty())
            else {
                return false;
            };

            style_condition_matches(current_view_id, all_views, field.trim(), expected)
        }
    }
}

fn selected_widget_style_name<'a>(
    widget: &'a Widget,
    current_view_id: Option<Uuid>,
    all_views: &BTreeMap<Uuid, AppView>,
) -> Option<&'a str> {
    let base_style_name = widget.properties.custom_style_name.as_deref();

    if should_use_alternate_style(widget, current_view_id, all_views) {
        widget
            .properties
            .active_style_name
            .as_deref()
            .or(base_style_name)
    } else {
        base_style_name
    }
}

fn generic_overlay_is_open(
    current_view_id: Option<Uuid>,
    all_views: &BTreeMap<Uuid, AppView>,
    widget_id: WidgetId,
) -> bool {
    let Some(view_id) = current_view_id else {
        return false;
    };
    let Some(view) = all_views.get(&view_id) else {
        return false;
    };

    let key = generic_overlay_open_state_key(view_id, widget_id);
    if let Some(ActionValue::Bool(is_open)) = view.custom_state_values.get(&key) {
        return *is_open;
    }

    view.hierarchy
        .get_widget_by_id(widget_id)
        .map(|widget| widget.properties.generic_overlay_initially_open)
        .unwrap_or(false)
}

fn date_picker_is_open(
    current_view_id: Option<Uuid>,
    all_views: &BTreeMap<Uuid, AppView>,
    widget_id: WidgetId,
) -> bool {
    let Some(view_id) = current_view_id else {
        return false;
    };
    let Some(view) = all_views.get(&view_id) else {
        return false;
    };

    let key = date_picker_open_state_key(view_id, widget_id);
    if let Some(ActionValue::Bool(is_open)) = view.custom_state_values.get(&key) {
        return *is_open;
    }

    view.hierarchy
        .get_widget_by_id(widget_id)
        .map(|widget| widget.properties.date_picker_initially_open)
        .unwrap_or(false)
}

fn parse_date_picker_date(raw: &str) -> Option<date_picker_widget::Date> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        None
    } else {
        date_picker_widget::Date::parse_from_str(trimmed, "%Y-%m-%d").ok()
    }
}

fn date_picker_selection(widget: &Widget) -> date_picker_widget::DateSelection {
    let props = &widget.properties;
    match props.date_picker_mode {
        DatePickerSelectionMode::Single => date_picker_widget::DateSelection::Single(
            parse_date_picker_date(&props.date_picker_initial_single_date),
        ),
        DatePickerSelectionMode::Range => date_picker_widget::DateSelection::Range {
            start: parse_date_picker_date(&props.date_picker_initial_range_start),
            end: parse_date_picker_date(&props.date_picker_initial_range_end),
        },
    }
}

fn date_picker_time(widget: &Widget) -> date_picker_widget::TimeSelection {
    let props = &widget.properties;
    date_picker_widget::TimeSelection::new(
        props.date_picker_initial_hour,
        props.date_picker_initial_minute,
    )
}

fn format_date_picker_time(time: date_picker_widget::TimeSelection) -> String {
    let is_pm = time.hour >= 12;
    let hour = match time.hour % 12 {
        0 => 12,
        value => value,
    };
    format!(
        "{:02}:{:02} {}",
        hour,
        time.minute,
        if is_pm { "PM" } else { "AM" }
    )
}

fn format_date_picker_button_label(
    selection: &date_picker_widget::DateSelection,
    time: date_picker_widget::TimeSelection,
    placeholder: &str,
    show_time: bool,
) -> String {
    let append_time = |base: String| {
        if show_time {
            format!("{} {}", base, format_date_picker_time(time))
        } else {
            base
        }
    };

    match selection {
        date_picker_widget::DateSelection::Single(Some(date)) => {
            append_time(date.format("%m/%d/%Y").to_string())
        }
        date_picker_widget::DateSelection::Range {
            start: Some(start),
            end: Some(end),
        } => append_time(format!(
            "{} -> {}",
            start.format("%m/%d/%Y"),
            end.format("%m/%d/%Y")
        )),
        date_picker_widget::DateSelection::Range {
            start: Some(start),
            end: None,
        } => append_time(format!("{} -> ...", start.format("%m/%d/%Y"))),
        _ => placeholder.to_string(),
    }
}

fn apply_date_picker_selection(widget: &mut Widget, selection: &date_picker_widget::DateSelection) {
    match selection {
        date_picker_widget::DateSelection::Single(date) => {
            widget.properties.date_picker_mode = DatePickerSelectionMode::Single;
            widget.properties.date_picker_initial_single_date = date
                .map(|value| value.format("%Y-%m-%d").to_string())
                .unwrap_or_default();
            widget.properties.date_picker_initial_range_start.clear();
            widget.properties.date_picker_initial_range_end.clear();
        }
        date_picker_widget::DateSelection::Range { start, end } => {
            widget.properties.date_picker_mode = DatePickerSelectionMode::Range;
            widget.properties.date_picker_initial_single_date.clear();
            widget.properties.date_picker_initial_range_start = start
                .map(|value| value.format("%Y-%m-%d").to_string())
                .unwrap_or_default();
            widget.properties.date_picker_initial_range_end = end
                .map(|value| value.format("%Y-%m-%d").to_string())
                .unwrap_or_default();
        }
    }
}

fn apply_date_picker_time(widget: &mut Widget, time: date_picker_widget::TimeSelection) {
    widget.properties.date_picker_initial_hour = time.hour.min(23);
    widget.properties.date_picker_initial_minute = time.minute.min(59);
}

fn style_condition_matches(
    current_view_id: Option<Uuid>,
    all_views: &BTreeMap<Uuid, AppView>,
    field_name: &str,
    expected: &str,
) -> bool {
    let Some(view_id) = current_view_id else {
        return false;
    };
    let Some(view) = all_views.get(&view_id) else {
        return false;
    };

    if let Some(custom_field) = view
        .custom_state
        .iter()
        .find(|field| field.name == field_name)
    {
        if let Some(value) = view.custom_state_values.get(&custom_field.id) {
            return action_value_matches_expected(value, expected);
        }
    }

    let selection_key = view_selection_state_key(view_id, field_name);
    view.custom_state_values
        .get(&selection_key)
        .is_some_and(|value| action_value_matches_expected(value, expected))
}

fn action_value_matches_expected(value: &ActionValue, expected: &str) -> bool {
    let expected = expected.trim();
    let expected_unquoted = expected
        .strip_prefix('"')
        .and_then(|trimmed| trimmed.strip_suffix('"'))
        .unwrap_or(expected);

    match value {
        ActionValue::Bool(flag) => {
            (*flag && expected.eq_ignore_ascii_case("true"))
                || (!*flag && expected.eq_ignore_ascii_case("false"))
        }
        ActionValue::Number(number) => expected
            .parse::<f64>()
            .map(|parsed| (*number - parsed).abs() < 0.000_001)
            .unwrap_or(false),
        ActionValue::String(text) => {
            text == expected_unquoted || expected.ends_with(&format!("::{}", text))
        }
        ActionValue::EnumVariant { type_name, variant } => {
            expected == variant || expected == format!("{type_name}::{variant}")
        }
    }
}

pub fn update(
    flows: &[crate::action_system::flow::AppFlow],
    all_views: &mut BTreeMap<Uuid, AppView>,
    type_system: &mut TypeSystem,
    message: Message,
) -> Task<Message> {
    match message {
        Message::PropertyChanged(id, change, view_id) => {
            if let Some(view_id) = view_id {
                if let Some(view) = all_views.get_mut(&view_id) {
                    view.hierarchy
                        .apply_property_change(id, change.clone(), type_system);

                    match view.hierarchy.get_widget_by_id(id) {
                        Some(widget) => {
                            if widget.widget_type == WidgetType::Space {
                                match change {
                                    PropertyChange::Orientation(Orientation::Horizontal) => {
                                        view.hierarchy.apply_property_change(
                                            id,
                                            PropertyChange::Width(Length::Fill),
                                            type_system,
                                        );
                                        view.hierarchy.apply_property_change(
                                            id,
                                            PropertyChange::Height(Length::Shrink),
                                            type_system,
                                        );
                                    }
                                    PropertyChange::Orientation(Orientation::Vertical) => {
                                        view.hierarchy.apply_property_change(
                                            id,
                                            PropertyChange::Width(Length::Shrink),
                                            type_system,
                                        );
                                        view.hierarchy.apply_property_change(
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
            }
        }

        Message::TextInputChanged(id, value, view_id) => {
            if let Some(view_id) = view_id {
                let has_graph = has_action_graph(flows, view_id, id, "on_input");
                if has_graph {
                    let mut payload = HashMap::new();
                    payload.insert("text".to_string(), ActionValue::String(value));
                    if let Some(nav) = interpreter::execute_event(
                        flows, all_views, view_id, id, "on_input", payload,
                    ) {
                        return Task::done(Message::NavigatedToView(nav));
                    }
                } else if let Some(view) = all_views.get_mut(&view_id) {
                    view.hierarchy.apply_property_change(
                        id,
                        PropertyChange::TextInputValue(value),
                        type_system,
                    );
                }
            }
        }

        Message::Submitted(id, view_id) => {
            if let Some(view_id) = view_id {
                let has_graph = has_action_graph(flows, view_id, id, "on_submit");
                if has_graph {
                    if let Some(nav) = interpreter::execute_event(
                        flows,
                        all_views,
                        view_id,
                        id,
                        "on_submit",
                        HashMap::new(),
                    ) {
                        return Task::done(Message::NavigatedToView(nav));
                    }
                } else {
                    println!("{:?}, text_input submitted.", id);
                }
            }
        }

        Message::TextPasted(id, value, view_id) => {
            if let Some(view_id) = view_id {
                let has_graph = has_action_graph(flows, view_id, id, "on_paste");
                if has_graph {
                    let mut payload = HashMap::new();
                    payload.insert("text".to_string(), ActionValue::String(value));
                    if let Some(nav) = interpreter::execute_event(
                        flows, all_views, view_id, id, "on_paste", payload,
                    ) {
                        return Task::done(Message::NavigatedToView(nav));
                    }
                } else if let Some(view) = all_views.get_mut(&view_id) {
                    view.hierarchy.apply_property_change(
                        id,
                        PropertyChange::TextInputValue(value),
                        type_system,
                    );
                }
            }
        }

        Message::CheckboxToggled(id, checked, view_id) => {
            if let Some(view_id) = view_id {
                let has_graph = has_action_graph(flows, view_id, id, "on_toggle");
                if has_graph {
                    let mut payload = HashMap::new();
                    payload.insert("checked".to_string(), ActionValue::Bool(checked));
                    if let Some(nav) = interpreter::execute_event(
                        flows,
                        all_views,
                        view_id,
                        id,
                        "on_toggle",
                        payload,
                    ) {
                        return Task::done(Message::NavigatedToView(nav));
                    }
                } else if let Some(view) = all_views.get_mut(&view_id) {
                    view.hierarchy.apply_property_change(
                        id,
                        PropertyChange::CheckboxChecked(checked),
                        type_system,
                    );
                }
            }
        }

        Message::RadioSelected(id, index, view_id) => {
            if let Some(view_id) = view_id {
                let has_graph = has_action_graph(flows, view_id, id, "on_select");
                if has_graph {
                    let mut payload = HashMap::new();
                    payload.insert("selected".to_string(), ActionValue::Number(index as f64));
                    if let Some(nav) = interpreter::execute_event(
                        flows,
                        all_views,
                        view_id,
                        id,
                        "on_select",
                        payload,
                    ) {
                        return Task::done(Message::NavigatedToView(nav));
                    }
                } else if let Some(view) = all_views.get_mut(&view_id) {
                    view.hierarchy.apply_property_change(
                        id,
                        PropertyChange::RadioSelectedIndex(index),
                        type_system,
                    );
                }
            }
        }

        Message::SliderChanged(id, value, view_id) => {
            if let Some(view_id) = view_id {
                let has_graph = has_action_graph(flows, view_id, id, "on_change");
                if has_graph {
                    let mut payload = HashMap::new();
                    payload.insert("value".to_string(), ActionValue::Number(value as f64));
                    if let Some(nav) = interpreter::execute_event(
                        flows,
                        all_views,
                        view_id,
                        id,
                        "on_change",
                        payload,
                    ) {
                        return Task::done(Message::NavigatedToView(nav));
                    }
                } else if let Some(view) = all_views.get_mut(&view_id) {
                    view.hierarchy.apply_property_change(
                        id,
                        PropertyChange::SliderValue(value),
                        type_system,
                    );
                }
            }
        }

        Message::TogglerToggled(id, active, view_id) => {
            if let Some(view_id) = view_id {
                let has_graph = has_action_graph(flows, view_id, id, "on_toggle");
                if has_graph {
                    let mut payload = HashMap::new();
                    payload.insert("checked".to_string(), ActionValue::Bool(active));
                    if let Some(nav) = interpreter::execute_event(
                        flows,
                        all_views,
                        view_id,
                        id,
                        "on_toggle",
                        payload,
                    ) {
                        return Task::done(Message::NavigatedToView(nav));
                    }
                } else if let Some(view) = all_views.get_mut(&view_id) {
                    view.hierarchy.apply_property_change(
                        id,
                        PropertyChange::TogglerActive(active),
                        type_system,
                    );
                }
            }
        }

        Message::PickListSelected(id, value, view_id) => {
            if let Some(view_id) = view_id {
                let has_graph = has_action_graph(flows, view_id, id, "on_select");
                if has_graph {
                    let mut payload = HashMap::new();
                    payload.insert("selected".to_string(), ActionValue::String(value));
                    if let Some(nav) = interpreter::execute_event(
                        flows,
                        all_views,
                        view_id,
                        id,
                        "on_select",
                        payload,
                    ) {
                        return Task::done(Message::NavigatedToView(nav));
                    }
                } else if let Some(view) = all_views.get_mut(&view_id) {
                    view.hierarchy.apply_property_change(
                        id,
                        PropertyChange::PickListSelected(Some(value)),
                        type_system,
                    );
                }
            }
        }

        Message::GenericOverlayToggled(id, is_open, view_id) => {
            if let Some(view_id) = view_id {
                if let Some(view) = all_views.get_mut(&view_id) {
                    let key = generic_overlay_open_state_key(view_id, id);
                    view.custom_state_values
                        .insert(key, ActionValue::Bool(is_open));
                }

                let has_graph = has_action_graph(flows, view_id, id, "on_toggle");
                if has_graph {
                    let mut payload = HashMap::new();
                    payload.insert("checked".to_string(), ActionValue::Bool(is_open));
                    if let Some(nav) = interpreter::execute_event(
                        flows,
                        all_views,
                        view_id,
                        id,
                        "on_toggle",
                        payload,
                    ) {
                        return Task::done(Message::NavigatedToView(nav));
                    }
                }
            }
        }

        Message::DatePickerOpenRequested(id, view_id) => {
            if let Some(view_id) = view_id
                && let Some(view) = all_views.get_mut(&view_id)
            {
                let key = date_picker_open_state_key(view_id, id);
                view.custom_state_values
                    .insert(key, ActionValue::Bool(true));
            }
        }

        Message::DatePickerChanged(id, selection, view_id) => {
            if let Some(view_id) = view_id
                && let Some(view) = all_views.get_mut(&view_id)
                && let Some(widget) = view.hierarchy.get_widget_by_id_mut(id)
            {
                apply_date_picker_selection(widget, &selection);
            }
        }

        Message::DatePickerChangedWithTime(id, selection, time, view_id) => {
            if let Some(view_id) = view_id
                && let Some(view) = all_views.get_mut(&view_id)
                && let Some(widget) = view.hierarchy.get_widget_by_id_mut(id)
            {
                apply_date_picker_selection(widget, &selection);
                apply_date_picker_time(widget, time);
            }
        }

        Message::DatePickerClosed(id, view_id) => {
            if let Some(view_id) = view_id
                && let Some(view) = all_views.get_mut(&view_id)
            {
                let key = date_picker_open_state_key(view_id, id);
                view.custom_state_values
                    .insert(key, ActionValue::Bool(false));
            }
        }

        Message::ComboBoxOnInput(id, value, view_id) => {
            if let Some(view_id) = view_id {
                if let Some(view) = all_views.get_mut(&view_id) {
                    let props = &view.hierarchy.get_widget_by_id(id).unwrap().properties;
                    if props.combobox_use_on_input {
                        println!("combobox {:?} input text: {}", id, value);
                    }
                }
            }
        }
        Message::ComboBoxOnOpen(id, view_id) => {
            if let Some(view_id) = view_id {
                if let Some(view) = all_views.get_mut(&view_id) {
                    let props = &view.hierarchy.get_widget_by_id(id).unwrap().properties;
                    if props.combobox_use_on_open {
                        println!("combobox {:?} opened!", id);
                    }
                }
            }
        }
        Message::ComboBoxOnClose(id, view_id) => {
            if let Some(view_id) = view_id {
                if let Some(view) = all_views.get_mut(&view_id) {
                    let props = &view.hierarchy.get_widget_by_id(id).unwrap().properties;
                    if props.combobox_use_on_close {
                        println!("combobox {:?} closed!", id);
                    }
                }
            }
        }
        Message::ComboBoxOnOptionHovered(id, options, view_id) => {
            if let Some(view_id) = view_id {
                if let Some(view) = all_views.get_mut(&view_id) {
                    let props = &view.hierarchy.get_widget_by_id(id).unwrap().properties;
                    if props.combobox_use_on_option_hovered {
                        println!("combobox option hovered: {:?}", options);
                    }
                }
            }
        }
        Message::ButtonPressed(id, view_id) => {
            if let Some(view_id) = view_id {
                let has_graph = has_action_graph(flows, view_id, id, "on_press");
                if has_graph {
                    if let Some(nav) = interpreter::execute_event(
                        flows,
                        all_views,
                        view_id,
                        id,
                        "on_press",
                        HashMap::new(),
                    ) {
                        return Task::done(Message::NavigatedToView(nav));
                    }
                }
                // No action graph or no nav result — nothing to do for on_press
            }
        }

        Message::NavigatedToView(_) => {
            // Handled upstream in main.rs — should not reach here
        }

        Message::Noop => {
            // Do nothing - for preview-only interactions
        }
    }

    Task::none()
}

/// Returns true if any app-level flow targets the widget+event in the given view.
fn has_action_graph(
    flows: &[crate::action_system::flow::AppFlow],
    view_id: Uuid,
    widget_id: WidgetId,
    event_name: &str,
) -> bool {
    use crate::action_system::flow::FlowTrigger;
    flows.iter().any(|f| {
        if !f.enabled {
            return false;
        }
        if let FlowTrigger::WidgetEvent { rows } = &f.trigger {
            rows.iter()
                .any(|r| r.event_type == event_name && r.target == Some((view_id, widget_id.0)))
        } else {
            false
        }
    })
}

pub fn view<'a>(
    hierarchy: &'a WidgetHierarchy,
    theme: &'a Theme,
    custom_themes: &'a CustomThemes,
    highlight_selected: bool,
    all_views: &'a BTreeMap<Uuid, AppView>,
    current_view_id: Option<Uuid>,
    type_system: &'a TypeSystem,
) -> Element<'a, Message> {
    let widget_preview = build_widget_preview(
        hierarchy,
        hierarchy.root(),
        theme,
        custom_themes,
        highlight_selected,
        all_views,
        current_view_id,
        type_system,
    );

    widget_preview

    /*
    let preview_scoped = themer(
        Some(theme.clone()),

        container(widget_preview)
            .width(Length::Fill)
            .height(Length::Fill)
            // Any style closures here will now see the scoped theme
            .style(|theme: &Theme| container::Style {
                background: Some(Background::Color(theme.palette().background)),
                border: Border {
                    color: theme.extended_palette().background.strong.color,
                    width: 2.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            }),
    );
    // Optional: set default text color / background for this scope:
     .text_color(|th| th.palette().text)
    .background(|th| Background::Color(th.palette().background));

    theme.extended_palette().secondary.base.text;

    column![
        row![
            tooltip(
                text("Preview Layout").size(20),
                text("This represents your app's main content container")
                    .size(12)
                    .color(Color::from_rgb(0.6, 0.6, 0.6))
                    .center(),
                tooltip::Position::Right
            ),
        ]
        .align_y(Alignment::Center)
        .padding(
            Padding {
                top: 5.0,
                right: 10.0,
                bottom: 0.0,
                left: 10.0,
            }
        )
        .spacing(20),

        rule::horizontal(5),
        space::horizontal().height(10),

        container(preview_scoped)
        .padding(5)
        .style(|theme: &Theme| container::Style {
                background: Some(Background::Color(theme.extended_palette().background.weak.color)),
                ..Default::default()
            }),
    ]
    .spacing(10)
    .padding(10)
    .into() */
}

fn build_widget_preview<'a>(
    hierarchy: &'a WidgetHierarchy,
    widget: &'a Widget,
    theme: &'a Theme,
    custom_themes: &'a CustomThemes,
    highlight_selected: bool,
    all_views: &'a BTreeMap<Uuid, AppView>,
    current_view_id: Option<Uuid>,
    type_system: &'a TypeSystem,
) -> Element<'a, Message> {
    let is_selected = hierarchy.selected_ids().contains(&widget.id);

    let content = match widget.widget_type {
        WidgetType::Container => build_container_preview(
            hierarchy,
            widget,
            theme,
            custom_themes,
            highlight_selected,
            all_views,
            current_view_id,
            type_system,
        ),
        WidgetType::Row => build_row_preview(
            hierarchy,
            widget,
            theme,
            custom_themes,
            highlight_selected,
            all_views,
            current_view_id,
            type_system,
        ),
        WidgetType::Column => build_column_preview(
            hierarchy,
            widget,
            theme,
            custom_themes,
            highlight_selected,
            all_views,
            current_view_id,
            type_system,
        ),
        WidgetType::Button => build_button_preview(
            hierarchy,
            widget,
            theme,
            custom_themes,
            highlight_selected,
            all_views,
            current_view_id,
            type_system,
        ),
        WidgetType::Text => build_text_preview(
            hierarchy,
            widget,
            theme,
            custom_themes,
            highlight_selected,
            all_views,
            current_view_id,
            type_system,
        ),
        WidgetType::TextInput => build_text_input_preview(
            hierarchy,
            widget,
            theme,
            custom_themes,
            highlight_selected,
            all_views,
            current_view_id,
            type_system,
        ),
        WidgetType::Checkbox => build_checkbox_preview(
            hierarchy,
            widget,
            theme,
            custom_themes,
            highlight_selected,
            all_views,
            current_view_id,
            type_system,
        ),
        WidgetType::Radio => build_radio_preview(
            hierarchy,
            widget,
            theme,
            custom_themes,
            highlight_selected,
            all_views,
            current_view_id,
            type_system,
        ),
        WidgetType::Slider => build_slider_preview(
            hierarchy,
            widget,
            theme,
            custom_themes,
            highlight_selected,
            all_views,
            current_view_id,
            type_system,
        ),
        WidgetType::VerticalSlider => build_vertical_slider_preview(
            hierarchy,
            widget,
            theme,
            custom_themes,
            highlight_selected,
            all_views,
            current_view_id,
            type_system,
        ),
        WidgetType::ProgressBar => build_progress_bar_preview(
            hierarchy,
            widget,
            theme,
            custom_themes,
            highlight_selected,
            all_views,
            current_view_id,
            type_system,
        ),
        WidgetType::Toggler => build_toggler_preview(
            hierarchy,
            widget,
            theme,
            custom_themes,
            highlight_selected,
            all_views,
            current_view_id,
            type_system,
        ),
        WidgetType::PickList => build_pick_list_preview(
            hierarchy,
            widget,
            theme,
            custom_themes,
            highlight_selected,
            all_views,
            current_view_id,
            type_system,
        ),
        WidgetType::DatePicker => build_date_picker_preview(
            hierarchy,
            widget,
            theme,
            custom_themes,
            highlight_selected,
            all_views,
            current_view_id,
            type_system,
        ),
        WidgetType::Scrollable => build_scrollable_preview(
            hierarchy,
            widget,
            theme,
            custom_themes,
            highlight_selected,
            all_views,
            current_view_id,
            type_system,
        ),
        WidgetType::Space => build_space_preview(
            hierarchy,
            widget,
            theme,
            custom_themes,
            highlight_selected,
            all_views,
            current_view_id,
            type_system,
        ),
        WidgetType::Rule => build_rule_preview(
            hierarchy,
            widget,
            theme,
            custom_themes,
            highlight_selected,
            all_views,
            current_view_id,
            type_system,
        ),
        WidgetType::Image => build_image_preview(
            hierarchy,
            widget,
            theme,
            custom_themes,
            highlight_selected,
            all_views,
            current_view_id,
            type_system,
        ),
        WidgetType::Svg => build_svg_preview(
            hierarchy,
            widget,
            theme,
            custom_themes,
            highlight_selected,
            all_views,
            current_view_id,
            type_system,
        ),
        WidgetType::Tooltip => build_tooltip_preview(
            hierarchy,
            widget,
            theme,
            custom_themes,
            highlight_selected,
            all_views,
            current_view_id,
            type_system,
        ),
        WidgetType::ComboBox => build_combo_box_preview(
            hierarchy,
            widget,
            theme,
            custom_themes,
            highlight_selected,
            all_views,
            current_view_id,
            type_system,
        ),
        WidgetType::Markdown => build_markdown_preview(
            hierarchy,
            widget,
            theme,
            custom_themes,
            highlight_selected,
            all_views,
            current_view_id,
            type_system,
        ),
        WidgetType::MouseArea => build_mouse_area_preview(
            hierarchy,
            widget,
            theme,
            custom_themes,
            highlight_selected,
            all_views,
            current_view_id,
            type_system,
        ),
        WidgetType::QRCode => build_qr_code_preview(
            hierarchy,
            widget,
            theme,
            custom_themes,
            highlight_selected,
            all_views,
            current_view_id,
            type_system,
        ),
        WidgetType::Stack => build_stack_preview(
            hierarchy,
            widget,
            theme,
            custom_themes,
            highlight_selected,
            all_views,
            current_view_id,
            type_system,
        ),
        WidgetType::Collapsible => build_collapsible_preview(
            hierarchy,
            widget,
            theme,
            custom_themes,
            highlight_selected,
            all_views,
            current_view_id,
            type_system,
        ),
        WidgetType::CollapsibleGroup => build_collapsible_group_preview(
            hierarchy,
            widget,
            theme,
            custom_themes,
            highlight_selected,
            all_views,
            current_view_id,
            type_system,
        ),
        WidgetType::GenericOverlay => build_generic_overlay_preview(
            hierarchy,
            widget,
            theme,
            custom_themes,
            highlight_selected,
            all_views,
            current_view_id,
            type_system,
        ),
        WidgetType::Themer => build_themer_preview(
            hierarchy,
            widget,
            theme,
            custom_themes,
            highlight_selected,
            all_views,
            current_view_id,
            type_system,
        ),
        WidgetType::Grid => build_grid_preview(
            hierarchy,
            widget,
            theme,
            custom_themes,
            highlight_selected,
            all_views,
            current_view_id,
            type_system,
        ),
        WidgetType::Pin => build_pin_preview(
            hierarchy,
            widget,
            theme,
            custom_themes,
            highlight_selected,
            all_views,
            current_view_id,
            type_system,
        ),
        WidgetType::Table => build_table_preview(
            hierarchy,
            widget,
            theme,
            custom_themes,
            highlight_selected,
            all_views,
            current_view_id,
            type_system,
        ),
        WidgetType::ViewReference => build_view_reference_preview(
            hierarchy,
            widget,
            theme,
            custom_themes,
            highlight_selected,
            all_views,
            current_view_id,
            type_system,
        ),
        WidgetType::Icon => build_icon_preview(
            hierarchy,
            widget,
            theme,
            custom_themes,
            highlight_selected,
            all_views,
            current_view_id,
            type_system,
        ),
    };

    if is_selected && highlight_selected {
        content
            .explain(theme.extended_palette().success.strong.color)
            .into()
    } else {
        content
    }
}

fn build_container_preview<'a>(
    hierarchy: &'a WidgetHierarchy,
    widget: &'a Widget,
    theme: &'a Theme,
    custom_themes: &'a CustomThemes,
    highlight_selected: bool,
    all_views: &'a BTreeMap<Uuid, AppView>,
    current_view_id: Option<Uuid>,
    type_system: &'a TypeSystem,
) -> Element<'a, Message> {
    let props = &widget.properties;
    let mut container = container(if widget.children.is_empty() {
        text("Empty Container").into()
    } else {
        build_widget_preview(
            hierarchy,
            &widget.children[0],
            theme,
            custom_themes,
            highlight_selected,
            all_views,
            current_view_id,
            type_system,
        )
    });
    let base_style_name = props.custom_style_name.as_deref();
    let alternate_style_name = props.active_style_name.as_deref();
    let use_alternate_style = should_use_alternate_style(widget, current_view_id, all_views);

    // Apply all container properties
    container = match props.container_sizing_mode {
        ContainerSizingMode::Manual => {
            // Match codegen: skip Shrink so iced uses its fluid (child-based) default.
            // Shrink = "let iced decide"; Fill and Fixed are emitted explicitly.
            let mut c = container;
            if !matches!(props.width, Length::Shrink) {
                c = c.width(props.width);
            }
            if !matches!(props.height, Length::Shrink) {
                c = c.height(props.height);
            }
            if !matches!(props.align_x, ContainerAlignX::Left) {
                c = c.align_x(props.align_x);
            }
            if !matches!(props.align_y, ContainerAlignY::Top) {
                c = c.align_y(props.align_y);
            }
            c
        }
        ContainerSizingMode::CenterX => container.center_x(props.container_center_length),
        ContainerSizingMode::CenterY => container.center_y(props.container_center_length),
        ContainerSizingMode::Center => container.center(props.container_center_length),
    };

    container = container.padding(props.padding);

    // NEW: Apply max_width
    if let Some(max_w) = props.max_width {
        container = container.max_width(max_w);
    }

    // NEW: Apply max_height
    if let Some(max_h) = props.max_height {
        container = container.max_height(max_h);
    }

    // NEW: Apply clip
    if props.clip {
        container = container.clip(true);
    }

    // NEW: Apply widget ID
    if let Some(ref id) = props.widget_id {
        if !id.is_empty() {
            container = container.id(id.clone());
        }
    }

    container = container.style(move |theme: &Theme| {
        let selected_style_name = if use_alternate_style {
            alternate_style_name.or(base_style_name)
        } else {
            base_style_name
        };

        if let Some(style_name) = selected_style_name {
            if let Some(style) = resolve_container_style(custom_themes, theme, style_name) {
                return style;
            }
        }
        // No style assigned — return default (no border, no background)
        container::Style::default()
    });

    container.into()
}

fn build_row_preview<'a>(
    hierarchy: &'a WidgetHierarchy,
    widget: &'a Widget,
    theme: &'a Theme,
    custom_themes: &'a CustomThemes,
    highlight_selected: bool,
    all_views: &'a BTreeMap<Uuid, AppView>,
    current_view_id: Option<Uuid>,
    type_system: &'a TypeSystem,
) -> Element<'a, Message> {
    let props = &widget.properties;
    let children: Vec<Element<'a, Message>> = widget
        .children
        .iter()
        .map(|_child| {
            build_widget_preview(
                hierarchy,
                &widget.children[0],
                theme,
                custom_themes,
                highlight_selected,
                all_views,
                current_view_id,
                type_system,
            )
        })
        .collect();

    if props.is_wrapping_row {
        // Wrapping rows MUST use row(children) pattern
        let mut wrapping = row(children)
            .spacing(props.spacing)
            .padding(props.padding)
            .width(props.width)
            .height(props.height)
            .wrap();

        // Apply vertical spacing if set
        if props.match_horizontal_spacing {
            wrapping = wrapping.vertical_spacing(props.wrapping_vertical_spacing);
        }

        // Apply horizontal alignment
        wrapping = wrapping.align_x(props.wrapping_align_x);

        wrapping.into()
    } else {
        // Non-wrapping rows: Use the old working pattern
        let mut content = row![]
            .spacing(props.spacing)
            .padding(props.padding)
            .width(props.width)
            .height(props.height)
            .align_y(match props.align_items {
                Alignment::Start => Vertical::Top,
                Alignment::Center => Vertical::Center,
                Alignment::End => Vertical::Bottom,
            });

        if widget.children.is_empty() {
            content = content.push(text("Row Item 1"));
            content = content.push(text("Row Item 2"));
        } else {
            for child in &widget.children {
                content = content.push(build_widget_preview(
                    hierarchy,
                    child,
                    theme,
                    custom_themes,
                    highlight_selected,
                    all_views,
                    current_view_id,
                    type_system,
                ));
            }
        }

        if props.clip {
            content = content.clip(true);
        }

        content.into()
    }
}

fn build_column_preview<'a>(
    hierarchy: &'a WidgetHierarchy,
    widget: &'a Widget,
    theme: &'a Theme,
    custom_themes: &'a CustomThemes,
    highlight_selected: bool,
    all_views: &'a BTreeMap<Uuid, AppView>,
    current_view_id: Option<Uuid>,
    type_system: &'a TypeSystem,
) -> Element<'a, Message> {
    let props = &widget.properties;
    let mut content = column![]
        .spacing(props.spacing)
        .padding(props.padding)
        .width(props.width)
        .height(props.height)
        .align_x(match props.align_items {
            Alignment::Start => Horizontal::Left,
            Alignment::Center => Horizontal::Center,
            Alignment::End => Horizontal::Right,
        });

    if widget.children.is_empty() {
        content = content.push(text("Column Item 1"));
        content = content.push(text("Column Item 2"));
    } else {
        for child in &widget.children {
            content = content.push(build_widget_preview(
                hierarchy,
                child,
                theme,
                custom_themes,
                highlight_selected,
                all_views,
                current_view_id,
                type_system,
            ));
        }
    }

    if let Some(max_w) = props.max_width {
        content = content.max_width(max_w);
    }

    if props.clip {
        content = content.clip(true);
    }

    content.into()
}

fn build_button_preview<'a>(
    hierarchy: &'a WidgetHierarchy,
    widget: &'a Widget,
    theme: &'a Theme,
    custom_themes: &'a CustomThemes,
    highlight_selected: bool,
    all_views: &'a BTreeMap<Uuid, AppView>,
    current_view_id: Option<Uuid>,
    type_system: &'a TypeSystem,
) -> Element<'a, Message> {
    let props = &widget.properties;

    // Use child element as button content when one is present,
    // otherwise fall back to the text_content property.
    let button_content: Element<_> = if widget.children.is_empty() {
        text(&props.text_content).into()
    } else {
        build_widget_preview(
            hierarchy,
            &widget.children[0],
            theme,
            custom_themes,
            highlight_selected,
            all_views,
            current_view_id,
            type_system,
        )
    };

    let mut btn = button(button_content);
    let base_style_name = props.custom_style_name.as_deref();
    let alternate_style_name = props.active_style_name.as_deref();
    let use_alternate_style = should_use_alternate_style(widget, current_view_id, all_views);

    if props.button_on_press_enabled {
        btn = btn.on_press(Message::ButtonPressed(widget.id, current_view_id));
    }

    if props.button_on_press_with_enabled {
        let wid = widget.id;
        btn = btn.on_press_with(move || Message::ButtonPressed(wid, current_view_id));
    }

    if props.button_on_press_maybe_enabled {
        btn = btn.on_press_maybe(Some(Message::ButtonPressed(widget.id, current_view_id)));
    }

    btn = btn.style(move |theme: &Theme, status: button::Status| {
        let selected_style_name = if use_alternate_style {
            alternate_style_name.or(base_style_name)
        } else {
            base_style_name
        };

        if let Some(style_name) = selected_style_name {
            if let Some(style) = resolve_button_style(custom_themes, theme, status, style_name) {
                return style;
            }
        }

        button::primary(theme, status)
    });

    // Apply layout properties
    btn = btn.width(props.width);
    btn = btn.height(props.height);

    // Apply padding
    if props.padding_mode == PaddingMode::Uniform {
        btn = btn.padding(props.padding.top);
    } else {
        btn = btn.padding(props.padding);
    }

    // Apply clip
    if props.clip {
        btn = btn.clip(true);
    }

    btn.into()
}

fn build_text_preview<'a>(
    _hierarchy: &'a WidgetHierarchy,
    widget: &'a Widget,
    theme: &'a Theme,
    _custom_themes: &'a CustomThemes,
    _highlight_selected: bool,
    _all_views: &'a BTreeMap<Uuid, AppView>,
    _current_view_id: Option<Uuid>,
    _type_system: &'a TypeSystem,
) -> Element<'a, Message> {
    let props = &widget.properties;
    let mut t = text(&props.text_content)
        .width(props.width)
        .height(props.height)
        .size(props.text_size)
        .font(match props.font {
            FontType::Default => Font::default(),
            FontType::Monospace => Font::MONOSPACE,
        });

    let user_color = props.text_color; // Only set the color if a color has been set :D
    t = t.style(move |th: &Theme| {
        let c = if user_color.a == 0.0 {
            th.palette().text
        } else {
            user_color
        };
        text::Style { color: Some(c) }
    });
    t = t.line_height(props.line_height);
    t = t.wrapping(match props.wrap {
        text::Wrapping::None => text::Wrapping::None,
        text::Wrapping::Word => text::Wrapping::Word,
        text::Wrapping::Glyph => text::Wrapping::Glyph,
        text::Wrapping::WordOrGlyph => text::Wrapping::WordOrGlyph,
    });
    t = t.shaping(match props.shaping {
        text::Shaping::Basic => text::Shaping::Basic,
        text::Shaping::Advanced => text::Shaping::Advanced,
        text::Shaping::Auto => text::Shaping::Auto,
    });
    t = t.align_x(props.text_align_x).align_y(props.text_align_y);

    let _ = theme; // used via closure capture above
    t.into()
}

fn build_text_input_preview<'a>(
    _hierarchy: &'a WidgetHierarchy,
    widget: &'a Widget,
    _theme: &'a Theme,
    custom_themes: &'a CustomThemes,
    _highlight_selected: bool,
    _all_views: &'a BTreeMap<Uuid, AppView>,
    current_view_id: Option<Uuid>,
    _type_system: &'a TypeSystem,
) -> Element<'a, Message> {
    let props = &widget.properties;

    // Create text_input with placeholder and value
    let mut input = text_input(&props.text_input_placeholder, &props.text_input_value);

    // Always add on_input in preview (using Noop since it's just preview)
    input = input.on_input(move |text| Message::TextInputChanged(widget.id, text, current_view_id));

    // Conditionally add on_submit
    if props.text_input_on_submit {
        input = input.on_submit(Message::Submitted(widget.id, current_view_id));
    }

    // Conditionally add on_paste
    if props.text_input_on_paste {
        input = input.on_paste(move |text| Message::TextPasted(widget.id, text, current_view_id));
    }

    // Apply secure mode
    if props.is_secure {
        input = input.secure(true);
    }

    // Apply size (font size)
    input = input.size(props.text_input_size);

    // Apply internal padding
    input = input.padding(props.text_input_padding);

    // Apply layout properties
    input = input.width(props.width);

    // Apply font if not default
    if props.text_input_font != FontType::Default {
        input = input.font(props.text_input_font.into());
    }

    // Apply line height if specified
    if props.text_input_line_height != text::LineHeight::default() {
        input = input.line_height(props.text_input_line_height);
    }

    // Apply alignment
    if props.text_input_alignment != ContainerAlignX::Left {
        input = input.align_x(props.text_input_alignment);
    }

    // Apply icon if enabled
    if props.text_input_icon_enabled {
        let cp = char::from_u32(props.text_input_icon_codepoint).unwrap_or('\u{FFFD}');
        let size = if props.text_input_icon_size > 0.0 {
            Some(iced::Pixels(props.text_input_icon_size))
        } else {
            None
        };
        input = input.icon(text_input::Icon {
            font: Font::with_name("lucide"),
            code_point: cp,
            size,
            spacing: props.text_input_icon_spacing,
            side: props.text_input_icon_side.into(),
        });
    }

    if let Some(style_name) = &props.custom_style_name {
        if let Some(definition) =
            custom_style_definition(custom_themes, ThemePaneEnum::TextInput, style_name)
        {
            input = input
                .style(move |theme: &Theme, status| definition.to_text_input_style(theme, status));
        }
    }

    input.into()
}

fn build_checkbox_preview<'a>(
    _hierarchy: &'a WidgetHierarchy,
    widget: &'a Widget,
    _theme: &'a Theme,
    custom_themes: &'a CustomThemes,
    _highlight_selected: bool,
    all_views: &'a BTreeMap<Uuid, AppView>,
    current_view_id: Option<Uuid>,
    _type_system: &'a TypeSystem,
) -> Element<'a, Message> {
    let props = &widget.properties;
    let mut cb = checkbox(props.checkbox_checked)
        .label(&props.checkbox_label)
        .size(props.checkbox_size)
        .spacing(props.checkbox_spacing)
        .width(props.width)
        .on_toggle(move |_| {
            Message::CheckboxToggled(widget.id, !props.checkbox_checked, current_view_id)
        });

    if let Some(style_name) = selected_widget_style_name(widget, current_view_id, all_views) {
        let style_name = style_name.to_string();
        cb = cb.style(move |theme: &Theme, status| {
            resolve_checkbox_style(custom_themes, theme, status, &style_name)
                .unwrap_or_else(|| checkbox::primary(theme, status))
        });
    }

    cb.into()
}

fn build_radio_preview<'a>(
    _hierarchy: &'a WidgetHierarchy,
    widget: &'a Widget,
    _theme: &'a Theme,
    custom_themes: &'a CustomThemes,
    _highlight_selected: bool,
    all_views: &'a BTreeMap<Uuid, AppView>,
    current_view_id: Option<Uuid>,
    _type_system: &'a TypeSystem,
) -> Element<'a, Message> {
    let props = &widget.properties;
    if !props.radio_options.is_empty() {
        let style_name =
            selected_widget_style_name(widget, current_view_id, all_views).map(str::to_string);
        column(
            props
                .radio_options
                .iter()
                .enumerate()
                .map(|(i, option)| {
                    let mut radio_widget = radio(
                        option,
                        i,
                        Some(props.radio_selected_index),
                        move |selected_index| {
                            Message::RadioSelected(widget.id, selected_index, current_view_id)
                        },
                    );

                    radio_widget = radio_widget
                        .size(props.radio_size)
                        .spacing(props.radio_spacing);

                    if let Some(style_name) = style_name.clone() {
                        radio_widget = radio_widget.style(move |theme: &Theme, status| {
                            resolve_radio_style(custom_themes, theme, status, &style_name)
                                .unwrap_or_else(|| radio::default(theme, status))
                        });
                    }

                    radio_widget.into()
                })
                .collect::<Vec<Element<Message>>>(),
        )
        .into()
    } else {
        text("No radio options").into()
    }
}

fn build_slider_preview<'a>(
    _hierarchy: &'a WidgetHierarchy,
    widget: &'a Widget,
    _theme: &'a Theme,
    custom_themes: &'a CustomThemes,
    _highlight_selected: bool,
    all_views: &'a BTreeMap<Uuid, AppView>,
    current_view_id: Option<Uuid>,
    _type_system: &'a TypeSystem,
) -> Element<'a, Message> {
    let props = &widget.properties;
    let mut content = slider(
        props.slider_min..=props.slider_max,
        props.slider_value,
        move |value| Message::SliderChanged(widget.id, value, current_view_id),
    )
    .step(props.slider_step)
    .height(props.slider_height);

    if let Some(style_name) = selected_widget_style_name(widget, current_view_id, all_views) {
        let style_name = style_name.to_string();
        content = content.style(move |theme: &Theme, status| {
            resolve_slider_style(custom_themes, theme, status, &style_name)
                .unwrap_or_else(|| slider::default(theme, status))
        });
    }

    content.into()
}

fn build_vertical_slider_preview<'a>(
    _hierarchy: &'a WidgetHierarchy,
    widget: &'a Widget,
    _theme: &'a Theme,
    custom_themes: &'a CustomThemes,
    _highlight_selected: bool,
    all_views: &'a BTreeMap<Uuid, AppView>,
    current_view_id: Option<Uuid>,
    _type_system: &'a TypeSystem,
) -> Element<'a, Message> {
    let props = &widget.properties;
    let mut content = vertical_slider(
        props.slider_min..=props.slider_max,
        props.slider_value,
        move |value| Message::SliderChanged(widget.id, value, current_view_id),
    )
    .step(props.slider_step)
    .width(props.slider_width);

    if let Some(style_name) = selected_widget_style_name(widget, current_view_id, all_views) {
        let style_name = style_name.to_string();
        content = content.style(move |theme: &Theme, status| {
            resolve_slider_style(custom_themes, theme, status, &style_name)
                .unwrap_or_else(|| slider::default(theme, status))
        });
    }

    content.into()
}

fn build_progress_bar_preview<'a>(
    _hierarchy: &'a WidgetHierarchy,
    widget: &'a Widget,
    _theme: &'a Theme,
    custom_themes: &'a CustomThemes,
    _highlight_selected: bool,
    all_views: &'a BTreeMap<Uuid, AppView>,
    current_view_id: Option<Uuid>,
    _type_system: &'a TypeSystem,
) -> Element<'a, Message> {
    let props = &widget.properties;
    let mut content = progress_bar(
        props.progress_min..=props.progress_max,
        props.progress_value,
    )
    .length(props.progress_length)
    .girth(props.progress_girth);

    if props.progress_vertical {
        content = content.vertical();
    }

    if let Some(style_name) = selected_widget_style_name(widget, current_view_id, all_views) {
        let style_name = style_name.to_string();
        content = content.style(move |theme: &Theme| {
            resolve_progress_bar_style(custom_themes, theme, &style_name)
                .unwrap_or_else(|| progress_bar::primary(theme))
        });
    }

    content.into()
}

fn build_toggler_preview<'a>(
    _hierarchy: &'a WidgetHierarchy,
    widget: &'a Widget,
    _theme: &'a Theme,
    custom_themes: &'a CustomThemes,
    _highlight_selected: bool,
    all_views: &'a BTreeMap<Uuid, AppView>,
    current_view_id: Option<Uuid>,
    _type_system: &'a TypeSystem,
) -> Element<'a, Message> {
    let props = &widget.properties;
    let mut content = toggler(props.toggler_active)
        .on_toggle(move |_| {
            Message::TogglerToggled(widget.id, !props.toggler_active, current_view_id)
        })
        .label(&props.toggler_label)
        .size(props.toggler_size)
        .spacing(props.toggler_spacing)
        .width(props.width);

    if let Some(style_name) = selected_widget_style_name(widget, current_view_id, all_views) {
        let style_name = style_name.to_string();
        content = content.style(move |theme: &Theme, status| {
            resolve_toggler_style(custom_themes, theme, status, &style_name)
                .unwrap_or_else(|| toggler::default(theme, status))
        });
    }

    content.into()
}

fn build_pick_list_preview<'a>(
    _hierarchy: &'a WidgetHierarchy,
    widget: &'a Widget,
    _theme: &'a Theme,
    custom_themes: &'a CustomThemes,
    _highlight_selected: bool,
    all_views: &'a BTreeMap<Uuid, AppView>,
    current_view_id: Option<Uuid>,
    _type_system: &'a TypeSystem,
) -> Element<'a, Message> {
    let props = &widget.properties;
    let mut content = pick_list(
        props.picklist_options.clone(),
        props.picklist_selected.clone(),
        move |selected| Message::PickListSelected(widget.id, selected, current_view_id),
    )
    .placeholder(&props.picklist_placeholder)
    .width(props.width);

    if let Some(style_name) = selected_widget_style_name(widget, current_view_id, all_views) {
        let style_name = style_name.to_string();
        content = content.style(move |theme: &Theme, status| {
            resolve_pick_list_style(custom_themes, theme, status, &style_name)
                .unwrap_or_else(|| pick_list::default(theme, status))
        });
    }

    if let Some(style_name) = &props.menu_style_name {
        if let Some(definition) =
            custom_style_definition(custom_themes, ThemePaneEnum::Menu, style_name)
        {
            content = content.menu_style(move |theme: &Theme| definition.to_menu_style(theme));
        }
    }

    content.into()
}

fn build_scrollable_preview<'a>(
    hierarchy: &'a WidgetHierarchy,
    widget: &'a Widget,
    theme: &'a Theme,
    custom_themes: &'a CustomThemes,
    highlight_selected: bool,
    all_views: &'a BTreeMap<Uuid, AppView>,
    current_view_id: Option<Uuid>,
    type_system: &'a TypeSystem,
) -> Element<'a, Message> {
    let props = &widget.properties;
    let mut content = column![];

    if widget.children.is_empty() {
        for i in 1..=10 {
            content = content.push(text(format!("Scrollable Item {}", i)));
        }
    } else {
        for child in &widget.children {
            content = content.push(build_widget_preview(
                hierarchy,
                child,
                theme,
                custom_themes,
                highlight_selected,
                all_views,
                current_view_id,
                type_system,
            ));
        }
    }

    scrollable(content)
        .direction(props.scroll_dir)
        .anchor_x(props.anchor_x)
        .anchor_y(props.anchor_y)
        .width(props.width)
        .height(props.height)
        .into()
}

fn build_space_preview<'a>(
    _hierarchy: &'a WidgetHierarchy,
    widget: &'a Widget,
    _theme: &'a Theme,
    _custom_themes: &'a CustomThemes,
    _highlight_selected: bool,
    _all_views: &'a BTreeMap<Uuid, AppView>,
    _current_view_id: Option<Uuid>,
    _type_system: &'a TypeSystem,
) -> Element<'a, Message> {
    let props = &widget.properties;
    let s = match props.orientation {
        Orientation::Horizontal => space::horizontal().width(props.width).height(props.height),
        Orientation::Vertical => space::vertical().width(props.width).height(props.height),
    };

    if props.show_widget_bounds {
        container(s)
            .style(|_| container::Style {
                background: Some(Background::Color(Color::from_rgba(0.2, 0.6, 1.0, 0.18))),
                border: Border {
                    color: Color::from_rgb(0.2, 0.6, 1.0),
                    width: 1.0,
                    radius: 2.0.into(),
                },
                ..Default::default()
            })
            .into()
    } else {
        s.into()
    }
}

fn build_rule_preview<'a>(
    _hierarchy: &'a WidgetHierarchy,
    widget: &'a Widget,
    theme: &'a Theme,
    custom_themes: &'a CustomThemes,
    _highlight_selected: bool,
    _all_views: &'a BTreeMap<Uuid, AppView>,
    _current_view_id: Option<Uuid>,
    _type_system: &'a TypeSystem,
) -> Element<'a, Message> {
    let props = &widget.properties;
    let mut r = match props.orientation {
        Orientation::Horizontal => rule::horizontal(props.rule_thickness),
        Orientation::Vertical => rule::vertical(props.rule_thickness),
    };
    if let Some(style_name) = &props.custom_style_name {
        if let Some(style_map) = custom_themes.styles().get(&ThemePaneEnum::Rule) {
            if let Some(style_definition) = style_map.get(style_name) {
                let style = style_definition.to_rule_style(theme);
                r = r.style(move |_theme| style);
            }
        }
    }
    r.into()
}

fn build_image_preview<'a>(
    _hierarchy: &'a WidgetHierarchy,
    widget: &'a Widget,
    _theme: &'a Theme,
    _custom_themes: &'a CustomThemes,
    _highlight_selected: bool,
    _all_views: &'a BTreeMap<Uuid, AppView>,
    _current_view_id: Option<Uuid>,
    _type_system: &'a TypeSystem,
) -> Element<'a, Message> {
    let props = &widget.properties;
    let el: Element<_> = if props.image_path.trim().is_empty() {
        // Placeholder box when no path provided
        container(text("🖼️ Image (no path)"))
            .width(props.width)
            .height(props.height)
            .style(|_| container::Style {
                border: Border {
                    color: Color::from_rgb(0.6, 0.6, 0.6),
                    width: 1.0,
                    radius: 4.0.into(),
                },
                background: Some(Background::Color(Color::from_rgba(0.5, 0.5, 0.5, 0.05))),
                ..Default::default()
            })
            .into()
    } else {
        image(image::Handle::from_path(&props.image_path))
            .content_fit(props.image_fit.into())
            .width(props.width)
            .height(props.height)
            .into()
    };
    el
}

fn build_svg_preview<'a>(
    _hierarchy: &'a WidgetHierarchy,
    widget: &'a Widget,
    _theme: &'a Theme,
    _custom_themes: &'a CustomThemes,
    _highlight_selected: bool,
    _all_views: &'a BTreeMap<Uuid, AppView>,
    _current_view_id: Option<Uuid>,
    _type_system: &'a TypeSystem,
) -> Element<'a, Message> {
    let props = &widget.properties;
    let el: Element<_> = if props.svg_path.trim().is_empty() {
        container(text("🧩 SVG (no path)"))
            .width(props.width)
            .height(props.height)
            .style(|_| container::Style {
                border: Border {
                    color: Color::from_rgb(0.6, 0.6, 0.6),
                    width: 1.0,
                    radius: 4.0.into(),
                },
                background: Some(Background::Color(Color::from_rgba(0.5, 0.5, 0.5, 0.05))),
                ..Default::default()
            })
            .into()
    } else {
        svg(svg::Handle::from_path(&props.svg_path))
            .content_fit(props.svg_fit.into())
            .width(props.width)
            .height(props.height)
            .into()
    };
    el
}

fn build_tooltip_preview<'a>(
    hierarchy: &'a WidgetHierarchy,
    widget: &'a Widget,
    theme: &'a Theme,
    custom_themes: &'a CustomThemes,
    highlight_selected: bool,
    all_views: &'a BTreeMap<Uuid, AppView>,
    current_view_id: Option<Uuid>,
    type_system: &'a TypeSystem,
) -> Element<'a, Message> {
    let props = &widget.properties;
    // child[0] = trigger (host), child[1] = popup content
    let host = {
        let element = widget
            .children
            .get(0)
            .map(|widget| {
                build_widget_preview(
                    hierarchy,
                    widget,
                    theme,
                    custom_themes,
                    highlight_selected,
                    all_views,
                    current_view_id,
                    type_system,
                )
            })
            .unwrap_or_else(|| text("Tooltip host").into());

        container(element)
            .padding(6)
            .style(|th: &Theme| container::Style {
                border: Border {
                    color: th.extended_palette().primary.strong.color,
                    width: 1.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            })
    };

    let popup = {
        let element = widget
            .children
            .get(1)
            .map(|widget| {
                build_widget_preview(
                    hierarchy,
                    widget,
                    theme,
                    custom_themes,
                    highlight_selected,
                    all_views,
                    current_view_id,
                    type_system,
                )
            })
            .unwrap_or_else(|| text(&props.tooltip_text).size(14).into());

        container(element)
            .padding(6)
            .style(|th: &Theme| container::Style {
                background: Some(Background::Color(
                    th.extended_palette().background.weak.color,
                )),
                border: Border {
                    color: th.extended_palette().background.strong.color,
                    width: 1.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            })
    };

    tooltip(host, popup, props.tooltip_position.into())
        .gap(6)
        .padding(8)
        .into()
}

fn build_combo_box_preview<'a>(
    _hierarchy: &'a WidgetHierarchy,
    widget: &'a Widget,
    _theme: &'a Theme,
    custom_themes: &'a CustomThemes,
    _highlight_selected: bool,
    _all_views: &'a BTreeMap<Uuid, AppView>,
    current_view_id: Option<Uuid>,
    _type_system: &'a TypeSystem,
) -> Element<'a, Message> {
    let props = &widget.properties;
    let id = widget.id;
    let on_selected = move |selected| {
        Message::PropertyChanged(
            id,
            PropertyChange::ComboBoxSelected(Some(selected)),
            current_view_id,
        )
    };

    let mut cb = combo_box(
        &props.combobox_state,
        &props.combobox_placeholder,
        props.combobox_selected.as_ref(),
        on_selected,
    )
    .on_close(Message::ComboBoxOnClose(id, current_view_id))
    .on_input(move |search| Message::ComboBoxOnInput(id, search, current_view_id))
    .on_open(Message::ComboBoxOnOpen(id, current_view_id))
    .on_option_hovered(move |hovered| {
        Message::ComboBoxOnOptionHovered(id, hovered, current_view_id)
    })
    .width(props.width);

    let mut uses_split_styles = false;

    if let Some(style_name) = props.custom_style_name.as_deref() {
        if let Some(definition) =
            custom_style_definition(custom_themes, ThemePaneEnum::TextInput, style_name)
        {
            cb = cb.input_style(move |theme: &Theme, status| {
                definition.to_text_input_style(theme, status)
            });
            uses_split_styles = true;
        }
    }

    if let Some(style_name) = props.menu_style_name.as_deref() {
        if let Some(definition) =
            custom_style_definition(custom_themes, ThemePaneEnum::Menu, style_name)
        {
            cb = cb.menu_style(move |theme: &Theme| definition.to_menu_style(theme));
            uses_split_styles = true;
        }
    }

    if !uses_split_styles {
        if let Some(style_name) = props.custom_style_name.as_deref() {
            if let Some(definition) =
                custom_style_definition(custom_themes, ThemePaneEnum::Combobox, style_name)
            {
                let def_input = definition.clone();
                cb = cb.input_style(move |theme: &Theme, status| {
                    def_input.to_combo_box_input_style(theme, status)
                });
                let def_menu = definition.clone();
                cb = cb.menu_style(move |theme: &Theme| def_menu.to_combo_box_menu_style(theme));
            }
        }
    }

    // Apply icon if enabled
    if props.combobox_icon_enabled {
        let cp = char::from_u32(props.combobox_icon_codepoint).unwrap_or('\u{FFFD}');
        let size = if props.combobox_icon_size > 0.0 {
            Some(iced::Pixels(props.combobox_icon_size))
        } else {
            None
        };
        cb = cb.icon(text_input::Icon {
            font: Font::with_name("lucide"),
            code_point: cp,
            size,
            spacing: props.combobox_icon_spacing,
            side: props.combobox_icon_side.into(),
        });
    }

    cb.into()
}

fn build_markdown_preview<'a>(
    _hierarchy: &'a WidgetHierarchy,
    widget: &'a Widget,
    theme: &'a Theme,
    _custom_themes: &'a CustomThemes,
    _highlight_selected: bool,
    _all_views: &'a BTreeMap<Uuid, AppView>,
    _current_view_id: Option<Uuid>,
    _type_system: &'a TypeSystem,
) -> Element<'a, Message> {
    let props = &widget.properties;
    markdown::view(
        &props.markdown_content,
        markdown::Settings::with_text_size(props.markdown_text_size, theme.clone()),
    )
    .map(|_| Message::Noop)
}

fn build_mouse_area_preview<'a>(
    hierarchy: &'a WidgetHierarchy,
    widget: &'a Widget,
    theme: &'a Theme,
    custom_themes: &'a CustomThemes,
    highlight_selected: bool,
    all_views: &'a BTreeMap<Uuid, AppView>,
    current_view_id: Option<Uuid>,
    type_system: &'a TypeSystem,
) -> Element<'a, Message> {
    let props = &widget.properties;

    // Build the child content
    let content = if widget.children.is_empty() {
        // Show placeholder when no child exists
        container(text("Mouse Area Content"))
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_| container::Style {
                border: Border {
                    color: Color::from_rgba(0.5, 0.5, 0.5, 0.3),
                    width: 1.0,
                    radius: 4.0.into(),
                },
                background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.05))),
                ..Default::default()
            })
            .into()
    } else {
        build_widget_preview(
            hierarchy,
            &widget.children[0],
            theme,
            custom_themes,
            highlight_selected,
            all_views,
            current_view_id,
            type_system,
        )
    };

    // Start building mouse_area with conditional handlers
    let mut area = mouse_area(content);

    // Conditionally add event handlers based on properties
    if props.mousearea_on_press {
        area = area.on_press(Message::Noop);
    }

    if props.mousearea_on_release {
        area = area.on_release(Message::Noop);
    }

    if props.mousearea_on_double_click {
        area = area.on_double_click(Message::Noop);
    }

    if props.mousearea_on_right_press {
        area = area.on_right_press(Message::Noop);
    }

    if props.mousearea_on_right_release {
        area = area.on_right_release(Message::Noop);
    }

    if props.mousearea_on_middle_press {
        area = area.on_middle_press(Message::Noop);
    }

    if props.mousearea_on_middle_release {
        area = area.on_middle_release(Message::Noop);
    }

    if props.mousearea_on_scroll {
        area = area.on_scroll(|_delta| Message::Noop);
    }

    if props.mousearea_on_enter {
        area = area.on_enter(Message::Noop);
    }

    if props.mousearea_on_move {
        area = area.on_move(|_point| Message::Noop);
    }

    if props.mousearea_on_exit {
        area = area.on_exit(Message::Noop);
    }

    // Set mouse interaction if specified
    if let Some(interaction) = props.mousearea_interaction {
        area = area.interaction(interaction.into());
    }

    // Apply common layout properties
    let mut element: Element<_> = area.into();

    if props.width != Length::Shrink {
        element = container(element).width(props.width).into();
    }
    if props.height != Length::Shrink {
        element = container(element).height(props.height).into();
    }

    element
}

fn build_qr_code_preview<'a>(
    _hierarchy: &'a WidgetHierarchy,
    widget: &'a Widget,
    _theme: &'a Theme,
    _custom_themes: &'a CustomThemes,
    _highlight_selected: bool,
    _all_views: &'a BTreeMap<Uuid, AppView>,
    _current_view_id: Option<Uuid>,
    _type_system: &'a TypeSystem,
) -> Element<'a, Message> {
    let props = &widget.properties;
    use iced::widget::qr_code;

    match &props.qrcode_data {
        Some(data) => qr_code::QRCode::new(data)
            .cell_size(props.qrcode_cell_size)
            .into(),
        _ => text("Invalid QR data").into(),
    }
}

fn build_stack_preview<'a>(
    hierarchy: &'a WidgetHierarchy,
    widget: &'a Widget,
    theme: &'a Theme,
    custom_themes: &'a CustomThemes,
    highlight_selected: bool,
    all_views: &'a BTreeMap<Uuid, AppView>,
    current_view_id: Option<Uuid>,
    type_system: &'a TypeSystem,
) -> Element<'a, Message> {
    let props = &widget.properties;
    let mut layers = Vec::new();

    if widget.children.is_empty() {
        layers.push(
            container(text("Stack Layer 1"))
                .width(Length::Fill)
                .height(Length::Fill)
                .center(Length::Fill)
                .into(),
        );
        layers.push(
            container(text("Stack Layer 2").color(Color::from_rgb(1.0, 0.0, 0.0)))
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(20)
                .into(),
        );
    } else {
        for child in &widget.children {
            layers.push(build_widget_preview(
                hierarchy,
                child,
                theme,
                custom_themes,
                highlight_selected,
                all_views,
                current_view_id,
                type_system,
            ));
        }
    }

    // Stack defaults to Shrink in iced — only set width/height when NOT Fill,
    // matching codegen which skips Fill so the exported Stack is Shrink.
    let s = stack(layers);
    let s = if !matches!(props.width, Length::Fill) {
        s.width(props.width)
    } else {
        s
    };
    let s = if !matches!(props.height, Length::Fill) {
        s.height(props.height)
    } else {
        s
    };
    s.into()
}

fn build_collapsible_preview<'a>(
    hierarchy: &'a WidgetHierarchy,
    widget: &'a Widget,
    theme: &'a Theme,
    custom_themes: &'a CustomThemes,
    highlight_selected: bool,
    all_views: &'a BTreeMap<Uuid, AppView>,
    current_view_id: Option<Uuid>,
    type_system: &'a TypeSystem,
) -> Element<'a, Message> {
    let props = &widget.properties;
    let child_content: Element<'a, Message> = if let Some(child) = widget.children.get(0) {
        build_widget_preview(
            hierarchy,
            child,
            theme,
            custom_themes,
            highlight_selected,
            all_views,
            current_view_id,
            type_system,
        )
    } else {
        space::horizontal().height(Length::Shrink).into()
    };

    let mut collapsible = collapsible_widget::collapsible(&props.collapsible_title, child_content)
        .width(props.width)
        .height(props.height)
        .header_height(props.collapsible_header_height)
        .title_alignment(match props.align_x {
            ContainerAlignX::Left => Alignment::Start,
            ContainerAlignX::Center => Alignment::Center,
            ContainerAlignX::Right => Alignment::End,
        })
        .header_clickable(props.collapsible_header_clickable)
        .padding(props.padding)
        .expanded(props.collapsible_expanded)
        .text_size(props.text_size);

    if props.font != FontType::Default {
        collapsible = collapsible.font(Font::MONOSPACE);
    }

    if let Some(style_name) = selected_widget_style_name(widget, current_view_id, all_views) {
        let style_name = style_name.to_string();
        collapsible = collapsible.style(move |theme: &Theme, status| {
            resolve_collapsible_style(theme, status, &style_name)
                .unwrap_or_else(|| collapsible_widget::default(theme, status))
        });
    }

    collapsible.into()
}

fn build_collapsible_group_preview<'a>(
    hierarchy: &'a WidgetHierarchy,
    widget: &'a Widget,
    theme: &'a Theme,
    custom_themes: &'a CustomThemes,
    highlight_selected: bool,
    all_views: &'a BTreeMap<Uuid, AppView>,
    current_view_id: Option<Uuid>,
    type_system: &'a TypeSystem,
) -> Element<'a, Message> {
    let props = &widget.properties;
    let items: Vec<Element<'a, Message>> = widget
        .children
        .iter()
        .map(|child| {
            build_widget_preview(
                hierarchy,
                child,
                theme,
                custom_themes,
                highlight_selected,
                all_views,
                current_view_id,
                type_system,
            )
        })
        .collect();

    CollapsibleGroup::new(items)
        .width(props.width)
        .height(props.height)
        .spacing(props.spacing)
        .into()
}

fn build_date_picker_preview<'a>(
    _hierarchy: &'a WidgetHierarchy,
    widget: &'a Widget,
    _theme: &'a Theme,
    custom_themes: &'a CustomThemes,
    _highlight_selected: bool,
    all_views: &'a BTreeMap<Uuid, AppView>,
    current_view_id: Option<Uuid>,
    _type_system: &'a TypeSystem,
) -> Element<'a, Message> {
    let props = &widget.properties;
    let selection = date_picker_selection(widget);
    let time = date_picker_time(widget);
    let label = format_date_picker_button_label(
        &selection,
        time,
        &props.text_content,
        props.date_picker_show_time,
    );
    let base_style_name = props.custom_style_name.as_deref();
    let alternate_style_name = props.active_style_name.as_deref();
    let use_alternate_style = should_use_alternate_style(widget, current_view_id, all_views);

    let mut trigger =
        button(text(label)).on_press(Message::DatePickerOpenRequested(widget.id, current_view_id));

    trigger = trigger.style(move |theme: &Theme, status: button::Status| {
        let selected_style_name = if use_alternate_style {
            alternate_style_name.or(base_style_name)
        } else {
            base_style_name
        };

        if let Some(style_name) = selected_style_name {
            if let Some(style) = resolve_button_style(custom_themes, theme, status, style_name) {
                return style;
            }
        }

        button::primary(theme, status)
    });

    trigger = trigger.width(props.width);
    trigger = trigger.height(props.height);

    if props.padding_mode == PaddingMode::Uniform {
        trigger = trigger.padding(props.padding.top);
    } else {
        trigger = trigger.padding(props.padding);
    }

    if props.clip {
        trigger = trigger.clip(true);
    }

    let is_open = date_picker_is_open(current_view_id, all_views, widget.id);
    let picker: Element<'a, Message> = if props.date_picker_show_time {
        date_picker_widget::date_picker(is_open, selection)
            .show_time()
            .initial_time(time)
            .on_change_with_time(move |selection, time| {
                Message::DatePickerChangedWithTime(widget.id, selection, time, current_view_id)
            })
            .on_close(move || Message::DatePickerClosed(widget.id, current_view_id))
            .into()
    } else {
        date_picker_widget::date_picker(is_open, selection)
            .on_change(move |selection| {
                Message::DatePickerChanged(widget.id, selection, current_view_id)
            })
            .on_close(move || Message::DatePickerClosed(widget.id, current_view_id))
            .into()
    };

    stack![trigger, picker].into()
}

fn build_generic_overlay_preview<'a>(
    hierarchy: &'a WidgetHierarchy,
    widget: &'a Widget,
    theme: &'a Theme,
    custom_themes: &'a CustomThemes,
    highlight_selected: bool,
    all_views: &'a BTreeMap<Uuid, AppView>,
    current_view_id: Option<Uuid>,
    type_system: &'a TypeSystem,
) -> Element<'a, Message> {
    let props = &widget.properties;
    let has_trigger_child = widget.children.get(0).is_some();
    let trigger_content: Element<'a, Message> = if let Some(child) = widget.children.get(0) {
        build_widget_preview(
            hierarchy,
            child,
            theme,
            custom_themes,
            highlight_selected,
            all_views,
            current_view_id,
            type_system,
        )
    } else {
        text(&props.text_content).into()
    };

    let overlay_content: Element<'a, Message> = if let Some(child) = widget.children.get(1) {
        build_widget_preview(
            hierarchy,
            child,
            theme,
            custom_themes,
            highlight_selected,
            all_views,
            current_view_id,
            type_system,
        )
    } else {
        text("Overlay content").into()
    };

    let uses_hover_placement =
        props.generic_overlay_on_hover || props.generic_overlay_hover_positions_on_click;
    let is_open = generic_overlay_is_open(current_view_id, all_views, widget.id);
    let mut overlay = generic_overlay::overlay_button(
        trigger_content,
        &props.generic_overlay_title,
        overlay_content,
    )
    .button_clip(props.clip)
    .overlay_padding(props.generic_overlay_overlay_padding)
    .overlay_radius(props.generic_overlay_overlay_radius)
    .opaque(props.generic_overlay_opaque)
    .opaque_alpha(props.generic_overlay_opaque_alpha)
    .interactive_base(true)
    .is_open(is_open)
    .on_toggle(move |value| Message::GenericOverlayToggled(widget.id, value, current_view_id));

    if has_trigger_child {
        overlay = overlay
            .width(Length::Shrink)
            .height(Length::Shrink)
            .padding(0.0)
            .style(button::text);
    } else {
        overlay = overlay
            .width(props.width)
            .height(props.height)
            .padding(props.padding);
    }

    if props.generic_overlay_overlay_width_dynamic {
        let factor = props.generic_overlay_overlay_width_dynamic_factor;
        overlay = overlay.overlay_width_dynamic(move |available| Length::Fixed(available * factor));
    } else {
        overlay = overlay.overlay_width(props.generic_overlay_overlay_width);
    }

    if props.generic_overlay_overlay_height_dynamic {
        let factor = props.generic_overlay_overlay_height_dynamic_factor;
        overlay =
            overlay.overlay_height_dynamic(move |available| Length::Fixed(available * factor));
    } else {
        overlay = overlay.overlay_height(props.generic_overlay_overlay_height);
    }

    if let Some(ref id) = props.widget_id {
        if !id.is_empty() {
            overlay = overlay.id(id.clone());
        }
    }

    if props.generic_overlay_on_hover {
        overlay = overlay.on_hover();
    }

    if props.generic_overlay_hover_positions_on_click {
        overlay = overlay.hover_positions_on_click();
    }

    if uses_hover_placement {
        overlay = overlay
            .hover_position(props.generic_overlay_hover_position.into())
            .hover_gap(props.generic_overlay_hover_gap)
            .hover_alignment(props.generic_overlay_hover_alignment.into())
            .hover_mode(props.generic_overlay_hover_mode.into())
            .hover_snap(props.generic_overlay_hover_snap)
            .safe_triangle(props.generic_overlay_safe_triangle);
    }

    if props.generic_overlay_close_on_click_outside {
        overlay = overlay.close_on_click_outside();
    }

    if props.generic_overlay_hide_header {
        overlay = overlay.hide_header();
    }

    if props.generic_overlay_hide_close_button {
        overlay = overlay.hide_close_button();
    }

    if props.generic_overlay_block_dragging {
        overlay = overlay.block_dragging();
    }

    if props.generic_overlay_resizable != GenericOverlayResizeMode::None {
        overlay = overlay.resizable(props.generic_overlay_resizable.into());
    }

    if props.generic_overlay_reset_on_close {
        overlay = overlay.reset_on_close();
    }

    if props.generic_overlay_animate {
        overlay = match props.generic_overlay_animation_preset {
            GenericOverlayAnimationPreset::Default => overlay.animate(true),
            GenericOverlayAnimationPreset::Quick => overlay.quick_animation(),
            GenericOverlayAnimationPreset::Slow => overlay.slow_animation(),
        };
    }

    if !has_trigger_child
        && let Some(style_name) = selected_widget_style_name(widget, current_view_id, all_views)
    {
        let style_name = style_name.to_string();
        overlay = overlay.style(move |theme: &Theme, status| {
            resolve_button_style(custom_themes, theme, status, &style_name)
                .unwrap_or_else(|| button::primary(theme, status))
        });
    }

    if let Some(style_name) = props.generic_overlay_overlay_style_name.as_deref() {
        if resolve_generic_overlay_style(theme, style_name).is_some() {
            let style_name = style_name.to_string();
            overlay = overlay.overlay_style(move |theme: &Theme| {
                resolve_generic_overlay_style(theme, &style_name)
                    .unwrap_or_else(|| generic_overlay::blank(theme))
            });
        }
    }

    overlay.into()
}

fn build_grid_preview<'a>(
    hierarchy: &'a WidgetHierarchy,
    widget: &'a Widget,
    theme: &'a Theme,
    custom_themes: &'a CustomThemes,
    highlight_selected: bool,
    all_views: &'a BTreeMap<Uuid, AppView>,
    current_view_id: Option<Uuid>,
    type_system: &'a TypeSystem,
) -> Element<'a, Message> {
    let props = &widget.properties;
    let children: Vec<Element<'a, Message>> = if widget.children.is_empty() {
        (1..=6)
            .map(|i| {
                container(text(format!("Cell {i}")))
                    .center(Length::Fill)
                    .style(|th: &Theme| container::Style {
                        border: Border {
                            color: th.extended_palette().background.strong.color,
                            width: 1.0,
                            radius: 2.0.into(),
                        },
                        ..Default::default()
                    })
                    .into()
            })
            .collect()
    } else {
        widget
            .children
            .iter()
            .map(|child| {
                build_widget_preview(
                    hierarchy,
                    child,
                    theme,
                    custom_themes,
                    highlight_selected,
                    all_views,
                    current_view_id,
                    type_system,
                )
            })
            .collect()
    };

    let g = grid(children);
    let g = if props.grid_use_fluid {
        g.fluid(props.grid_fluid_max_width)
    } else {
        g.columns(props.grid_columns)
    };
    let g = g.spacing(props.grid_spacing);
    let g = if let Some(w) = props.grid_fixed_width {
        g.width(w)
    } else {
        g
    };
    g.into()
}

fn build_themer_preview<'a>(
    hierarchy: &'a WidgetHierarchy,
    widget: &'a Widget,
    theme: &'a Theme,
    custom_themes: &'a CustomThemes,
    highlight_selected: bool,
    all_views: &'a BTreeMap<Uuid, AppView>,
    current_view_id: Option<Uuid>,
    type_system: &'a TypeSystem,
) -> Element<'a, Message> {
    let props = &widget.properties;
    let content = if widget.children.is_empty() {
        container(text("Themed Content")).padding(10).into()
    } else {
        let mut col = column![];
        for child in &widget.children {
            col = col.push(build_widget_preview(
                hierarchy,
                child,
                theme,
                custom_themes,
                highlight_selected,
                all_views,
                current_view_id,
                type_system,
            ));
        }
        col.into()
    };

    if let Some(theme) = &props.themer_theme {
        themer(Some(theme.clone()), content).into()
    } else {
        content
    }
}

fn build_pin_preview<'a>(
    hierarchy: &'a WidgetHierarchy,
    widget: &'a Widget,
    theme: &'a Theme,
    custom_themes: &'a CustomThemes,
    highlight_selected: bool,
    all_views: &'a BTreeMap<Uuid, AppView>,
    current_view_id: Option<Uuid>,
    type_system: &'a TypeSystem,
) -> Element<'a, Message> {
    let props = &widget.properties;
    let content = if widget.children.is_empty() {
        text("Pinned Content").into()
    } else {
        build_widget_preview(
            hierarchy,
            &widget.children[0],
            theme,
            custom_themes,
            highlight_selected,
            all_views,
            current_view_id,
            type_system,
        )
    };

    let mut p = Pin::new(content).width(props.width).height(props.height);

    if props.pin_point.x != 0.0 || props.pin_point.y != 0.0 {
        p = p.position(props.pin_point);
    }

    p.into()
}

fn build_table_preview<'a>(
    _hierarchy: &'a WidgetHierarchy,
    widget: &'a Widget,
    theme: &'a Theme,
    _custom_themes: &'a CustomThemes,
    _highlight_selected: bool,
    _all_views: &'a BTreeMap<Uuid, AppView>,
    _current_view_id: Option<Uuid>,
    type_system: &'a TypeSystem,
) -> Element<'a, Message> {
    let props = &widget.properties;
    let border_color = theme.extended_palette().background.strong.color;

    if let Some(struct_id) = props.table_referenced_struct {
        if let Some(struct_def) = type_system.get_struct(struct_id) {
            let fields = &struct_def.fields;
            let bold_headers = props.table_bold_headers;

            // Pre-build per-column sample values so each closure can
            // capture owned data — this avoids lifetime issues with
            // local row data vs the 'a lifetime on the returned Element.
            let sample_data: Vec<Vec<String>> = (0..3)
                .map(|row_idx| {
                    fields
                        .iter()
                        .map(|f| sample_value_for_type(&f.field_type, type_system, row_idx))
                        .collect()
                })
                .collect();

            let bold = |header: String| {
                text(header).font(Font {
                    weight: font::Weight::Bold,
                    ..Font::DEFAULT
                })
            };

            // T = usize (row index). Each column closure captures its
            // values by move — no references to locals escape.
            let columns: Vec<_> = fields
                .iter()
                .enumerate()
                .map(|(col_idx, field)| {
                    let header_text = field.name.clone();
                    let header = if bold_headers {
                        bold(header_text)
                    } else {
                        text(header_text)
                    };
                    let col_values: Vec<String> = sample_data
                        .iter()
                        .map(|row| row.get(col_idx).cloned().unwrap_or_default())
                        .collect();
                    table::column(header, move |row_idx: usize| {
                        text(col_values.get(row_idx).cloned().unwrap_or_default())
                    })
                })
                .collect();

            let tbl = table(columns, 0..3_usize)
                .padding_x(props.table_padding_x)
                .padding_y(props.table_padding_y)
                .separator_x(props.table_separator_x)
                .separator_y(props.table_separator_y)
                .width(props.width);

            container(tbl)
                .width(props.width)
                .height(props.height)
                .into()
        } else {
            // Struct was deleted
            container(
                text("Table: struct not found")
                    .size(14)
                    .color(Color::from_rgb(0.8, 0.2, 0.2)),
            )
            .width(props.width)
            .height(props.height)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(move |_theme: &Theme| container::Style {
                border: Border {
                    color: border_color,
                    width: 1.0,
                    radius: 2.0.into(),
                },
                ..Default::default()
            })
            .into()
        }
    } else {
        container(
            text("Table: No struct selected")
                .size(14)
                .color(Color::from_rgb(0.5, 0.5, 0.5)),
        )
        .width(props.width)
        .height(props.height)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .style(move |_theme: &Theme| container::Style {
            border: Border {
                color: border_color,
                width: 1.0,
                radius: 2.0.into(),
            },
            ..Default::default()
        })
        .into()
    }
}

fn build_icon_preview<'a>(
    _hierarchy: &'a WidgetHierarchy,
    widget: &'a Widget,
    _theme: &'a Theme,
    _custom_themes: &'a CustomThemes,
    _highlight_selected: bool,
    _all_views: &'a BTreeMap<Uuid, AppView>,
    _current_view_id: Option<Uuid>,
    _type_system: &'a TypeSystem,
) -> Element<'a, Message> {
    let props = &widget.properties;
    let codepoint_char = char::from_u32(props.icon_codepoint).unwrap_or('\u{FFFD}');
    text(codepoint_char.to_string())
        .font(Font::with_name("lucide"))
        .size(props.icon_size)
        .width(props.width)
        .height(props.height)
        .into()
}

fn build_view_reference_preview<'a>(
    hierarchy: &'a WidgetHierarchy,
    widget: &'a Widget,
    theme: &'a Theme,
    custom_themes: &'a CustomThemes,
    highlight_selected: bool,
    all_views: &'a BTreeMap<Uuid, AppView>,
    current_view_id: Option<Uuid>,
    type_system: &'a TypeSystem,
) -> Element<'a, Message> {
    match resolve_view_reference_target_view_id(widget, current_view_id, all_views) {
        Some(view_id) => {
            match all_views.get(&view_id) {
                Some(referenced_view) => build_widget_preview(
                    &referenced_view.hierarchy,
                    referenced_view.hierarchy.root(),
                    theme,
                    custom_themes,
                    highlight_selected,
                    all_views,
                    Some(view_id),
                    type_system,
                ),
                None => {
                    // View doesn't exist (need to protect used views from being deleted)
                    container(
                        text(format!("View not found: {}", view_id))
                            .color(Color::from_rgb(1.0, 0.0, 0.0)),
                    )
                    .padding(10)
                    .style(|_| container::Style {
                        border: Border {
                            color: Color::from_rgb(1.0, 0.0, 0.0),
                            width: 2.0,
                            radius: 4.0.into(),
                        },
                        background: Some(Background::Color(Color::from_rgba(1.0, 0.0, 0.0, 0.1))),
                        ..Default::default()
                    })
                    .into()
                }
            }
        }
        None => {
            // No view selected yet
            container(text("Select a view to reference").color(Color::from_rgb(0.5, 0.5, 0.5)))
                .padding(20)
                .style(|_| container::Style {
                    border: Border {
                        color: Color::from_rgba(0.5, 0.5, 0.5, 0.5),
                        width: 1.0,
                        radius: 4.0.into(),
                    },
                    background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.05))),
                    ..Default::default()
                })
                .into()
        }
    }
}

fn resolve_view_reference_target_view_id(
    widget: &Widget,
    current_view_id: Option<Uuid>,
    all_views: &BTreeMap<Uuid, AppView>,
) -> Option<Uuid> {
    let primary_view_id = widget.properties.referenced_view_id?;
    if widget.properties.extra_view_ids.is_empty() {
        return Some(primary_view_id);
    }

    let Some(owner_view_id) = current_view_id else {
        return Some(primary_view_id);
    };
    let Some(owner_view) = all_views.get(&owner_view_id) else {
        return Some(primary_view_id);
    };
    let Some(primary_view) = all_views.get(&primary_view_id) else {
        return Some(primary_view_id);
    };

    let candidates =
        std::iter::once(primary_view_id).chain(widget.properties.extra_view_ids.iter().copied());
    let candidate_set: std::collections::HashSet<Uuid> = candidates.clone().collect();

    // Canonical path: owner + widget identity -> selected target view id.
    let canonical_key = view_reference_selection_state_key(owner_view_id, widget.id);
    if let Some(ActionValue::String(raw_id)) = owner_view.custom_state_values.get(&canonical_key) {
        if let Ok(target_view_id) = Uuid::parse_str(raw_id) {
            if candidate_set.contains(&target_view_id) {
                return Some(target_view_id);
            }
        }
    }

    // Legacy fallback: name-derived selection enum variant.
    let base_name = if !widget.properties.widget_name.trim().is_empty() {
        widget
            .properties
            .widget_name
            .trim()
            .to_lowercase()
            .replace(' ', "_")
    } else {
        primary_view.name.trim().to_lowercase().replace(' ', "_")
    };
    let field_name = format!("{}_selection", base_name);
    let selection_key = view_selection_state_key(owner_view_id, &field_name);
    let selected_variant = match owner_view.custom_state_values.get(&selection_key) {
        Some(ActionValue::EnumVariant { variant, .. }) => Some(variant.as_str()),
        Some(ActionValue::String(value)) => Some(value.as_str()),
        _ => None,
    };
    let Some(selected_variant) = selected_variant else {
        return Some(primary_view_id);
    };

    for candidate_id in candidates {
        let Some(candidate_view) = all_views.get(&candidate_id) else {
            continue;
        };
        if view_name_to_selection_variant(&candidate_view.name) == selected_variant {
            return Some(candidate_id);
        }
    }

    Some(primary_view_id)
}

fn view_name_to_selection_variant(view_name: &str) -> String {
    view_name
        .trim()
        .to_lowercase()
        .replace(' ', "_")
        .split('_')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect()
}

/// Generate a sample value string for a given FieldType, for table preview
fn sample_value_for_type(
    field_type: &crate::enum_builder::FieldType,
    type_system: &TypeSystem,
    row_idx: usize,
) -> String {
    use crate::enum_builder::FieldType;
    match field_type {
        FieldType::String => {
            let samples = ["Hello", "World", "Sample"];
            samples[row_idx % samples.len()].to_string()
        }
        FieldType::F32 | FieldType::F64 => {
            let samples = [1.5, 3.14, 42.0];
            format!("{}", samples[row_idx % samples.len()])
        }
        FieldType::I32 | FieldType::I64 => {
            let samples = [1, -7, 42];
            format!("{}", samples[row_idx % samples.len()])
        }
        FieldType::U32 | FieldType::U64 | FieldType::Usize => {
            let samples = [0, 5, 100];
            format!("{}", samples[row_idx % samples.len()])
        }
        FieldType::Bool => {
            if row_idx % 2 == 0 {
                "true".to_string()
            } else {
                "false".to_string()
            }
        }
        FieldType::CustomEnum(id) => type_system
            .get_enum(*id)
            .and_then(|e| e.variants.get(row_idx % e.variants.len()))
            .map(|v| v.name.clone())
            .unwrap_or_else(|| "?".to_string()),
        FieldType::CustomStruct(id) => type_system
            .get_struct(*id)
            .map(|s| format!("{}::new()", s.name))
            .unwrap_or_else(|| "?".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn view_reference_widget(
        widget_id: WidgetId,
        widget_name: &str,
        primary_view_id: Uuid,
        extra_view_ids: Vec<Uuid>,
    ) -> Widget {
        let mut widget = Widget::new(WidgetType::ViewReference, widget_id);
        widget.properties.widget_name = widget_name.to_string();
        widget.properties.referenced_view_id = Some(primary_view_id);
        widget.properties.extra_view_ids = extra_view_ids;
        widget
    }

    #[test]
    fn resolve_view_reference_target_uses_selection_state_when_present() {
        let owner_id = Uuid::new_v4();
        let primary_id = Uuid::new_v4();
        let secondary_id = Uuid::new_v4();
        let mut owner = AppView::with_id(owner_id, "Owner".to_string(), 0);
        let primary = AppView::with_id(primary_id, "Primary".to_string(), 1);
        let secondary = AppView::with_id(secondary_id, "Secondary".to_string(), 2);
        let widget = view_reference_widget(WidgetId(9), "nav", primary_id, vec![secondary_id]);

        let selection_key = view_selection_state_key(owner_id, "nav_selection");
        owner.custom_state_values.insert(
            selection_key,
            ActionValue::EnumVariant {
                type_name: "NavSelection".to_string(),
                variant: "Secondary".to_string(),
            },
        );

        let views = BTreeMap::from([
            (owner_id, owner),
            (primary_id, primary),
            (secondary_id, secondary),
        ]);
        let resolved = resolve_view_reference_target_view_id(&widget, Some(owner_id), &views);
        assert_eq!(resolved, Some(secondary_id));
    }

    #[test]
    fn resolve_view_reference_target_defaults_to_primary_without_selection_state() {
        let owner_id = Uuid::new_v4();
        let primary_id = Uuid::new_v4();
        let secondary_id = Uuid::new_v4();
        let owner = AppView::with_id(owner_id, "Owner".to_string(), 0);
        let primary = AppView::with_id(primary_id, "Primary".to_string(), 1);
        let secondary = AppView::with_id(secondary_id, "Secondary".to_string(), 2);
        let widget = view_reference_widget(WidgetId(11), "nav", primary_id, vec![secondary_id]);

        let views = BTreeMap::from([
            (owner_id, owner),
            (primary_id, primary),
            (secondary_id, secondary),
        ]);
        let resolved = resolve_view_reference_target_view_id(&widget, Some(owner_id), &views);
        assert_eq!(resolved, Some(primary_id));
    }

    #[test]
    fn resolve_view_reference_target_prefers_canonical_widget_identity_state() {
        let owner_id = Uuid::new_v4();
        let primary_id = Uuid::new_v4();
        let secondary_id = Uuid::new_v4();
        let mut owner = AppView::with_id(owner_id, "Owner".to_string(), 0);
        let primary = AppView::with_id(primary_id, "Primary".to_string(), 1);
        let secondary = AppView::with_id(secondary_id, "Renamed Secondary".to_string(), 2);
        let widget = view_reference_widget(WidgetId(42), "nav", primary_id, vec![secondary_id]);

        // Legacy selection points to primary, but canonical target should win.
        let legacy_key = view_selection_state_key(owner_id, "nav_selection");
        owner.custom_state_values.insert(
            legacy_key,
            ActionValue::EnumVariant {
                type_name: "NavSelection".to_string(),
                variant: "Primary".to_string(),
            },
        );
        let canonical_key = view_reference_selection_state_key(owner_id, WidgetId(42));
        owner
            .custom_state_values
            .insert(canonical_key, ActionValue::String(secondary_id.to_string()));

        let views = BTreeMap::from([
            (owner_id, owner),
            (primary_id, primary),
            (secondary_id, secondary),
        ]);
        let resolved = resolve_view_reference_target_view_id(&widget, Some(owner_id), &views);
        assert_eq!(resolved, Some(secondary_id));
    }
}

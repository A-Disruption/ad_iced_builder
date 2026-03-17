use crate::action_system::custom_state::CustomFieldType;
use crate::code_gen_version_two::builder::sanitize_name;
use crate::data_structures::types::types::{AppView, Widget, WidgetType};
use crate::enum_builder::TypeSystem;
use crate::icon;
use crate::icon_lucide;
use crate::views::add_views::Message;
use iced::widget::{
    button, checkbox, column, container, pick_list, row, rule, scrollable, text, text_input,
};
use iced::{Element, Length, padding};
use std::collections::HashMap;
use widgets::generic_overlay::overlay_button;

struct AutoStateRow<'a> {
    widget: &'a Widget,
    generated_name: String,
    type_label: &'static str,
}

fn stateful_widget_type_label(widget_type: WidgetType) -> Option<&'static str> {
    match widget_type {
        WidgetType::TextInput => Some("String"),
        WidgetType::Checkbox => Some("bool"),
        WidgetType::Slider | WidgetType::VerticalSlider => Some("f32"),
        WidgetType::Toggler => Some("bool"),
        WidgetType::GenericOverlay => Some("bool"),
        WidgetType::DatePicker => Some("bool"),
        WidgetType::PickList => Some("Option<String>"),
        WidgetType::Radio => Some("usize"),
        _ => None,
    }
}

fn auto_state_base_name(widget: &Widget) -> Option<String> {
    if stateful_widget_type_label(widget.widget_type).is_none() {
        return None;
    }

    if !widget.properties.widget_name.trim().is_empty() {
        return Some(sanitize_name(&widget.properties.widget_name));
    }

    Some(
        match widget.widget_type {
            WidgetType::TextInput => "text_input",
            WidgetType::Checkbox => "checkbox",
            WidgetType::Slider => "slider",
            WidgetType::VerticalSlider => "vertical_slider",
            WidgetType::Toggler => "toggler",
            WidgetType::GenericOverlay => "genericoverlay",
            WidgetType::DatePicker => "date_picker",
            WidgetType::PickList => "pick_list",
            WidgetType::Radio => "radio",
            _ => return None,
        }
        .to_string(),
    )
}

fn collect_stateful_widgets<'a>(
    widget: &'a Widget,
    widget_counts: &mut HashMap<String, usize>,
    out: &mut Vec<AutoStateRow<'a>>,
) {
    let type_label = stateful_widget_type_label(widget.widget_type);
    let base_name = auto_state_base_name(widget);
    let has_custom_widget_name = !widget.properties.widget_name.trim().is_empty();

    if let (Some(type_label), Some(base_name)) = (type_label, base_name) {
        let generated_name = if has_custom_widget_name {
            base_name
        } else {
            let type_key = format!("{:?}", widget.widget_type).to_lowercase();
            let count = widget_counts.entry(type_key).or_insert(0);
            *count += 1;

            if *count > 1 {
                format!("{}_{}", base_name, count)
            } else {
                base_name
            }
        };

        out.push(AutoStateRow {
            widget,
            generated_name,
            type_label,
        });
    }

    for child in &widget.children {
        collect_stateful_widgets(child, widget_counts, out);
    }
}

fn custom_field_type_options(
    type_system: &TypeSystem,
    selected_type: Option<&CustomFieldType>,
) -> Vec<CustomFieldType> {
    let mut options = CustomFieldType::primitives();
    options.extend(
        type_system
            .all_enums()
            .into_iter()
            .map(|enum_def| CustomFieldType::Enum(enum_def.name.clone())),
    );
    options.extend(
        type_system
            .all_structs()
            .into_iter()
            .map(|struct_def| CustomFieldType::Struct(struct_def.name.clone())),
    );

    if let Some(selected_type) = selected_type {
        if !options.iter().any(|option| option == selected_type) {
            options.push(selected_type.clone());
        }
    }

    options
}

pub fn view<'a>(
    view: &'a AppView,
    type_system: &'a TypeSystem,
    can_delete: bool,
) -> Element<'a, Message> {
    let vid = view.id;

    let title = column![
        text("Name").width(Length::Fixed(200.0)).center(),
        text_input("View Name", &view.name)
            .on_input(move |s| Message::RenameView(vid, s))
            .width(Length::Fixed(200.0)),
    ]
    .spacing(3);

    let state_header = row![
        text("State").size(13),
        button(icon_lucide::plus().size(12))
            .on_press(Message::AddCustomStateField(vid))
            .style(button::text)
            .padding(2),
    ]
    .spacing(6);

    let mut stateful_widgets = Vec::new();
    let mut widget_counts = HashMap::new();
    collect_stateful_widgets(
        view.hierarchy.root(),
        &mut widget_counts,
        &mut stateful_widgets,
    );

    let auto_state_rows: Vec<Element<'a, Message>> = stateful_widgets
        .iter()
        .map(|row_data| {
            let wid_u64 = row_data.widget.id.0 as u64;
            let effective_name = view
                .widget_state_names
                .get(&wid_u64)
                .cloned()
                .filter(|name| !name.trim().is_empty())
                .unwrap_or_else(|| row_data.generated_name.clone());

            row![
                text(effective_name).width(Length::Fill).size(12),
                text(row_data.type_label)
                    .size(11)
                    .width(Length::Fixed(100.0)),
            ]
            .spacing(4)
            .align_y(iced::Alignment::Center)
            .into()
        })
        .collect();

    let state_rows: Vec<Element<'a, Message>> = view
        .custom_state
        .iter()
        .map(|field| {
            let fid = field.id;
            let name_val = field.name.clone();
            let default_val = field.default_expr.clone();
            let selected_type = field.field_type.clone();
            let type_options = custom_field_type_options(type_system, Some(&selected_type));

            row![
                text_input("name", &name_val)
                    .on_input(move |s| Message::SetCustomFieldName(vid, fid, s))
                    .width(Length::Fixed(90.0))
                    .size(12),
                pick_list(type_options, Some(selected_type), move |t| {
                    Message::SetCustomFieldType(vid, fid, t)
                },)
                .width(Length::Fixed(140.0))
                .text_size(12),
                text_input("default", &default_val)
                    .on_input(move |s| Message::SetCustomFieldDefault(vid, fid, s))
                    .width(Length::Fill)
                    .size(12),
                button(icon::trash())
                    .on_press(Message::RemoveCustomStateField(vid, fid))
                    .style(button::text)
                    .padding(2),
            ]
            .spacing(4)
            .into()
        })
        .collect();

    let mut state_col = column![state_header].spacing(4);
    for row in auto_state_rows {
        state_col = state_col.push(row);
    }
    if !stateful_widgets.is_empty() && !view.custom_state.is_empty() {
        state_col = state_col.push(rule::horizontal(1));
    }
    for row in state_rows {
        state_col = state_col.push(row);
    }

    let delete_view_button = if can_delete {
        button(text("Delete View"))
            .on_press(Message::RemoveView(vid))
            .style(button::danger)
            .width(Length::Fill)
    } else {
        button(text("Delete View"))
            .style(button::secondary)
            .width(Length::Fill)
    };

    let settings_view = container(scrollable(
        column![
            title,
            checkbox(view.show_widget_bounds)
                .label("Enable .explain()")
                .on_toggle(move |_| Message::ToggleExplain(vid)),
            rule::horizontal(1),
            state_col,
            rule::horizontal(1),
            delete_view_button,
        ]
        .spacing(12)
        .padding(padding::top(10.0)),
    ))
    .center_x(Length::Fill);

    overlay_button(
        icon_lucide::settings(),
        format!("{} Settings", &view.name),
        settings_view,
    )
    .overlay_padding(5.0)
    .overlay_radius(10.0)
    .overlay_width(420.0)
    .overlay_height(400.0)
    .close_on_click_outside()
    .style(button::text)
    .padding(1.0)
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_structures::types::types::WidgetId;

    #[test]
    fn collect_stateful_widgets_matches_codegen_style_names() {
        let mut root = Widget::new(WidgetType::Column, WidgetId(1));
        let first = Widget::new(WidgetType::TextInput, WidgetId(2));
        let mut second = Widget::new(WidgetType::TextInput, WidgetId(3));
        second.properties.widget_name = "Search Box".to_string();
        root.children.push(first);
        root.children.push(second);

        let mut rows = Vec::new();
        let mut widget_counts = HashMap::new();
        collect_stateful_widgets(&root, &mut widget_counts, &mut rows);

        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].generated_name, "text_input");
        assert_eq!(rows[1].generated_name, "search_box");
    }

    #[test]
    fn collect_stateful_widgets_includes_generic_overlay_open_state() {
        let mut root = Widget::new(WidgetType::Column, WidgetId(1));
        root.children
            .push(Widget::new(WidgetType::GenericOverlay, WidgetId(2)));

        let mut rows = Vec::new();
        let mut widget_counts = HashMap::new();
        collect_stateful_widgets(&root, &mut widget_counts, &mut rows);

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].generated_name, "genericoverlay");
        assert_eq!(rows[0].type_label, "bool");
    }

    #[test]
    fn collect_stateful_widgets_includes_date_picker_open_state() {
        let mut root = Widget::new(WidgetType::Column, WidgetId(1));
        root.children
            .push(Widget::new(WidgetType::DatePicker, WidgetId(2)));

        let mut rows = Vec::new();
        let mut widget_counts = HashMap::new();
        collect_stateful_widgets(&root, &mut widget_counts, &mut rows);

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].generated_name, "date_picker");
        assert_eq!(rows[0].type_label, "bool");
    }
}

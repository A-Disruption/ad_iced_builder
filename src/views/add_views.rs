use crate::EnumEditorView;
use crate::action_system::custom_state::CustomFieldType;
use crate::data_structures::types::types::{AppView, WidgetId};
use crate::enum_builder::TypeSystem;
use crate::icon;
use crate::icon_lucide;
use crate::views::theme_and_stylefn_builder::CustomThemes;
use crate::widget_tree;
use iced::{
    Alignment, Element, Length, Task, Theme, padding,
    widget::{button, column, container, row, scrollable, space, text, text_editor},
};
use std::collections::BTreeMap;
use uuid::Uuid;
use widgets::collapsible::CollapsibleGroup;
use widgets::collapsible::collapsible;

// Application messages
#[derive(Debug, Clone)]
pub enum Message {
    AddView,
    ViewSelected(Uuid),
    RemoveView(Uuid),
    RenameView(Uuid, String),
    ToggleExplain(Uuid),
    TreeMessage(widget_tree::Message),
    // Custom state field management
    AddCustomStateField(Uuid),
    RemoveCustomStateField(Uuid, Uuid),
    SetCustomFieldName(Uuid, Uuid, String),
    SetCustomFieldType(Uuid, Uuid, CustomFieldType),
    SetCustomFieldDefault(Uuid, Uuid, String),
}

fn ensure_single_main_view(views: &mut BTreeMap<Uuid, AppView>) {
    let mut main_ids: Vec<Uuid> = views
        .iter()
        .filter_map(|(id, view)| view.is_main.then_some(*id))
        .collect();

    if main_ids.is_empty() {
        if let Some((&replacement_id, _)) = views.iter().min_by_key(|(_, view)| view.order) {
            if let Some(view) = views.get_mut(&replacement_id) {
                view.is_main = true;
            }
        }
        return;
    }

    main_ids.sort_by_key(|id| views.get(id).map(|view| view.order).unwrap_or(usize::MAX));
    let retained_main_id = main_ids[0];

    for (id, view) in views.iter_mut() {
        view.is_main = *id == retained_main_id;
    }
}

pub fn update<'a>(
    views: &'a mut BTreeMap<Uuid, AppView>,
    selected_view_id: &'a mut Uuid,
    type_system: &'a mut TypeSystem,
    type_editor: &'a mut EnumEditorView,
    message: Message,
) -> Task<Message> {
    match message {
        Message::ViewSelected(id) => {
            // Check if view exists before selecting
            if views.contains_key(&id) {
                *selected_view_id = id;
            }
        }

        Message::AddView => {
            let order = views.iter().count() + 1;
            let new_view = AppView::new("New View".to_string(), order);
            let new_id = new_view.id;
            views.insert(new_id, new_view);
            *selected_view_id = new_id; // Auto-select new view
        }

        Message::RemoveView(id) => {
            if views.get(&id).is_some_and(|view| view.is_main) {
                return Task::none();
            }

            if views.len() > 1 {
                let removed_selected_view = id == *selected_view_id;
                views.remove(&id);

                if removed_selected_view {
                    if let Some((&replacement_id, _)) =
                        views.iter().min_by_key(|(_, view)| view.order)
                    {
                        *selected_view_id = replacement_id;
                    }
                }

                ensure_single_main_view(views);
            }
        }

        Message::RenameView(id, new_name) => {
            if let Some(view) = views.get_mut(&id) {
                view.name = new_name;
            }
        }
        Message::TreeMessage(msg) => {
            let view = views
                .get_mut(selected_view_id)
                .expect("Selected view must exist");

            return widget_tree::update(msg, &mut view.hierarchy, type_system, type_editor)
                .map(|m| Message::TreeMessage(m));
        }
        Message::ToggleExplain(id) => {
            if let Some(view) = views.get_mut(&id) {
                view.show_widget_bounds = !view.show_widget_bounds;
            }
        }
        Message::AddCustomStateField(view_id) => {
            use crate::action_system::custom_state::CustomStateField;
            if let Some(view) = views.get_mut(&view_id) {
                let idx = view.custom_state.len() + 1;
                view.custom_state
                    .push(CustomStateField::new(format!("field_{}", idx)));
            }
        }
        Message::RemoveCustomStateField(view_id, field_id) => {
            if let Some(view) = views.get_mut(&view_id) {
                view.custom_state.retain(|f| f.id != field_id);
            }
        }
        Message::SetCustomFieldName(view_id, field_id, name) => {
            if let Some(view) = views.get_mut(&view_id) {
                if let Some(f) = view.custom_state.iter_mut().find(|f| f.id == field_id) {
                    f.display_name = name.replace('_', " ");
                    f.name = name;
                }
            }
        }
        Message::SetCustomFieldType(view_id, field_id, field_type) => {
            if let Some(view) = views.get_mut(&view_id) {
                if let Some(f) = view.custom_state.iter_mut().find(|f| f.id == field_id) {
                    f.default_expr = field_type.default_expr();
                    f.field_type = field_type;
                }
            }
        }
        Message::SetCustomFieldDefault(view_id, field_id, expr) => {
            if let Some(view) = views.get_mut(&view_id) {
                if let Some(f) = view.custom_state.iter_mut().find(|f| f.id == field_id) {
                    f.default_expr = expr;
                }
            }
        }
    }
    Task::none()
}

pub fn view<'a>(
    views: &'a BTreeMap<Uuid, AppView>,
    _selected_view_id: &'a Uuid,
    type_system: &'a TypeSystem,
    theme: &'a Theme,
    custom_themes: &'a CustomThemes,
    widget_preview_content: &'a text_editor::Content,
    open_editor_widget_id: Option<WidgetId>,
) -> Element<'a, Message> {
    let header = row![
        space::horizontal(),
        text("Views").size(18),
        space::horizontal(),
        button(icon_lucide::plus())
            .on_press(Message::AddView)
            .style(button::text)
    ]
    .align_y(Alignment::Center)
    .padding(padding::right(10));

    let sorted_list = sorted_views(views, ViewSortMode::InsertionOrder);

    let view_list: Vec<_> = sorted_list
        .iter()
        .map(|view| {
            collapsible(
                &view.1.name,
                crate::widget_tree::view(
                    &view.1.hierarchy,
                    type_system,
                    theme,
                    views,
                    custom_themes,
                    widget_preview_content,
                    open_editor_widget_id,
                    view.1.id,
                )
                .map(|msg| Message::TreeMessage(msg)),
            )
            .expand_icon(icon::collapsed())
            .on_toggle(|_| Message::ViewSelected(view.1.id))
            .collapse_icon(icon::expanded())
            .action_icon(crate::views::settings_views::view_settings::view(
                &view.1,
                type_system,
                views.len() > 1 && !view.1.is_main,
            ))
            .into()
        })
        .collect();

    container(column![
        header,
        scrollable(
            container(CollapsibleGroup::new(view_list).spacing(5.0)).padding(padding::right(10))
        )
        .height(Length::Fill)
    ])
    .width(400)
    .height(Length::Fill)
    .into()
}

fn sorted_views<'a>(
    views: &'a BTreeMap<Uuid, AppView>,
    sort_mode: ViewSortMode,
) -> Vec<(&'a Uuid, &'a AppView)> {
    let mut sorted: Vec<_> = views.iter().collect();

    match sort_mode {
        ViewSortMode::InsertionOrder => {
            sorted.sort_by_key(|(_, v)| v.order);
        }
        ViewSortMode::Alphabetical => {
            sorted.sort_by(|(_, a), (_, b)| a.name.cmp(&b.name));
        }
        ViewSortMode::MostRecentlyModified => {
            //            sorted.sort_by_key(|(_, v)| std::cmp::Reverse(v.last_modified));
            sorted.sort_by_key(|(_, v)| v.order);
        }
    }

    sorted
}

#[derive(Debug, Clone, Copy)]
enum ViewSortMode {
    InsertionOrder,
    Alphabetical,
    MostRecentlyModified,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn removing_main_view_is_ignored() {
        let main_id = Uuid::new_v4();
        let secondary_id = Uuid::new_v4();
        let tertiary_id = Uuid::new_v4();

        let mut views = BTreeMap::new();
        views.insert(main_id, AppView::with_id(main_id, "Main".to_string(), 0));
        views.insert(
            secondary_id,
            AppView::with_id(secondary_id, "Second".to_string(), 1),
        );
        views.insert(
            tertiary_id,
            AppView::with_id(tertiary_id, "Third".to_string(), 2),
        );

        let mut selected_view_id = secondary_id;
        let mut type_system = TypeSystem::new();
        let mut type_editor = EnumEditorView::new();

        let _ = update(
            &mut views,
            &mut selected_view_id,
            &mut type_system,
            &mut type_editor,
            Message::RemoveView(main_id),
        );

        assert_eq!(views.len(), 3);
        assert!(views.contains_key(&main_id));
        assert_eq!(selected_view_id, secondary_id);
        assert!(views.get(&main_id).is_some_and(|view| view.is_main));
        assert!(views.get(&secondary_id).is_some_and(|view| !view.is_main));
        assert!(views.get(&tertiary_id).is_some_and(|view| !view.is_main));
    }

    #[test]
    fn removing_non_main_view_keeps_main_view() {
        let main_id = Uuid::new_v4();
        let secondary_id = Uuid::new_v4();
        let tertiary_id = Uuid::new_v4();

        let mut views = BTreeMap::new();
        views.insert(main_id, AppView::with_id(main_id, "Main".to_string(), 0));
        views.insert(
            secondary_id,
            AppView::with_id(secondary_id, "Second".to_string(), 1),
        );
        views.insert(
            tertiary_id,
            AppView::with_id(tertiary_id, "Third".to_string(), 2),
        );

        let mut selected_view_id = secondary_id;
        let mut type_system = TypeSystem::new();
        let mut type_editor = EnumEditorView::new();

        let _ = update(
            &mut views,
            &mut selected_view_id,
            &mut type_system,
            &mut type_editor,
            Message::RemoveView(secondary_id),
        );

        assert_eq!(views.len(), 2);
        assert!(!views.contains_key(&secondary_id));
        assert_eq!(selected_view_id, main_id);
        assert!(views.get(&main_id).is_some_and(|view| view.is_main));
        assert!(views.get(&tertiary_id).is_some_and(|view| !view.is_main));
    }
}

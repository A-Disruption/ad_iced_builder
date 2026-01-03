use iced::{Alignment, Element, Length::{self, Shrink}, padding, Settings, Task, Theme, widget::{ Column, button, column, container, row, rule, scrollable, space, text }
};
use std::collections::BTreeMap;
use uuid::Uuid;
use crate::icon;
use crate::styles;
use widgets::tree::{tree_handle, branch, DropInfo, DropPosition, Branch};
use widgets::collapsible::collapsible;
use widgets::collapsible::CollapsibleGroup;
use crate::data_structures::types::types::AppView;
use crate::enum_builder::TypeSystem;
use crate::EnumEditorView;
use crate::widget_tree;
use crate::settings_views::window_settings;
use crate::views::theme_and_stylefn_builder::CustomThemes;

// Application messages
#[derive(Debug, Clone)]
pub enum Message {
    AddView,
    ViewSelected(Uuid),
    RemoveView(Uuid),
    RenameView(Uuid, String),
    ToggleExplain(Uuid),
    TreeMessage(widget_tree::Message)
}

pub fn update<'a>(
    views: &'a mut BTreeMap<Uuid, AppView>, 
    selected_view_id: &'a mut Uuid,
    type_system: &'a mut TypeSystem,
    type_editor: &'a mut EnumEditorView,
    message: Message
) -> Task<Message>{
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
            // Don't allow removing the last view
            if views.len() > 1 && id != *selected_view_id {
                views.remove(&id);
            } else if views.len() > 1 {
                // If removing selected view, select another one first
                views.remove(&id);
                if let Some((&first_id, _)) = views.iter().next() {
                    *selected_view_id = first_id;
                }
            }
        }
        
        Message::RenameView(id, new_name) => {
            if let Some(view) = views.get_mut(&id) {
                view.name = new_name;
            }
        }
        Message::TreeMessage(msg) => {
            let view = views.get_mut(selected_view_id).expect("Selected view must exist");                       

            return widget_tree::update(
                msg, 
                &mut view.hierarchy, 
                type_system,
                type_editor
            )
            .map(|m| Message::TreeMessage(m));            
        }
        Message::ToggleExplain(id) => {
            if let Some(view) = views.get_mut(&id) {
                view.show_widget_bounds = !view.show_widget_bounds;
            }
        }
    }
    Task::none()
}

pub fn view<'a>(
    views: &'a BTreeMap<Uuid, AppView>, 
    selected_view_id: &'a Uuid,
    type_system: &'a TypeSystem,
    theme: &'a Theme,
    custom_themes: &'a CustomThemes,
) -> Element<'a, Message> {
    let header =
        row![
            space::horizontal(),
            text("Views").size(18),
            space::horizontal(),

            button(icon::plus())
                .on_press(Message::AddView)
                .style(button::text)
        ].align_y(Alignment::Center).padding(padding::right(10));

    let sorted_list = sorted_views(views, ViewSortMode::InsertionOrder);

    let view_list: Vec<_> = sorted_list.iter().map( |view| {
        collapsible(
            &view.1.name,
            crate::widget_tree::view(&view.1.hierarchy, type_system, theme, views, custom_themes).map(|msg| Message::TreeMessage(msg) )
        )
        .expand_icon(icon::collapsed())
        .on_toggle(|_| Message::ViewSelected(view.1.id))
        .collapse_icon(icon::expanded())
        .action_icon(crate::views::settings_views::view_settings::view(&view.1))
        .into()
    }).collect();
    
    container(
        column![
            header,
            scrollable(
                container(
                    CollapsibleGroup::new(view_list).spacing(5.0)
                ).padding(padding::right(10))
            )
            .height(Length::Fill)
        ]
    )
    .width(400)
    .height(Length::Fill)
    .into()   
}

fn sorted_views<'a>(
    views: &'a BTreeMap<Uuid, AppView>,
    sort_mode: ViewSortMode
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
use iced::{event, window, Alignment, Element, Subscription, Task, Theme, Length};
use iced::widget::{column, row, container, text, scrollable, space};
use arboard::Clipboard;
use std::collections::BTreeMap;
use uuid::Uuid;

mod code_generator;
mod controls;
mod data_structures;
mod icon;
mod styles;
mod enum_builder;
mod views;

use views::*;
use views::enum_editor::EnumEditorView;
use views::theme_and_stylefn_builder::CustomThemes;
use data_structures::widget_hierarchy::WidgetHierarchy;
use data_structures::types::types::WidgetType;
use data_structures::types::types::AppView;
use data_structures::types::types::WindowConfig;

use views::theme_and_stylefn_builder;
//use crate::styles::stylefn_builders;


fn main() {
    iced::daemon(AdUiBuilder::new, AdUiBuilder::update, AdUiBuilder::view)
        .title(AdUiBuilder::title)
        .theme(AdUiBuilder::theme)
        .subscription(AdUiBuilder::subscription)
        .font(icon::FONT)
        .run()
        .unwrap()
}

struct AdUiBuilder {
    windows: BTreeMap<window::Id, Window>,
    selected_view: navigation_bar::ViewSelection,
    theme: Theme,

    views: BTreeMap<Uuid, AppView>,
    selected_view_id: Uuid, // Used to load the preview for a given user defined view
    selected_window: Option<window::Id>,
    active_code_tab: String,

    app_name: String,
    custom_themes: CustomThemes,
    type_system: enum_builder::TypeSystem,
    type_editor: EnumEditorView,

    generated_files: std::collections::HashMap<String, Vec<crate::code_generator::tokens::Token>>,
}

#[derive(Clone, Debug)]
enum Message {
    ChooseTheme(Theme),

    // View Messages
    ViewMessages(ViewMessage),

    //window handles
    WindowClosed(iced::window::Id),
    RequestOpenWindow(WindowEnum),
    WindowOpened(iced::window::Id, WindowEnum),
}

#[derive(Clone, Debug)]
enum ViewMessage {
    NavigationBar(navigation_bar::Message),
    WidgetTree(widget_tree::Message),
    EnumEditor(enum_editor::Message),
    ThemeBuilder(theme_and_stylefn_builder::Message),
    AddWidgets(add_widgets::Message),
    AddViews(add_views::Message),
    Preview(preview::Message),
    WindowSettings(settings_views::window_settings::Message),
}

impl AdUiBuilder {
    fn new() -> (Self, Task<Message>) {
        let view_id = Uuid::new_v4();
        let initial_view = AppView::with_id(view_id, "Main View".to_string(), 0);
        

        let mut editor = Self {
            windows: BTreeMap::new(),
            selected_window: None,
            selected_view: navigation_bar::ViewSelection::Main,
            theme: iced::theme::Theme::Dark,

            views: BTreeMap::from([(view_id, initial_view)]),
            selected_view_id: view_id,
            active_code_tab: "main.rs".to_string(),

            app_name: "App".to_string(),
            custom_themes: CustomThemes::new(&Theme::Dark),
            type_system: enum_builder::TypeSystem::new(),
            type_editor: EnumEditorView::new(),

            generated_files: std::collections::HashMap::new(),
        };

        editor.regenerate_code();

        (
            editor, 
            Task::done(Message::RequestOpenWindow(WindowEnum::Editor))
                .chain(Task::done(Message::RequestOpenWindow(WindowEnum::Visualizer)))
        )
    }

    fn theme(&self, _window_id: window::Id) -> Theme {
        self.theme.clone()
    }

    fn title(&self, window_id: window::Id) -> String {
        self.windows.get(&window_id).map(|window| window.config.title.clone()).unwrap_or_default()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ChooseTheme(theme) => {
                self.theme = theme;
            }

            Message::ViewMessages(view) => {
                match view {
                    ViewMessage::NavigationBar(msg) => {
                        match msg {
                            navigation_bar::Message::ActiveView(selection) => {
                                self.selected_view = selection;
                            }
                        }
                    }
                    ViewMessage::WidgetTree(msg) => {
                        let view = self.views.get_mut(&self.selected_view_id).expect("Selected view must exist");
                        if let widget_tree::Message::CodeTabSelected(file) = msg {
                            self.active_code_tab = file;
                            return Task::none()
                        }                       

                        let result = widget_tree::update(
                            msg, 
                            &mut view.hierarchy, 
                            &mut self.type_system,
                            &mut self.type_editor
                        );

                        self.regenerate_code();
                        return result.map(|m| Message::ViewMessages(ViewMessage::WidgetTree(m)));
                    }
                    ViewMessage::EnumEditor(msg) => {
                    
                        let result = enum_editor::update(
                            msg,
                            &mut self.type_system,
                            &mut self.type_editor
                        );

                        self.regenerate_code();
                        return result.map(|m| Message::ViewMessages(ViewMessage::EnumEditor(m)));
                    }
                    ViewMessage::ThemeBuilder(msg) => {
                    
                        return self.custom_themes.update(msg)
                            .map(|m| Message::ViewMessages(ViewMessage::ThemeBuilder(m)));
                    }
                    ViewMessage::AddWidgets(msg) => {
                        let view = self.views.get_mut(&self.selected_view_id).expect("Selected view must exist");

                        let result = add_widgets::update(
                            &mut view.hierarchy,
                            &mut self.type_system, 
                            msg
                        );

                        self.regenerate_code();
                        return result.map(|m| Message::ViewMessages(ViewMessage::AddWidgets(m)));
                    }
                    ViewMessage::AddViews(msg) => {

                        let result = add_views::update(
                            &mut self.views,
                            &mut self.selected_view_id,
                            &mut self.type_system,
                            &mut self.type_editor,
                            msg
                        );

                        self.regenerate_code();
                        return result.map(|m| Message::ViewMessages(ViewMessage::AddViews(m)));
                    }
                    ViewMessage::Preview(msg) => {
//                        let view = self.views.get_mut(&self.selected_view_id).expect("Selected view must exist");

                        return preview::update(
                            &mut self.views,
                            &mut self.type_system,
                            msg
                        )
                        .map(|m| Message::ViewMessages(ViewMessage::Preview(m)))
                    }
                    ViewMessage::WindowSettings(msg) => {
                        if let settings_views::window_settings::Message::UpdateTheme(theme) = msg {
                            self.theme = theme;
                            return Task::none();
                        }

                        return settings_views::window_settings::update(
                            &mut self.windows,
                            msg
                        )
                        .map(|m| Message::ViewMessages(ViewMessage::WindowSettings(m)))                        
                    }
                }
            }

            //window handles
            Message::WindowClosed(window_id) => {
                println!("Close window request, requested.");
                self.windows.remove(&window_id);
                if self.windows.is_empty() {
                    return iced::exit()
                }
                else {}
            },
            Message::RequestOpenWindow(window_type) => {
                match window_type {
                    WindowEnum::Editor => {
                        let config = WindowConfig::editor();
                        let (_id, open) = iced::window::open(config.settings);

                        return open.map(|id| Message::WindowOpened(id, WindowEnum::Editor))
                    }
                    WindowEnum::Visualizer => {
                        // Check if already exists
                        if let Some(window_id) = self.windows
                            .iter()
                            .find(|(_, w)| w.windowtype == WindowEnum::Visualizer)
                            .map(|(id, _)| *id)
                        {
                            self.selected_window = Some(window_id);
                            return iced::Task::batch([
                                window::minimize(window_id, false),
                                window::gain_focus(window_id)
                            ]);
                        }
                        
                        // Create new visualizer
                        let config = WindowConfig::visualizer();
                        let (_id, open) = iced::window::open(config.settings.clone());
                        
                        return open.map(move |id| Message::WindowOpened(id, WindowEnum::Visualizer))
                    }
                }
            },
            Message::WindowOpened(window_id, window_type) => {
                let new_window = Window::new(window_id, window_type);
                self.windows.insert(window_id, new_window);
            },

        }

        Task::none()
    }

    fn view<'a>(&'a self, window_id: window::Id) -> Element<'a, Message> {
        let preview_view = self.views.get(&self.selected_view_id).expect("Selected view must exist");
        let selected_widget = preview_view.hierarchy.get_single_selected().unwrap_or(preview_view.hierarchy.root());
        let selected_window = match self.selected_window {
            Some(selected_window) => self.windows.get(&selected_window),
            None => None
        };
        let selected_view = self.views.get(&self.selected_view_id).expect("Failed to get View Id");

        let view = match self.selected_view {
            navigation_bar::ViewSelection::Main => {
                row![
                    column![ // Left Side
                        add_views::view(&self.views, &self.selected_view_id, &self.type_system, &self.theme, &self.custom_themes ).map(|msg| Message::ViewMessages( ViewMessage::AddViews(msg))),
                        container(add_widgets::view(&preview_view.hierarchy, &selected_widget.id).map(|msg| Message::ViewMessages( ViewMessage::AddWidgets(msg)))).align_bottom(Length::Shrink).width(400),
                    ]
                    .spacing(5),

                    // Right side
                    full_code_view::view(             
                        &self.generated_files, 
                        &self.active_code_tab,
                        &self.theme
                    ).map(|msg| Message::ViewMessages( ViewMessage::WidgetTree(msg))),
                ].into()
                
            }
//            navigation_bar::ViewSelection::Code => {}
            navigation_bar::ViewSelection::ThemeBuilder => {
                self.custom_themes.view(&self.theme).map(|msg| Message::ViewMessages( ViewMessage::ThemeBuilder(msg)))
            }
//            navigation_bar::ViewSelection::WidgetStyleBuilder => {}
            navigation_bar::ViewSelection::EnumBuilder => {
                row![// Left Side
                    enum_editor::view(&self.type_system, &self.type_editor).map(|msg| Message::ViewMessages( ViewMessage::EnumEditor(msg))),

                    // Right side
                    full_code_view::view(             
                        &self.generated_files, 
                        &self.active_code_tab,
                        &self.theme
                    ).map(|msg| Message::ViewMessages( ViewMessage::WidgetTree(msg))),
                ].into()
                
            }
            navigation_bar::ViewSelection::Settings => {
                row![
                    settings_views::window_settings::view(
                        &self.windows, 
                        &self.views,
                        &self.theme
                    ).map(|msg| Message::ViewMessages( ViewMessage::WindowSettings(msg))),

                    // Right side
                    full_code_view::view(             
                        &self.generated_files, 
                        &self.active_code_tab,
                        &self.theme
                    ).map(|msg| Message::ViewMessages( ViewMessage::WidgetTree(msg))),
                ].into()
            }
              _ => { container("No done, sorry :(").into() }
        };

        match self.windows.get(&window_id) {
            Some(window) => match window.windowtype {
                WindowEnum::Editor => {
                    column![
                        navigation_bar::navigation_bar(&self.selected_view, &self.theme).map(|selection| Message::ViewMessages( ViewMessage::NavigationBar(selection))),
                        container(view).padding(10).height(Length::Fill).width(Length::Fill),
                    ].into()
                }
                WindowEnum::Visualizer => {
                    let view_id = window.view_to_display(self.selected_view_id);
                    let view_to_render = self.views.get(&view_id)
                        .expect("View assigned to window must exist");

                    preview::view(&view_to_render.hierarchy, &self.theme, &self.custom_themes, selected_view.show_widget_bounds, &self.views, None)
                            .map(|msg| Message::ViewMessages(ViewMessage::Preview(msg)))                    
                }
            }
            None => { 
                let content = column![
                    text(format!("Something has gone terribly wrong. Window Id: {:?}", window_id)),
                ];
                container(
                    content
                ).into() 
            }
        }

        
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            event::listen_with(handle_event),
        ])
    }

    fn regenerate_code(&mut self) {
        use crate::code_generator::CodeGenerator;
        
        // No more dummy hierarchy! 
        // We pass the actual views map.
        let mut generator = CodeGenerator::new(
            &self.views, 
            &self.theme, 
            &self.type_system
        );
        
        generator.set_app_name(self.app_name.clone());

        // Now generates logic for all views
        self.generated_files = generator.generate_project_structure();
    } 
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum WindowEnum {
    #[default]
    Editor,
    Visualizer
}

#[derive(Debug, Clone,)]
pub struct Window {
    pub windowtype: WindowEnum,
    pub config: WindowConfig,
}

impl Window {
    pub fn new(_id: window::Id, window_type: WindowEnum) -> Self {
        let config = match window_type {
            WindowEnum::Editor => WindowConfig::editor(),
            WindowEnum::Visualizer => WindowConfig::visualizer(),
        };

        Self {
            windowtype: window_type,
            config: config,
        }
    }

    // Helper accessor
    pub fn title(&self) -> &str {
        &self.config.title
    }

    /// Helper to get the view to display (with fallback)
    pub fn view_to_display(&self, fallback_view_id: Uuid) -> Uuid {
        self.config.assigned_view_id.unwrap_or(fallback_view_id)
    }
}

fn handle_event(event: event::Event, _status: event::Status, id: iced::window::Id) -> Option<Message> {
    match event {
        event::Event::Window(window::Event::Closed) => Some(Message::WindowClosed(id)),
        _ => None,
    }
}
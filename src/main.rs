use iced::widget::{button, column, container, row, text, text_editor};
use iced::{Element, Length, Subscription, Task, Theme, event, keyboard, window};
use std::collections::BTreeMap;
use std::path::PathBuf;
use uuid::Uuid;

mod action_system;
mod code_gen_version_two;
mod controls;
mod data_structures;
mod enum_builder;
mod icon;
mod icon_lucide;
mod persistence;
mod preview_runtime;
mod styles;
mod views;

use action_system::flow::AppFlow;
use data_structures::types::types::{AppView, WidgetId, WindowConfig};
use views::enum_editor::EnumEditorView;
use views::struct_editor::StructEditorView;
use views::theme_and_stylefn_builder;
use views::theme_and_stylefn_builder::CustomThemes;
use views::*;

fn main() {
    iced::daemon(AdUiBuilder::new, AdUiBuilder::update, AdUiBuilder::view)
        .title(AdUiBuilder::title)
        .theme(AdUiBuilder::theme)
        .subscription(AdUiBuilder::subscription)
        .font(icon::FONT)
        .font(iced_lucide::FONT_BYTES)
        .run()
        .unwrap()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TypeEditorTab {
    Enums,
    Structs,
}

struct AdUiBuilder {
    windows: BTreeMap<window::Id, Window>,
    selected_view: navigation_bar::ViewSelection,
    theme: Theme,

    views: BTreeMap<Uuid, AppView>,
    selected_view_id: Uuid,
    selected_window: Option<window::Id>,
    active_code_tab: String,

    app_name: String,
    custom_styles: CustomThemes,
    type_system: enum_builder::TypeSystem,
    type_editor: EnumEditorView,
    struct_editor: StructEditorView,
    type_editor_tab: TypeEditorTab,

    generated_files: std::collections::HashMap<String, String>,
    code_view_content: text_editor::Content,
    widget_preview_content: text_editor::Content,

    open_editor_widget_id: Option<WidgetId>,
    action_editor: views::action_editor::ActionEditorState,

    /// App-level flows (moved out of AppView — flows are not view-specific).
    flows: Vec<AppFlow>,

    save_path: Option<PathBuf>,
    unsaved_changes: bool,
    pending_new_project: bool,
}

#[derive(Clone, Debug)]
enum Message {
    // View Messages
    ViewMessages(ViewMessage),

    //window handles
    WindowClosed(iced::window::Id),
    RequestOpenWindow(WindowEnum),
    WindowOpened(iced::window::Id, WindowEnum),

    // Persistence
    NewProject,
    NewProjectDiscard,
    SaveAndNewProject,
    SaveProject,
    SaveProjectAs,
    LoadProject,
    ProjectSaveReady(PathBuf),
    /// Carries the raw JSON string from disk (avoids Clone requirement on ProjectData).
    ProjectLoaded(PathBuf, String),
    PersistenceError(String),
}

#[derive(Clone, Debug)]
enum ViewMessage {
    NavigationBar(navigation_bar::Message),
    WidgetTree(widget_tree::Message),
    EnumEditor(enum_editor::Message),
    StructEditor(struct_editor::Message),
    SwitchTypeEditorTab(TypeEditorTab),
    ThemeBuilder(theme_and_stylefn_builder::Message),
    AddWidgets(add_widgets::Message),
    AddViews(add_views::Message),
    Preview(preview::Message),
    WindowSettings(settings_views::window_settings::Message),
    ActionEditor(action_editor::Message),
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
            custom_styles: CustomThemes::new(&Theme::Dark),
            type_system: enum_builder::TypeSystem::new(),
            type_editor: EnumEditorView::new(),
            struct_editor: StructEditorView::new(),
            type_editor_tab: TypeEditorTab::Enums,

            generated_files: std::collections::HashMap::new(),
            code_view_content: text_editor::Content::new(),
            widget_preview_content: text_editor::Content::new(),

            open_editor_widget_id: None,
            action_editor: views::action_editor::ActionEditorState::default(),
            flows: Vec::new(),

            save_path: None,
            unsaved_changes: false,
            pending_new_project: false,
        };

        editor.regenerate_code();

        (
            editor,
            Task::done(Message::RequestOpenWindow(WindowEnum::Editor)).chain(Task::done(
                Message::RequestOpenWindow(WindowEnum::Visualizer),
            )),
        )
    }

    fn theme(&self, _window_id: window::Id) -> Theme {
        self.theme.clone()
    }

    fn title(&self, window_id: window::Id) -> String {
        let base = self
            .windows
            .get(&window_id)
            .map(|w| w.config.title.clone())
            .unwrap_or_default();
        if let Some(path) = &self.save_path {
            let filename = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("project");
            let modified = if self.unsaved_changes { "* " } else { "" };
            format!("{modified}{filename} — {base}")
        } else {
            base
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        println!("Message: {:?}", message);
        match message {
            Message::ViewMessages(view) => {
                match view {
                    ViewMessage::NavigationBar(msg) => match msg {
                        navigation_bar::Message::ActiveView(selection) => {
                            self.selected_view = selection;
                        }
                        navigation_bar::Message::NewProject => {
                            return Task::done(Message::NewProject);
                        }
                        navigation_bar::Message::OpenProject => {
                            return Task::done(Message::LoadProject);
                        }
                        navigation_bar::Message::SaveProject => {
                            return Task::done(Message::SaveProject);
                        }
                        navigation_bar::Message::SaveProjectAs => {
                            return Task::done(Message::SaveProjectAs);
                        }
                    },
                    ViewMessage::WidgetTree(msg) => {
                        if let widget_tree::Message::CodeTabSelected(file) = msg {
                            if self.active_code_tab != file {
                                self.active_code_tab = file;
                                self.update_code_view_content();
                            }
                            return Task::none();
                        }
                        if let widget_tree::Message::CodeViewEdit(action) = msg {
                            match action {
                                text_editor::Action::Edit(_edit) => return Task::none(),
                                _ => {
                                    self.code_view_content.perform(action);
                                }
                            }
                            return Task::none();
                        }
                        if let widget_tree::Message::WidgetPreviewEdit(action) = msg {
                            match action {
                                text_editor::Action::Edit(_edit) => return Task::none(),
                                _ => {
                                    self.widget_preview_content.perform(action);
                                }
                            }
                            return Task::none();
                        }

                        let should_regenerate = matches!(
                            msg,
                            widget_tree::Message::TreeMove(_)
                                | widget_tree::Message::DeleteWidget(_)
                                | widget_tree::Message::PropertyChanged(_, _)
                                | widget_tree::Message::SwapKind(_)
                        );

                        let view = self
                            .views
                            .get_mut(&self.selected_view_id)
                            .expect("Selected view must exist");
                        let result = widget_tree::update(
                            msg,
                            &mut view.hierarchy,
                            &mut self.type_system,
                            &mut self.type_editor,
                        );

                        if should_regenerate {
                            self.regenerate_code();
                        }
                        return result.map(|m| Message::ViewMessages(ViewMessage::WidgetTree(m)));
                    }
                    ViewMessage::EnumEditor(msg) => {
                        let result =
                            enum_editor::update(msg, &mut self.type_system, &mut self.type_editor);

                        self.regenerate_code();
                        return result.map(|m| Message::ViewMessages(ViewMessage::EnumEditor(m)));
                    }
                    ViewMessage::StructEditor(msg) => {
                        let result = struct_editor::update(
                            msg,
                            &mut self.type_system,
                            &mut self.struct_editor,
                        );

                        self.regenerate_code();
                        return result.map(|m| Message::ViewMessages(ViewMessage::StructEditor(m)));
                    }
                    ViewMessage::SwitchTypeEditorTab(tab) => {
                        self.type_editor_tab = tab;
                    }
                    ViewMessage::ThemeBuilder(msg) => {
                        return self
                            .custom_styles
                            .update(msg)
                            .map(|m| Message::ViewMessages(ViewMessage::ThemeBuilder(m)));
                    }
                    ViewMessage::AddWidgets(msg) => {
                        let view = self
                            .views
                            .get_mut(&self.selected_view_id)
                            .expect("Selected view must exist");

                        let result =
                            add_widgets::update(&mut view.hierarchy, &mut self.type_system, msg);

                        self.regenerate_code();
                        return result.map(|m| Message::ViewMessages(ViewMessage::AddWidgets(m)));
                    }
                    ViewMessage::AddViews(msg) => {
                        // Handle overlay open/close from AddViews tree
                        if let add_views::Message::TreeMessage(
                            widget_tree::Message::OverlayOpened(widget_id, _, _),
                        ) = msg
                        {
                            self.open_editor_widget_id = Some(widget_id);
                            self.update_widget_preview_content_for(widget_id);
                            return Task::none();
                        }
                        if let add_views::Message::TreeMessage(
                            widget_tree::Message::OverlayClosed(_),
                        ) = msg
                        {
                            self.open_editor_widget_id = None;
                            return Task::none();
                        }

                        let should_regenerate = matches!(
                            &msg,
                            add_views::Message::AddView
                                | add_views::Message::RemoveView(_)
                                | add_views::Message::RenameView(_, _)
                                | add_views::Message::AddCustomStateField(_)
                                | add_views::Message::RemoveCustomStateField(_, _)
                                | add_views::Message::SetCustomFieldName(_, _, _)
                                | add_views::Message::SetCustomFieldType(_, _, _)
                                | add_views::Message::SetCustomFieldDefault(_, _, _)
                                | add_views::Message::TreeMessage(
                                    widget_tree::Message::TreeMove(_)
                                        | widget_tree::Message::DeleteWidget(_)
                                        | widget_tree::Message::PropertyChanged(_, _)
                                        | widget_tree::Message::SwapKind(_)
                                )
                        );

                        let result = add_views::update(
                            &mut self.views,
                            &mut self.selected_view_id,
                            &mut self.type_system,
                            &mut self.type_editor,
                            msg,
                        );

                        if should_regenerate {
                            self.regenerate_code();
                        }
                        return result.map(|m| Message::ViewMessages(ViewMessage::AddViews(m)));
                    }
                    ViewMessage::Preview(preview::Message::NavigatedToView(view_id)) => {
                        if self.views.contains_key(&view_id) {
                            self.selected_view_id = view_id;
                        }
                        return Task::none();
                    }
                    ViewMessage::Preview(msg) => {
                        return preview::update(
                            &self.flows,
                            &mut self.views,
                            &mut self.type_system,
                            msg,
                        )
                        .map(|m| Message::ViewMessages(ViewMessage::Preview(m)));
                    }
                    ViewMessage::ActionEditor(msg) => {
                        let task = action_editor::update_with_type_system(
                            &mut self.views,
                            &mut self.flows,
                            &mut self.action_editor,
                            msg,
                            &self.type_system,
                        )
                        .map(|m| Message::ViewMessages(ViewMessage::ActionEditor(m)));
                        self.regenerate_code();
                        return task;
                    }
                    ViewMessage::WindowSettings(msg) => {
                        if let settings_views::window_settings::Message::UpdateTheme(theme) = msg {
                            self.theme = theme;
                            self.custom_styles.theme(&self.theme);
                            self.regenerate_code();
                            return Task::none();
                        }

                        let result =
                            settings_views::window_settings::update(&mut self.windows, msg)
                                .map(|m| Message::ViewMessages(ViewMessage::WindowSettings(m)));

                        self.regenerate_code();
                        return result;
                    }
                }
            }

            //window handles
            Message::WindowClosed(window_id) => {
                println!("Close window request, requested.");
                self.windows.remove(&window_id);
                if self.windows.is_empty() {
                    return iced::exit();
                } else {
                }
            }
            Message::RequestOpenWindow(window_type) => {
                match window_type {
                    WindowEnum::Editor => {
                        let config = WindowConfig::editor();
                        let (_id, open) = iced::window::open(config.settings);

                        return open.map(|id| Message::WindowOpened(id, WindowEnum::Editor));
                    }
                    WindowEnum::Visualizer => {
                        // Check if already exists
                        if let Some(window_id) = self
                            .windows
                            .iter()
                            .find(|(_, w)| w.windowtype == WindowEnum::Visualizer)
                            .map(|(id, _)| *id)
                        {
                            self.selected_window = Some(window_id);
                            return iced::Task::batch([
                                window::minimize(window_id, false),
                                window::gain_focus(window_id),
                            ]);
                        }

                        // Create new visualizer
                        let config = WindowConfig::visualizer();
                        let (_id, open) = iced::window::open(config.settings.clone());

                        return open
                            .map(move |id| Message::WindowOpened(id, WindowEnum::Visualizer));
                    }
                }
            }
            Message::WindowOpened(window_id, window_type) => {
                let new_window = Window::new(window_id, window_type);
                self.windows.insert(window_id, new_window);
            }

            Message::NewProject => {
                let has_project = self.unsaved_changes || self.save_path.is_some();
                if !has_project {
                    let windows = std::mem::take(&mut self.windows);
                    let selected_window = self.selected_window;
                    let (mut new_state, _) = Self::new();
                    new_state.windows = windows;
                    new_state.selected_window = selected_window;
                    *self = new_state;
                    return Task::none();
                }
                return Task::future(async {
                    let save = rfd::AsyncMessageDialog::new()
                        .set_title("New Project")
                        .set_description("Save changes before creating a new project?")
                        .set_buttons(rfd::MessageButtons::YesNoCancel)
                        .show()
                        .await;
                    match save {
                        rfd::MessageDialogResult::Yes => Message::SaveAndNewProject,
                        rfd::MessageDialogResult::No => Message::NewProjectDiscard,
                        _ => Message::PersistenceError("new project cancelled".to_string()),
                    }
                });
            }

            Message::NewProjectDiscard => {
                let windows = std::mem::take(&mut self.windows);
                let selected_window = self.selected_window;
                let (mut new_state, _) = Self::new();
                new_state.windows = windows;
                new_state.selected_window = selected_window;
                *self = new_state;
                return Task::none();
            }

            Message::SaveAndNewProject => {
                self.pending_new_project = true;
                return Task::done(Message::SaveProject);
            }

            Message::SaveProject => {
                if let Some(path) = self.save_path.clone() {
                    match self.save_to_path(&path) {
                        Ok(()) => {
                            self.unsaved_changes = false;
                        }
                        Err(e) => {
                            eprintln!("Save failed: {e}");
                        }
                    }
                    if self.pending_new_project {
                        return Task::done(Message::NewProjectDiscard);
                    }
                } else {
                    return Task::done(Message::SaveProjectAs);
                }
            }

            Message::SaveProjectAs => {
                return Task::future(async {
                    let handle = rfd::AsyncFileDialog::new()
                        .set_title("Save Project As")
                        .add_filter("Ad UI Project", &["adui"])
                        .save_file()
                        .await;
                    match handle {
                        Some(h) => Message::ProjectSaveReady(h.path().to_path_buf()),
                        None => Message::PersistenceError("Save cancelled".to_string()),
                    }
                });
            }

            Message::ProjectSaveReady(path) => {
                match self.save_to_path(&path) {
                    Ok(()) => {
                        self.save_path = Some(path);
                        self.unsaved_changes = false;
                    }
                    Err(e) => {
                        eprintln!("Save failed: {e}");
                    }
                }
                if self.pending_new_project {
                    return Task::done(Message::NewProjectDiscard);
                }
            }

            Message::LoadProject => {
                return Task::future(async {
                    let handle = rfd::AsyncFileDialog::new()
                        .set_title("Open Project")
                        .add_filter("Ad UI Project", &["adui"])
                        .pick_file()
                        .await;
                    match handle {
                        Some(h) => {
                            let path = h.path().to_path_buf();
                            match persistence::load_project(&path) {
                                Ok(json) => Message::ProjectLoaded(path, json),
                                Err(e) => Message::PersistenceError(e),
                            }
                        }
                        None => Message::PersistenceError("Open cancelled".to_string()),
                    }
                });
            }

            Message::ProjectLoaded(path, json) => match persistence::deserialize_project(&json) {
                Ok(data) => {
                    let windows = std::mem::take(&mut self.windows);
                    *self = Self::from_project_data(data);
                    self.windows = windows;
                    self.save_path = Some(path);
                    self.unsaved_changes = false;
                }
                Err(e) => {
                    eprintln!("Failed to parse project: {e}");
                }
            },

            Message::PersistenceError(msg) => {
                if !msg.ends_with("cancelled") {
                    eprintln!("Persistence error: {msg}");
                }
            }
        }

        Task::none()
    }

    fn view<'a>(&'a self, window_id: window::Id) -> Element<'a, Message> {
        let preview_view = self
            .views
            .get(&self.selected_view_id)
            .expect("Selected view must exist");
        let selected_widget = preview_view
            .hierarchy
            .get_single_selected()
            .unwrap_or(preview_view.hierarchy.root());
        let _selected_window = match self.selected_window {
            Some(selected_window) => self.windows.get(&selected_window),
            None => None,
        };
        let selected_view = self
            .views
            .get(&self.selected_view_id)
            .expect("Failed to get View Id");
        let code_view = full_code_view::view(
            &self.generated_files,
            &self.active_code_tab,
            &self.code_view_content,
        )
        .map(|msg| Message::ViewMessages(ViewMessage::WidgetTree(msg)));

        let view = match self.selected_view {
            navigation_bar::ViewSelection::Main => {
                row![
                    column![
                        // Left Side
                        add_views::view(
                            &self.views,
                            &self.selected_view_id,
                            &self.type_system,
                            &self.theme,
                            &self.custom_styles,
                            &self.widget_preview_content,
                            self.open_editor_widget_id
                        )
                        .map(|msg| Message::ViewMessages(ViewMessage::AddViews(msg))),
                        container(
                            add_widgets::view(&preview_view.hierarchy, &selected_widget.id)
                                .map(|msg| Message::ViewMessages(ViewMessage::AddWidgets(msg)))
                        )
                        .align_bottom(Length::Shrink)
                        .width(400),
                    ]
                    .spacing(5),
                    // Right side
                    code_view
                ]
                .into()
            }
            //             navigation_bar::ViewSelection::Code => {}
            navigation_bar::ViewSelection::ThemeBuilder => self
                .custom_styles
                .view(&self.theme)
                .map(|msg| Message::ViewMessages(ViewMessage::ThemeBuilder(msg))),
            //            navigation_bar::ViewSelection::WidgetStyleBuilder => {}
            navigation_bar::ViewSelection::EnumBuilder => {
                let enum_tab_style = if self.type_editor_tab == TypeEditorTab::Enums {
                    button::primary
                } else {
                    button::secondary
                };
                let struct_tab_style = if self.type_editor_tab == TypeEditorTab::Structs {
                    button::primary
                } else {
                    button::secondary
                };

                let tab_bar = row![
                    button("Enums")
                        .style(enum_tab_style)
                        .on_press(Message::ViewMessages(ViewMessage::SwitchTypeEditorTab(
                            TypeEditorTab::Enums
                        ))),
                    button("Structs")
                        .style(struct_tab_style)
                        .on_press(Message::ViewMessages(ViewMessage::SwitchTypeEditorTab(
                            TypeEditorTab::Structs
                        ))),
                ]
                .spacing(5)
                .padding(5);

                let editor_content: Element<'_, Message> = match self.type_editor_tab {
                    TypeEditorTab::Enums => enum_editor::view(&self.type_system, &self.type_editor)
                        .map(|msg| Message::ViewMessages(ViewMessage::EnumEditor(msg))),
                    TypeEditorTab::Structs => {
                        struct_editor::view(&self.type_system, &self.struct_editor)
                            .map(|msg| Message::ViewMessages(ViewMessage::StructEditor(msg)))
                    }
                };

                row![column![tab_bar, editor_content,], code_view].into()
            }
            navigation_bar::ViewSelection::Settings => {
                row![
                    settings_views::window_settings::view(&self.windows, &self.views, &self.theme)
                        .map(|msg| Message::ViewMessages(ViewMessage::WindowSettings(msg))),
                    // Right side
                    code_view
                ]
                .into()
            }
            navigation_bar::ViewSelection::Actions => action_editor::view(
                &self.views,
                &self.flows,
                &self.action_editor,
                &self.type_system,
            )
            .map(|m| Message::ViewMessages(ViewMessage::ActionEditor(m))),
            _ => container("No done, sorry :(").into(),
        };

        match self.windows.get(&window_id) {
            Some(window) => match window.windowtype {
                WindowEnum::Editor => {
                    let content = if self.selected_view == navigation_bar::ViewSelection::Actions {
                        container(view).height(Length::Fill).width(Length::Fill)
                    } else {
                        container(view)
                            .padding(10)
                            .height(Length::Fill)
                            .width(Length::Fill)
                    };

                    column![
                        navigation_bar::navigation_bar(&self.selected_view, &self.theme).map(
                            |selection| Message::ViewMessages(ViewMessage::NavigationBar(
                                selection
                            ))
                        ),
                        content,
                    ]
                    .into()
                }
                WindowEnum::Visualizer => {
                    let view_id = window.view_to_display(self.selected_view_id);
                    let view_to_render = self
                        .views
                        .get(&view_id)
                        .expect("View assigned to window must exist");

                    preview::view(
                        &view_to_render.hierarchy,
                        &self.theme,
                        &self.custom_styles,
                        selected_view.show_widget_bounds,
                        &self.views,
                        Some(view_to_render.id),
                        &self.type_system,
                    )
                    .map(|msg| Message::ViewMessages(ViewMessage::Preview(msg)))
                }
            },
            None => {
                let content = column![text(format!(
                    "Something has gone terribly wrong. Window Id: {:?}",
                    window_id
                )),];
                container(content).into()
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![event::listen_with(handle_event)])
    }

    fn regenerate_code(&mut self) {
        self.unsaved_changes = true;
        use crate::code_gen_version_two::CodeGeneratorV2;

        let mut generator = CodeGeneratorV2::new(
            &self.views,
            &self.windows,
            &self.theme,
            &self.type_system,
            &self.flows,
        );

        generator.set_app_name(self.app_name.clone());

        self.generated_files = generator.generate_project_structure(&self.custom_styles);
        self.update_code_view_content();
        self.update_widget_preview_content();
    }

    fn update_code_view_content(&mut self) {
        let code = self
            .generated_files
            .get(&self.active_code_tab)
            .or_else(|| self.generated_files.get("main.rs"))
            .cloned()
            .unwrap_or_default();
        self.code_view_content = text_editor::Content::with_text(&code);
    }

    fn update_widget_preview_content(&mut self) {
        use crate::code_gen_version_two::CodeGeneratorV2;

        let view = match self.views.get(&self.selected_view_id) {
            Some(v) => v,
            None => return,
        };
        let selected = view
            .hierarchy
            .get_single_selected()
            .unwrap_or(view.hierarchy.root());
        let mut generator =
            CodeGeneratorV2::new_single(&view.hierarchy, &self.theme, &self.type_system);
        let code = generator.generate_widget_code_rewrite(selected.id, &self.custom_styles);
        self.widget_preview_content = text_editor::Content::with_text(&code);
    }

    fn update_widget_preview_content_for(&mut self, widget_id: WidgetId) {
        use crate::code_gen_version_two::CodeGeneratorV2;

        let view = match self.views.get(&self.selected_view_id) {
            Some(v) => v,
            None => return,
        };
        let mut generator =
            CodeGeneratorV2::new_single(&view.hierarchy, &self.theme, &self.type_system);
        let code = generator.generate_widget_code_rewrite(widget_id, &self.custom_styles);
        self.widget_preview_content = text_editor::Content::with_text(&code);
    }

    fn save_to_path(&self, path: &std::path::Path) -> Result<(), String> {
        use persistence::serde_iced::iced_theme_opt::theme_key_str;
        use persistence::{
            FORMAT_VERSION, ProjectDataRef, SerializableThemesRef, save_project_ref,
        };

        let data = ProjectDataRef {
            format_version: FORMAT_VERSION,
            app_name: &self.app_name,
            flows: &self.flows,
            views: &self.views,
            selected_view_id: self.selected_view_id,
            custom_styles: SerializableThemesRef {
                theme_name: "",
                styles: self.custom_styles.styles(),
            },
            type_system: &self.type_system,
            theme_name: theme_key_str(&self.theme),
        };
        save_project_ref(&data, path)
    }

    fn from_project_data(data: persistence::ProjectData) -> Self {
        use persistence::serde_iced::iced_theme_opt::theme_from_key_str;

        let theme = theme_from_key_str(&data.theme_name).unwrap_or(Theme::Dark);

        // Rebuild transient state in all views.
        let mut views = data.views;
        for view in views.values_mut() {
            view.hierarchy.post_load();
        }

        // App-level flows: use top-level if present, else migrate from views (old saves).
        let mut flows = data.flows;
        if flows.is_empty() {
            for view in views.values_mut() {
                flows.append(&mut view.flows);
            }
        }
        action_system::flow::rebuild_cached_graph_ports(&mut flows);
        // Drop legacy GetState nodes (replaced by ValueSource::StateField).
        for flow in &mut flows {
            let removed: std::collections::HashSet<_> = flow
                .graph
                .nodes
                .iter()
                .filter(|n| {
                    matches!(
                        n.kind,
                        action_system::node_kinds::ActionNodeKind::LegacyGetState { .. }
                    )
                })
                .map(|n| n.id)
                .collect();
            if !removed.is_empty() {
                flow.graph.nodes.retain(|n| !removed.contains(&n.id));
                flow.graph
                    .edges
                    .retain(|e| !removed.contains(&e.from_node) && !removed.contains(&e.to_node));
            }
        }

        // Validate selected_view_id or pick the first view.
        let selected_view_id = if views.contains_key(&data.selected_view_id) {
            data.selected_view_id
        } else {
            views.keys().next().copied().unwrap_or_else(Uuid::nil)
        };

        let mut custom_styles = CustomThemes::new(&theme);
        custom_styles.restore_styles(data.custom_styles.styles);

        let mut builder = Self {
            windows: BTreeMap::new(),
            selected_window: None,
            selected_view: navigation_bar::ViewSelection::Main,
            theme,
            views,
            selected_view_id,
            active_code_tab: "main.rs".to_string(),
            app_name: data.app_name,
            custom_styles,
            type_system: data.type_system,
            type_editor: EnumEditorView::new(),
            struct_editor: StructEditorView::new(),
            type_editor_tab: TypeEditorTab::Enums,
            generated_files: std::collections::HashMap::new(),
            code_view_content: text_editor::Content::new(),
            widget_preview_content: text_editor::Content::new(),
            open_editor_widget_id: None,
            action_editor: views::action_editor::ActionEditorState::default(),
            flows,
            save_path: None,
            unsaved_changes: false,
            pending_new_project: false,
        };
        builder.regenerate_code();
        builder
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum WindowEnum {
    #[default]
    Editor,
    Visualizer,
}

#[derive(Debug, Clone)]
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

fn handle_event(
    event: event::Event,
    _status: event::Status,
    id: iced::window::Id,
) -> Option<Message> {
    match event {
        event::Event::Window(window::Event::Closed) => Some(Message::WindowClosed(id)),
        event::Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. }) => {
            if modifiers.command() {
                match key {
                    keyboard::Key::Character(c) if c.as_str() == "s" => {
                        if modifiers.shift() {
                            Some(Message::SaveProjectAs)
                        } else {
                            Some(Message::SaveProject)
                        }
                    }
                    keyboard::Key::Character(c) if c.as_str() == "o" => Some(Message::LoadProject),
                    _ => None,
                }
            } else {
                None
            }
        }
        _ => None,
    }
}

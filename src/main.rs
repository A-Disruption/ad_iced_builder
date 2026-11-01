use iced::Widget as _;
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

use iced_native_frame::{FrameAction, NativeFrame, NativeFrameConfig};

use action_system::flow::AppFlow;
use data_structures::types::types::{AppView, NativeFrameSettings, WidgetId, WindowConfig};
use views::enum_editor::EnumEditorView;
use views::struct_editor::StructEditorView;
use views::theme_and_stylefn_builder;
use views::theme_and_stylefn_builder::CustomThemes;
use views::*;

fn main() {
    let mut app = iced::daemon(AdUiBuilder::new, AdUiBuilder::update, AdUiBuilder::view)
        .title(AdUiBuilder::title)
        .theme(AdUiBuilder::theme)
        .subscription(AdUiBuilder::subscription);

    // One font per icon family, so the pickers can draw all of them.
    for font in icon::FONTS {
        app = app.font(*font);
    }

    app.run().unwrap()
}

/// How many tree edits Ctrl+Z can walk back through.
///
/// Each entry is one view's serialized hierarchy, so the cost is a few
/// kilobytes per step rather than a whole project.
const UNDO_DEPTH: usize = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TypeEditorTab {
    Enums,
    Structs,
}

struct AdUiBuilder {
    /// The builder's own title bar, drawn rather than asked of the platform.
    ///
    /// This decorates the Editor window only. State is keyed by `window::Id`,
    /// so it and `preview_frame` do not collide.
    frame: NativeFrame,

    /// The title bar the *project* asks for, previewed on the Visualizer.
    ///
    /// The Visualizer stands in for the generated application, so it gets that
    /// application's frame — which means no custom frame at all until the
    /// project turns one on. Rebuilt by `sync_preview_frame` whenever the
    /// settings change.
    preview_frame: NativeFrame,

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

    /// Which branch's `+` is showing the add-widget picker, and what has been
    /// typed into its search box.
    widget_picker: views::widget_picker::PickerState,

    /// The tree row under the cursor, which is the only row that shows its
    /// actions.
    hovered_row: Option<WidgetId>,

    /// Where the inspector's two dividers sit.
    inspector_splits: views::inspector::Splits,

    /// A copied widget subtree, as JSON.
    ///
    /// JSON rather than a `Widget`, because `Widget` is not `Clone` and
    /// pasting twice has to produce two independent trees.
    widget_clipboard: Option<String>,

    /// Widget trees as they were before each edit, for Ctrl+Z.
    ///
    /// One view's hierarchy per entry, serialized, because that is what a tree
    /// edit changes and `WidgetHierarchy` is no more `Clone` than the widgets
    /// in it. Oldest entries are dropped past [`UNDO_DEPTH`].
    undo_stack: Vec<(Uuid, String)>,

    /// The tree row the keyboard is on, reported by the tree.
    ///
    /// What the keyboard shortcuts act on: the cursor may be anywhere, or
    /// nowhere, while you are working from the keyboard.
    focused_row: Option<WidgetId>,

    /// The Main view's pane layout — tree, inspector and code, resizable and
    /// rearrangeable. The live preview takes a pane here once it moves in from
    /// the Visualizer window.
    panes: iced::widget::pane_grid::State<views::editor_panes::PaneKind>,

    #[cfg(feature = "action_editor")]
    action_editor: views::action_editor::ActionEditorState,

    /// App-level flows (moved out of AppView — flows are not view-specific).
    flows: Vec<AppFlow>,

    /// Which flow the action editor shows, and its right-click palette.
    #[cfg(not(feature = "action_editor"))]
    action_editor: action_system::node_graph::EditorState,

    /// The choices the node bodies offer -- state fields, views, flows.
    ///
    /// Held rather than built in `view` because the editor borrows it, and a
    /// local would not outlive the frame. Refreshed by `regenerate_code`,
    /// which already runs on every change that can move these.
    #[cfg(not(feature = "action_editor"))]
    node_context: action_system::node_body::NodeContext,

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
    /// Something the custom title bar did — a drag, a caption button, a focus
    /// change. Forwarded to [`NativeFrame::update`].
    Frame(FrameAction),

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

    /// Ctrl+Shift+A — open the add-widget picker on the focused tree row.
    AddToFocusedRow,
    /// Ctrl+E — put the focused tree row in the inspector.
    EditFocusedRow,
    /// Ctrl+C — take a copy of the focused row and everything under it.
    CopyFocusedRow,
    /// Ctrl+V — paste that copy.
    PasteIntoFocusedRow,
    /// Delete — remove the focused row.
    DeleteFocusedRow,
    /// Ctrl+Z — put the widget tree back the way it was.
    Undo,
    /// Up or Down while the add-widget picker is open.
    PickerHighlight(i32),
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
    /// A pane was resized, dragged or maximised.
    EditorPanes(editor_panes::Message),
    /// One of the inspector's dividers was dragged.
    InspectorSplit(inspector::Divider, f32),
    Preview(preview::Message),
    WindowSettings(settings_views::window_settings::Message),
    /// The action editor reported an edit.
    #[cfg(not(feature = "action_editor"))]
    ActionGraph(action_system::node_graph::Message),
    #[cfg(feature = "action_editor")]
    ActionEditor(action_editor::Message),
}

impl AdUiBuilder {
    fn new() -> (Self, Task<Message>) {
        let view_id = Uuid::new_v4();
        let initial_view = AppView::with_id(view_id, "Main View".to_string(), 0);

        let mut editor = Self {
            frame: NativeFrame::new(NativeFrameConfig::platform_default()),
            preview_frame: NativeFrame::new(
                NativeFrameSettings::default().to_config(false),
            ),
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

            widget_picker: views::widget_picker::PickerState::default(),
            hovered_row: None,
            focused_row: None,
            inspector_splits: views::inspector::Splits::default(),
            widget_clipboard: None,
            undo_stack: Vec::new(),
            panes: views::editor_panes::new_state(),
            #[cfg(feature = "action_editor")]
            action_editor: views::action_editor::ActionEditorState::default(),
            flows: Vec::new(),
            #[cfg(not(feature = "action_editor"))]
            action_editor: Default::default(),
            #[cfg(not(feature = "action_editor"))]
            node_context: Default::default(),

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
                        // The picker's open state belongs to the application,
                        // not the hierarchy, so it never reaches
                        // `widget_tree::update`.
                        let msg = match msg {
                            widget_tree::Message::Picker(picker_msg) => {
                                return self.handle_picker(picker_msg);
                            }
                            other => other,
                        };

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

                        if should_regenerate {
                            self.push_undo();
                        }

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
                        // Every widget tree is drawn inside this panel, so a
                        // tree message arrives wrapped here rather than on
                        // `ViewMessage::WidgetTree` — the add-widget picker's
                        // included.
                        let msg = match msg {
                            add_views::Message::TreeMessage(widget_tree::Message::Picker(
                                picker_msg,
                            )) => {
                                return self.handle_picker(picker_msg);
                            }
                            // Hover is only ever about which row draws its
                            // actions, so it stops here rather than travelling
                            // down into the hierarchy.
                            add_views::Message::TreeMessage(widget_tree::Message::RowEntered(
                                row,
                            )) => {
                                self.hovered_row = Some(row);
                                return Task::none();
                            }
                            add_views::Message::TreeMessage(widget_tree::Message::RowFocused(
                                row,
                            )) => {
                                self.focused_row = row;
                                return Task::none();
                            }
                            add_views::Message::TreeMessage(
                                widget_tree::Message::IconGridScrolled(widget_id, start, snap),
                            ) => {
                                return self.slide_icon_grid(widget_id, start, snap);
                            }
                            add_views::Message::TreeMessage(widget_tree::Message::RowLeft(
                                row,
                            )) => {
                                // Leaving A and entering B publish in no
                                // guaranteed order, so a late exit must not
                                // clear the hover B has already claimed.
                                if self.hovered_row == Some(row) {
                                    self.hovered_row = None;
                                }

                                return Task::none();
                            }
                            other => other,
                        };

                        // Selection drives the inspector, and the inspector
                        // shows a widget's generated code beside its
                        // properties. That code used to be regenerated when the
                        // modal opened; now it follows the selection.
                        if let add_views::Message::TreeMessage(
                            widget_tree::Message::SelectWidgets(ids),
                        ) = &msg
                        {
                            if let (1, Some(&id)) = (ids.len(), ids.iter().next()) {
                                self.update_widget_preview_content_for(WidgetId(id));
                            }
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

                        if should_regenerate {
                            self.push_undo();
                        }

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
                    ViewMessage::EditorPanes(msg) => {
                        editor_panes::update(&mut self.panes, msg);
                        return Task::none();
                    }
                    ViewMessage::InspectorSplit(divider, ratio) => {
                        self.inspector_splits.set(divider, ratio);
                        return Task::none();
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
                    #[cfg(feature = "action_editor")]
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
                    // The action graph owns the flow it is showing, so the
                    // reducer says whether anything generated changed.
                    #[cfg(not(feature = "action_editor"))]
                    ViewMessage::ActionGraph(msg) => {
                        use action_system::node_graph;

                        // The flow bar owns the list; the graph and palette act on
                        // whichever flow it has selected.
                        let changed = match msg {
                            node_graph::Message::Flow(msg) => node_graph::update_flows(
                                &mut self.flows,
                                &mut self.action_editor,
                                msg,
                            ),
                            node_graph::Message::Graph(msg) => self
                                .action_editor
                                .selected_mut_graph(&mut self.flows)
                                .is_some_and(|graph| node_graph::update(graph, msg)),
                            // A trigger edit belongs to the flow, not to the
                            // node that draws it.
                            node_graph::Message::Node(
                                action_system::node_body::NodeMessage::Trigger(msg),
                            ) => self
                                .action_editor
                                .selected_mut(&mut self.flows)
                                .is_some_and(|flow| {
                                    action_system::node_body::update_trigger(flow, msg)
                                }),
                            node_graph::Message::Node(msg) => self
                                .action_editor
                                .selected_mut_graph(&mut self.flows)
                                .is_some_and(|graph| {
                                    action_system::node_body::update(graph, msg)
                                }),
                            node_graph::Message::Palette(msg) => {
                                let palette = &mut self.action_editor.palette;
                                let mut state = std::mem::take(palette);

                                let changed = self
                                    .action_editor
                                    .selected_mut_graph(&mut self.flows)
                                    .is_some_and(|graph| {
                                        node_graph::update_palette(graph, &mut state, msg)
                                    });

                                self.action_editor.palette = state;
                                changed
                            }
                        };

                        if changed {
                            self.regenerate_code();
                        }

                        return Task::none();
                    }
                    ViewMessage::WindowSettings(msg) => {
                        if let settings_views::window_settings::Message::UpdateTheme(theme) = msg {
                            self.theme = theme;
                            self.custom_styles.theme(&self.theme);
                            self.regenerate_code();
                            return Task::none();
                        }

                        let touches_frame = settings_views::window_settings::touches_frame(&msg);

                        let result =
                            settings_views::window_settings::update(&mut self.windows, msg)
                                .map(|m| Message::ViewMessages(ViewMessage::WindowSettings(m)));

                        self.regenerate_code();

                        // The Visualizer previews the project's title bar, so
                        // its frame has to be rebuilt when those settings move.
                        if touches_frame {
                            return result.chain(self.sync_preview_frame());
                        }

                        return result;
                    }
                }
            }

            //window handles
            Message::WindowClosed(window_id) => {
                println!("Close window request, requested.");
                self.windows.remove(&window_id);

                // Both frames keep per-window state of their own; without this
                // they would hold hover and maximization for a window that is
                // gone. Neither knows which windows are the other's, and
                // releasing an id they never had is a no-op.
                let released = Task::batch([
                    self.frame
                        .update(FrameAction::WindowClosed(window_id), Message::Frame),
                    self.preview_frame
                        .update(FrameAction::WindowClosed(window_id), Message::Frame),
                ]);

                if self.windows.is_empty() {
                    return released.chain(iced::exit());
                }

                return released;
            }
            // Actions carry the window they came from, and each frame ignores
            // ids it does not know, so both get a look.
            Message::Frame(action) => {
                return Task::batch([
                    self.frame.update(action.clone(), Message::Frame),
                    self.preview_frame.update(action, Message::Frame),
                ]);
            }
            Message::RequestOpenWindow(window_type) => {
                match window_type {
                    WindowEnum::Editor => {
                        let config = WindowConfig::editor();
                        let (_id, open) =
                            iced::window::open(self.frame.window_settings(config.settings));

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

                        // Create new visualizer. It shows the project's frame,
                        // not the builder's, so its settings come from there.
                        let config = WindowConfig::visualizer();
                        let (_id, open) = iced::window::open(
                            self.preview_frame.window_settings(config.settings.clone()),
                        );

                        return open
                            .map(move |id| Message::WindowOpened(id, WindowEnum::Visualizer));
                    }
                }
            }
            Message::WindowOpened(window_id, window_type) => {
                let new_window = Window::new(window_id, window_type.clone());
                let uses_preview_frame = new_window.config.use_native_frame;
                self.windows.insert(window_id, new_window);

                // Once per window, on the frame that decorates it: on Windows
                // this installs the `WM_NCHITTEST` subclass the Snap Layout
                // flyout needs.
                return match window_type {
                    WindowEnum::Editor => self.frame.install(window_id).discard(),
                    WindowEnum::Visualizer if uses_preview_frame => {
                        self.preview_frame.install(window_id).discard()
                    }
                    WindowEnum::Visualizer => Task::none(),
                };
            }

            Message::NewProject => {
                let has_project = self.unsaved_changes || self.save_path.is_some();
                if !has_project {
                    let windows = std::mem::take(&mut self.windows);
                    let selected_window = self.selected_window;
                    // The windows survive the reset, so the frame that knows
                    // about them has to as well.
                    let frame = self.frame.clone();
                    let preview_frame = self.preview_frame.clone();
                    let (mut new_state, _) = Self::new();
                    new_state.windows = windows;
                    new_state.frame = frame;
                    new_state.preview_frame = preview_frame;
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
                let frame = self.frame.clone();
                let preview_frame = self.preview_frame.clone();
                let (mut new_state, _) = Self::new();
                new_state.windows = windows;
                new_state.frame = frame;
                new_state.preview_frame = preview_frame;
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

            Message::AddToFocusedRow => {
                let Some(row) = self.focused_row else {
                    return Task::none();
                };

                // Ctrl+A is select-all inside a text field, and this listener
                // sees keys wherever they were typed — so it stands aside while
                // the picker's own search box is up.
                if self.widget_picker.is_open_for(row) {
                    return Task::none();
                }

                let view = self
                    .views
                    .get(&self.selected_view_id)
                    .expect("Selected view must exist");

                if add_widgets::get_available_types(&view.hierarchy, &row).is_empty() {
                    return Task::none();
                }

                return self.handle_picker(views::widget_picker::Message::Toggled(row, true));
            }

            Message::EditFocusedRow => {
                let Some(row) = self.focused_row else {
                    return Task::none();
                };

                let view = self
                    .views
                    .get_mut(&self.selected_view_id)
                    .expect("Selected view must exist");

                view.hierarchy
                    .set_selected_ids(std::collections::HashSet::from([row]));

                self.update_widget_preview_content_for(row);

                return Task::none();
            }

            Message::PickerHighlight(delta) => {
                let Some(parent) = self.widget_picker.open_for() else {
                    return Task::none();
                };

                let count = self.picker_matches(parent).len();
                self.widget_picker.move_highlight(delta, count);

                // The highlight is no use off the bottom of the list.
                return iced::widget::operation::scrollable::snap_to(
                    views::widget_picker::LIST_ID,
                    iced::widget::operation::scrollable::RelativeOffset {
                        x: Some(0.0),
                        y: Some(self.widget_picker.scroll_offset(count)),
                    },
                    iced::widget::operation::Animation::Instant,
                );
            }

            Message::DeleteFocusedRow => {
                let Some(row) = self.focused_row else {
                    return Task::none();
                };

                // Delete is a plain key, so it arrives from wherever it was
                // pressed — including a text field the user is editing. Only
                // act when nothing is asking for typed input.
                if self.widget_picker.open_for().is_some() {
                    return Task::none();
                }

                self.push_undo();

                let view = self
                    .views
                    .get_mut(&self.selected_view_id)
                    .expect("Selected view must exist");

                if view.hierarchy.delete_widget(row).is_ok() {
                    self.focused_row = None;
                    self.regenerate_code();
                } else {
                    // Nothing happened, so nothing to undo.
                    self.undo_stack.pop();
                }

                return Task::none();
            }

            Message::Undo => {
                let Some((view_id, json)) = self.undo_stack.pop() else {
                    return Task::none();
                };

                let hierarchy: data_structures::widget_hierarchy::WidgetHierarchy =
                    match serde_json::from_str(&json) {
                        Ok(hierarchy) => hierarchy,
                        Err(error) => {
                            return Task::done(Message::PersistenceError(format!(
                                "Could not undo: {error}"
                            )));
                        }
                    };

                let Some(view) = self.views.get_mut(&view_id) else {
                    return Task::none();
                };

                view.hierarchy = hierarchy;
                // The transient fields — next id, selection — are `serde(skip)`,
                // so a restored tree needs the same repair a loaded one does.
                view.hierarchy.post_load();

                self.selected_view_id = view_id;
                self.focused_row = None;
                self.regenerate_code();

                return Task::none();
            }

            Message::CopyFocusedRow => {
                let Some(row) = self.focused_row else {
                    return Task::none();
                };

                let view = self
                    .views
                    .get(&self.selected_view_id)
                    .expect("Selected view must exist");

                let Some(widget) = view.hierarchy.get_widget_by_id(row) else {
                    return Task::none();
                };

                // A `Widget` is not `Clone` — `Properties` carries a pile of
                // iced types that have no business growing that bound — but the
                // whole tree is `Serialize`, for saving. So the copy is JSON,
                // through the same `serde_iced` helpers a project file uses.
                match serde_json::to_string(widget) {
                    Ok(json) => self.widget_clipboard = Some(json),
                    Err(error) => {
                        return Task::done(Message::PersistenceError(format!(
                            "Could not copy widget: {error}"
                        )));
                    }
                }

                return Task::none();
            }

            Message::PasteIntoFocusedRow => {
                let (Some(row), Some(json)) = (self.focused_row, self.widget_clipboard.clone())
                else {
                    return Task::none();
                };

                let subtree: data_structures::types::types::Widget =
                    match serde_json::from_str(&json) {
                        Ok(widget) => widget,
                        Err(error) => {
                            return Task::done(Message::PersistenceError(format!(
                                "Could not paste widget: {error}"
                            )));
                        }
                    };

                // Taken before the tree changes, and only pushed once the
                // paste actually lands — a refused paste leaves no step to
                // undo.
                let before = self
                    .views
                    .get(&self.selected_view_id)
                    .and_then(|view| serde_json::to_string(&view.hierarchy).ok())
                    .unwrap_or_default();

                let view = self
                    .views
                    .get_mut(&self.selected_view_id)
                    .expect("Selected view must exist");

                // Into the focused row if it will take it, otherwise beside it.
                // Copying a button to paste three more of them means aiming at
                // one of the buttons, not at the row above them.
                let target = if view.hierarchy.can_add_child(row, subtree.widget_type) {
                    let index = view
                        .hierarchy
                        .get_widget_by_id(row)
                        .map_or(0, |widget| widget.children.len());

                    Some((row, index))
                } else {
                    view.hierarchy.find_parent_id(row).map(|parent| {
                        let index = view
                            .hierarchy
                            .get_widget_by_id(parent)
                            .and_then(|widget| {
                                widget.children.iter().position(|child| child.id == row)
                            })
                            .map_or(0, |position| position + 1);

                        (parent, index)
                    })
                };

                let Some((parent, index)) = target else {
                    return Task::none();
                };

                match view.hierarchy.insert_subtree(parent, subtree, index) {
                    Ok(_) => {
                        self.undo_stack.push((self.selected_view_id, before));

                        if self.undo_stack.len() > UNDO_DEPTH {
                            self.undo_stack.remove(0);
                        }

                        self.regenerate_code();
                    }
                    Err(error) => {
                        return Task::done(Message::PersistenceError(error));
                    }
                }

                return Task::none();
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
                    let frame = self.frame.clone();
                    let preview_frame = self.preview_frame.clone();
                    let loaded_window = data.window.clone();

                    *self = Self::from_project_data(data);
                    self.windows = windows;
                    self.frame = frame;
                    self.preview_frame = preview_frame;
                    self.save_path = Some(path);
                    self.unsaved_changes = false;

                    if let Some(config) = loaded_window {
                        self.apply_generated_window_config(config);
                    }

                    // The loaded project may want a different title bar than
                    // the one on screen.
                    return self.sync_preview_frame();
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
            navigation_bar::ViewSelection::Main => self.main_panes(),
            //             navigation_bar::ViewSelection::Code => {}
            navigation_bar::ViewSelection::ThemeBuilder => self
                .custom_styles
                .view(&self.theme)
                .map(|msg| Message::ViewMessages(ViewMessage::ThemeBuilder(msg))).boxed(),
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
                        .map(|msg| Message::ViewMessages(ViewMessage::EnumEditor(msg))).boxed(),
                    TypeEditorTab::Structs => {
                        struct_editor::view(&self.type_system, &self.struct_editor)
                            .map(|msg| Message::ViewMessages(ViewMessage::StructEditor(msg))).boxed()
                    }
                };

                row![column![tab_bar, editor_content,], code_view].boxed()
            }
            navigation_bar::ViewSelection::Settings => {
                row![
                    settings_views::window_settings::view(&self.windows, &self.views, &self.theme)
                        .map(|msg| Message::ViewMessages(ViewMessage::WindowSettings(msg))),
                    // Right side
                    code_view
                ]
                .boxed()
            }
            #[cfg(feature = "action_editor")]
            navigation_bar::ViewSelection::Actions => action_editor::view(
                &self.views,
                &self.flows,
                &self.action_editor,
                &self.type_system,
            )
            .map(|m| Message::ViewMessages(ViewMessage::ActionEditor(m))),

            // The action editor, rebuilt on `iced_nodegraph`. Shows the first
            // flow; the flow browser that picks between them is the next
            // piece, and lives in the parked `action_editor` module.
            #[cfg(not(feature = "action_editor"))]
            navigation_bar::ViewSelection::Actions =>
                action_system::node_graph::editor(
                    &self.flows,
                    &self.action_editor,
                    &self.node_context,
                )
                    .map(|msg| Message::ViewMessages(ViewMessage::ActionGraph(msg))).boxed(),

            _ => container("No done, sorry :(").boxed(),
        };

        let Some(window) = self.windows.get(&window_id) else {
            // No frame here: a window the builder does not know about has no
            // title to draw, and this is the one place that says so.
            return container(column![text(format!(
                "Something has gone terribly wrong. Window Id: {:?}",
                window_id
            ))])
            .boxed();
        };

        let content: Element<'a, Message> = match window.windowtype {
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
                        |selection| Message::ViewMessages(ViewMessage::NavigationBar(selection))
                    ),
                    content,
                ]
                .boxed()
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
                .map(|msg| Message::ViewMessages(ViewMessage::Preview(msg))).boxed()
            }
        };

        match window.windowtype {
            // The builder's own chrome.
            WindowEnum::Editor => {
                self.frame
                    .view(window_id, window.title(), None, None, content, Message::Frame)
            }

            // The project's chrome, or the platform's when the project has not
            // asked for one — the Visualizer is standing in for the generated
            // application, so it shows what that application would show.
            WindowEnum::Visualizer => {
                if !window.config.use_native_frame {
                    return content;
                }

                let title_bar = window
                    .config
                    .title_bar_view_id
                    .and_then(|view_id| self.views.get(&view_id))
                    .map(|title_view| {
                        preview::view(
                            &title_view.hierarchy,
                            &self.theme,
                            &self.custom_styles,
                            false,
                            &self.views,
                            Some(title_view.id),
                            &self.type_system,
                        )
                        .map(|msg| Message::ViewMessages(ViewMessage::Preview(msg)))
                        .boxed()
                    });

                self.preview_frame.view(
                    window_id,
                    &window.config.title,
                    None,
                    title_bar,
                    content,
                    Message::Frame,
                )
            }
        }
    }

    /// Rebuilds the Visualizer's frame from the project's settings.
    ///
    /// Returns the tasks that make the window agree with them: the platform
    /// decorations have to be flipped, and on Windows the `WM_NCHITTEST`
    /// subclass installed or removed.
    fn sync_preview_frame(&mut self) -> Task<Message> {
        let Some((window_id, window)) = self
            .windows
            .iter()
            .find(|(_, window)| window.windowtype == WindowEnum::Visualizer)
        else {
            return Task::none();
        };

        let window_id = *window_id;
        let custom = window.config.use_native_frame;
        let was_custom = self.preview_frame.decoration_mode().uses_custom_frame();
        let config = window.config.native_frame.to_config(custom);

        self.preview_frame = NativeFrame::new(config);

        // Only the mode change needs anything of the window itself; the rest
        // of the config is read afresh every frame.
        if custom == was_custom {
            return Task::none();
        }

        let decorations = window::toggle_decorations(window_id);

        if custom {
            self.preview_frame
                .install(window_id)
                .discard()
                .chain(decorations)
        } else {
            decorations
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            event::listen_with(handle_event),
            self.frame.subscription().map(Message::Frame),
            self.preview_frame.subscription().map(Message::Frame),
        ])
    }

    fn regenerate_code(&mut self) {
        self.unsaved_changes = true;

        // The node bodies pick from these, and every change that can move them
        // also regenerates.
        #[cfg(not(feature = "action_editor"))]
        {
            self.node_context = action_system::node_body::NodeContext::gather(
                &self.views,
                &self.flows,
                action_system::state_ref::collect_state_fields(&self.views),
                &self.type_system,
                self.action_editor.selected(&self.flows),
            );
        }
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

    /// The Main view: tree, inspector and code as resizable panes.
    ///
    /// Each pane's body is built inside the closure rather than passed in,
    /// because `PaneGrid::new` takes an `Fn` and calls it once per pane — an
    /// element built outside could only be handed to one of them.
    fn main_panes<'a>(&'a self) -> Element<'a, Message> {
        use iced::widget::pane_grid::{self, PaneGrid};
        use views::editor_panes::PaneKind;

        let selected_view = self
            .views
            .get(&self.selected_view_id)
            .expect("Selected view must exist");

        let grid = PaneGrid::new(&self.panes, move |pane, kind, is_maximized| {
            let body: Element<'a, Message> = match kind {
                PaneKind::Tree => add_views::view(
                    &self.views,
                    &self.type_system,
                    &self.widget_picker,
                    self.hovered_row,
                )
                .map(|msg| Message::ViewMessages(ViewMessage::AddViews(msg)))
                .boxed(),

                // The inspector scrolls its own properties region, between its
                // own dividers — a scrollable out here would be a second one
                // around the first.
                PaneKind::Inspector => inspector::view(
                    &selected_view.hierarchy,
                    &self.type_system,
                    &self.theme,
                    &self.views,
                    &self.custom_styles,
                    &self.widget_preview_content,
                    selected_view.id,
                    self.inspector_splits,
                )
                .map(|msg| match msg {
                    // A single widget's controls are the same `PropertyChanged`
                    // messages the modal used to send, and they reach the
                    // hierarchy through the tree.
                    inspector::Message::Single(msg) => Message::ViewMessages(
                        ViewMessage::AddViews(add_views::Message::TreeMessage(msg)),
                    ),
                    inspector::Message::Batch(msg) => {
                        Message::ViewMessages(ViewMessage::AddWidgets(msg))
                    }
                    inspector::Message::SplitMoved(divider, ratio) => Message::ViewMessages(
                        ViewMessage::InspectorSplit(divider, ratio),
                    ),
                }).boxed(),

                PaneKind::Code => full_code_view::view(
                    &self.generated_files,
                    &self.active_code_tab,
                    &self.code_view_content,
                )
                .map(|msg| Message::ViewMessages(ViewMessage::WidgetTree(msg))).boxed(),
            };

            // The title bar is also the drag handle: a pane can only be
            // rearranged by something the cursor can grab.
            let title_bar = pane_grid::TitleBar::new(
                text(kind.title()).size(12).style(text::secondary),
            )
            .controls(pane_grid::Controls::new(
                button(text(if is_maximized { "\u{2013}" } else { "\u{2922}" }).size(12))
                    .style(button::text)
                    .padding(2)
                    .on_press(Message::ViewMessages(ViewMessage::EditorPanes(
                        editor_panes::Message::ToggleMaximize(pane),
                    ))),
            ))
            .padding(iced::padding::left(8).right(4).top(2).bottom(2));

            pane_grid::Content::new(body)
                .title_bar(title_bar)
                .style(styles::container::pane)
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .spacing(6)
        .min_size(140.0)
        .on_resize(8.0, |event| {
            Message::ViewMessages(ViewMessage::EditorPanes(
                editor_panes::Message::Resized(event),
            ))
        })
        .on_drag(|event| {
            Message::ViewMessages(ViewMessage::EditorPanes(
                editor_panes::Message::Dragged(event),
            ))
        });

        grid.boxed()
    }

    /// Everything the add-widget picker can report.
    ///
    /// Lives here rather than in `widget_tree::update` because the open flag is
    /// application state, and because adding a child has to regenerate code.
    /// Reachable from two arms — the tree is drawn inside the views panel, so
    /// its messages usually arrive wrapped in `AddViews`.
    fn handle_picker(&mut self, message: views::widget_picker::Message) -> Task<Message> {
        use views::widget_picker::Message as Picker;

        match message {
            // The `+` only needs to look pressable; the popover reports the
            // toggle itself. See `widget_picker::Message::TriggerPressed`.
            Picker::TriggerPressed => {}
            Picker::Toggled(widget_id, true) => {
                self.widget_picker.open(widget_id);

                // Opening on the keyboard is the point: the search box takes
                // the next keystroke.
                return iced::widget::operation::focus(views::widget_picker::SEARCH_INPUT_ID);
            }
            Picker::Toggled(widget_id, false) => {
                self.widget_picker.close(widget_id);
            }
            Picker::QueryChanged(query) => {
                self.widget_picker.set_query(query);
            }
            Picker::Picked(parent_id, widget_type) => {
                return self.add_from_picker(parent_id, widget_type);
            }
            Picker::SubmitHighlighted(parent_id) => {
                // Resolved now rather than when the box was drawn: adding a
                // child can change what the branch will accept next.
                let matches = self.picker_matches(parent_id);

                let Some(&widget_type) = matches.get(self.widget_picker.highlight()) else {
                    return Task::none();
                };

                return self.add_from_picker(parent_id, widget_type);
            }
        }

        Task::none()
    }

    /// Moves the icon grid's window of results, and puts the view back.
    ///
    /// Sliding the window moves every icon on screen, so the scrollable is
    /// nudged back by as much as it shifted — without that the grid jumps, and
    /// a jump at the moment you reach the bottom reads as the list fighting
    /// you. No snap means the window did not move.
    fn slide_icon_grid(
        &mut self,
        widget_id: WidgetId,
        start: usize,
        snap: Option<f32>,
    ) -> Task<Message> {
        let Some(view) = self.views.get_mut(&self.selected_view_id) else {
            return Task::none();
        };

        view.hierarchy.apply_property_change(
            widget_id,
            data_structures::properties::messages::PropertyChange::IconPickerWindow(start),
            &self.type_system,
        );

        let Some(y) = snap else {
            return Task::none();
        };

        iced::widget::operation::scrollable::snap_to(
            iced::advanced::widget::Id::new(controls::ICON_GRID_ID),
            iced::widget::operation::scrollable::RelativeOffset {
                x: Some(0.0),
                y: Some(y),
            },
            iced::widget::operation::Animation::Instant,
        )
    }

    /// Records the selected view's tree as it stands, before something changes
    /// it.
    ///
    /// Called on the way *in* to an edit rather than after it, so the top of
    /// the stack is always the state to go back to. An edit that turns out to
    /// be a no-op pops its own entry again, so Ctrl+Z never appears to do
    /// nothing.
    fn push_undo(&mut self) {
        let Some(view) = self.views.get(&self.selected_view_id) else {
            return;
        };

        let Ok(json) = serde_json::to_string(&view.hierarchy) else {
            return;
        };

        self.undo_stack.push((self.selected_view_id, json));

        if self.undo_stack.len() > UNDO_DEPTH {
            self.undo_stack.remove(0);
        }
    }

    /// What the open picker is currently offering, in the order it draws them.
    fn picker_matches(
        &self,
        parent_id: WidgetId,
    ) -> Vec<data_structures::types::types::WidgetType> {
        let Some(view) = self.views.get(&self.selected_view_id) else {
            return Vec::new();
        };

        let available = add_widgets::get_available_types(&view.hierarchy, &parent_id);

        views::widget_picker::first_matches(&available, self.widget_picker.query())
    }

    /// Adds one child from the picker, leaving the picker open.
    ///
    /// Staying open is the point: adding four buttons to a row should be four
    /// keystrokes, not four trips through the `+`. The search text is kept too,
    /// so typing "button" once and pressing Return three times adds three.
    ///
    /// It closes itself only when the branch is full — a `Container` takes one
    /// child — because the `+` it is anchored to is about to disappear.
    fn add_from_picker(
        &mut self,
        parent_id: WidgetId,
        widget_type: data_structures::types::types::WidgetType,
    ) -> Task<Message> {
        self.push_undo();

        let view = self
            .views
            .get_mut(&self.selected_view_id)
            .expect("Selected view must exist");

        if view.hierarchy.add_child(parent_id, widget_type).is_err() {
            self.undo_stack.pop();
            return Task::none();
        }

        self.regenerate_code();

        let view = self
            .views
            .get(&self.selected_view_id)
            .expect("Selected view must exist");

        if add_widgets::get_available_types(&view.hierarchy, &parent_id).is_empty() {
            self.widget_picker.dismiss();
            return Task::none();
        }

        // A click on a type button took focus out of the search box; put it
        // back so Return keeps adding.
        iced::widget::operation::focus(views::widget_picker::SEARCH_INPUT_ID)
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
            window: self.generated_window_config(),
        };
        save_project_ref(&data, path)
    }

    /// The config describing the window the generated application opens.
    ///
    /// The Visualizer stands in for that application, so its config is the
    /// project's.
    fn generated_window_config(&self) -> Option<&WindowConfig> {
        self.windows
            .values()
            .find(|window| window.windowtype == WindowEnum::Visualizer)
            .map(|window| &window.config)
    }

    /// Puts a loaded project's window config onto the live Visualizer.
    ///
    /// `window::Settings` is not serialized, so the one already on screen is
    /// kept; everything serde carried — the title, the custom title bar and
    /// its settings — comes from the file.
    fn apply_generated_window_config(&mut self, config: WindowConfig) {
        let Some(window) = self
            .windows
            .values_mut()
            .find(|window| window.windowtype == WindowEnum::Visualizer)
        else {
            return;
        };

        let settings = window.config.settings.clone();
        window.config = WindowConfig { settings, ..config };
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
            frame: NativeFrame::new(NativeFrameConfig::platform_default()),
            preview_frame: NativeFrame::new(
                NativeFrameSettings::default().to_config(false),
            ),
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
            widget_picker: views::widget_picker::PickerState::default(),
            hovered_row: None,
            focused_row: None,
            inspector_splits: views::inspector::Splits::default(),
            widget_clipboard: None,
            undo_stack: Vec::new(),
            panes: views::editor_panes::new_state(),
            #[cfg(feature = "action_editor")]
            action_editor: views::action_editor::ActionEditorState::default(),
            flows,
            #[cfg(not(feature = "action_editor"))]
            action_editor: Default::default(),
            #[cfg(not(feature = "action_editor"))]
            node_context: Default::default(),
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
                    // Shifted, because plain Ctrl+A is select-all inside every
                    // text field in the editor and this listener sees keys
                    // wherever they were typed.
                    keyboard::Key::Character(c)
                        if c.as_str().eq_ignore_ascii_case("a") && modifiers.shift() =>
                    {
                        Some(Message::AddToFocusedRow)
                    }
                    keyboard::Key::Character(c) if c.as_str() == "e" => {
                        Some(Message::EditFocusedRow)
                    }
                    keyboard::Key::Character(c) if c.as_str() == "c" => {
                        Some(Message::CopyFocusedRow)
                    }
                    keyboard::Key::Character(c) if c.as_str() == "v" => {
                        Some(Message::PasteIntoFocusedRow)
                    }
                    keyboard::Key::Character(c) if c.as_str() == "z" => Some(Message::Undo),
                    _ => None,
                }
            } else {
                match key {
                    keyboard::Key::Named(keyboard::key::Named::Delete) => {
                        Some(Message::DeleteFocusedRow)
                    }
                    // Only acted on while the picker is open — `update` checks,
                    // because a listener cannot see the application's state.
                    keyboard::Key::Named(keyboard::key::Named::ArrowDown) => {
                        Some(Message::PickerHighlight(1))
                    }
                    keyboard::Key::Named(keyboard::key::Named::ArrowUp) => {
                        Some(Message::PickerHighlight(-1))
                    }
                    _ => None,
                }
            }
        }
        _ => None,
    }
}

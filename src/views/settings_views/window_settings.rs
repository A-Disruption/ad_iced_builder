use crate::Window;
use crate::WindowEnum;
use std::collections::BTreeMap;
use iced::widget::button::secondary;
use iced::{Alignment, Element, Point, Length, Task, window::{self, Position, Level}};
use iced::widget::{button, text, text_input, checkbox, row, column, space, pick_list, scrollable, container};
use widgets::collapsible::Collapsible;
use crate::data_structures::types::types::{AppView, WindowConfig};
use widgets::collapsible::{collapsible, CollapsibleGroup};
use uuid::Uuid;
use crate::icon;

// Message for updating window config
#[derive(Clone, Debug)]
pub enum Message {
    // Basic Settings
    Title(window::Id, String),
    AssignView(window::Id, Option<Uuid>),
    UpdateTheme(iced::Theme),
    
    // Size Settings
    Preset4K(window::Id),
    Preset1440p(window::Id),
    Preset1080p(window::Id),
    Width(window::Id, String),
    Height(window::Id, String),
    MinWidth(window::Id, String),
    MinHeight(window::Id, String),
    MaxWidth(window::Id, String),
    MaxHeight(window::Id, String),
    ClearMinSize(window::Id),
    ClearMaxSize(window::Id),
    
    // Position Settings
    PositionX(window::Id, String),
    PositionY(window::Id, String),
    PositionCentered(window::Id),
    PositionDefault(window::Id),
    
    // Boolean Flags
    Maximized(window::Id, bool),
    Fullscreen(window::Id, bool),
    Visible(window::Id, bool),
    Resizable(window::Id, bool),
    Decorations(window::Id, bool),
    Transparent(window::Id, bool),
    ExitOnCloseRequest(window::Id, bool),
    
    // Window Level
    SetLevel(window::Id, Level),


    // Icon & PlatformSpecific?
}

pub fn update(
    windows: &mut BTreeMap<window::Id, Window>,
    message: Message,
) -> Task<Message> {
    match message {
        // Updated in main
        Message::UpdateTheme(theme) => {}

        // Basic Settings
        Message::Title(id, title) => {
            if let Some(window) = windows.get_mut(&id){
                window.config.title = title;
            }
            
        }
        Message::AssignView(id, view_id) => {
            if let Some(window) = windows.get_mut(&id){
                window.config.assigned_view_id = view_id;
            }
        }
        
        // Size Settings
        Message::Preset4K(id) => {
            if let Some(window) = windows.get_mut(&id){
                let size = iced::Size::new(3840.0, 2160.0);
                 window.config.draft_width = "3840.0".to_string();
                 window.config.draft_height = "2160.0".to_string();
                 window.config.settings.size = size;
                return iced::window::resize(id, size)
            }
        }
        Message::Preset1440p(id) => {
            if let Some(window) = windows.get_mut(&id){
                let size = iced::Size::new(2560.0, 1440.0);
                 window.config.draft_width = "2560.0".to_string();
                 window.config.draft_height = "1440.0".to_string();
                 window.config.settings.size = size;
                return iced::window::resize(id, size)
            }
        }
        Message::Preset1080p(id) => {
            if let Some(window) = windows.get_mut(&id){
                let size = iced::Size::new(1920.0, 1080.0);
                 window.config.draft_width = "1920.0".to_string();
                 window.config.draft_height = "1080.0".to_string();
                 window.config.settings.size = size;
                return iced::window::resize(id, size)
            }
        }
        Message::Width(id, w) => {
            if let Some(window) = windows.get_mut(&id){
                window.config.draft_width = w;
                if let Ok(width) = window.config.draft_width.parse::<f32>() {
                    if width > 0.0 {
                        window.config.settings.size.width = width;
                        return iced::window::resize(id, window.config.settings.size)
                    }
                }
            }
        }
        Message::Height(id, h) => {
            if let Some(window) = windows.get_mut(&id){
                window.config.draft_height = h;
                if let Ok(height) = window.config.draft_height.parse::<f32>() {
                    if height > 0.0 {
                        window.config.settings.size.height = height;
                        return iced::window::resize(id, window.config.settings.size)
                    }
                }
            }
        }
        Message::MinWidth(id, w) => {
            if let Some(window) = windows.get_mut(&id){
                window.config.draft_min_width = w;

                if window.config.draft_min_width.len() == 0 && window.config.settings.min_size.is_some() {
                    if let Some(min_size) = window.config.settings.min_size {
                        if min_size.height == 100.0 && window.config.draft_min_height.len() == 0 {
                            window.config.settings.min_size = None;
                            return iced::window::set_min_size(id, None)                            
                        } else {
                            window.config.settings.min_size = Some(iced::Size::new(100.0, min_size.height));
                            return iced::window::set_min_size(id, window.config.settings.min_size)                            
                        }
                    }
                }

                if let Ok(width) = window.config.draft_min_width.parse::<f32>() {
                    if width > 0.0 {
                        let height = window.config.settings.min_size
                            .map(|s| s.height)
                            .unwrap_or(100.0);
                        window.config.settings.min_size = Some(iced::Size::new(width, height));
                        return iced::window::set_min_size(id, window.config.settings.min_size)
                    }
                }
            }
        }
        Message::MinHeight(id, h) => {
            if let Some(window) = windows.get_mut(&id){
                window.config.draft_min_height = h;

                if window.config.draft_min_height.len() == 0 && window.config.settings.min_size.is_some() {
                    if let Some(min_size) = window.config.settings.min_size {
                        if min_size.width == 100.0 && window.config.draft_min_width.len() == 0 {
                            window.config.settings.min_size = None;
                            return iced::window::set_min_size(id, None)                            
                        } else {
                            window.config.settings.min_size = Some(iced::Size::new( min_size.width, 100.0));
                            return iced::window::set_min_size(id, window.config.settings.min_size)                            
                        }
                    }
                }

                if let Ok(height) = window.config.draft_min_height.parse::<f32>() {
                    if height > 0.0 {
                        let width = window.config.settings.min_size
                            .map(|s| s.width)
                            .unwrap_or(100.0);
                        window.config.settings.min_size = Some(iced::Size::new(width, height));
                        return iced::window::set_min_size(id, window.config.settings.min_size)
                    }
                }
            }
        }
        Message::MaxWidth(id, w) => {
            if let Some(window) = windows.get_mut(&id){
                window.config.draft_max_width = w;

                if window.config.draft_max_width.len() == 0 && window.config.settings.max_size.is_some() {
                    if let Some(max_size) = window.config.settings.max_size {
                        if max_size.height == 2000.0 && window.config.draft_max_height.len() == 0 {
                            window.config.settings.max_size = None;
                            return iced::window::set_max_size(id, None)                            
                        } else {
                            window.config.settings.max_size = Some(iced::Size::new(2000.0, max_size.height));
                            return iced::window::set_max_size(id, window.config.settings.max_size)                            
                        }
                    }
                }

                if let Ok(width) = window.config.draft_max_width.parse::<f32>() {
                    if width > 0.0 {
                        let height = window.config.settings.max_size
                            .map(|s| s.height)
                            .unwrap_or(2000.0);
                        window.config.settings.max_size = Some(iced::Size::new(width, height));
                        return iced::window::set_max_size(id, window.config.settings.max_size)
                    }
                }
            }
        }
        Message::MaxHeight(id, h) => {
            if let Some(window) = windows.get_mut(&id){
                window.config.draft_max_height = h;

                if window.config.draft_max_height.len() == 0 && window.config.settings.max_size.is_some() {
                    if let Some(max_size) = window.config.settings.max_size {
                        if max_size.width == 2000.0 && window.config.draft_max_width.len() == 0 {
                            window.config.settings.max_size = None;
                            return iced::window::set_max_size(id, None)                            
                        } else {
                            window.config.settings.max_size = Some(iced::Size::new(max_size.width, 2000.0));
                            return iced::window::set_max_size(id, window.config.settings.max_size)                            
                        }
                    }
                }

                if let Ok(height) = window.config.draft_max_height.parse::<f32>() {
                    if height > 0.0 {
                        let width = window.config.settings.max_size
                            .map(|s| s.width)
                            .unwrap_or(2000.0);
                        window.config.settings.max_size = Some(iced::Size::new(width, height));
                        return iced::window::set_max_size(id, window.config.settings.max_size)
                    }
                }
            }
        }
        Message::ClearMinSize(id) => {
            if let Some(window) = windows.get_mut(&id){
                window.config.settings.min_size = None;

                window.config.draft_min_width = String::new();
                window.config.draft_min_height = String::new();
            }
        }
        Message::ClearMaxSize(id) => {
            if let Some(window) = windows.get_mut(&id){
                window.config.settings.max_size = None;

                window.config.draft_max_width = String::new();
                window.config.draft_max_height = String::new();
            }
        }
        
        // Position Settings
        Message::PositionX(id, x) => {
            if let Some(window) = windows.get_mut(&id){
                if let Ok(x_pos) = x.parse::<f32>() {
                    let y = match window.config.settings.position {
                        Position::Specific(point) => point.y,
                        _ => 0.0,
                    };
                    window.config.settings.position = Position::Specific(Point::new(x_pos, y));
                }
            }
        }
        Message::PositionY(id, y) => {
            if let Some(window) = windows.get_mut(&id){
                if let Ok(y_pos) = y.parse::<f32>() {
                    let x = match window.config.settings.position {
                        Position::Specific(point) => point.x,
                        _ => 0.0,
                    };
                    window.config.settings.position = Position::Specific(Point::new(x, y_pos));
                }
            }
        }
        Message::PositionCentered(id) => {
            if let Some(window) = windows.get_mut(&id){
                window.config.settings.position = Position::Centered;
            }
        }
        Message::PositionDefault(id) => {
            if let Some(window) = windows.get_mut(&id){
                window.config.settings.position = Position::Default;
            }
        }
        
        // Boolean Flags
        Message::Maximized(id, value) => {
            if let Some(window) = windows.get_mut(&id){
                window.config.settings.maximized = value;
                return iced::window::toggle_maximize(id)
            }
        }
        Message::Fullscreen(id, value) => {
            if let Some(window) = windows.get_mut(&id){
                window.config.settings.fullscreen = value;
                if !window.config.settings.fullscreen {
                    return iced::window::set_mode(id, iced::window::Mode::Fullscreen)
                } else {
                    return iced::window::set_mode(id, iced::window::Mode::Windowed)
                }
            }   
        }
        Message::Visible(id, value) => {
            if let Some(window) = windows.get_mut(&id){
                window.config.settings.visible = value;
                if !window.config.settings.visible {
                    return iced::window::set_mode(id, iced::window::Mode::Hidden)
                } else {
                    return iced::window::set_mode(id, iced::window::Mode::Windowed)
                }
            }
        }
        Message::Resizable(id, value) => {
            if let Some(window) = windows.get_mut(&id){
                window.config.settings.resizable = value;
                return iced::window::set_resizable(id, value);
            }
        }
        Message::Decorations(id, value) => {
            if let Some(window) = windows.get_mut(&id){
                window.config.settings.decorations = value;
                return iced::window::toggle_decorations(id);
            }
        }
        Message::Transparent(id, value) => {
            if let Some(window) = windows.get_mut(&id){
                window.config.settings.transparent = value;
            }
        }
        Message::ExitOnCloseRequest(id, value) => {
            if let Some(window) = windows.get_mut(&id){
                window.config.settings.exit_on_close_request = value;
            }
        }
        
        // Window Level
        Message::SetLevel(id, level) => {
            if let Some(window) = windows.get_mut(&id){
                window.config.settings.level = level;
                return iced::window::set_level(id, level);
            }
        }
    }
    Task::none()
}


pub fn view<'a>(
    windows: &'a BTreeMap<window::Id, Window>,
    views: &'a BTreeMap<Uuid, AppView>,
    theme: &'a iced::Theme,
) -> Element<'a, Message> {
    let mut window_settings = windows.iter().map(
        |(id, window)|{
            let settings = &window.config.settings;
            
            let content = column![
                // Header
                text(format!("{} Settings", window.config.title))
                    .size(24),
                
                // Basic Settings Section
                section_header("Basic Settings"),
                
                labeled_input(
                    "Window Title",
                    &window.config.title,
                    move |title| Message::Title(*id, title)
                ),
                
                labeled_picklist(
                    "Assigned View",
                    views,
                    window.config.assigned_view_id,
                    move |view| Message::AssignView(*id, view)
                ),
                
                // Size Settings Section
                section_header("Size Settings"),

                row![
                    button("4k").on_press(Message::Preset4K(*id)).style(button::secondary),
                    button("1440p").on_press(Message::Preset1440p(*id)).style(button::secondary),
                    button("1080p").on_press(Message::Preset1080p(*id)).style(button::secondary),
                ].spacing(10),
                
                row![
                    labeled_text_input(
                        "Width",
                        &window.config.draft_width,
                        move |width| Message::Width(*id, width)
                    ),
                    labeled_text_input(
                        "Height",
                        &window.config.draft_height,
                        move |height| Message::Height(*id, height)
                    ),
                ]
                .spacing(10),
                
                // Min Size
                row![
                    labeled_text_input(
                        "Min Width",
                        &window.config.draft_min_width,
                        move |width| Message::MinWidth(*id, width)
                    ),
                    labeled_text_input(
                        "Min Height",
                        &window.config.draft_min_height,
                        move |height| Message::MinHeight(*id, height)
                    ),
                    button("Clear")
                        .on_press(Message::ClearMinSize(*id))
                        .padding(5),
                ]
                .spacing(10)
                .align_y(Alignment::End),
                
                // Max Size
                row![
                    labeled_text_input(
                        "Max Width",
                        &window.config.draft_max_width,
                        move |width| Message::MaxWidth(*id, width)
                    ),
                    labeled_text_input(
                        "Max Height",
                        &window.config.draft_max_height,
                        move |height| Message::MaxHeight(*id, height)
                    ),
                    button("Clear")
                        .on_press(Message::ClearMaxSize(*id))
                        .padding(5),
                ]
                .spacing(10)
                .align_y(Alignment::End),
                
                // Position Settings Section
                section_header("Position Settings"),
                
                position_controls(id, settings.position, &window.config.draft_pos_x, &window.config.draft_pos_y),
                
                // Window Behavior Section
                section_header("Window Behavior"),
                
                checkbox(settings.maximized)
                    .label("Start Maximized")
                    .on_toggle(move |b| Message::Maximized(*id, b)),
                    
                checkbox(settings.fullscreen)
                    .label("Start Fullscreen")
                    .on_toggle(move |b| Message::Fullscreen(*id, b)),
                    
                checkbox(settings.visible)
                    .label("Visible")
                    .on_toggle(move |b| Message::Visible(*id, b)),
                    
                checkbox(settings.resizable)
                    .label("Resizable")
                    .on_toggle(move |b| Message::Resizable(*id, b)),
                    
                checkbox(settings.decorations)
                    .label("Show Decorations (title bar, borders)")
                    .on_toggle(move |b| Message::Decorations(*id, b)),
                    
                checkbox(settings.transparent)
                    .label("Transparent Background")
                    .on_toggle(move |b| Message::Transparent(*id, b)),
                    
                checkbox(settings.exit_on_close_request)
                    .label("Exit App on Close")
                    .on_toggle(move |b| Message::ExitOnCloseRequest(*id, b)),
                
                // Window Level Section
                section_header("Window Level"),
                
                pick_list(
                    LevelChoice::ALL,
                    Some(LevelChoice::from(settings.level)),
                    move |level| Message::SetLevel(*id, level.into())
                )
                .placeholder("Select window level")
                .width(Length::Fill),
            ]
            .spacing(15)
            .padding(20);
            
            collapsible(
                window.title(),
                scrollable(content)
                    .height(Length::Fill)
            )
            .expand_icon(icon::collapsed())
            .collapse_icon(icon::expanded())
            .into()

        }).collect::<Vec<_>>();
    window_settings.remove(0);

        column![
            row![
                text("Window Settings").size(18),
                space::horizontal(),
                text("Theme").size(14),
                pick_list(
                    iced::Theme::ALL,
                    Some(theme.clone()),
                    Message::UpdateTheme
                )
                .width(Length::Fixed(200.0))
            ]
            .align_y(Alignment::Center)
            .spacing(10)
            .padding(5),
            
            scrollable(
                CollapsibleGroup::new(
                    window_settings
                ).spacing(5.0)
                .height(Length::Fill)
            )
        ]
        .padding( iced::Padding { top: 0.0, bottom: 10.0, left: 0.0, right: 0.0 })
        .spacing(10)
        .width(400)
        .into()
}

fn section_header<'a>(title: &'a str) -> Element<'a, Message> {
    column![
        space::vertical().height(10),
        text(title).size(18),
        container(space::vertical().height(1))
            .width(Length::Fill)
            .style(|theme: &iced::Theme| {
                container::Style {
                    background: Some(iced::Background::Color(
                        theme.extended_palette().background.strong.color
                    )),
                    ..Default::default()
                }
            }),
        space::vertical().height(5),
    ]
    .into()
}

fn labeled_text_input<'a>(
    label: &'a str,
    value: &'a str,
    on_input: impl Fn(String) -> Message + 'a,
) -> Element<'a, Message> {
    column![
        text(label).size(12),
        text_input("", value)
            .on_input(on_input)
            .padding(5),
    ]
    .spacing(3)
    .width(Length::Fill)
    .into()
}

/// Text input with label (for non-numeric strings)
fn labeled_input<'a>(
    label: &'a str,
    value: &'a str,
    on_input: impl Fn(String) -> Message + 'a,
) -> Element<'a, Message> {
    column![
        text(label).size(14),
        text_input("", value)
            .on_input(on_input)
            .padding(8)
            .width(Length::Fill),
    ]
    .spacing(5)
    .into()
}

fn labeled_picklist<'a>(
    label: &'a str,
    views: &'a BTreeMap<Uuid, AppView>,
    selected: Option<Uuid>,
    on_select: impl Fn(Option<Uuid>) -> Message + 'a,
) -> Element<'a, Message> {
    // Helper struct to wrap view options with names
    #[derive(Debug, Clone, PartialEq)]
    enum ViewOption {
        None,
        View { id: Uuid, name: String },
    }
    
    impl std::fmt::Display for ViewOption {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                ViewOption::None => write!(f, "No View Assigned"),
                ViewOption::View { name, .. } => write!(f, "{}", name),
            }
        }
    }
    
    // Build options list: None first, then all views
    let mut options = vec![ViewOption::None];
    options.extend(views.iter().map(|(id, view)| ViewOption::View {
        id: *id,
        name: view.name.clone(),
    }));
    
    // Determine current selection
    let current_selection = selected
        .and_then(|id| {
            views.get(&id).map(|view| ViewOption::View {
                id,
                name: view.name.clone(),
            })
        })
        .unwrap_or(ViewOption::None);
    
    column![
        text(label).size(14),
        pick_list(
            options,
            Some(current_selection),
            move |option| match option {
                ViewOption::None => on_select(None),
                ViewOption::View { id, .. } => on_select(Some(id)),
            }
        )
        .placeholder("Select view...")
        .width(Length::Fill),
    ]
    .spacing(5)
    .into()
}

/// Position controls with preset buttons
fn position_controls<'a>(id: &'a window::Id, position: Position, x: &'a str, y: &'a str) -> Element<'a, Message> {
    let is_specific = match position {
        Position::SpecificWith(_) | Position::Specific(_) => true,
        _ => false
    };

    column![
        row![
            button("Default")
                .on_press(Message::PositionDefault(*id))
                .padding(8)
                .style(if matches!(position, Position::Default) {
                    button::primary
                } else {
                    button::secondary
                }),
            button("Centered")
                .on_press(Message::PositionCentered(*id))
                .padding(8)
                .style(if matches!(position, Position::Centered) {
                    button::primary
                } else {
                    button::secondary
                }),
        ]
        .spacing(10),
        
        text("Or set specific position:").size(12),
        
        row![
            labeled_text_input(
                "X Position",
                &x,
                move |pos_x| Message::PositionX(*id, pos_x)
            ),
            labeled_text_input(
                "Y Position",
                &y,
                move |pos_y| Message::PositionY(*id, pos_y)
            ),
        ]
        .spacing(10),
        
        if is_specific {
            text("Note: Specific position is relative to the screen")
                .size(11)
                .style(text::secondary)
        } else {
            text("")
        },
    ]
    .spacing(8)
    .into()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq,)]
pub enum LevelChoice {
    Normal,
    AlwaysOnBottom,
    AlwaysOnTop,
}
impl LevelChoice {
    pub const ALL: &'static [Self] = &[
        Self::Normal,
        Self::AlwaysOnBottom,
        Self::AlwaysOnTop
    ];
}
impl std::fmt::Display for LevelChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LevelChoice::Normal => write!(f, "Normal"),
            LevelChoice::AlwaysOnBottom => write!(f, "Always on Bottom"),
            LevelChoice::AlwaysOnTop => write!(f, "Always on Top"),
        }
    }
}

impl From<LevelChoice> for Level {
    fn from(a: LevelChoice) -> Self {
        match a {
            LevelChoice::Normal => Self::Normal,
            LevelChoice::AlwaysOnBottom => Self::AlwaysOnBottom,
            LevelChoice::AlwaysOnTop => Self::AlwaysOnTop,
        }
    }
}
impl From<Level> for LevelChoice {
    fn from(a: Level) -> Self {
        match a {
            Level::Normal => Self::Normal,
            Level::AlwaysOnBottom => Self::AlwaysOnBottom,
            Level::AlwaysOnTop => Self::AlwaysOnTop,
        }
    }
}
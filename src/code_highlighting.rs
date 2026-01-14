use iced::{
    Length, Border, Element, Theme, Task,
    widget::{container},
};

#[derive(Debug, Clone)]
pub enum Message {
    Noop,
}

struct App {
}

impl App {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
            },
            Task::none()
        )
    }

    fn title(&self) -> String {
        String::from("Visualizer")
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Noop => {}
        }
    }

    pub fn view<'a>(&'a self) -> Element<'a, Message> {
        container(text("Empty")).into()
    }
}

pub fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .theme(App::theme)
        .title(App::title)
        .run()
}
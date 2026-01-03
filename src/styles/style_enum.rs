use iced::widget::{button, container};

#[derive(Debug, Clone, Copy)]
pub enum WidgetStyle {
    Container(container::Style),
    Button(button::Style),
}
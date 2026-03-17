use iced::widget::{button, column, container, row, rule, space, tooltip};
use iced::{Element, Length, Theme};

use crate::icon;
use crate::icon_lucide;
use crate::styles;

#[derive(Debug, Clone)]
pub enum Message {
    ActiveView(ViewSelection),
    NewProject,
    OpenProject,
    SaveProject,
    SaveProjectAs,
}

pub fn navigation_bar<'a>(
    selected_view: &'a ViewSelection,
    theme: &'a Theme,
) -> Element<'a, Message> {
    let background = theme
        .extended_palette()
        .background
        .weakest
        .color
        .scale_alpha(0.5);

    container(column![
        row![
            //button(icon::home().center())
            button(icon_lucide::house().center())
                .width(35)
                .style(if *selected_view == ViewSelection::Main {
                    styles::button::selected_text
                } else {
                    button::text
                })
                .on_press(Message::ActiveView(ViewSelection::Main)),
            button(icon_lucide::workflow().center())
                .width(35)
                .style(if *selected_view == ViewSelection::Actions {
                    styles::button::selected_text
                } else {
                    button::text
                })
                .on_press(Message::ActiveView(ViewSelection::Actions)),
            button(icon_lucide::blocks().center())
                .width(35)
                .style(if *selected_view == ViewSelection::EnumBuilder {
                    styles::button::selected_text
                } else {
                    button::text
                })
                .on_press(Message::ActiveView(ViewSelection::EnumBuilder)),
            button(icon_lucide::palette().center())
                .width(35)
                .style(if *selected_view == ViewSelection::ThemeBuilder {
                    styles::button::selected_text
                } else {
                    button::text
                })
                .on_press(Message::ActiveView(ViewSelection::ThemeBuilder)),
            button(icon::code().center())
                .width(35)
                .style(if *selected_view == ViewSelection::Code {
                    styles::button::selected_text
                } else {
                    button::text
                })
                .on_press(Message::ActiveView(ViewSelection::Code)),
            space::horizontal(),
            tooltip(
                button(icon_lucide::file_plus().center())
                    .width(35)
                    .style(button::text)
                    .on_press(Message::NewProject),
                "New project",
                tooltip::Position::Bottom,
            ),
            tooltip(
                button(icon_lucide::folder_open().center())
                    .width(35)
                    .style(button::text)
                    .on_press(Message::OpenProject),
                "Open project",
                tooltip::Position::Bottom,
            ),
            tooltip(
                button(icon_lucide::save().center())
                    .width(35)
                    .style(button::text)
                    .on_press(Message::SaveProject),
                "Save (Ctrl+S)",
                tooltip::Position::Bottom,
            ),
            tooltip(
                button(icon_lucide::save_all().center())
                    .width(35)
                    .style(button::text)
                    .on_press(Message::SaveProjectAs),
                "Save As… (Ctrl+Shift+S)",
                tooltip::Position::Bottom,
            ),
            //button(icon::cog().center())
            button(icon_lucide::settings().center())
                .width(35)
                .style(if *selected_view == ViewSelection::Settings {
                    styles::button::selected_text
                } else {
                    button::text
                })
                .on_press(Message::ActiveView(ViewSelection::Settings)),
        ]
        .spacing(2.5),
        rule::horizontal(1).style(styles::rule::rule_strong),
        rule::horizontal(1).style(styles::rule::rule_weak),
    ])
    .style(move |_| container::Style {
        background: Some(iced::Background::from(background)),
        ..container::Style::default()
    })
    .center_y(Length::Fixed(35.0))
    .width(Length::Fill)
    .into()
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum ViewSelection {
    #[default]
    Main,
    Actions,
    Code,
    ThemeBuilder,
    EnumBuilder,
    Settings,
}

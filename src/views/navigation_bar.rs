use iced::widget::{container, column, row, button, rule, space};
use iced::{Element, Length, Theme};

use crate::icon;
use crate::styles;

#[derive(Debug, Clone)]
pub enum Message {
    ActiveView(ViewSelection),
}

pub fn navigation_bar<'a>(selected_view: &'a ViewSelection, theme: &'a Theme) -> Element<'a, Message> {
    let background = theme.extended_palette().background.weakest.color.scale_alpha(0.5);

    container(
        column![
            row![
                button(icon::home().center())
                    .width(35)
                    .style(
                        if *selected_view == ViewSelection::Main {
                            styles::button::selected_text
                        } else {
                            button::text
                        }
                    )
                    .on_press(Message::ActiveView(ViewSelection::Main)),

                button(icon::global().center())
                    .width(35)
                    .style(
                        if *selected_view == ViewSelection::Settings {
                            styles::button::selected_text
                        } else {
                            button::text
                        }
                    )
                    .on_press(Message::ActiveView(ViewSelection::Settings)),

                button(icon::type_icon().center())
                    .width(35)
                    .style(
                        if *selected_view == ViewSelection::EnumBuilder {
                            styles::button::selected_text
                        } else {
                            button::text
                        }
                    )
                    .on_press(Message::ActiveView(ViewSelection::EnumBuilder)),

                button(icon::theme().center())
                    .width(35)
                    .style(
                        if *selected_view == ViewSelection::ThemeBuilder {
                            styles::button::selected_text
                        } else {
                            button::text
                        }
                    )
                    .on_press(Message::ActiveView(ViewSelection::ThemeBuilder)),

                button(icon::code().center())
                    .width(35)
                    .style(
                        if *selected_view == ViewSelection::Code {
                            styles::button::selected_text
                        } else {
                            button::text
                        }
                    )
                    .on_press(Message::ActiveView(ViewSelection::Code)),

                space::horizontal(),
                button(icon::cog().center())
                    .width(35)
                    .style(
                        if *selected_view == ViewSelection::Settings {
                            styles::button::selected_text
                        } else {
                            button::text
                        }
                    )
                    .on_press(Message::ActiveView(ViewSelection::Settings)),
            ]
            .spacing(2.5),
            rule::horizontal(1).style(styles::rule::rule_strong),
            rule::horizontal(1).style(styles::rule::rule_weak),
        ]

            
    )
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
    Code,
    ThemeBuilder,
    WidgetStyleBuilder,
    EnumBuilder,
    Settings,
}
use iced::{Element, Length, padding};
use iced::widget::{button, text, text_input, checkbox, column, container};
use crate::data_structures::types::types::AppView;
use crate::views::add_views::Message;
use widgets::generic_overlay::overlay_button;
use crate::icon;

pub fn view<'a>(view: &'a AppView) -> Element<'a, Message> {

    let title = column![
        text("Name")
            .width(Length::Fixed(200.0))
            .center(),
        text_input("View Name", &view.name)
            .on_input(|s| Message::RenameView(view.id, s))
            .width(Length::Fixed(200.0)),
    ].spacing(3);

    let settings_view = container(
        column![
            title,
            checkbox(view.show_widget_bounds)
                .label("Enable .explain()")
                .on_toggle(|_| Message::ToggleExplain(view.id))
        ].spacing(15)
        .padding(padding::top(10.0))
    ).center_x(Length::Fill);

    overlay_button(
        icon::cog(),
        format!("{} Settings", &view.name),
        settings_view
    )
    .overlay_padding(5.0)
    .overlay_radius(10.0)
    .overlay_width(400.0)
    .overlay_height(250.0)
    .close_on_click_outside()
    .style(button::text)
    .padding(1.0)
    .into()
}
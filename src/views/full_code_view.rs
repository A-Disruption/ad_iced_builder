use iced::{Alignment, Theme, Element, Length, Padding};
use iced::widget::{column, row, space, tooltip, scrollable, container, button, text, rule};
use crate::icon;
use crate::views::widget_tree::Message;
use crate::code_generator::{build_code_view_with_height, tokens::Token};
use std::collections::HashMap;

pub fn view<'a>(
    // REMOVED: app_name, app_view, window, type_system 
    // ADDED: 
    generated_files: &HashMap<String, Vec<Token>>,
    current_filename: &str,
    theme: &'a Theme, 
) -> Element<'a, Message> {

    // Sort filenames manually for better UX (main.rs first, then types.rs, then alphabetical)
    let mut filenames: Vec<String> = generated_files.keys().cloned().collect();
    filenames.sort_by(|a, b| {
        match (a.as_str(), b.as_str()) {
            ("main.rs", _) => std::cmp::Ordering::Less,
            (_, "main.rs") => std::cmp::Ordering::Greater,
            ("types.rs", _) => std::cmp::Ordering::Less,
            (_, "types.rs") => std::cmp::Ordering::Greater,
            (a, b) => a.cmp(b),
        }
    });

    // Get tokens for current file
    let active_tokens = generated_files.get(current_filename)
        .or_else(|| generated_files.get("main.rs"))
        .unwrap_or_else(|| generated_files.values().next().expect("No files generated"));
    
    let code_string: String = active_tokens.iter().map(|t| t.text.clone()).collect();

    let file_tabs = row(
        filenames.into_iter().map(|filename| {
            let is_selected = &filename == current_filename;
            
            button(text(filename.clone()).size(14))
                .style(if is_selected { button::primary } else { button::text })
                .padding(Padding::new(5.0).horizontal(10.0))
                .on_press(Message::CodeTabSelected(filename))
                .into()
        })
    )
    .spacing(5)
    .align_y(Alignment::Center);
    
    column![
        row![
            scrollable(file_tabs)
                .direction(scrollable::Direction::Horizontal(scrollable::Scrollbar::new()))
                .width(Length::Fill),

            space::horizontal().width(20),

            tooltip(
                button(icon::copy())
                    .style(button::text)
                    .on_press(Message::CopyCode(code_string)),
                text("Copy current file").size(12),
                tooltip::Position::Left
            ),
        ]
        .align_y(Alignment::Center)
        .padding(Padding::new(0.0).horizontal(10.0))
        .height(Length::Shrink),
        
        rule::horizontal(1),
        
        // Code Content
        container(
            scrollable(
                build_code_view_with_height(active_tokens, 0.0, theme)
            )
            .width(Length::Fill)
        )
        .width(Length::Fill)
        .height(Length::Fill),
    ]
    .spacing(10)
    .padding(10)
    .into()
}
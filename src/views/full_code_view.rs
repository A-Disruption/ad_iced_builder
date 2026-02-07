use iced::{Alignment, Element, Length, Padding};
use iced::widget::{column, row, space, tooltip, scrollable, button, text, text_editor, rule};
use std::sync::Arc;
use std::collections::HashMap;
use tree_sitter_highlighter::{TsSettings, TreeSitterIcedHighlighter, code_gen_text_editor_style};
use crate::icon;
use crate::views::widget_tree::Message;
use crate::code_gen_version_two::helpers::internal_overlay;
use widgets::generic_overlay;

pub fn view<'a>(
    generated_files: &HashMap<String, String>,
    current_filename: &str,
    code_content: &'a text_editor::Content,
) -> Element<'a, Message> {

    // Sort filenames for better UX (main.rs, types.rs, then alphabetical)
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

    // Get code string for copy button
    let code_string = generated_files.get(current_filename)
        .or_else(|| generated_files.get("main.rs"))
        .cloned()
        .unwrap_or_default();

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

    let settings = TsSettings {
        text: Arc::<str>::from(code_content.text().as_str()),
    };

    column![
        row![
            scrollable(file_tabs)
                .direction(scrollable::Direction::Horizontal(scrollable::Scrollbar::new()))
                .width(Length::Fill),

            space::horizontal().width(20),
        ]
        .align_y(Alignment::Center)
        .padding(Padding::new(0.0).horizontal(10.0))
        .height(Length::Shrink),

        rule::horizontal(1),

        // Code Content
        internal_overlay(
            text_editor(code_content)
                .highlight_with::<TreeSitterIcedHighlighter>(
                    settings,
                    TreeSitterIcedHighlighter::to_format,
                )
                .on_action(Message::CodeViewEdit)
                .height(Length::Fill)
                .style(code_gen_text_editor_style)
                .size(14.0),

            tooltip(
                button(icon::copy())
                    .style(button::text)
                    .on_press(Message::CopyCode(code_string)),
                text("Copy current file").size(12),
                tooltip::Position::Left
            ),
        )
        .style(button::text)
        .overlay_style(generic_overlay::blank)

    ]
    .spacing(10)
    .padding(10)
    .into()
}

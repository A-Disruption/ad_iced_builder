use iced::widget::{Column, text_editor,};
use iced::{Element, Length, Task, Theme};
use tree_sitter_highlighter::{
    TsSettings, TreeSitterIcedHighlighter, SyntaxHighlight, code_gen_text_editor_style
};
use std::sync::Arc;

pub struct CodeView {
    content: text_editor::Content,
    theme: Theme,
}

#[derive(Debug, Clone)]
pub enum Message {
    Edit(text_editor::Action),
    ChangeTheme(Theme),
}

impl CodeView {
    pub fn new(content: text_editor::Content, theme: Theme) -> Self {
        Self {
            content,
            theme
        }
    }

    pub fn view(state: &CodeView) -> Element<'_, Message> {
        let settings = TsSettings {
            text: Arc::<str>::from(state.content.text()),
        };
        let extended = state.theme.extended_palette();
        let syntax = SyntaxHighlight::generate(extended);

        Column::new()
            .push(
                text_editor(&state.content)
                    .placeholder("Type something here...")
                    .highlight_with::<TreeSitterIcedHighlighter>(
                        settings,
                        TreeSitterIcedHighlighter::to_format,
                    )
                    .on_action(Message::Edit)
                    .height(Length::Fill)
                    .style(code_gen_text_editor_style)
                    .size(14.0),
            )
            .into()
    }

    pub fn update(state: &mut CodeView, message: Message) -> Task<Message>  {
        match message {
            Message::Edit(action) => {
                state.content.perform(action);
            }

            Message::ChangeTheme(theme) => {
                state.theme = theme;
            }
        }
        Task::none()
    }
}
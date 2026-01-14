use iced::widget::{Column, pick_list, text_editor,};
use iced::{Element, Length, Task, Theme};
use iced::font::Font;
use iced::advanced::text::highlighter::Format;
//use iced_custom_highlighter::{Highlight, Highlighter, Settings, Scope};
use testing_rust_knowledge::{Highlight, Highlighter, Settings, Scope};

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
        let custom_scopes = vec![
            Scope::custom("Block", "meta.block.rust"),
            Scope::custom("BlockFunctionBlockPath", "meta.block.rust meta.function.rust meta.block.rust meta.path.rust, meta.function.rust meta.block.rust meta.block.rust meta.path.rust"),
            Scope::custom("MetaPath", "meta.path.rust" ),
            Scope::custom("FunctionBlock", "meta.function.rust meta.block.rust"),
            Scope::custom("MetaPathGroup", "meta.group.rust meta.path.rust"),
            Scope::custom("Punctuation", "punctuation.separator.rust, "),
            Scope::custom("PunctuationBlock", "punctuation.section.group.end.rust, punctuation.section.group.begin.rust, punctuation.section.block.end.rust, punctuation.section.block.begin.rust,"),
            Scope::custom("Accessor", "punctuation.accessor.rust"),
            Scope::custom("EntityName", "entity.name.struct.rust, entity.name.impl.rust, entity.name.enum.rust"),
            Scope::custom("EntityType", "storage.type.enum.rust" ),
            Scope::custom("Annotation", "variable.annotation.rust" ),
            Scope::custom("AnnotationGroup", "meta.group.rust"),
            Scope::custom("AnnotationPunctuation", "punctuation.definition.annotation.rust"),
            Scope::custom("ReturnType", "meta.function.return-type.rust"),
            Scope::custom("GenericRust", "meta.function.return-type.rust meta.generic.rust"), // Task, Message, etc inside a return type
            Scope::custom("GenericRustPunctuation", "punctuation.definition.generic.begin.rust, punctuation.definition.generic.rust"),
            // Add more custom scopes if needed
        ];
        Column::new()
            .push(
                text_editor(&state.content)
                    .placeholder("Type something here...")
                    .highlight_with::<Highlighter>(
//                        Settings::new(vec![], codegen_style, "rs"),
                        Settings::new(custom_scopes, codegen_style /* Highlight::default_style */, "rs"),
                        Highlight::to_format,
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

pub fn codegen_style(theme: &Theme, scope: &Scope) -> Format<Font> {
    let p = theme.extended_palette();

    let primary_danger = iced::theme::palette::mix(p.primary.strong.color, p.danger.strong.color, 0.5);
    let primary_success = iced::theme::palette::mix(p.primary.strong.color, p.success.strong.color, 0.5);
    let primary_success_strong = iced::theme::palette::mix(p.primary.base.color, p.success.strong.color, 0.75);
    let danger_warning_strong = iced::theme::palette::mix(p.danger.strong.color, p.warning.strong.color, 0.35);
    let danger_warning = iced::theme::palette::mix(p.danger.strong.color, p.warning.strong.color, 0.55);

    let color = match scope {
        // Comments
        Scope::Comment => Some(p.success.weak.color),

        // Strings
        Scope::String | Scope::QuotedString => {
            Some(p.success.strong.color)
        }

        // Numbers
        Scope::Number => Some(p.success.strong.color),

        // Keywords-ish (pub/fn/let/etc)
        Scope::Storage
        | Scope::StorageModifier
        | Scope::BuiltinConstant
        | Scope::UserDefinedConstant
        | Scope::Import => Some(p.danger.base.color),

        Scope::EscapeSequence
        | Scope::Exception
        | Scope::SupportConstruct
        | Scope::Continuation => Some(p.primary.strong.color),

        // Operators
        Scope::Operator | Scope::KeywordOperator => Some(danger_warning_strong),

        // “Known” syntect buckets you already have
        Scope::LibraryFunction | Scope::VariableFunction | Scope::LibraryClass => {
            Some(p.primary.strong.color)
        }

        Scope::Class | Scope::Special => Some(p.warning.strong.color),

        Scope::VariableStart | Scope::TagName => Some(p.primary.base.color),

        Scope::Variable=> Some(p.warning.strong.color),

        Scope::Invalid => Some(p.danger.strong.color),

        Scope::Brackets | Scope::Parantheses | Scope::Braces | Scope::TagStart => {
            Some(theme.palette().text)
        }
        Scope::StorageType
            | Scope::KeywordOther
            | Scope::Keyword => Some(primary_danger),
        
        Scope::FunctionName 
            | Scope::RegExp  
            | Scope::QuotedSingle => Some(primary_success_strong),

        Scope::Custom { name, .. } 
            if name == "Block" => {
                Some(primary_danger)
            },
        Scope::Custom { name, .. }
        if name == "MetaPathGroup" || name == "GenericRust" || name == "EntityType" || name == "BlockFunctionBlockPath" => {
            Some(primary_danger)
        }
        Scope::Custom { name, .. } 
            if name == "MetaPath" || name == "PunctuationBlock" || name == "AnnotationGroup" => {
                Some(theme.extended_palette().background.weakest.text)
            },
        Scope::Custom { name, .. } 
            if name == "Accessor" || name == "Punctuation" => {
                Some(theme.extended_palette().secondary.weak.color)
            },
        Scope::Custom { name, .. }
            if name == "AnnotationPunctuation" || name == "GenericRustPunctuation" => {
                Some(primary_success_strong)
            },
        Scope::Custom { name, .. }
            if name == "Annotation" 
                || name == "ReturnType" 
                || name == "FunctionBlock" => {
                Some(p.primary.strong.color)
            },
        Scope::Custom { name, .. }
            if name == "EntityName" => {
                Some(p.warning.strong.color)
            }
        // Then you probably want a fallback for other Custom scopes:
        Scope::Custom { .. } => {
            Some(p.secondary.strong.color) // or whatever default you want
        },

        // Default “plain”
        Scope::Other => {
            Some(p.secondary.strong.color)
        },
        
    };

    let font = Font {
        family: iced::font::Family::Serif,
        stretch: iced::font::Stretch::Normal,
        weight: iced::font::Weight::Normal,
        style: iced::font::Style::Normal,
    };

    Format { color, font: Some(font) }
}

/* pub fn codegen_style(theme: &Theme, scope: &Scope) -> Format<Font> {
    let p = theme.extended_palette();

    let primary_danger = iced::theme::palette::mix(p.primary.strong.color, p.danger.strong.color, 0.5);
    let primary_success = iced::theme::palette::mix(p.primary.strong.color, p.success.strong.color, 0.5);
    let primary_success_strong = iced::theme::palette::mix(p.primary.base.color, p.success.strong.color, 0.75);

    let color = match scope {
        // Comments
        Scope::Comment => Some(p.success.weak.color),

        // Strings
        Scope::String | Scope::RegExp | Scope::QuotedString | Scope::QuotedSingle => {
            Some(p.warning.base.color)
        }

        // Numbers
        Scope::Number | Scope::Parameter | Scope::TypeStart => Some(p.success.strong.color),

        // Keywords-ish (pub/fn/let/etc)
        Scope::Storage
        | Scope::StorageModifier
        | Scope::BuiltinConstant
        | Scope::UserDefinedConstant
        | Scope::Import => Some(p.danger.base.color),

        Scope::EscapeSequence
        | Scope::Exception
        | Scope::SupportConstruct
        | Scope::Continuation => Some(p.primary.strong.color),

        // Operators
        Scope::Operator | Scope::KeywordOperator => Some(p.warning.base.color),

        // “Known” syntect buckets you already have
        Scope::FunctionCall | Scope::LibraryFunction | Scope::VariableFunction | Scope::LibraryClass => {
            Some(p.primary.strong.color)
        }

        Scope::Class | Scope::Special => Some(p.warning.strong.color),

        Scope::VariableStart | Scope::TagName => Some(p.primary.base.color),

        Scope::Field | Scope::Variable | Scope::Path | Scope::FunctionPath => Some(p.background.weakest.text),

        Scope::Invalid => Some(p.danger.strong.color),

        Scope::Method => Some(p.background.strongest.text),

        Scope::FunctionStart => Some(primary_success),

        // Braces/brackets/parens *after you update scope_str() to match punctuation.section.*
        Scope::Brackets | Scope::Parantheses | Scope::Braces | Scope::TagStart => {
            Some(theme.palette().text)
        }

        Scope::Carrots => {
            Some(primary_success_strong)
        }

        Scope::SectionPunctuation => Some(theme.palette().text),
        Scope::ReturnType
            | Scope::StorageType
            | Scope::Constructor
            | Scope::KeywordOther
            | Scope::Keyword => Some(primary_danger),

        // Default “plain”
        Scope::Other | Scope::Custom { .. } | Scope::Punctuation => Some(p.secondary.strong.color),
    };

    let font = Font {
        family: iced::font::Family::Serif,
        stretch: iced::font::Stretch::Normal,
        weight: iced::font::Weight::Normal,
        style: iced::font::Style::Normal,
    };

    Format { color, font: Some(font) }
} */

pub fn code_gen_text_editor_style(theme: &Theme, status: text_editor::Status) -> text_editor::Style {
    let palette = theme.extended_palette();

    let active = text_editor::Style {
        background: iced::Background::Color(palette.background.weakest.color),
        border: iced::Border {
            radius: 4.0.into(),
            width: 1.0,
            color: palette.background.weak.color,
        },
        placeholder: palette.secondary.base.color,
        value: palette.background.base.text,
        selection: palette.primary.weak.color,
    };

    match status {
        text_editor::Status::Active => active,
        text_editor::Status::Hovered => text_editor::Style {
            border: iced::Border {
                color: palette.background.base.text,
                ..active.border
            },
            ..active
        },
        text_editor::Status::Focused { .. } => text_editor::Style {
            border: iced::Border {
                color: palette.primary.strong.color,
                ..active.border
            },
            ..active
        },
        text_editor::Status::Disabled => text_editor::Style {
            background: iced::Background::Color(palette.background.weakest.color),
            value: active.placeholder,
            placeholder: palette.background.strongest.color,
            ..active
        },
    }
}

/* pub fn test(theme: &Theme) -> Style {
    let palette = theme.extended_palette();

    Style {
        text_color: Some(Color::from_rgba(0.9, 0.9, 0.9, 1.0)),
        background: Some(Background::Color(Color::from_rgba(0.2, 0.2, 0.2, 1.0))),
        border: Border {
            color: Color::from_rgba(0.3, 0.3, 0.4, 1.0),
            width: 0.0,
            radius: Radius {
                top_left: 0.0,
                top_right: 0.0,
                bottom_right: 0.0,
                bottom_left: 0.0,
            },
        },
        shadow: Shadow::default(),
        snap: true,
    }
} */
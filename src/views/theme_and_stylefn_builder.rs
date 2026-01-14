use iced::widget::{button, checkbox, column, container, slider, row, scrollable, text, text_input, tooltip, Space};
use iced::{Alignment, Background, Border, Color, Element, Length, Shadow, Theme, Padding, Task,};
use std::collections::BTreeMap;
use widgets::{color_picker, collapsible::collapsible};
use widgets::generic_overlay;
use iced::clipboard;
use crate::code_generator::{generate_container_style_tokens, build_code_view_with_height_generic, internal_overlay};
use crate::{icon, styles};
use crate::styles::style_enum::{WidgetStyle, SavedStyleDefinition, evaluate_theme_expression};


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThemePaneEnum {
    ExtendedPalette,
    Button,
    Checkbox,
    Combobox,
    Container,
    Panegrid,
    Picklist,
    Progressbar,
    QRCode,
    Radio,
    Rule,
    Slider,
    Table,
    TextEditor,
    TextInput,
    Toggler,
}

impl std::fmt::Display for ThemePaneEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ThemePaneEnum::ExtendedPalette  =>  "ExtendedPalette",   
            ThemePaneEnum::Button           =>  "Button",
            ThemePaneEnum::Checkbox         =>  "Checkbox",
            ThemePaneEnum::Combobox         =>  "Combobox",
            ThemePaneEnum::Container        =>  "Container",
            ThemePaneEnum::Panegrid         =>  "Panegrid",
            ThemePaneEnum::Picklist         =>  "Picklist",
            ThemePaneEnum::Progressbar      =>  "Progressbar",
            ThemePaneEnum::QRCode           =>  "QRCode",
            ThemePaneEnum::Radio            =>  "Radio",
            ThemePaneEnum::Rule             =>  "Rule",
            ThemePaneEnum::Slider           =>  "Slider",
            ThemePaneEnum::Table            =>  "Table",
            ThemePaneEnum::TextEditor       =>  "TextEditor",
            ThemePaneEnum::TextInput        =>  "TextInput",
            ThemePaneEnum::Toggler          =>  "Toggler",
        })
    }
}

/// StyleFn Builders

/// Function to build a custom container style in app
fn container_stylefn_builder(text_color: Color, background: iced::Background, border: iced::Border, shadow: iced::Shadow, snap: bool) -> iced::widget::container::Style {
    iced::widget::container::Style {
        text_color: Some(text_color),
        background: Some(background),
        border: border,
        shadow: shadow,
        snap
    }
}

fn button_stylefn_builder(text_color: Color, background: iced::Background, border: iced::Border, shadow: iced::Shadow, snap: bool) -> iced::widget::button::Style {
    iced::widget::button::Style {
        text_color: text_color,
        background: Some(background),
        border: border,
        shadow: shadow,
        snap
    }
}

pub struct CustomThemes {
    pub theme: Theme,
    selected_view: ThemePaneEnum,
    style_name: String,
    styles: BTreeMap<ThemePaneEnum, BTreeMap<String, SavedStyleDefinition>>,
    text_color: Color,
    border_color: Color,
    border_width: f32,
    border_radius_top_left: f32,
    border_radius_top_right: f32,
    border_radius_bottom_right: f32,
    border_radius_bottom_left: f32,
    background_color: Color,
    shadow_enabled: bool,
    shadow_color: Color,
    shadow_offset_x: f32,
    shadow_offset_y: f32,
    shadow_blur_radius: f32,
    snap: bool,

    // source tracking for palette reference in color_picker
    text_color_source: Option<String>,
    border_color_source: Option<String>,
    background_color_source: Option<String>,
    shadow_color_source: Option<String>,
}

impl CustomThemes {
    pub fn new(theme: &Theme) -> Self {
        let palette = theme.extended_palette();
        let mut styles = BTreeMap::new();
        styles.insert(ThemePaneEnum::Container, BTreeMap::new());
        styles.insert(ThemePaneEnum::Button, BTreeMap::new());

        Self {
            theme: theme.clone(),
            selected_view: ThemePaneEnum::Container,
            style_name: String::new(),
            styles,
            text_color: palette.background.base.text,
            border_color: palette.background.strong.color,
            border_width: 0.0,
            border_radius_top_left: 0.0,
            border_radius_top_right: 0.0,
            border_radius_bottom_right: 0.0,
            border_radius_bottom_left: 0.0,
            background_color: palette.background.base.color,
            shadow_enabled: false,
            shadow_color: palette.background.weak.color,
            shadow_offset_x: 0.0,
            shadow_offset_y: 0.0,
            shadow_blur_radius: 0.0,
            snap: true,
            text_color_source: None,
            border_color_source: None,
            background_color_source: None,
            shadow_color_source: None,
        }
    }

    pub fn theme(&mut self, theme: &Theme) {
        self.theme = theme.clone();
        self.reset_to_theme();
    }

    fn reset_to_theme(&mut self) {
        let palette = self.theme.extended_palette();

        self.style_name = String::new();
        self.text_color = palette.background.base.text;
        self.border_color = palette.background.strong.color;
        self.border_width = 0.0;
        self.border_radius_top_left = 0.0;
        self.border_radius_top_right = 0.0;
        self.border_radius_bottom_right = 0.0;
        self.border_radius_bottom_left = 0.0;
        self.background_color = palette.background.base.color;
        self.shadow_enabled = false;
        self.shadow_color = palette.background.weak.color;
        self.shadow_offset_x = 0.0;
        self.shadow_offset_y = 0.0;
        self.shadow_blur_radius = 0.0;
        self.text_color_source = None;
        self.border_color_source = None;
        self.background_color_source = None;
        self.shadow_color_source = None;
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ChangeView(view) => {
                self.selected_view = view;
                self.reset_style_editor(view);
            }
            Message::CopyCode(code) => {
                return clipboard::write(code)
            }
            Message::UpdateStyleName(name) => {
                self.style_name = name;
            }
            Message::UpdateTextColor { color, source } => {
                self.text_color = color;
                self.text_color_source = source;
            }
            Message::UpdateBorderColor { color, source } => {
                self.border_color = color;
                self.border_color_source = source;
            }
            Message::UpdateBorderWidth(width) => self.border_width = width,
            Message::UpdateBorderRadiusTopLeft(radius) => self.border_radius_top_left = radius,
            Message::UpdateBorderRadiusTopRight(radius) => self.border_radius_top_right = radius,
            Message::UpdateBorderRadiusBottomRight(radius) => self.border_radius_bottom_right = radius,
            Message::UpdateBorderRadiusBottomLeft(radius) => self.border_radius_bottom_left = radius,
            Message::UpdateBackgroundColor { color, source } => {
                self.background_color = color;
                self.background_color_source = source;
            }
            Message::UpdateShadowEnabled(enabled) => self.shadow_enabled = enabled,
            Message::UpdateShadowColor { color, source } => {
                self.shadow_color = color;
                self.shadow_color_source = source;
            }
            Message::UpdateShadowOffsetX(x) => self.shadow_offset_x = x,
            Message::UpdateShadowOffsetY(y) => self.shadow_offset_y = y,
            Message::UpdateShadowBlurRadius(blur_radius) => self.shadow_blur_radius = blur_radius,
            Message::UpdateSnap(enabled) => self.snap = enabled,
            
            Message::SaveStyle => {
                if !self.style_name.is_empty() {
                    let definition = SavedStyleDefinition {
                        name: self.style_name.clone(),
                        widget_type: self.selected_view,
                        text_color: self.text_color,
                        text_color_source: self.text_color_source.clone(),
                        background_color: self.background_color,
                        background_color_source: self.background_color_source.clone(),
                        border_color: self.border_color,
                        border_color_source: self.border_color_source.clone(),
                        border_width: self.border_width,
                        border_radius_top_left: self.border_radius_top_left,
                        border_radius_top_right: self.border_radius_top_right,
                        border_radius_bottom_right: self.border_radius_bottom_right,
                        border_radius_bottom_left: self.border_radius_bottom_left,
                        shadow_enabled: self.shadow_enabled,
                        shadow_color: self.shadow_color,
                        shadow_color_source: self.shadow_color_source.clone(),
                        shadow_offset_x: self.shadow_offset_x,
                        shadow_offset_y: self.shadow_offset_y,
                        shadow_blur_radius: self.shadow_blur_radius,
                        snap: self.snap,
                    };

                    self.styles
                        .entry(self.selected_view)
                        .or_default()
                        .insert(self.style_name.clone(), definition);

                    self.style_name.clear();

                }
            }

            Message::SelectStyle(name) => {
                if let Some(style_map) = self.styles.get(&self.selected_view) {
                    if let Some(definition) = style_map.get(&name) {
                        self.style_name = definition.name.clone();
                        self.border_width = definition.border_width;
                        self.border_radius_top_left = definition.border_radius_top_left;
                        self.border_radius_top_right = definition.border_radius_top_right;
                        self.border_radius_bottom_right = definition.border_radius_bottom_right;
                        self.border_radius_bottom_left = definition.border_radius_bottom_left;
                        self.shadow_enabled = definition.shadow_enabled;
                        self.shadow_offset_x = definition.shadow_offset_x;
                        self.shadow_offset_y = definition.shadow_offset_y;
                        self.shadow_blur_radius = definition.shadow_blur_radius;
                        self.snap = definition.snap;

                        match &definition.text_color_source {
                            Some(expression) => {
                                self.text_color = evaluate_theme_expression(&self.theme, &expression).unwrap();
                                self.text_color_source = definition.text_color_source.clone();
                                }
                            None => {
                                self.text_color = definition.text_color;
                                self.text_color_source = None;
                            }
                        }
                        match &definition.background_color_source {
                            Some(expression) => {
                                self.background_color = evaluate_theme_expression(&self.theme, &expression).unwrap();
                                self.background_color_source = definition.background_color_source.clone();
                                }
                            None => {
                                self.background_color = definition.background_color;
                                self.background_color_source = None;
                            }
                        }
                        match &definition.border_color_source {
                            Some(expression) => {
                                self.border_color = evaluate_theme_expression(&self.theme, &expression).unwrap();
                                self.border_color_source = definition.border_color_source.clone();
                                }
                            None => {
                                self.border_color = definition.border_color;
                                self.border_color_source = None;
                            }
                        }
                        match &definition.shadow_color_source {
                            Some(expression) => {
                                self.shadow_color = evaluate_theme_expression(&self.theme, &expression).unwrap();
                                self.shadow_color_source = definition.shadow_color_source.clone();
                                }
                            None => {
                                self.shadow_color = definition.shadow_color;
                                self.shadow_color_source = None;
                            }
                        }
                    }
                }


            }

            Message::ResetToDefault => {
                self.reset_to_theme();
            }
        }
        Task::none()
    }

    pub fn view<'a>(&'a self, theme: &'a Theme) -> Element<'a, Message> {
        let content = match self.selected_view {
            ThemePaneEnum::ExtendedPalette => self.show_theme_colors(theme),
            ThemePaneEnum::Container => self.show_style_builder(theme),
            ThemePaneEnum::Button => self.show_style_builder(theme),
            _ => self.show_style_builder(theme),
        };

        column![
            container(
                text("Themes and Styles").size(20),
            ).center_x(Length::Fill),

            row![
                button("Palette Viewer")
                    .style(
                        if self.selected_view == ThemePaneEnum::ExtendedPalette {
                            styles::button::selected_text
                        } else { button::text }
                    )
                    .on_press(Message::ChangeView(ThemePaneEnum::ExtendedPalette)),
                button("Button")
                    .style(
                        if self.selected_view == ThemePaneEnum::Button {
                            styles::button::selected_text
                        } else { button::text }
                    )
                    .on_press(Message::ChangeView(ThemePaneEnum::Button)),
                button("Checkbox")
                    .style(
                        if self.selected_view == ThemePaneEnum::Checkbox {
                            styles::button::selected_text
                        } else { button::text }
                    )                
                    , //.on_press(Message::ChangeView(ThemePaneEnum::Checkbox)),
                button("Combo Box")
                    .style(
                        if self.selected_view == ThemePaneEnum::Combobox {
                            styles::button::selected_text
                        } else { button::text }
                    )
                    , //.on_press(Message::ChangeView(ThemePaneEnum::Combobox)),
                button("Container")
                    .style(
                        if self.selected_view == ThemePaneEnum::Container {
                            styles::button::selected_text
                        } else { button::text }
                    )
                    .on_press(Message::ChangeView(ThemePaneEnum::Container)),
                button("Pane Grid")
                    .style(
                        if self.selected_view == ThemePaneEnum::Panegrid {
                            styles::button::selected_text
                        } else { button::text }
                    )
                    , //.on_press(Message::ChangeView(ThemePaneEnum::Panegrid)),
                button("Picklist")
                    .style(
                        if self.selected_view == ThemePaneEnum::Picklist {
                            styles::button::selected_text
                        } else { button::text }
                    )
                    , //.on_press(Message::ChangeView(ThemePaneEnum::Picklist)),
                button("Progressbar")
                    .style(
                        if self.selected_view == ThemePaneEnum::Progressbar {
                            styles::button::selected_text
                        } else { button::text }
                    )
                    , //.on_press(Message::ChangeView(ThemePaneEnum::Progressbar)),
                button("QR Code")
                    .style(
                        if self.selected_view == ThemePaneEnum::QRCode {
                            styles::button::selected_text
                        } else { button::text }
                    )
                    , //.on_press(Message::ChangeView(ThemePaneEnum::QRCode)),
                button("Radio")
                    .style(
                        if self.selected_view == ThemePaneEnum::Radio {
                            styles::button::selected_text
                        } else { button::text }
                    )
                    , //.on_press(Message::ChangeView(ThemePaneEnum::Radio)),
                button("Rule")
                    .style(
                        if self.selected_view == ThemePaneEnum::Rule {
                            styles::button::selected_text
                        } else { button::text }
                    )
                    , //.on_press(Message::ChangeView(ThemePaneEnum::Rule)),
                button("Slider")
                    .style(
                        if self.selected_view == ThemePaneEnum::Slider {
                            styles::button::selected_text
                        } else { button::text }
                    )
                    , //.on_press(Message::ChangeView(ThemePaneEnum::Slider)),
                button("Table")
                    .style(
                        if self.selected_view == ThemePaneEnum::Table {
                            styles::button::selected_text
                        } else { button::text }
                    )
                    , //.on_press(Message::ChangeView(ThemePaneEnum::Table)),
                button("Text Editor")
                    .style(
                        if self.selected_view == ThemePaneEnum::TextEditor {
                            styles::button::selected_text
                        } else { button::text }
                    )
                    , //.on_press(Message::ChangeView(ThemePaneEnum::TextEditor)),
                button("Text Input")
                    .style(
                        if self.selected_view == ThemePaneEnum::TextInput {
                            styles::button::selected_text
                        } else { button::text }
                    )
                    , //.on_press(Message::ChangeView(ThemePaneEnum::TextInput)),
                button("Toggler")
                    .style(
                        if self.selected_view == ThemePaneEnum::Toggler {
                            styles::button::selected_text
                        } else { button::text }
                    )
                    , //.on_press(Message::ChangeView(ThemePaneEnum::Toggler)),
            ].spacing(10).wrap(),

            content
        ].spacing(10).into()
        
    }

    pub fn show_style_builder<'a>(&'a self, theme: &'a Theme) -> Element<'a, Message> {

        let content = column![
            container(text(format!("{} Style", self.selected_view)).size(20)).center_x(Length::Fill),
            row![
                column![
                    container(text("text_color").size(16)).center_x(Length::Fill),
                    color_picker::ColorButton::new(self.text_color)
                        .on_change_with_source(|color, source| Message::UpdateTextColor { color, source })
                        .title("text_color")
                        .width(Length::Fill)
                        .height(Length::Fixed(50.0))
                        .show_hex(),
                ]
                .width(Length::FillPortion(1)),

                column![
                    container(text("background color").size(16)).center_x(Length::Fill),
                    color_picker::ColorButton::new(self.background_color)
                        .on_change_with_source(|color, source| Message::UpdateBackgroundColor { color, source })
                        .title("background color")
                        .width(Length::Fill)
                        .height(Length::Fixed(50.0))
                        .show_hex(),
                ]
                .width(Length::FillPortion(1)),
            ].spacing(10),

            column![
                container(text("Border").size(20)).center_x(Length::Fill),

                row![
                    column![
                        text("Width:").size(16),
                        slider(0.0..=30.0, self.border_width, Message::UpdateBorderWidth)
                        .step(1.0),
                        text(format!("{:.0}", self.border_width)).size(12).center(),
                    ].width(Length::FillPortion(1)).align_x(Alignment::Center),


                    column![
                        container(text("border color").size(16)).center_x(Length::Fill),
                        color_picker::ColorButton::new(self.border_color)
                            .on_change_with_source(|color, source| Message::UpdateBorderColor { color, source })
                            .title("border color")
                            .width(Length::Fill)
                            .height(Length::Fixed(50.0))
                            .show_hex(),
                    ]
                    .width(Length::FillPortion(1)),
                ].spacing(10).align_y(Alignment::Center),

                container(text("border radius").size(18)).center_x(Length::Fill),
                row![
                    column![
                        text("Top left").size(16),

                        slider(0.0..=30.0, self.border_radius_top_left, Message::UpdateBorderRadiusTopLeft)
                        .step(1.0),
                        text(format!("{:.0}", self.border_radius_top_left)).size(12).center(),

                    ].spacing(5),

                    column![
                        text("Top right").size(16),
                        slider(0.0..=30.0, self.border_radius_top_right, Message::UpdateBorderRadiusTopRight)
                        .step(1.0),
                        text(format!("{:.0}", self.border_radius_top_right)).size(12).center(),
                    ].spacing(5),

                ].spacing(10),

                row![
                    column![
                        text("Bottom left").size(16),
                        slider(0.0..=30.0, self.border_radius_bottom_left, Message::UpdateBorderRadiusBottomLeft)
                        .step(1.0),
                        text(format!("{:.0}", self.border_radius_bottom_left)).size(12).center(),
                    ].spacing(5),

                    column![
                        text("Bottom right").size(16),
                        slider(0.0..=30.0, self.border_radius_bottom_right, Message::UpdateBorderRadiusBottomRight)
                        .step(1.0),
                        text(format!("{:.0}", self.border_radius_bottom_right)).size(12).center(),
                    ].spacing(5),

                ].spacing(10),

            ].spacing(10),

            // Shadow
            column![
                container(text("Shadow").size(20)).center_x(Length::Fill),
                
                row![
                    column![
                        Space::new().width(Length::Fill).height(Length::Fixed(10.0)),
                        checkbox(self.shadow_enabled)
                            .on_toggle(Message::UpdateShadowEnabled)
                            .label("Enable Shadow"),
                    ].width(Length::FillPortion(1)).height(Length::Fixed(50.0)),

                    if self.shadow_enabled {
                        column![
                            container(text("shadow color").size(16)).center_x(Length::Fill),

                            color_picker::ColorButton::new(self.shadow_color)
                                .on_change_with_source(|color, source| Message::UpdateShadowColor { color, source })
                                .title("shadow color")
                                .width(Length::Fill)
                                .height(Length::Fixed(50.0))
                                .show_hex()
                        ]
                        .width(Length::FillPortion(1))
                    } else { column![].width(Length::FillPortion(1)) }
                ].align_y(Alignment::Start),



                if self.shadow_enabled {
                    column![
                        row![
                            column![
                                text("Offset X").size(12),
                                slider(-20.0..=20.0, self.shadow_offset_x, Message::UpdateShadowOffsetX)
                                .step(1.0),
                                text(format!("{:.0}", self.shadow_offset_x)).size(12).center(),
                            ],
                            column![
                                text("Offset Y").size(12),
                                slider(-20.0..=20.0, self.shadow_offset_y, Message::UpdateShadowOffsetY)
                                .step(1.0),
                                text(format!("{:.0}", self.shadow_offset_y)).size(12).center(),
                            ],
                        ]
                        .spacing(15),
                        column![
                            text("Blur Radius").size(12),
                            slider(0.0..=50.0, self.shadow_blur_radius, Message::UpdateShadowBlurRadius)
                            .step(1.0),
                            text(format!("{:.0}", self.shadow_blur_radius)).size(12).center(),
                        ],
                    ]
                    .spacing(10)
                } else {
                    column![]
                },

                // Snap
                column![
                    container(text("Snap").size(20)).center_x(Length::Fill),
                    checkbox(self.snap)
                        .label("Enable Snap")
                        .on_toggle(Message::UpdateSnap),
                ].spacing(10),
            ]
            .spacing(10),
        ]
        .spacing(15)
        .padding(15);

        let saved_styles_list: Element<_> = {

            if let Some(style_map) = self.styles.get(&self.selected_view) {
                    if !style_map.is_empty() {

                    }
            }

            let styles_for_view = self
                .styles
                .get(&self.selected_view);

            if styles_for_view.is_none() {
                container(text("No styles saved for this widget yet.").center()).into()
            } else {
                let list_content = column(
                    styles_for_view
                        .unwrap()
                        .iter()
                        .map(|(name, definition)| {
                            let preview_style = match definition.widget_type {
                                ThemePaneEnum::Container => definition.to_container_style(&theme),
                                ThemePaneEnum::Button => {
                                    let button_style = definition.to_button_style(&theme);
                                    // Convert to container style for preview
                                    container::Style {
                                        text_color: Some(button_style.text_color),
                                        background: button_style.background,
                                        border: button_style.border,
                                        shadow: button_style.shadow,
                                        snap: button_style.snap,
                                    }
                                }
                                _ => container::Style::default(),
                            };

                            button(
                                container(
                                    text(name)
                                        .size(12)
                                        .center()
                                )
                                .center(Length::Fill)
                                .width(Length::Fill)
                                .height(Length::Fixed(30.0))
                                .style(move |_: &Theme| preview_style)
                            )
                            .style(button::text)
                            .width(Length::Fill)
                            .on_press(Message::SelectStyle(name.clone()))
                            .into()
                        })
                        .collect::<Vec<_>>(),
                )
                .padding(10)
                .spacing(5);
                
                scrollable(list_content).into()
            }
        };

        let style_selection = column![
            // The list of saved styles
            collapsible("Saved Styles", saved_styles_list)
                .title_alignment(Alignment::Center)
                .collapse_icon(icon::expanded())
                .expand_icon(icon::collapsed()),

            // The controls for saving a new style
            container(
                column![
                    text_input("Enter Style Name...", &self.style_name)
                        .on_input(Message::UpdateStyleName)
                        .padding(10),
                    row![
                        button("Save Style").on_press(Message::SaveStyle).style(button::secondary),
                        button("Reset to Default").on_press(Message::ResetToDefault).style(button::secondary),
                    ].spacing(10),
                ]
                .spacing(10)
                .align_x(Alignment::Center),
            ),
        ].spacing(15);

        let preview_style = self.create_current_preview_style(theme);
        let preview_content = container(
            column![
                text("Preview").size(16),
                text("This is how your custom style looks!").size(14).center(),
                text("Lorem ipsum dolor sit amet, consectetur adipiscing elit.").size(12).center(),
                row![
                    text("Sample text").size(10),
                    Space::new().width(Length::Fill).height(Length::Fixed(1.0)),
                    text("More text").size(10),
                ]
            ]
            .spacing(5)
            .padding(15)
        ).center_x(Length::Fill)
        .style(move |_: &Theme| {
            preview_style
        });

        let code_view = {
            let tokens = generate_container_style_tokens(
                &self.style_name,
                self.text_color,
                &self.text_color_source,
                self.background_color,
                &self.background_color_source,
                self.border_color,
                &self.border_color_source,
                self.border_width,
                self.border_radius_top_left,
                self.border_radius_top_right,
                self.border_radius_bottom_right,
                self.border_radius_bottom_left,
                self.shadow_enabled,
                self.shadow_color,
                &self.shadow_color_source,
                self.shadow_offset_x,
                self.shadow_offset_y,
                self.shadow_blur_radius,
                self.snap
            );

            let code_string: String = tokens.iter().map(|t| t.text.clone()).collect();

            column![
                container(text("Style Code").size(18)).center_x(Length::Fill),
                internal_overlay(
                        build_code_view_with_height_generic::<Message>(&tokens, 0.0, self.theme.clone()),

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
            .height(Length::Fill)
            
        };

            row![
                scrollable(
                    column![
                        style_selection,
                        content,
                    ]
                    .spacing(10)
                    .padding(
                        Padding {
                            top: 0.0,
                            right: 15.0,
                            left: 0.0,
                            bottom: 0.0,
                        }
                    )
                ).width(Length::Fixed(420.0)),

                column![
                    container(text("Live Preview").size(18)).center_x(Length::Fill),
                    preview_content.center_x(Length::Fill),
                    code_view,            
                    
                ]
                .height(Length::Fill)
                .spacing(10)
                .padding(25),
            ]
            .spacing(10)
            .padding(
                Padding {
                    top: 10.0,
                    right: 5.0,
                    left: 5.0,
                    bottom: 10.0,
                }
            )
            .into()
    }

    fn create_current_preview_style(&self, theme: &Theme) -> container::Style {
        let text_color = if let Some(ref source) = self.text_color_source {
            evaluate_theme_expression(theme, source).unwrap_or(self.text_color)
        } else {
            self.text_color
        };
        
        let background_color = if let Some(ref source) = self.background_color_source {
            evaluate_theme_expression(theme, source).unwrap_or(self.background_color)
        } else {
            self.background_color
        };
        
        let border_color = if let Some(ref source) = self.border_color_source {
            evaluate_theme_expression(theme, source).unwrap_or(self.border_color)
        } else {
            self.border_color
        };
        
        let shadow_color = if let Some(ref source) = self.shadow_color_source {
            evaluate_theme_expression(theme, source).unwrap_or(self.shadow_color)
        } else {
            self.shadow_color
        };
        
        container_stylefn_builder(
            text_color,
            Background::Color(background_color),
            Border {
                color: border_color,
                width: self.border_width,
                radius: iced::border::Radius {
                    top_left: self.border_radius_top_left,
                    top_right: self.border_radius_top_right,
                    bottom_right: self.border_radius_bottom_right,
                    bottom_left: self.border_radius_bottom_left,
                },
            },
            if self.shadow_enabled {
                Shadow {
                    color: shadow_color,
                    offset: iced::Vector {
                        x: self.shadow_offset_x,
                        y: self.shadow_offset_y,
                    },
                    blur_radius: self.shadow_blur_radius,
                }
            } else {
                Shadow::default()
            },
            self.snap,
        )
    }

    fn reset_style_editor(&mut self, view: ThemePaneEnum) {
        let palette = self.theme.extended_palette();
        self.style_name = String::new();
        
        match view {
            ThemePaneEnum::Container => {
                self.text_color = palette.background.base.text;
                self.text_color_source = None;
                self.background_color = palette.background.base.color;
                self.background_color_source = None;
                self.border_color = palette.background.strong.color;
                self.border_color_source = None;
                self.border_width = 0.0;
                self.border_radius_top_left = 0.0;
                self.border_radius_top_right = 0.0;
                self.border_radius_bottom_right = 0.0;
                self.border_radius_bottom_left = 0.0;
                self.shadow_enabled = false;
                self.shadow_color = palette.background.weak.color;
                self.shadow_color_source = None;
                self.shadow_offset_x = 0.0;
                self.shadow_offset_y = 0.0;
                self.shadow_blur_radius = 0.0;
                self.snap = true;
            }
            ThemePaneEnum::Button => {
                // Using primary button colors as a default
                self.text_color = palette.primary.base.text;
                self.text_color_source = None;
                self.background_color = palette.primary.base.color;
                self.background_color_source = None;
                self.border_color = palette.primary.strong.color;
                self.border_color_source = None;
                self.border_width = 1.0;
                // Buttons in iced typically have a single radius value
                let radius = 4.0; 
                self.border_radius_top_left = radius;
                self.border_radius_top_right = radius;
                self.border_radius_bottom_right = radius;
                self.border_radius_bottom_left = radius;
                self.shadow_enabled = true;
                self.shadow_color = palette.background.weak.color;
                self.shadow_color_source = None;
                self.shadow_offset_x = 0.0;
                self.shadow_offset_y = 2.0; // A subtle shadow is a nice default
                self.shadow_blur_radius = 4.0;
                self.snap = true;
            }
            // For other views, just reset to the base theme defaults for now
            _ => {
                self.text_color = palette.background.base.text;
                self.text_color_source = None;
                self.background_color = palette.background.base.color;
                self.background_color_source = None;
                self.border_color = palette.background.strong.color;
                self.border_color_source = None;
                self.border_width = 0.0;
                self.border_radius_top_left = 0.0;
                self.border_radius_top_right = 0.0;
                self.border_radius_bottom_right = 0.0;
                self.border_radius_bottom_left = 0.0;
                self.shadow_enabled = false;
                self.shadow_color = palette.background.weak.color;
                self.shadow_color_source = None;
                self.shadow_offset_x = 0.0;
                self.shadow_offset_y = 0.0;
                self.shadow_blur_radius = 0.0;
                self.snap = true;
            }
        }
    }

    /// View to see all colors of a theme
    pub fn show_theme_colors<'a>(&'a self, theme: &'a Theme) -> Element<'a, Message> {
        let palette = theme.extended_palette();
        let base = theme.palette();

        let base_palette = container(
            column![
                container(text("Palette").size(24).color(palette.background.strong.text)).center(Length::Fill),
                row![
                    container(
                        text("Background").center(),
                    ).style(move |_: &Theme| container::Style { 
                        text_color: Some(base.text), 
                        background: Some( Background::Color(base.background)),
                        border: Border { color: palette.background.strong.color, width: 1.0, radius: 0.0.into() }, 
                        ..Default::default()}
                    )
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .width(Length::FillPortion(1))
                    .height(Length::Fixed(50.0)),

                    container(
                        text("Text").center(),
                    ).style(move |_: &Theme| container::Style { 
                        text_color: Some(base.background), 
                        background: Some( Background::Color(base.text)), 
                        ..Default::default()}
                    )
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .width(Length::FillPortion(1))
                    .height(Length::Fixed(50.0)),

                ].spacing(10),

                row![
                    container(
                        text("Primary").center(),
                    ).style(move |_: &Theme| container::Style { 
                        text_color: Some(base.background), 
                        background: Some( Background::Color(base.primary)),
                        ..Default::default()}
                    )
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .width(Length::FillPortion(1))
                    .height(Length::Fixed(50.0)),

                    container(
                        text("Success").center(),
                    ).style(move |_: &Theme| container::Style { 
                        text_color: Some(base.background), 
                        background: Some( Background::Color(base.success)),
                        ..Default::default()}
                    )
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .width(Length::FillPortion(1))
                    .height(Length::Fixed(50.0)),

                ].spacing(10),

                row![
                    container(
                        text("Warning").center(),
                    ).style(move |_: &Theme| container::Style { 
                        text_color: Some(base.background), 
                        background: Some( Background::Color(base.warning)),
                        ..Default::default()}
                    )
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .width(Length::FillPortion(1))
                    .height(Length::Fixed(50.0)),

                    container(
                        text("Danger").center(),
                    ).style(move |_: &Theme| container::Style { 
                        text_color: Some(base.background), 
                        background: Some( Background::Color(base.danger)),
                        ..Default::default()}
                    )
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .width(Length::FillPortion(1))
                    .height(Length::Fixed(50.0)),

                ].spacing(10),
            ]
            .spacing(15)
            .padding(15)
            .width(Length::Fill)
            .height(Length::Shrink)
        );

        let palette_showcase = scrollable(
            column![

                base_palette,

                container(text("Extended Palette").size(24).color(palette.background.strong.text)).center(Length::Fill),

                container(text("Background").size(16).color(palette.background.base.text)).center(Length::Fill),
                column![
                    row![
                        container(
                            text("Base").center(),
                        ).style(|_| container::Style { 
                            text_color: Some(palette.background.base.text), 
                            background: Some( Background::Color(palette.background.base.color)),
                            border: Border { color: palette.background.strong.color, width: 1.0, radius: 0.0.into() }, 
                            ..Default::default()}
                        )
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center)
                        .width(Length::FillPortion(1))
                        .height(Length::Fixed(50.0)),

                        container(
                            text("Neutral").center(),
                        ).style(|_| container::Style { 
                            text_color: Some(palette.background.neutral.text), 
                            background: Some( Background::Color(palette.background.neutral.color)
                            ), 
                            ..Default::default()}
                        )
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center)
                        .width(Length::FillPortion(1))
                        .height(Length::Fixed(50.0)),
                    ].spacing(10),

                    row![
                        container(
                            text("Weak").center(),
                        ).style(|_| container::Style { 
                            text_color: Some(palette.background.weak.text), 
                            background: Some( Background::Color(palette.background.weak.color)
                            ), 
                            ..Default::default()}
                        )
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center)
                        .width(Length::FillPortion(1))
                        .height(Length::Fixed(50.0)),

                        container(
                            text("Weaker").center(),
                        ).style(|_| container::Style { 
                            text_color: Some(palette.background.weaker.text), 
                            background: Some( Background::Color(palette.background.weaker.color)
                            ), 
                            ..Default::default()}
                        )
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center)
                        .width(Length::FillPortion(1))
                        .height(Length::Fixed(50.0)),

                        container(
                            text("Weakest").center(),
                        ).style(|_| container::Style { 
                            text_color: Some(palette.background.weakest.text), 
                            background: Some( Background::Color(palette.background.weakest.color)
                            ), 
                            ..Default::default()}
                        )
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center)
                        .width(Length::FillPortion(1))
                        .height(Length::Fixed(50.0)),
                    ].spacing(10),

                    row![
                        container(
                            text("Strong").center(),
                        ).style(|_| container::Style { 
                            text_color: Some(palette.background.strong.text), 
                            background: Some( Background::Color(palette.background.strong.color)
                            ), 
                            ..Default::default()}
                        )
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center)
                        .width(Length::FillPortion(1))
                        .height(Length::Fixed(50.0)),

                        container(
                            text("Stronger").center(),
                        ).style(|_| container::Style { 
                            text_color: Some(palette.background.stronger.text), 
                            background: Some( Background::Color(palette.background.stronger.color)
                            ), 
                            ..Default::default()}
                        )
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center)
                        .width(Length::FillPortion(1))
                        .height(Length::Fixed(50.0)),

                        container(
                            text("Strongest").center(),
                        ).style(|_| container::Style { 
                            text_color: Some(palette.background.strongest.text), 
                            background: Some( Background::Color(palette.background.strongest.color)
                            ), 
                            ..Default::default()}
                        )
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center)
                        .width(Length::FillPortion(1))
                        .height(Length::Fixed(50.0)),
                    ].spacing(10),

                ]
                .spacing(10),

                container(text("Primary").size(16).color(palette.background.base.text)).center(Length::Fill),
                row![
                    container(
                        text("Base").center(),
                    ).style(|_| container::Style { 
                        text_color: Some(palette.primary.base.text), 
                        background: Some( Background::Color(palette.primary.base.color)
                        ), 
                        ..Default::default()}
                    )
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .width(Length::FillPortion(1))
                    .height(Length::Fixed(50.0)),

                    container(
                        text("Weak").center(),
                    ).style(|_| container::Style { 
                        text_color: Some(palette.primary.weak.text), 
                        background: Some( Background::Color(palette.primary.weak.color)
                        ), 
                        ..Default::default()}
                    )
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .width(Length::FillPortion(1))
                    .height(Length::Fixed(50.0)),

                    container(
                        text("Strong").center(),
                    ).style(|_| container::Style { 
                        text_color: Some(palette.primary.strong.text), 
                        background: Some( Background::Color(palette.primary.strong.color)
                        ), 
                        ..Default::default()}
                    )
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .width(Length::FillPortion(1))
                    .height(Length::Fixed(50.0)),
                ]
                .spacing(10),

                container(text("Secondary").size(16).color(palette.background.base.text)).center(Length::Fill),
                row![
                    container(
                        text("Base").center(),
                    ).style(|_| container::Style { 
                        text_color: Some(palette.secondary.base.text), 
                        background: Some( Background::Color(palette.secondary.base.color)
                        ), 
                        ..Default::default()}
                    )
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .width(Length::FillPortion(1))
                    .height(Length::Fixed(50.0)),

                    container(
                        text("Weak").center(),
                    ).style(|_| container::Style { 
                        text_color: Some(palette.secondary.weak.text), 
                        background: Some( Background::Color(palette.secondary.weak.color)
                        ), 
                        ..Default::default()}
                    )
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .width(Length::FillPortion(1))
                    .height(Length::Fixed(50.0)),

                    container(
                        text("Strong").center(),
                    ).style(|_| container::Style { 
                        text_color: Some(palette.secondary.strong.text), 
                        background: Some( Background::Color(palette.secondary.strong.color)
                        ), 
                        ..Default::default()}
                    )
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .width(Length::FillPortion(1))
                    .height(Length::Fixed(50.0)),
                ]
                .spacing(10),

                container(text("Success").size(16).color(palette.background.base.text)).center(Length::Fill),
                row![
                    container(
                        text("Base").center(),
                    ).style(|_| container::Style { 
                        text_color: Some(palette.success.base.text), 
                        background: Some( Background::Color(palette.success.base.color)
                        ), 
                        ..Default::default()}
                    )
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .width(Length::FillPortion(1))
                    .height(Length::Fixed(50.0)),

                    container(
                        text("Weak").center(),
                    ).style(|_| container::Style { 
                        text_color: Some(palette.success.weak.text), 
                        background: Some( Background::Color(palette.success.weak.color)
                        ), 
                        ..Default::default()}
                    )
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .width(Length::FillPortion(1))
                    .height(Length::Fixed(50.0)),

                    container(
                        text("Strong").center(),
                    ).style(|_| container::Style { 
                        text_color: Some(palette.success.strong.text), 
                        background: Some( Background::Color(palette.success.strong.color)
                        ), 
                        ..Default::default()}
                    )
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .width(Length::FillPortion(1))
                    .height(Length::Fixed(50.0)),
                ]
                .spacing(10),

                container(text("Warning").size(16).color(palette.background.base.text)).center(Length::Fill),
                row![
                    container(
                        text("Base").center(),
                    ).style(|_| container::Style { 
                        text_color: Some(palette.warning.base.text), 
                        background: Some( Background::Color(palette.warning.base.color)
                        ), 
                        ..Default::default()}
                    )
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .width(Length::FillPortion(1))
                    .height(Length::Fixed(50.0)),

                    container(
                        text("Weak").center(),
                    ).style(|_| container::Style { 
                        text_color: Some(palette.warning.weak.text), 
                        background: Some( Background::Color(palette.warning.weak.color)
                        ), 
                        ..Default::default()}
                    )
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .width(Length::FillPortion(1))
                    .height(Length::Fixed(50.0)),

                    container(
                        text("Strong").center(),
                    ).style(|_| container::Style { 
                        text_color: Some(palette.warning.strong.text), 
                        background: Some( Background::Color(palette.warning.strong.color)
                        ), 
                        ..Default::default()}
                    )
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .width(Length::FillPortion(1))
                    .height(Length::Fixed(50.0)),
                ]
                .spacing(10),

                container(text("Danger").size(16).color(palette.background.base.text)).center(Length::Fill),
                row![
                    container(
                        text("Base").center(),
                    ).style(|_| container::Style { 
                        text_color: Some(palette.danger.base.text), 
                        background: Some( Background::Color(palette.danger.base.color)
                        ), 
                        ..Default::default()}
                    )
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .width(Length::FillPortion(1))
                    .height(Length::Fixed(50.0)),

                    container(
                        text("Weak").center(),
                    ).style(|_| container::Style { 
                        text_color: Some(palette.danger.weak.text), 
                        background: Some( Background::Color(palette.danger.weak.color)
                        ), 
                        ..Default::default()}
                    )
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .width(Length::FillPortion(1))
                    .height(Length::Fixed(50.0)),

                    container(
                        text("Strong").center(),
                    ).style(|_| container::Style { 
                        text_color: Some(palette.danger.strong.text), 
                        background: Some( Background::Color(palette.danger.strong.color)
                        ), 
                        ..Default::default()}
                    )
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .width(Length::FillPortion(1))
                    .height(Length::Fixed(50.0)),
                ]
                .spacing(10),

            ]
            .spacing(15)
            .padding(15)
            .width(Length::Fill)
            .height(Length::Shrink)
        );

        column![
            palette_showcase.style(|theme: &Theme, status| {
                let palette = theme.extended_palette();

                // theme+status-aware default
                let mut s = scrollable::default(theme, status);

                // update values and return
                s.container.background = Some(Background::Color(palette.background.base.color));
                s.container.border = Border { color: palette.background.strong.color, width: 1.0, radius: 5.0.into() };
                s
            })
        ]
        .padding(
            Padding {
                top: 10.0,
                right: 5.0,
                left: 5.0,
                bottom: 10.0,
            }
        )
        .width(Length::Fill)
        .into()
    }

    pub fn styles(&self) -> &BTreeMap<ThemePaneEnum, BTreeMap<String, SavedStyleDefinition >> {
        &self.styles
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ChangeView(ThemePaneEnum),
    CopyCode(String),

    // Generic Style properties
    UpdateTextColor { color: Color, source: Option<String> },
    UpdateBorderColor { color: Color, source: Option<String> },
    UpdateBorderWidth(f32),
    UpdateBorderRadiusTopLeft(f32),
    UpdateBorderRadiusTopRight(f32),
    UpdateBorderRadiusBottomRight(f32),
    UpdateBorderRadiusBottomLeft(f32),
    UpdateBackgroundColor { color: Color, source: Option<String> },
    UpdateShadowEnabled(bool),
    UpdateShadowColor { color: Color, source: Option<String> },
    UpdateShadowOffsetX(f32),
    UpdateShadowOffsetY(f32),
    UpdateShadowBlurRadius(f32),
    UpdateSnap(bool),

    // Style management
    UpdateStyleName(String),
    SaveStyle,
    SelectStyle(String),
    ResetToDefault,
}
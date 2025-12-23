use iced::widget::{button, checkbox, column, container, rule::horizontal as horizontal_rule, space::horizontal as horizontal_space, slider, row, scrollable, text, text_input, Space};
use iced::Length::FillPortion;
use iced::{Alignment, Background, Border, Color, Element, Length, Shadow, Theme, Padding, Task,};
use std::collections::BTreeMap;
use widgets::{color_picker, collapsible::collapsible};
use crate::{icon, styles};
use widgets::generic_overlay::{overlay_button, OverlayButton};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    // Container
    container_styles: BTreeMap<usize, iced::widget::container::Style>,
    container_text_color: Color,
    container_border_color: Color,
    container_border_width: f32,
    container_border_radius_top_left: f32,
    container_border_radius_top_right: f32,
    container_border_radius_bottom_right: f32,
    container_border_radius_bottom_left: f32,
    container_background_color: Color,
    container_shadow_enabled: bool,
    container_shadow_color: Color,
    container_shadow_offset_x: f32,
    container_shadow_offset_y: f32,
    container_shadow_blur_radius: f32,
    container_snap: bool,

    // Button
    button_styles: BTreeMap<usize, iced::widget::button::Style>,
}

impl CustomThemes {
    pub fn new(theme: &Theme) -> Self {
        let palette = theme.extended_palette();
        Self {
            theme: theme.clone(),
            selected_view: ThemePaneEnum::Container,

            // Container
            container_styles: BTreeMap::new(),
            container_text_color: palette.background.base.text,
            container_border_color: palette.background.strong.color,
            container_border_width: 0.0,
            container_border_radius_top_left: 0.0,
            container_border_radius_top_right: 0.0,
            container_border_radius_bottom_right: 0.0,
            container_border_radius_bottom_left: 0.0,
            container_background_color: palette.background.base.color,
            container_shadow_enabled: false,
            container_shadow_color: palette.background.weak.color,
            container_shadow_offset_x: 0.0,
            container_shadow_offset_y: 0.0,
            container_shadow_blur_radius: 0.0,
            container_snap: true,

            // Button
            button_styles: BTreeMap::new(), 
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ChangeView(view) => {
                match view {
                    ThemePaneEnum::ExtendedPalette => self.selected_view = ThemePaneEnum::ExtendedPalette,
                    ThemePaneEnum::Button => self.selected_view = ThemePaneEnum::Button,
                    ThemePaneEnum::Checkbox => self.selected_view = ThemePaneEnum::Checkbox,
                    ThemePaneEnum::Combobox => self.selected_view = ThemePaneEnum::Combobox,
                    ThemePaneEnum::Container => self.selected_view = ThemePaneEnum::Container,
                    ThemePaneEnum::Panegrid => self.selected_view = ThemePaneEnum::Panegrid,
                    ThemePaneEnum::Picklist => self.selected_view = ThemePaneEnum::Picklist,
                    ThemePaneEnum::Progressbar => self.selected_view = ThemePaneEnum::Progressbar,
                    ThemePaneEnum::QRCode => self.selected_view = ThemePaneEnum::QRCode,
                    ThemePaneEnum::Radio => self.selected_view = ThemePaneEnum::Radio,
                    ThemePaneEnum::Rule => self.selected_view = ThemePaneEnum::Rule,
                    ThemePaneEnum::Slider => self.selected_view = ThemePaneEnum::Slider,
                    ThemePaneEnum::Table => self.selected_view = ThemePaneEnum::Table,
                    ThemePaneEnum::TextEditor => self.selected_view = ThemePaneEnum::TextEditor,
                    ThemePaneEnum::TextInput => self.selected_view = ThemePaneEnum::TextInput,
                    ThemePaneEnum::Toggler => self.selected_view = ThemePaneEnum::Toggler,
                    
                }
            }

            Message::UpdateContainerTextColor(color) => self.container_text_color = color,
            Message::UpdateContainerBorderColor(color) => self.container_border_color = color,
            Message::UpdateContainerBorderWidth(width) => self.container_border_width = width,
            Message::UpdateContainerBorderRadiusTopLeft(radius) => self.container_border_radius_top_left = radius,
            Message::UpdateContainerBorderRadiusTopRight(radius) => self.container_border_radius_top_right = radius,
            Message::UpdateContainerBorderRadiusBottomRight(radius) => self.container_border_radius_bottom_right = radius,
            Message::UpdateContainerBorderRadiusBottomLeft(radius) => self.container_border_radius_bottom_left = radius,
            Message::UpdateContainerBackgroundColor(color) => self.container_background_color = color,
            Message::UpdateContainerShadowEnabled(enabled) => self.container_shadow_enabled = enabled,
            Message::UpdateContainerShadowColor(color) => self.container_shadow_color = color,
            Message::UpdateContainerShadowOffsetX(x) => self.container_shadow_offset_x = x,
            Message::UpdateContainerShadowOffsetY(y) => self.container_shadow_offset_y = y,
            Message::UpdateContainerShadowBlurRadius(blur_radius) => self.container_shadow_blur_radius = blur_radius,
            Message::UpdateContainerSnap(snap) => self.container_snap = snap,
            Message::SaveCustomContainerStyle(id, style) => { let _ = self.container_styles.insert(id, style); }

            Message::SaveContainerStyle => {
                let style = container_stylefn_builder(
                    self.container_text_color,
                    Background::Color(self.container_background_color),
                    Border {
                        color: self.container_border_color,
                        width: self.container_border_width,
                        radius: iced::border::Radius {
                            top_left: self.container_border_radius_top_left,
                            top_right: self.container_border_radius_top_right,
                            bottom_right: self.container_border_radius_bottom_right,
                            bottom_left: self.container_border_radius_bottom_left,
                        }
                    },
                    if self.container_shadow_enabled {
                        Shadow {
                            color: self.container_shadow_color,
                            offset: iced::Vector {
                                x: self.container_shadow_offset_x,
                                y: self.container_shadow_offset_y,
                            },
                            blur_radius: self.container_shadow_blur_radius,
                        }
                    } else {
                        Shadow::default()
                    },
                    self.container_snap
                );
                
                // Generate unique ID (could use timestamp or increment)
                let id = self.container_styles.len();
                self.container_styles.insert(id, style);
            }

            Message::SelectContainerStyle(id) => {
                if let Some(style) = self.container_styles.get(&id) {
                    // Extract properties from the saved style
                    if let Some(text_color) = style.text_color {
                        self.container_text_color = text_color;
                    }
                    if let Some(Background::Color(bg_color)) = style.background {
                        self.container_background_color = bg_color;
                    }
                    self.container_border_color = style.border.color;
                    self.container_border_width = style.border.width;
                    self.container_border_radius_top_left = style.border.radius.top_left;
                    self.container_border_radius_top_right = style.border.radius.top_right;
                    self.container_border_radius_bottom_right = style.border.radius.bottom_right;
                    self.container_border_radius_bottom_left = style.border.radius.bottom_left;
                    self.container_shadow_enabled = style.shadow.color.a > 0.0;
                    if self.container_shadow_enabled {
                        self.container_shadow_color = style.shadow.color;
                        self.container_shadow_offset_x = style.shadow.offset.x;
                        self.container_shadow_offset_y = style.shadow.offset.y;
                        self.container_shadow_blur_radius = style.shadow.blur_radius;
                    }
                    self.container_snap = style.snap;
                }
            }

            Message::ResetToTheme => {
                let palette = self.theme.extended_palette();
                self.container_text_color = palette.background.base.text;
                self.container_border_color = palette.background.strong.color;
                self.container_border_width = 0.0;
                self.container_border_radius_top_left = 0.0;
                self.container_border_radius_top_right = 0.0;
                self.container_border_radius_bottom_right = 0.0;
                self.container_border_radius_bottom_left = 0.0;
                self.container_background_color = palette.background.base.color;
                self.container_shadow_enabled = false;
                self.container_shadow_color = palette.background.weak.color;
                self.container_shadow_offset_x = 0.0;
                self.container_shadow_offset_y = 0.0;
                self.container_shadow_blur_radius = 0.0;
                self.container_snap = true;
            }
        }
        Task::none()
    }

    pub fn view<'a>(&'a self, theme: &'a Theme) -> Element<'a, Message> {
        let content = match self.selected_view {
            ThemePaneEnum::ExtendedPalette => self.show_theme_colors(theme),
            ThemePaneEnum::Container => self.show_container_stylefn_builder(theme),
            ThemePaneEnum::Button => self.show_container_stylefn_builder(theme),
            _ => self.show_container_stylefn_builder(theme),
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

    pub fn show_container_stylefn_builder<'a>(&'a self, theme: &'a Theme) -> Element<'a, Message> {

        //  Container {
        //      text_color: Some(Color),
        //      background: Some(Background::Color(Color)), //Or Some(Background::Gradient), but I'm not going to support that... yet?
        //      border: Border {
        //          color: Color,
        //          width: f32,
        //          radius: Radius {
        //              top_left: f32,
        //              top_right: f32,
        //              bottom_left: f32,
        //              bottom_right: f32,         
        //              }
        //      },
        //      shadow: Shadow {
        //          color: Color,
        //          offset: Vector {
        //              x: f32,
        //              y: f32,
        //          },
        //          blur_radius: f32,
        //      },
        //      snap: bool,
        //  }
        //
        //

        let content = column![
            container(text("Container Colors").size(20)).center_x(Length::Fill),
            row![
                column![
                    container(text("text_color").size(16)).center_x(Length::Fill),
                    color_picker::ColorButton::new(self.container_text_color)
                    .on_change(|color| Message::UpdateContainerTextColor(color))
                    .title("text_color")
                    .width(Length::Fill)
                    .height(Length::Fixed(50.0))
                    .show_hex(),
                ]
                .width(Length::FillPortion(1)),

                column![
                    container(text("background color").size(16)).center_x(Length::Fill),
                    color_picker::ColorButton::new(self.container_background_color)
                    .on_change(|color| Message::UpdateContainerBackgroundColor(color))
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
                        slider(0.0..=30.0, self.container_border_width, move |v| {
                            Message::UpdateContainerBorderWidth(v)
                        })
                        .step(1.0),
                        text(format!("{:.0}", self.container_border_width)).size(12).center(),
                    ].width(Length::FillPortion(1)).align_x(Alignment::Center),


                    column![
                        container(text("border color").size(16)).center_x(Length::Fill),
                        color_picker::ColorButton::new(self.container_border_color)
                        .on_change(|color| Message::UpdateContainerBorderColor(color))
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

                        slider(0.0..=30.0, self.container_border_radius_top_left, move |v| {
                            Message::UpdateContainerBorderRadiusTopLeft(v)
                        })
                        .step(1.0),
                        text(format!("{:.0}", self.container_border_radius_top_left)).size(12).center(),

                    ].spacing(5),

                    column![
                        text("Top right").size(16),
                        slider(0.0..=30.0, self.container_border_radius_top_right, move |v| {
                            Message::UpdateContainerBorderRadiusTopRight(v)
                        })
                        .step(1.0),
                        text(format!("{:.0}", self.container_border_radius_top_right)).size(12).center(),
                    ].spacing(5),

                ].spacing(10),

                row![
                    column![
                        text("Bottom left").size(16),
                        slider(0.0..=30.0, self.container_border_radius_bottom_left, move |v| {
                            Message::UpdateContainerBorderRadiusBottomLeft(v)
                        })
                        .step(1.0),
                        text(format!("{:.0}", self.container_border_radius_bottom_left)).size(12).center(),
                    ].spacing(5),

                    column![
                        text("Bottom right").size(16),
                        slider(0.0..=30.0, self.container_border_radius_bottom_right, move |v| {
                            Message::UpdateContainerBorderRadiusBottomRight(v)
                        })
                        .step(1.0),
                        text(format!("{:.0}", self.container_border_radius_bottom_right)).size(12).center(),
                    ].spacing(5),

                ].spacing(10),

            ].spacing(10),

            // Shadow
            column![
                container(text("Shadow").size(20)).center_x(Length::Fill),
                
                row![
                    column![
                        Space::new().width(Length::Fill).height(Length::Fixed(10.0)),
                        checkbox(self.container_shadow_enabled)
                            .label("Enable Shadow")
                            .on_toggle(Message::UpdateContainerShadowEnabled),
                    ].width(Length::FillPortion(1)).height(Length::Fixed(50.0)),

                    if self.container_shadow_enabled {
                        column![
                            container(text("shadow color").size(16)).center_x(Length::Fill),

                            color_picker::ColorButton::new(self.container_shadow_color)
                            .on_change(|color| Message::UpdateContainerShadowColor(color))
                            .title("shadow color")
                            .width(Length::Fill)
                            .height(Length::Fixed(50.0))
                            .show_hex()
                        ]
                        .width(Length::FillPortion(1))
                    } else { column![].width(Length::FillPortion(1)) }
                ].align_y(Alignment::Start),



                if self.container_shadow_enabled {
                    column![
                        row![
                            column![
                                text("Offset X").size(12),
                                slider(-20.0..=20.0, self.container_shadow_offset_x, move |v| {
                                    Message::UpdateContainerShadowOffsetX(v)
                                })
                                .step(1.0),
                                text(format!("{:.0}", self.container_shadow_offset_x)).size(12).center(),
                            ],
                            column![
                                text("Offset Y").size(12),
                                slider(-20.0..=20.0, self.container_shadow_offset_y, move |v| {
                                    Message::UpdateContainerShadowOffsetY(v)
                                })
                                .step(1.0),
                                text(format!("{:.0}", self.container_shadow_offset_y)).size(12).center(),
                            ],
                        ]
                        .spacing(15),
                        column![
                            text("Blur Radius").size(12),
                            slider(0.0..=50.0, self.container_shadow_blur_radius, move |v| {
                                Message::UpdateContainerShadowBlurRadius(v)
                            })
                            .step(1.0),
                            text(format!("{:.0}", self.container_shadow_blur_radius)).size(12).center(),
                        ],
                    ]
                    .spacing(10)
                } else {
                    column![]
                }
            ]
            .spacing(10),

            // Snap
            column![
                container(text("Snap").size(20)).center_x(Length::Fill),
                checkbox(self.container_snap)
                    .label("Enable Snap")
                    .on_toggle(Message::UpdateContainerSnap),
            ].spacing(10),


        ]
        .spacing(15)
        .padding(15);

        let style_selection = column![

            collapsible(
                "Saved Styles",

                if !self.container_styles.is_empty() {
                    column![
                        scrollable(
                            column(
                                self.container_styles.iter().map(|(id, style)| {
                                    button(
                                        container(
                                            text(format!("Style {}", id + 1))
                                                .size(12)
                                                .center()
                                        )
                                        .center(Length::Fill)
                                        .width(Length::Fill)
                                        .height(Length::Fixed(30.0))
                                        .style(move |_: &Theme| *style)
                                    )
                                    .style(button::text)
                                    .width(Length::Fill)
                                    .on_press(Message::SelectContainerStyle(*id))
                                    .into()
                                }).collect::<Vec<Element<Message>>>()
                            )
                            .padding(10)
                            .spacing(5)
                        )
                    ]
                    .spacing(5)
                } else {
                    column![]
                }                
            )
            .title_alignment(Alignment::Center)
            .collapse_icon(icon::expanded())
            .expand_icon(icon::collapsed()),

            container(
                row![
                    button("Save Style").on_press(Message::SaveContainerStyle).style(button::secondary),
                    button("Reset to Default").on_press(Message::ResetToTheme).style(button::secondary),
                ].spacing(10),
            ).center_x(Length::Fill),
        ].spacing(15);

        let preview_content = container(
            column![
                text("Preview").size(16),
                text("This is how your custom container style looks!").size(14).center(),
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
            container_stylefn_builder(
                self.container_text_color,
                Background::Color(self.container_background_color),
                Border {
                    color: self.container_border_color,
                    width: self.container_border_width,
                    radius: iced::border::Radius {
                        top_left: self.container_border_radius_top_left,
                        top_right: self.container_border_radius_top_right,
                        bottom_right: self.container_border_radius_bottom_right,
                        bottom_left: self.container_border_radius_bottom_left,
                    }
                },
                if self.container_shadow_enabled {
                    Shadow {
                        color: self.container_shadow_color,
                        offset: iced::Vector {
                            x: self.container_shadow_offset_x,
                            y: self.container_shadow_offset_y,
                        },
                        blur_radius: self.container_shadow_blur_radius,
                    }
                } else {
                    Shadow::default()
                },
                self.container_snap
            )
        });

        let code_view = {
            use crate::code_generator::{generate_container_style_tokens, build_code_view_with_height_generic};
            
            let tokens = generate_container_style_tokens(
                self.container_text_color,
                self.container_background_color,
                self.container_border_color,
                self.container_border_width,
                self.container_border_radius_top_left,
                self.container_border_radius_top_right,
                self.container_border_radius_bottom_right,
                self.container_border_radius_bottom_left,
                self.container_shadow_enabled,
                self.container_shadow_color,
                self.container_shadow_offset_x,
                self.container_shadow_offset_y,
                self.container_shadow_blur_radius,
                self.container_snap,
            );
            
/*             overlay_button(
                "Container Style Code",
                "Container Style Code",
                build_code_view_with_height_generic::<Message>(&tokens, 0.0, self.theme.clone())
            ).width(150.0).overlay_width(750.0).overlay_height(575.0) */

            column![
                container(text("Container Style Code").size(18)).center_x(Length::Fill),
                build_code_view_with_height_generic::<Message>(&tokens, 0.0, self.theme.clone())
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
}

#[derive(Debug, Clone)]
pub enum Message {
    ChangeView(ThemePaneEnum),


    // Containers
    SaveCustomContainerStyle(usize, iced::widget::container::Style),
    UpdateContainerTextColor(Color),
    UpdateContainerBorderColor(Color),
    UpdateContainerBorderWidth(f32),
    UpdateContainerBorderRadiusTopLeft(f32),
    UpdateContainerBorderRadiusTopRight(f32),
    UpdateContainerBorderRadiusBottomRight(f32),
    UpdateContainerBorderRadiusBottomLeft(f32),
    UpdateContainerBackgroundColor(Color),
    UpdateContainerShadowEnabled(bool),
    UpdateContainerShadowColor(Color),
    UpdateContainerShadowOffsetX(f32),
    UpdateContainerShadowOffsetY(f32),
    UpdateContainerShadowBlurRadius(f32),
    UpdateContainerSnap(bool),
    SaveContainerStyle,
    SelectContainerStyle(usize),
    ResetToTheme,

    //Buttons
}
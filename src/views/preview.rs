use iced::{Alignment, Border, Padding, Element, Length, Background, Theme, Color, Font, Shadow, Task};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, checkbox, column, combo_box, container, image, pick_list, progress_bar, markdown, mouse_area, radio, row, rule, scrollable, space, slider, stack, svg, themer, text, text_input, toggler, tooltip, vertical_slider};
use uuid::Uuid;
use std::collections::BTreeMap;

use crate::data_structures::types::types::*;
use crate::data_structures::types::type_implementations::*;
use crate::data_structures::widget_hierarchy::WidgetHierarchy;
use crate::data_structures::properties::messages::PropertyChange;
use crate::enum_builder::TypeSystem;
use crate::views::theme_and_stylefn_builder::{CustomThemes, ThemePaneEnum};
use crate::styles::style_enum::{WidgetStyle, SavedStyleDefinition};

#[derive(Debug, Clone)]
pub enum Message {
    // Widget Operations
    PropertyChanged(WidgetId, PropertyChange, Option<Uuid>),

    // Interactive widget messages
    ButtonPressed(WidgetId, Option<Uuid>),
    TextInputChanged(WidgetId, String, Option<Uuid>),
    Submitted(WidgetId, Option<Uuid>),
    TextPasted(WidgetId, String, Option<Uuid>),
    CheckboxToggled(WidgetId, bool, Option<Uuid>),
    RadioSelected(WidgetId, usize, Option<Uuid>),
    SliderChanged(WidgetId, f32, Option<Uuid>),
    TogglerToggled(WidgetId, bool, Option<Uuid>),
    PickListSelected(WidgetId, String, Option<Uuid>),
    ComboBoxOnInput(WidgetId, String, Option<Uuid>),
    ComboBoxSelected(WidgetId, String, Option<Uuid>),
    ComboBoxOnOptionHovered(WidgetId, String, Option<Uuid>),
    ComboBoxOnClose(WidgetId, Option<Uuid>),
    ComboBoxOnOpen(WidgetId, Option<Uuid>),
    Noop,
}

pub fn update(
    all_views: &mut BTreeMap<Uuid, AppView>,    
    type_system: &mut TypeSystem,
    message: Message,
) -> Task<Message> {
    match message {
        Message::PropertyChanged(id, change, view_id) => {
            if let Some(view_id) = view_id {
                if let Some(view) = all_views.get_mut(&view_id) {
                    view.hierarchy.apply_property_change(id, change.clone(), type_system);

                    match view.hierarchy.get_widget_by_id(id) {
                        Some(widget) => { 
                            if widget.widget_type == WidgetType::Space {
                                match change {
                                    PropertyChange::Orientation(Orientation::Horizontal) => {
                                        view.hierarchy.apply_property_change(id, PropertyChange::Width(Length::Fill), type_system);
                                        view.hierarchy.apply_property_change(id, PropertyChange::Height(Length::Shrink), type_system);
                                    }
                                    PropertyChange::Orientation(Orientation::Vertical) => {
                                        view.hierarchy.apply_property_change(id, PropertyChange::Width(Length::Shrink), type_system);
                                        view.hierarchy.apply_property_change(id, PropertyChange::Height(Length::Fill), type_system);
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        // Interactive widget messages
        Message::ButtonPressed(id, view_id) => {
            println!("{:?}, button pressed", id);
        }
        
        Message::TextInputChanged(id, value, view_id) => {
            if let Some(view_id) = view_id {
                if let Some(view) = all_views.get_mut(&view_id) {
                    view.hierarchy.apply_property_change(
                        id, 
                        PropertyChange::TextInputValue(value), 
                        type_system
                    );
                }
            }
        }

        Message::Submitted(id, view_id) => { println!("{:?}, text_input submitted.", id); }

        Message::TextPasted(id, value, view_id) => {
            println!("{:?}, text pasted.", id);
            if let Some(view_id) = view_id {
                if let Some(view) = all_views.get_mut(&view_id) {
                    view.hierarchy.apply_property_change(id, PropertyChange::TextInputValue(value), type_system)
                }
            }
        }
        
        Message::CheckboxToggled(id, checked, view_id) => {
            if let Some(view_id) = view_id {
                if let Some(view) = all_views.get_mut(&view_id) {
                    view.hierarchy.apply_property_change(id, PropertyChange::CheckboxChecked(checked), type_system);
                }
            }
        }
        
        Message::RadioSelected(id, index, view_id) => {
            if let Some(view_id) = view_id {
                if let Some(view) = all_views.get_mut(&view_id) {
                    view.hierarchy.apply_property_change(id, PropertyChange::RadioSelectedIndex(index), type_system);
                }
            }  
        }
        
        Message::SliderChanged(id, value, view_id) => {
            if let Some(view_id) = view_id {
                if let Some(view) = all_views.get_mut(&view_id) {
                    view.hierarchy.apply_property_change(id, PropertyChange::SliderValue(value), type_system);
                }
            }
        }
        
        Message::TogglerToggled(id, active, view_id) => {
            if let Some(view_id) = view_id {
                if let Some(view) = all_views.get_mut(&view_id) {
                    view.hierarchy.apply_property_change(id, PropertyChange::TogglerActive(active), type_system);
                }
            }
        }
        
        Message::PickListSelected(id, index, view_id) => {
            if let Some(view_id) = view_id {
                if let Some(view) = all_views.get_mut(&view_id) {
                    view.hierarchy.apply_property_change(id, PropertyChange::PickListSelected(Some(index)), type_system);
                }
            }
        }

        Message::ComboBoxOnInput(id, value, view_id) => {
            if let Some(view_id) = view_id {
                if let Some(view) = all_views.get_mut(&view_id) {
                    let props = &view.hierarchy.get_widget_by_id(id).unwrap().properties;
                    if props.combobox_use_on_input {
                        println!("combobox {:?} input text: {}", id, value);
                    }
                }
            }
        }
        Message::ComboBoxSelected(id, value, view_id) => {
            println!("combobox selected: {:?}", value);
            if let Some(view_id) = view_id {
                if let Some(view) = all_views.get_mut(&view_id) {
                    view.hierarchy.apply_property_change(id, PropertyChange::ComboBoxSelected(Some(value)), type_system);
                }
            }
        }
        Message::ComboBoxOnOpen(id, view_id) => {
            if let Some(view_id) = view_id {
                if let Some(view) = all_views.get_mut(&view_id) {
                    let props = &view.hierarchy.get_widget_by_id(id).unwrap().properties;
                    if props.combobox_use_on_open {
                        println!("combobox {:?} opened!", id);
                    }
                }
            }
        }
        Message::ComboBoxOnClose(id, view_id) => {
            if let Some(view_id) = view_id {
                if let Some(view) = all_views.get_mut(&view_id) {
                    let props = &view.hierarchy.get_widget_by_id(id).unwrap().properties;
                    if props.combobox_use_on_close {
                        println!("combobox {:?} closed!", id);
                    }
                }
            }

        }
        Message::ComboBoxOnOptionHovered(id, options, view_id) => {
            if let Some(view_id) = view_id {
                if let Some(view) = all_views.get_mut(&view_id) {
                    let props = &view.hierarchy.get_widget_by_id(id).unwrap().properties;
                    if props.combobox_use_on_option_hovered {
                        println!("combobox option hovered: {:?}", options);
                    }
                }
            }
        }
        Message::Noop => {
            // Do nothing - for preview-only interactions
        }
    }
    
    Task::none()
}

pub fn view<'a>(
    hierarchy: &'a WidgetHierarchy, 
    theme: &'a Theme, 
    custom_themes: &'a CustomThemes,
    highlight_selected: bool,
    all_views: &'a BTreeMap<Uuid, AppView>,
    current_view_id: Option<Uuid>,
) -> Element<'a, Message> {
    let widget_preview = build_widget_preview(hierarchy, hierarchy.root(), theme, custom_themes, highlight_selected, all_views, current_view_id);

    widget_preview

/* 
    let preview_scoped = themer(
        Some(theme.clone()),

        container(widget_preview)
            .width(Length::Fill)
            .height(Length::Fill)
            // Any style closures here will now see the scoped theme
            .style(|theme: &Theme| container::Style {
                background: Some(Background::Color(theme.palette().background)),
                border: Border {
                    color: theme.extended_palette().background.strong.color,
                    width: 2.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            }),
    );
    // Optional: set default text color / background for this scope:
     .text_color(|th| th.palette().text)
    .background(|th| Background::Color(th.palette().background)); 
    
    theme.extended_palette().secondary.base.text;
    
    column![
        row![
            tooltip(
                text("Preview Layout").size(20),
                text("This represents your app's main content container")
                    .size(12)
                    .color(Color::from_rgb(0.6, 0.6, 0.6))
                    .center(),
                tooltip::Position::Right
            ),
        ]
        .align_y(Alignment::Center)
        .padding(
            Padding {
                top: 5.0,
                right: 10.0,
                bottom: 0.0,
                left: 10.0,
            }
        )
        .spacing(20),

        rule::horizontal(5),
        space::horizontal().height(10),

        container(preview_scoped)
        .padding(5)
        .style(|theme: &Theme| container::Style {
                background: Some(Background::Color(theme.extended_palette().background.weak.color)),
                ..Default::default()
            }), 
    ]
    .spacing(10)
    .padding(10)
    .into() */
}

fn build_widget_preview<'a>(
    hierarchy: &'a WidgetHierarchy, 
    widget: &'a Widget, 
    theme: &'a Theme, 
    custom_themes: &'a CustomThemes,
    highlight_selected: bool,
    all_views: &'a BTreeMap<Uuid, AppView>,
    current_view_id: Option<Uuid>,
) -> Element<'a, Message> {
    let is_selected = hierarchy.selected_ids().contains(&widget.id);
    let props = &widget.properties;

    let content = match widget.widget_type {
        WidgetType::Container => {
            let mut container = container(
                if widget.children.is_empty() {
                    text("Empty Container").into()
                } else {
                    build_widget_preview(hierarchy, &widget.children[0], theme, custom_themes, highlight_selected, all_views, current_view_id)
                }
            );
            
            // Apply all container properties
            container = match props.container_sizing_mode {
                ContainerSizingMode::Manual => {
                    container
                        .width(props.width)
                        .height(props.height)
                        .align_x(props.align_x)
                        .align_y(props.align_y)
                }
                ContainerSizingMode::CenterX => {
                    container.center_x(props.container_center_length)
                }
                ContainerSizingMode::CenterY => {
                    container.center_y(props.container_center_length)
                }
                ContainerSizingMode::Center => {
                    container.center(props.container_center_length)
                }
            };
            
            // NEW: Apply max_width
            if let Some(max_w) = props.max_width {
                container = container.max_width(max_w);
            }
            
            // NEW: Apply max_height
            if let Some(max_h) = props.max_height {
                container = container.max_height(max_h);
            }
            
            // NEW: Apply clip
            if props.clip {
                container = container.clip(true);
            }
            
            // NEW: Apply widget ID
            if let Some(ref id) = props.widget_id {
                if !id.is_empty() {
                    container = container.id(id.clone());
                }
            }

            container = container.style(move |theme: &Theme| {
                // Check for a style name
                if let Some(style_name) = &props.custom_style_name {
                    if let Some(style_map) = custom_themes.styles().get(&ThemePaneEnum::Container) {
                        if let Some(style_definition) = style_map.get(style_name) { // check if it's a custom style
                            let style = style_definition.to_container_style(theme);
                            return style;
                        } else if ContainerStyleType::all().contains(style_name) { // check if it's a built-in style
                            let style = ContainerStyleType::get(style_name).unwrap();
                            return match style {
                                ContainerStyleType::Transparent => container::transparent(theme),
                                ContainerStyleType::Background => container::background(theme.extended_palette().background.base.color), 
                                ContainerStyleType::RoundedBox => container::rounded_box(theme), 
                                ContainerStyleType::BorderedBox => container::bordered_box(theme), 
                                ContainerStyleType::Dark => container::dark(theme), 
                                ContainerStyleType::Primary => container::primary(theme), 
                                ContainerStyleType::Secondary => container::secondary(theme), 
                                ContainerStyleType::Success => container::success(theme), 
                                ContainerStyleType::Danger => container::danger(theme), 
                                ContainerStyleType::Warning => container::warning(theme),
                            }
                        }
                    }
                }

                // Fallback to individual properties if no custom style is found
                let mut st = container::Style::default();

                if props.background_color.a > 0.0 {
                    st.background = Some(Background::Color(props.background_color));
                }

                st.border = Border {
                    color: props.border_color,
                    width: props.border_width,
                    radius: props.border_radius.into(),
                };

                if props.has_shadow {
                    st.shadow = Shadow {
                        color: props.shadow_color,
                        offset: props.shadow_offset,
                        blur_radius: props.shadow_blur,
                    };
                }

                st
            });

            container.into()
        }
        
        WidgetType::Row => {
            let children: Vec<Element<'a, Message>> = widget.children
                .iter()
                .map(|child| build_widget_preview(hierarchy, &widget.children[0], theme, custom_themes, highlight_selected, all_views, current_view_id))
                .collect();
            
            if props.is_wrapping_row {
                // Wrapping rows MUST use row(children) pattern
                let mut wrapping = row(children)
                    .spacing(props.spacing)
                    .padding(props.padding)
                    .width(props.width)
                    .height(props.height)
                    .wrap();
                
                // Apply vertical spacing if set
                if props.match_horizontal_spacing {
                    wrapping = wrapping.vertical_spacing(props.wrapping_vertical_spacing);
                }
                
                // Apply horizontal alignment
                wrapping = wrapping.align_x(props.wrapping_align_x);
                
                wrapping.into()
            } else {
                // Non-wrapping rows: Use the old working pattern
                let mut content = row![]
                    .spacing(props.spacing)
                    .padding(props.padding)
                    .width(props.width)
                    .height(props.height)
                    .align_y(match props.align_items {
                        Alignment::Start => Vertical::Top,
                        Alignment::Center => Vertical::Center,
                        Alignment::End => Vertical::Bottom,
                    });
                
                if widget.children.is_empty() {
                    content = content.push(text("Row Item 1"));
                    content = content.push(text("Row Item 2"));
                } else {
                    for child in &widget.children {
                        content = content.push(build_widget_preview(hierarchy, child, theme, custom_themes, highlight_selected, all_views, current_view_id));
                    }
                }
                
                if props.clip {
                    content = content.clip(true);
                }
                
                content.into()
            }
        }
        
        WidgetType::Column => {
            let mut content = column![]
                .spacing(props.spacing)
                .padding(props.padding)
                .width(props.width)
                .height(props.height)
                .align_x(match props.align_items {
                    Alignment::Start => Horizontal::Left,
                    Alignment::Center => Horizontal::Center,
                    Alignment::End => Horizontal::Right,
                });
            
            if widget.children.is_empty() {
                content = content.push(text("Column Item 1"));
                content = content.push(text("Column Item 2"));
            } else {
                for child in &widget.children {
                    content = content.push(build_widget_preview(hierarchy, child, theme, custom_themes, highlight_selected, all_views, current_view_id));
                }
            }
            
            if let Some(max_w) = props.max_width {
                content = content.max_width(max_w);
            }
            
            if props.clip {
                content = content.clip(true);
            }
            
            content.into()
        }
        
        WidgetType::Button => {
            let props = &widget.properties;
            
            // Create button with text content
            let mut btn = button(text(&props.text_content));
            
            if props.button_on_press_enabled {
                btn = btn.on_press(Message::Noop);
            }

            if props.button_on_press_with_enabled{
                btn = btn.on_press_with(|| Message::Noop);
            }

            if props.button_on_press_maybe_enabled{
                btn = btn.on_press_maybe(Some(Message::Noop));
            }

            btn = btn.style(move |theme: &Theme, status: button::Status| {
                if let Some(style_name) = &props.custom_style_name {
                    if let Some(style_map) = custom_themes.styles().get(&ThemePaneEnum::Button) {
                        if let Some(style_definition) = style_map.get(style_name) { // check if it's a custom style
                            let style = style_definition.to_button_style(theme, status);
                            return style;
                        } else if ButtonStyleType::all().contains(style_name) { // check if it's a built-in style
                            let style = ButtonStyleType::get(style_name).unwrap();
                            return match style {
                                ButtonStyleType::Primary => button::primary(theme, status),
                                ButtonStyleType::Secondary => button::secondary(theme, status),
                                ButtonStyleType::Success => button::success(theme, status),
                                ButtonStyleType::Danger => button::danger(theme, status),
                                ButtonStyleType::Text => button::text(theme, status),
                                ButtonStyleType::Background => button::background(theme, status),
                                ButtonStyleType::Subtle => button::subtle(theme, status),
                            }
                        }
                    }
                }

                //fallback to default style
                button::primary(theme, status)
            });
            
            // Apply layout properties
            btn = btn.width(props.width);
            btn = btn.height(props.height);
            
            // Apply padding
            if props.padding_mode == PaddingMode::Uniform {
                btn = btn.padding(props.padding.top);
            } else {
                btn = btn.padding(props.padding);
            }
            
            // Apply clip
            if props.clip {
                btn = btn.clip(true);
            }
            
            btn.into()
        }
        
        WidgetType::Text => {
            
            let mut t = text(&props.text_content)
                .width(props.width)
                .height(props.height)
                .size(props.text_size)
                .font(match props.font { FontType::Default => Font::default(), FontType::Monospace => Font::MONOSPACE });

            let user_color = props.text_color; // Only set the color if a color has been set :D
            t = t.style(move |th: &Theme| {
                let c = if user_color.a == 0.0 { th.palette().text } else { user_color };
                text::Style { color: Some(c) }
            });
            t = t.line_height(props.line_height);
            t = t.wrapping(match props.wrap {
                text::Wrapping::None => text::Wrapping::None,
                text::Wrapping::Word => text::Wrapping::Word,
                text::Wrapping::Glyph => text::Wrapping::Glyph,
                text::Wrapping::WordOrGlyph => text::Wrapping::WordOrGlyph,
            });
            t = t.shaping(match props.shaping {
                text::Shaping::Basic => text::Shaping::Basic,
                text::Shaping::Advanced => text::Shaping::Advanced,
                text::Shaping::Auto => text::Shaping::Auto,
            });
            t = t.align_x(props.text_align_x).align_y(props.text_align_y);

            t.into()
        }

        WidgetType::TextInput => {
            let props = &widget.properties;
            
            // Create text_input with placeholder and value
            let mut input = text_input(
                &props.text_input_placeholder,
                &props.text_input_value
            );
            
            // Always add on_input in preview (using Noop since it's just preview)
            input = input.on_input(move |text| Message::TextInputChanged(widget.id, text, current_view_id));
            
            // Conditionally add on_submit
            if props.text_input_on_submit {
                input = input.on_submit(Message::Submitted(widget.id, current_view_id));
            }
            
            // Conditionally add on_paste
            if props.text_input_on_paste {
                input = input.on_paste(move |text| Message::TextPasted(widget.id, text, current_view_id));
            }
            
            // Apply secure mode
            if props.is_secure {
                input = input.secure(true);
            }
            
            // Apply size (font size)
            input = input.size(props.text_input_size);
            
            // Apply internal padding
            input = input.padding(props.text_input_padding);
            
            // Apply layout properties
            input = input.width(props.width);
            
            // Apply font if not default
            if props.text_input_font != FontType::Default {
                input = input.font(props.text_input_font.into());
            }
            
            // Apply line height if specified
            if props.text_input_line_height != text::LineHeight::default() {
                input = input.line_height(props.text_input_line_height);
            }
            
            // Apply alignment
            if props.text_input_alignment != ContainerAlignX::Left {
                input = input.align_x(props.text_input_alignment);
            }
            
            input.into()
        }

        WidgetType::Checkbox => {
            checkbox(props.checkbox_checked)
                .label(&props.checkbox_label)
                .size(props.checkbox_size)
                .spacing(props.checkbox_spacing)
                .width(props.width)
                .on_toggle(move |_| Message::CheckboxToggled(widget.id, !props.checkbox_checked, current_view_id))
                .into()
        }

        WidgetType::Radio => {
            if !props.radio_options.is_empty() {
                column(
                    props.radio_options.iter().enumerate().map(|(i, option)| {
                        radio(
                            option,
                            i,
                            Some(props.radio_selected_index),
                            move |selected_index| Message::RadioSelected(widget.id, selected_index, current_view_id)
                        )
                        .size(props.radio_size)
                        .spacing(props.radio_spacing)
                        .into()
                    }).collect::<Vec<Element<Message>>>()
                )
                .into()
            } else {
                text("No radio options").into()
            }
        }

        WidgetType::Slider => {
            slider(props.slider_min..=props.slider_max, props.slider_value, move |value| {
                Message::SliderChanged(widget.id, value, current_view_id)
            })
            .step(props.slider_step)
            .height(props.slider_height)
            .into()
        }

        WidgetType::VerticalSlider => {
            vertical_slider(props.slider_min..=props.slider_max, props.slider_value, move |value| {
                Message::SliderChanged(widget.id, value, current_view_id)
            })
            .step(props.slider_step)
            .width(props.slider_width)
            .into()
        }

        WidgetType::ProgressBar => {
            let mut content = progress_bar(props.progress_min..=props.progress_max, props.progress_value)
                .length(props.progress_length)
                .girth(props.progress_girth);

            if props.progress_vertical {
                content = content.vertical();
            }

            content.into()
        }

        WidgetType::Toggler => {
            toggler(props.toggler_active)
                .on_toggle(move |_| Message::TogglerToggled(widget.id, !props.toggler_active, current_view_id))
                .label(&props.toggler_label)
                .size(props.toggler_size)
                .spacing(props.toggler_spacing)
                .width(props.width)
                .into()
        }

        WidgetType::PickList => {
            pick_list(
                props.picklist_options.clone(),
                props.picklist_selected.clone(),
                move |selected| Message::PickListSelected(widget.id, selected, current_view_id)
            )
            .placeholder(&props.picklist_placeholder)
            .width(props.width)
            .into()
        }

        WidgetType::Scrollable => {
            let mut content = column![];
            
            if widget.children.is_empty() {
                for i in 1..=10 {
                    content = content.push(text(format!("Scrollable Item {}", i)));
                }
            } else {
                for child in &widget.children {
                    content = content.push(build_widget_preview(hierarchy, child, theme, custom_themes, highlight_selected, all_views, current_view_id));
                }
            }
            
            scrollable(content)
                .direction(props.scroll_dir)
                .anchor_x(props.anchor_x)
                .anchor_y(props.anchor_y)
                .width(props.width)
                .height(props.height)
                .into()
        }

        WidgetType::Space => {
            let s = match props.orientation {
                Orientation::Horizontal => space::horizontal().width(props.width).height(props.height),
                Orientation::Vertical => space::vertical().width(props.width).height(props.height),
            };

            if props.show_widget_bounds {
                container(s)
                    .style(|_| container::Style {
                        background: Some(Background::Color(Color::from_rgba(0.2, 0.6, 1.0, 0.18))),
                        border: Border { color: Color::from_rgb(0.2, 0.6, 1.0), width: 1.0, radius: 2.0.into() },
                        ..Default::default()
                    })
                    .into()
            } else {
                s.into()
            }
        }

        WidgetType::Rule => {
            match props.orientation {
                Orientation::Horizontal => rule::horizontal(props.rule_thickness).into(),
                Orientation::Vertical => rule::vertical(props.rule_thickness).into(),
            }
        }

        WidgetType::Image => {
            let el: Element<_> = if props.image_path.trim().is_empty() {
                // Placeholder box when no path provided
                container(text("🖼️ Image (no path)"))
                    .width(props.width).height(props.height)
                    .style(|_| container::Style {
                        border: Border{ color: Color::from_rgb(0.6,0.6,0.6), width: 1.0, radius: 4.0.into() },
                        background: Some(Background::Color(Color::from_rgba(0.5,0.5,0.5,0.05))),
                        ..Default::default()
                    })
                    .into()
            } else {
                image(image::Handle::from_path(&props.image_path))
                    .content_fit(props.image_fit.into())
                    .width(props.width).height(props.height)
                    .into()
            };
            el
        }

        WidgetType::Svg => {
            let el: Element<_> = if props.svg_path.trim().is_empty() {
                container(text("🧩 SVG (no path)"))
                    .width(props.width).height(props.height)
                    .style(|_| container::Style {
                        border: Border{ color: Color::from_rgb(0.6,0.6,0.6), width: 1.0, radius: 4.0.into() },
                        background: Some(Background::Color(Color::from_rgba(0.5,0.5,0.5,0.05))),
                        ..Default::default()
                    })
                    .into()
            } else {
                svg(svg::Handle::from_path(&props.svg_path))
                    .content_fit(props.svg_fit.into())
                    .width(props.width).height(props.height)
                    .into()
            };
            el
        }

        WidgetType::Tooltip => {
            // child[0] = trigger (host), child[1] = popup content
            let host = {
                let element = widget.children.get(0)
                    .map(|widget| build_widget_preview(hierarchy, widget, theme, custom_themes, highlight_selected, all_views, current_view_id))
                    .unwrap_or_else(|| text("Tooltip host").into());

                container(element)
                    .padding(6)
                    .style(|th: &Theme| container::Style {
                        border: Border { color: th.extended_palette().primary.strong.color, width: 1.0, radius: 4.0.into() },
                        ..Default::default()
                    })
            };

            let popup = {
                let element = widget.children.get(1)
                    .map(|widget| build_widget_preview(hierarchy, widget, theme, custom_themes, highlight_selected, all_views, current_view_id))
                    .unwrap_or_else(|| text(&props.tooltip_text).size(14).into());

                container(element)
                    .padding(6)
                    .style(|th: &Theme| container::Style {
                        background: Some(Background::Color(th.extended_palette().background.weak.color)),
                        border: Border { color: th.extended_palette().background.strong.color, width: 1.0, radius: 4.0.into() },
                        ..Default::default()
                    })
            };

            tooltip(host, popup, props.tooltip_position.into())
                .gap(6)
                .padding(8)
                .into()
        }
        
        WidgetType::ComboBox => {
            let id = widget.id;
            let on_selected = move |selected| {
                Message::PropertyChanged(
                    id,
                    PropertyChange::ComboBoxSelected(Some(selected)),
                    current_view_id
                )
            };

            let mut cb = combo_box(
                &props.combobox_state,
                &props.combobox_placeholder,
                props.combobox_selected.as_ref(),
                on_selected
            )
            .on_close(Message::ComboBoxOnClose(id, current_view_id))
            .on_input(move |search| Message::ComboBoxOnInput(id, search, current_view_id))
            .on_open(Message::ComboBoxOnOpen(id, current_view_id))
            .on_option_hovered(move |hovered| Message::ComboBoxOnOptionHovered(id, hovered, current_view_id))
            .width(props.width);

            // Apply custom style if set
            if let Some(ref style_name) = props.custom_style_name {
                if let Some(style_map) = custom_themes.styles().get(&ThemePaneEnum::Combobox) {
                    if let Some(definition) = style_map.get(style_name) {
                        let def_input = definition.clone();
                        cb = cb.input_style(move |theme: &Theme, status| {
                            def_input.to_combo_box_input_style(theme, status)
                        });
                        let def_menu = definition.clone();
                        cb = cb.menu_style(move |theme: &Theme| {
                            def_menu.to_combo_box_menu_style(theme)
                        });
                    }
                }
            }

            cb.into()
        }
        
        WidgetType::Markdown => {
            
            markdown::view(
                &props.markdown_content,
                markdown::Settings::with_text_size(
                    props.markdown_text_size,
                    theme.clone()
                )
            ).map(|_| Message::Noop)
        }
        
        WidgetType::MouseArea => {
            let props = &widget.properties;
            
            // Build the child content
            let content = if widget.children.is_empty() {
                // Show placeholder when no child exists
                container(text("Mouse Area Content"))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .style(|_| container::Style {
                        border: Border { 
                            color: Color::from_rgba(0.5, 0.5, 0.5, 0.3), 
                            width: 1.0, 
                            radius: 4.0.into() 
                        },
                        background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.05))),
                        ..Default::default()
                    })
                    .into()
            } else {
                build_widget_preview(hierarchy, &widget.children[0], theme, custom_themes, highlight_selected, all_views, current_view_id)
            };
            
            // Start building mouse_area with conditional handlers
            let mut area = mouse_area(content);
            
            // Conditionally add event handlers based on properties
            if props.mousearea_on_press {
                area = area.on_press(Message::Noop);
            }
            
            if props.mousearea_on_release {
                area = area.on_release(Message::Noop);
            }
            
            if props.mousearea_on_double_click {
                area = area.on_double_click(Message::Noop);
            }
            
            if props.mousearea_on_right_press {
                area = area.on_right_press(Message::Noop);
            }
            
            if props.mousearea_on_right_release {
                area = area.on_right_release(Message::Noop);
            }
            
            if props.mousearea_on_middle_press {
                area = area.on_middle_press(Message::Noop);
            }
            
            if props.mousearea_on_middle_release {
                area = area.on_middle_release(Message::Noop);
            }
            
            if props.mousearea_on_scroll {
                area = area.on_scroll(|_delta| Message::Noop);
            }
            
            if props.mousearea_on_enter {
                area = area.on_enter(Message::Noop);
            }
            
            if props.mousearea_on_move {
                area = area.on_move(|_point| Message::Noop);
            }
            
            if props.mousearea_on_exit {
                area = area.on_exit(Message::Noop);
            }
            
            // Set mouse interaction if specified
            if let Some(interaction) = props.mousearea_interaction {
                area = area.interaction(interaction.into());
            }
            
            // Apply common layout properties
            let mut element: Element<_> = area.into();
            
            if props.width != Length::Shrink {
                element = container(element).width(props.width).into();
            }
            if props.height != Length::Shrink {
                element = container(element).height(props.height).into();
            }
            
            element
        }
        
        WidgetType::QRCode => {
            use iced::widget::qr_code;
            
            match &props.qrcode_data {  
                Some(data) => {
                    qr_code::QRCode::new(data)
                        .cell_size(props.qrcode_cell_size)
                        .into()
                }
                _ => {
                    text("Invalid QR data").into()
                }
            }
        }
        
        WidgetType::Stack => {
            let mut layers = Vec::new();
            
            if widget.children.is_empty() {
                layers.push(
                    container(text("Stack Layer 1"))
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .center(Length::Fill)
                        .into()
                );
                layers.push(
                    container(text("Stack Layer 2").color(Color::from_rgb(1.0, 0.0, 0.0)))
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .padding(20)
                        .into()
                );
            } else {
                for child in &widget.children {
                    layers.push(build_widget_preview(hierarchy, child, theme, custom_themes, highlight_selected, all_views, current_view_id));
                }
            }
            
            stack(layers)
                .width(props.width)
                .height(props.height)
                .into()
        }
        
        WidgetType::Themer => {
            let content = if widget.children.is_empty() {
                container(text("Themed Content"))
                    .padding(10)
                    .into()
            } else {
                let mut col = column![];
                for child in &widget.children {
                    col = col.push(build_widget_preview(hierarchy, child, theme, custom_themes, highlight_selected, all_views, current_view_id));
                }
                col.into()
            };
            
            if let Some(theme) = &props.themer_theme {
                themer(Some(theme.clone()), content).into()
            } else {
                content
            }
        }

        WidgetType::ViewReference => {
            let props = &widget.properties;
            
            match props.referenced_view_id {
                Some(view_id) => {
                    match all_views.get(&view_id) {
                        Some(referenced_view) => {
                            build_widget_preview(
                                &referenced_view.hierarchy,
                                referenced_view.hierarchy.root(),
                                theme, 
                                custom_themes,
                                highlight_selected,
                                all_views, 
                                Some(view_id),
                            )
                        }
                        None => {
                            // View doesn't exist (need to protect used views from being deleted)
                            container(
                                text(format!("View not found: {}", view_id))
                                    .color(Color::from_rgb(1.0, 0.0, 0.0))
                            )
                            .padding(10)
                            .style(|_| container::Style {
                                border: Border { 
                                    color: Color::from_rgb(1.0, 0.0, 0.0), 
                                    width: 2.0, 
                                    radius: 4.0.into() 
                                },
                                background: Some(Background::Color(Color::from_rgba(1.0, 0.0, 0.0, 0.1))),
                                ..Default::default()
                            })
                            .into()
                        }
                    }
                }
                None => {
                    // No view selected yet
                    container(
                        text("Select a view to reference")
                            .color(Color::from_rgb(0.5, 0.5, 0.5))
                    )
                    .padding(20)
                    .style(|_| container::Style {
                        border: Border { 
                            color: Color::from_rgba(0.5, 0.5, 0.5, 0.5), 
                            width: 1.0, 
                            radius: 4.0.into() 
                        },
                        background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.05))),
                        ..Default::default()
                    })
                    .into()
                }
            }
        }

        _ => {
            text(format!("{:?} preview", widget.widget_type)).into()
        }
    };

    if is_selected && highlight_selected {
        content.explain(theme.extended_palette().success.strong.color)
            .into()
    } else {
        content
    }
}
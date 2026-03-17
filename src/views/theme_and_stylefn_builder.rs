use crate::code_gen_version_two::{
    generate_button_style_code, generate_checkbox_style_code, generate_combo_box_style_code,
    generate_container_style_code, generate_menu_style_code, generate_pick_list_style_code,
    generate_progress_bar_style_code, generate_radio_style_code, generate_rule_style_code,
    generate_slider_style_code, generate_text_input_style_code, generate_toggler_style_code,
    helpers::internal_overlay,
};
use crate::styles::style_enum::{
    RuleFillMode, SavedStyleDefinition, StatusColorOverride, evaluate_theme_expression,
};
use crate::{icon, styles};
use iced::clipboard;
use iced::widget::{
    Space, button, checkbox, column, combo_box, container, pick_list, progress_bar, radio, row,
    rule, scrollable, slider, text, text_editor, text_input, toggler, tooltip,
};
use iced::{Alignment, Background, Border, Color, Element, Length, Padding, Shadow, Task, Theme};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::Arc;
use tree_sitter_highlighter::{TreeSitterIcedHighlighter, TsSettings, code_gen_text_editor_style};
use widgets::generic_overlay;
use widgets::{collapsible::collapsible, color_picker};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ThemePaneEnum {
    ExtendedPalette,
    Button,
    Checkbox,
    Combobox,
    Container,
    Menu,
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
        write!(
            f,
            "{}",
            match self {
                ThemePaneEnum::ExtendedPalette => "ExtendedPalette",
                ThemePaneEnum::Button => "Button",
                ThemePaneEnum::Checkbox => "Checkbox",
                ThemePaneEnum::Combobox => "Combobox",
                ThemePaneEnum::Container => "Container",
                ThemePaneEnum::Menu => "Menu",
                ThemePaneEnum::Panegrid => "Panegrid",
                ThemePaneEnum::Picklist => "Picklist",
                ThemePaneEnum::Progressbar => "Progressbar",
                ThemePaneEnum::QRCode => "QRCode",
                ThemePaneEnum::Radio => "Radio",
                ThemePaneEnum::Rule => "Rule",
                ThemePaneEnum::Slider => "Slider",
                ThemePaneEnum::Table => "Table",
                ThemePaneEnum::TextEditor => "TextEditor",
                ThemePaneEnum::TextInput => "TextInput",
                ThemePaneEnum::Toggler => "Toggler",
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EditingStatus {
    #[default]
    Active,
    Hovered,
    Pressed,
    Disabled,
    Focused,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusColorField {
    Text,
    Background,
    Border,
    Icon,
}

/// StyleFn Builders

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
    rule_fill_mode: RuleFillMode,

    // source tracking for palette reference in color_picker
    text_color_source: Option<String>,
    border_color_source: Option<String>,
    background_color_source: Option<String>,
    shadow_color_source: Option<String>,

    // checkbox/combobox-specific fields
    icon_color: Color,
    icon_color_source: Option<String>,

    // combobox-specific fields
    placeholder_color: Color,
    placeholder_color_source: Option<String>,
    selection_color: Color,
    selection_color_source: Option<String>,
    selected_text_color: Color,
    selected_text_color_source: Option<String>,
    selected_background_color: Color,
    selected_background_color_source: Option<String>,

    // per-status overrides
    editing_status: EditingStatus,
    status_hovered_override: Option<StatusColorOverride>,
    status_pressed_override: Option<StatusColorOverride>,
    status_disabled_override: Option<StatusColorOverride>,
    status_focused_override: Option<StatusColorOverride>,

    // tree_sitter text_editor content for style code preview (shared across widget types)
    style_code_content: text_editor::Content,

    preview_value: String,
    combobox_state: combo_box::State<String>,
}

impl CustomThemes {
    pub fn new(theme: &Theme) -> Self {
        let palette = theme.extended_palette();
        let mut styles = BTreeMap::new();
        styles.insert(ThemePaneEnum::Container, BTreeMap::new());
        styles.insert(ThemePaneEnum::Button, BTreeMap::new());
        styles.insert(ThemePaneEnum::Checkbox, BTreeMap::new());
        styles.insert(ThemePaneEnum::TextInput, BTreeMap::new());
        styles.insert(ThemePaneEnum::Menu, BTreeMap::new());
        styles.insert(ThemePaneEnum::Picklist, BTreeMap::new());
        styles.insert(ThemePaneEnum::Slider, BTreeMap::new());
        styles.insert(ThemePaneEnum::Progressbar, BTreeMap::new());
        styles.insert(ThemePaneEnum::Radio, BTreeMap::new());
        styles.insert(ThemePaneEnum::Toggler, BTreeMap::new());
        styles.insert(ThemePaneEnum::Combobox, BTreeMap::new());
        styles.insert(ThemePaneEnum::Rule, BTreeMap::new());

        let mut new_instance = Self {
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
            rule_fill_mode: RuleFillMode::Full,
            text_color_source: None,
            border_color_source: None,
            background_color_source: None,
            shadow_color_source: None,
            icon_color: palette.primary.base.text,
            icon_color_source: None,
            placeholder_color: palette.background.weak.text,
            placeholder_color_source: None,
            selection_color: palette.primary.weak.color,
            selection_color_source: None,
            selected_text_color: palette.primary.base.text,
            selected_text_color_source: None,
            selected_background_color: palette.primary.base.color,
            selected_background_color_source: None,
            editing_status: EditingStatus::Active,
            status_hovered_override: None,
            status_pressed_override: None,
            status_disabled_override: None,
            status_focused_override: None,
            style_code_content: text_editor::Content::new(),
            preview_value: String::new(),
            combobox_state: combo_box::State::new(vec![
                "Option 1".to_string(),
                "Option 2".to_string(),
                "Option 3".to_string(),
            ]),
        };
        new_instance.regenerate_container_code();
        new_instance
    }

    pub fn theme(&mut self, theme: &Theme) {
        self.theme = theme.clone();
        self.reset_to_theme();
    }

    /// Restores saved style definitions after loading a project.
    pub fn restore_styles(
        &mut self,
        styles: BTreeMap<ThemePaneEnum, BTreeMap<String, SavedStyleDefinition>>,
    ) {
        self.styles = styles;
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
        self.icon_color = palette.primary.base.text;
        self.icon_color_source = None;
        self.placeholder_color = palette.background.weak.text;
        self.placeholder_color_source = None;
        self.selection_color = palette.primary.weak.color;
        self.selection_color_source = None;
        self.selected_text_color = palette.primary.base.text;
        self.selected_text_color_source = None;
        self.selected_background_color = palette.primary.base.color;
        self.selected_background_color_source = None;
        self.editing_status = EditingStatus::Active;
        self.status_hovered_override = None;
        self.status_pressed_override = None;
        self.status_disabled_override = None;
        self.status_focused_override = None;
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ChangeView(view) => {
                if self.selected_view != view {
                    self.selected_view = view;
                    self.reset_style_editor(view);
                } else {
                    return Task::none();
                }
            }
            Message::Edit(action) => {
                match action {
                    text_editor::Action::Edit(_edit) => return Task::none(),
                    _ => {
                        self.style_code_content.perform(action);
                    }
                }
                return Task::none();
            }
            Message::CopyCode(code) => return clipboard::write(code),
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
            Message::UpdateBorderRadiusBottomRight(radius) => {
                self.border_radius_bottom_right = radius
            }
            Message::UpdateBorderRadiusBottomLeft(radius) => {
                self.border_radius_bottom_left = radius
            }
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
            Message::UpdateRuleFillMode(mode) => self.rule_fill_mode = mode,
            Message::UpdateIconColor { color, source } => {
                self.icon_color = color;
                self.icon_color_source = source;
            }
            Message::UpdatePlaceholderColor { color, source } => {
                self.placeholder_color = color;
                self.placeholder_color_source = source;
            }
            Message::UpdateSelectionColor { color, source } => {
                self.selection_color = color;
                self.selection_color_source = source;
            }
            Message::UpdateSelectedTextColor { color, source } => {
                self.selected_text_color = color;
                self.selected_text_color_source = source;
            }
            Message::UpdateSelectedBackgroundColor { color, source } => {
                self.selected_background_color = color;
                self.selected_background_color_source = source;
            }

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
                        rule_fill_mode: self.rule_fill_mode.clone(),
                        icon_color: self.icon_color,
                        icon_color_source: self.icon_color_source.clone(),
                        placeholder_color: self.placeholder_color,
                        placeholder_color_source: self.placeholder_color_source.clone(),
                        selection_color: self.selection_color,
                        selection_color_source: self.selection_color_source.clone(),
                        selected_text_color: self.selected_text_color,
                        selected_text_color_source: self.selected_text_color_source.clone(),
                        selected_background_color: self.selected_background_color,
                        selected_background_color_source: self
                            .selected_background_color_source
                            .clone(),
                        status_hovered: self.status_hovered_override.clone(),
                        status_pressed: self.status_pressed_override.clone(),
                        status_disabled: self.status_disabled_override.clone(),
                        status_focused: self.status_focused_override.clone(),
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
                        self.rule_fill_mode = definition.rule_fill_mode.clone();

                        match &definition.text_color_source {
                            Some(expression) => {
                                self.text_color =
                                    evaluate_theme_expression(&self.theme, &expression)
                                        .unwrap_or(definition.text_color);
                                self.text_color_source = definition.text_color_source.clone();
                            }
                            None => {
                                self.text_color = definition.text_color;
                                self.text_color_source = None;
                            }
                        }
                        match &definition.background_color_source {
                            Some(expression) => {
                                self.background_color =
                                    evaluate_theme_expression(&self.theme, &expression)
                                        .unwrap_or(definition.background_color);
                                self.background_color_source =
                                    definition.background_color_source.clone();
                            }
                            None => {
                                self.background_color = definition.background_color;
                                self.background_color_source = None;
                            }
                        }
                        match &definition.border_color_source {
                            Some(expression) => {
                                self.border_color =
                                    evaluate_theme_expression(&self.theme, &expression)
                                        .unwrap_or(definition.border_color);
                                self.border_color_source = definition.border_color_source.clone();
                            }
                            None => {
                                self.border_color = definition.border_color;
                                self.border_color_source = None;
                            }
                        }
                        match &definition.shadow_color_source {
                            Some(expression) => {
                                self.shadow_color =
                                    evaluate_theme_expression(&self.theme, &expression)
                                        .unwrap_or(definition.shadow_color);
                                self.shadow_color_source = definition.shadow_color_source.clone();
                            }
                            None => {
                                self.shadow_color = definition.shadow_color;
                                self.shadow_color_source = None;
                            }
                        }
                        match &definition.icon_color_source {
                            Some(expression) => {
                                self.icon_color =
                                    evaluate_theme_expression(&self.theme, &expression)
                                        .unwrap_or(definition.icon_color);
                                self.icon_color_source = definition.icon_color_source.clone();
                            }
                            None => {
                                self.icon_color = definition.icon_color;
                                self.icon_color_source = None;
                            }
                        }
                        match &definition.placeholder_color_source {
                            Some(expression) => {
                                self.placeholder_color =
                                    evaluate_theme_expression(&self.theme, &expression)
                                        .unwrap_or(definition.placeholder_color);
                                self.placeholder_color_source =
                                    definition.placeholder_color_source.clone();
                            }
                            None => {
                                self.placeholder_color = definition.placeholder_color;
                                self.placeholder_color_source = None;
                            }
                        }
                        match &definition.selection_color_source {
                            Some(expression) => {
                                self.selection_color =
                                    evaluate_theme_expression(&self.theme, &expression)
                                        .unwrap_or(definition.selection_color);
                                self.selection_color_source =
                                    definition.selection_color_source.clone();
                            }
                            None => {
                                self.selection_color = definition.selection_color;
                                self.selection_color_source = None;
                            }
                        }
                        match &definition.selected_text_color_source {
                            Some(expression) => {
                                self.selected_text_color =
                                    evaluate_theme_expression(&self.theme, &expression)
                                        .unwrap_or(definition.selected_text_color);
                                self.selected_text_color_source =
                                    definition.selected_text_color_source.clone();
                            }
                            None => {
                                self.selected_text_color = definition.selected_text_color;
                                self.selected_text_color_source = None;
                            }
                        }
                        match &definition.selected_background_color_source {
                            Some(expression) => {
                                self.selected_background_color =
                                    evaluate_theme_expression(&self.theme, &expression)
                                        .unwrap_or(definition.selected_background_color);
                                self.selected_background_color_source =
                                    definition.selected_background_color_source.clone();
                            }
                            None => {
                                self.selected_background_color =
                                    definition.selected_background_color;
                                self.selected_background_color_source = None;
                            }
                        }
                        self.status_hovered_override = definition.status_hovered.clone();
                        self.status_pressed_override = definition.status_pressed.clone();
                        self.status_disabled_override = definition.status_disabled.clone();
                        self.status_focused_override = definition.status_focused.clone();
                        self.editing_status = EditingStatus::Active;
                    }
                }
            }

            Message::ResetToDefault => {
                //                self.reset_to_theme();
                self.reset_style_editor(self.selected_view);
                return Task::none();
            }

            Message::SetEditingStatus(status) => {
                self.editing_status = status;
                return Task::none();
            }
            Message::ResetStatusOverride(status) => match status {
                EditingStatus::Hovered => self.status_hovered_override = None,
                EditingStatus::Pressed => self.status_pressed_override = None,
                EditingStatus::Disabled => self.status_disabled_override = None,
                EditingStatus::Focused => self.status_focused_override = None,
                EditingStatus::Active => {}
            },
            Message::UpdateStatusColor {
                status,
                field,
                color,
                source,
            } => {
                let ov = match status {
                    EditingStatus::Hovered => self
                        .status_hovered_override
                        .get_or_insert_with(StatusColorOverride::default),
                    EditingStatus::Pressed => self
                        .status_pressed_override
                        .get_or_insert_with(StatusColorOverride::default),
                    EditingStatus::Disabled => self
                        .status_disabled_override
                        .get_or_insert_with(StatusColorOverride::default),
                    EditingStatus::Focused => self
                        .status_focused_override
                        .get_or_insert_with(StatusColorOverride::default),
                    EditingStatus::Active => {
                        return Task::none();
                    }
                };
                match field {
                    StatusColorField::Text => {
                        ov.text_color = Some(color);
                        ov.text_color_source = source;
                    }
                    StatusColorField::Background => {
                        ov.background_color = Some(color);
                        ov.background_color_source = source;
                    }
                    StatusColorField::Border => {
                        ov.border_color = Some(color);
                        ov.border_color_source = source;
                    }
                    StatusColorField::Icon => {
                        ov.icon_color = Some(color);
                        ov.icon_color_source = source;
                    }
                }
            }
            Message::Noop => {}
            Message::ComboboxSelected(value) => {
                self.preview_value = value;
            }
        }
        match self.selected_view {
            ThemePaneEnum::Button => {
                self.regenerate_button_code();
            }
            ThemePaneEnum::Container => {
                self.regenerate_container_code();
            }
            ThemePaneEnum::Checkbox => {
                self.regenerate_checkbox_code();
            }
            ThemePaneEnum::TextInput => {
                self.regenerate_text_input_code();
            }
            ThemePaneEnum::Menu => {
                self.regenerate_menu_code();
            }
            ThemePaneEnum::Picklist => {
                self.regenerate_pick_list_code();
            }
            ThemePaneEnum::Slider => {
                self.regenerate_slider_code();
            }
            ThemePaneEnum::Progressbar => {
                self.regenerate_progress_bar_code();
            }
            ThemePaneEnum::Radio => {
                self.regenerate_radio_code();
            }
            ThemePaneEnum::Toggler => {
                self.regenerate_toggler_code();
            }
            ThemePaneEnum::Combobox => {
                self.regenerate_combo_box_code();
            }
            ThemePaneEnum::Rule => {
                self.regenerate_rule_code();
            }
            _ => {}
        }

        Task::none()
    }

    fn regenerate_button_code(&mut self) {
        let code = generate_button_style_code(
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
            self.snap,
            self.status_hovered_override.as_ref(),
            self.status_pressed_override.as_ref(),
            self.status_disabled_override.as_ref(),
        );
        self.style_code_content = text_editor::Content::with_text(&code);
    }

    fn regenerate_container_code(&mut self) {
        let code = generate_container_style_code(
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
            self.snap,
        );
        self.style_code_content = text_editor::Content::with_text(&code);
    }

    fn regenerate_checkbox_code(&mut self) {
        let code = generate_checkbox_style_code(
            &self.style_name,
            self.text_color,
            &self.text_color_source,
            self.background_color,
            &self.background_color_source,
            self.icon_color,
            &self.icon_color_source,
            self.border_color,
            &self.border_color_source,
            self.border_width,
            self.border_radius_top_left,
            self.border_radius_top_right,
            self.border_radius_bottom_right,
            self.border_radius_bottom_left,
            self.status_hovered_override.as_ref(),
            self.status_disabled_override.as_ref(),
        );
        self.style_code_content = text_editor::Content::with_text(&code);
    }

    fn regenerate_text_input_code(&mut self) {
        let code = generate_text_input_style_code(
            &self.style_name,
            self.background_color,
            &self.background_color_source,
            self.border_color,
            &self.border_color_source,
            self.border_width,
            self.border_radius_top_left,
            self.border_radius_top_right,
            self.border_radius_bottom_right,
            self.border_radius_bottom_left,
            self.icon_color,
            &self.icon_color_source,
            self.placeholder_color,
            &self.placeholder_color_source,
            self.text_color,
            &self.text_color_source,
            self.selection_color,
            &self.selection_color_source,
            self.status_hovered_override.as_ref(),
            self.status_focused_override.as_ref(),
            self.status_disabled_override.as_ref(),
        );
        self.style_code_content = text_editor::Content::with_text(&code);
    }

    fn regenerate_menu_code(&mut self) {
        let code = generate_menu_style_code(
            &self.style_name,
            self.background_color,
            &self.background_color_source,
            self.border_color,
            &self.border_color_source,
            self.border_width,
            self.border_radius_top_left,
            self.border_radius_top_right,
            self.border_radius_bottom_right,
            self.border_radius_bottom_left,
            self.text_color,
            &self.text_color_source,
            self.selected_text_color,
            &self.selected_text_color_source,
            self.selected_background_color,
            &self.selected_background_color_source,
            self.shadow_enabled,
            self.shadow_color,
            &self.shadow_color_source,
            self.shadow_offset_x,
            self.shadow_offset_y,
            self.shadow_blur_radius,
        );
        self.style_code_content = text_editor::Content::with_text(&code);
    }

    fn regenerate_pick_list_code(&mut self) {
        let code = generate_pick_list_style_code(
            &self.style_name,
            self.text_color,
            &self.text_color_source,
            self.background_color,
            &self.background_color_source,
            self.placeholder_color,
            &self.placeholder_color_source,
            self.icon_color,
            &self.icon_color_source,
            self.border_color,
            &self.border_color_source,
            self.border_width,
            self.border_radius_top_left,
            self.border_radius_top_right,
            self.border_radius_bottom_right,
            self.border_radius_bottom_left,
            self.status_hovered_override.as_ref(),
            self.status_focused_override.as_ref(),
        );
        self.style_code_content = text_editor::Content::with_text(&code);
    }

    fn regenerate_slider_code(&mut self) {
        let code = generate_slider_style_code(
            &self.style_name,
            self.text_color,
            &self.text_color_source,
            self.background_color,
            &self.background_color_source,
            self.icon_color,
            &self.icon_color_source,
            self.border_color,
            &self.border_color_source,
            self.border_width,
            self.border_radius_top_left,
            self.border_radius_top_right,
            self.border_radius_bottom_right,
            self.border_radius_bottom_left,
            self.status_hovered_override.as_ref(),
            self.status_pressed_override.as_ref(),
        );
        self.style_code_content = text_editor::Content::with_text(&code);
    }

    fn regenerate_progress_bar_code(&mut self) {
        let code = generate_progress_bar_style_code(
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
        );
        self.style_code_content = text_editor::Content::with_text(&code);
    }

    fn regenerate_radio_code(&mut self) {
        let code = generate_radio_style_code(
            &self.style_name,
            self.text_color,
            &self.text_color_source,
            self.background_color,
            &self.background_color_source,
            self.icon_color,
            &self.icon_color_source,
            self.border_color,
            &self.border_color_source,
            self.border_width,
            self.status_hovered_override.as_ref(),
        );
        self.style_code_content = text_editor::Content::with_text(&code);
    }

    fn regenerate_toggler_code(&mut self) {
        let code = generate_toggler_style_code(
            &self.style_name,
            self.text_color,
            &self.text_color_source,
            self.background_color,
            &self.background_color_source,
            self.icon_color,
            &self.icon_color_source,
            self.border_color,
            &self.border_color_source,
            self.border_width,
            self.border_radius_top_left,
            self.border_radius_top_right,
            self.border_radius_bottom_right,
            self.border_radius_bottom_left,
            self.status_hovered_override.as_ref(),
            self.status_disabled_override.as_ref(),
        );
        self.style_code_content = text_editor::Content::with_text(&code);
    }

    fn regenerate_combo_box_code(&mut self) {
        let code = generate_combo_box_style_code(
            &self.style_name,
            self.background_color,
            &self.background_color_source,
            self.border_color,
            &self.border_color_source,
            self.border_width,
            self.border_radius_top_left,
            self.border_radius_top_right,
            self.border_radius_bottom_right,
            self.border_radius_bottom_left,
            self.icon_color,
            &self.icon_color_source,
            self.placeholder_color,
            &self.placeholder_color_source,
            self.text_color,
            &self.text_color_source,
            self.selection_color,
            &self.selection_color_source,
            self.selected_text_color,
            &self.selected_text_color_source,
            self.selected_background_color,
            &self.selected_background_color_source,
            self.shadow_enabled,
            self.shadow_color,
            &self.shadow_color_source,
            self.shadow_offset_x,
            self.shadow_offset_y,
            self.shadow_blur_radius,
            self.status_hovered_override.as_ref(),
            self.status_focused_override.as_ref(),
            self.status_disabled_override.as_ref(),
        );
        self.style_code_content = text_editor::Content::with_text(&code);
    }

    fn regenerate_rule_code(&mut self) {
        let code = generate_rule_style_code(
            &self.style_name,
            self.border_color,
            &self.border_color_source,
            self.border_radius_top_left,
            self.border_radius_top_right,
            self.border_radius_bottom_right,
            self.border_radius_bottom_left,
            &self.rule_fill_mode,
            self.snap,
        );
        self.style_code_content = text_editor::Content::with_text(&code);
    }

    pub fn view<'a>(&'a self, theme: &'a Theme) -> Element<'a, Message> {
        let content = match self.selected_view {
            ThemePaneEnum::ExtendedPalette => self.show_theme_colors(theme),
            ThemePaneEnum::Container => self.show_style_builder(theme),
            ThemePaneEnum::Button => self.show_style_builder(theme),
            _ => self.show_style_builder(theme),
        };

        column![
            container(text("Themes and Styles").size(20),).center_x(Length::Fill),
            row![
                button("Palette Viewer")
                    .style(if self.selected_view == ThemePaneEnum::ExtendedPalette {
                        styles::button::selected_text
                    } else {
                        button::text
                    })
                    .on_press(Message::ChangeView(ThemePaneEnum::ExtendedPalette)),
                button("Button")
                    .style(if self.selected_view == ThemePaneEnum::Button {
                        styles::button::selected_text
                    } else {
                        button::text
                    })
                    .on_press(Message::ChangeView(ThemePaneEnum::Button)),
                button("Checkbox")
                    .style(if self.selected_view == ThemePaneEnum::Checkbox {
                        styles::button::selected_text
                    } else {
                        button::text
                    })
                    .on_press(Message::ChangeView(ThemePaneEnum::Checkbox)),
                button("Container")
                    .style(if self.selected_view == ThemePaneEnum::Container {
                        styles::button::selected_text
                    } else {
                        button::text
                    })
                    .on_press(Message::ChangeView(ThemePaneEnum::Container)),
                button("Menu")
                    .style(if self.selected_view == ThemePaneEnum::Menu {
                        styles::button::selected_text
                    } else {
                        button::text
                    })
                    .on_press(Message::ChangeView(ThemePaneEnum::Menu)),
                button("Pane Grid").style(if self.selected_view == ThemePaneEnum::Panegrid {
                    styles::button::selected_text
                } else {
                    button::text
                }), //.on_press(Message::ChangeView(ThemePaneEnum::Panegrid)),
                button("Picklist")
                    .style(if self.selected_view == ThemePaneEnum::Picklist {
                        styles::button::selected_text
                    } else {
                        button::text
                    })
                    .on_press(Message::ChangeView(ThemePaneEnum::Picklist)),
                button("Progressbar")
                    .style(if self.selected_view == ThemePaneEnum::Progressbar {
                        styles::button::selected_text
                    } else {
                        button::text
                    })
                    .on_press(Message::ChangeView(ThemePaneEnum::Progressbar)),
                button("QR Code").style(if self.selected_view == ThemePaneEnum::QRCode {
                    styles::button::selected_text
                } else {
                    button::text
                }), //.on_press(Message::ChangeView(ThemePaneEnum::QRCode)),
                button("Radio")
                    .style(if self.selected_view == ThemePaneEnum::Radio {
                        styles::button::selected_text
                    } else {
                        button::text
                    })
                    .on_press(Message::ChangeView(ThemePaneEnum::Radio)),
                button("Rule")
                    .style(if self.selected_view == ThemePaneEnum::Rule {
                        styles::button::selected_text
                    } else {
                        button::text
                    })
                    .on_press(Message::ChangeView(ThemePaneEnum::Rule)),
                button("Slider")
                    .style(if self.selected_view == ThemePaneEnum::Slider {
                        styles::button::selected_text
                    } else {
                        button::text
                    })
                    .on_press(Message::ChangeView(ThemePaneEnum::Slider)),
                button("Table").style(if self.selected_view == ThemePaneEnum::Table {
                    styles::button::selected_text
                } else {
                    button::text
                }), //.on_press(Message::ChangeView(ThemePaneEnum::Table)),
                button("Text Editor").style(if self.selected_view == ThemePaneEnum::TextEditor {
                    styles::button::selected_text
                } else {
                    button::text
                }), //.on_press(Message::ChangeView(ThemePaneEnum::TextEditor)),
                button("Text Input")
                    .style(if self.selected_view == ThemePaneEnum::TextInput {
                        styles::button::selected_text
                    } else {
                        button::text
                    })
                    .on_press(Message::ChangeView(ThemePaneEnum::TextInput)),
                button("Toggler")
                    .style(if self.selected_view == ThemePaneEnum::Toggler {
                        styles::button::selected_text
                    } else {
                        button::text
                    })
                    .on_press(Message::ChangeView(ThemePaneEnum::Toggler)),
            ]
            .spacing(10)
            .wrap(),
            content
        ]
        .spacing(10)
        .into()
    }

    pub fn show_style_builder<'a>(&'a self, theme: &'a Theme) -> Element<'a, Message> {
        let has_statuses = matches!(
            self.selected_view,
            ThemePaneEnum::Button
                | ThemePaneEnum::Checkbox
                | ThemePaneEnum::TextInput
                | ThemePaneEnum::Picklist
                | ThemePaneEnum::Slider
                | ThemePaneEnum::Radio
                | ThemePaneEnum::Toggler
                | ThemePaneEnum::Combobox
        );
        let is_active_status = self.editing_status == EditingStatus::Active;
        let status = self.editing_status;
        let status_override = match status {
            EditingStatus::Hovered => self.status_hovered_override.as_ref(),
            EditingStatus::Pressed => self.status_pressed_override.as_ref(),
            EditingStatus::Disabled => self.status_disabled_override.as_ref(),
            EditingStatus::Focused => self.status_focused_override.as_ref(),
            EditingStatus::Active => None,
        };
        let resolve_override_color =
            |color: Option<Color>, source: Option<&String>, fallback: Color| -> Color {
                source
                    .and_then(|expr| evaluate_theme_expression(theme, expr))
                    .or(color)
                    .unwrap_or(fallback)
            };
        let current_text_color = if is_active_status {
            self.text_color
        } else {
            resolve_override_color(
                status_override.and_then(|ov| ov.text_color),
                status_override.and_then(|ov| ov.text_color_source.as_ref()),
                self.text_color,
            )
        };
        let current_background_color = if is_active_status {
            self.background_color
        } else {
            resolve_override_color(
                status_override.and_then(|ov| ov.background_color),
                status_override.and_then(|ov| ov.background_color_source.as_ref()),
                self.background_color,
            )
        };
        let current_border_color = if is_active_status {
            self.border_color
        } else {
            resolve_override_color(
                status_override.and_then(|ov| ov.border_color),
                status_override.and_then(|ov| ov.border_color_source.as_ref()),
                self.border_color,
            )
        };
        let current_icon_color = if is_active_status {
            self.icon_color
        } else {
            resolve_override_color(
                status_override.and_then(|ov| ov.icon_color),
                status_override.and_then(|ov| ov.icon_color_source.as_ref()),
                self.icon_color,
            )
        };

        let content = column![
            container(text(format!("{} Style", self.selected_view)).size(20))
                .center_x(Length::Fill),
            // Status tab row for widgets with Status variants
            if has_statuses {
                let statuses: &[(&str, EditingStatus)] = match self.selected_view {
                    ThemePaneEnum::Button => &[
                        ("Active", EditingStatus::Active),
                        ("Hovered", EditingStatus::Hovered),
                        ("Pressed", EditingStatus::Pressed),
                        ("Disabled", EditingStatus::Disabled),
                    ],
                    ThemePaneEnum::Checkbox => &[
                        ("Active", EditingStatus::Active),
                        ("Hovered", EditingStatus::Hovered),
                        ("Disabled", EditingStatus::Disabled),
                    ],
                    ThemePaneEnum::TextInput => &[
                        ("Active", EditingStatus::Active),
                        ("Hovered", EditingStatus::Hovered),
                        ("Focused", EditingStatus::Focused),
                        ("Disabled", EditingStatus::Disabled),
                    ],
                    ThemePaneEnum::Picklist => &[
                        ("Active", EditingStatus::Active),
                        ("Hovered", EditingStatus::Hovered),
                        ("Opened", EditingStatus::Focused),
                    ],
                    ThemePaneEnum::Slider => &[
                        ("Active", EditingStatus::Active),
                        ("Hovered", EditingStatus::Hovered),
                        ("Dragged", EditingStatus::Pressed),
                    ],
                    ThemePaneEnum::Radio => &[
                        ("Active", EditingStatus::Active),
                        ("Hovered", EditingStatus::Hovered),
                    ],
                    ThemePaneEnum::Toggler => &[
                        ("Active", EditingStatus::Active),
                        ("Hovered", EditingStatus::Hovered),
                        ("Disabled", EditingStatus::Disabled),
                    ],
                    ThemePaneEnum::Combobox => &[
                        ("Active", EditingStatus::Active),
                        ("Hovered", EditingStatus::Hovered),
                        ("Focused", EditingStatus::Focused),
                        ("Disabled", EditingStatus::Disabled),
                    ],
                    _ => &[],
                };
                let mut r = row![].spacing(4);
                for (label, s) in statuses {
                    let s = *s;
                    let has_override = match s {
                        EditingStatus::Hovered => self.status_hovered_override.is_some(),
                        EditingStatus::Pressed => self.status_pressed_override.is_some(),
                        EditingStatus::Disabled => self.status_disabled_override.is_some(),
                        EditingStatus::Focused => self.status_focused_override.is_some(),
                        EditingStatus::Active => false,
                    };
                    let display = if has_override {
                        format!("{}*", label)
                    } else {
                        label.to_string()
                    };
                    r = r.push(
                        button(text(display))
                            .style(if self.editing_status == s {
                                styles::button::selected_text
                            } else {
                                button::text
                            })
                            .on_press(Message::SetEditingStatus(s)),
                    );
                }
                Element::from(r)
            } else {
                Element::from(row![])
            },
            if self.selected_view != ThemePaneEnum::Rule {
                Element::from(
                    row![
                        column![
                            container(text("text_color").size(16)).center_x(Length::Fill),
                            color_picker::ColorButton::new(current_text_color)
                                .on_change_with_source(move |color, source| {
                                    if is_active_status {
                                        Message::UpdateTextColor { color, source }
                                    } else {
                                        Message::UpdateStatusColor {
                                            status,
                                            field: StatusColorField::Text,
                                            color,
                                            source,
                                        }
                                    }
                                })
                                .title(if is_active_status {
                                    "text_color"
                                } else {
                                    "text_color (override)"
                                })
                                .width(Length::Fill)
                                .height(Length::Fixed(50.0))
                                .show_hex(),
                        ]
                        .width(Length::FillPortion(1)),
                        column![
                            container(text("background color").size(16)).center_x(Length::Fill),
                            color_picker::ColorButton::new(current_background_color)
                                .on_change_with_source(move |color, source| {
                                    if is_active_status {
                                        Message::UpdateBackgroundColor { color, source }
                                    } else {
                                        Message::UpdateStatusColor {
                                            status,
                                            field: StatusColorField::Background,
                                            color,
                                            source,
                                        }
                                    }
                                })
                                .title(if is_active_status {
                                    "background color"
                                } else {
                                    "background color (override)"
                                })
                                .width(Length::Fill)
                                .height(Length::Fixed(50.0))
                                .show_hex(),
                        ]
                        .width(Length::FillPortion(1)),
                    ]
                    .spacing(10),
                )
            } else {
                Element::from(row![])
            },
            if matches!(
                self.selected_view,
                ThemePaneEnum::Checkbox
                    | ThemePaneEnum::TextInput
                    | ThemePaneEnum::Picklist
                    | ThemePaneEnum::Slider
                    | ThemePaneEnum::Radio
                    | ThemePaneEnum::Toggler
                    | ThemePaneEnum::Combobox
            ) {
                row![
                    column![
                        container(text("icon_color").size(16)).center_x(Length::Fill),
                        color_picker::ColorButton::new(current_icon_color)
                            .on_change_with_source(move |color, source| {
                                if is_active_status {
                                    Message::UpdateIconColor { color, source }
                                } else {
                                    Message::UpdateStatusColor {
                                        status,
                                        field: StatusColorField::Icon,
                                        color,
                                        source,
                                    }
                                }
                            })
                            .title(if is_active_status {
                                "icon_color"
                            } else {
                                "icon_color (override)"
                            })
                            .width(Length::Fill)
                            .height(Length::Fixed(50.0))
                            .show_hex(),
                    ]
                    .width(Length::FillPortion(1)),
                ]
                .spacing(10)
            } else {
                row![]
            },
            if matches!(
                self.selected_view,
                ThemePaneEnum::TextInput
                    | ThemePaneEnum::Menu
                    | ThemePaneEnum::Picklist
                    | ThemePaneEnum::Combobox
            ) && self.editing_status == EditingStatus::Active
            {
                column![
                    row![
                        if matches!(
                            self.selected_view,
                            ThemePaneEnum::TextInput
                                | ThemePaneEnum::Picklist
                                | ThemePaneEnum::Combobox
                        ) {
                            Element::from(
                                column![
                                    container(text("placeholder color").size(16))
                                        .center_x(Length::Fill),
                                    color_picker::ColorButton::new(self.placeholder_color)
                                        .on_change_with_source(|color, source| {
                                            Message::UpdatePlaceholderColor { color, source }
                                        })
                                        .title("placeholder color")
                                        .width(Length::Fill)
                                        .height(Length::Fixed(50.0))
                                        .show_hex(),
                                ]
                                .width(Length::FillPortion(1)),
                            )
                        } else {
                            Element::from(column![].width(Length::FillPortion(1)))
                        },
                        if matches!(
                            self.selected_view,
                            ThemePaneEnum::TextInput | ThemePaneEnum::Combobox
                        ) {
                            Element::from(
                                column![
                                    container(text("selection color").size(16))
                                        .center_x(Length::Fill),
                                    color_picker::ColorButton::new(self.selection_color)
                                        .on_change_with_source(|color, source| {
                                            Message::UpdateSelectionColor { color, source }
                                        })
                                        .title("selection color")
                                        .width(Length::Fill)
                                        .height(Length::Fixed(50.0))
                                        .show_hex(),
                                ]
                                .width(Length::FillPortion(1)),
                            )
                        } else {
                            Element::from(column![].width(Length::FillPortion(1)))
                        },
                    ]
                    .spacing(10),
                    if matches!(
                        self.selected_view,
                        ThemePaneEnum::Menu | ThemePaneEnum::Combobox
                    ) {
                        Element::from(
                            column![
                                container(text("Menu Style").size(18)).center_x(Length::Fill),
                                row![
                                    column![
                                        container(text("selected text color").size(16))
                                            .center_x(Length::Fill),
                                        color_picker::ColorButton::new(self.selected_text_color)
                                            .on_change_with_source(|color, source| {
                                                Message::UpdateSelectedTextColor { color, source }
                                            })
                                            .title("selected text color")
                                            .width(Length::Fill)
                                            .height(Length::Fixed(50.0))
                                            .show_hex(),
                                    ]
                                    .width(Length::FillPortion(1)),
                                    column![
                                        container(text("selected bg color").size(16))
                                            .center_x(Length::Fill),
                                        color_picker::ColorButton::new(
                                            self.selected_background_color
                                        )
                                        .on_change_with_source(|color, source| {
                                            Message::UpdateSelectedBackgroundColor { color, source }
                                        })
                                        .title("selected background color")
                                        .width(Length::Fill)
                                        .height(Length::Fixed(50.0))
                                        .show_hex(),
                                    ]
                                    .width(Length::FillPortion(1)),
                                ]
                                .spacing(10),
                            ]
                            .spacing(10),
                        )
                    } else {
                        Element::from(column![])
                    },
                ]
                .spacing(10)
            } else {
                column![]
            },
            if self.selected_view == ThemePaneEnum::Rule {
                column![
                    container(text("Fill Mode").size(20)).center_x(Length::Fill),
                    row![
                        button("Full")
                            .style(if matches!(self.rule_fill_mode, RuleFillMode::Full) {
                                styles::button::selected_text
                            } else {
                                button::text
                            })
                            .on_press(Message::UpdateRuleFillMode(RuleFillMode::Full)),
                        button("Percent")
                            .style(if matches!(self.rule_fill_mode, RuleFillMode::Percent(_)) {
                                styles::button::selected_text
                            } else {
                                button::text
                            })
                            .on_press(Message::UpdateRuleFillMode(RuleFillMode::Percent(80.0))),
                        button("Padded")
                            .style(if matches!(self.rule_fill_mode, RuleFillMode::Padded(_)) {
                                styles::button::selected_text
                            } else {
                                button::text
                            })
                            .on_press(Message::UpdateRuleFillMode(RuleFillMode::Padded(10))),
                        button("Asymmetric")
                            .style(
                                if matches!(
                                    self.rule_fill_mode,
                                    RuleFillMode::AsymmetricPadding(_, _)
                                ) {
                                    styles::button::selected_text
                                } else {
                                    button::text
                                }
                            )
                            .on_press(Message::UpdateRuleFillMode(
                                RuleFillMode::AsymmetricPadding(10, 20)
                            )),
                    ]
                    .spacing(5)
                    .wrap(),
                    match &self.rule_fill_mode {
                        RuleFillMode::Percent(p) => {
                            let p = *p;
                            column![
                                text(format!("Percent: {:.0}%", p)).size(14),
                                slider(1.0..=100.0, p, |v| Message::UpdateRuleFillMode(
                                    RuleFillMode::Percent(v)
                                ))
                                .step(1.0),
                            ]
                            .spacing(5)
                        }
                        RuleFillMode::Padded(pad) => {
                            let pad = *pad;
                            column![
                                text(format!("Padding: {}", pad)).size(14),
                                slider(0.0..=100.0, pad as f32, |v| Message::UpdateRuleFillMode(
                                    RuleFillMode::Padded(v as u16)
                                ))
                                .step(1.0),
                            ]
                            .spacing(5)
                        }
                        RuleFillMode::AsymmetricPadding(a, b) => {
                            let (a, b) = (*a, *b);
                            column![
                                text(format!("Left: {}  Right: {}", a, b)).size(14),
                                slider(
                                    0.0..=100.0,
                                    a as f32,
                                    move |v| Message::UpdateRuleFillMode(
                                        RuleFillMode::AsymmetricPadding(v as u16, b)
                                    )
                                )
                                .step(1.0),
                                slider(
                                    0.0..=100.0,
                                    b as f32,
                                    move |v| Message::UpdateRuleFillMode(
                                        RuleFillMode::AsymmetricPadding(a, v as u16)
                                    )
                                )
                                .step(1.0),
                            ]
                            .spacing(5)
                        }
                        _ => column![],
                    },
                ]
                .spacing(10)
            } else {
                column![]
            },
            column![
                container(text("Border").size(20)).center_x(Length::Fill),
                row![
                    column![
                        text("Width:").size(16),
                        slider(0.0..=30.0, self.border_width, Message::UpdateBorderWidth).step(1.0),
                        text(format!("{:.0}", self.border_width)).size(12).center(),
                    ]
                    .width(Length::FillPortion(1))
                    .align_x(Alignment::Center),
                    column![
                        container(text("border color").size(16)).center_x(Length::Fill),
                        color_picker::ColorButton::new(current_border_color)
                            .on_change_with_source(move |color, source| {
                                if is_active_status {
                                    Message::UpdateBorderColor { color, source }
                                } else {
                                    Message::UpdateStatusColor {
                                        status,
                                        field: StatusColorField::Border,
                                        color,
                                        source,
                                    }
                                }
                            })
                            .title(if is_active_status {
                                "border color"
                            } else {
                                "border color (override)"
                            })
                            .width(Length::Fill)
                            .height(Length::Fixed(50.0))
                            .show_hex(),
                    ]
                    .width(Length::FillPortion(1)),
                    if has_statuses && !is_active_status {
                        Element::from(
                            button("Reset to auto")
                                .style(button::text)
                                .on_press(Message::ResetStatusOverride(status)),
                        )
                    } else {
                        Element::from(Space::new())
                    },
                ]
                .spacing(10)
                .align_y(Alignment::Center),
                container(text("border radius").size(18)).center_x(Length::Fill),
                row![
                    column![
                        text("Top left").size(16),
                        slider(
                            0.0..=30.0,
                            self.border_radius_top_left,
                            Message::UpdateBorderRadiusTopLeft
                        )
                        .step(1.0),
                        text(format!("{:.0}", self.border_radius_top_left))
                            .size(12)
                            .center(),
                    ]
                    .spacing(5),
                    column![
                        text("Top right").size(16),
                        slider(
                            0.0..=30.0,
                            self.border_radius_top_right,
                            Message::UpdateBorderRadiusTopRight
                        )
                        .step(1.0),
                        text(format!("{:.0}", self.border_radius_top_right))
                            .size(12)
                            .center(),
                    ]
                    .spacing(5),
                ]
                .spacing(10),
                row![
                    column![
                        text("Bottom left").size(16),
                        slider(
                            0.0..=30.0,
                            self.border_radius_bottom_left,
                            Message::UpdateBorderRadiusBottomLeft
                        )
                        .step(1.0),
                        text(format!("{:.0}", self.border_radius_bottom_left))
                            .size(12)
                            .center(),
                    ]
                    .spacing(5),
                    column![
                        text("Bottom right").size(16),
                        slider(
                            0.0..=30.0,
                            self.border_radius_bottom_right,
                            Message::UpdateBorderRadiusBottomRight
                        )
                        .step(1.0),
                        text(format!("{:.0}", self.border_radius_bottom_right))
                            .size(12)
                            .center(),
                    ]
                    .spacing(5),
                ]
                .spacing(10),
            ]
            .spacing(10),
            if matches!(
                self.selected_view,
                ThemePaneEnum::Container
                    | ThemePaneEnum::Button
                    | ThemePaneEnum::Menu
                    | ThemePaneEnum::Combobox
            ) {
                column![
                    container(text("Shadow").size(20)).center_x(Length::Fill),
                    row![
                        column![
                            Space::new().width(Length::Fill).height(Length::Fixed(10.0)),
                            checkbox(self.shadow_enabled)
                                .on_toggle(Message::UpdateShadowEnabled)
                                .label("Enable Shadow"),
                        ]
                        .width(Length::FillPortion(1))
                        .height(Length::Fixed(50.0)),
                        if self.shadow_enabled {
                            column![
                                container(text("shadow color").size(16)).center_x(Length::Fill),
                                color_picker::ColorButton::new(self.shadow_color)
                                    .on_change_with_source(|color, source| {
                                        Message::UpdateShadowColor { color, source }
                                    })
                                    .title("shadow color")
                                    .width(Length::Fill)
                                    .height(Length::Fixed(50.0))
                                    .show_hex()
                            ]
                            .width(Length::FillPortion(1))
                        } else {
                            column![].width(Length::FillPortion(1))
                        }
                    ]
                    .align_y(Alignment::Start),
                    if self.shadow_enabled {
                        column![
                            row![
                                column![
                                    text("Offset X").size(12),
                                    slider(
                                        -20.0..=20.0,
                                        self.shadow_offset_x,
                                        Message::UpdateShadowOffsetX
                                    )
                                    .step(1.0),
                                    text(format!("{:.0}", self.shadow_offset_x))
                                        .size(12)
                                        .center(),
                                ],
                                column![
                                    text("Offset Y").size(12),
                                    slider(
                                        -20.0..=20.0,
                                        self.shadow_offset_y,
                                        Message::UpdateShadowOffsetY
                                    )
                                    .step(1.0),
                                    text(format!("{:.0}", self.shadow_offset_y))
                                        .size(12)
                                        .center(),
                                ],
                            ]
                            .spacing(15),
                            column![
                                text("Blur Radius").size(12),
                                slider(
                                    0.0..=50.0,
                                    self.shadow_blur_radius,
                                    Message::UpdateShadowBlurRadius
                                )
                                .step(1.0),
                                text(format!("{:.0}", self.shadow_blur_radius))
                                    .size(12)
                                    .center(),
                            ],
                        ]
                        .spacing(10)
                    } else {
                        column![]
                    },
                ]
                .spacing(10)
            } else {
                column![]
            },
            if matches!(
                self.selected_view,
                ThemePaneEnum::Container | ThemePaneEnum::Button | ThemePaneEnum::Rule
            ) {
                column![
                    container(text("Snap").size(20)).center_x(Length::Fill),
                    checkbox(self.snap)
                        .label("Enable Snap")
                        .on_toggle(Message::UpdateSnap),
                ]
                .spacing(10)
            } else {
                column![]
            },
        ]
        .spacing(15)
        .padding(15);

        let saved_styles_list: Element<_> = {
            if let Some(style_map) = self.styles.get(&self.selected_view) {
                if !style_map.is_empty() {}
            }

            let styles_for_view = self.styles.get(&self.selected_view);

            if styles_for_view.is_none() {
                container(text("No styles saved for this widget yet.").center()).into()
            } else {
                let list_content = column(
                    styles_for_view
                        .unwrap()
                        .iter()
                        .map(|(name, definition)| {
                            let preview_style = Self::preview_swatch_style(definition, &theme);

                            button(
                                container(text(name).size(12).center())
                                    .center(Length::Fill)
                                    .width(Length::Fill)
                                    .height(Length::Fixed(30.0))
                                    .style(move |_: &Theme| preview_style),
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
                        button("Save Style")
                            .on_press(Message::SaveStyle)
                            .style(button::secondary),
                        button("Reset to Default")
                            .on_press(Message::ResetToDefault)
                            .style(button::secondary),
                    ]
                    .spacing(10),
                ]
                .spacing(10)
                .align_x(Alignment::Center),
            ),
        ]
        .spacing(15);

        let current_definition = self.current_style_definition();
        let preview_content: Element<'_, Message> = match self.selected_view {
            ThemePaneEnum::Button => {
                let definition = current_definition.clone();
                button("Button Style Preview")
                    .style(move |theme, status| definition.to_button_style(theme, status))
                    .on_press(Message::Noop)
                    .into()
            }
            ThemePaneEnum::Checkbox => {
                let definition = current_definition.clone();
                checkbox(true)
                    .label("Checkbox Style Preview")
                    .on_toggle(|_| Message::Noop)
                    .style(move |theme, status| definition.to_checkbox_style(theme, status))
                    .into()
            }
            ThemePaneEnum::TextInput => {
                let definition = current_definition.clone();
                text_input("Type to search...", &self.preview_value)
                    .on_input(Message::ComboboxSelected)
                    .style(move |theme, status| definition.to_text_input_style(theme, status))
                    .into()
            }
            ThemePaneEnum::Menu => {
                let menu_style = current_definition.to_menu_style(theme);
                let outer_style = container::Style {
                    text_color: Some(menu_style.text_color),
                    background: Some(menu_style.background),
                    border: menu_style.border,
                    shadow: menu_style.shadow,
                    snap: false,
                };
                let selected_text_color = menu_style.selected_text_color;
                let selected_background = menu_style.selected_background;
                container(
                    column![
                        container(text("Menu Item 1"))
                            .padding(8)
                            .width(Length::Fill),
                        container(text("Selected Item").color(selected_text_color))
                            .padding(8)
                            .width(Length::Fill)
                            .style(move |_| container::Style {
                                text_color: Some(selected_text_color),
                                background: Some(selected_background),
                                ..Default::default()
                            }),
                        container(text("Menu Item 3"))
                            .padding(8)
                            .width(Length::Fill),
                    ]
                    .spacing(4)
                    .width(Length::Fill),
                )
                .width(Length::Fixed(220.0))
                .style(move |_| outer_style)
                .into()
            }
            ThemePaneEnum::Picklist => {
                let definition = current_definition.clone();
                let options = vec![
                    "Option 1".to_string(),
                    "Option 2".to_string(),
                    "Option 3".to_string(),
                ];
                let selected = options
                    .iter()
                    .find(|option| **option == self.preview_value)
                    .cloned();
                pick_list(options, selected, Message::ComboboxSelected)
                    .placeholder("Choose an option...")
                    .style(move |theme, status| definition.to_pick_list_style(theme, status))
                    .into()
            }
            ThemePaneEnum::Slider => {
                let definition = current_definition.clone();
                slider(0.0..=100.0, 42.0, |_| Message::Noop)
                    .style(move |theme, status| definition.to_slider_style(theme, status))
                    .into()
            }
            ThemePaneEnum::Progressbar => {
                let definition = current_definition.clone();
                progress_bar(0.0..=100.0, 62.0)
                    .style(move |theme| definition.to_progress_bar_style(theme))
                    .into()
            }
            ThemePaneEnum::Radio => {
                let definition = current_definition.clone();
                radio("Radio Style Preview", 1, Some(1), |_| Message::Noop)
                    .style(move |theme, status| definition.to_radio_style(theme, status))
                    .into()
            }
            ThemePaneEnum::Rule => {
                let rule_style = current_definition.to_rule_style(theme);
                column![
                    text("Rule Preview").size(16),
                    rule::horizontal(2).style(move |_| rule_style),
                ]
                .spacing(10)
                .into()
            }
            ThemePaneEnum::Container => {
                let preview_style = current_definition.to_container_style(theme);
                container(
                    column![
                        text("Preview").size(16),
                        text("This is how your custom style looks!")
                            .size(14)
                            .center(),
                        text("Lorem ipsum dolor sit amet, consectetur adipiscing elit.")
                            .size(12)
                            .center(),
                        row![
                            text("Sample text").size(10),
                            Space::new().width(Length::Fill).height(Length::Fixed(1.0)),
                            text("More text").size(10),
                        ]
                    ]
                    .spacing(5)
                    .padding(15),
                )
                .center_x(Length::Fill)
                .style(move |_| preview_style)
                .into()
            }
            ThemePaneEnum::Toggler => {
                let definition = current_definition.clone();
                toggler(true)
                    .on_toggle(|_| Message::Noop)
                    .label("Toggler Style Preview")
                    .style(move |theme, status| definition.to_toggler_style(theme, status))
                    .into()
            }
            ThemePaneEnum::Combobox => {
                let input_definition = current_definition.clone();
                let menu_definition = current_definition.clone();
                combo_box(
                    &self.combobox_state,
                    "Type to search...",
                    if self.preview_value.is_empty() {
                        None
                    } else {
                        Some(&self.preview_value)
                    },
                    Message::ComboboxSelected,
                )
                .input_style(move |theme, status| {
                    input_definition.to_combo_box_input_style(theme, status)
                })
                .menu_style(move |theme| menu_definition.to_combo_box_menu_style(theme))
                .into()
            }
            _ => {
                let preview_style = Self::preview_swatch_style(&current_definition, theme);
                container(text("Preview"))
                    .center_x(Length::Fixed(220.0))
                    .padding(15)
                    .style(move |_| preview_style)
                    .into()
            }
        };

        let code_view = {
            // Generate code based on selected widget type
            let (code_element, code_string): (Element<'_, Message>, String) =
                match self.selected_view {
                    ThemePaneEnum::Button
                    | ThemePaneEnum::Container
                    | ThemePaneEnum::Checkbox
                    | ThemePaneEnum::TextInput
                    | ThemePaneEnum::Menu
                    | ThemePaneEnum::Picklist
                    | ThemePaneEnum::Slider
                    | ThemePaneEnum::Progressbar
                    | ThemePaneEnum::Radio
                    | ThemePaneEnum::Toggler
                    | ThemePaneEnum::Combobox
                    | ThemePaneEnum::Rule => {
                        let code_string = self.style_code_content.text();
                        let settings = TsSettings {
                            text: Arc::<str>::from(code_string.as_str()),
                        };
                        let element: Element<'_, Message> = text_editor(&self.style_code_content)
                            .highlight_with::<TreeSitterIcedHighlighter>(
                                settings,
                                TreeSitterIcedHighlighter::to_format,
                            )
                            .on_action(Message::Edit)
                            .height(Length::Fill)
                            .style(code_gen_text_editor_style)
                            .font(EDITOR_FONT)
                            .size(14.0)
                            .into();
                        (element, code_string)
                    }
                    _ => {
                        let element = container(text("Not Ready")).into();
                        let code_str = "Code Generation not prepared".to_string();
                        (element, code_str)
                    }
                };

            column![
                container(text("Style Code").size(18)).center_x(Length::Fill),
                internal_overlay(
                    code_element,
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
                column![style_selection, content,]
                    .spacing(10)
                    .padding(Padding {
                        top: 0.0,
                        right: 15.0,
                        left: 0.0,
                        bottom: 0.0,
                    })
            )
            .width(Length::Fixed(420.0)),
            column![
                container(text("Live Preview").size(18)).center_x(Length::Fill),
                container(preview_content).center_x(Length::Fill),
                code_view,
            ]
            .height(Length::Fill)
            .spacing(10)
            .padding(25),
        ]
        .spacing(10)
        .padding(Padding {
            top: 10.0,
            right: 5.0,
            left: 5.0,
            bottom: 10.0,
        })
        .into()
    }

    fn current_style_definition(&self) -> SavedStyleDefinition {
        SavedStyleDefinition {
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
            rule_fill_mode: self.rule_fill_mode.clone(),
            icon_color: self.icon_color,
            icon_color_source: self.icon_color_source.clone(),
            placeholder_color: self.placeholder_color,
            placeholder_color_source: self.placeholder_color_source.clone(),
            selection_color: self.selection_color,
            selection_color_source: self.selection_color_source.clone(),
            selected_text_color: self.selected_text_color,
            selected_text_color_source: self.selected_text_color_source.clone(),
            selected_background_color: self.selected_background_color,
            selected_background_color_source: self.selected_background_color_source.clone(),
            status_hovered: self.status_hovered_override.clone(),
            status_pressed: self.status_pressed_override.clone(),
            status_disabled: self.status_disabled_override.clone(),
            status_focused: self.status_focused_override.clone(),
        }
    }

    fn preview_swatch_style(definition: &SavedStyleDefinition, theme: &Theme) -> container::Style {
        match definition.widget_type {
            ThemePaneEnum::Container => definition.to_container_style(theme),
            ThemePaneEnum::Button => {
                let button_style = definition.to_button_style(theme, button::Status::Active);
                container::Style {
                    text_color: Some(button_style.text_color),
                    background: button_style.background,
                    border: button_style.border,
                    shadow: button_style.shadow,
                    snap: button_style.snap,
                }
            }
            ThemePaneEnum::Checkbox => {
                let checkbox_style = definition
                    .to_checkbox_style(theme, checkbox::Status::Active { is_checked: true });
                container::Style {
                    text_color: checkbox_style.text_color,
                    background: Some(checkbox_style.background),
                    border: checkbox_style.border,
                    shadow: Shadow::default(),
                    snap: false,
                }
            }
            ThemePaneEnum::TextInput => {
                let style = definition.to_text_input_style(theme, text_input::Status::Active);
                container::Style {
                    text_color: Some(style.value),
                    background: Some(style.background),
                    border: style.border,
                    shadow: Shadow::default(),
                    snap: false,
                }
            }
            ThemePaneEnum::Menu => {
                let style = definition.to_menu_style(theme);
                container::Style {
                    text_color: Some(style.text_color),
                    background: Some(style.background),
                    border: style.border,
                    shadow: style.shadow,
                    snap: false,
                }
            }
            ThemePaneEnum::Picklist => {
                let style = definition.to_pick_list_style(theme, pick_list::Status::Active);
                container::Style {
                    text_color: Some(style.text_color),
                    background: Some(style.background),
                    border: style.border,
                    shadow: Shadow::default(),
                    snap: false,
                }
            }
            ThemePaneEnum::Slider => {
                let style = definition.to_slider_style(theme, slider::Status::Active);
                container::Style {
                    text_color: None,
                    background: Some(style.rail.backgrounds.0),
                    border: style.rail.border,
                    shadow: Shadow::default(),
                    snap: false,
                }
            }
            ThemePaneEnum::Progressbar => {
                let style = definition.to_progress_bar_style(theme);
                container::Style {
                    text_color: None,
                    background: Some(style.bar),
                    border: style.border,
                    shadow: Shadow::default(),
                    snap: false,
                }
            }
            ThemePaneEnum::Radio => {
                let style =
                    definition.to_radio_style(theme, radio::Status::Active { is_selected: true });
                container::Style {
                    text_color: style.text_color,
                    background: Some(style.background),
                    border: Border {
                        color: style.border_color,
                        width: style.border_width,
                        radius: iced::border::Radius::from(999.0),
                    },
                    shadow: Shadow::default(),
                    snap: false,
                }
            }
            ThemePaneEnum::Rule => {
                let rule_style = definition.to_rule_style(theme);
                container::Style {
                    text_color: None,
                    background: Some(Background::Color(rule_style.color)),
                    border: Border {
                        color: rule_style.color,
                        width: 1.0,
                        radius: rule_style.radius,
                    },
                    shadow: Shadow::default(),
                    snap: rule_style.snap,
                }
            }
            ThemePaneEnum::Toggler => {
                let style = definition
                    .to_toggler_style(theme, toggler::Status::Active { is_toggled: true });
                container::Style {
                    text_color: style.text_color,
                    background: Some(style.background),
                    border: Border {
                        color: style.background_border_color,
                        width: style.background_border_width,
                        radius: style.border_radius.unwrap_or_else(|| 0.0.into()),
                    },
                    shadow: Shadow::default(),
                    snap: false,
                }
            }
            ThemePaneEnum::Combobox => {
                let style = definition.to_combo_box_input_style(theme, text_input::Status::Active);
                container::Style {
                    text_color: Some(style.value),
                    background: Some(style.background),
                    border: style.border,
                    shadow: Shadow::default(),
                    snap: false,
                }
            }
            _ => container::Style::default(),
        }
    }

    fn _create_current_combobox_input_style(&self, theme: &Theme) -> text_input::Style {
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
        let icon_color = if let Some(ref source) = self.icon_color_source {
            evaluate_theme_expression(theme, source).unwrap_or(self.icon_color)
        } else {
            self.icon_color
        };
        let placeholder_color = if let Some(ref source) = self.placeholder_color_source {
            evaluate_theme_expression(theme, source).unwrap_or(self.placeholder_color)
        } else {
            self.placeholder_color
        };
        let text_color = if let Some(ref source) = self.text_color_source {
            evaluate_theme_expression(theme, source).unwrap_or(self.text_color)
        } else {
            self.text_color
        };
        let selection_color = if let Some(ref source) = self.selection_color_source {
            evaluate_theme_expression(theme, source).unwrap_or(self.selection_color)
        } else {
            self.selection_color
        };

        text_input::Style {
            background: Background::Color(background_color),
            border: Border {
                color: border_color,
                width: self.border_width,
                radius: iced::border::Radius {
                    top_left: self.border_radius_top_left,
                    top_right: self.border_radius_top_right,
                    bottom_right: self.border_radius_bottom_right,
                    bottom_left: self.border_radius_bottom_left,
                },
            },
            icon: icon_color,
            placeholder: placeholder_color,
            value: text_color,
            selection: selection_color,
        }
    }

    fn reset_style_editor(&mut self, view: ThemePaneEnum) {
        let palette = self.theme.extended_palette();
        self.style_name = String::new();
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
        self.rule_fill_mode = RuleFillMode::Full;
        self.icon_color = palette.primary.base.text;
        self.icon_color_source = None;
        self.placeholder_color = palette.background.weak.text;
        self.placeholder_color_source = None;
        self.selection_color = palette.primary.weak.color;
        self.selection_color_source = None;
        self.selected_text_color = palette.primary.base.text;
        self.selected_text_color_source = None;
        self.selected_background_color = palette.primary.base.color;
        self.selected_background_color_source = None;

        match view {
            ThemePaneEnum::Container => {
                self.text_color = palette.background.base.text;
                self.text_color_source = Some("palette.background.base.text".to_string());
                self.background_color = palette.background.base.color;
                self.background_color_source = Some("palette.background.base.color".to_string());
                self.border_color = palette.background.strong.color;
                self.border_color_source = Some("palette.background.strong.color".to_string());
                self.border_width = 0.0;
                self.border_radius_top_left = 0.0;
                self.border_radius_top_right = 0.0;
                self.border_radius_bottom_right = 0.0;
                self.border_radius_bottom_left = 0.0;
                self.shadow_enabled = false;
                self.shadow_color = palette.background.weak.color;
                self.shadow_color_source = Some("palette.background.weak.color".to_string());
                self.shadow_offset_x = 0.0;
                self.shadow_offset_y = 0.0;
                self.shadow_blur_radius = 0.0;
                self.snap = true;
            }
            ThemePaneEnum::Button => {
                // Using primary button colors as a default
                self.text_color = palette.primary.base.text;
                self.text_color_source = Some("palette.primary.base.text".to_string());
                self.background_color = palette.primary.base.color;
                self.background_color_source = Some("palette.primary.base.color".to_string());
                self.border_color = palette.primary.strong.color;
                self.border_color_source = Some("palette.primary.strong.color".to_string());
                self.border_width = 1.0;
                // Buttons in iced typically have a single radius value
                let radius = 4.0;
                self.border_radius_top_left = radius;
                self.border_radius_top_right = radius;
                self.border_radius_bottom_right = radius;
                self.border_radius_bottom_left = radius;
                self.shadow_enabled = true;
                self.shadow_color = palette.background.weak.color;
                self.shadow_color_source = Some("palette.background.weak.color".to_string());
                self.shadow_offset_x = 0.0;
                self.shadow_offset_y = 2.0; // A subtle shadow is a nice default
                self.shadow_blur_radius = 4.0;
                self.snap = true;
            }
            ThemePaneEnum::Checkbox => {
                self.text_color = palette.background.base.text;
                self.text_color_source = Some("palette.background.base.text".to_string());
                self.background_color = palette.background.base.color;
                self.background_color_source = Some("palette.background.base.color".to_string());
                self.icon_color = palette.primary.base.text;
                self.icon_color_source = Some("palette.primary.base.text".to_string());
                self.border_color = palette.background.strong.color;
                self.border_color_source = Some("palette.background.strong.color".to_string());
                self.border_width = 1.0;
                let radius = 2.0;
                self.border_radius_top_left = radius;
                self.border_radius_top_right = radius;
                self.border_radius_bottom_right = radius;
                self.border_radius_bottom_left = radius;
                self.shadow_enabled = false;
                self.shadow_color = palette.background.weak.color;
                self.shadow_color_source = Some("palette.background.weak.color".to_string());
                self.shadow_offset_x = 0.0;
                self.shadow_offset_y = 0.0;
                self.shadow_blur_radius = 0.0;
                self.snap = true;
            }
            ThemePaneEnum::TextInput => {
                self.text_color = palette.background.base.text;
                self.text_color_source = Some("palette.background.base.text".to_string());
                self.background_color = palette.background.base.color;
                self.background_color_source = Some("palette.background.base.color".to_string());
                self.icon_color = palette.background.weak.text;
                self.icon_color_source = Some("palette.background.weak.text".to_string());
                self.placeholder_color = palette.background.weak.text;
                self.placeholder_color_source = Some("palette.background.weak.text".to_string());
                self.selection_color = palette.primary.weak.color;
                self.selection_color_source = Some("palette.primary.weak.color".to_string());
                self.border_color = palette.background.strong.color;
                self.border_color_source = Some("palette.background.strong.color".to_string());
                self.border_width = 1.0;
                let radius = 4.0;
                self.border_radius_top_left = radius;
                self.border_radius_top_right = radius;
                self.border_radius_bottom_right = radius;
                self.border_radius_bottom_left = radius;
                self.snap = false;
            }
            ThemePaneEnum::Menu => {
                self.text_color = palette.background.base.text;
                self.text_color_source = Some("palette.background.base.text".to_string());
                self.background_color = palette.background.base.color;
                self.background_color_source = Some("palette.background.base.color".to_string());
                self.border_color = palette.background.strong.color;
                self.border_color_source = Some("palette.background.strong.color".to_string());
                self.border_width = 1.0;
                let radius = 4.0;
                self.border_radius_top_left = radius;
                self.border_radius_top_right = radius;
                self.border_radius_bottom_right = radius;
                self.border_radius_bottom_left = radius;
                self.selected_text_color = palette.primary.base.text;
                self.selected_text_color_source = Some("palette.primary.base.text".to_string());
                self.selected_background_color = palette.primary.base.color;
                self.selected_background_color_source =
                    Some("palette.primary.base.color".to_string());
                self.shadow_enabled = true;
                self.shadow_color = palette.background.strong.color;
                self.shadow_color_source = Some("palette.background.strong.color".to_string());
                self.shadow_offset_x = 0.0;
                self.shadow_offset_y = 2.0;
                self.shadow_blur_radius = 8.0;
                self.snap = false;
            }
            ThemePaneEnum::Picklist => {
                self.text_color = palette.background.base.text;
                self.text_color_source = Some("palette.background.base.text".to_string());
                self.background_color = palette.background.base.color;
                self.background_color_source = Some("palette.background.base.color".to_string());
                self.icon_color = palette.background.weak.text;
                self.icon_color_source = Some("palette.background.weak.text".to_string());
                self.placeholder_color = palette.background.weak.text;
                self.placeholder_color_source = Some("palette.background.weak.text".to_string());
                self.border_color = palette.background.strong.color;
                self.border_color_source = Some("palette.background.strong.color".to_string());
                self.border_width = 1.0;
                let radius = 4.0;
                self.border_radius_top_left = radius;
                self.border_radius_top_right = radius;
                self.border_radius_bottom_right = radius;
                self.border_radius_bottom_left = radius;
                self.snap = false;
            }
            ThemePaneEnum::Slider => {
                self.text_color = palette.primary.base.color;
                self.text_color_source = Some("palette.primary.base.color".to_string());
                self.background_color = palette.background.weak.color;
                self.background_color_source = Some("palette.background.weak.color".to_string());
                self.icon_color = palette.primary.base.color;
                self.icon_color_source = Some("palette.primary.base.color".to_string());
                self.border_color = palette.background.strong.color;
                self.border_color_source = Some("palette.background.strong.color".to_string());
                self.border_width = 2.0;
                let radius = 8.0;
                self.border_radius_top_left = radius;
                self.border_radius_top_right = radius;
                self.border_radius_bottom_right = radius;
                self.border_radius_bottom_left = radius;
                self.snap = false;
            }
            ThemePaneEnum::Progressbar => {
                self.text_color = palette.primary.base.color;
                self.text_color_source = Some("palette.primary.base.color".to_string());
                self.background_color = palette.background.weak.color;
                self.background_color_source = Some("palette.background.weak.color".to_string());
                self.border_color = palette.background.strong.color;
                self.border_color_source = Some("palette.background.strong.color".to_string());
                self.border_width = 1.0;
                let radius = 6.0;
                self.border_radius_top_left = radius;
                self.border_radius_top_right = radius;
                self.border_radius_bottom_right = radius;
                self.border_radius_bottom_left = radius;
                self.snap = false;
            }
            ThemePaneEnum::Radio => {
                self.text_color = palette.background.base.text;
                self.text_color_source = Some("palette.background.base.text".to_string());
                self.background_color = palette.background.base.color;
                self.background_color_source = Some("palette.background.base.color".to_string());
                self.icon_color = palette.primary.base.color;
                self.icon_color_source = Some("palette.primary.base.color".to_string());
                self.border_color = palette.background.strong.color;
                self.border_color_source = Some("palette.background.strong.color".to_string());
                self.border_width = 1.0;
                let radius = 12.0;
                self.border_radius_top_left = radius;
                self.border_radius_top_right = radius;
                self.border_radius_bottom_right = radius;
                self.border_radius_bottom_left = radius;
                self.snap = false;
            }
            ThemePaneEnum::Toggler => {
                self.text_color = palette.background.base.text;
                self.text_color_source = Some("palette.background.base.text".to_string());
                self.background_color = palette.background.weak.color;
                self.background_color_source = Some("palette.background.weak.color".to_string());
                self.icon_color = palette.primary.base.color;
                self.icon_color_source = Some("palette.primary.base.color".to_string());
                self.border_color = palette.background.strong.color;
                self.border_color_source = Some("palette.background.strong.color".to_string());
                self.border_width = 1.0;
                let radius = 12.0;
                self.border_radius_top_left = radius;
                self.border_radius_top_right = radius;
                self.border_radius_bottom_right = radius;
                self.border_radius_bottom_left = radius;
                self.snap = false;
            }
            ThemePaneEnum::Combobox => {
                // text_input value color
                self.text_color = palette.background.base.text;
                self.text_color_source = Some("palette.background.base.text".to_string());
                self.background_color = palette.background.base.color;
                self.background_color_source = Some("palette.background.base.color".to_string());
                self.icon_color = palette.background.weak.text;
                self.icon_color_source = Some("palette.background.weak.text".to_string());
                self.placeholder_color = palette.background.weak.text;
                self.placeholder_color_source = Some("palette.background.weak.text".to_string());
                self.selection_color = palette.primary.weak.color;
                self.selection_color_source = Some("palette.primary.weak.color".to_string());
                self.border_color = palette.background.strong.color;
                self.border_color_source = Some("palette.background.strong.color".to_string());
                self.border_width = 1.0;
                let radius = 4.0;
                self.border_radius_top_left = radius;
                self.border_radius_top_right = radius;
                self.border_radius_bottom_right = radius;
                self.border_radius_bottom_left = radius;
                // menu style defaults
                self.selected_text_color = palette.primary.base.text;
                self.selected_text_color_source = Some("palette.primary.base.text".to_string());
                self.selected_background_color = palette.primary.base.color;
                self.selected_background_color_source =
                    Some("palette.primary.base.color".to_string());
                self.shadow_enabled = true;
                self.shadow_color = palette.background.strong.color;
                self.shadow_color_source = Some("palette.background.strong.color".to_string());
                self.shadow_offset_x = 0.0;
                self.shadow_offset_y = 2.0;
                self.shadow_blur_radius = 8.0;
                self.snap = false;
            }
            ThemePaneEnum::Rule => {
                self.border_color = palette.background.strong.color;
                self.border_color_source = Some("palette.background.strong.color".to_string());
                self.border_width = 1.0;
                self.border_radius_top_left = 0.0;
                self.border_radius_top_right = 0.0;
                self.border_radius_bottom_right = 0.0;
                self.border_radius_bottom_left = 0.0;
                self.rule_fill_mode = RuleFillMode::Full;
                self.snap = true;
            }
            _ => {}
        }
        self.editing_status = EditingStatus::Active;
        self.status_hovered_override = None;
        self.status_pressed_override = None;
        self.status_disabled_override = None;
        self.status_focused_override = None;
    }

    /// View to see all colors of a theme
    pub fn show_theme_colors<'a>(&'a self, theme: &'a Theme) -> Element<'a, Message> {
        let palette = theme.extended_palette();
        let base = theme.palette();

        let base_palette = container(
            column![
                container(
                    text("Palette")
                        .size(24)
                        .color(palette.background.strong.text)
                )
                .center(Length::Fill),
                row![
                    container(text("Background").center(),)
                        .style(move |_: &Theme| container::Style {
                            text_color: Some(base.text),
                            background: Some(Background::Color(base.background)),
                            border: Border {
                                color: palette.background.strong.color,
                                width: 1.0,
                                radius: 0.0.into()
                            },
                            ..Default::default()
                        })
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center)
                        .width(Length::FillPortion(1))
                        .height(Length::Fixed(50.0)),
                    container(text("Text").center(),)
                        .style(move |_: &Theme| container::Style {
                            text_color: Some(base.background),
                            background: Some(Background::Color(base.text)),
                            ..Default::default()
                        })
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center)
                        .width(Length::FillPortion(1))
                        .height(Length::Fixed(50.0)),
                ]
                .spacing(10),
                row![
                    container(text("Primary").center(),)
                        .style(move |_: &Theme| container::Style {
                            text_color: Some(base.background),
                            background: Some(Background::Color(base.primary)),
                            ..Default::default()
                        })
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center)
                        .width(Length::FillPortion(1))
                        .height(Length::Fixed(50.0)),
                    container(text("Success").center(),)
                        .style(move |_: &Theme| container::Style {
                            text_color: Some(base.background),
                            background: Some(Background::Color(base.success)),
                            ..Default::default()
                        })
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center)
                        .width(Length::FillPortion(1))
                        .height(Length::Fixed(50.0)),
                ]
                .spacing(10),
                row![
                    container(text("Warning").center(),)
                        .style(move |_: &Theme| container::Style {
                            text_color: Some(base.background),
                            background: Some(Background::Color(base.warning)),
                            ..Default::default()
                        })
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center)
                        .width(Length::FillPortion(1))
                        .height(Length::Fixed(50.0)),
                    container(text("Danger").center(),)
                        .style(move |_: &Theme| container::Style {
                            text_color: Some(base.background),
                            background: Some(Background::Color(base.danger)),
                            ..Default::default()
                        })
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
            .height(Length::Shrink),
        );

        let palette_showcase = scrollable(
            column![
                base_palette,
                container(
                    text("Extended Palette")
                        .size(24)
                        .color(palette.background.strong.text)
                )
                .center(Length::Fill),
                container(
                    text("Background")
                        .size(16)
                        .color(palette.background.base.text)
                )
                .center(Length::Fill),
                column![
                    row![
                        container(text("Base").center(),)
                            .style(|_| container::Style {
                                text_color: Some(palette.background.base.text),
                                background: Some(Background::Color(palette.background.base.color)),
                                border: Border {
                                    color: palette.background.strong.color,
                                    width: 1.0,
                                    radius: 0.0.into()
                                },
                                ..Default::default()
                            })
                            .align_x(Alignment::Center)
                            .align_y(Alignment::Center)
                            .width(Length::FillPortion(1))
                            .height(Length::Fixed(50.0)),
                        container(text("Neutral").center(),)
                            .style(|_| container::Style {
                                text_color: Some(palette.background.neutral.text),
                                background: Some(Background::Color(
                                    palette.background.neutral.color
                                )),
                                ..Default::default()
                            })
                            .align_x(Alignment::Center)
                            .align_y(Alignment::Center)
                            .width(Length::FillPortion(1))
                            .height(Length::Fixed(50.0)),
                    ]
                    .spacing(10),
                    row![
                        container(text("Weak").center(),)
                            .style(|_| container::Style {
                                text_color: Some(palette.background.weak.text),
                                background: Some(Background::Color(palette.background.weak.color)),
                                ..Default::default()
                            })
                            .align_x(Alignment::Center)
                            .align_y(Alignment::Center)
                            .width(Length::FillPortion(1))
                            .height(Length::Fixed(50.0)),
                        container(text("Weaker").center(),)
                            .style(|_| container::Style {
                                text_color: Some(palette.background.weaker.text),
                                background: Some(Background::Color(
                                    palette.background.weaker.color
                                )),
                                ..Default::default()
                            })
                            .align_x(Alignment::Center)
                            .align_y(Alignment::Center)
                            .width(Length::FillPortion(1))
                            .height(Length::Fixed(50.0)),
                        container(text("Weakest").center(),)
                            .style(|_| container::Style {
                                text_color: Some(palette.background.weakest.text),
                                background: Some(Background::Color(
                                    palette.background.weakest.color
                                )),
                                ..Default::default()
                            })
                            .align_x(Alignment::Center)
                            .align_y(Alignment::Center)
                            .width(Length::FillPortion(1))
                            .height(Length::Fixed(50.0)),
                    ]
                    .spacing(10),
                    row![
                        container(text("Strong").center(),)
                            .style(|_| container::Style {
                                text_color: Some(palette.background.strong.text),
                                background: Some(Background::Color(
                                    palette.background.strong.color
                                )),
                                ..Default::default()
                            })
                            .align_x(Alignment::Center)
                            .align_y(Alignment::Center)
                            .width(Length::FillPortion(1))
                            .height(Length::Fixed(50.0)),
                        container(text("Stronger").center(),)
                            .style(|_| container::Style {
                                text_color: Some(palette.background.stronger.text),
                                background: Some(Background::Color(
                                    palette.background.stronger.color
                                )),
                                ..Default::default()
                            })
                            .align_x(Alignment::Center)
                            .align_y(Alignment::Center)
                            .width(Length::FillPortion(1))
                            .height(Length::Fixed(50.0)),
                        container(text("Strongest").center(),)
                            .style(|_| container::Style {
                                text_color: Some(palette.background.strongest.text),
                                background: Some(Background::Color(
                                    palette.background.strongest.color
                                )),
                                ..Default::default()
                            })
                            .align_x(Alignment::Center)
                            .align_y(Alignment::Center)
                            .width(Length::FillPortion(1))
                            .height(Length::Fixed(50.0)),
                    ]
                    .spacing(10),
                ]
                .spacing(10),
                container(text("Primary").size(16).color(palette.background.base.text))
                    .center(Length::Fill),
                row![
                    container(text("Base").center(),)
                        .style(|_| container::Style {
                            text_color: Some(palette.primary.base.text),
                            background: Some(Background::Color(palette.primary.base.color)),
                            ..Default::default()
                        })
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center)
                        .width(Length::FillPortion(1))
                        .height(Length::Fixed(50.0)),
                    container(text("Weak").center(),)
                        .style(|_| container::Style {
                            text_color: Some(palette.primary.weak.text),
                            background: Some(Background::Color(palette.primary.weak.color)),
                            ..Default::default()
                        })
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center)
                        .width(Length::FillPortion(1))
                        .height(Length::Fixed(50.0)),
                    container(text("Strong").center(),)
                        .style(|_| container::Style {
                            text_color: Some(palette.primary.strong.text),
                            background: Some(Background::Color(palette.primary.strong.color)),
                            ..Default::default()
                        })
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center)
                        .width(Length::FillPortion(1))
                        .height(Length::Fixed(50.0)),
                ]
                .spacing(10),
                container(
                    text("Secondary")
                        .size(16)
                        .color(palette.background.base.text)
                )
                .center(Length::Fill),
                row![
                    container(text("Base").center(),)
                        .style(|_| container::Style {
                            text_color: Some(palette.secondary.base.text),
                            background: Some(Background::Color(palette.secondary.base.color)),
                            ..Default::default()
                        })
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center)
                        .width(Length::FillPortion(1))
                        .height(Length::Fixed(50.0)),
                    container(text("Weak").center(),)
                        .style(|_| container::Style {
                            text_color: Some(palette.secondary.weak.text),
                            background: Some(Background::Color(palette.secondary.weak.color)),
                            ..Default::default()
                        })
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center)
                        .width(Length::FillPortion(1))
                        .height(Length::Fixed(50.0)),
                    container(text("Strong").center(),)
                        .style(|_| container::Style {
                            text_color: Some(palette.secondary.strong.text),
                            background: Some(Background::Color(palette.secondary.strong.color)),
                            ..Default::default()
                        })
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center)
                        .width(Length::FillPortion(1))
                        .height(Length::Fixed(50.0)),
                ]
                .spacing(10),
                container(text("Success").size(16).color(palette.background.base.text))
                    .center(Length::Fill),
                row![
                    container(text("Base").center(),)
                        .style(|_| container::Style {
                            text_color: Some(palette.success.base.text),
                            background: Some(Background::Color(palette.success.base.color)),
                            ..Default::default()
                        })
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center)
                        .width(Length::FillPortion(1))
                        .height(Length::Fixed(50.0)),
                    container(text("Weak").center(),)
                        .style(|_| container::Style {
                            text_color: Some(palette.success.weak.text),
                            background: Some(Background::Color(palette.success.weak.color)),
                            ..Default::default()
                        })
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center)
                        .width(Length::FillPortion(1))
                        .height(Length::Fixed(50.0)),
                    container(text("Strong").center(),)
                        .style(|_| container::Style {
                            text_color: Some(palette.success.strong.text),
                            background: Some(Background::Color(palette.success.strong.color)),
                            ..Default::default()
                        })
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center)
                        .width(Length::FillPortion(1))
                        .height(Length::Fixed(50.0)),
                ]
                .spacing(10),
                container(text("Warning").size(16).color(palette.background.base.text))
                    .center(Length::Fill),
                row![
                    container(text("Base").center(),)
                        .style(|_| container::Style {
                            text_color: Some(palette.warning.base.text),
                            background: Some(Background::Color(palette.warning.base.color)),
                            ..Default::default()
                        })
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center)
                        .width(Length::FillPortion(1))
                        .height(Length::Fixed(50.0)),
                    container(text("Weak").center(),)
                        .style(|_| container::Style {
                            text_color: Some(palette.warning.weak.text),
                            background: Some(Background::Color(palette.warning.weak.color)),
                            ..Default::default()
                        })
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center)
                        .width(Length::FillPortion(1))
                        .height(Length::Fixed(50.0)),
                    container(text("Strong").center(),)
                        .style(|_| container::Style {
                            text_color: Some(palette.warning.strong.text),
                            background: Some(Background::Color(palette.warning.strong.color)),
                            ..Default::default()
                        })
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center)
                        .width(Length::FillPortion(1))
                        .height(Length::Fixed(50.0)),
                ]
                .spacing(10),
                container(text("Danger").size(16).color(palette.background.base.text))
                    .center(Length::Fill),
                row![
                    container(text("Base").center(),)
                        .style(|_| container::Style {
                            text_color: Some(palette.danger.base.text),
                            background: Some(Background::Color(palette.danger.base.color)),
                            ..Default::default()
                        })
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center)
                        .width(Length::FillPortion(1))
                        .height(Length::Fixed(50.0)),
                    container(text("Weak").center(),)
                        .style(|_| container::Style {
                            text_color: Some(palette.danger.weak.text),
                            background: Some(Background::Color(palette.danger.weak.color)),
                            ..Default::default()
                        })
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center)
                        .width(Length::FillPortion(1))
                        .height(Length::Fixed(50.0)),
                    container(text("Strong").center(),)
                        .style(|_| container::Style {
                            text_color: Some(palette.danger.strong.text),
                            background: Some(Background::Color(palette.danger.strong.color)),
                            ..Default::default()
                        })
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
            .height(Length::Shrink),
        );

        column![palette_showcase.style(|theme: &Theme, status| {
            let palette = theme.extended_palette();

            // theme+status-aware default
            let mut s = scrollable::default(theme, status);

            // update values and return
            s.container.background = Some(Background::Color(palette.background.base.color));
            s.container.border = Border {
                color: palette.background.strong.color,
                width: 1.0,
                radius: 5.0.into(),
            };
            s
        })]
        .padding(Padding {
            top: 10.0,
            right: 5.0,
            left: 5.0,
            bottom: 10.0,
        })
        .width(Length::Fill)
        .into()
    }

    pub fn styles(&self) -> &BTreeMap<ThemePaneEnum, BTreeMap<String, SavedStyleDefinition>> {
        &self.styles
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ChangeView(ThemePaneEnum),
    Edit(text_editor::Action),
    CopyCode(String),

    // Generic Style properties
    UpdateTextColor {
        color: Color,
        source: Option<String>,
    },
    UpdateBorderColor {
        color: Color,
        source: Option<String>,
    },
    UpdateBorderWidth(f32),
    UpdateBorderRadiusTopLeft(f32),
    UpdateBorderRadiusTopRight(f32),
    UpdateBorderRadiusBottomRight(f32),
    UpdateBorderRadiusBottomLeft(f32),
    UpdateBackgroundColor {
        color: Color,
        source: Option<String>,
    },
    UpdateShadowEnabled(bool),
    UpdateShadowColor {
        color: Color,
        source: Option<String>,
    },
    UpdateShadowOffsetX(f32),
    UpdateShadowOffsetY(f32),
    UpdateShadowBlurRadius(f32),
    UpdateSnap(bool),
    UpdateRuleFillMode(RuleFillMode),
    UpdateIconColor {
        color: Color,
        source: Option<String>,
    },
    UpdatePlaceholderColor {
        color: Color,
        source: Option<String>,
    },
    UpdateSelectionColor {
        color: Color,
        source: Option<String>,
    },
    UpdateSelectedTextColor {
        color: Color,
        source: Option<String>,
    },
    UpdateSelectedBackgroundColor {
        color: Color,
        source: Option<String>,
    },

    // Style management
    UpdateStyleName(String),
    SaveStyle,
    SelectStyle(String),
    ResetToDefault,

    // Per-status style editing
    SetEditingStatus(EditingStatus),
    ResetStatusOverride(EditingStatus),
    UpdateStatusColor {
        status: EditingStatus,
        field: StatusColorField,
        color: Color,
        source: Option<String>,
    },

    // Messages to handle widget previews
    Noop,
    ComboboxSelected(String),
}

//const EDITOR_FONT: iced::Font = iced::Font::with_name("Consolas").weight(iced::font::Weight::Medium);
pub const EDITOR_FONT: iced::Font = iced::Font {
    family: iced::font::Family::Name("Consolas"),
    weight: iced::font::Weight::Medium,
    stretch: iced::font::Stretch::SemiExpanded,
    style: iced::font::Style::Normal,
};

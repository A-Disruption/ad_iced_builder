use super::builder::{CodeBuilder, to_pascal_case, to_snake_case, handle_whitespace};
use crate::data_structures::types::types::{AppView, Widget, WidgetType, WidgetId};
use crate::data_structures::types::type_implementations::{
    ContainerAlignX, ContainerAlignY, FontType, PaddingMode,
};
use crate::enum_builder::TypeSystem;
use iced::advanced::text;
use iced::Alignment;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use uuid::Uuid;

/// Metadata about a ViewReference widget resolved against the views map.
#[derive(Debug, Clone)]
pub struct ViewRefInfo {
    pub widget_id: WidgetId,
    /// snake_case field name used in the App struct (from widget_name or view name)
    pub field_name: String,
    /// snake_case module name (same as generated file stem)
    pub module_name: String,
    /// PascalCase struct name
    pub struct_name: String,
}

/// Collect all ViewReference widgets from a widget tree, resolving each against the views map.
pub fn collect_view_refs(root: &Widget, views: &BTreeMap<Uuid, AppView>) -> Vec<ViewRefInfo> {
    let mut refs = Vec::new();
    collect_view_refs_recursive(root, views, &mut refs);
    refs
}

fn collect_view_refs_recursive(widget: &Widget, views: &BTreeMap<Uuid, AppView>, refs: &mut Vec<ViewRefInfo>) {
    if widget.widget_type == WidgetType::ViewReference {
        if let Some(view_id) = widget.properties.referenced_view_id {
            if let Some(view) = views.get(&view_id) {
                let field_name = if !widget.properties.widget_name.trim().is_empty() {
                    to_snake_case(&widget.properties.widget_name)
                } else {
                    to_snake_case(&view.name)
                };
                refs.push(ViewRefInfo {
                    widget_id: widget.id,
                    field_name,
                    module_name: to_snake_case(&view.name),
                    struct_name: handle_whitespace(&to_pascal_case(&view.name)),
                });
            }
        }
    }
    for child in &widget.children {
        collect_view_refs_recursive(child, views, refs);
    }
}

pub struct ImportTracker {
    pub used_widgets: HashSet<&'static str>,

    pub uses_length: bool,
    pub uses_alignment: bool,
    pub uses_padding: bool,
    pub uses_color: bool,

    pub uses_text_line_height: bool,
    pub uses_text_wrapping: bool,
    pub uses_text_shaping: bool,
    pub uses_text_alignment: bool,

    pub uses_mouse: bool,
    pub uses_mouse_interaction: bool,
    pub uses_mouse_scroll_delta: bool,

    pub uses_point: bool,
    pub uses_font: bool,
    pub uses_font_module: bool,
    pub uses_border: bool,
    pub uses_shadow: bool,
    pub uses_background: bool,
    pub uses_vector: bool,
    /// True if any icon widget, or text_input/combo_box with icon enabled, is present.
    pub uses_icon: bool,
}

impl ImportTracker {
    pub fn new() -> Self {
        Self {
            used_widgets: HashSet::new(),
            uses_length: false,
            uses_alignment: false,
            uses_padding: false,
            uses_color: false,
            uses_text_line_height: false,
            uses_text_wrapping: false,
            uses_text_shaping: false,
            uses_text_alignment: false,
            uses_mouse: false,
            uses_mouse_interaction: false,
            uses_mouse_scroll_delta: false,
            uses_point: false,
            uses_font: false,
            uses_font_module: false,
            uses_border: false,
            uses_shadow: false,
            uses_background: false,
            uses_vector: false,
            uses_icon: false,
        }
    }

    pub fn scan_widget(&mut self, widget: &Widget) {
        let props = &widget.properties;

        match widget.widget_type {
            WidgetType::Container => {
                self.used_widgets.insert("container");
                if widget.children.is_empty() { self.used_widgets.insert("text"); }
            }
            WidgetType::Row => {
                self.used_widgets.insert("row");
                if widget.children.is_empty() { self.used_widgets.insert("text"); }
            }
            WidgetType::Column => {
                self.used_widgets.insert("column");
                if widget.children.is_empty() { self.used_widgets.insert("text"); }
            }
            WidgetType::Button => {
                self.used_widgets.insert("button");
                // button always generates text(...) for content, either as placeholder or through child scan
                self.used_widgets.insert("text");
            }
            WidgetType::Text => { self.used_widgets.insert("text"); }
            WidgetType::TextInput => { self.used_widgets.insert("text_input"); }
            WidgetType::Checkbox => { self.used_widgets.insert("checkbox"); }
            WidgetType::Radio => { self.used_widgets.insert("radio"); }
            WidgetType::Slider => { self.used_widgets.insert("slider"); }
            WidgetType::VerticalSlider => { self.used_widgets.insert("vertical_slider"); }
            WidgetType::ProgressBar => { self.used_widgets.insert("progress_bar"); }
            WidgetType::Toggler => { self.used_widgets.insert("toggler"); }
            WidgetType::PickList => { self.used_widgets.insert("pick_list"); }
            WidgetType::Scrollable => {
                self.used_widgets.insert("scrollable");
                if widget.children.is_empty() { self.used_widgets.insert("text"); }
            }
            WidgetType::Space => { self.used_widgets.insert("space"); }
            WidgetType::Rule => { self.used_widgets.insert("rule"); }
            WidgetType::Image => { self.used_widgets.insert("image"); }
            WidgetType::Svg => { self.used_widgets.insert("svg"); }
            WidgetType::Tooltip => {
                self.used_widgets.insert("tooltip");
                // tooltip always generates text(...) fallbacks for host and/or content
                self.used_widgets.insert("text");
            }
            WidgetType::ComboBox => { self.used_widgets.insert("combo_box"); }
            WidgetType::Markdown => { self.used_widgets.insert("markdown"); }
            WidgetType::MouseArea => {
                self.used_widgets.insert("mouse_area");
                self.uses_mouse = true;
            }
            WidgetType::QRCode => { self.used_widgets.insert("qr_code"); }
            WidgetType::Stack => {
                self.used_widgets.insert("stack");
                if widget.children.is_empty() { self.used_widgets.insert("text"); }
            }
            WidgetType::Themer => {
                self.used_widgets.insert("themer");
                if widget.children.is_empty() {
                    self.used_widgets.insert("container");
                    self.used_widgets.insert("text");
                }
            }
            WidgetType::Grid => {
                self.used_widgets.insert("grid");
                if widget.children.is_empty() { self.used_widgets.insert("text"); }
            }
            WidgetType::Pin => {
                self.used_widgets.insert("pin");
                if widget.children.is_empty() { self.used_widgets.insert("text"); }
            }
            WidgetType::Table => {
                self.used_widgets.insert("table");
                self.used_widgets.insert("text"); // table columns always use text
                if widget.properties.table_bold_headers {
                    self.uses_font = true;
                    self.uses_font_module = true;
                }
            }
            WidgetType::Icon => {
                // Icons use icon::name() — no text/Font imports needed at call site
                self.uses_icon = true;
            }
            WidgetType::ViewReference => {}
        }

        self.uses_length = true;

        if matches!(widget.widget_type, WidgetType::Row | WidgetType::Column) {
            if props.align_items != Alignment::Start {
                self.uses_alignment = true;
            }
        }

        if props.padding_mode == PaddingMode::Individual {
            self.uses_padding = true;
        }

        if widget.widget_type == WidgetType::Container {
            if props.border_width > 0.0 {
                self.uses_border = true;
            }
            if props.background_color.a > 0.0 {
                self.uses_background = true;
                self.uses_color = true;
            }
            if props.has_shadow {
                self.uses_shadow = true;
                self.uses_vector = true;
            }
            if props.align_x != ContainerAlignX::Left || props.align_y != ContainerAlignY::Top {
                self.uses_alignment = true;
            }
        }

        if widget.widget_type == WidgetType::Text {
            if props.text_color.a > 0.0 {
                self.uses_color = true;
            }
            if props.font != FontType::Default {
                self.uses_font = true;
            }
            if props.line_height != text::LineHeight::default() {
                self.uses_text_line_height = true;
            }
            if props.wrap != text::Wrapping::default() {
                self.uses_text_wrapping = true;
            }
            if props.shaping != text::Shaping::default() {
                self.uses_text_shaping = true;
            }
            if props.text_align_x != text::Alignment::default()
                || props.text_align_y != iced::alignment::Vertical::Top
            {
                self.uses_text_alignment = true;
                self.uses_alignment = true;
            }
        }

        if widget.widget_type == WidgetType::TextInput {
            if props.text_input_font != FontType::Default {
                self.uses_font = true;
            }
            if props.text_input_line_height != text::LineHeight::default() {
                self.uses_text_line_height = true;
            }
            if props.text_input_alignment != ContainerAlignX::Left {
                self.uses_alignment = true;
            }
            if props.text_input_icon_enabled {
                self.uses_font = true;
                self.uses_icon = true;
            }
        }

        if widget.widget_type == WidgetType::ComboBox {
            if props.combobox_icon_enabled {
                self.uses_font = true;
                self.uses_icon = true;
                self.used_widgets.insert("text_input"); // combo_box icon uses text_input::Icon/Side
            }
        }

        if widget.widget_type == WidgetType::Pin {
            if props.pin_point != iced::Point::ORIGIN {
                self.uses_point = true;
            }
        }

        if widget.widget_type == WidgetType::MouseArea {
            if props.mousearea_on_scroll {
                self.uses_mouse_scroll_delta = true;
            }
            if props.mousearea_on_move {
                self.uses_point = true;
            }
            if props.mousearea_interaction.is_some() {
                self.uses_mouse_interaction = true;
            }
        }

        for child in &widget.children {
            self.scan_widget(child);
        }
    }
}

pub fn generate_imports(b: &mut CodeBuilder, root: &Widget) -> ImportTracker {
    let mut tracker = ImportTracker::new();
    tracker.scan_widget(root);

    b.push("use iced::{");
    b.newline();
    b.increase_indent();

    let mut core_imports = Vec::new();
    if tracker.uses_length { core_imports.push("Length"); }
    if tracker.uses_alignment { core_imports.push("Alignment"); }
    if tracker.uses_color { core_imports.push("Color"); }
    if tracker.uses_padding { core_imports.push("Padding"); }
    if tracker.uses_font { core_imports.push("Font"); }
    if tracker.uses_font_module { core_imports.push("font"); }
    if tracker.uses_border { core_imports.push("Border"); }
    if tracker.uses_shadow { core_imports.push("Shadow"); }
    if tracker.uses_background { core_imports.push("Background"); }
    if tracker.uses_vector { core_imports.push("Vector"); }
    if tracker.uses_point { core_imports.push("Point"); }
    core_imports.push("Element");
    core_imports.push("Theme");
    core_imports.push("Task");

    b.indent();
    b.push(&core_imports.join(", "));
    b.push(",");
    b.newline();

    if !tracker.used_widgets.is_empty() {
        b.indent();
        let mut widgets: Vec<_> = tracker.used_widgets.iter().copied().collect();
        widgets.sort();
        b.push(&format!("widget::{{{}}},", widgets.join(", ")));
        b.newline();
    }

    if tracker.uses_mouse {
        b.indent();
        let mut mouse_items = Vec::new();
        if tracker.uses_mouse_interaction {
            mouse_items.push("Interaction");
        }
        if tracker.uses_mouse_scroll_delta {
            mouse_items.push("ScrollDelta");
        }
        if !mouse_items.is_empty() {
            b.push(&format!("mouse::{{{}}},", mouse_items.join(", ")));
        } else {
            b.push("mouse,");
        }
        b.newline();
    }

    if tracker.uses_text_line_height
        || tracker.uses_text_wrapping
        || tracker.uses_text_shaping
        || tracker.uses_text_alignment
    {
        b.indent();
        let mut text_items = Vec::new();
        if tracker.uses_text_line_height { text_items.push("LineHeight"); }
        if tracker.uses_text_wrapping { text_items.push("Wrapping"); }
        if tracker.uses_text_shaping { text_items.push("Shaping"); }
        if tracker.uses_text_alignment { text_items.push("Alignment as TextAlignment"); }

        if !text_items.is_empty() {
            b.push(&format!("widget::text::{{{}}},", text_items.join(", ")));
        }
        b.newline();
    }

    b.decrease_indent();
    b.line("};");

    if tracker.uses_icon {
        b.line("mod icon;");
    }

    tracker
}

/// Collect all icon names used in this widget tree (icon widgets + text_input/combo_box icons).
pub fn collect_all_icon_names(root: &Widget) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    collect_icon_names_recursive(root, &mut names);
    names
}

fn collect_icon_names_recursive(widget: &Widget, names: &mut BTreeSet<String>) {
    let props = &widget.properties;
    if widget.widget_type == WidgetType::Icon {
        names.insert(props.icon_name.clone());
    }
    if widget.widget_type == WidgetType::TextInput && props.text_input_icon_enabled {
        names.insert(props.text_input_icon_name.clone());
    }
    if widget.widget_type == WidgetType::ComboBox && props.combobox_icon_enabled {
        names.insert(props.combobox_icon_name.clone());
    }
    for child in &widget.children {
        collect_icon_names_recursive(child, names);
    }
}

pub fn generate_message_enum(
    b: &mut CodeBuilder,
    root: &Widget,
    names: &HashMap<WidgetId, String>,
    type_system: &TypeSystem,
    view_refs: &[ViewRefInfo],
) {
    b.newline();
    b.line("#[derive(Debug, Clone)]");
    b.line("pub enum Message {");
    b.increase_indent();

    generate_message_variants(b, root, names, type_system);

    if !view_refs.is_empty() {
        b.line("ViewMessages(ViewMessages),");
    }
    b.line("Noop,");

    b.decrease_indent();
    b.line("}");

    // Generate the ViewMessages sub-enum if there are view references
    if !view_refs.is_empty() {
        b.newline();
        b.line("#[derive(Debug, Clone)]");
        b.line("pub enum ViewMessages {");
        b.increase_indent();
        for vr in view_refs {
            b.line(&format!("{}({}::Message),", to_pascal_case(&vr.field_name), vr.module_name));
        }
        b.decrease_indent();
        b.line("}");
    }
}

fn generate_message_variants(
    b: &mut CodeBuilder,
    widget: &Widget,
    names: &HashMap<WidgetId, String>,
    type_system: &TypeSystem,
) {
    let name = names.get(&widget.id).unwrap_or(&"widget".to_string()).clone();
    let props = &widget.properties;

    match widget.widget_type {
        WidgetType::Button => {
            if props.button_on_press_enabled
                || props.button_on_press_maybe_enabled
                || props.button_on_press_with_enabled
            {
                b.line(&format!("{}Pressed,", to_pascal_case(&name)));
            }
        }
        WidgetType::TextInput => {
            b.line(&format!("{}OnInput(String),", to_pascal_case(&name)));
            if props.text_input_on_submit {
                b.line(&format!("{}Submitted,", to_pascal_case(&name)));
            }
            if props.text_input_on_paste {
                b.line(&format!("{}Pasted(String),", to_pascal_case(&name)));
            }
        }
        WidgetType::Checkbox => {
            b.line(&format!("{}Toggled(bool),", to_pascal_case(&name)));
        }
        WidgetType::Radio => {
            b.line(&format!("{}Selected(usize),", to_pascal_case(&name)));
        }
        WidgetType::Slider | WidgetType::VerticalSlider => {
            b.line(&format!("{}Changed(f32),", to_pascal_case(&name)));
        }
        WidgetType::Toggler => {
            b.line(&format!("{}Toggled(bool),", to_pascal_case(&name)));
        }
        WidgetType::PickList => {
            b.line(&format!("{}Selected(String),", to_pascal_case(&name)));
        }
        WidgetType::ComboBox => {
            let type_name = if let Some(ref enum_id) = props.referenced_enum {
                if let Some(enum_def) = type_system.get_enum(enum_id.clone()) {
                    enum_def.name.clone()
                } else {
                    "String".to_string()
                }
            } else {
                "String".to_string()
            };

            b.line(&format!("{}Selected({}),", to_pascal_case(&name), type_name));
            if props.combobox_use_on_input {
                b.line(&format!("{}OnInput(String),", to_pascal_case(&name)));
            }
            if props.combobox_use_on_option_hovered {
                b.line(&format!(
                    "{}OnOptionHovered({}),",
                    to_pascal_case(&name),
                    type_name
                ));
            }
            if props.combobox_use_on_open {
                b.line(&format!("{}OnOpen,", to_pascal_case(&name)));
            }
            if props.combobox_use_on_close {
                b.line(&format!("{}OnClose,", to_pascal_case(&name)));
            }
        }
        WidgetType::Markdown => {
            b.line(&format!("{}LinkClicked(markdown::Uri),", to_pascal_case(&name)));
        }
        WidgetType::MouseArea => {
            if props.mousearea_on_press {
                b.line(&format!("{}Pressed,", to_pascal_case(&name)));
            }
            if props.mousearea_on_release {
                b.line(&format!("{}Released,", to_pascal_case(&name)));
            }
            if props.mousearea_on_double_click {
                b.line(&format!("{}DoubleClicked,", to_pascal_case(&name)));
            }
            if props.mousearea_on_right_press {
                b.line(&format!("{}RightPressed,", to_pascal_case(&name)));
            }
            if props.mousearea_on_right_release {
                b.line(&format!("{}RightReleased,", to_pascal_case(&name)));
            }
            if props.mousearea_on_middle_press {
                b.line(&format!("{}MiddlePressed,", to_pascal_case(&name)));
            }
            if props.mousearea_on_middle_release {
                b.line(&format!("{}MiddleReleased,", to_pascal_case(&name)));
            }
            if props.mousearea_on_scroll {
                b.line(&format!(
                    "{}Scrolled(mouse::ScrollDelta),",
                    to_pascal_case(&name)
                ));
            }
            if props.mousearea_on_enter {
                b.line(&format!("{}Entered(Point),", to_pascal_case(&name)));
            }
            if props.mousearea_on_move {
                b.line(&format!("{}Moved(Point),", to_pascal_case(&name)));
            }
            if props.mousearea_on_exit {
                b.line(&format!("{}Exited(Point),", to_pascal_case(&name)));
            }
        }
        _ => {}
    }

    for child in &widget.children {
        generate_message_variants(b, child, names, type_system);
    }
}

pub fn generate_update(
    b: &mut CodeBuilder,
    root: &Widget,
    names: &HashMap<WidgetId, String>,
    view_refs: &[ViewRefInfo],
) {
    b.line("pub fn update(&mut self, message: Message) -> Task<Message> {");
    b.increase_indent();
    b.line("match message {");
    b.increase_indent();

    generate_match_arms(b, root, names);

    if !view_refs.is_empty() {
        b.line("Message::ViewMessages(view_msg) => match view_msg {");
        b.increase_indent();
        for vr in view_refs {
            let pascal = to_pascal_case(&vr.field_name);
            b.line(&format!("ViewMessages::{}(msg) => {{", pascal));
            b.increase_indent();
            b.line(&format!(
                "return self.{}.update(msg).map(|m| Message::ViewMessages(ViewMessages::{}(m)));",
                vr.field_name, pascal
            ));
            b.decrease_indent();
            b.line("},");
        }
        b.decrease_indent();
        b.line("},");
    }

    b.line("Message::Noop => {}");

    b.decrease_indent();
    b.line("}");
    b.line("Task::none()");
    b.decrease_indent();
    b.line("}");
}

fn generate_match_arms(
    b: &mut CodeBuilder,
    widget: &Widget,
    names: &HashMap<WidgetId, String>,
) {
    let name = names
        .get(&widget.id)
        .unwrap_or(&"widget".to_string())
        .clone();
    let props = &widget.properties;

    match widget.widget_type {
        WidgetType::Button => {
            if props.button_on_press_enabled
                || props.button_on_press_maybe_enabled
                || props.button_on_press_with_enabled
            {
                b.line(&format!(
                    "Message::{}Pressed => {{",
                    to_pascal_case(&handle_whitespace(&name))
                ));
                b.increase_indent();
                b.line(&format!("// {} pressed", name));
                b.decrease_indent();
                b.line("}");
            }
        }
        WidgetType::TextInput => {
            b.line(&format!(
                "Message::{}OnInput(value) => {{",
                to_pascal_case(&name)
            ));
            b.increase_indent();
            b.line(&format!("self.{}_value = value;", to_snake_case(&name)));
            b.decrease_indent();
            b.line("}");

            if props.text_input_on_submit {
                b.line(&format!(
                    "Message::{}Submitted => {{",
                    to_pascal_case(&name)
                ));
                b.increase_indent();
                b.line("// Handle text input submission (Enter key pressed)");
                b.line(&format!(
                    "// Current value: self.{}_value",
                    to_snake_case(&name)
                ));
                b.decrease_indent();
                b.line("}");
            }

            if props.text_input_on_paste {
                b.line(&format!(
                    "Message::{}Pasted(pasted_text) => {{",
                    to_pascal_case(&name)
                ));
                b.increase_indent();
                b.line("// Handle text being pasted");
                b.line("// pasted_text contains the pasted string");
                b.line("// Note: on_input will also fire with the new combined value");
                b.line(&format!(
                    "self.{}_value = pasted_text;",
                    to_snake_case(&name)
                ));
                b.decrease_indent();
                b.line("}");
            }
        }
        WidgetType::Checkbox => {
            b.line(&format!(
                "Message::{}Toggled(checked) => {{",
                to_pascal_case(&name)
            ));
            b.increase_indent();
            b.line(&format!(
                "self.{}_checked = checked;",
                to_snake_case(&name)
            ));
            b.decrease_indent();
            b.line("}");
        }
        WidgetType::Radio => {
            b.line(&format!(
                "Message::{}Selected(index) => {{",
                to_pascal_case(&name)
            ));
            b.increase_indent();
            b.line(&format!(
                "self.{}_selected = index;",
                to_snake_case(&name)
            ));
            b.decrease_indent();
            b.line("}");
        }
        WidgetType::Slider | WidgetType::VerticalSlider => {
            b.line(&format!(
                "Message::{}Changed(value) => {{",
                to_pascal_case(&name)
            ));
            b.increase_indent();
            b.line(&format!("self.{}_value = value;", to_snake_case(&name)));
            b.decrease_indent();
            b.line("}");
        }
        WidgetType::Toggler => {
            b.line(&format!(
                "Message::{}Toggled(active) => {{",
                to_pascal_case(&name)
            ));
            b.increase_indent();
            b.line(&format!(
                "self.{}_active = active;",
                to_snake_case(&name)
            ));
            b.decrease_indent();
            b.line("}");
        }
        WidgetType::PickList => {
            b.line(&format!(
                "Message::{}Selected(value) => {{",
                to_pascal_case(&name)
            ));
            b.increase_indent();
            b.line(&format!(
                "self.{}_selected = Some(value);",
                to_snake_case(&name)
            ));
            b.decrease_indent();
            b.line("}");
        }
        WidgetType::ComboBox => {
            b.line(&format!(
                "Message::{}Selected(value) => {{",
                to_pascal_case(&name)
            ));
            b.increase_indent();
            b.line(&format!(
                "println!(\"{} selected: {{:?}}\", value);",
                name
            ));
            b.line(&format!("self.{}_value = value;", to_snake_case(&name)));
            b.decrease_indent();
            b.line("}");

            if props.combobox_use_on_input {
                b.line(&format!(
                    "Message::{}OnInput(text) => {{",
                    to_pascal_case(&name)
                ));
                b.increase_indent();
                b.line(&format!(
                    "println!(\"{} input text: {{}}\", text);",
                    name
                ));
                b.line("// You can filter options, update state, etc.");
                b.decrease_indent();
                b.line("}");
            }

            if props.combobox_use_on_option_hovered {
                b.line(&format!(
                    "Message::{}OnOptionHovered(option) => {{",
                    to_pascal_case(&name)
                ));
                b.increase_indent();
                b.line(&format!(
                    "println!(\"{} option hovered: {{:?}}\", option);",
                    name
                ));
                b.line("// Preview the hovered option, update UI, etc.");
                b.decrease_indent();
                b.line("}");
            }

            if props.combobox_use_on_open {
                b.line(&format!(
                    "Message::{}OnOpen => {{",
                    to_pascal_case(&name)
                ));
                b.increase_indent();
                b.line(&format!("println!(\"{} opened!\");", name));
                b.line("// Refresh data, log analytics, etc.");
                b.decrease_indent();
                b.line("}");
            }

            if props.combobox_use_on_close {
                b.line(&format!(
                    "Message::{}OnClose => {{",
                    to_pascal_case(&name)
                ));
                b.increase_indent();
                b.line(&format!("println!(\"{} closed!\");", name));
                b.line("// Save user choice, validate selection, etc.");
                b.decrease_indent();
                b.line("}");
            }
        }
        WidgetType::Markdown => {
            b.line(&format!(
                "Message::{}LinkClicked(url) => {{",
                to_pascal_case(&name)
            ));
            b.increase_indent();
            b.line("// Handle markdown link click");
            b.line("// url is a markdown::Uri containing the link target");
            b.decrease_indent();
            b.line("}");
        }
        WidgetType::MouseArea => {
            if props.mousearea_on_press {
                b.line(&format!(
                    "Message::{}Pressed => {{",
                    to_pascal_case(&name)
                ));
                b.increase_indent();
                b.line("// Handle left mouse button press");
                b.decrease_indent();
                b.line("}");
            }
            if props.mousearea_on_release {
                b.line(&format!(
                    "Message::{}Released => {{",
                    to_pascal_case(&name)
                ));
                b.increase_indent();
                b.line("// Handle left mouse button release");
                b.decrease_indent();
                b.line("}");
            }
            if props.mousearea_on_double_click {
                b.line(&format!(
                    "Message::{}DoubleClicked => {{",
                    to_pascal_case(&name)
                ));
                b.increase_indent();
                b.line("// Handle double click");
                b.line("// Note: on_press and on_release will also fire");
                b.decrease_indent();
                b.line("}");
            }
            if props.mousearea_on_right_press {
                b.line(&format!(
                    "Message::{}RightPressed => {{",
                    to_pascal_case(&name)
                ));
                b.increase_indent();
                b.line("// Handle right mouse button press");
                b.decrease_indent();
                b.line("}");
            }
            if props.mousearea_on_right_release {
                b.line(&format!(
                    "Message::{}RightReleased => {{",
                    to_pascal_case(&name)
                ));
                b.increase_indent();
                b.line("// Handle right mouse button release");
                b.decrease_indent();
                b.line("}");
            }
            if props.mousearea_on_middle_press {
                b.line(&format!(
                    "Message::{}MiddlePressed => {{",
                    to_pascal_case(&name)
                ));
                b.increase_indent();
                b.line("// Handle middle mouse button press");
                b.decrease_indent();
                b.line("}");
            }
            if props.mousearea_on_middle_release {
                b.line(&format!(
                    "Message::{}MiddleReleased => {{",
                    to_pascal_case(&name)
                ));
                b.increase_indent();
                b.line("// Handle middle mouse button release");
                b.decrease_indent();
                b.line("}");
            }
            if props.mousearea_on_scroll {
                b.line(&format!(
                    "Message::{}Scrolled(delta) => {{",
                    to_pascal_case(&name)
                ));
                b.increase_indent();
                b.line("// Handle scroll event");
                b.line("// delta is mouse::ScrollDelta enum:");
                b.line("//   Lines { x: f32, y: f32 } - scroll in lines");
                b.line("//   Pixels { x: f32, y: f32 } - scroll in pixels");
                b.line("match delta {");
                b.increase_indent();
                b.line("mouse::ScrollDelta::Lines { x, y } => {");
                b.increase_indent();
                b.line("// Handle line-based scrolling");
                b.decrease_indent();
                b.line("}");
                b.line("mouse::ScrollDelta::Pixels { x, y } => {");
                b.increase_indent();
                b.line("// Handle pixel-based scrolling");
                b.decrease_indent();
                b.line("}");
                b.decrease_indent();
                b.line("}");
                b.decrease_indent();
                b.line("}");
            }
            if props.mousearea_on_enter {
                b.line(&format!(
                    "Message::{}Entered(_point) => {{",
                    to_pascal_case(&name)
                ));
                b.increase_indent();
                b.line("// Handle mouse entering the area");
                b.decrease_indent();
                b.line("}");
            }
            if props.mousearea_on_move {
                b.line(&format!(
                    "Message::{}Moved(point) => {{",
                    to_pascal_case(&name)
                ));
                b.increase_indent();
                b.line("// Handle mouse movement within the area");
                b.line("// point is Point { x: f32, y: f32 } relative to the widget's bounds");
                b.line("let _x = point.x;");
                b.line("let _y = point.y;");
                b.decrease_indent();
                b.line("}");
            }
            if props.mousearea_on_exit {
                b.line(&format!(
                    "Message::{}Exited(_point) => {{",
                    to_pascal_case(&name)
                ));
                b.increase_indent();
                b.line("// Handle mouse leaving the area");
                b.decrease_indent();
                b.line("}");
            }
        }
        _ => {}
    }

    for child in &widget.children {
        generate_match_arms(b, child, names);
    }
}

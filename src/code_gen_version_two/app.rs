use super::builder::{CodeBuilder, to_pascal_case, to_snake_case};
use super::events::{CrossViewIntercept, ViewRefInfo};
use super::window_settings::{generate_window_settings, window_settings_are_default};
use super::{events, view};
use crate::action_system::custom_state::CustomStateField;
use crate::action_system::flow::{AppFlow, FlowTrigger};
use crate::action_system::semantic::{
    SemanticValidationContext, ViewReferenceIndex, callable_flow_ids, format_diagnostic,
    validate_and_lower_flow_graph_with_view_refs,
};
use crate::data_structures::types::type_implementations::DatePickerSelectionMode;
use crate::data_structures::types::types::{Widget, WidgetId, WidgetType, WindowConfig};
use crate::enum_builder::TypeSystem;
use crate::views::theme_and_stylefn_builder::CustomThemes;
use iced::Theme;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

fn callable_method_name(flow_id: Uuid) -> String {
    format!("callable_flow_{}", flow_id.as_simple())
}

fn startup_method_name(flow_id: Uuid) -> String {
    format!("startup_flow_{}", flow_id.as_simple())
}

fn escape_rust_string_literal(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

fn widget_tree_contains_type(widget: &Widget, target: WidgetType) -> bool {
    widget.widget_type == target
        || widget
            .children
            .iter()
            .any(|child| widget_tree_contains_type(child, target))
}

fn date_picker_parse_expr(value: &str) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        "None".to_string()
    } else {
        format!(
            "widgets::date_picker::Date::parse_from_str(\"{}\", \"%Y-%m-%d\").ok()",
            escape_rust_string_literal(trimmed)
        )
    }
}

fn date_picker_selection_initializer(
    mode: DatePickerSelectionMode,
    single: &str,
    range_start: &str,
    range_end: &str,
) -> String {
    match mode {
        DatePickerSelectionMode::Single => {
            let trimmed = single.trim();
            if trimmed.is_empty() {
                "widgets::date_picker::DateSelection::single()".to_string()
            } else {
                format!(
                    "widgets::date_picker::Date::parse_from_str(\"{}\", \"%Y-%m-%d\").ok().map(widgets::date_picker::DateSelection::with_date).unwrap_or_else(widgets::date_picker::DateSelection::single)",
                    escape_rust_string_literal(trimmed)
                )
            }
        }
        DatePickerSelectionMode::Range => format!(
            "match ({}, {}) {{ (Some(start), Some(end)) => widgets::date_picker::DateSelection::with_range(start, end), (start, end) => widgets::date_picker::DateSelection::Range {{ start, end }}, }}",
            date_picker_parse_expr(range_start),
            date_picker_parse_expr(range_end)
        ),
    }
}

fn generate_date_picker_button_label_method(b: &mut CodeBuilder) {
    b.line("fn date_picker_button_label(");
    b.increase_indent();
    b.line("selection: &widgets::date_picker::DateSelection,");
    b.line("time: widgets::date_picker::TimeSelection,");
    b.line("placeholder: &str,");
    b.line("show_time: bool,");
    b.decrease_indent();
    b.line(") -> String {");
    b.increase_indent();
    b.line("let format_time = |time: widgets::date_picker::TimeSelection| {");
    b.increase_indent();
    b.line("let is_pm = time.hour >= 12;");
    b.line("let hour = match time.hour % 12 {");
    b.increase_indent();
    b.line("0 => 12,");
    b.line("value => value,");
    b.decrease_indent();
    b.line("};");
    b.line("format!(\"{:02}:{:02} {}\", hour, time.minute, if is_pm { \"PM\" } else { \"AM\" })");
    b.decrease_indent();
    b.line("};");
    b.line("let append_time = |base: String| {");
    b.increase_indent();
    b.line("if show_time {");
    b.increase_indent();
    b.line("format!(\"{} {}\", base, format_time(time))");
    b.decrease_indent();
    b.line("} else {");
    b.increase_indent();
    b.line("base");
    b.decrease_indent();
    b.line("}");
    b.decrease_indent();
    b.line("};");
    b.line("match selection {");
    b.increase_indent();
    b.line("widgets::date_picker::DateSelection::Single(Some(date)) => {");
    b.increase_indent();
    b.line("append_time(date.format(\"%m/%d/%Y\").to_string())");
    b.decrease_indent();
    b.line("}");
    b.line("widgets::date_picker::DateSelection::Range {");
    b.increase_indent();
    b.line("start: Some(start),");
    b.line("end: Some(end),");
    b.decrease_indent();
    b.line("} => append_time(format!(\"{} -> {}\", start.format(\"%m/%d/%Y\"), end.format(\"%m/%d/%Y\"))),");
    b.line("widgets::date_picker::DateSelection::Range {");
    b.increase_indent();
    b.line("start: Some(start),");
    b.line("end: None,");
    b.decrease_indent();
    b.line("} => append_time(format!(\"{} -> ...\", start.format(\"%m/%d/%Y\"))),");
    b.line("_ => placeholder.to_string(),");
    b.decrease_indent();
    b.line("}");
    b.decrease_indent();
    b.line("}");
}

pub fn generate_app_struct(
    b: &mut CodeBuilder,
    root: &Widget,
    names: &HashMap<WidgetId, String>,
    struct_name: &str,
    type_system: &TypeSystem,
    view_refs: &[ViewRefInfo],
    is_main: bool,
    custom_state: &[CustomStateField],
    initial_view_variant: Option<&str>,
) {
    b.newline();
    if is_main {
        b.line(&format!("struct {} {{", struct_name));
    } else {
        b.line(&format!("pub struct {} {{", struct_name));
    }
    b.increase_indent();
    // current_view field comes first when view routing is active
    if initial_view_variant.is_some() {
        b.line("current_view: View,");
    }
    generate_state_fields(b, root, names, type_system, view_refs);
    for field in custom_state {
        b.line(&format!(
            "pub {}: {},",
            field.name,
            field.field_type.rust_type()
        ));
    }
    b.decrease_indent();
    b.line("}");
}

pub fn generate_impl(
    b: &mut CodeBuilder,
    root: &Widget,
    names: &HashMap<WidgetId, String>,
    struct_name: &str,
    current_view_id: Uuid,
    main_config: Option<(&WindowConfig, &Theme)>,
    type_system: &TypeSystem,
    custom_styles: &CustomThemes,
    view_refs: &[ViewRefInfo],
    view_names: &HashMap<Uuid, String>,
    all_view_widget_names: &HashMap<Uuid, HashMap<WidgetId, String>>,
    custom_state: &[CustomStateField],
    flows: &[AppFlow],
    all_flows: &[&AppFlow],
    view_reference_index: Option<&ViewReferenceIndex>,
    initial_view_variant: Option<&str>,
    emits_bubbled_navigation_message: bool,
    bubbled_navigation_subviews: &HashSet<Uuid>,
    sub_view_intercepts: &HashMap<Uuid, Vec<CrossViewIntercept>>,
) {
    let is_main = main_config.is_some();
    let view_reference_selection_index =
        events::build_view_reference_selection_index(current_view_id, view_refs);
    let callable_method_names: HashMap<Uuid, String> = flows
        .iter()
        .filter(|f| f.enabled && matches!(f.trigger, FlowTrigger::Callable))
        .map(|f| (f.id, callable_method_name(f.id)))
        .collect();

    b.line(&format!("impl {} {{", struct_name));
    b.increase_indent();

    let startup_flow_names: Vec<String> = if is_main {
        flows
            .iter()
            .filter(|f| f.enabled && matches!(f.trigger, FlowTrigger::AppStartup))
            .map(|f| startup_method_name(f.id))
            .collect()
    } else {
        Vec::new()
    };
    generate_new_method(
        b,
        root,
        names,
        type_system,
        view_refs,
        custom_state,
        &startup_flow_names,
        initial_view_variant,
    );
    b.newline();

    if widget_tree_contains_type(root, WidgetType::DatePicker) {
        generate_date_picker_button_label_method(b);
        b.newline();
    }

    // Only generate title, theme, and window settings if this is the Main App
    if let Some((window_config, theme)) = main_config {
        generate_title_method(b, &window_config.title);
        b.newline();
        generate_theme_method(b, theme);
        b.newline();
        generate_window_settings(b, window_config);
    }

    events::generate_update(
        b,
        root,
        names,
        current_view_id,
        view_refs,
        view_names,
        flows,
        all_flows,
        view_reference_index,
        is_main,
        emits_bubbled_navigation_message,
        bubbled_navigation_subviews,
        sub_view_intercepts,
        &callable_method_names,
    );
    b.newline();
    view::generate_view_method(b, root, names, custom_styles, type_system, view_refs);

    let all_names: HashMap<(Uuid, WidgetId), String> = all_view_widget_names
        .iter()
        .flat_map(|(view_id, widget_names)| {
            widget_names
                .iter()
                .map(move |(widget_id, name)| ((*view_id, *widget_id), name.clone()))
        })
        .collect();

    let mut view_field_names: HashMap<Uuid, String> = HashMap::new();
    for vr in view_refs {
        view_field_names.insert(vr.referenced_view_id, vr.field_name.clone());
        for (i, &eid) in vr.extra_view_ids.iter().enumerate() {
            if let Some((ef, _, _)) = vr.extra_views.get(i) {
                view_field_names.insert(eid, ef.clone());
            }
        }
    }

    let known_view_ids: HashSet<Uuid> = view_names
        .keys()
        .copied()
        .chain(std::iter::once(current_view_id))
        .collect();
    let callable_ids = callable_flow_ids(all_flows);
    let validation_context = SemanticValidationContext {
        callable_flow_ids: &callable_ids,
        known_view_ids: Some(&known_view_ids),
    };

    // Callable flows → private methods
    let callable_flows: Vec<&AppFlow> = flows
        .iter()
        .filter(|f| f.enabled && matches!(f.trigger, FlowTrigger::Callable))
        .collect();
    if !callable_flows.is_empty() {
        for flow in &callable_flows {
            let method_name = callable_method_names
                .get(&flow.id)
                .cloned()
                .unwrap_or_else(|| callable_method_name(flow.id));
            b.newline();
            b.line(&format!("fn {}(&mut self) {{", method_name));
            b.increase_indent();
            match validate_and_lower_flow_graph_with_view_refs(
                flow,
                &validation_context,
                view_reference_index,
            ) {
                Ok(lowered) => super::action_codegen::generate_action_graph_body(
                    b,
                    &lowered,
                    current_view_id,
                    view_names,
                    &all_names,
                    &view_field_names,
                    &view_reference_selection_index,
                    is_main,
                    0,
                    &callable_method_names,
                ),
                Err(diags) => {
                    for diag in diags {
                        b.line(&format!(
                            "// Skipped invalid action flow during codegen: {}",
                            format_diagnostic(&diag)
                        ));
                    }
                }
            }
            b.decrease_indent();
            b.line("}");
        }
    }

    // AppStartup flows → private methods (called from new())
    if is_main {
        let startup_flows: Vec<&AppFlow> = flows
            .iter()
            .filter(|f| f.enabled && matches!(f.trigger, FlowTrigger::AppStartup))
            .collect();
        for flow in &startup_flows {
            b.newline();
            b.line(&format!(
                "fn {}(&mut self) {{",
                startup_method_name(flow.id)
            ));
            b.increase_indent();
            match validate_and_lower_flow_graph_with_view_refs(
                flow,
                &validation_context,
                view_reference_index,
            ) {
                Ok(lowered) => super::action_codegen::generate_action_graph_body(
                    b,
                    &lowered,
                    current_view_id,
                    view_names,
                    &all_names,
                    &view_field_names,
                    &view_reference_selection_index,
                    is_main,
                    0,
                    &callable_method_names,
                ),
                Err(diags) => {
                    for diag in diags {
                        b.line(&format!(
                            "// Skipped invalid action flow during codegen: {}",
                            format_diagnostic(&diag)
                        ));
                    }
                }
            }
            b.decrease_indent();
            b.line("}");
        }
    }

    // Timer/KeyCombo flows → subscription() method
    let timer_key_flows: Vec<&AppFlow> = flows
        .iter()
        .filter(|f| {
            f.enabled
                && matches!(
                    f.trigger,
                    FlowTrigger::Timer { .. } | FlowTrigger::KeyCombo { .. }
                )
        })
        .collect();
    if !timer_key_flows.is_empty() {
        b.newline();
        generate_subscription_method(b, &timer_key_flows, struct_name);
    }

    b.newline();
    b.decrease_indent();
    b.line("}");
}

fn generate_subscription_method(b: &mut CodeBuilder, flows: &[&AppFlow], struct_name: &str) {
    b.line("fn subscription(&self) -> iced::Subscription<Message> {");
    b.increase_indent();
    if flows.len() == 1 {
        emit_subscription_source(b, flows[0]);
    } else {
        b.line("iced::Subscription::batch([");
        b.increase_indent();
        for flow in flows {
            b.indent();
            emit_subscription_source(b, flow);
            b.push(",");
            b.newline();
        }
        b.decrease_indent();
        b.line("])");
    }
    b.decrease_indent();
    b.line("}");
    let _ = struct_name;
}

fn emit_subscription_source(b: &mut CodeBuilder, flow: &AppFlow) {
    let msg_variant = to_pascal_case(&flow.name.replace(' ', "_"));
    match &flow.trigger {
        FlowTrigger::Timer { interval_ms } => {
            b.line(&format!(
                "iced::time::every(std::time::Duration::from_millis({})).map(|_| Message::{})",
                interval_ms, msg_variant
            ));
        }
        FlowTrigger::KeyCombo {
            ctrl,
            shift,
            alt,
            key,
        } => {
            let mods_check = {
                let mut checks = Vec::new();
                if *ctrl {
                    checks.push("mods.control()");
                }
                if *shift {
                    checks.push("mods.shift()");
                }
                if *alt {
                    checks.push("mods.alt()");
                }
                if checks.is_empty() {
                    "true".to_string()
                } else {
                    checks.join(" && ")
                }
            };
            b.line("iced::keyboard::on_key_press(|key, mods| {");
            b.increase_indent();
            b.line(&format!(
                "if key == iced::keyboard::Key::Character(\"{}\") && {} {{",
                key, mods_check
            ));
            b.increase_indent();
            b.line(&format!("Some(Message::{})", msg_variant));
            b.decrease_indent();
            b.line("} else { None }");
            b.decrease_indent();
            b.line("})");
        }
        _ => {} // AppStartup / WidgetEvent / Callable handled elsewhere
    }
}

fn generate_state_fields(
    b: &mut CodeBuilder,
    widget: &Widget,
    names: &HashMap<WidgetId, String>,
    type_system: &TypeSystem,
    view_refs: &[ViewRefInfo],
) {
    let name = names
        .get(&widget.id)
        .unwrap_or(&"widget".to_string())
        .clone();
    let props = &widget.properties;

    match widget.widget_type {
        WidgetType::TextInput => {
            b.line(&format!("pub {}_value: String,", to_snake_case(&name)));
        }
        WidgetType::Checkbox => {
            b.line(&format!("pub {}_checked: bool,", to_snake_case(&name)));
        }
        WidgetType::Radio => {
            b.line(&format!("pub {}_selected: usize,", to_snake_case(&name)));
        }
        WidgetType::Slider | WidgetType::VerticalSlider => {
            b.line(&format!("pub {}_value: f32,", to_snake_case(&name)));
        }
        WidgetType::Toggler => {
            b.line(&format!("pub {}_active: bool,", to_snake_case(&name)));
        }
        WidgetType::GenericOverlay => {
            b.line(&format!("pub {}_open: bool,", to_snake_case(&name)));
        }
        WidgetType::DatePicker => {
            b.line(&format!("pub {}_open: bool,", to_snake_case(&name)));
            b.line(&format!(
                "{}_selection: widgets::date_picker::DateSelection,",
                to_snake_case(&name)
            ));
            b.line(&format!(
                "{}_time: widgets::date_picker::TimeSelection,",
                to_snake_case(&name)
            ));
        }
        WidgetType::PickList => {
            b.line(&format!(
                "pub {}_selected: Option<String>,",
                to_snake_case(&name)
            ));
        }
        WidgetType::ComboBox => {
            if let Some(ref enum_id) = props.referenced_enum {
                if let Some(enum_def) = type_system.get_enum(enum_id.clone()) {
                    b.line(&format!(
                        "{}_value: {},",
                        to_snake_case(&name),
                        enum_def.name
                    ));
                    b.line(&format!(
                        "{}_state: combo_box::State<{}>,",
                        to_snake_case(&name),
                        enum_def.name
                    ));
                    return;
                }
            }
            b.line(&format!("{}_value: String,", to_snake_case(&name)));
            b.line(&format!(
                "{}_state: combo_box::State<String>,",
                to_snake_case(&name)
            ));
        }
        WidgetType::Markdown => {
            b.line(&format!(
                "{}_items: Vec<markdown::Item>,",
                to_snake_case(&name)
            ));
        }
        WidgetType::QRCode => {
            b.line("qr_data: qr_code::Data,");
        }
        WidgetType::Table => {
            if let Some(ref struct_id) = props.table_referenced_struct {
                if let Some(struct_def) = type_system.get_struct(*struct_id) {
                    b.line(&format!(
                        "{}_rows: Vec<{}>,",
                        to_snake_case(&name),
                        struct_def.name
                    ));
                }
            }
        }
        WidgetType::ViewReference => {
            if let Some(vr) = view_refs.iter().find(|vr| vr.widget_id == widget.id) {
                b.line(&format!("pub {}: {},", vr.field_name, vr.struct_name));
                for (ef, _, es) in &vr.extra_views {
                    b.line(&format!("pub {}: {},", ef, es));
                }
                if vr.is_multi() {
                    b.line(&format!(
                        "pub {}_selection: {},",
                        vr.field_name,
                        vr.selection_type()
                    ));
                }
            }
        }
        _ => {}
    }

    for child in &widget.children {
        generate_state_fields(b, child, names, type_system, view_refs);
    }
}

fn generate_new_method(
    b: &mut CodeBuilder,
    root: &Widget,
    names: &HashMap<WidgetId, String>,
    type_system: &TypeSystem,
    view_refs: &[ViewRefInfo],
    custom_state: &[CustomStateField],
    startup_sub_names: &[String],
    initial_view_variant: Option<&str>,
) {
    b.line("pub fn new() -> (Self, Task<Message>) {");
    b.increase_indent();

    if startup_sub_names.is_empty() {
        b.line("(");
        b.increase_indent();
        b.line("Self {");
        b.increase_indent();
        emit_new_fields(
            b,
            root,
            names,
            type_system,
            view_refs,
            custom_state,
            initial_view_variant,
        );
        b.decrease_indent();
        b.line("},");
        b.line("Task::none()");
        b.decrease_indent();
        b.line(")");
    } else {
        // Need a local variable to call startup methods on self
        b.line("let mut app = Self {");
        b.increase_indent();
        emit_new_fields(
            b,
            root,
            names,
            type_system,
            view_refs,
            custom_state,
            initial_view_variant,
        );
        b.decrease_indent();
        b.line("};");
        for name in startup_sub_names {
            b.line(&format!("app.{}();", name));
        }
        b.line("(app, Task::none())");
    }

    b.decrease_indent();
    b.line("}");
}

fn emit_new_fields(
    b: &mut CodeBuilder,
    root: &Widget,
    names: &HashMap<WidgetId, String>,
    type_system: &TypeSystem,
    view_refs: &[ViewRefInfo],
    custom_state: &[CustomStateField],
    initial_view_variant: Option<&str>,
) {
    if let Some(variant) = initial_view_variant {
        b.line(&format!("current_view: View::{},", variant));
    }
    generate_state_initializers(b, root, names, type_system, view_refs);
    for field in custom_state {
        b.line(&format!("{}: {},", field.name, field.default_expr));
    }
}

fn generate_state_initializers(
    b: &mut CodeBuilder,
    widget: &Widget,
    names: &HashMap<WidgetId, String>,
    type_system: &TypeSystem,
    view_refs: &[ViewRefInfo],
) {
    let props = &widget.properties;
    let name = names
        .get(&widget.id)
        .unwrap_or(&"widget".to_string())
        .clone();

    match widget.widget_type {
        WidgetType::TextInput => {
            b.line(&format!("{}_value: String::new(),", to_snake_case(&name)));
        }
        WidgetType::Checkbox => {
            b.line(&format!(
                "{}_checked: {},",
                to_snake_case(&name),
                if props.checkbox_checked {
                    "true"
                } else {
                    "false"
                }
            ));
        }
        WidgetType::Radio => {
            b.line(&format!(
                "{}_selected: {},",
                to_snake_case(&name),
                props.radio_selected_index
            ));
        }
        WidgetType::Slider | WidgetType::VerticalSlider => {
            b.line(&format!(
                "{}_value: {:.1},",
                to_snake_case(&name),
                props.slider_value
            ));
        }
        WidgetType::Toggler => {
            b.line(&format!(
                "{}_active: {},",
                to_snake_case(&name),
                if props.toggler_active {
                    "true"
                } else {
                    "false"
                }
            ));
        }
        WidgetType::GenericOverlay => {
            b.line(&format!(
                "{}_open: {},",
                to_snake_case(&name),
                if props.generic_overlay_initially_open {
                    "true"
                } else {
                    "false"
                }
            ));
        }
        WidgetType::DatePicker => {
            b.line(&format!(
                "{}_open: {},",
                to_snake_case(&name),
                if props.date_picker_initially_open {
                    "true"
                } else {
                    "false"
                }
            ));
            b.line(&format!(
                "{}_selection: {},",
                to_snake_case(&name),
                date_picker_selection_initializer(
                    props.date_picker_mode,
                    &props.date_picker_initial_single_date,
                    &props.date_picker_initial_range_start,
                    &props.date_picker_initial_range_end,
                )
            ));
            b.line(&format!(
                "{}_time: widgets::date_picker::TimeSelection::new({}, {}),",
                to_snake_case(&name),
                props.date_picker_initial_hour.min(23),
                props.date_picker_initial_minute.min(59)
            ));
        }
        WidgetType::PickList => {
            b.line(&format!("{}_selected: None,", to_snake_case(&name)));
        }
        WidgetType::ComboBox => {
            if let Some(ref enum_id) = props.referenced_enum {
                if let Some(enum_def) = type_system.get_enum(enum_id.clone()) {
                    let default_value = crate::enum_builder::FieldType::CustomEnum(*enum_id)
                        .default_value(type_system);
                    b.line(&format!(
                        "{}_value: {},",
                        to_snake_case(&name),
                        default_value
                    ));
                    b.line(&format!(
                        "{}_state: combo_box::State::new({}::all()),",
                        to_snake_case(&name),
                        enum_def.name
                    ));
                    return;
                }
            }
            // String-based combo box
            b.line(&format!("{}_value: String::new(),", to_snake_case(&name)));
            let options: Vec<String> = props
                .combobox_options
                .iter()
                .map(|o| format!("\"{}\".to_string()", o))
                .collect();
            b.line(&format!(
                "{}_state: combo_box::State::new(vec![{}]),",
                to_snake_case(&name),
                options.join(", ")
            ));
        }
        WidgetType::Markdown => {
            let source_text = props.markdown_source.text();
            let escaped = source_text
                .replace('\\', "\\\\")
                .replace('"', "\\\"")
                .replace('\n', "\\n");
            b.line(&format!(
                "{}_items: markdown::parse(\"{}\").collect(),",
                to_snake_case(&name),
                escaped
            ));
        }
        WidgetType::QRCode => {
            b.line(&format!(
                "qr_data: qr_code::Data::new(\"{}\").unwrap(),",
                props.qrcode_link
            ));
        }
        WidgetType::Table => {
            if let Some(ref struct_id) = props.table_referenced_struct {
                if let Some(struct_def) = type_system.get_struct(*struct_id) {
                    b.line(&format!(
                        "{}_rows: vec![],  // Add {} data here",
                        to_snake_case(&name),
                        struct_def.name
                    ));
                }
            }
        }
        WidgetType::ViewReference => {
            if let Some(vr) = view_refs.iter().find(|vr| vr.widget_id == widget.id) {
                b.line(&format!("{}: {}::new().0,", vr.field_name, vr.struct_name));
                for (ef, _, es) in &vr.extra_views {
                    b.line(&format!("{}: {}::new().0,", ef, es));
                }
                if vr.is_multi() {
                    b.line(&format!(
                        "{}_selection: {}::{},",
                        vr.field_name,
                        vr.selection_type(),
                        vr.primary_variant()
                    ));
                }
            }
        }
        _ => {}
    }

    for child in &widget.children {
        generate_state_initializers(b, child, names, type_system, view_refs);
    }
}

fn generate_title_method(b: &mut CodeBuilder, window_title: &str) {
    b.line("fn title(&self) -> String {");
    b.increase_indent();
    b.line(&format!("String::from(\"{}\")", window_title));
    b.decrease_indent();
    b.line("}");
}

fn generate_theme_method(b: &mut CodeBuilder, theme: &Theme) {
    b.line("fn theme(&self) -> Theme {");
    b.increase_indent();

    let variant = match theme {
        Theme::Light => "Light",
        Theme::Dark => "Dark",
        Theme::Dracula => "Dracula",
        Theme::Nord => "Nord",
        Theme::SolarizedLight => "SolarizedLight",
        Theme::SolarizedDark => "SolarizedDark",
        Theme::GruvboxLight => "GruvboxLight",
        Theme::GruvboxDark => "GruvboxDark",
        Theme::CatppuccinLatte => "CatppuccinLatte",
        Theme::CatppuccinFrappe => "CatppuccinFrappe",
        Theme::CatppuccinMacchiato => "CatppuccinMacchiato",
        Theme::CatppuccinMocha => "CatppuccinMocha",
        Theme::TokyoNight => "TokyoNight",
        Theme::TokyoNightStorm => "TokyoNightStorm",
        Theme::TokyoNightLight => "TokyoNightLight",
        Theme::KanagawaWave => "KanagawaWave",
        Theme::KanagawaDragon => "KanagawaDragon",
        Theme::KanagawaLotus => "KanagawaLotus",
        Theme::Moonfly => "Moonfly",
        Theme::Nightfly => "Nightfly",
        Theme::Oxocarbon => "Oxocarbon",
        Theme::Ferra => "Ferra",
        _ => "Dark",
    };

    b.line(&format!("Theme::{}", variant));
    b.decrease_indent();
    b.line("}");
}

pub fn generate_main_function(
    b: &mut CodeBuilder,
    struct_name: &str,
    window_config: Option<&WindowConfig>,
    uses_icon: bool,
    flows: &[AppFlow],
) {
    let has_timer_key_subs = flows.iter().any(|f| {
        f.enabled
            && matches!(
                f.trigger,
                FlowTrigger::Timer { .. } | FlowTrigger::KeyCombo { .. }
            )
    });

    b.line("pub fn main() -> iced::Result {");
    b.increase_indent();

    b.indent();
    b.push(&format!(
        "iced::application({}::new, {}::update, {}::view)",
        struct_name, struct_name, struct_name
    ));
    b.newline();

    b.increase_indent();
    if let Some(window_config) = window_config {
        if !window_settings_are_default(&window_config.settings) {
            b.indent();
            b.push(&format!(".window({}::window_settings())", struct_name));
            b.newline();
        }
    }

    if has_timer_key_subs {
        b.indent();
        b.push(&format!(".subscription({}::subscription)", struct_name));
        b.newline();
    }

    b.indent();
    b.push(&format!(".theme({}::theme)", struct_name));
    b.newline();

    b.indent();
    b.push(&format!(".title({}::title)", struct_name));
    b.newline();

    if uses_icon {
        b.indent();
        b.push(".font(icon::FONT)");
        b.newline();
    }

    b.indent();
    b.push(".run()");
    b.newline();

    b.decrease_indent();
    b.decrease_indent();
    b.line("}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};

    use crate::data_structures::types::types::{Widget, WidgetId, WidgetType};

    #[test]
    fn generate_state_initializers_respects_generic_overlay_initial_open() {
        let overlay_id = WidgetId(7);
        let mut overlay = Widget::new(WidgetType::GenericOverlay, overlay_id);
        overlay.properties.generic_overlay_initially_open = true;

        let mut b = CodeBuilder::new();
        generate_state_initializers(
            &mut b,
            &overlay,
            &HashMap::from([(overlay_id, "login_overlay".to_string())]),
            &TypeSystem::new(),
            &[],
        );

        assert!(b.build().contains("login_overlay_open: true,"));
    }

    #[test]
    fn generate_state_initializers_respects_date_picker_initial_open() {
        let date_picker_id = WidgetId(8);
        let mut date_picker = Widget::new(WidgetType::DatePicker, date_picker_id);
        date_picker.properties.date_picker_initially_open = true;

        let mut b = CodeBuilder::new();
        generate_state_initializers(
            &mut b,
            &date_picker,
            &HashMap::from([(date_picker_id, "date_picker".to_string())]),
            &TypeSystem::new(),
            &[],
        );

        let generated = b.build();
        assert!(generated.contains("date_picker_open: true,"));
        assert!(generated.contains("date_picker_selection:"));
        assert!(
            generated.contains("date_picker_time: widgets::date_picker::TimeSelection::new(0, 0),")
        );
    }

    #[test]
    fn generate_impl_skips_startup_methods_for_subviews() {
        let theme = Theme::Dark;
        let custom_styles = CustomThemes::new(&theme);
        let root = Widget::new(WidgetType::Container, WidgetId(0));
        let current_view_id = Uuid::new_v4();
        let view_names = HashMap::from([(current_view_id, "Subview".to_string())]);
        let all_view_widget_names = HashMap::from([(current_view_id, HashMap::new())]);
        let flows = vec![AppFlow::new("startup".to_string(), FlowTrigger::AppStartup)];
        let all_flows: Vec<&AppFlow> = flows.iter().collect();
        let mut b = CodeBuilder::new();

        generate_impl(
            &mut b,
            &root,
            &HashMap::new(),
            "Subview",
            current_view_id,
            None,
            &TypeSystem::new(),
            &custom_styles,
            &[],
            &view_names,
            &all_view_widget_names,
            &[],
            &flows,
            &all_flows,
            None,
            None,
            false,
            &HashSet::new(),
            &HashMap::new(),
        );

        let code = b.build();
        assert!(!code.contains("startup_flow_"));
        assert!(!code.contains("app.startup_flow_"));
    }
}

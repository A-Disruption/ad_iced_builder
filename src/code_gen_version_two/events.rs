use super::action_codegen::{ViewReferenceSelectionCodegen, ViewReferenceSelectionIndex};
use super::builder::{CodeBuilder, handle_whitespace, to_pascal_case, to_snake_case};
use crate::action_system::flow::{AppFlow, FlowTrigger};
use crate::action_system::graph::ActionNodeId;
use crate::action_system::node_kinds::{ActionNodeKind, NavigateTarget};
use crate::action_system::semantic::{
    LoweredActionGraph, LoweredExpression, LoweredWidgetEventResult, SemanticDiagnostic,
    SemanticValidationContext, ViewReferenceIndex, callable_flow_ids, format_diagnostic,
    lower_widget_event_flows_with_view_refs, validate_and_lower_flow_graph_with_view_refs,
};
use crate::action_system::state_ref::StateRefSource;
use crate::data_structures::types::type_implementations::{
    ContainerAlignX, ContainerAlignY, FontType, PaddingMode,
};
use crate::data_structures::types::types::{AppView, Widget, WidgetId, WidgetType};
use crate::enum_builder::TypeSystem;
use iced::Alignment;
use iced::advanced::text;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use uuid::Uuid;

fn sub_variant_name(name: &str) -> String {
    to_pascal_case(&name.replace(|c: char| !c.is_alphanumeric(), "_"))
}

/// Returns matching `(graph, trigger_output_slot)` pairs for all flows triggered by this widget event.
/// The slot is the flow output port index on the Trigger node for the matched row.
fn find_flow_graphs<'a>(
    all_flows: &[&'a AppFlow],
    view_id: Uuid,
    widget_id: WidgetId,
    event_type: &str,
    known_view_ids: Option<&HashSet<Uuid>>,
    view_reference_index: Option<&ViewReferenceIndex>,
) -> LoweredWidgetEventResult {
    lower_widget_event_flows_with_view_refs(
        all_flows,
        view_id,
        widget_id,
        event_type,
        known_view_ids,
        view_reference_index,
    )
}

fn flow_reaches_app_view_navigation(
    graph: &LoweredActionGraph,
    from_node: ActionNodeId,
    output_slot: usize,
    depth: usize,
) -> bool {
    if depth > 100 {
        return false;
    }

    for (next_id, input_slot) in graph.flow_successors_with_input_slots(from_node, output_slot) {
        let Some(node) = graph.node(next_id) else {
            continue;
        };

        if matches!(
            graph.navigation_target_for_input_slot(node.id, input_slot),
            Some(NavigateTarget::AppView { .. })
        ) {
            return true;
        }

        let continuation_slots: Vec<usize> = match &node.kind {
            ActionNodeKind::Conditional => vec![0, 1],
            ActionNodeKind::Match { arms, .. } => (0..=arms.len()).collect(),
            ActionNodeKind::NavigateToView { .. } => vec![input_slot],
            _ => vec![0],
        };
        for continuation_slot in continuation_slots {
            if flow_reaches_app_view_navigation(graph, next_id, continuation_slot, depth + 1) {
                return true;
            }
        }
    }

    false
}

pub fn view_requires_bubbled_navigation_message(
    view_id: Uuid,
    all_flows: &[&AppFlow],
    known_view_ids: &HashSet<Uuid>,
    view_reference_index: Option<&ViewReferenceIndex>,
) -> bool {
    let callable_ids = callable_flow_ids(all_flows);
    let validation_context = SemanticValidationContext {
        callable_flow_ids: &callable_ids,
        known_view_ids: Some(known_view_ids),
    };

    for flow in all_flows {
        if !flow.enabled {
            continue;
        }

        match &flow.trigger {
            FlowTrigger::WidgetEvent { rows } => {
                if !rows.iter().any(
                    |row| matches!(row.target, Some((target_view_id, _)) if target_view_id == view_id),
                ) {
                    continue;
                }

                let Ok(lowered) = validate_and_lower_flow_graph_with_view_refs(
                    flow,
                    &validation_context,
                    view_reference_index,
                ) else {
                    continue;
                };
                let trigger_id = lowered.trigger_node_id();

                for (row_idx, row) in rows.iter().enumerate() {
                    let Some((target_view_id, _)) = row.target else {
                        continue;
                    };
                    if target_view_id != view_id {
                        continue;
                    }

                    let trigger_slot =
                        crate::action_system::graph::ActionNodeData::widget_event_row_slot(
                            rows, row_idx,
                        );
                    if flow_reaches_app_view_navigation(&lowered, trigger_id, trigger_slot, 0) {
                        return true;
                    }
                }
            }
            FlowTrigger::Timer { .. } | FlowTrigger::KeyCombo { .. } => {
                let Ok(lowered) = validate_and_lower_flow_graph_with_view_refs(
                    flow,
                    &validation_context,
                    view_reference_index,
                ) else {
                    continue;
                };
                if flow_reaches_app_view_navigation(&lowered, lowered.trigger_node_id(), 0, 0) {
                    return true;
                }
            }
            _ => {}
        }
    }

    false
}

fn emit_semantic_diagnostics_as_comments(b: &mut CodeBuilder, diagnostics: &[SemanticDiagnostic]) {
    for diag in diagnostics {
        b.line(&format!(
            "// Skipped invalid action flow during codegen: {}",
            format_diagnostic(diag)
        ));
    }
}

fn emit_lowered_action_graphs(
    b: &mut CodeBuilder,
    lowered: &LoweredWidgetEventResult,
    current_view_id: Uuid,
    view_names: &HashMap<Uuid, String>,
    all_names: &HashMap<(Uuid, WidgetId), String>,
    view_field_names: &HashMap<Uuid, String>,
    view_reference_selection_index: &ViewReferenceSelectionIndex,
    is_main: bool,
    callable_method_names: &HashMap<Uuid, String>,
) -> bool {
    emit_semantic_diagnostics_as_comments(b, &lowered.diagnostics);
    if lowered.flows.is_empty() {
        return false;
    }
    for lowered_flow in &lowered.flows {
        super::action_codegen::generate_action_graph_body(
            b,
            &lowered_flow.graph,
            current_view_id,
            view_names,
            all_names,
            view_field_names,
            view_reference_selection_index,
            is_main,
            lowered_flow.trigger_slot,
            callable_method_names,
        );
    }
    true
}

/// Metadata about a ViewReference widget resolved against the views map.
#[derive(Debug, Clone)]
pub struct ViewRefInfo {
    pub widget_id: WidgetId,
    /// The UUID of the primary referenced view.
    pub referenced_view_id: Uuid,
    /// snake_case field name used in the App struct (from widget_name or view name)
    pub field_name: String,
    /// The actual name of the primary view (used for Selection enum primary variant).
    pub primary_view_name: String,
    /// snake_case module name (same as generated file stem)
    pub module_name: String,
    /// PascalCase struct name
    pub struct_name: String,
    /// Additional views for multi-view selection: (field_name, module_name, struct_name).
    /// When non-empty, a `{PrimaryField}Selection` enum is generated and view() uses a match.
    pub extra_views: Vec<(String, String, String)>,
    /// UUIDs of extra referenced views, parallel to `extra_views`.
    pub extra_view_ids: Vec<Uuid>,
}

impl ViewRefInfo {
    pub fn is_multi(&self) -> bool {
        !self.extra_views.is_empty()
    }
    /// PascalCase name of the generated selection enum, e.g. `NavigationBarSelection`.
    pub fn selection_type(&self) -> String {
        format!("{}Selection", to_pascal_case(&self.field_name))
    }
    /// PascalCase name for the primary variant in the Selection enum (uses actual view name).
    pub fn primary_variant(&self) -> String {
        to_pascal_case(&handle_whitespace(&self.primary_view_name))
    }
    /// PascalCase name for use in ViewMessages enum variant (uses actual view name, not widget name).
    pub fn msg_variant(&self) -> String {
        to_pascal_case(&handle_whitespace(&self.primary_view_name))
    }
}

pub fn build_view_reference_selection_index(
    owner_view_id: Uuid,
    view_refs: &[ViewRefInfo],
) -> ViewReferenceSelectionIndex {
    let mut index = ViewReferenceSelectionIndex::new();
    for vr in view_refs.iter().filter(|vr| vr.is_multi()) {
        let mut variants_by_view: HashMap<Uuid, String> = HashMap::new();
        variants_by_view.insert(vr.referenced_view_id, vr.primary_variant());
        for (i, extra_view_id) in vr.extra_view_ids.iter().enumerate() {
            if let Some((field_name, _, _)) = vr.extra_views.get(i) {
                variants_by_view.insert(
                    *extra_view_id,
                    to_pascal_case(&handle_whitespace(field_name)),
                );
            }
        }
        index.insert(
            (owner_view_id, vr.widget_id),
            ViewReferenceSelectionCodegen {
                field_name: vr.field_name.clone(),
                selection_type: vr.selection_type(),
                variants_by_view,
            },
        );
    }
    index
}

/// A parent-side intercept: when main sees `module::Message::variant_pattern` from a sub-view,
/// emit the assignments directly before forwarding to the sub-view's update().
#[derive(Debug, Clone)]
pub struct CrossViewIntercept {
    /// Pattern after `Message::`, with `(_)` for ignored payloads. E.g. "ButtonPressed" or "CheckboxToggled(_)".
    pub variant_pattern: String,
    /// Assignment lines to emit in main. E.g. ["self.view_2_selection = View2Selection::View3;"].
    pub assignments: Vec<String>,
}

/// For a given sub-view, collect all parent-side intercepts needed: flows (across ALL views) that
/// contain StateMutation nodes targeting `parent_view_id` and are triggered by widgets in `sub_view_id`.
/// Uses per-row execution path walking (not a full node scan) so each trigger row only collects
/// StateMutation nodes reachable from its specific output port on the Trigger node.
/// Deduplicates by variant_pattern so multiple flows sharing the same trigger merge assignments.
pub fn collect_cross_view_intercepts(
    all_flows: &[&AppFlow],
    sub_view_id: Uuid,
    sub_view_root: &Widget,
    sub_view_names: &HashMap<WidgetId, String>,
    parent_view_names: &HashMap<WidgetId, String>,
    parent_view_id: Uuid,
    view_reference_index: Option<&ViewReferenceIndex>,
    parent_view_reference_selection_index: &ViewReferenceSelectionIndex,
) -> Vec<CrossViewIntercept> {
    // Group assignments by variant_pattern so flows sharing a trigger merge their assignments
    let mut by_pattern: HashMap<String, Vec<String>> = HashMap::new();
    let callable_ids = callable_flow_ids(all_flows);
    let validation_context = SemanticValidationContext {
        callable_flow_ids: &callable_ids,
        known_view_ids: None,
    };

    for flow in all_flows {
        if !flow.enabled {
            continue;
        }
        let FlowTrigger::WidgetEvent { rows } = &flow.trigger else {
            continue;
        };
        let lowered_graph = match validate_and_lower_flow_graph_with_view_refs(
            flow,
            &validation_context,
            view_reference_index,
        ) {
            Ok(lowered) => lowered,
            Err(diags) => {
                for diag in diags {
                    eprintln!(
                        "Action flow skipped in codegen intercepts: {}",
                        format_diagnostic(&diag)
                    );
                }
                continue;
            }
        };
        let trigger_id = lowered_graph.trigger_node_id();

        // For each trigger row that targets a widget in sub_view_id
        for (row_idx, row) in rows.iter().enumerate() {
            let Some((target_view_id, widget_id_raw)) = row.target else {
                continue;
            };
            if target_view_id != sub_view_id {
                continue;
            }

            let widget_id = WidgetId(widget_id_raw);
            let Some(widget) = find_widget_by_id(sub_view_root, widget_id) else {
                continue;
            };
            let snake_name = sub_view_names
                .get(&widget_id)
                .cloned()
                .unwrap_or_else(|| "widget".to_string());
            let Some(pattern) = widget_event_variant_pattern(widget, &snake_name, &row.event_type)
            else {
                continue;
            };

            // Walk only the execution path from this row's trigger output slot
            let trigger_slot =
                crate::action_system::graph::ActionNodeData::widget_event_row_slot(rows, row_idx);
            let assignments = collect_reachable_state_mutations(
                &lowered_graph,
                trigger_id,
                trigger_slot,
                parent_view_id,
                parent_view_names,
                parent_view_reference_selection_index,
                0,
            );
            if assignments.is_empty() {
                continue;
            }

            by_pattern.entry(pattern).or_default().extend(assignments);
        }
    }

    by_pattern
        .into_iter()
        .map(|(variant_pattern, assignments)| CrossViewIntercept {
            variant_pattern,
            assignments,
        })
        .collect()
}

/// Recursively walks the flow graph from `from_node` output slot `output_slot`,
/// collecting StateMutation assignment strings for nodes that target `parent_view_id`.
fn collect_reachable_state_mutations(
    graph: &LoweredActionGraph,
    from_node: ActionNodeId,
    output_slot: usize,
    parent_view_id: Uuid,
    parent_view_names: &HashMap<WidgetId, String>,
    parent_view_reference_selection_index: &ViewReferenceSelectionIndex,
    depth: usize,
) -> Vec<String> {
    if depth > 100 {
        return vec![];
    }
    let successors = graph.flow_successors_with_input_slots(from_node, output_slot);
    if successors.is_empty() {
        return vec![];
    }
    let mut result = vec![];
    for (next_id, input_slot) in successors {
        let Some(node) = graph.node(next_id) else {
            continue;
        };
        if let ActionNodeKind::StateMutation { assignments } = &node.kind {
            for (idx, a) in assignments.iter().enumerate() {
                if let Some(target) = &a.target {
                    if target.view_id == parent_view_id {
                        let field_name = match &target.source {
                            StateRefSource::ViewSelection { field_name, .. } => field_name.clone(),
                            StateRefSource::Custom { field_name, .. } => field_name.clone(),
                            StateRefSource::Widget {
                                widget_id,
                                field_suffix,
                            } => {
                                let widget_name = parent_view_names
                                    .get(widget_id)
                                    .cloned()
                                    .unwrap_or_else(|| format!("widget_{}", widget_id.0));
                                format!("{}{}", widget_name, field_suffix)
                            }
                        };
                        let input_label = format!("value_{idx}");
                        let assignment = if let Some(expr) =
                            graph.expression_for_input_label(node.id, &input_label)
                        {
                            if let Some(rhs) = emit_assignment_expression(expr) {
                                format!("self.{} = {};", field_name, rhs)
                            } else {
                                format!(
                                    "// StateMutation({}): expression not emit-safe for cross-view intercept",
                                    field_name
                                )
                            }
                        } else {
                            format!(
                                "// StateMutation({}): missing lowered expression ({})",
                                field_name, input_label
                            )
                        };
                        result.push(assignment);
                    }
                }
            }
        }
        if let Some(NavigateTarget::ViewReference {
            owner_view_id,
            widget_id,
            target_view_id,
        }) = graph.navigation_target_for_input_slot(node.id, input_slot)
        {
            if *owner_view_id == parent_view_id {
                match parent_view_reference_selection_index.get(&(*owner_view_id, *widget_id)) {
                    Some(selection) => {
                        if let Some(variant) = selection.variants_by_view.get(target_view_id) {
                            result.push(format!(
                                "self.{}_selection = {}::{};",
                                selection.field_name, selection.selection_type, variant
                            ));
                        } else {
                            result.push(format!(
                                "// NavigateToView: target view {} missing in ViewReference ({}, {})",
                                target_view_id, owner_view_id, widget_id.0
                            ));
                        }
                    }
                    None => result.push(format!(
                        "// NavigateToView: unresolved ViewReference ({}, {})",
                        owner_view_id, widget_id.0
                    )),
                }
            }
        }

        let continuation_slots: Vec<usize> = match &node.kind {
            ActionNodeKind::Conditional => vec![0, 1],
            ActionNodeKind::Match { arms, .. } => (0..=arms.len()).collect(),
            ActionNodeKind::NavigateToView { .. } => vec![input_slot],
            _ => vec![0],
        };
        for continuation_slot in continuation_slots {
            result.extend(collect_reachable_state_mutations(
                graph,
                next_id,
                continuation_slot,
                parent_view_id,
                parent_view_names,
                parent_view_reference_selection_index,
                depth + 1,
            ));
        }
    }
    result
}

fn emit_assignment_expression(expr: &LoweredExpression) -> Option<String> {
    match expr {
        LoweredExpression::Literal(v) => Some(v.rust_literal()),
        LoweredExpression::StateField(_) => None,
        LoweredExpression::TriggerInput { .. } => None,
        LoweredExpression::Formula(_) => None,
        LoweredExpression::Compare { .. }
        | LoweredExpression::LogicAnd { .. }
        | LoweredExpression::LogicOr { .. }
        | LoweredExpression::LogicNot { .. } => None,
    }
}

fn find_widget_by_id(root: &Widget, id: WidgetId) -> Option<&Widget> {
    if root.id == id {
        return Some(root);
    }
    for child in &root.children {
        if let Some(w) = find_widget_by_id(child, id) {
            return Some(w);
        }
    }
    None
}

fn widget_event_variant_pattern(
    widget: &Widget,
    snake_name: &str,
    event_type: &str,
) -> Option<String> {
    let pascal = to_pascal_case(snake_name);
    let has_custom = !widget.properties.widget_name.trim().is_empty();
    match widget.widget_type {
        WidgetType::Button => match event_type {
            "on_press" => {
                let props = &widget.properties;
                if props.button_on_press_enabled
                    || props.button_on_press_maybe_enabled
                    || props.button_on_press_with_enabled
                {
                    if has_custom {
                        Some(pascal)
                    } else {
                        Some(format!("{}Pressed", pascal))
                    }
                } else {
                    None
                }
            }
            _ => None,
        },
        WidgetType::TextInput => match event_type {
            "on_input" => {
                if has_custom {
                    Some(format!("{}(_)", pascal))
                } else {
                    Some(format!("{}OnInput(_)", pascal))
                }
            }
            "on_submit" => {
                if has_custom {
                    Some(format!("{}Submitted", pascal))
                } else {
                    Some(format!("{}Submitted", pascal))
                }
            }
            "on_paste" => {
                if has_custom {
                    Some(format!("{}Pasted(_)", pascal))
                } else {
                    Some(format!("{}Pasted(_)", pascal))
                }
            }
            _ => None,
        },
        WidgetType::Checkbox => match event_type {
            "on_toggle" => {
                if has_custom {
                    Some(format!("{}(_)", pascal))
                } else {
                    Some(format!("{}Toggled(_)", pascal))
                }
            }
            _ => None,
        },
        WidgetType::Radio => match event_type {
            "on_select" => {
                if has_custom {
                    Some(format!("{}(_)", pascal))
                } else {
                    Some(format!("{}Selected(_)", pascal))
                }
            }
            _ => None,
        },
        WidgetType::Slider | WidgetType::VerticalSlider => match event_type {
            "on_change" => {
                if has_custom {
                    Some(format!("{}(_)", pascal))
                } else {
                    Some(format!("{}Changed(_)", pascal))
                }
            }
            _ => None,
        },
        WidgetType::Toggler => match event_type {
            "on_toggle" => {
                if has_custom {
                    Some(format!("{}(_)", pascal))
                } else {
                    Some(format!("{}Toggled(_)", pascal))
                }
            }
            _ => None,
        },
        WidgetType::PickList => match event_type {
            "on_select" => {
                if has_custom {
                    Some(format!("{}(_)", pascal))
                } else {
                    Some(format!("{}Selected(_)", pascal))
                }
            }
            _ => None,
        },
        WidgetType::MouseArea => match event_type {
            "on_press" => Some(format!("{}Pressed", pascal)),
            "on_release" => Some(format!("{}Released", pascal)),
            "on_right_press" => Some(format!("{}RightPressed", pascal)),
            _ => None,
        },
        _ => None,
    }
}

/// Collect all ViewReference widgets from a widget tree, resolving each against the views map.
pub fn collect_view_refs(root: &Widget, views: &BTreeMap<Uuid, AppView>) -> Vec<ViewRefInfo> {
    let mut refs = Vec::new();
    collect_view_refs_recursive(root, views, &mut refs);
    refs
}

fn collect_view_refs_recursive(
    widget: &Widget,
    views: &BTreeMap<Uuid, AppView>,
    refs: &mut Vec<ViewRefInfo>,
) {
    if widget.widget_type == WidgetType::ViewReference {
        if let Some(view_id) = widget.properties.referenced_view_id {
            if let Some(view) = views.get(&view_id) {
                let field_name = if !widget.properties.widget_name.trim().is_empty() {
                    to_snake_case(&widget.properties.widget_name)
                } else {
                    to_snake_case(&view.name)
                };
                let extra_view_ids: Vec<Uuid> = widget.properties.extra_view_ids.clone();
                let extra_views: Vec<(String, String, String)> = extra_view_ids
                    .iter()
                    .filter_map(|eid| {
                        views.get(eid).map(|ev| {
                            (
                                to_snake_case(&ev.name),
                                to_snake_case(&ev.name),
                                to_pascal_case(&handle_whitespace(&ev.name)),
                            )
                        })
                    })
                    .collect();
                refs.push(ViewRefInfo {
                    widget_id: widget.id,
                    referenced_view_id: view_id,
                    field_name,
                    primary_view_name: view.name.clone(),
                    module_name: to_snake_case(&view.name),
                    struct_name: to_pascal_case(&handle_whitespace(&view.name)),
                    extra_views,
                    extra_view_ids,
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
                if widget.children.is_empty() {
                    self.used_widgets.insert("text");
                }
            }
            WidgetType::Row => {
                self.used_widgets.insert("row");
                if widget.children.is_empty() {
                    self.used_widgets.insert("text");
                }
            }
            WidgetType::Column => {
                self.used_widgets.insert("column");
                if widget.children.is_empty() {
                    self.used_widgets.insert("text");
                }
            }
            WidgetType::Button => {
                self.used_widgets.insert("button");
                // button always generates text(...) for content, either as placeholder or through child scan
                self.used_widgets.insert("text");
            }
            WidgetType::Text => {
                self.used_widgets.insert("text");
            }
            WidgetType::TextInput => {
                self.used_widgets.insert("text_input");
            }
            WidgetType::Checkbox => {
                self.used_widgets.insert("checkbox");
            }
            WidgetType::Collapsible => {
                if widget.children.is_empty() {
                    self.used_widgets.insert("space");
                }
            }
            WidgetType::CollapsibleGroup => {}
            WidgetType::GenericOverlay => {}
            WidgetType::DatePicker => {
                self.used_widgets.insert("button");
                self.used_widgets.insert("stack");
                self.used_widgets.insert("text");
            }
            WidgetType::Radio => {
                self.used_widgets.insert("radio");
            }
            WidgetType::Slider => {
                self.used_widgets.insert("slider");
            }
            WidgetType::VerticalSlider => {
                self.used_widgets.insert("vertical_slider");
            }
            WidgetType::ProgressBar => {
                self.used_widgets.insert("progress_bar");
            }
            WidgetType::Toggler => {
                self.used_widgets.insert("toggler");
            }
            WidgetType::PickList => {
                self.used_widgets.insert("pick_list");
            }
            WidgetType::Scrollable => {
                self.used_widgets.insert("scrollable");
                if widget.children.is_empty() {
                    self.used_widgets.insert("text");
                }
            }
            WidgetType::Space => {
                self.used_widgets.insert("space");
            }
            WidgetType::Rule => {
                self.used_widgets.insert("rule");
            }
            WidgetType::Image => {
                self.used_widgets.insert("image");
            }
            WidgetType::Svg => {
                self.used_widgets.insert("svg");
            }
            WidgetType::Tooltip => {
                self.used_widgets.insert("tooltip");
                // tooltip always generates text(...) fallbacks for host and/or content
                self.used_widgets.insert("text");
            }
            WidgetType::ComboBox => {
                self.used_widgets.insert("combo_box");
            }
            WidgetType::Markdown => {
                self.used_widgets.insert("markdown");
            }
            WidgetType::MouseArea => {
                self.used_widgets.insert("mouse_area");
                self.uses_mouse = true;
            }
            WidgetType::QRCode => {
                self.used_widgets.insert("qr_code");
            }
            WidgetType::Stack => {
                self.used_widgets.insert("stack");
                if widget.children.is_empty() {
                    self.used_widgets.insert("text");
                }
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
                if widget.children.is_empty() {
                    self.used_widgets.insert("text");
                }
            }
            WidgetType::Pin => {
                self.used_widgets.insert("pin");
                if widget.children.is_empty() {
                    self.used_widgets.insert("text");
                }
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
                // Icons use icon::name() - no text/Font imports needed at call site
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

        if widget.widget_type == WidgetType::Collapsible {
            if props.align_x != ContainerAlignX::Left {
                self.uses_alignment = true;
            }
            if props.font != FontType::Default {
                self.uses_font = true;
            }
            if props.padding_mode == PaddingMode::Individual {
                self.uses_padding = true;
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

pub fn generate_imports(b: &mut CodeBuilder, root: &Widget, is_main: bool) -> ImportTracker {
    let mut tracker = ImportTracker::new();
    tracker.scan_widget(root);

    b.push("use iced::{");
    b.newline();
    b.increase_indent();

    let mut core_imports = Vec::new();
    if tracker.uses_length {
        core_imports.push("Length");
    }
    if tracker.uses_alignment {
        core_imports.push("Alignment");
    }
    if tracker.uses_color {
        core_imports.push("Color");
    }
    if tracker.uses_padding {
        core_imports.push("Padding");
    }
    if tracker.uses_font {
        core_imports.push("Font");
    }
    if tracker.uses_font_module {
        core_imports.push("font");
    }
    if tracker.uses_border {
        core_imports.push("Border");
    }
    if tracker.uses_shadow {
        core_imports.push("Shadow");
    }
    if tracker.uses_background {
        core_imports.push("Background");
    }
    if tracker.uses_vector {
        core_imports.push("Vector");
    }
    if tracker.uses_point {
        core_imports.push("Point");
    }
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
        if tracker.uses_text_line_height {
            text_items.push("LineHeight");
        }
        if tracker.uses_text_wrapping {
            text_items.push("Wrapping");
        }
        if tracker.uses_text_shaping {
            text_items.push("Shaping");
        }
        if tracker.uses_text_alignment {
            text_items.push("Alignment as TextAlignment");
        }

        if !text_items.is_empty() {
            b.push(&format!("widget::text::{{{}}},", text_items.join(", ")));
        }
        b.newline();
    }

    b.decrease_indent();
    b.line("};");

    if tracker.uses_icon {
        if is_main {
            b.line("mod icon;");
        } else {
            b.line("use crate::icon;");
        }
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
    flows: &[AppFlow],
    is_main: bool,
    emits_bubbled_navigation_message: bool,
) {
    b.newline();
    b.line("#[derive(Debug, Clone)]");
    b.line("pub enum Message {");
    b.increase_indent();

    let has_variants = generate_message_variants(b, root, names, type_system);

    // Timer / KeyCombo flows emit a message variant each (AppStartup and Callable do not)
    let mut has_sub_variants = false;
    for flow in flows {
        if !flow.enabled {
            continue;
        }
        match &flow.trigger {
            FlowTrigger::Timer { .. } | FlowTrigger::KeyCombo { .. } => {
                b.line(&format!("{},", sub_variant_name(&flow.name)));
                has_sub_variants = true;
            }
            _ => {}
        }
    }

    // Non-main views only need NavigateTo when they can bubble an app-view switch to main.
    if !is_main && emits_bubbled_navigation_message {
        b.line("NavigateTo(View),");
    }

    if !view_refs.is_empty() {
        b.line("ViewMessages(ViewMessages),");
    }
    if !has_variants
        && !has_sub_variants
        && view_refs.is_empty()
        && !emits_bubbled_navigation_message
    {
        b.line("Noop,");
    }

    b.decrease_indent();
    b.line("}");

    // Generate the ViewMessages sub-enum if there are view references
    if !view_refs.is_empty() {
        b.newline();
        b.line("#[derive(Debug, Clone)]");
        b.line("pub enum ViewMessages {");
        b.increase_indent();
        for vr in view_refs {
            b.line(&format!(
                "{}({}::Message),",
                vr.msg_variant(),
                vr.module_name
            ));
            for (ef, em, _) in &vr.extra_views {
                b.line(&format!("{}({}::Message),", to_pascal_case(ef), em));
            }
        }
        b.decrease_indent();
        b.line("}");
    }
}

/// Emit selection enum definitions for multi-view ViewReference widgets.
pub fn generate_view_selection_enums(b: &mut CodeBuilder, view_refs: &[ViewRefInfo]) {
    for vr in view_refs {
        if !vr.is_multi() {
            continue;
        }
        b.newline();
        b.line("#[derive(Debug, Clone, Copy, PartialEq, Eq)]");
        b.line(&format!("pub enum {} {{", vr.selection_type()));
        b.increase_indent();
        b.line(&format!("{},", vr.primary_variant()));
        for (ef, _, _) in &vr.extra_views {
            b.line(&format!("{},", to_pascal_case(ef)));
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
) -> bool {
    let name = names
        .get(&widget.id)
        .unwrap_or(&"widget".to_string())
        .clone();
    let props = &widget.properties;
    let mut emitted = false;

    let has_custom_name = !props.widget_name.trim().is_empty();
    let v = to_pascal_case(&name);
    match widget.widget_type {
        WidgetType::Button => {
            if props.button_on_press_enabled
                || props.button_on_press_maybe_enabled
                || props.button_on_press_with_enabled
            {
                if has_custom_name {
                    b.line(&format!("{},", v));
                } else {
                    b.line(&format!("{}Pressed,", v));
                }
                emitted = true;
            }
        }
        WidgetType::TextInput => {
            if has_custom_name {
                b.line(&format!("{}(String),", v));
            } else {
                b.line(&format!("{}OnInput(String),", v));
            }
            emitted = true;
            if props.text_input_on_submit {
                b.line(&format!("{}Submitted,", v));
            }
            if props.text_input_on_paste {
                b.line(&format!("{}Pasted(String),", v));
            }
        }
        WidgetType::Checkbox => {
            if has_custom_name {
                b.line(&format!("{}(bool),", v));
            } else {
                b.line(&format!("{}Toggled(bool),", v));
            }
            emitted = true;
        }
        WidgetType::Radio => {
            if has_custom_name {
                b.line(&format!("{}(usize),", v));
            } else {
                b.line(&format!("{}Selected(usize),", v));
            }
            emitted = true;
        }
        WidgetType::Slider | WidgetType::VerticalSlider => {
            if has_custom_name {
                b.line(&format!("{}(f32),", v));
            } else {
                b.line(&format!("{}Changed(f32),", v));
            }
            emitted = true;
        }
        WidgetType::Toggler => {
            if has_custom_name {
                b.line(&format!("{}(bool),", v));
            } else {
                b.line(&format!("{}Toggled(bool),", v));
            }
            emitted = true;
        }
        WidgetType::GenericOverlay => {
            if has_custom_name {
                b.line(&format!("{}(bool),", v));
            } else {
                b.line(&format!("{}Toggled(bool),", v));
            }
            emitted = true;
        }
        WidgetType::DatePicker => {
            b.line(&format!("{}OpenRequested,", v));
            b.line(&format!(
                "{}Changed(widgets::date_picker::DateSelection),",
                v
            ));
            b.line(&format!(
                "{}ChangedWithTime(widgets::date_picker::DateSelection, widgets::date_picker::TimeSelection),",
                v
            ));
            b.line(&format!("{}Closed,", v));
            emitted = true;
        }
        WidgetType::PickList => {
            if has_custom_name {
                b.line(&format!("{}(String),", v));
            } else {
                b.line(&format!("{}Selected(String),", v));
            }
            emitted = true;
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

            b.line(&format!(
                "{}Selected({}),",
                to_pascal_case(&name),
                type_name
            ));
            emitted = true;
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
            b.line(&format!(
                "{}LinkClicked(markdown::Uri),",
                to_pascal_case(&name)
            ));
            emitted = true;
        }
        WidgetType::MouseArea => {
            let any = props.mousearea_on_press
                || props.mousearea_on_release
                || props.mousearea_on_double_click
                || props.mousearea_on_right_press
                || props.mousearea_on_right_release
                || props.mousearea_on_middle_press
                || props.mousearea_on_middle_release
                || props.mousearea_on_scroll
                || props.mousearea_on_enter
                || props.mousearea_on_move
                || props.mousearea_on_exit;
            if any {
                emitted = true;
            }
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
        emitted |= generate_message_variants(b, child, names, type_system);
    }
    emitted
}

pub fn generate_update(
    b: &mut CodeBuilder,
    root: &Widget,
    names: &HashMap<WidgetId, String>,
    current_view_id: Uuid,
    view_refs: &[ViewRefInfo],
    view_names: &HashMap<Uuid, String>,
    flows: &[AppFlow],
    all_flows: &[&AppFlow],
    view_reference_index: Option<&ViewReferenceIndex>,
    is_main: bool,
    emits_bubbled_navigation_message: bool,
    bubbled_navigation_subviews: &HashSet<Uuid>,
    sub_view_intercepts: &HashMap<Uuid, Vec<CrossViewIntercept>>,
    callable_method_names: &HashMap<Uuid, String>,
) {
    let all_names: HashMap<(Uuid, WidgetId), String> = names
        .iter()
        .map(|(id, name)| ((current_view_id, *id), name.clone()))
        .collect();

    // Map from sub-view UUID -> field name in this view's struct (for cross-view state writes).
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
    let view_reference_selection_index =
        build_view_reference_selection_index(current_view_id, view_refs);

    b.line("pub fn update(&mut self, message: Message) -> Task<Message> {");
    b.increase_indent();
    b.line("match message {");
    b.increase_indent();

    generate_match_arms(
        b,
        root,
        names,
        current_view_id,
        view_names,
        &all_names,
        &view_field_names,
        all_flows,
        view_reference_index,
        &view_reference_selection_index,
        is_main,
        callable_method_names,
    );

    // Timer / KeyCombo flow match arms
    let mut has_sub_variants = false;
    for flow in flows {
        if !flow.enabled {
            continue;
        }
        match &flow.trigger {
            FlowTrigger::Timer { .. } | FlowTrigger::KeyCombo { .. } => {
                has_sub_variants = true;
                b.line(&format!("Message::{} => {{", sub_variant_name(&flow.name)));
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
                        callable_method_names,
                    ),
                    Err(diags) => emit_semantic_diagnostics_as_comments(b, &diags),
                }
                b.decrease_indent();
                b.line("}");
            }
            _ => {}
        }
    }

    if !view_refs.is_empty() {
        b.line("Message::ViewMessages(view_msg) => match view_msg {");
        b.increase_indent();
        for vr in view_refs {
            let pascal = vr.msg_variant();
            b.line(&format!("ViewMessages::{}(msg) => {{", pascal));
            b.increase_indent();
            // Intercept bubbled app-view navigation from sub-views (main only).
            if is_main && bubbled_navigation_subviews.contains(&vr.referenced_view_id) {
                b.line(&format!(
                    "if let {}::Message::NavigateTo(v) = &msg {{",
                    vr.module_name
                ));
                b.increase_indent();
                b.line("self.current_view = *v;");
                b.line("return Task::none();");
                b.decrease_indent();
                b.line("}");
            }
            // Intercept original trigger messages from sub-view to apply cross-parent state changes.
            if is_main {
                if let Some(intercepts) = sub_view_intercepts.get(&vr.referenced_view_id) {
                    for ci in intercepts {
                        b.line(&format!(
                            "if let {}::Message::{} = &msg {{",
                            vr.module_name, ci.variant_pattern
                        ));
                        b.increase_indent();
                        for assignment in &ci.assignments {
                            b.line(assignment);
                        }
                        b.decrease_indent();
                        b.line("}");
                    }
                }
            }
            b.line(&format!(
                "return self.{}.update(msg).map(|m| Message::ViewMessages(ViewMessages::{}(m)));",
                vr.field_name, pascal
            ));
            b.decrease_indent();
            b.line("},");
            // Arms for extra views in multi-view groups
            for (i, (ef, em, _)) in vr.extra_views.iter().enumerate() {
                let ep = to_pascal_case(ef);
                let extra_view_id = vr.extra_view_ids.get(i).copied();
                b.line(&format!("ViewMessages::{}(msg) => {{", ep));
                b.increase_indent();
                if extra_view_id.is_some_and(|view_id| {
                    is_main && bubbled_navigation_subviews.contains(&view_id)
                }) {
                    b.line(&format!("if let {}::Message::NavigateTo(v) = &msg {{", em));
                    b.increase_indent();
                    b.line("self.current_view = *v;");
                    b.line("return Task::none();");
                    b.decrease_indent();
                    b.line("}");
                }
                // Intercept original trigger messages from extra views.
                if is_main {
                    if let Some(eid) = extra_view_id {
                        if let Some(intercepts) = sub_view_intercepts.get(&eid) {
                            for ci in intercepts {
                                b.line(&format!(
                                    "if let {}::Message::{} = &msg {{",
                                    em, ci.variant_pattern
                                ));
                                b.increase_indent();
                                for assignment in &ci.assignments {
                                    b.line(assignment);
                                }
                                b.decrease_indent();
                                b.line("}");
                            }
                        }
                    }
                }
                b.line(&format!(
                    "return self.{}.update(msg).map(|m| Message::ViewMessages(ViewMessages::{}(m)));",
                    ef, ep
                ));
                b.decrease_indent();
                b.line("},");
            }
        }
        b.decrease_indent();
        b.line("},");
    }

    // Non-main views: handle NavigateTo only when app-view navigation bubbles to main.
    if !is_main && emits_bubbled_navigation_message {
        b.line("Message::NavigateTo(_) => {}");
    }

    let has_variants = has_any_message_variants(root);
    if !has_variants
        && !has_sub_variants
        && view_refs.is_empty()
        && !emits_bubbled_navigation_message
    {
        b.line("Message::Noop => {}");
    }

    b.decrease_indent();
    b.line("}");
    b.line("Task::none()");
    b.decrease_indent();
    b.line("}");
}

fn has_any_message_variants(widget: &Widget) -> bool {
    let props = &widget.properties;
    let self_has = match widget.widget_type {
        WidgetType::Button => {
            props.button_on_press_enabled
                || props.button_on_press_maybe_enabled
                || props.button_on_press_with_enabled
        }
        WidgetType::TextInput
        | WidgetType::Checkbox
        | WidgetType::Radio
        | WidgetType::Slider
        | WidgetType::VerticalSlider
        | WidgetType::Toggler
        | WidgetType::GenericOverlay
        | WidgetType::DatePicker
        | WidgetType::PickList
        | WidgetType::ComboBox
        | WidgetType::Markdown => true,
        WidgetType::MouseArea => {
            props.mousearea_on_press
                || props.mousearea_on_release
                || props.mousearea_on_double_click
                || props.mousearea_on_right_press
                || props.mousearea_on_right_release
                || props.mousearea_on_middle_press
                || props.mousearea_on_middle_release
                || props.mousearea_on_scroll
                || props.mousearea_on_enter
                || props.mousearea_on_move
                || props.mousearea_on_exit
        }
        _ => false,
    };
    if self_has {
        return true;
    }
    widget.children.iter().any(has_any_message_variants)
}

fn generate_match_arms(
    b: &mut CodeBuilder,
    widget: &Widget,
    names: &HashMap<WidgetId, String>,
    current_view_id: Uuid,
    view_names: &HashMap<Uuid, String>,
    all_names: &HashMap<(Uuid, WidgetId), String>,
    view_field_names: &HashMap<Uuid, String>,
    all_flows: &[&AppFlow],
    view_reference_index: Option<&ViewReferenceIndex>,
    view_reference_selection_index: &ViewReferenceSelectionIndex,
    is_main: bool,
    callable_method_names: &HashMap<Uuid, String>,
) {
    let name = names
        .get(&widget.id)
        .unwrap_or(&"widget".to_string())
        .clone();
    let props = &widget.properties;

    let has_custom_name = !props.widget_name.trim().is_empty();
    let v = to_pascal_case(&handle_whitespace(&name));
    let sname = to_snake_case(&name);
    let known_view_ids: HashSet<Uuid> = view_names
        .keys()
        .copied()
        .chain(std::iter::once(current_view_id))
        .collect();
    match widget.widget_type {
        WidgetType::Button => {
            if props.button_on_press_enabled
                || props.button_on_press_maybe_enabled
                || props.button_on_press_with_enabled
            {
                let arm = if has_custom_name {
                    v.clone()
                } else {
                    format!("{}Pressed", v)
                };
                b.line(&format!("Message::{} => {{", arm));
                b.increase_indent();
                let lowered = find_flow_graphs(
                    all_flows,
                    current_view_id,
                    widget.id,
                    "on_press",
                    Some(&known_view_ids),
                    view_reference_index,
                );
                if !emit_lowered_action_graphs(
                    b,
                    &lowered,
                    current_view_id,
                    view_names,
                    all_names,
                    view_field_names,
                    view_reference_selection_index,
                    is_main,
                    callable_method_names,
                ) {
                    b.line(&format!("// {} pressed", name));
                }
                b.decrease_indent();
                b.line("}");
            }
        }
        WidgetType::TextInput => {
            let arm = if has_custom_name {
                format!("{}(value)", v)
            } else {
                format!("{}OnInput(value)", v)
            };
            b.line(&format!("Message::{} => {{", arm));
            b.increase_indent();
            let lowered = find_flow_graphs(
                all_flows,
                current_view_id,
                widget.id,
                "on_input",
                Some(&known_view_ids),
                view_reference_index,
            );
            if !emit_lowered_action_graphs(
                b,
                &lowered,
                current_view_id,
                view_names,
                all_names,
                view_field_names,
                view_reference_selection_index,
                is_main,
                callable_method_names,
            ) {
                b.line(&format!("self.{}_value = value;", sname));
            }
            b.decrease_indent();
            b.line("}");

            if props.text_input_on_submit {
                b.line(&format!("Message::{}Submitted => {{", v));
                b.increase_indent();
                let lowered = find_flow_graphs(
                    all_flows,
                    current_view_id,
                    widget.id,
                    "on_submit",
                    Some(&known_view_ids),
                    view_reference_index,
                );
                if !emit_lowered_action_graphs(
                    b,
                    &lowered,
                    current_view_id,
                    view_names,
                    all_names,
                    view_field_names,
                    view_reference_selection_index,
                    is_main,
                    callable_method_names,
                ) {
                    b.line("// Handle text input submission (Enter key pressed)");
                    b.line(&format!("// Current value: self.{}_value", sname));
                }
                b.decrease_indent();
                b.line("}");
            }

            if props.text_input_on_paste {
                b.line(&format!("Message::{}Pasted(pasted_text) => {{", v));
                b.increase_indent();
                let lowered = find_flow_graphs(
                    all_flows,
                    current_view_id,
                    widget.id,
                    "on_paste",
                    Some(&known_view_ids),
                    view_reference_index,
                );
                if !emit_lowered_action_graphs(
                    b,
                    &lowered,
                    current_view_id,
                    view_names,
                    all_names,
                    view_field_names,
                    view_reference_selection_index,
                    is_main,
                    callable_method_names,
                ) {
                    b.line("// Handle text being pasted");
                    b.line("// pasted_text contains the pasted string");
                    b.line("// Note: on_input will also fire with the new combined value");
                    b.line(&format!("self.{}_value = pasted_text;", sname));
                }
                b.decrease_indent();
                b.line("}");
            }
        }
        WidgetType::Checkbox => {
            let arm = if has_custom_name {
                format!("{}(checked)", v)
            } else {
                format!("{}Toggled(checked)", v)
            };
            b.line(&format!("Message::{} => {{", arm));
            b.increase_indent();
            let lowered = find_flow_graphs(
                all_flows,
                current_view_id,
                widget.id,
                "on_toggle",
                Some(&known_view_ids),
                view_reference_index,
            );
            if !emit_lowered_action_graphs(
                b,
                &lowered,
                current_view_id,
                view_names,
                all_names,
                view_field_names,
                view_reference_selection_index,
                is_main,
                callable_method_names,
            ) {
                b.line(&format!("self.{}_checked = checked;", sname));
            }
            b.decrease_indent();
            b.line("}");
        }
        WidgetType::Radio => {
            let arm = if has_custom_name {
                format!("{}(index)", v)
            } else {
                format!("{}Selected(index)", v)
            };
            b.line(&format!("Message::{} => {{", arm));
            b.increase_indent();
            let lowered = find_flow_graphs(
                all_flows,
                current_view_id,
                widget.id,
                "on_select",
                Some(&known_view_ids),
                view_reference_index,
            );
            if !emit_lowered_action_graphs(
                b,
                &lowered,
                current_view_id,
                view_names,
                all_names,
                view_field_names,
                view_reference_selection_index,
                is_main,
                callable_method_names,
            ) {
                b.line(&format!("self.{}_selected = index;", sname));
            }
            b.decrease_indent();
            b.line("}");
        }
        WidgetType::Slider | WidgetType::VerticalSlider => {
            let arm = if has_custom_name {
                format!("{}(value)", v)
            } else {
                format!("{}Changed(value)", v)
            };
            b.line(&format!("Message::{} => {{", arm));
            b.increase_indent();
            let lowered = find_flow_graphs(
                all_flows,
                current_view_id,
                widget.id,
                "on_change",
                Some(&known_view_ids),
                view_reference_index,
            );
            if !emit_lowered_action_graphs(
                b,
                &lowered,
                current_view_id,
                view_names,
                all_names,
                view_field_names,
                view_reference_selection_index,
                is_main,
                callable_method_names,
            ) {
                b.line(&format!("self.{}_value = value;", sname));
            }
            b.decrease_indent();
            b.line("}");
        }
        WidgetType::Toggler => {
            let arm = if has_custom_name {
                format!("{}(active)", v)
            } else {
                format!("{}Toggled(active)", v)
            };
            b.line(&format!("Message::{} => {{", arm));
            b.increase_indent();
            let lowered = find_flow_graphs(
                all_flows,
                current_view_id,
                widget.id,
                "on_toggle",
                Some(&known_view_ids),
                view_reference_index,
            );
            if !emit_lowered_action_graphs(
                b,
                &lowered,
                current_view_id,
                view_names,
                all_names,
                view_field_names,
                view_reference_selection_index,
                is_main,
                callable_method_names,
            ) {
                b.line(&format!("self.{}_active = active;", sname));
            }
            b.decrease_indent();
            b.line("}");
        }
        WidgetType::GenericOverlay => {
            let arm = if has_custom_name {
                format!("{}(is_open)", v)
            } else {
                format!("{}Toggled(is_open)", v)
            };
            b.line(&format!("Message::{} => {{", arm));
            b.increase_indent();
            b.line(&format!("self.{}_open = is_open;", sname));
            let lowered = find_flow_graphs(
                all_flows,
                current_view_id,
                widget.id,
                "on_toggle",
                Some(&known_view_ids),
                view_reference_index,
            );
            let _ = emit_lowered_action_graphs(
                b,
                &lowered,
                current_view_id,
                view_names,
                all_names,
                view_field_names,
                view_reference_selection_index,
                is_main,
                callable_method_names,
            );
            b.decrease_indent();
            b.line("}");
        }
        WidgetType::DatePicker => {
            b.line(&format!("Message::{}OpenRequested => {{", v));
            b.increase_indent();
            b.line(&format!("self.{}_open = true;", sname));
            b.decrease_indent();
            b.line("}");

            b.line(&format!("Message::{}Changed(selection) => {{", v));
            b.increase_indent();
            b.line(&format!("self.{}_selection = selection;", sname));
            b.decrease_indent();
            b.line("}");

            b.line(&format!(
                "Message::{}ChangedWithTime(selection, time) => {{",
                v
            ));
            b.increase_indent();
            b.line(&format!("self.{}_selection = selection;", sname));
            b.line(&format!("self.{}_time = time;", sname));
            b.decrease_indent();
            b.line("}");

            b.line(&format!("Message::{}Closed => {{", v));
            b.increase_indent();
            b.line(&format!("self.{}_open = false;", sname));
            b.decrease_indent();
            b.line("}");
        }
        WidgetType::PickList => {
            let arm = if has_custom_name {
                format!("{}(value)", v)
            } else {
                format!("{}Selected(value)", v)
            };
            b.line(&format!("Message::{} => {{", arm));
            b.increase_indent();
            let lowered = find_flow_graphs(
                all_flows,
                current_view_id,
                widget.id,
                "on_select",
                Some(&known_view_ids),
                view_reference_index,
            );
            if !emit_lowered_action_graphs(
                b,
                &lowered,
                current_view_id,
                view_names,
                all_names,
                view_field_names,
                view_reference_selection_index,
                is_main,
                callable_method_names,
            ) {
                b.line(&format!("self.{}_selected = Some(value);", sname));
            }
            b.decrease_indent();
            b.line("}");
        }
        WidgetType::ComboBox => {
            b.line(&format!(
                "Message::{}Selected(value) => {{",
                to_pascal_case(&name)
            ));
            b.increase_indent();
            b.line(&format!("println!(\"{} selected: {{:?}}\", value);", name));
            b.line(&format!("self.{}_value = value;", to_snake_case(&name)));
            b.decrease_indent();
            b.line("}");

            if props.combobox_use_on_input {
                b.line(&format!(
                    "Message::{}OnInput(text) => {{",
                    to_pascal_case(&name)
                ));
                b.increase_indent();
                b.line(&format!("println!(\"{} input text: {{}}\", text);", name));
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
                b.line(&format!("Message::{}OnOpen => {{", to_pascal_case(&name)));
                b.increase_indent();
                b.line(&format!("println!(\"{} opened!\");", name));
                b.line("// Refresh data, log analytics, etc.");
                b.decrease_indent();
                b.line("}");
            }

            if props.combobox_use_on_close {
                b.line(&format!("Message::{}OnClose => {{", to_pascal_case(&name)));
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
                b.line(&format!("Message::{}Pressed => {{", to_pascal_case(&name)));
                b.increase_indent();
                b.line("// Handle left mouse button press");
                b.decrease_indent();
                b.line("}");
            }
            if props.mousearea_on_release {
                b.line(&format!("Message::{}Released => {{", to_pascal_case(&name)));
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
        generate_match_arms(
            b,
            child,
            names,
            current_view_id,
            view_names,
            all_names,
            view_field_names,
            all_flows,
            view_reference_index,
            view_reference_selection_index,
            is_main,
            callable_method_names,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::Point;

    use crate::action_system::flow::WidgetEventRow;
    use crate::action_system::graph::{ActionEdge, ActionGraph, ActionNodeData};
    use crate::action_system::node_kinds::{ActionValue, NavigateTarget, ValueSource};
    use crate::action_system::semantic::{
        SemanticValidationContext, validate_and_lower_flow_graph,
    };
    use crate::action_system::state_ref::{ActionValueType, StateFieldRef, StateRefSource};

    fn custom_target(view_id: Uuid, field_id: Uuid, field_name: &str) -> StateFieldRef {
        StateFieldRef {
            view_id,
            source: StateRefSource::Custom {
                field_id,
                field_name: field_name.to_string(),
            },
            field_type: ActionValueType::String,
            display_name: field_name.to_string(),
        }
    }

    fn widget_target(
        view_id: Uuid,
        widget_id: WidgetId,
        field_suffix: &str,
        field_type: ActionValueType,
    ) -> StateFieldRef {
        StateFieldRef {
            view_id,
            source: StateRefSource::Widget {
                widget_id,
                field_suffix: field_suffix.to_string(),
            },
            field_type,
            display_name: field_suffix.to_string(),
        }
    }

    fn flow_in(node: &ActionNodeData) -> u64 {
        node.cached_ports_in
            .iter()
            .find(|p| p.label == "flow_in")
            .expect("flow_in")
            .id
            .0
    }

    fn trigger_flow_out(flow: &AppFlow) -> u64 {
        flow.graph
            .nodes
            .iter()
            .find(|n| n.is_trigger())
            .and_then(|n| n.cached_ports_out.iter().find(|p| p.label == "flow_out"))
            .expect("trigger flow_out")
            .id
            .0
    }

    fn button_widget(button_id: WidgetId) -> Widget {
        let mut button = Widget::new(WidgetType::Button, button_id);
        button.properties.button_on_press_enabled = true;
        button
    }

    #[test]
    fn find_flow_graphs_skips_disabled_flows() {
        let view_id = Uuid::new_v4();
        let widget_id = WidgetId(12);
        let enabled_row = WidgetEventRow {
            id: Uuid::new_v4(),
            event_type: "on_press".to_string(),
            target: Some((view_id, widget_id.0)),
        };
        let disabled_row = WidgetEventRow {
            id: Uuid::new_v4(),
            event_type: "on_press".to_string(),
            target: Some((view_id, widget_id.0)),
        };

        let mut enabled = AppFlow::new(
            "enabled".to_string(),
            FlowTrigger::WidgetEvent {
                rows: vec![enabled_row.clone()],
            },
        );
        enabled.enabled = true;
        enabled.sync_trigger_topology();

        let mut disabled = AppFlow::new(
            "disabled".to_string(),
            FlowTrigger::WidgetEvent {
                rows: vec![disabled_row],
            },
        );
        disabled.enabled = false;
        disabled.sync_trigger_topology();

        let all_flows: Vec<&AppFlow> = vec![&enabled, &disabled];
        let matches = find_flow_graphs(&all_flows, view_id, widget_id, "on_press", None, None);
        let expected_slot =
            crate::action_system::graph::ActionNodeData::widget_event_row_slot(&[enabled_row], 0);
        assert_eq!(matches.flows.len(), 1);
        assert_eq!(matches.flows[0].trigger_slot, expected_slot);
    }

    #[test]
    fn find_flow_graphs_reports_invalid_flows_in_diagnostics() {
        let view_id = Uuid::new_v4();
        let widget_id = WidgetId(7);
        let row = WidgetEventRow {
            id: Uuid::new_v4(),
            event_type: "on_press".to_string(),
            target: Some((view_id, widget_id.0)),
        };
        let mut flow = AppFlow::new(
            "broken".to_string(),
            FlowTrigger::WidgetEvent { rows: vec![row] },
        );
        flow.sync_trigger_topology();
        let set_state = ActionNodeData::new(
            2,
            ActionNodeKind::StateMutation {
                assignments: vec![crate::action_system::node_kinds::StateAssignment {
                    target: None,
                    value_source: ValueSource::Literal(ActionValue::String(String::new())),
                }],
            },
            Point::new(280.0, 120.0),
        );
        let to_port = set_state
            .cached_ports_in
            .iter()
            .find(|p| p.label == "flow_in")
            .expect("flow_in")
            .id
            .0;
        flow.graph.nodes.push(set_state);
        flow.graph.z_order.push(2);
        flow.graph.next_id = 3;
        flow.graph.edges.push(ActionEdge {
            from_node: 1,
            from_port: 1001,
            to_node: 2,
            to_port,
        });

        let all_flows: Vec<&AppFlow> = vec![&flow];
        let lowered = find_flow_graphs(&all_flows, view_id, widget_id, "on_press", None, None);
        assert!(lowered.flows.is_empty());
        assert!(!lowered.diagnostics.is_empty());
    }

    #[test]
    fn view_requires_bubbled_navigation_message_detects_app_view_navigation() {
        let source_view_id = Uuid::new_v4();
        let destination_view_id = Uuid::new_v4();
        let button_id = WidgetId(12);
        let row = WidgetEventRow {
            id: Uuid::new_v4(),
            event_type: "on_press".to_string(),
            target: Some((source_view_id, button_id.0)),
        };

        let mut flow = AppFlow::new(
            "app_nav".to_string(),
            FlowTrigger::WidgetEvent {
                rows: vec![row.clone()],
            },
        );
        let navigate = ActionNodeData::new(
            2,
            ActionNodeKind::NavigateToView {
                targets: vec![Some(NavigateTarget::AppView {
                    view_id: destination_view_id,
                })],
            },
            Point::new(260.0, 120.0),
        );
        flow.graph.nodes.push(navigate.clone());
        flow.graph.z_order.push(2);
        flow.graph.next_id = 3;
        assert!(
            flow.graph
                .connect_ports(1, trigger_flow_out(&flow), 2, flow_in(&navigate),)
        );

        let all_flows: Vec<&AppFlow> = vec![&flow];
        let known_view_ids = HashSet::from([source_view_id, destination_view_id]);
        assert!(view_requires_bubbled_navigation_message(
            source_view_id,
            &all_flows,
            &known_view_ids,
            None,
        ));
    }

    #[test]
    fn view_requires_bubbled_navigation_message_ignores_parent_view_reference_navigation() {
        let parent_view_id = Uuid::new_v4();
        let source_view_id = Uuid::new_v4();
        let destination_view_id = Uuid::new_v4();
        let button_id = WidgetId(12);
        let row = WidgetEventRow {
            id: Uuid::new_v4(),
            event_type: "on_press".to_string(),
            target: Some((source_view_id, button_id.0)),
        };

        let mut flow = AppFlow::new(
            "view_ref_nav".to_string(),
            FlowTrigger::WidgetEvent {
                rows: vec![row.clone()],
            },
        );
        let navigate = ActionNodeData::new(
            2,
            ActionNodeKind::NavigateToView {
                targets: vec![Some(NavigateTarget::ViewReference {
                    owner_view_id: parent_view_id,
                    widget_id: WidgetId(77),
                    target_view_id: destination_view_id,
                })],
            },
            Point::new(260.0, 120.0),
        );
        flow.graph.nodes.push(navigate.clone());
        flow.graph.z_order.push(2);
        flow.graph.next_id = 3;
        assert!(
            flow.graph
                .connect_ports(1, trigger_flow_out(&flow), 2, flow_in(&navigate),)
        );

        let all_flows: Vec<&AppFlow> = vec![&flow];
        let known_view_ids = HashSet::from([parent_view_id, source_view_id, destination_view_id]);
        assert!(!view_requires_bubbled_navigation_message(
            source_view_id,
            &all_flows,
            &known_view_ids,
            None,
        ));
    }

    #[test]
    fn generate_message_enum_omits_navigate_to_when_view_only_changes_parent_view_reference() {
        let button_id = WidgetId(12);
        let root = button_widget(button_id);
        let names = HashMap::from([(button_id, "button".to_string())]);
        let mut b = CodeBuilder::new();

        generate_message_enum(
            &mut b,
            &root,
            &names,
            &TypeSystem::default(),
            &[],
            &[],
            false,
            false,
        );

        let code = b.build();
        assert!(code.contains("ButtonPressed,"));
        assert!(!code.contains("NavigateTo(View),"));
    }

    #[test]
    fn collect_reachable_state_mutations_includes_all_fanout_successors_in_order() {
        let view_id = Uuid::new_v4();
        let field_id = Uuid::new_v4();
        let target = custom_target(view_id, field_id, "result");

        let mut graph = ActionGraph::new_with_trigger("on_press");
        let first = ActionNodeData::new(
            2,
            ActionNodeKind::StateMutation {
                assignments: vec![crate::action_system::node_kinds::StateAssignment {
                    target: Some(target.clone()),
                    value_source: ValueSource::Literal(ActionValue::String("first".to_string())),
                }],
            },
            Point::new(260.0, 120.0),
        );
        let second = ActionNodeData::new(
            3,
            ActionNodeKind::StateMutation {
                assignments: vec![crate::action_system::node_kinds::StateAssignment {
                    target: Some(target),
                    value_source: ValueSource::Literal(ActionValue::String("second".to_string())),
                }],
            },
            Point::new(380.0, 120.0),
        );
        let first_in = flow_in(&first);
        let second_in = flow_in(&second);

        graph.nodes.extend([first, second]);
        graph.z_order.extend([2, 3]);
        graph.next_id = 4;

        let trigger_flow_out = graph
            .nodes
            .iter()
            .find(|n| n.is_trigger())
            .and_then(|n| n.cached_ports_out.iter().find(|p| p.label == "flow_out"))
            .expect("trigger flow_out")
            .id
            .0;
        assert!(graph.connect_ports(1, trigger_flow_out, 2, first_in));
        assert!(graph.connect_ports(1, trigger_flow_out, 3, second_in));

        let callable = HashSet::new();
        let known_views = HashSet::from([view_id]);
        let mut flow = AppFlow::new("flow".to_string(), FlowTrigger::Callable);
        flow.graph = graph;
        let context = SemanticValidationContext {
            callable_flow_ids: &callable,
            known_view_ids: Some(&known_views),
        };
        let lowered = validate_and_lower_flow_graph(&flow, &context).expect("lowered graph");

        let assignments = collect_reachable_state_mutations(
            &lowered,
            lowered.trigger_node_id(),
            0,
            view_id,
            &HashMap::new(),
            &ViewReferenceSelectionIndex::new(),
            0,
        );
        assert_eq!(
            assignments,
            vec![
                "self.result = String::from(\"first\");".to_string(),
                "self.result = String::from(\"second\");".to_string(),
            ]
        );
    }

    #[test]
    fn collect_reachable_state_mutations_includes_parent_view_reference_navigation() {
        let parent_view_id = Uuid::new_v4();
        let primary_view_id = Uuid::new_v4();
        let secondary_view_id = Uuid::new_v4();
        let view_ref_widget_id = WidgetId(77);
        let mut flow = AppFlow::new("flow".to_string(), FlowTrigger::Callable);
        let navigate = ActionNodeData::new(
            2,
            ActionNodeKind::NavigateToView {
                targets: vec![Some(NavigateTarget::ViewReference {
                    owner_view_id: parent_view_id,
                    widget_id: view_ref_widget_id,
                    target_view_id: secondary_view_id,
                })],
            },
            Point::new(260.0, 120.0),
        );
        flow.graph.nodes.push(navigate.clone());
        flow.graph.z_order.push(2);
        flow.graph.next_id = 3;
        assert!(
            flow.graph
                .connect_ports(1, trigger_flow_out(&flow), 2, flow_in(&navigate),)
        );

        let callable = HashSet::new();
        let known_views = HashSet::from([parent_view_id, primary_view_id, secondary_view_id]);
        let context = SemanticValidationContext {
            callable_flow_ids: &callable,
            known_view_ids: Some(&known_views),
        };
        let lowered = validate_and_lower_flow_graph(&flow, &context).expect("lowered graph");

        let mut selection_index = ViewReferenceSelectionIndex::new();
        selection_index.insert(
            (parent_view_id, view_ref_widget_id),
            ViewReferenceSelectionCodegen {
                field_name: "main_pane".to_string(),
                selection_type: "MainPaneSelection".to_string(),
                variants_by_view: HashMap::from([
                    (primary_view_id, "Primary".to_string()),
                    (secondary_view_id, "Secondary".to_string()),
                ]),
            },
        );
        let assignments = collect_reachable_state_mutations(
            &lowered,
            lowered.trigger_node_id(),
            0,
            parent_view_id,
            &HashMap::new(),
            &selection_index,
            0,
        );
        assert_eq!(
            assignments,
            vec!["self.main_pane_selection = MainPaneSelection::Secondary;".to_string()]
        );
    }

    #[test]
    fn collect_reachable_state_mutations_uses_parent_widget_names_for_widget_targets() {
        let parent_view_id = Uuid::new_v4();
        let overlay_id = WidgetId(4);
        let target = widget_target(parent_view_id, overlay_id, "_open", ActionValueType::Bool);
        let mut flow = AppFlow::new("flow".to_string(), FlowTrigger::Callable);
        let state_mutation = ActionNodeData::new(
            2,
            ActionNodeKind::StateMutation {
                assignments: vec![crate::action_system::node_kinds::StateAssignment {
                    target: Some(target),
                    value_source: ValueSource::Literal(ActionValue::Bool(false)),
                }],
            },
            Point::new(260.0, 120.0),
        );
        flow.graph.nodes.push(state_mutation.clone());
        flow.graph.z_order.push(2);
        flow.graph.next_id = 3;
        assert!(
            flow.graph
                .connect_ports(1, trigger_flow_out(&flow), 2, flow_in(&state_mutation),)
        );

        let callable = HashSet::new();
        let known_views = HashSet::from([parent_view_id]);
        let context = SemanticValidationContext {
            callable_flow_ids: &callable,
            known_view_ids: Some(&known_views),
        };
        let lowered = validate_and_lower_flow_graph(&flow, &context).expect("lowered graph");

        let assignments = collect_reachable_state_mutations(
            &lowered,
            lowered.trigger_node_id(),
            0,
            parent_view_id,
            &HashMap::from([(overlay_id, "genericoverlay".to_string())]),
            &ViewReferenceSelectionIndex::new(),
            0,
        );

        assert_eq!(
            assignments,
            vec!["self.genericoverlay_open = false;".to_string()]
        );
    }
}

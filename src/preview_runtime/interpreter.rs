use std::collections::{BTreeMap, HashMap, HashSet};
use uuid::Uuid;

use crate::action_system::graph::{ActionNodeData, ActionNodeId};
use crate::action_system::node_kinds::{ActionNodeKind, ActionValue, NavigateTarget};
use crate::action_system::semantic::{
    LoweredActionGraph, LoweredExpression, SemanticValidationContext, build_view_reference_index,
    callable_flow_ids, format_diagnostic, lower_widget_event_flows_with_view_refs,
    validate_and_lower_flow_graph_with_view_refs,
};
use crate::action_system::state_ref::{
    ActionValueType, StateFieldRef, StateRefSource, date_picker_open_state_key,
    generic_overlay_open_state_key, view_reference_selection_state_key, view_selection_state_key,
};
use crate::data_structures::types::types::{AppView, WidgetId, WidgetType};

/// Executes all flows triggered by a widget event in the live preview.
///
/// Returns `Some(view_id)` if a `NavigateToView` node fired in any flow, otherwise `None`.
pub fn execute_event(
    flows: &[crate::action_system::flow::AppFlow],
    all_views: &mut BTreeMap<Uuid, AppView>,
    origin_view_id: Uuid,
    widget_id: WidgetId,
    event_name: &str,
    payload: HashMap<String, ActionValue>,
) -> Option<Uuid> {
    let flow_refs: Vec<&crate::action_system::flow::AppFlow> = flows.iter().collect();
    let known_view_ids: HashSet<Uuid> = all_views.keys().copied().collect();
    let view_reference_index = build_view_reference_index(all_views);
    let callable_ids = callable_flow_ids(&flow_refs);
    let validation_context = SemanticValidationContext {
        callable_flow_ids: &callable_ids,
        known_view_ids: Some(&known_view_ids),
    };
    let mut callable_graphs: HashMap<Uuid, LoweredActionGraph> = HashMap::new();
    for flow in flow_refs.iter().copied().filter(|f| {
        f.enabled && matches!(f.trigger, crate::action_system::flow::FlowTrigger::Callable)
    }) {
        match validate_and_lower_flow_graph_with_view_refs(
            flow,
            &validation_context,
            Some(&view_reference_index),
        ) {
            Ok(lowered) => {
                callable_graphs.insert(flow.id, lowered);
            }
            Err(diags) => {
                for diag in diags {
                    eprintln!(
                        "Callable flow skipped in preview: {}",
                        format_diagnostic(&diag)
                    );
                }
            }
        }
    }
    let lowered = lower_widget_event_flows_with_view_refs(
        &flow_refs,
        origin_view_id,
        widget_id,
        event_name,
        Some(&known_view_ids),
        Some(&view_reference_index),
    );
    for diag in &lowered.diagnostics {
        eprintln!(
            "Action flow skipped in preview: {}",
            format_diagnostic(diag)
        );
    }

    let mut nav_result: Option<Uuid> = None;
    for lowered_flow in lowered.flows {
        let mut ctx = InterpContext {
            variables: payload.clone(),
            nav_result: None,
        };
        walk_flow(
            &lowered_flow.graph,
            lowered_flow.graph.trigger_node_id(),
            lowered_flow.trigger_slot,
            0,
            &callable_graphs,
            all_views,
            &mut ctx,
        );
        if ctx.nav_result.is_some() {
            nav_result = ctx.nav_result;
        }
    }
    nav_result
}

// ── Internal context ─────────────────────────────────────────────────────────

struct InterpContext {
    /// Values collected from Trigger payload.
    variables: HashMap<String, ActionValue>,
    /// Set by NavigateToView — propagates back to the caller.
    nav_result: Option<Uuid>,
}

// ── Graph traversal ───────────────────────────────────────────────────────────

fn walk_flow(
    graph: &LoweredActionGraph,
    from_node: ActionNodeId,
    output_slot: usize,
    depth: usize,
    callable_graphs: &HashMap<Uuid, LoweredActionGraph>,
    all_views: &mut BTreeMap<Uuid, AppView>,
    ctx: &mut InterpContext,
) {
    if depth > 100 {
        return;
    }
    if ctx.nav_result.is_some() {
        return; // NavigateToView already fired — stop execution
    }

    let successors = graph.flow_successors_with_input_slots(from_node, output_slot);
    if successors.is_empty() {
        return;
    }

    for (next_id, input_slot) in successors {
        if ctx.nav_result.is_some() {
            break;
        }
        let Some(next_node) = graph.node(next_id) else {
            continue;
        };
        execute_node(
            graph,
            next_node,
            input_slot,
            depth,
            callable_graphs,
            all_views,
            ctx,
        );
    }
}

fn execute_node(
    graph: &LoweredActionGraph,
    node: &ActionNodeData,
    input_slot: usize,
    depth: usize,
    callable_graphs: &HashMap<Uuid, LoweredActionGraph>,
    all_views: &mut BTreeMap<Uuid, AppView>,
    ctx: &mut InterpContext,
) {
    match &node.kind {
        ActionNodeKind::StateMutation { assignments } => {
            for (idx, assignment) in assignments.iter().enumerate() {
                if let Some(target) = &assignment.target {
                    let input_label = format!("value_{idx}");
                    let value = graph
                        .expression_for_input_label(node.id, &input_label)
                        .and_then(|expr| eval_lowered_expression(expr, all_views, ctx));
                    if let Some(v) = value {
                        apply_state_write(target, v, all_views);
                    }
                }
            }
            walk_flow(
                graph,
                node.id,
                0,
                depth + 1,
                callable_graphs,
                all_views,
                ctx,
            );
        }

        ActionNodeKind::NavigateToView { .. } => {
            if let Some(target) = graph.navigation_target_for_input_slot(node.id, input_slot) {
                match target {
                    NavigateTarget::AppView { view_id } => {
                        ctx.nav_result = Some(*view_id);
                    }
                    NavigateTarget::ViewReference {
                        owner_view_id,
                        widget_id,
                        target_view_id,
                    } => {
                        apply_view_reference_navigation(
                            all_views,
                            *owner_view_id,
                            *widget_id,
                            *target_view_id,
                        );
                    }
                }
            }
            // Continue execution from the matching output slot
            walk_flow(
                graph,
                node.id,
                input_slot,
                depth + 1,
                callable_graphs,
                all_views,
                ctx,
            );
        }

        ActionNodeKind::Conditional => {
            let condition = graph
                .expression_for_input_label(node.id, "condition")
                .and_then(|expr| eval_lowered_expression(expr, all_views, ctx))
                .map(|v| as_bool(&v))
                .unwrap_or(false);

            if condition {
                walk_flow(
                    graph,
                    node.id,
                    0,
                    depth + 1,
                    callable_graphs,
                    all_views,
                    ctx,
                );
            } else {
                walk_flow(
                    graph,
                    node.id,
                    1,
                    depth + 1,
                    callable_graphs,
                    all_views,
                    ctx,
                );
            }
        }

        ActionNodeKind::Match { arms, .. } => {
            let value = graph
                .expression_for_input_label(node.id, "value")
                .and_then(|expr| eval_lowered_expression(expr, all_views, ctx));
            let value_str = value.map(|v| match v {
                ActionValue::String(s) => s,
                ActionValue::Number(n) => n.to_string(),
                ActionValue::Bool(b) => b.to_string(),
                ActionValue::EnumVariant { variant, .. } => variant,
            });
            let matched_slot = if let Some(s) = &value_str {
                arms.iter().position(|arm| arm == s)
            } else {
                None
            };
            // Walk the matched arm, or the default arm (slot = arms.len())
            let slot = matched_slot.unwrap_or(arms.len());
            walk_flow(
                graph,
                node.id,
                slot,
                depth + 1,
                callable_graphs,
                all_views,
                ctx,
            );
        }

        ActionNodeKind::CallFlow { flow_id } => {
            if let Some(target_flow_id) = flow_id {
                if let Some(callable_graph) = callable_graphs.get(target_flow_id) {
                    walk_flow(
                        callable_graph,
                        callable_graph.trigger_node_id(),
                        0,
                        depth + 1,
                        callable_graphs,
                        all_views,
                        ctx,
                    );
                }
            }
            walk_flow(
                graph,
                node.id,
                0,
                depth + 1,
                callable_graphs,
                all_views,
                ctx,
            );
        }

        // Data-only and Trigger nodes — not executed in flow walk
        ActionNodeKind::StringLiteral { .. }
        | ActionNodeKind::NumberLiteral { .. }
        | ActionNodeKind::BoolLiteral { .. }
        | ActionNodeKind::EnumLiteral { .. }
        | ActionNodeKind::Compare { .. }
        | ActionNodeKind::LogicAnd
        | ActionNodeKind::LogicOr
        | ActionNodeKind::LogicNot
        | ActionNodeKind::Trigger { .. }
        | ActionNodeKind::Expression { .. }
        | ActionNodeKind::SetState { .. }
        | ActionNodeKind::UpdateState { .. }
        | ActionNodeKind::CallAction { .. }
        | ActionNodeKind::LegacyGetState { .. } => {}
    }
}

fn apply_view_reference_navigation(
    all_views: &mut BTreeMap<Uuid, AppView>,
    owner_view_id: Uuid,
    widget_id: WidgetId,
    target_view_id: Uuid,
) {
    let Some(owner_view) = all_views.get_mut(&owner_view_id) else {
        return;
    };
    let key = view_reference_selection_state_key(owner_view_id, widget_id);
    owner_view
        .custom_state_values
        .insert(key, ActionValue::String(target_view_id.to_string()));
}

// ── Data port resolution ──────────────────────────────────────────────────────

fn eval_lowered_expression(
    expr: &LoweredExpression,
    all_views: &BTreeMap<Uuid, AppView>,
    ctx: &InterpContext,
) -> Option<ActionValue> {
    match expr {
        LoweredExpression::Literal(v) => Some(v.clone()),
        LoweredExpression::StateField(src) => read_state(src, all_views),
        LoweredExpression::TriggerInput { name, .. } => ctx.variables.get(name).cloned(),
        LoweredExpression::Formula(formula) => eval_expression(formula, all_views, ctx),
        LoweredExpression::Compare { operator, lhs, rhs } => {
            let lhs_val = eval_lowered_expression(lhs, all_views, ctx)?;
            let rhs_val = if operator.needs_rhs() {
                eval_lowered_expression(rhs.as_ref()?, all_views, ctx)?
            } else {
                ActionValue::Bool(false)
            };
            let result = match operator {
                crate::action_system::node_kinds::CompareOp::Eq => lhs_val == rhs_val,
                crate::action_system::node_kinds::CompareOp::NotEq => lhs_val != rhs_val,
                crate::action_system::node_kinds::CompareOp::Lt => {
                    compare_order(&lhs_val, &rhs_val).is_lt()
                }
                crate::action_system::node_kinds::CompareOp::Gt => {
                    compare_order(&lhs_val, &rhs_val).is_gt()
                }
                crate::action_system::node_kinds::CompareOp::LtEq => {
                    !compare_order(&lhs_val, &rhs_val).is_gt()
                }
                crate::action_system::node_kinds::CompareOp::GtEq => {
                    !compare_order(&lhs_val, &rhs_val).is_lt()
                }
                crate::action_system::node_kinds::CompareOp::Contains => {
                    match (&lhs_val, &rhs_val) {
                        (ActionValue::String(s), ActionValue::String(r)) => s.contains(r.as_str()),
                        _ => false,
                    }
                }
                crate::action_system::node_kinds::CompareOp::StartsWith => {
                    match (&lhs_val, &rhs_val) {
                        (ActionValue::String(s), ActionValue::String(r)) => {
                            s.starts_with(r.as_str())
                        }
                        _ => false,
                    }
                }
                crate::action_system::node_kinds::CompareOp::EndsWith => {
                    match (&lhs_val, &rhs_val) {
                        (ActionValue::String(s), ActionValue::String(r)) => s.ends_with(r.as_str()),
                        _ => false,
                    }
                }
                crate::action_system::node_kinds::CompareOp::IsEmpty => match &lhs_val {
                    ActionValue::String(s) => s.is_empty(),
                    _ => false,
                },
                crate::action_system::node_kinds::CompareOp::IsNotEmpty => match &lhs_val {
                    ActionValue::String(s) => !s.is_empty(),
                    _ => false,
                },
                crate::action_system::node_kinds::CompareOp::IsTrue => {
                    matches!(lhs_val, ActionValue::Bool(true))
                }
                crate::action_system::node_kinds::CompareOp::IsFalse => {
                    matches!(lhs_val, ActionValue::Bool(false))
                }
                crate::action_system::node_kinds::CompareOp::IsValidEmail => match &lhs_val {
                    ActionValue::String(s) => is_valid_email(s),
                    _ => false,
                },
            };
            Some(ActionValue::Bool(result))
        }
        LoweredExpression::LogicAnd { lhs, rhs } => {
            let lhs_val = eval_lowered_expression(lhs, all_views, ctx)?;
            let rhs_val = eval_lowered_expression(rhs, all_views, ctx)?;
            Some(ActionValue::Bool(as_bool(&lhs_val) && as_bool(&rhs_val)))
        }
        LoweredExpression::LogicOr { lhs, rhs } => {
            let lhs_val = eval_lowered_expression(lhs, all_views, ctx)?;
            let rhs_val = eval_lowered_expression(rhs, all_views, ctx)?;
            Some(ActionValue::Bool(as_bool(&lhs_val) || as_bool(&rhs_val)))
        }
        LoweredExpression::LogicNot { value } => {
            let value = eval_lowered_expression(value, all_views, ctx)?;
            Some(ActionValue::Bool(!as_bool(&value)))
        }
    }
}

// ── Expression evaluator ──────────────────────────────────────────────────────

/// Evaluates a simple formula string. Supports:
/// - `self.field_name` state variable references (looked up in ctx.variables)
/// - Number/bool/string literals
/// - Operators: `==`, `!=`, `>`, `<`, `>=`, `<=`, `&&`, `||`, `!`, `+`, `-`, `*`, `/`
/// - Parentheses
///
/// Returns `None` if the formula is empty or cannot be parsed.
fn eval_expression(
    formula: &str,
    _all_views: &BTreeMap<Uuid, AppView>,
    ctx: &InterpContext,
) -> Option<ActionValue> {
    let formula = formula.trim();
    if formula.is_empty() {
        return None;
    }

    // Substitute `self.field` references with their current values
    let mut expanded = formula.to_string();
    for (key, val) in &ctx.variables {
        let placeholder = format!("self.{}", key);
        let replacement = match val {
            ActionValue::Bool(b) => b.to_string(),
            ActionValue::Number(n) => n.to_string(),
            ActionValue::String(s) => format!("{:?}", s),
            ActionValue::EnumVariant { variant, .. } => format!("{:?}", variant),
        };
        expanded = expanded.replace(&placeholder, &replacement);
    }

    // Try to evaluate via a minimal expression parser
    eval_expr_str(expanded.trim())
}

/// Recursive descent parser/evaluator for simple boolean/numeric expressions.
fn eval_expr_str(input: &str) -> Option<ActionValue> {
    let input = input.trim();
    // Try OR (lowest precedence)
    if let Some(idx) = find_binary_op(input, "||") {
        let lhs = eval_expr_str(&input[..idx])?;
        let rhs = eval_expr_str(&input[idx + 2..])?;
        return Some(ActionValue::Bool(as_bool(&lhs) || as_bool(&rhs)));
    }
    if let Some(idx) = find_binary_op(input, "&&") {
        let lhs = eval_expr_str(&input[..idx])?;
        let rhs = eval_expr_str(&input[idx + 2..])?;
        return Some(ActionValue::Bool(as_bool(&lhs) && as_bool(&rhs)));
    }
    // Comparison operators
    for op in &["==", "!=", ">=", "<=", ">", "<"] {
        if let Some(idx) = find_binary_op(input, op) {
            let lhs = eval_expr_str(&input[..idx])?;
            let rhs = eval_expr_str(&input[idx + op.len()..])?;
            let result = match *op {
                "==" => lhs == rhs,
                "!=" => lhs != rhs,
                ">" => compare_order(&lhs, &rhs).is_gt(),
                "<" => compare_order(&lhs, &rhs).is_lt(),
                ">=" => !compare_order(&lhs, &rhs).is_lt(),
                "<=" => !compare_order(&lhs, &rhs).is_gt(),
                _ => false,
            };
            return Some(ActionValue::Bool(result));
        }
    }
    // Additive
    if let Some(idx) = find_binary_op(input, "+") {
        let lhs = eval_expr_str(&input[..idx])?;
        let rhs = eval_expr_str(&input[idx + 1..])?;
        return match (&lhs, &rhs) {
            (ActionValue::Number(a), ActionValue::Number(b)) => Some(ActionValue::Number(a + b)),
            (ActionValue::String(a), ActionValue::String(b)) => {
                Some(ActionValue::String(format!("{}{}", a, b)))
            }
            _ => None,
        };
    }
    if let Some(idx) = find_binary_op(input, "-") {
        let lhs = eval_expr_str(&input[..idx])?;
        let rhs = eval_expr_str(&input[idx + 1..])?;
        if let (ActionValue::Number(a), ActionValue::Number(b)) = (&lhs, &rhs) {
            return Some(ActionValue::Number(a - b));
        }
    }
    // Multiplicative
    if let Some(idx) = find_binary_op(input, "*") {
        let lhs = eval_expr_str(&input[..idx])?;
        let rhs = eval_expr_str(&input[idx + 1..])?;
        if let (ActionValue::Number(a), ActionValue::Number(b)) = (&lhs, &rhs) {
            return Some(ActionValue::Number(a * b));
        }
    }
    if let Some(idx) = find_binary_op(input, "/") {
        let lhs = eval_expr_str(&input[..idx])?;
        let rhs = eval_expr_str(&input[idx + 1..])?;
        if let (ActionValue::Number(a), ActionValue::Number(b)) = (&lhs, &rhs) {
            if *b != 0.0 {
                return Some(ActionValue::Number(a / b));
            }
        }
    }
    // Unary NOT
    if let Some(rest) = input.strip_prefix('!') {
        let val = eval_expr_str(rest.trim())?;
        return Some(ActionValue::Bool(!as_bool(&val)));
    }
    // Parentheses
    if input.starts_with('(') && input.ends_with(')') {
        return eval_expr_str(&input[1..input.len() - 1]);
    }
    // Literals
    if input == "true" {
        return Some(ActionValue::Bool(true));
    }
    if input == "false" {
        return Some(ActionValue::Bool(false));
    }
    if let Ok(n) = input.parse::<f64>() {
        return Some(ActionValue::Number(n));
    }
    if (input.starts_with('"') && input.ends_with('"'))
        || (input.starts_with('\'') && input.ends_with('\''))
    {
        return Some(ActionValue::String(input[1..input.len() - 1].to_string()));
    }
    None
}

/// Finds the rightmost occurrence of `op` in `input` that is not inside parentheses.
/// Used for left-to-right evaluation (lowest precedence first).
fn find_binary_op(input: &str, op: &str) -> Option<usize> {
    let mut depth = 0i32;
    let bytes = input.as_bytes();
    let op_len = op.len();
    // Scan right to left to get left-associative evaluation
    let mut i = input.len().saturating_sub(op_len);
    loop {
        // Track paren depth scanning right to left
        for j in (i..input.len()).rev() {
            match bytes[j] {
                b')' => depth += 1,
                b'(' => depth -= 1,
                _ => {}
            }
        }
        depth = 0;
        // Re-scan left to right to get proper depth at position i
        for j in 0..i {
            match bytes[j] {
                b'(' => depth += 1,
                b')' => depth -= 1,
                _ => {}
            }
        }
        if depth == 0 && input.get(i..i + op_len) == Some(op) {
            // Don't match ">" when looking for ">=" etc.
            let next_char = input.as_bytes().get(i + op_len).copied();
            let prev_char = if i > 0 {
                input.as_bytes().get(i - 1).copied()
            } else {
                None
            };
            // Avoid matching "==" inside "!=" when looking for "="
            let is_valid = match op {
                ">" => next_char != Some(b'='),
                "<" => next_char != Some(b'='),
                "!" => next_char != Some(b'='),
                "=" => {
                    prev_char != Some(b'!')
                        && prev_char != Some(b'=')
                        && prev_char != Some(b'>')
                        && prev_char != Some(b'<')
                        && next_char != Some(b'=')
                }
                "-" => i > 0, // don't match unary minus at start
                _ => true,
            };
            if is_valid {
                return Some(i);
            }
        }
        if i == 0 {
            break;
        }
        i -= 1;
    }
    None
}

fn compare_order(a: &ActionValue, b: &ActionValue) -> std::cmp::Ordering {
    match (a, b) {
        (ActionValue::Number(x), ActionValue::Number(y)) => {
            x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal)
        }
        (ActionValue::String(x), ActionValue::String(y)) => x.cmp(y),
        _ => std::cmp::Ordering::Equal,
    }
}

fn is_valid_email(raw: &str) -> bool {
    let candidate = raw.trim();
    if candidate.is_empty() || candidate.contains(' ') {
        return false;
    }
    let Some(at_idx) = candidate.find('@') else {
        return false;
    };
    if at_idx == 0 || at_idx + 1 >= candidate.len() {
        return false;
    }
    let local = &candidate[..at_idx];
    let domain = &candidate[at_idx + 1..];
    if local.is_empty() || domain.is_empty() {
        return false;
    }
    if !domain.contains('.') {
        return false;
    }
    if domain.starts_with('.') || domain.ends_with('.') {
        return false;
    }
    true
}

fn as_bool(v: &ActionValue) -> bool {
    match v {
        ActionValue::Bool(b) => *b,
        ActionValue::Number(n) => *n != 0.0,
        ActionValue::String(s) => !s.is_empty(),
        ActionValue::EnumVariant { variant, .. } => !variant.is_empty(),
    }
}

// ── State read/write helpers ──────────────────────────────────────────────────

fn read_state(source: &StateFieldRef, all_views: &BTreeMap<Uuid, AppView>) -> Option<ActionValue> {
    let view = all_views.get(&source.view_id)?;
    match &source.source {
        StateRefSource::Widget {
            widget_id,
            field_suffix,
        } => {
            let widget = view.hierarchy.get_widget_by_id(*widget_id)?;
            if field_suffix == "_open" {
                let (key, fallback) = match widget.widget_type {
                    WidgetType::GenericOverlay => (
                        generic_overlay_open_state_key(source.view_id, *widget_id),
                        widget.properties.generic_overlay_initially_open,
                    ),
                    WidgetType::DatePicker => (
                        date_picker_open_state_key(source.view_id, *widget_id),
                        widget.properties.date_picker_initially_open,
                    ),
                    _ => return None,
                };
                let is_open = match view.custom_state_values.get(&key) {
                    Some(ActionValue::Bool(is_open)) => *is_open,
                    _ => fallback,
                };
                return Some(ActionValue::Bool(is_open));
            }
            let props = &widget.properties;
            match (field_suffix.as_str(), &source.field_type) {
                ("_value", ActionValueType::String) => {
                    Some(ActionValue::String(props.text_input_value.clone()))
                }
                ("_value", ActionValueType::F32) => {
                    Some(ActionValue::Number(props.slider_value as f64))
                }
                ("_checked", _) => Some(ActionValue::Bool(props.checkbox_checked)),
                ("_active", _) => Some(ActionValue::Bool(props.toggler_active)),
                ("_selected", ActionValueType::String) => Some(ActionValue::String(
                    props.picklist_selected.clone().unwrap_or_default(),
                )),
                ("_selected", ActionValueType::Usize) => {
                    Some(ActionValue::Number(props.radio_selected_index as f64))
                }
                _ => None,
            }
        }
        StateRefSource::Custom { field_id, .. } => view.custom_state_values.get(field_id).cloned(),
        StateRefSource::ViewSelection { field_name, .. } => {
            let key = view_selection_state_key(source.view_id, field_name);
            view.custom_state_values.get(&key).cloned()
        }
    }
}

fn apply_state_write(
    target: &StateFieldRef,
    value: ActionValue,
    all_views: &mut BTreeMap<Uuid, AppView>,
) {
    let Some(view) = all_views.get_mut(&target.view_id) else {
        return;
    };
    match &target.source {
        StateRefSource::Widget {
            widget_id,
            field_suffix,
        } => {
            if field_suffix == "_open" {
                let Some(widget) = view.hierarchy.get_widget_by_id(*widget_id) else {
                    return;
                };
                if let ActionValue::Bool(is_open) = value {
                    let key = match widget.widget_type {
                        WidgetType::GenericOverlay => {
                            generic_overlay_open_state_key(target.view_id, *widget_id)
                        }
                        WidgetType::DatePicker => {
                            date_picker_open_state_key(target.view_id, *widget_id)
                        }
                        _ => return,
                    };
                    view.custom_state_values
                        .insert(key, ActionValue::Bool(is_open));
                }
                return;
            }
            let Some(widget) = view.hierarchy.get_widget_by_id_mut(*widget_id) else {
                return;
            };
            let props = &mut widget.properties;
            match (field_suffix.as_str(), value) {
                ("_value", ActionValue::String(s)) => props.text_input_value = s,
                ("_value", ActionValue::Number(n)) => props.slider_value = n as f32,
                ("_checked", ActionValue::Bool(b)) => props.checkbox_checked = b,
                ("_active", ActionValue::Bool(b)) => props.toggler_active = b,
                ("_selected", ActionValue::String(s)) => props.picklist_selected = Some(s),
                ("_selected", ActionValue::Number(n)) => props.radio_selected_index = n as usize,
                _ => {}
            }
        }
        StateRefSource::Custom { field_id, .. } => {
            view.custom_state_values.insert(*field_id, value);
        }
        StateRefSource::ViewSelection { field_name, .. } => {
            let key = view_selection_state_key(target.view_id, field_name);
            view.custom_state_values.insert(key, value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::Point;

    use crate::action_system::flow::{AppFlow, FlowTrigger, WidgetEventRow};
    use crate::action_system::graph::{ActionEdge, ActionNodeData};
    use crate::action_system::node_kinds::{
        ActionNodeKind, ActionValue, AuthoredCondition, AuthoredValueSource, CompareOp,
        ConditionJoinMode, ValueSource,
    };
    use crate::data_structures::types::types::{AppView, WidgetId, WidgetType};

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

    fn trigger_flow_out(flow: &AppFlow) -> u64 {
        flow.graph
            .nodes
            .iter()
            .find(|n| n.is_trigger())
            .and_then(|n| n.cached_ports_out.first())
            .expect("trigger flow_out")
            .id
            .0
    }

    fn set_state_node(id: u64, target: StateFieldRef, value: &str) -> ActionNodeData {
        ActionNodeData::new(
            id,
            ActionNodeKind::StateMutation {
                assignments: vec![crate::action_system::node_kinds::StateAssignment {
                    target: Some(target),
                    value_source: ValueSource::Literal(ActionValue::String(value.to_string())),
                }],
            },
            Point::new(220.0 + id as f32 * 20.0, 120.0),
        )
    }

    fn set_state_flow_in(node: &ActionNodeData) -> u64 {
        node.cached_ports_in
            .iter()
            .find(|p| p.label == "flow_in")
            .expect("setstate flow_in")
            .id
            .0
    }

    fn port_in(node: &ActionNodeData, label: &str) -> u64 {
        node.cached_ports_in
            .iter()
            .find(|p| p.label == label)
            .unwrap_or_else(|| panic!("missing input port '{label}'"))
            .id
            .0
    }

    fn port_out(node: &ActionNodeData, label: &str) -> u64 {
        node.cached_ports_out
            .iter()
            .find(|p| p.label == label)
            .unwrap_or_else(|| panic!("missing output port '{label}'"))
            .id
            .0
    }

    fn one_view(view_id: Uuid) -> BTreeMap<Uuid, AppView> {
        let view = AppView::with_id(view_id, "View".to_string(), 0);
        BTreeMap::from([(view_id, view)])
    }

    fn widget_field(
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

    fn views_with_single_view_reference(
        owner_view_id: Uuid,
        primary_view_id: Uuid,
        secondary_view_id: Uuid,
    ) -> (BTreeMap<Uuid, AppView>, WidgetId) {
        let mut owner_view = AppView::with_id(owner_view_id, "Owner".to_string(), 0);
        let layout_id = owner_view
            .hierarchy
            .add_child(WidgetId(0), WidgetType::Column)
            .expect("add owner layout");
        let view_ref_id = owner_view
            .hierarchy
            .add_child(layout_id, WidgetType::ViewReference)
            .expect("add view reference");
        let view_ref = owner_view
            .hierarchy
            .get_widget_by_id_mut(view_ref_id)
            .expect("view reference");
        view_ref.properties.widget_name = "Pane".to_string();
        view_ref.properties.referenced_view_id = Some(primary_view_id);
        view_ref.properties.extra_view_ids = vec![secondary_view_id];

        let primary_view = AppView::with_id(primary_view_id, "Primary".to_string(), 1);
        let secondary_view = AppView::with_id(secondary_view_id, "Secondary".to_string(), 2);
        (
            BTreeMap::from([
                (owner_view_id, owner_view),
                (primary_view_id, primary_view),
                (secondary_view_id, secondary_view),
            ]),
            view_ref_id,
        )
    }

    fn view_selection_target(
        view_id: Uuid,
        widget_id: WidgetId,
        field_name: &str,
    ) -> StateFieldRef {
        StateFieldRef {
            view_id,
            source: StateRefSource::ViewSelection {
                widget_id,
                field_name: field_name.to_string(),
            },
            field_type: ActionValueType::Enum {
                type_name: "PreviewSelection".to_string(),
                variants: vec!["Primary".to_string(), "Secondary".to_string()],
            },
            display_name: field_name.to_string(),
        }
    }

    #[test]
    fn generic_overlay_widget_state_reads_and_writes_open_flag() {
        let view_id = Uuid::new_v4();
        let mut all_views = one_view(view_id);
        let widget_id = {
            let view = all_views.get_mut(&view_id).expect("view");
            let layout_id = view
                .hierarchy
                .add_child(WidgetId(0), WidgetType::Column)
                .expect("add layout");
            view.hierarchy
                .add_child(layout_id, WidgetType::GenericOverlay)
                .expect("add overlay")
        };
        all_views
            .get_mut(&view_id)
            .and_then(|view| view.hierarchy.get_widget_by_id_mut(widget_id))
            .expect("overlay")
            .properties
            .generic_overlay_initially_open = true;
        let field = widget_field(view_id, widget_id, "_open", ActionValueType::Bool);

        assert_eq!(
            read_state(&field, &all_views),
            Some(ActionValue::Bool(true))
        );

        apply_state_write(&field, ActionValue::Bool(false), &mut all_views);

        assert_eq!(
            read_state(&field, &all_views),
            Some(ActionValue::Bool(false))
        );
        let key = generic_overlay_open_state_key(view_id, widget_id);
        assert_eq!(
            all_views
                .get(&view_id)
                .and_then(|view| view.custom_state_values.get(&key)),
            Some(&ActionValue::Bool(false))
        );
    }

    #[test]
    fn date_picker_widget_state_reads_and_writes_open_flag() {
        let view_id = Uuid::new_v4();
        let mut all_views = one_view(view_id);
        let widget_id = {
            let view = all_views.get_mut(&view_id).expect("view");
            let layout_id = view
                .hierarchy
                .add_child(WidgetId(0), WidgetType::Column)
                .expect("add layout");
            view.hierarchy
                .add_child(layout_id, WidgetType::DatePicker)
                .expect("add date picker")
        };
        all_views
            .get_mut(&view_id)
            .and_then(|view| view.hierarchy.get_widget_by_id_mut(widget_id))
            .expect("date picker")
            .properties
            .date_picker_initially_open = true;
        let field = widget_field(view_id, widget_id, "_open", ActionValueType::Bool);

        assert_eq!(
            read_state(&field, &all_views),
            Some(ActionValue::Bool(true))
        );

        apply_state_write(&field, ActionValue::Bool(false), &mut all_views);

        assert_eq!(
            read_state(&field, &all_views),
            Some(ActionValue::Bool(false))
        );
        let key = date_picker_open_state_key(view_id, widget_id);
        assert_eq!(
            all_views
                .get(&view_id)
                .and_then(|view| view.custom_state_values.get(&key)),
            Some(&ActionValue::Bool(false))
        );
    }

    #[test]
    fn execute_event_skips_disabled_widget_event_flows() {
        let view_id = Uuid::new_v4();
        let widget = WidgetId(9);
        let field_id = Uuid::new_v4();
        let target = custom_target(view_id, field_id, "result");

        let row = WidgetEventRow {
            id: Uuid::new_v4(),
            event_type: "on_press".to_string(),
            target: Some((view_id, widget.0)),
        };

        let mut enabled = AppFlow::new(
            "enabled".to_string(),
            FlowTrigger::WidgetEvent {
                rows: vec![row.clone()],
            },
        );
        enabled.sync_trigger_topology();
        enabled.enabled = true;
        let enabled_node = set_state_node(2, target.clone(), "enabled");
        let enabled_in = set_state_flow_in(&enabled_node);
        enabled.graph.nodes.push(enabled_node);
        enabled.graph.z_order.push(2);
        enabled.graph.next_id = 3;
        assert!(
            enabled
                .graph
                .connect_ports(1, trigger_flow_out(&enabled), 2, enabled_in)
        );

        let mut disabled = AppFlow::new(
            "disabled".to_string(),
            FlowTrigger::WidgetEvent { rows: vec![row] },
        );
        disabled.sync_trigger_topology();
        disabled.enabled = false;
        let disabled_node = set_state_node(2, target.clone(), "disabled");
        let disabled_in = set_state_flow_in(&disabled_node);
        disabled.graph.nodes.push(disabled_node);
        disabled.graph.z_order.push(2);
        disabled.graph.next_id = 3;
        assert!(
            disabled
                .graph
                .connect_ports(1, trigger_flow_out(&disabled), 2, disabled_in)
        );

        // Disabled flow is last; if it executes at all, it will overwrite the enabled value.
        let flows = vec![enabled, disabled];
        let mut all_views = one_view(view_id);
        let _ = execute_event(
            &flows,
            &mut all_views,
            view_id,
            widget,
            "on_press",
            HashMap::new(),
        );

        let actual = all_views
            .get(&view_id)
            .and_then(|v| v.custom_state_values.get(&field_id))
            .cloned();
        assert_eq!(actual, Some(ActionValue::String("enabled".to_string())));
    }

    #[test]
    fn execute_event_navigate_to_app_view_returns_target_view_id() {
        let origin_view_id = Uuid::new_v4();
        let destination_view_id = Uuid::new_v4();
        let trigger_widget = WidgetId(17);
        let row = WidgetEventRow {
            id: Uuid::new_v4(),
            event_type: "on_press".to_string(),
            target: Some((origin_view_id, trigger_widget.0)),
        };
        let mut flow = AppFlow::new(
            "navigate".to_string(),
            FlowTrigger::WidgetEvent { rows: vec![row] },
        );
        flow.sync_trigger_topology();
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
        assert!(flow.graph.connect_ports(
            1,
            trigger_flow_out(&flow),
            2,
            port_in(&navigate, "flow_in"),
        ));

        let mut all_views = BTreeMap::from([
            (
                origin_view_id,
                AppView::with_id(origin_view_id, "Origin".to_string(), 0),
            ),
            (
                destination_view_id,
                AppView::with_id(destination_view_id, "Destination".to_string(), 1),
            ),
        ]);
        let nav = execute_event(
            &[flow],
            &mut all_views,
            origin_view_id,
            trigger_widget,
            "on_press",
            HashMap::new(),
        );
        assert_eq!(nav, Some(destination_view_id));
    }

    #[test]
    fn execute_event_navigate_to_view_reference_updates_canonical_selection_state() {
        let owner_view_id = Uuid::new_v4();
        let primary_view_id = Uuid::new_v4();
        let secondary_view_id = Uuid::new_v4();
        let trigger_widget = WidgetId(55);
        let (mut all_views, view_ref_id) =
            views_with_single_view_reference(owner_view_id, primary_view_id, secondary_view_id);
        let row = WidgetEventRow {
            id: Uuid::new_v4(),
            event_type: "on_press".to_string(),
            target: Some((owner_view_id, trigger_widget.0)),
        };
        let mut flow = AppFlow::new(
            "switch-pane".to_string(),
            FlowTrigger::WidgetEvent { rows: vec![row] },
        );
        flow.sync_trigger_topology();
        let navigate = ActionNodeData::new(
            2,
            ActionNodeKind::NavigateToView {
                targets: vec![Some(NavigateTarget::ViewReference {
                    owner_view_id,
                    widget_id: view_ref_id,
                    target_view_id: secondary_view_id,
                })],
            },
            Point::new(260.0, 120.0),
        );
        flow.graph.nodes.push(navigate.clone());
        flow.graph.z_order.push(2);
        flow.graph.next_id = 3;
        assert!(flow.graph.connect_ports(
            1,
            trigger_flow_out(&flow),
            2,
            port_in(&navigate, "flow_in"),
        ));

        let nav = execute_event(
            &[flow],
            &mut all_views,
            owner_view_id,
            trigger_widget,
            "on_press",
            HashMap::new(),
        );
        assert_eq!(nav, None);

        let key = view_reference_selection_state_key(owner_view_id, view_ref_id);
        let stored = all_views
            .get(&owner_view_id)
            .and_then(|view| view.custom_state_values.get(&key))
            .cloned();
        assert_eq!(
            stored,
            Some(ActionValue::String(secondary_view_id.to_string()))
        );
    }

    #[test]
    fn match_branch_routing_uses_aligned_slots() {
        let view_id = Uuid::new_v4();
        let widget = WidgetId(7);
        let field_id = Uuid::new_v4();
        let target = custom_target(view_id, field_id, "branch");

        let row = WidgetEventRow {
            id: Uuid::new_v4(),
            event_type: "on_press".to_string(),
            target: Some((view_id, widget.0)),
        };
        let mut flow = AppFlow::new(
            "match".to_string(),
            FlowTrigger::WidgetEvent { rows: vec![row] },
        );
        flow.sync_trigger_topology();

        let match_node = ActionNodeData::new(
            2,
            ActionNodeKind::Match {
                arms: vec!["A".to_string()],
                enum_type: None,
            },
            Point::new(260.0, 120.0),
        );
        let literal_node = ActionNodeData::new(
            3,
            ActionNodeKind::StringLiteral {
                value: "A".to_string(),
            },
            Point::new(120.0, 220.0),
        );
        let arm_set = set_state_node(4, target.clone(), "arm");
        let default_set = set_state_node(5, target.clone(), "default");

        let match_flow_in = match_node
            .cached_ports_in
            .iter()
            .find(|p| p.label == "flow_in")
            .expect("match flow_in")
            .id
            .0;
        let match_value_in = match_node
            .cached_ports_in
            .iter()
            .find(|p| p.label == "value")
            .expect("match value")
            .id
            .0;
        let match_arm_out = match_node
            .cached_ports_out
            .iter()
            .find(|p| p.label == "A")
            .expect("arm out")
            .id
            .0;
        let match_default_out = match_node
            .cached_ports_out
            .iter()
            .find(|p| p.label == "default")
            .expect("default out")
            .id
            .0;
        let lit_out = literal_node.cached_ports_out[0].id.0;
        let arm_in = set_state_flow_in(&arm_set);
        let default_in = set_state_flow_in(&default_set);

        flow.graph
            .nodes
            .extend([match_node, literal_node, arm_set, default_set]);
        flow.graph.z_order.extend([2, 3, 4, 5]);
        flow.graph.next_id = 6;

        assert!(
            flow.graph
                .connect_ports(1, trigger_flow_out(&flow), 2, match_flow_in)
        );
        assert!(flow.graph.connect_ports(3, lit_out, 2, match_value_in));
        assert!(flow.graph.connect_ports(2, match_arm_out, 4, arm_in));
        assert!(
            flow.graph
                .connect_ports(2, match_default_out, 5, default_in)
        );

        let mut all_views = one_view(view_id);
        let _ = execute_event(
            &[flow.clone()],
            &mut all_views,
            view_id,
            widget,
            "on_press",
            HashMap::new(),
        );
        let first = all_views
            .get(&view_id)
            .and_then(|v| v.custom_state_values.get(&field_id))
            .cloned();
        assert_eq!(first, Some(ActionValue::String("arm".to_string())));

        // Change match input to force default branch.
        if let Some(lit) = flow.graph.nodes.iter_mut().find(|n| n.id == 3) {
            lit.kind = ActionNodeKind::StringLiteral {
                value: "Z".to_string(),
            };
            lit.rebuild_ports();
        }
        let mut all_views = one_view(view_id);
        let _ = execute_event(
            &[flow],
            &mut all_views,
            view_id,
            widget,
            "on_press",
            HashMap::new(),
        );
        let second = all_views
            .get(&view_id)
            .and_then(|v| v.custom_state_values.get(&field_id))
            .cloned();
        assert_eq!(second, Some(ActionValue::String("default".to_string())));
    }

    #[test]
    fn conditional_evaluates_lowered_boolean_expression() {
        let view_id = Uuid::new_v4();
        let widget = WidgetId(17);
        let field_id = Uuid::new_v4();
        let target = custom_target(view_id, field_id, "conditional_result");

        let row = WidgetEventRow {
            id: Uuid::new_v4(),
            event_type: "on_press".to_string(),
            target: Some((view_id, widget.0)),
        };
        let mut flow = AppFlow::new(
            "conditional".to_string(),
            FlowTrigger::WidgetEvent { rows: vec![row] },
        );
        flow.sync_trigger_topology();

        let lhs = ActionNodeData::new(
            2,
            ActionNodeKind::StringLiteral {
                value: "A".to_string(),
            },
            Point::new(80.0, 220.0),
        );
        let rhs = ActionNodeData::new(
            3,
            ActionNodeKind::StringLiteral {
                value: "A".to_string(),
            },
            Point::new(80.0, 280.0),
        );
        let compare = ActionNodeData::new(
            4,
            ActionNodeKind::Compare {
                operator: crate::action_system::node_kinds::CompareOp::Eq,
                rhs: crate::action_system::node_kinds::CompareRhs::FromPort,
                rhs_literal: ActionValue::String(String::new()),
            },
            Point::new(260.0, 240.0),
        );
        let not = ActionNodeData::new(5, ActionNodeKind::LogicNot, Point::new(420.0, 240.0));
        let conditional =
            ActionNodeData::new(6, ActionNodeKind::Conditional, Point::new(580.0, 240.0));
        let true_set = set_state_node(7, target.clone(), "true_branch");
        let false_set = set_state_node(8, target.clone(), "false_branch");

        flow.graph.nodes.extend([
            lhs.clone(),
            rhs.clone(),
            compare.clone(),
            not.clone(),
            conditional.clone(),
            true_set.clone(),
            false_set.clone(),
        ]);
        flow.graph.z_order.extend([2, 3, 4, 5, 6, 7, 8]);
        flow.graph.next_id = 9;

        assert!(flow.graph.connect_ports(
            1,
            trigger_flow_out(&flow),
            6,
            port_in(&conditional, "flow_in"),
        ));
        assert!(flow.graph.connect_ports(
            2,
            port_out(&lhs, "value"),
            4,
            port_in(&compare, "value")
        ));
        assert!(
            flow.graph
                .connect_ports(3, port_out(&rhs, "value"), 4, port_in(&compare, "rhs"))
        );
        assert!(flow.graph.connect_ports(
            4,
            port_out(&compare, "result"),
            5,
            port_in(&not, "value")
        ));
        assert!(flow.graph.connect_ports(
            5,
            port_out(&not, "result"),
            6,
            port_in(&conditional, "condition"),
        ));
        assert!(flow.graph.connect_ports(
            6,
            port_out(&conditional, "true"),
            7,
            set_state_flow_in(&true_set),
        ));
        assert!(flow.graph.connect_ports(
            6,
            port_out(&conditional, "false"),
            8,
            set_state_flow_in(&false_set),
        ));

        // A == A -> true, then NOT -> false, so false branch should run.
        let mut all_views = one_view(view_id);
        let _ = execute_event(
            &[flow.clone()],
            &mut all_views,
            view_id,
            widget,
            "on_press",
            HashMap::new(),
        );
        let first = all_views
            .get(&view_id)
            .and_then(|v| v.custom_state_values.get(&field_id))
            .cloned();
        assert_eq!(first, Some(ActionValue::String("false_branch".to_string())));

        // Change rhs to force compare false; NOT false -> true branch.
        if let Some(rhs_node) = flow.graph.nodes.iter_mut().find(|n| n.id == 3) {
            rhs_node.kind = ActionNodeKind::StringLiteral {
                value: "B".to_string(),
            };
            rhs_node.rebuild_ports();
        }
        let mut all_views = one_view(view_id);
        let _ = execute_event(
            &[flow],
            &mut all_views,
            view_id,
            widget,
            "on_press",
            HashMap::new(),
        );
        let second = all_views
            .get(&view_id)
            .and_then(|v| v.custom_state_values.get(&field_id))
            .cloned();
        assert_eq!(second, Some(ActionValue::String("true_branch".to_string())));
    }

    #[test]
    fn authored_conditional_evaluates_trigger_input_email_validation() {
        let view_id = Uuid::new_v4();
        let widget = WidgetId(32);
        let true_field_id = Uuid::new_v4();
        let false_field_id = Uuid::new_v4();
        let true_target = custom_target(view_id, true_field_id, "if_true");
        let false_target = custom_target(view_id, false_field_id, "if_false");

        let row = WidgetEventRow {
            id: Uuid::new_v4(),
            event_type: "on_press".to_string(),
            target: Some((view_id, widget.0)),
        };
        let mut flow = AppFlow::new(
            "if_authored".to_string(),
            FlowTrigger::WidgetEvent { rows: vec![row] },
        );
        flow.sync_trigger_topology();

        let mut conditional =
            ActionNodeData::new(2, ActionNodeKind::Conditional, Point::new(320.0, 120.0));
        conditional.authored_conditions = vec![AuthoredCondition {
            lhs: AuthoredValueSource::TriggerInput {
                name: "email".to_string(),
                value_type: ActionValueType::String,
            },
            operator: CompareOp::IsValidEmail,
            rhs_literal: ActionValue::String(String::new()),
        }];
        let true_write = set_state_node(3, true_target, "true_branch");
        let false_write = set_state_node(4, false_target, "false_branch");

        flow.graph
            .nodes
            .extend([conditional.clone(), true_write.clone(), false_write.clone()]);
        flow.graph.z_order.extend([2, 3, 4]);
        flow.graph.next_id = 5;

        assert!(flow.graph.connect_ports(
            1,
            trigger_flow_out(&flow),
            2,
            port_in(&conditional, "flow_in"),
        ));
        assert!(flow.graph.connect_ports(
            2,
            port_out(&conditional, "true"),
            3,
            set_state_flow_in(&true_write),
        ));
        assert!(flow.graph.connect_ports(
            2,
            port_out(&conditional, "false"),
            4,
            set_state_flow_in(&false_write),
        ));

        let mut all_views = one_view(view_id);
        let _ = execute_event(
            &[flow.clone()],
            &mut all_views,
            view_id,
            widget,
            "on_press",
            HashMap::from([(
                "email".to_string(),
                ActionValue::String("admin@example.com".to_string()),
            )]),
        );
        let true_branch_value = all_views
            .get(&view_id)
            .and_then(|v| v.custom_state_values.get(&true_field_id))
            .cloned();
        let false_branch_value = all_views
            .get(&view_id)
            .and_then(|v| v.custom_state_values.get(&false_field_id))
            .cloned();
        assert_eq!(
            true_branch_value,
            Some(ActionValue::String("true_branch".to_string()))
        );
        assert_eq!(false_branch_value, None);

        let mut all_views = one_view(view_id);
        let _ = execute_event(
            &[flow],
            &mut all_views,
            view_id,
            widget,
            "on_press",
            HashMap::from([(
                "email".to_string(),
                ActionValue::String("invalid-email".to_string()),
            )]),
        );
        let true_branch_value = all_views
            .get(&view_id)
            .and_then(|v| v.custom_state_values.get(&true_field_id))
            .cloned();
        let false_branch_value = all_views
            .get(&view_id)
            .and_then(|v| v.custom_state_values.get(&false_field_id))
            .cloned();
        assert_eq!(true_branch_value, None);
        assert_eq!(
            false_branch_value,
            Some(ActionValue::String("false_branch".to_string()))
        );
    }

    #[test]
    fn authored_conditional_multi_conditions_honor_all_vs_any_join_mode() {
        let view_id = Uuid::new_v4();
        let widget = WidgetId(33);
        let true_field_id = Uuid::new_v4();
        let false_field_id = Uuid::new_v4();
        let true_target = custom_target(view_id, true_field_id, "if_true");
        let false_target = custom_target(view_id, false_field_id, "if_false");

        let row = WidgetEventRow {
            id: Uuid::new_v4(),
            event_type: "on_press".to_string(),
            target: Some((view_id, widget.0)),
        };
        let mut flow = AppFlow::new(
            "if_authored_multi".to_string(),
            FlowTrigger::WidgetEvent { rows: vec![row] },
        );
        flow.sync_trigger_topology();

        let mut conditional =
            ActionNodeData::new(2, ActionNodeKind::Conditional, Point::new(320.0, 120.0));
        conditional.authored_condition_join = ConditionJoinMode::All;
        conditional.authored_conditions = vec![
            AuthoredCondition {
                lhs: AuthoredValueSource::TriggerInput {
                    name: "email".to_string(),
                    value_type: ActionValueType::String,
                },
                operator: CompareOp::IsValidEmail,
                rhs_literal: ActionValue::String(String::new()),
            },
            AuthoredCondition {
                lhs: AuthoredValueSource::TriggerInput {
                    name: "role".to_string(),
                    value_type: ActionValueType::String,
                },
                operator: CompareOp::Eq,
                rhs_literal: ActionValue::String("admin".to_string()),
            },
        ];
        let true_write = set_state_node(3, true_target, "true_branch");
        let false_write = set_state_node(4, false_target, "false_branch");

        flow.graph
            .nodes
            .extend([conditional.clone(), true_write.clone(), false_write.clone()]);
        flow.graph.z_order.extend([2, 3, 4]);
        flow.graph.next_id = 5;
        assert!(flow.graph.connect_ports(
            1,
            trigger_flow_out(&flow),
            2,
            port_in(&conditional, "flow_in"),
        ));
        assert!(flow.graph.connect_ports(
            2,
            port_out(&conditional, "true"),
            3,
            set_state_flow_in(&true_write),
        ));
        assert!(flow.graph.connect_ports(
            2,
            port_out(&conditional, "false"),
            4,
            set_state_flow_in(&false_write),
        ));

        let mut all_views = one_view(view_id);
        let _ = execute_event(
            &[flow.clone()],
            &mut all_views,
            view_id,
            widget,
            "on_press",
            HashMap::from([
                (
                    "email".to_string(),
                    ActionValue::String("admin@example.com".to_string()),
                ),
                ("role".to_string(), ActionValue::String("user".to_string())),
            ]),
        );
        let true_branch_value = all_views
            .get(&view_id)
            .and_then(|v| v.custom_state_values.get(&true_field_id))
            .cloned();
        let false_branch_value = all_views
            .get(&view_id)
            .and_then(|v| v.custom_state_values.get(&false_field_id))
            .cloned();
        assert_eq!(true_branch_value, None);
        assert_eq!(
            false_branch_value,
            Some(ActionValue::String("false_branch".to_string()))
        );

        if let Some(node) = flow.graph.nodes.iter_mut().find(|n| n.id == 2) {
            node.authored_condition_join = ConditionJoinMode::Any;
        }
        let mut all_views = one_view(view_id);
        let _ = execute_event(
            &[flow],
            &mut all_views,
            view_id,
            widget,
            "on_press",
            HashMap::from([
                (
                    "email".to_string(),
                    ActionValue::String("admin@example.com".to_string()),
                ),
                ("role".to_string(), ActionValue::String("user".to_string())),
            ]),
        );
        let true_branch_value = all_views
            .get(&view_id)
            .and_then(|v| v.custom_state_values.get(&true_field_id))
            .cloned();
        assert_eq!(
            true_branch_value,
            Some(ActionValue::String("true_branch".to_string()))
        );
    }

    #[test]
    fn authored_match_subject_routes_to_arm_or_default_without_value_input_edge() {
        let view_id = Uuid::new_v4();
        let widget = WidgetId(52);
        let field_id = Uuid::new_v4();
        let target = custom_target(view_id, field_id, "branch");

        let row = WidgetEventRow {
            id: Uuid::new_v4(),
            event_type: "on_press".to_string(),
            target: Some((view_id, widget.0)),
        };
        let mut flow = AppFlow::new(
            "match_authored".to_string(),
            FlowTrigger::WidgetEvent { rows: vec![row] },
        );
        flow.sync_trigger_topology();

        let mut match_node = ActionNodeData::new(
            2,
            ActionNodeKind::Match {
                arms: vec!["A".to_string()],
                enum_type: None,
            },
            Point::new(260.0, 120.0),
        );
        match_node.authored_match_subject = Some(AuthoredValueSource::Literal(
            ActionValue::String("A".to_string()),
        ));
        let arm_set = set_state_node(3, target.clone(), "arm");
        let default_set = set_state_node(4, target, "default");

        flow.graph
            .nodes
            .extend([match_node.clone(), arm_set.clone(), default_set.clone()]);
        flow.graph.z_order.extend([2, 3, 4]);
        flow.graph.next_id = 5;

        assert!(flow.graph.connect_ports(
            1,
            trigger_flow_out(&flow),
            2,
            port_in(&match_node, "flow_in"),
        ));
        assert!(flow.graph.connect_ports(
            2,
            port_out(&match_node, "A"),
            3,
            set_state_flow_in(&arm_set),
        ));
        assert!(flow.graph.connect_ports(
            2,
            port_out(&match_node, "default"),
            4,
            set_state_flow_in(&default_set),
        ));

        let mut all_views = one_view(view_id);
        let _ = execute_event(
            &[flow.clone()],
            &mut all_views,
            view_id,
            widget,
            "on_press",
            HashMap::new(),
        );
        let matched = all_views
            .get(&view_id)
            .and_then(|v| v.custom_state_values.get(&field_id))
            .cloned();
        assert_eq!(matched, Some(ActionValue::String("arm".to_string())));

        if let Some(node) = flow.graph.nodes.iter_mut().find(|n| n.id == 2) {
            node.authored_match_subject = Some(AuthoredValueSource::Literal(ActionValue::String(
                "Z".to_string(),
            )));
        }

        let mut all_views = one_view(view_id);
        let _ = execute_event(
            &[flow],
            &mut all_views,
            view_id,
            widget,
            "on_press",
            HashMap::new(),
        );
        let fallback = all_views
            .get(&view_id)
            .and_then(|v| v.custom_state_values.get(&field_id))
            .cloned();
        assert_eq!(fallback, Some(ActionValue::String("default".to_string())));
    }

    #[test]
    fn flow_walk_executes_fanout_successors_in_insertion_order() {
        let view_id = Uuid::new_v4();
        let widget = WidgetId(5);
        let field_id = Uuid::new_v4();
        let target = custom_target(view_id, field_id, "fanout");

        let row = WidgetEventRow {
            id: Uuid::new_v4(),
            event_type: "on_press".to_string(),
            target: Some((view_id, widget.0)),
        };
        let mut flow = AppFlow::new(
            "fanout".to_string(),
            FlowTrigger::WidgetEvent { rows: vec![row] },
        );
        flow.sync_trigger_topology();

        let first = set_state_node(2, target.clone(), "first");
        let second = set_state_node(3, target, "second");
        let first_in = set_state_flow_in(&first);
        let second_in = set_state_flow_in(&second);
        flow.graph.nodes.extend([first, second]);
        flow.graph.z_order.extend([2, 3]);
        flow.graph.next_id = 4;

        assert!(
            flow.graph
                .connect_ports(1, trigger_flow_out(&flow), 2, first_in)
        );
        assert!(
            flow.graph
                .connect_ports(1, trigger_flow_out(&flow), 3, second_in)
        );

        let mut all_views = one_view(view_id);
        let _ = execute_event(
            &[flow],
            &mut all_views,
            view_id,
            widget,
            "on_press",
            HashMap::new(),
        );
        let actual = all_views
            .get(&view_id)
            .and_then(|v| v.custom_state_values.get(&field_id))
            .cloned();
        assert_eq!(actual, Some(ActionValue::String("second".to_string())));
    }

    #[test]
    fn call_flow_executes_by_stable_id_after_callable_flow_rename() {
        let view_id = Uuid::new_v4();
        let widget = WidgetId(11);
        let called_field_id = Uuid::new_v4();
        let caller_tail_field_id = Uuid::new_v4();
        let callable_id = Uuid::new_v4();

        let called_target = custom_target(view_id, called_field_id, "called_result");
        let tail_target = custom_target(view_id, caller_tail_field_id, "caller_tail");

        let mut callable_flow =
            AppFlow::new("Original Callable".to_string(), FlowTrigger::Callable);
        callable_flow.id = callable_id;
        callable_flow.sync_trigger_topology();
        let callable_write = set_state_node(2, called_target, "called");
        let callable_write_in = set_state_flow_in(&callable_write);
        callable_flow.graph.nodes.push(callable_write);
        callable_flow.graph.z_order.push(2);
        callable_flow.graph.next_id = 3;
        assert!(callable_flow.graph.connect_ports(
            1,
            trigger_flow_out(&callable_flow),
            2,
            callable_write_in
        ));

        // Rename the callable flow to prove call resolution is id-based.
        callable_flow.name = "Renamed Callable".to_string();

        let row = WidgetEventRow {
            id: Uuid::new_v4(),
            event_type: "on_press".to_string(),
            target: Some((view_id, widget.0)),
        };
        let mut caller_flow = AppFlow::new(
            "Caller".to_string(),
            FlowTrigger::WidgetEvent { rows: vec![row] },
        );
        caller_flow.sync_trigger_topology();
        let call_node = ActionNodeData::new(
            2,
            ActionNodeKind::CallFlow {
                flow_id: Some(callable_id),
            },
            Point::new(260.0, 120.0),
        );
        let tail_write = set_state_node(3, tail_target, "after_call");
        let call_in = call_node
            .cached_ports_in
            .iter()
            .find(|p| p.label == "flow_in")
            .expect("call flow_in")
            .id
            .0;
        let call_out = call_node
            .cached_ports_out
            .iter()
            .find(|p| p.label == "flow_out")
            .expect("call flow_out")
            .id
            .0;
        let tail_in = set_state_flow_in(&tail_write);
        caller_flow.graph.nodes.extend([call_node, tail_write]);
        caller_flow.graph.z_order.extend([2, 3]);
        caller_flow.graph.next_id = 4;
        assert!(
            caller_flow
                .graph
                .connect_ports(1, trigger_flow_out(&caller_flow), 2, call_in)
        );
        assert!(caller_flow.graph.connect_ports(2, call_out, 3, tail_in));

        let flows = vec![caller_flow, callable_flow];
        let mut all_views = one_view(view_id);
        let _ = execute_event(
            &flows,
            &mut all_views,
            view_id,
            widget,
            "on_press",
            HashMap::new(),
        );

        let called_value = all_views
            .get(&view_id)
            .and_then(|v| v.custom_state_values.get(&called_field_id))
            .cloned();
        let tail_value = all_views
            .get(&view_id)
            .and_then(|v| v.custom_state_values.get(&caller_tail_field_id))
            .cloned();
        assert_eq!(
            called_value,
            Some(ActionValue::String("called".to_string()))
        );
        assert_eq!(
            tail_value,
            Some(ActionValue::String("after_call".to_string()))
        );
    }

    #[test]
    fn state_mutation_supports_multi_assignment_and_value_sources() {
        let view_id = Uuid::new_v4();
        let widget = WidgetId(21);
        let ordered_field_id = Uuid::new_v4();
        let from_state_field_id = Uuid::new_v4();
        let from_port_field_id = Uuid::new_v4();
        let seed_field_id = Uuid::new_v4();

        let row = WidgetEventRow {
            id: Uuid::new_v4(),
            event_type: "on_press".to_string(),
            target: Some((view_id, widget.0)),
        };
        let mut flow = AppFlow::new(
            "multi".to_string(),
            FlowTrigger::WidgetEvent { rows: vec![row] },
        );
        flow.sync_trigger_topology();

        let seed_ref = custom_target(view_id, seed_field_id, "seed");
        let mut_node = ActionNodeData::new(
            2,
            ActionNodeKind::StateMutation {
                assignments: vec![
                    crate::action_system::node_kinds::StateAssignment {
                        target: Some(custom_target(view_id, ordered_field_id, "ordered")),
                        value_source: ValueSource::Literal(ActionValue::String(
                            "first".to_string(),
                        )),
                    },
                    crate::action_system::node_kinds::StateAssignment {
                        target: Some(custom_target(view_id, ordered_field_id, "ordered")),
                        value_source: ValueSource::Literal(ActionValue::String(
                            "second".to_string(),
                        )),
                    },
                    crate::action_system::node_kinds::StateAssignment {
                        target: Some(custom_target(view_id, from_state_field_id, "from_state")),
                        value_source: ValueSource::StateField(seed_ref.clone()),
                    },
                    crate::action_system::node_kinds::StateAssignment {
                        target: Some(custom_target(view_id, from_port_field_id, "from_port")),
                        value_source: ValueSource::FromPort,
                    },
                ],
            },
            Point::new(260.0, 120.0),
        );
        let literal_node = ActionNodeData::new(
            3,
            ActionNodeKind::StringLiteral {
                value: "port_value".to_string(),
            },
            Point::new(120.0, 220.0),
        );
        let flow_in = mut_node
            .cached_ports_in
            .iter()
            .find(|p| p.label == "flow_in")
            .expect("flow_in")
            .id
            .0;
        let from_port_in = mut_node
            .cached_ports_in
            .iter()
            .find(|p| p.label == "value_3")
            .expect("value_3 input")
            .id
            .0;
        let literal_out = literal_node.cached_ports_out[0].id.0;
        flow.graph.nodes.extend([mut_node, literal_node]);
        flow.graph.z_order.extend([2, 3]);
        flow.graph.next_id = 4;
        assert!(
            flow.graph
                .connect_ports(1, trigger_flow_out(&flow), 2, flow_in)
        );
        assert!(flow.graph.connect_ports(3, literal_out, 2, from_port_in));

        let mut all_views = one_view(view_id);
        all_views
            .get_mut(&view_id)
            .expect("view exists")
            .custom_state_values
            .insert(seed_field_id, ActionValue::String("seed_value".to_string()));

        let _ = execute_event(
            &[flow],
            &mut all_views,
            view_id,
            widget,
            "on_press",
            HashMap::new(),
        );

        let ordered_value = all_views
            .get(&view_id)
            .and_then(|v| v.custom_state_values.get(&ordered_field_id))
            .cloned();
        let from_state_value = all_views
            .get(&view_id)
            .and_then(|v| v.custom_state_values.get(&from_state_field_id))
            .cloned();
        let from_port_value = all_views
            .get(&view_id)
            .and_then(|v| v.custom_state_values.get(&from_port_field_id))
            .cloned();
        assert_eq!(
            ordered_value,
            Some(ActionValue::String("second".to_string()))
        );
        assert_eq!(
            from_state_value,
            Some(ActionValue::String("seed_value".to_string()))
        );
        assert_eq!(
            from_port_value,
            Some(ActionValue::String("port_value".to_string()))
        );
    }

    #[test]
    fn state_mutation_view_selection_assignments_use_field_name_storage_keys() {
        let view_id = Uuid::new_v4();
        let widget = WidgetId(44);
        let first_custom_field_id = Uuid::new_v4();
        let second_custom_field_id = Uuid::new_v4();
        let first_selection = view_selection_target(view_id, widget, "first_selection");
        let second_selection = view_selection_target(view_id, widget, "second_selection");

        let row = WidgetEventRow {
            id: Uuid::new_v4(),
            event_type: "on_press".to_string(),
            target: Some((view_id, widget.0)),
        };
        let mut flow = AppFlow::new(
            "view_selection".to_string(),
            FlowTrigger::WidgetEvent { rows: vec![row] },
        );
        flow.sync_trigger_topology();

        let mutation = ActionNodeData::new(
            2,
            ActionNodeKind::StateMutation {
                assignments: vec![
                    crate::action_system::node_kinds::StateAssignment {
                        target: Some(first_selection.clone()),
                        value_source: ValueSource::Literal(ActionValue::EnumVariant {
                            type_name: "PreviewSelection".to_string(),
                            variant: "Primary".to_string(),
                        }),
                    },
                    crate::action_system::node_kinds::StateAssignment {
                        target: Some(second_selection.clone()),
                        value_source: ValueSource::Literal(ActionValue::EnumVariant {
                            type_name: "PreviewSelection".to_string(),
                            variant: "Secondary".to_string(),
                        }),
                    },
                    crate::action_system::node_kinds::StateAssignment {
                        target: Some(custom_target(
                            view_id,
                            first_custom_field_id,
                            "first_result",
                        )),
                        value_source: ValueSource::StateField(first_selection),
                    },
                    crate::action_system::node_kinds::StateAssignment {
                        target: Some(custom_target(
                            view_id,
                            second_custom_field_id,
                            "second_result",
                        )),
                        value_source: ValueSource::StateField(second_selection),
                    },
                ],
            },
            Point::new(280.0, 120.0),
        );
        let flow_in = set_state_flow_in(&mutation);
        flow.graph.nodes.push(mutation);
        flow.graph.z_order.push(2);
        flow.graph.next_id = 3;
        assert!(
            flow.graph
                .connect_ports(1, trigger_flow_out(&flow), 2, flow_in)
        );

        let mut all_views = one_view(view_id);
        let _ = execute_event(
            &[flow],
            &mut all_views,
            view_id,
            widget,
            "on_press",
            HashMap::new(),
        );

        let first = all_views
            .get(&view_id)
            .and_then(|v| v.custom_state_values.get(&first_custom_field_id))
            .cloned();
        let second = all_views
            .get(&view_id)
            .and_then(|v| v.custom_state_values.get(&second_custom_field_id))
            .cloned();
        assert_eq!(
            first,
            Some(ActionValue::EnumVariant {
                type_name: "PreviewSelection".to_string(),
                variant: "Primary".to_string(),
            })
        );
        assert_eq!(
            second,
            Some(ActionValue::EnumVariant {
                type_name: "PreviewSelection".to_string(),
                variant: "Secondary".to_string(),
            })
        );
    }

    #[test]
    fn invalid_graph_does_not_execute_as_if_valid() {
        let view_id = Uuid::new_v4();
        let widget = WidgetId(33);
        let field_id = Uuid::new_v4();
        let target = custom_target(view_id, field_id, "result");

        let row = WidgetEventRow {
            id: Uuid::new_v4(),
            event_type: "on_press".to_string(),
            target: Some((view_id, widget.0)),
        };
        let mut flow = AppFlow::new(
            "invalid".to_string(),
            FlowTrigger::WidgetEvent { rows: vec![row] },
        );
        flow.sync_trigger_topology();
        let set_state = set_state_node(2, target, "should_not_run");
        let set_state_in = set_state_flow_in(&set_state);
        flow.graph.nodes.push(set_state);
        flow.graph.z_order.push(2);
        flow.graph.next_id = 3;
        flow.graph.edges.push(ActionEdge {
            from_node: 1,
            from_port: 1001,
            to_node: 2,
            to_port: set_state_in,
        });

        let mut all_views = one_view(view_id);
        let _ = execute_event(
            &[flow],
            &mut all_views,
            view_id,
            widget,
            "on_press",
            HashMap::new(),
        );

        let actual = all_views
            .get(&view_id)
            .and_then(|v| v.custom_state_values.get(&field_id));
        assert!(actual.is_none());
    }

    #[test]
    fn invalid_expression_source_does_not_execute_as_if_valid() {
        let view_id = Uuid::new_v4();
        let widget = WidgetId(41);
        let field_id = Uuid::new_v4();
        let target = custom_target(view_id, field_id, "result");

        let row = WidgetEventRow {
            id: Uuid::new_v4(),
            event_type: "on_press".to_string(),
            target: Some((view_id, widget.0)),
        };
        let mut flow = AppFlow::new(
            "invalid_expr".to_string(),
            FlowTrigger::WidgetEvent { rows: vec![row] },
        );
        flow.sync_trigger_topology();

        let conditional =
            ActionNodeData::new(2, ActionNodeKind::Conditional, Point::new(320.0, 120.0));
        let enum_lit = ActionNodeData::new(
            3,
            ActionNodeKind::EnumLiteral {
                enum_name: None,
                variant: None,
            },
            Point::new(120.0, 220.0),
        );
        let write_node = set_state_node(4, target, "should_not_run");

        flow.graph
            .nodes
            .extend([conditional.clone(), enum_lit.clone(), write_node.clone()]);
        flow.graph.z_order.extend([2, 3, 4]);
        flow.graph.next_id = 5;

        assert!(flow.graph.connect_ports(
            1,
            trigger_flow_out(&flow),
            2,
            port_in(&conditional, "flow_in"),
        ));
        assert!(flow.graph.connect_ports(
            3,
            port_out(&enum_lit, "value"),
            2,
            port_in(&conditional, "condition"),
        ));
        assert!(flow.graph.connect_ports(
            2,
            port_out(&conditional, "true"),
            4,
            set_state_flow_in(&write_node),
        ));

        let mut all_views = one_view(view_id);
        let _ = execute_event(
            &[flow],
            &mut all_views,
            view_id,
            widget,
            "on_press",
            HashMap::new(),
        );

        let actual = all_views
            .get(&view_id)
            .and_then(|v| v.custom_state_values.get(&field_id));
        assert!(actual.is_none());
    }
}

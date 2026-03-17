use std::collections::HashMap;
use uuid::Uuid;

use super::builder::{CodeBuilder, handle_whitespace, to_pascal_case};
use crate::action_system::graph::{ActionNodeData, ActionNodeId};
use crate::action_system::node_kinds::{ActionNodeKind, ActionValue, NavigateTarget};
use crate::action_system::semantic::{LoweredActionGraph, LoweredExpression};
use crate::data_structures::types::types::WidgetId;

#[derive(Debug, Clone, Default)]
pub struct ViewReferenceSelectionCodegen {
    pub field_name: String,
    pub selection_type: String,
    pub variants_by_view: HashMap<Uuid, String>,
}

pub type ViewReferenceSelectionIndex = HashMap<(Uuid, WidgetId), ViewReferenceSelectionCodegen>;

/// Generates the body of an event match arm from the given action graph.
///
/// `current_view_id` is the UUID of the view being generated.
/// `all_names` maps `(view_id, widget_id)` → snake_case widget name.
/// `view_names` maps `view_id` → display name (for NavigateToView).
/// `is_main` — true for the root App (main.rs). NavigateToView sets `self.current_view` in main;
/// in non-main views it returns `Task::done(Message::NavigateTo(View::X))` instead.
pub fn generate_action_graph_body(
    b: &mut CodeBuilder,
    graph: &LoweredActionGraph,
    current_view_id: Uuid,
    view_names: &HashMap<Uuid, String>,
    all_names: &HashMap<(Uuid, WidgetId), String>,
    view_field_names: &HashMap<Uuid, String>,
    view_reference_selection_index: &ViewReferenceSelectionIndex,
    is_main: bool,
    trigger_output_slot: usize,
    callable_method_names: &HashMap<Uuid, String>,
) {
    // Walk flow from the trigger's matched output slot (per-row port)
    walk_flow(
        b,
        graph,
        graph.trigger_node_id(),
        trigger_output_slot,
        0,
        current_view_id,
        view_names,
        all_names,
        view_field_names,
        view_reference_selection_index,
        is_main,
        callable_method_names,
    );
}

/// Recursively walk flow edges, emitting statements for each visited node.
fn walk_flow(
    b: &mut CodeBuilder,
    graph: &LoweredActionGraph,
    from_node: ActionNodeId,
    output_slot: usize,
    depth: usize,
    current_view_id: Uuid,
    view_names: &HashMap<Uuid, String>,
    all_names: &HashMap<(Uuid, WidgetId), String>,
    view_field_names: &HashMap<Uuid, String>,
    view_reference_selection_index: &ViewReferenceSelectionIndex,
    is_main: bool,
    callable_method_names: &HashMap<Uuid, String>,
) {
    if depth > 100 {
        b.line("// Action graph: depth limit reached");
        return;
    }

    let successors = graph.flow_successors_with_input_slots(from_node, output_slot);
    if successors.is_empty() {
        return; // End of this flow path
    }

    for (next_id, input_slot) in successors {
        let Some(next_node) = graph.node(next_id) else {
            continue;
        };
        emit_node_statement(
            b,
            graph,
            next_node,
            input_slot,
            depth,
            current_view_id,
            view_names,
            all_names,
            view_field_names,
            view_reference_selection_index,
            is_main,
            callable_method_names,
        );
    }
}

fn emit_node_statement(
    b: &mut CodeBuilder,
    graph: &LoweredActionGraph,
    node: &ActionNodeData,
    input_slot: usize,
    depth: usize,
    current_view_id: Uuid,
    view_names: &HashMap<Uuid, String>,
    all_names: &HashMap<(Uuid, WidgetId), String>,
    view_field_names: &HashMap<Uuid, String>,
    view_reference_selection_index: &ViewReferenceSelectionIndex,
    is_main: bool,
    callable_method_names: &HashMap<Uuid, String>,
) {
    match &node.kind {
        ActionNodeKind::StateMutation { assignments } => {
            for (idx, assignment) in assignments.iter().enumerate() {
                if let Some(target) = &assignment.target {
                    if target.view_id != current_view_id
                        && !view_field_names.contains_key(&target.view_id)
                    {
                        b.line("// Cross-parent StateMutation: intercepted in parent's ViewMessages arm");
                    } else {
                        let input_label = format!("value_{idx}");
                        let value_expr = graph
                            .expression_for_input_label(node.id, &input_label)
                            .map(|expr| {
                                emit_lowered_expression(
                                    expr,
                                    current_view_id,
                                    all_names,
                                    view_field_names,
                                )
                            })
                            .unwrap_or_else(|| {
                                format!("/* missing lowered expression for '{}' */", input_label)
                            });
                        let field_path =
                            target.rust_path(current_view_id, all_names, view_field_names);
                        b.line(&format!("{} = {};", field_path, value_expr));
                    }
                }
            }
            walk_flow(
                b,
                graph,
                node.id,
                0,
                depth + 1,
                current_view_id,
                view_names,
                all_names,
                view_field_names,
                view_reference_selection_index,
                is_main,
                callable_method_names,
            );
        }

        ActionNodeKind::NavigateToView { .. } => {
            // input_slot tells us which row's flow_in was wired to reach this node.
            if let Some(target) = graph.navigation_target_for_input_slot(node.id, input_slot) {
                match target {
                    NavigateTarget::AppView { view_id } => {
                        if let Some(name) = view_names.get(view_id) {
                            let variant = to_pascal_case(&handle_whitespace(name));
                            if is_main {
                                b.line(&format!("self.current_view = View::{};", variant));
                            } else {
                                b.line(&format!(
                                    "return Task::done(Message::NavigateTo(View::{}));",
                                    variant
                                ));
                            }
                        } else {
                            b.line(&format!("// NavigateToView: unknown app view {}", view_id));
                        }
                    }
                    NavigateTarget::ViewReference {
                        owner_view_id,
                        widget_id,
                        target_view_id,
                    } => {
                        if !is_main
                            && *owner_view_id != current_view_id
                            && !view_field_names.contains_key(owner_view_id)
                        {
                            b.line("// ViewChange captured in main.rs");
                            walk_flow(
                                b,
                                graph,
                                node.id,
                                input_slot,
                                depth + 1,
                                current_view_id,
                                view_names,
                                all_names,
                                view_field_names,
                                view_reference_selection_index,
                                is_main,
                                callable_method_names,
                            );
                            return;
                        }
                        let Some(selection) =
                            view_reference_selection_index.get(&(*owner_view_id, *widget_id))
                        else {
                            b.line(&format!(
                                "// NavigateToView: unresolved ViewReference ({}, {})",
                                owner_view_id, widget_id.0
                            ));
                            walk_flow(
                                b,
                                graph,
                                node.id,
                                input_slot,
                                depth + 1,
                                current_view_id,
                                view_names,
                                all_names,
                                view_field_names,
                                view_reference_selection_index,
                                is_main,
                                callable_method_names,
                            );
                            return;
                        };
                        let Some(variant) = selection.variants_by_view.get(target_view_id) else {
                            b.line(&format!(
                                "// NavigateToView: target view {} is not configured for ViewReference ({}, {})",
                                target_view_id, owner_view_id, widget_id.0
                            ));
                            walk_flow(
                                b,
                                graph,
                                node.id,
                                input_slot,
                                depth + 1,
                                current_view_id,
                                view_names,
                                all_names,
                                view_field_names,
                                view_reference_selection_index,
                                is_main,
                                callable_method_names,
                            );
                            return;
                        };
                        let lhs = if *owner_view_id == current_view_id {
                            format!("self.{}_selection", selection.field_name)
                        } else if let Some(owner_field) = view_field_names.get(owner_view_id) {
                            format!("self.{}.{}_selection", owner_field, selection.field_name)
                        } else {
                            b.line(&format!(
                                "// NavigateToView: owner view {} is unreachable from this module",
                                owner_view_id
                            ));
                            walk_flow(
                                b,
                                graph,
                                node.id,
                                input_slot,
                                depth + 1,
                                current_view_id,
                                view_names,
                                all_names,
                                view_field_names,
                                view_reference_selection_index,
                                is_main,
                                callable_method_names,
                            );
                            return;
                        };
                        b.line(&format!(
                            "{} = {}::{};",
                            lhs, selection.selection_type, variant
                        ));
                    }
                }
            } else {
                b.line("// NavigateToView: target view not configured");
            }
            walk_flow(
                b,
                graph,
                node.id,
                input_slot,
                depth + 1,
                current_view_id,
                view_names,
                all_names,
                view_field_names,
                view_reference_selection_index,
                is_main,
                callable_method_names,
            );
        }

        ActionNodeKind::Conditional => {
            let condition = graph
                .expression_for_input_label(node.id, "condition")
                .map(|expr| {
                    emit_lowered_expression(expr, current_view_id, all_names, view_field_names)
                })
                .unwrap_or_else(|| "/* missing condition expression */ false".to_string());
            b.line(&format!("if {} {{", condition));
            b.increase_indent();
            walk_flow(
                b,
                graph,
                node.id,
                0,
                depth + 1,
                current_view_id,
                view_names,
                all_names,
                view_field_names,
                view_reference_selection_index,
                is_main,
                callable_method_names,
            ); // true branch
            b.decrease_indent();
            b.line("} else {");
            b.increase_indent();
            walk_flow(
                b,
                graph,
                node.id,
                1,
                depth + 1,
                current_view_id,
                view_names,
                all_names,
                view_field_names,
                view_reference_selection_index,
                is_main,
                callable_method_names,
            ); // false branch
            b.decrease_indent();
            b.line("}");
        }

        ActionNodeKind::Match { arms, enum_type } => {
            let value_expr = graph
                .expression_for_input_label(node.id, "value")
                .map(|expr| {
                    emit_lowered_expression(expr, current_view_id, all_names, view_field_names)
                })
                .unwrap_or_else(|| "/* missing match value expression */".to_string());
            b.line(&format!("match {} {{", value_expr));
            b.increase_indent();
            let arms = arms.clone();
            let type_prefix = enum_type.as_deref().unwrap_or("");
            for (i, arm) in arms.iter().enumerate() {
                let pattern = if !type_prefix.is_empty() {
                    format!("{}::{}", type_prefix, arm)
                } else {
                    format!("{:?}", arm)
                };
                b.line(&format!("{} => {{", pattern));
                b.increase_indent();
                walk_flow(
                    b,
                    graph,
                    node.id,
                    i,
                    depth + 1,
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
            // Default arm
            b.line("_ => {");
            b.increase_indent();
            walk_flow(
                b,
                graph,
                node.id,
                arms.len(),
                depth + 1,
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
            b.decrease_indent();
            b.line("}");
        }

        ActionNodeKind::CallFlow { flow_id } => {
            if let Some(target_flow_id) = flow_id {
                if let Some(method_name) = callable_method_names.get(target_flow_id) {
                    b.line(&format!("self.{}();", method_name));
                } else {
                    b.line(&format!(
                        "// CallFlow: unresolved callable target {}",
                        target_flow_id
                    ));
                }
            } else {
                b.line("// CallFlow: no callable flow selected");
            }
            walk_flow(
                b,
                graph,
                node.id,
                0,
                depth + 1,
                current_view_id,
                view_names,
                all_names,
                view_field_names,
                view_reference_selection_index,
                is_main,
                callable_method_names,
            );
        }

        ActionNodeKind::StringLiteral { .. }
        | ActionNodeKind::NumberLiteral { .. }
        | ActionNodeKind::BoolLiteral { .. }
        | ActionNodeKind::EnumLiteral { .. }
        | ActionNodeKind::Expression { .. }
        | ActionNodeKind::Compare { .. }
        | ActionNodeKind::LogicAnd
        | ActionNodeKind::LogicOr
        | ActionNodeKind::LogicNot
        | ActionNodeKind::Trigger { .. }
        | ActionNodeKind::SetState { .. }
        | ActionNodeKind::UpdateState { .. }
        | ActionNodeKind::CallAction { .. }
        | ActionNodeKind::LegacyGetState { .. } => {}
    }
}

fn emit_lowered_expression(
    expr: &LoweredExpression,
    current_view_id: Uuid,
    all_names: &HashMap<(Uuid, WidgetId), String>,
    view_field_names: &HashMap<Uuid, String>,
) -> String {
    match expr {
        LoweredExpression::Literal(v) => literal_to_expr(v),
        LoweredExpression::StateField(src) => {
            format!(
                "{}.clone()",
                src.rust_path(current_view_id, all_names, view_field_names)
            )
        }
        LoweredExpression::TriggerInput { name, .. } => name.clone(),
        LoweredExpression::Formula(formula) => formula.clone(),
        LoweredExpression::Compare { operator, lhs, rhs } => {
            let lhs_expr =
                emit_lowered_expression(lhs, current_view_id, all_names, view_field_names);
            if operator.needs_rhs() {
                let rhs_expr = rhs
                    .as_ref()
                    .map(|r| {
                        emit_lowered_expression(r, current_view_id, all_names, view_field_names)
                    })
                    .unwrap_or_else(|| "/* missing rhs */".to_string());
                format!("({})", operator.rust_expr(&lhs_expr, &rhs_expr))
            } else {
                format!("({})", operator.rust_expr(&lhs_expr, ""))
            }
        }
        LoweredExpression::LogicAnd { lhs, rhs } => {
            let lhs_expr =
                emit_lowered_expression(lhs, current_view_id, all_names, view_field_names);
            let rhs_expr =
                emit_lowered_expression(rhs, current_view_id, all_names, view_field_names);
            format!("({lhs_expr} && {rhs_expr})")
        }
        LoweredExpression::LogicOr { lhs, rhs } => {
            let lhs_expr =
                emit_lowered_expression(lhs, current_view_id, all_names, view_field_names);
            let rhs_expr =
                emit_lowered_expression(rhs, current_view_id, all_names, view_field_names);
            format!("({lhs_expr} || {rhs_expr})")
        }
        LoweredExpression::LogicNot { value } => {
            let value_expr =
                emit_lowered_expression(value, current_view_id, all_names, view_field_names);
            format!("!{value_expr}")
        }
    }
}

fn literal_to_expr(v: &ActionValue) -> String {
    v.rust_literal()
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::Point;

    use crate::action_system::flow::{AppFlow, FlowTrigger};
    use crate::action_system::graph::{ActionGraph, ActionNodeData};
    use crate::action_system::node_kinds::{
        AuthoredCondition, AuthoredValueSource, CompareOp, ConditionJoinMode, NavigateTarget,
        ValueSource,
    };
    use crate::action_system::semantic::{
        SemanticValidationContext, validate_and_lower_flow_graph,
    };
    use crate::action_system::state_ref::{ActionValueType, StateFieldRef, StateRefSource};
    use crate::data_structures::types::types::WidgetId;

    fn flow_out(node: &ActionNodeData) -> u64 {
        node.cached_ports_out
            .iter()
            .find(|p| matches!(p.kind, widgets::flow_editor::PortType::Flow))
            .expect("flow out")
            .id
            .0
    }

    fn flow_in(node: &ActionNodeData) -> u64 {
        node.cached_ports_in
            .iter()
            .find(|p| matches!(p.kind, widgets::flow_editor::PortType::Flow))
            .expect("flow in")
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

    fn lower_graph_for_codegen(
        graph: ActionGraph,
        callable_ids: std::collections::HashSet<Uuid>,
    ) -> LoweredActionGraph {
        let mut flow = AppFlow::new("test".to_string(), FlowTrigger::Callable);
        flow.graph = graph;
        let context = SemanticValidationContext {
            callable_flow_ids: &callable_ids,
            known_view_ids: None,
        };
        validate_and_lower_flow_graph(&flow, &context).expect("lowered graph")
    }

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

    #[test]
    fn codegen_walk_emits_fanout_paths_in_insertion_order() {
        let mut graph = ActionGraph::new_with_trigger("on_press");
        let first_flow_id = Uuid::new_v4();
        let second_flow_id = Uuid::new_v4();
        let first = ActionNodeData::new(
            2,
            ActionNodeKind::CallFlow {
                flow_id: Some(first_flow_id),
            },
            Point::new(260.0, 120.0),
        );
        let second = ActionNodeData::new(
            3,
            ActionNodeKind::CallFlow {
                flow_id: Some(second_flow_id),
            },
            Point::new(360.0, 120.0),
        );
        let first_in = flow_in(&first);
        let second_in = flow_in(&second);
        graph.nodes.extend([first, second]);
        graph.z_order.extend([2, 3]);
        graph.next_id = 4;

        let trigger_flow_out = {
            let trigger = graph.nodes.iter().find(|n| n.id == 1).expect("trigger");
            flow_out(trigger)
        };
        assert!(graph.connect_ports(1, trigger_flow_out, 2, first_in));
        assert!(graph.connect_ports(1, trigger_flow_out, 3, second_in));

        let lowered = lower_graph_for_codegen(
            graph,
            std::collections::HashSet::from([first_flow_id, second_flow_id]),
        );
        let callable_methods = HashMap::from([
            (first_flow_id, "call_first".to_string()),
            (second_flow_id, "call_second".to_string()),
        ]);
        let mut b = CodeBuilder::new();
        generate_action_graph_body(
            &mut b,
            &lowered,
            Uuid::new_v4(),
            &HashMap::new(),
            &HashMap::<(Uuid, WidgetId), String>::new(),
            &HashMap::new(),
            &ViewReferenceSelectionIndex::new(),
            true,
            0,
            &callable_methods,
        );
        let code = b.build();
        let first_idx = code.find("self.call_first();").expect("first call");
        let second_idx = code.find("self.call_second();").expect("second call");
        assert!(first_idx < second_idx);
    }

    #[test]
    fn codegen_match_arms_follow_aligned_slots() {
        let mut graph = ActionGraph::new_with_trigger("on_press");
        let arm_flow_id = Uuid::new_v4();
        let default_flow_id = Uuid::new_v4();
        let match_node = ActionNodeData::new(
            2,
            ActionNodeKind::Match {
                arms: vec!["A".to_string()],
                enum_type: None,
            },
            Point::new(260.0, 120.0),
        );
        let literal = ActionNodeData::new(
            3,
            ActionNodeKind::StringLiteral {
                value: "A".to_string(),
            },
            Point::new(140.0, 220.0),
        );
        let arm = ActionNodeData::new(
            4,
            ActionNodeKind::CallFlow {
                flow_id: Some(arm_flow_id),
            },
            Point::new(420.0, 100.0),
        );
        let default = ActionNodeData::new(
            5,
            ActionNodeKind::CallFlow {
                flow_id: Some(default_flow_id),
            },
            Point::new(420.0, 220.0),
        );

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
        let arm_out = match_node
            .cached_ports_out
            .iter()
            .find(|p| p.label == "A")
            .expect("arm out")
            .id
            .0;
        let default_out = match_node
            .cached_ports_out
            .iter()
            .find(|p| p.label == "default")
            .expect("default out")
            .id
            .0;
        let lit_out = literal.cached_ports_out[0].id.0;
        let arm_in = flow_in(&arm);
        let default_in = flow_in(&default);

        graph.nodes.extend([match_node, literal, arm, default]);
        graph.z_order.extend([2, 3, 4, 5]);
        graph.next_id = 6;

        let trigger_flow_out = {
            let trigger = graph.nodes.iter().find(|n| n.id == 1).expect("trigger");
            flow_out(trigger)
        };
        assert!(graph.connect_ports(1, trigger_flow_out, 2, match_flow_in));
        assert!(graph.connect_ports(3, lit_out, 2, match_value_in));
        assert!(graph.connect_ports(2, arm_out, 4, arm_in));
        assert!(graph.connect_ports(2, default_out, 5, default_in));

        let lowered = lower_graph_for_codegen(
            graph,
            std::collections::HashSet::from([arm_flow_id, default_flow_id]),
        );
        let callable_methods = HashMap::from([
            (arm_flow_id, "arm_branch".to_string()),
            (default_flow_id, "default_branch".to_string()),
        ]);
        let mut b = CodeBuilder::new();
        generate_action_graph_body(
            &mut b,
            &lowered,
            Uuid::new_v4(),
            &HashMap::new(),
            &HashMap::<(Uuid, WidgetId), String>::new(),
            &HashMap::new(),
            &ViewReferenceSelectionIndex::new(),
            true,
            0,
            &callable_methods,
        );
        let code = b.build();
        assert!(code.contains("self.arm_branch();"));
        assert!(code.contains("self.default_branch();"));
    }

    #[test]
    fn codegen_conditional_emits_lowered_boolean_expression() {
        let mut graph = ActionGraph::new_with_trigger("on_press");
        let true_flow_id = Uuid::new_v4();
        let false_flow_id = Uuid::new_v4();

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
        let true_node = ActionNodeData::new(
            7,
            ActionNodeKind::CallFlow {
                flow_id: Some(true_flow_id),
            },
            Point::new(760.0, 180.0),
        );
        let false_node = ActionNodeData::new(
            8,
            ActionNodeKind::CallFlow {
                flow_id: Some(false_flow_id),
            },
            Point::new(760.0, 300.0),
        );

        graph.nodes.extend([
            lhs.clone(),
            rhs.clone(),
            compare.clone(),
            not.clone(),
            conditional.clone(),
            true_node.clone(),
            false_node.clone(),
        ]);
        graph.z_order.extend([2, 3, 4, 5, 6, 7, 8]);
        graph.next_id = 9;

        let trigger_flow_out = {
            let trigger = graph.nodes.iter().find(|n| n.id == 1).expect("trigger");
            flow_out(trigger)
        };
        assert!(graph.connect_ports(1, trigger_flow_out, 6, flow_in(&conditional)));
        assert!(graph.connect_ports(2, port_out(&lhs, "value"), 4, port_in(&compare, "value")));
        assert!(graph.connect_ports(3, port_out(&rhs, "value"), 4, port_in(&compare, "rhs")));
        assert!(graph.connect_ports(4, port_out(&compare, "result"), 5, port_in(&not, "value")));
        assert!(graph.connect_ports(
            5,
            port_out(&not, "result"),
            6,
            port_in(&conditional, "condition")
        ));
        assert!(graph.connect_ports(6, port_out(&conditional, "true"), 7, flow_in(&true_node)));
        assert!(graph.connect_ports(6, port_out(&conditional, "false"), 8, flow_in(&false_node)));

        let lowered = lower_graph_for_codegen(
            graph,
            std::collections::HashSet::from([true_flow_id, false_flow_id]),
        );
        let callable_methods = HashMap::from([
            (true_flow_id, "true_branch".to_string()),
            (false_flow_id, "false_branch".to_string()),
        ]);
        let mut b = CodeBuilder::new();
        generate_action_graph_body(
            &mut b,
            &lowered,
            Uuid::new_v4(),
            &HashMap::new(),
            &HashMap::<(Uuid, WidgetId), String>::new(),
            &HashMap::new(),
            &ViewReferenceSelectionIndex::new(),
            true,
            0,
            &callable_methods,
        );
        let code = b.build();
        assert!(code.contains("if !(") || code.contains("if (!("));
        assert!(code.contains("=="));
        assert!(code.contains("self.true_branch();"));
        assert!(code.contains("self.false_branch();"));
    }

    #[test]
    fn codegen_authored_conditional_emits_in_node_email_validation_expression() {
        let mut graph = ActionGraph::new_with_trigger("on_press");
        let true_flow_id = Uuid::new_v4();
        let false_flow_id = Uuid::new_v4();

        let mut conditional =
            ActionNodeData::new(2, ActionNodeKind::Conditional, Point::new(420.0, 180.0));
        conditional.authored_conditions = vec![AuthoredCondition {
            lhs: AuthoredValueSource::TriggerInput {
                name: "email".to_string(),
                value_type: ActionValueType::String,
            },
            operator: CompareOp::IsValidEmail,
            rhs_literal: ActionValue::String(String::new()),
        }];
        let true_node = ActionNodeData::new(
            3,
            ActionNodeKind::CallFlow {
                flow_id: Some(true_flow_id),
            },
            Point::new(620.0, 140.0),
        );
        let false_node = ActionNodeData::new(
            4,
            ActionNodeKind::CallFlow {
                flow_id: Some(false_flow_id),
            },
            Point::new(620.0, 260.0),
        );
        graph
            .nodes
            .extend([conditional.clone(), true_node.clone(), false_node.clone()]);
        graph.z_order.extend([2, 3, 4]);
        graph.next_id = 5;

        let trigger_flow_out = {
            let trigger = graph.nodes.iter().find(|n| n.id == 1).expect("trigger");
            flow_out(trigger)
        };
        assert!(graph.connect_ports(1, trigger_flow_out, 2, flow_in(&conditional)));
        assert!(graph.connect_ports(2, port_out(&conditional, "true"), 3, flow_in(&true_node)));
        assert!(graph.connect_ports(2, port_out(&conditional, "false"), 4, flow_in(&false_node)));

        let lowered = lower_graph_for_codegen(
            graph,
            std::collections::HashSet::from([true_flow_id, false_flow_id]),
        );
        let callable_methods = HashMap::from([
            (true_flow_id, "true_branch".to_string()),
            (false_flow_id, "false_branch".to_string()),
        ]);
        let mut b = CodeBuilder::new();
        generate_action_graph_body(
            &mut b,
            &lowered,
            Uuid::new_v4(),
            &HashMap::new(),
            &HashMap::<(Uuid, WidgetId), String>::new(),
            &HashMap::new(),
            &ViewReferenceSelectionIndex::new(),
            true,
            0,
            &callable_methods,
        );
        let code = b.build();
        assert!(code.contains("let __at = __email.find('@')"));
        assert!(code.contains("self.true_branch();"));
        assert!(code.contains("self.false_branch();"));
    }

    #[test]
    fn codegen_authored_multi_condition_conditional_emits_joined_expression() {
        let mut graph = ActionGraph::new_with_trigger("on_press");
        let true_flow_id = Uuid::new_v4();
        let false_flow_id = Uuid::new_v4();

        let mut conditional =
            ActionNodeData::new(2, ActionNodeKind::Conditional, Point::new(420.0, 180.0));
        conditional.authored_condition_join = ConditionJoinMode::Any;
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
        let true_node = ActionNodeData::new(
            3,
            ActionNodeKind::CallFlow {
                flow_id: Some(true_flow_id),
            },
            Point::new(620.0, 140.0),
        );
        let false_node = ActionNodeData::new(
            4,
            ActionNodeKind::CallFlow {
                flow_id: Some(false_flow_id),
            },
            Point::new(620.0, 260.0),
        );
        graph
            .nodes
            .extend([conditional.clone(), true_node.clone(), false_node.clone()]);
        graph.z_order.extend([2, 3, 4]);
        graph.next_id = 5;
        let trigger_flow_out = {
            let trigger = graph.nodes.iter().find(|n| n.id == 1).expect("trigger");
            flow_out(trigger)
        };
        assert!(graph.connect_ports(1, trigger_flow_out, 2, flow_in(&conditional)));
        assert!(graph.connect_ports(2, port_out(&conditional, "true"), 3, flow_in(&true_node)));
        assert!(graph.connect_ports(2, port_out(&conditional, "false"), 4, flow_in(&false_node)));

        let lowered = lower_graph_for_codegen(
            graph,
            std::collections::HashSet::from([true_flow_id, false_flow_id]),
        );
        let callable_methods = HashMap::from([
            (true_flow_id, "true_branch".to_string()),
            (false_flow_id, "false_branch".to_string()),
        ]);
        let mut b = CodeBuilder::new();
        generate_action_graph_body(
            &mut b,
            &lowered,
            Uuid::new_v4(),
            &HashMap::new(),
            &HashMap::<(Uuid, WidgetId), String>::new(),
            &HashMap::new(),
            &ViewReferenceSelectionIndex::new(),
            true,
            0,
            &callable_methods,
        );
        let code = b.build();
        assert!(code.contains("||"));
        assert!(code.contains("self.true_branch();"));
        assert!(code.contains("self.false_branch();"));
    }

    #[test]
    fn codegen_authored_match_subject_emits_match_without_value_input_edge() {
        let mut graph = ActionGraph::new_with_trigger("on_press");
        let arm_flow_id = Uuid::new_v4();
        let default_flow_id = Uuid::new_v4();

        let mut match_node = ActionNodeData::new(
            2,
            ActionNodeKind::Match {
                arms: vec!["A".to_string()],
                enum_type: None,
            },
            Point::new(320.0, 120.0),
        );
        match_node.authored_match_subject = Some(AuthoredValueSource::Literal(
            ActionValue::String("A".to_string()),
        ));
        let arm_node = ActionNodeData::new(
            3,
            ActionNodeKind::CallFlow {
                flow_id: Some(arm_flow_id),
            },
            Point::new(540.0, 100.0),
        );
        let default_node = ActionNodeData::new(
            4,
            ActionNodeKind::CallFlow {
                flow_id: Some(default_flow_id),
            },
            Point::new(540.0, 220.0),
        );

        graph
            .nodes
            .extend([match_node.clone(), arm_node.clone(), default_node.clone()]);
        graph.z_order.extend([2, 3, 4]);
        graph.next_id = 5;

        let trigger_flow_out = {
            let trigger = graph.nodes.iter().find(|n| n.id == 1).expect("trigger");
            flow_out(trigger)
        };
        assert!(graph.connect_ports(1, trigger_flow_out, 2, flow_in(&match_node)));
        assert!(graph.connect_ports(2, port_out(&match_node, "A"), 3, flow_in(&arm_node)));
        assert!(graph.connect_ports(
            2,
            port_out(&match_node, "default"),
            4,
            flow_in(&default_node)
        ));

        let lowered = lower_graph_for_codegen(
            graph,
            std::collections::HashSet::from([arm_flow_id, default_flow_id]),
        );
        let callable_methods = HashMap::from([
            (arm_flow_id, "arm_branch".to_string()),
            (default_flow_id, "default_branch".to_string()),
        ]);
        let mut b = CodeBuilder::new();
        generate_action_graph_body(
            &mut b,
            &lowered,
            Uuid::new_v4(),
            &HashMap::new(),
            &HashMap::<(Uuid, WidgetId), String>::new(),
            &HashMap::new(),
            &ViewReferenceSelectionIndex::new(),
            true,
            0,
            &callable_methods,
        );
        let code = b.build();
        assert!(code.contains("match String::from(\"A\") {"));
        assert!(code.contains("self.arm_branch();"));
        assert!(code.contains("self.default_branch();"));
    }

    #[test]
    fn codegen_state_mutation_emits_multi_assignment_in_order() {
        let view_id = Uuid::new_v4();
        let field_ordered = Uuid::new_v4();
        let field_from_state = Uuid::new_v4();
        let field_from_port = Uuid::new_v4();
        let field_seed = Uuid::new_v4();
        let seed_ref = custom_target(view_id, field_seed, "seed");

        let mut graph = ActionGraph::new_with_trigger("on_press");
        let mutate = ActionNodeData::new(
            2,
            ActionNodeKind::StateMutation {
                assignments: vec![
                    crate::action_system::node_kinds::StateAssignment {
                        target: Some(custom_target(view_id, field_ordered, "ordered")),
                        value_source: ValueSource::Literal(ActionValue::String(
                            "first".to_string(),
                        )),
                    },
                    crate::action_system::node_kinds::StateAssignment {
                        target: Some(custom_target(view_id, field_ordered, "ordered")),
                        value_source: ValueSource::Literal(ActionValue::String(
                            "second".to_string(),
                        )),
                    },
                    crate::action_system::node_kinds::StateAssignment {
                        target: Some(custom_target(view_id, field_from_state, "from_state")),
                        value_source: ValueSource::StateField(seed_ref),
                    },
                    crate::action_system::node_kinds::StateAssignment {
                        target: Some(custom_target(view_id, field_from_port, "from_port")),
                        value_source: ValueSource::FromPort,
                    },
                ],
            },
            Point::new(260.0, 120.0),
        );
        let literal = ActionNodeData::new(
            3,
            ActionNodeKind::StringLiteral {
                value: "port_value".to_string(),
            },
            Point::new(100.0, 220.0),
        );
        let mutate_in = flow_in(&mutate);
        let from_port_in = mutate
            .cached_ports_in
            .iter()
            .find(|p| p.label == "value_3")
            .expect("value_3 input")
            .id
            .0;
        let literal_out = literal.cached_ports_out[0].id.0;
        graph.nodes.extend([mutate, literal]);
        graph.z_order.extend([2, 3]);
        graph.next_id = 4;

        let trigger_flow_out = {
            let trigger = graph.nodes.iter().find(|n| n.id == 1).expect("trigger");
            flow_out(trigger)
        };
        assert!(graph.connect_ports(1, trigger_flow_out, 2, mutate_in));
        assert!(graph.connect_ports(3, literal_out, 2, from_port_in));

        let lowered = lower_graph_for_codegen(graph, std::collections::HashSet::new());
        let mut b = CodeBuilder::new();
        generate_action_graph_body(
            &mut b,
            &lowered,
            view_id,
            &HashMap::new(),
            &HashMap::<(Uuid, WidgetId), String>::new(),
            &HashMap::new(),
            &ViewReferenceSelectionIndex::new(),
            true,
            0,
            &HashMap::new(),
        );
        let code = b.build();

        let first_idx = code
            .find("self.ordered = String::from(\"first\");")
            .expect("first ordered write");
        let second_idx = code
            .find("self.ordered = String::from(\"second\");")
            .expect("second ordered write");
        let from_state_idx = code
            .find("self.from_state = self.seed.clone();")
            .expect("from_state write");
        let from_port_idx = code
            .find("self.from_port = String::from(\"port_value\");")
            .expect("from_port write");

        assert!(first_idx < second_idx);
        assert!(second_idx < from_state_idx);
        assert!(from_state_idx < from_port_idx);
    }

    #[test]
    fn codegen_navigate_app_view_emits_top_level_view_switch() {
        let destination_view_id = Uuid::new_v4();
        let mut graph = ActionGraph::new_with_trigger("on_press");
        let navigate = ActionNodeData::new(
            2,
            ActionNodeKind::NavigateToView {
                targets: vec![Some(NavigateTarget::AppView {
                    view_id: destination_view_id,
                })],
            },
            Point::new(260.0, 120.0),
        );
        graph.nodes.push(navigate.clone());
        graph.z_order.push(2);
        graph.next_id = 3;
        let trigger_flow_out = {
            let trigger = graph.nodes.iter().find(|n| n.id == 1).expect("trigger");
            flow_out(trigger)
        };
        assert!(graph.connect_ports(1, trigger_flow_out, 2, flow_in(&navigate),));

        let lowered = lower_graph_for_codegen(graph, std::collections::HashSet::new());
        let mut b = CodeBuilder::new();
        let view_names = HashMap::from([(destination_view_id, "View 2".to_string())]);
        generate_action_graph_body(
            &mut b,
            &lowered,
            Uuid::new_v4(),
            &view_names,
            &HashMap::<(Uuid, WidgetId), String>::new(),
            &HashMap::new(),
            &ViewReferenceSelectionIndex::new(),
            true,
            0,
            &HashMap::new(),
        );
        let code = b.build();
        assert!(code.contains("self.current_view = View::View2;"));
    }

    #[test]
    fn codegen_navigate_view_reference_emits_canonical_selection_assignment() {
        let owner_view_id = Uuid::new_v4();
        let primary_view_id = Uuid::new_v4();
        let secondary_view_id = Uuid::new_v4();
        let widget_id = WidgetId(81);

        let mut graph = ActionGraph::new_with_trigger("on_press");
        let navigate = ActionNodeData::new(
            2,
            ActionNodeKind::NavigateToView {
                targets: vec![Some(NavigateTarget::ViewReference {
                    owner_view_id,
                    widget_id,
                    target_view_id: secondary_view_id,
                })],
            },
            Point::new(260.0, 120.0),
        );
        graph.nodes.push(navigate.clone());
        graph.z_order.push(2);
        graph.next_id = 3;
        let trigger_flow_out = {
            let trigger = graph.nodes.iter().find(|n| n.id == 1).expect("trigger");
            flow_out(trigger)
        };
        assert!(graph.connect_ports(1, trigger_flow_out, 2, flow_in(&navigate),));

        let lowered = lower_graph_for_codegen(graph, std::collections::HashSet::new());
        let mut selection_index = ViewReferenceSelectionIndex::new();
        selection_index.insert(
            (owner_view_id, widget_id),
            ViewReferenceSelectionCodegen {
                field_name: "main_pane".to_string(),
                selection_type: "MainPaneSelection".to_string(),
                variants_by_view: HashMap::from([
                    (primary_view_id, "Primary".to_string()),
                    (secondary_view_id, "Secondary".to_string()),
                ]),
            },
        );

        let mut b = CodeBuilder::new();
        generate_action_graph_body(
            &mut b,
            &lowered,
            owner_view_id,
            &HashMap::new(),
            &HashMap::<(Uuid, WidgetId), String>::new(),
            &HashMap::new(),
            &selection_index,
            true,
            0,
            &HashMap::new(),
        );
        let code = b.build();
        assert!(code.contains("self.main_pane_selection = MainPaneSelection::Secondary;"));
    }

    #[test]
    fn codegen_navigate_parent_view_reference_emits_main_intercept_comment() {
        let parent_view_id = Uuid::new_v4();
        let child_view_id = Uuid::new_v4();
        let secondary_view_id = Uuid::new_v4();
        let widget_id = WidgetId(81);

        let mut graph = ActionGraph::new_with_trigger("on_press");
        let navigate = ActionNodeData::new(
            2,
            ActionNodeKind::NavigateToView {
                targets: vec![Some(NavigateTarget::ViewReference {
                    owner_view_id: parent_view_id,
                    widget_id,
                    target_view_id: secondary_view_id,
                })],
            },
            Point::new(260.0, 120.0),
        );
        graph.nodes.push(navigate.clone());
        graph.z_order.push(2);
        graph.next_id = 3;
        let trigger_flow_out = {
            let trigger = graph.nodes.iter().find(|n| n.id == 1).expect("trigger");
            flow_out(trigger)
        };
        assert!(graph.connect_ports(1, trigger_flow_out, 2, flow_in(&navigate),));

        let lowered = lower_graph_for_codegen(graph, std::collections::HashSet::new());
        let mut b = CodeBuilder::new();
        generate_action_graph_body(
            &mut b,
            &lowered,
            child_view_id,
            &HashMap::new(),
            &HashMap::<(Uuid, WidgetId), String>::new(),
            &HashMap::new(),
            &ViewReferenceSelectionIndex::new(),
            false,
            0,
            &HashMap::new(),
        );
        let code = b.build();

        assert!(code.contains("// ViewChange captured in main.rs"));
        assert!(!code.contains("// NavigateToView: unresolved ViewReference"));
    }
}

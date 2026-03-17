use crate::action_system::graph::ActionGraph;
use crate::action_system::node_kinds::{ActionNodeKind, StateAssignment};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

/// One row in a WidgetEvent trigger: independently selects an event type + a target widget.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WidgetEventRow {
    pub id: Uuid,
    /// Event name, e.g. "on_press".
    pub event_type: String,
    /// `(view_id, widget_id.0)`, `None` when not yet selected.
    pub target: Option<(Uuid, usize)>,
}

impl WidgetEventRow {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            event_type: "on_press".to_string(),
            target: None,
        }
    }
}

/// A flow — one unit of behavior triggered by an event, timer, key combo, startup, or callable entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppFlow {
    pub id: Uuid,
    pub name: String,
    pub trigger: FlowTrigger,
    pub graph: ActionGraph,
    pub enabled: bool,
}

/// How a flow is triggered.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FlowTrigger {
    /// Triggered by widget events. Each row independently selects an event + target widget.
    WidgetEvent { rows: Vec<WidgetEventRow> },
    /// Triggered at a repeating interval.
    Timer { interval_ms: u64 },
    /// Triggered by a keyboard shortcut.
    KeyCombo {
        ctrl: bool,
        shift: bool,
        alt: bool,
        key: String,
    },
    /// Triggered once when the app starts.
    AppStartup,
    /// Callable flow (invoked via `CallFlow`) — not triggered automatically.
    #[serde(alias = "Named")]
    Callable,
}

impl FlowTrigger {
    pub fn kind_label(&self) -> &'static str {
        match self {
            Self::WidgetEvent { .. } => "Widget Event",
            Self::Timer { .. } => "Timer",
            Self::KeyCombo { .. } => "Key Combo",
            Self::AppStartup => "App Startup",
            Self::Callable => "Callable Flow",
        }
    }

    /// All selectable trigger kinds (for picker lists).
    pub fn all_kinds() -> Vec<Self> {
        vec![
            Self::WidgetEvent {
                rows: vec![WidgetEventRow::new()],
            },
            Self::Timer { interval_ms: 1000 },
            Self::KeyCombo {
                ctrl: false,
                shift: false,
                alt: false,
                key: String::new(),
            },
            Self::AppStartup,
            Self::Callable,
        ]
    }
}

impl std::fmt::Display for FlowTrigger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind_label())
    }
}

impl AppFlow {
    pub fn new(name: String, trigger: FlowTrigger) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            trigger,
            graph: ActionGraph::new_named_action(),
            enabled: true,
        }
    }

    /// Canonical trigger-node topology sync for this flow.
    /// Rebuilds trigger ports for the current trigger kind and prunes stale trigger edges.
    pub fn sync_trigger_topology(&mut self) {
        let trigger_kind_label = self.trigger.kind_label().to_string();
        let trigger_event_name = match &self.trigger {
            FlowTrigger::WidgetEvent { .. } => "widget_event".to_string(),
            FlowTrigger::Timer { .. } => "timer".to_string(),
            FlowTrigger::KeyCombo { .. } => "key_combo".to_string(),
            FlowTrigger::AppStartup => "app_startup".to_string(),
            FlowTrigger::Callable => "entry".to_string(),
        };

        let (trigger_id, valid_in_ports, valid_out_ports) = {
            let Some(trigger_node) = self.graph.nodes.iter_mut().find(|n| n.is_trigger()) else {
                return;
            };

            if !matches!(trigger_node.kind, ActionNodeKind::Trigger { .. }) {
                trigger_node.kind = ActionNodeKind::Trigger {
                    event_name: trigger_event_name.clone(),
                    // Non-widget trigger kinds currently expose no payload ports.
                    output_ports: Vec::new(),
                };
            }

            if let ActionNodeKind::Trigger {
                event_name,
                output_ports,
            } = &mut trigger_node.kind
            {
                *event_name = trigger_event_name;
                // Non-widget trigger kinds currently expose no payload ports.
                output_ports.clear();
            }

            match &self.trigger {
                FlowTrigger::WidgetEvent { rows } => {
                    trigger_node.rebuild_ports_for_widget_event(rows)
                }
                _ => trigger_node.rebuild_ports(),
            }
            trigger_node.kind_label_override = Some(trigger_kind_label);

            let valid_in_ports: HashSet<u64> = trigger_node
                .cached_ports_in
                .iter()
                .map(|p| p.id.0)
                .collect();
            let valid_out_ports: HashSet<u64> = trigger_node
                .cached_ports_out
                .iter()
                .map(|p| p.id.0)
                .collect();
            (trigger_node.id, valid_in_ports, valid_out_ports)
        };

        self.graph.edges.retain(|e| {
            let from_ok = if e.from_node == trigger_id {
                valid_out_ports.contains(&e.from_port)
            } else {
                true
            };
            let to_ok = if e.to_node == trigger_id {
                valid_in_ports.contains(&e.to_port)
            } else {
                true
            };
            from_ok && to_ok
        });
    }
}

fn callable_flow_id_index(flows: &[AppFlow]) -> HashMap<String, Option<Uuid>> {
    let mut by_name: HashMap<String, Option<Uuid>> = HashMap::new();
    for flow in flows
        .iter()
        .filter(|f| f.enabled && matches!(f.trigger, FlowTrigger::Callable))
    {
        by_name
            .entry(flow.name.clone())
            .and_modify(|entry| *entry = None)
            .or_insert(Some(flow.id));
    }
    by_name
}

fn normalize_legacy_node_kinds(flows: &mut [AppFlow]) {
    let callable_by_name = callable_flow_id_index(flows);
    for flow in flows {
        for node in &mut flow.graph.nodes {
            let replacement = match &node.kind {
                ActionNodeKind::SetState {
                    target,
                    value_source,
                } => Some(ActionNodeKind::StateMutation {
                    assignments: vec![StateAssignment {
                        target: target.clone(),
                        value_source: value_source.clone(),
                    }],
                }),
                ActionNodeKind::UpdateState { assignments } => {
                    Some(ActionNodeKind::StateMutation {
                        assignments: assignments.clone(),
                    })
                }
                ActionNodeKind::CallAction { action_name } => Some(ActionNodeKind::CallFlow {
                    flow_id: action_name
                        .as_ref()
                        .and_then(|name| callable_by_name.get(name).copied().flatten()),
                }),
                _ => None,
            };
            if let Some(kind) = replacement {
                node.kind = kind;
            }
        }
    }
}

/// Rebuilds transient graph caches for all flows and applies canonical trigger topology sync.
pub fn rebuild_cached_graph_ports(flows: &mut [AppFlow]) {
    normalize_legacy_node_kinds(flows);
    for flow in flows {
        for node in &mut flow.graph.nodes {
            node.rebuild_ports();
        }
        flow.sync_trigger_topology();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::action_system::graph::{ActionEdge, ActionNodeData};
    use crate::action_system::node_kinds::{ActionValue, StateAssignment, ValueSource};
    use iced::Point;

    #[test]
    fn sync_trigger_topology_rebuilds_widget_event_ports_and_prunes_stale_trigger_edges() {
        let view_id = Uuid::new_v4();
        let row = WidgetEventRow {
            id: Uuid::new_v4(),
            event_type: "on_press".to_string(),
            target: Some((view_id, 1)),
        };
        let mut flow = AppFlow::new(
            "flow".to_string(),
            FlowTrigger::WidgetEvent {
                rows: vec![row.clone()],
            },
        );

        let set_state = ActionNodeData::new(
            2,
            ActionNodeKind::StateMutation {
                assignments: vec![StateAssignment {
                    target: None,
                    value_source: ValueSource::Literal(ActionValue::String(String::new())),
                }],
            },
            Point::new(280.0, 120.0),
        );
        let set_state_in = set_state
            .cached_ports_in
            .iter()
            .find(|p| p.label == "flow_in")
            .expect("setstate flow_in")
            .id
            .0;
        flow.graph.nodes.push(set_state);
        flow.graph.z_order.push(2);
        flow.graph.next_id = 3;

        // Stale edge from old generic trigger flow_out port (1001).
        flow.graph.edges.push(ActionEdge {
            from_node: 1,
            from_port: 1001,
            to_node: 2,
            to_port: set_state_in,
        });

        flow.sync_trigger_topology();

        let trigger = flow
            .graph
            .nodes
            .iter()
            .find(|n| n.is_trigger())
            .expect("trigger node");
        assert_eq!(trigger.cached_ports_out.len(), 1);
        assert_eq!(trigger.cached_ports_out[0].label, "on_press");
        let expected_port_id = 1 * 1_000 + 500 + (row.id.as_u128() as u64 & 0xFFF);
        assert_eq!(trigger.cached_ports_out[0].id.0, expected_port_id);
        assert_eq!(trigger.kind_label_override.as_deref(), Some("Widget Event"));
        assert!(flow.graph.edges.is_empty());
    }

    #[test]
    fn rebuild_cached_graph_ports_applies_trigger_sync_for_all_flows() {
        let view_id = Uuid::new_v4();
        let row = WidgetEventRow {
            id: Uuid::new_v4(),
            event_type: "on_toggle".to_string(),
            target: Some((view_id, 3)),
        };
        let mut widget_flow = AppFlow::new(
            "widget".to_string(),
            FlowTrigger::WidgetEvent { rows: vec![row] },
        );
        let mut callable_flow = AppFlow::new("callable".to_string(), FlowTrigger::Callable);

        for node in &mut widget_flow.graph.nodes {
            node.cached_ports_in.clear();
            node.cached_ports_out.clear();
            node.cached_height = 0.0;
        }
        for node in &mut callable_flow.graph.nodes {
            node.cached_ports_in.clear();
            node.cached_ports_out.clear();
            node.cached_height = 0.0;
        }

        let mut flows = vec![widget_flow, callable_flow];
        rebuild_cached_graph_ports(&mut flows);

        let widget_trigger = flows[0]
            .graph
            .nodes
            .iter()
            .find(|n| n.is_trigger())
            .expect("widget trigger");
        assert_eq!(widget_trigger.cached_ports_out.len(), 1);
        assert_eq!(widget_trigger.cached_ports_out[0].label, "on_toggle");
        assert_eq!(
            widget_trigger.kind_label_override.as_deref(),
            Some("Widget Event")
        );

        let callable_trigger = flows[1]
            .graph
            .nodes
            .iter()
            .find(|n| n.is_trigger())
            .expect("callable trigger");
        assert_eq!(callable_trigger.cached_ports_out.len(), 1);
        assert_eq!(callable_trigger.cached_ports_out[0].label, "flow_out");
        assert_eq!(
            callable_trigger.kind_label_override.as_deref(),
            Some("Callable Flow")
        );
    }

    #[test]
    fn rebuild_cached_graph_ports_converts_legacy_call_action_to_call_flow_id() {
        let callable_id = Uuid::new_v4();
        let mut callable = AppFlow::new("Callable".to_string(), FlowTrigger::Callable);
        callable.id = callable_id;

        let mut caller = AppFlow::new("Caller".to_string(), FlowTrigger::Callable);
        caller.graph.nodes.push(ActionNodeData::new(
            2,
            ActionNodeKind::CallAction {
                action_name: Some("Callable".to_string()),
            },
            Point::new(260.0, 120.0),
        ));
        caller.graph.z_order.push(2);
        caller.graph.next_id = 3;

        let mut flows = vec![callable, caller];
        rebuild_cached_graph_ports(&mut flows);

        let caller_node = flows[1]
            .graph
            .nodes
            .iter()
            .find(|n| n.id == 2)
            .expect("caller node");
        match &caller_node.kind {
            ActionNodeKind::CallFlow { flow_id } => assert_eq!(*flow_id, Some(callable_id)),
            other => panic!("expected CallFlow, got {other:?}"),
        }

        // Rename callable flow after conversion; stable id reference must remain valid.
        flows[0].name = "Callable Renamed".to_string();
        rebuild_cached_graph_ports(&mut flows);
        let caller_node_after_rename = flows[1]
            .graph
            .nodes
            .iter()
            .find(|n| n.id == 2)
            .expect("caller node after rename");
        match &caller_node_after_rename.kind {
            ActionNodeKind::CallFlow { flow_id } => assert_eq!(*flow_id, Some(callable_id)),
            other => panic!("expected CallFlow, got {other:?}"),
        }
    }
}

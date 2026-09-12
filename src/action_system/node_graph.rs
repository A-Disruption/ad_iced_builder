//! Renders an [`ActionGraph`] with the `iced_nodegraph` widget.
//!
//! This replaces the vendored `flow_editor`. The two share an architecture —
//! the host owns the graph and the widget reports intent — so the mapping is
//! mostly a matter of vocabulary.
//!
//! The one real difference is worth stating, because it deletes a whole class
//! of bug: `flow_editor` needed every port's pixel offset computed up front
//! (`port_y_offset`, and a page of constants in [`super::graph`] that had to
//! stay in step with both the widget's hit-testing and the node bodies here).
//! `iced_nodegraph` derives a pin's position from where its wrapper lands in
//! the node's own layout, so the offsets — and the constants — are gone.

use iced::widget::{column, container, row, text};
use iced::{Alignment, Element, Length, Point};
use iced_nodegraph::prelude::*;

use super::flow_model::{PortDef, PortSide, PortType};
use super::graph::{ActionEdge, ActionGraph, ActionNodeData, ActionNodeId, node_width};

/// The id vocabulary the action editor speaks to `iced_nodegraph`.
///
/// `Payload` is the port's own [`PortType`], which is what makes
/// [`can_connect`] a one-liner: a flow port only takes a flow port, and a data
/// port only takes data of the same type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ActionIds;

impl Ids for ActionIds {
    type NodeId = ActionNodeId;
    type PinId = u64;
    type EdgeId = EdgeKey;
    type AnchorId = usize;
    type Payload = PortType;
}

/// An edge named by its endpoints.
///
/// [`ActionEdge`] has no id of its own, and giving it one would have to be
/// persisted. The endpoints already identify it uniquely — two ports cannot be
/// wired twice — so they are the key.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EdgeKey {
    pub from_node: ActionNodeId,
    pub from_port: u64,
    pub to_node: ActionNodeId,
    pub to_port: u64,
}

impl EdgeKey {
    pub fn of(edge: &ActionEdge) -> Self {
        Self {
            from_node: edge.from_node,
            from_port: edge.from_port,
            to_node: edge.to_node,
            to_port: edge.to_port,
        }
    }

    pub fn matches(&self, edge: &ActionEdge) -> bool {
        *self == Self::of(edge)
    }
}

/// What the graph reports back.
///
/// Deliberately one variant per intent rather than one `Action` enum: these go
/// straight into the editor's own message type, and a caller reading this list
/// can see everything the widget can ask for.
#[derive(Debug, Clone)]
pub enum GraphMessage {
    Connect(PinRef<ActionIds>, PinRef<ActionIds>),
    Disconnect(PinRef<ActionIds>, PinRef<ActionIds>),
    NodesMoved(iced::Vector, Vec<ActionNodeId>),
    SelectionChanged(Vec<ActionNodeId>),
    NodesDeleted(Vec<ActionNodeId>),
    NodesCloned(Vec<ActionNodeId>),
    EdgesDeleted(Vec<EdgeKey>),
    CameraChanged(Point, f32),
}

/// A port row: its label, and the pin the graph attaches an edge to.
///
/// The pin wraps the dot rather than the whole row, so an edge meets the dot
/// and not the row's outer edge.
fn port_row<'a>(port: &PortDef) -> Element<'a, GraphMessage> {
    let dot = text("\u{25CF}").size(10);
    let label = text(port.label.clone()).size(12);

    match port.side {
        PortSide::Input => row![
            pin!(Left, port.id.0, dot, Input, port.kind.clone()),
            label,
        ],
        PortSide::Output => row![
            label,
            pin!(Right, port.id.0, dot, Output, port.kind.clone()),
        ],
    }
    .spacing(6)
    .align_y(Alignment::Center)
    .into()
}

/// One node: a header and its ports.
///
/// Inputs and outputs are separate columns so a node with three of one and one
/// of the other still reads as two lists rather than a ragged pairing.
fn node_content<'a>(node: &ActionNodeData) -> Element<'a, GraphMessage> {
    let header = container(text(node.kind.kind_label()).size(13))
        .padding([4, 8])
        .width(Length::Fill);

    let inputs = column(node.cached_ports_in.iter().map(port_row))
        .spacing(4)
        .width(Length::Shrink);

    let outputs = column(node.cached_ports_out.iter().map(port_row))
        .spacing(4)
        .align_x(Alignment::End)
        .width(Length::Shrink);

    container(
        column![
            header,
            row![inputs, iced::widget::space::horizontal(), outputs]
                .spacing(16)
                .padding([4, 8]),
        ]
        .spacing(2),
    )
    .width(Length::Fixed(node_width(&node.kind)))
    .into()
}

/// Builds the widget for a whole [`ActionGraph`].
pub fn view<'a>(graph: &'a ActionGraph) -> Element<'a, GraphMessage> {
    let mut widget = NodeGraph::<'a, ActionIds, GraphMessage>::new()
        .camera(Point::new(graph.pan[0], graph.pan[1]), graph.zoom)
        .on_connect(GraphMessage::Connect)
        .on_disconnect(GraphMessage::Disconnect)
        .on_move(GraphMessage::NodesMoved)
        .on_select(GraphMessage::SelectionChanged)
        .on_delete(GraphMessage::NodesDeleted)
        .on_clone(GraphMessage::NodesCloned)
        .on_edge_delete(GraphMessage::EdgesDeleted)
        .on_camera(GraphMessage::CameraChanged)
        // Outputs wire to inputs, and only where the port types agree — which
        // is the whole of the action system's connection rule.
        .can_connect(|from, to| from.direction() != to.direction() && from.info() == to.info());

    // `z_order` is the draw order the editor maintains, so nodes are pushed in
    // it rather than in `nodes` order.
    for id in &graph.z_order {
        let Some(data) = graph.nodes.iter().find(|node| node.id == *id) else {
            continue;
        };

        widget = widget.push_node(node(*id, data.position, node_content(data)));
    }

    for wire in &graph.edges {
        widget = widget.push_edge(edge(
            EdgeKey::of(wire),
            PinRef::new(wire.from_node, wire.from_port),
            PinRef::new(wire.to_node, wire.to_port),
        ));
    }

    widget.into()
}

/// Applies an edit the graph reported to the flow's own model.
///
/// The widget never mutates the graph — it says what the user asked for, and
/// this is the only place that acts on it. Returns whether anything changed,
/// so the caller knows when to regenerate code.
pub fn update(graph: &mut ActionGraph, message: GraphMessage) -> bool {
    match message {
        GraphMessage::Connect(from, to) => {
            // The widget has already checked direction and type through
            // `can_connect`; what it cannot know is that a flow input takes
            // one edge, so an existing one is replaced rather than doubled.
            graph.edges.retain(|edge| {
                !(edge.to_node == to.node_id && edge.to_port == to.pin_id)
            });

            graph.edges.push(ActionEdge {
                from_node: from.node_id,
                from_port: from.pin_id,
                to_node: to.node_id,
                to_port: to.pin_id,
            });

            true
        }

        GraphMessage::Disconnect(from, to) => {
            let before = graph.edges.len();

            graph.edges.retain(|edge| {
                !(edge.from_node == from.node_id
                    && edge.from_port == from.pin_id
                    && edge.to_node == to.node_id
                    && edge.to_port == to.pin_id)
            });

            graph.edges.len() != before
        }

        GraphMessage::NodesMoved(delta, ids) => {
            for node in graph.nodes.iter_mut().filter(|node| ids.contains(&node.id)) {
                node.position.x += delta.x;
                node.position.y += delta.y;
            }

            !ids.is_empty()
        }

        GraphMessage::SelectionChanged(ids) => {
            for node in &mut graph.nodes {
                node.selected = ids.contains(&node.id);
            }

            // Selection is not part of the generated output.
            false
        }

        GraphMessage::NodesDeleted(ids) => {
            // A node's edges go with it, or they would point at nothing.
            graph.nodes.retain(|node| !ids.contains(&node.id));
            graph.edges.retain(|edge| {
                !ids.contains(&edge.from_node) && !ids.contains(&edge.to_node)
            });
            graph.z_order.retain(|id| !ids.contains(id));

            !ids.is_empty()
        }

        GraphMessage::EdgesDeleted(keys) => {
            let before = graph.edges.len();
            graph.edges.retain(|edge| !keys.iter().any(|key| key.matches(edge)));

            graph.edges.len() != before
        }

        GraphMessage::NodesCloned(ids) => {
            // Offset so the copy is visibly its own node rather than exactly
            // covering the original.
            const OFFSET: f32 = 24.0;

            let clones: Vec<ActionNodeData> = ids
                .iter()
                .filter_map(|id| graph.nodes.iter().find(|node| node.id == *id))
                .map(|node| {
                    let mut clone = node.clone();
                    clone.id = graph.next_id;
                    clone.position.x += OFFSET;
                    clone.position.y += OFFSET;
                    clone.selected = false;
                    graph.next_id += 1;
                    clone
                })
                .collect();

            let cloned = !clones.is_empty();

            for clone in clones {
                graph.z_order.push(clone.id);
                graph.nodes.push(clone);
            }

            cloned
        }

        GraphMessage::CameraChanged(position, zoom) => {
            graph.pan = [position.x, position.y];
            graph.zoom = zoom;

            // The camera is remembered, but it changes nothing generated.
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pin(node: ActionNodeId, port: u64) -> PinRef<ActionIds> {
        PinRef::new(node, port)
    }

    /// A graph with a trigger and one node, unwired.
    fn graph() -> ActionGraph {
        let mut graph = ActionGraph::new_named_action();
        graph.edges.clear();
        graph
    }

    #[test]
    fn connecting_a_flow_input_replaces_what_was_there() {
        let mut graph = graph();
        let [a, b, target] = [0, 1, 2];

        assert!(update(&mut graph, GraphMessage::Connect(pin(a, 0), pin(target, 0))));
        assert!(update(&mut graph, GraphMessage::Connect(pin(b, 0), pin(target, 0))));

        // One edge into the port, and it is the later one: a flow input takes
        // a single edge.
        assert_eq!(graph.edges.len(), 1);
        assert_eq!(graph.edges[0].from_node, b);
    }

    #[test]
    fn deleting_a_node_takes_its_edges_with_it() {
        let mut graph = graph();

        // Two real nodes: the trigger the graph starts with, and a copy of it.
        let a = graph.nodes[0].id;
        let _ = update(&mut graph, GraphMessage::NodesCloned(vec![a]));
        let b = graph.nodes.last().unwrap().id;

        let _ = update(&mut graph, GraphMessage::Connect(pin(a, 0), pin(b, 0)));
        let before = graph.nodes.len();

        let _ = update(&mut graph, GraphMessage::NodesDeleted(vec![a]));

        assert_eq!(graph.nodes.len(), before - 1);
        assert!(
            graph.edges.is_empty(),
            "an edge outlived the node it was attached to",
        );
    }

    #[test]
    fn an_edge_is_named_by_its_endpoints() {
        let mut graph = graph();
        let _ = update(&mut graph, GraphMessage::Connect(pin(0, 0), pin(1, 0)));

        let key = EdgeKey::of(&graph.edges[0]);
        assert!(update(&mut graph, GraphMessage::EdgesDeleted(vec![key])));
        assert!(graph.edges.is_empty());
    }

    #[test]
    fn cloning_offsets_the_copy_and_gives_it_a_fresh_id() {
        let mut graph = graph();
        let original = graph.nodes[0].id;
        let position = graph.nodes[0].position;
        let before = graph.nodes.len();

        assert!(update(&mut graph, GraphMessage::NodesCloned(vec![original])));
        assert_eq!(graph.nodes.len(), before + 1);

        let clone = graph.nodes.last().unwrap();
        assert_ne!(clone.id, original);
        assert_ne!(clone.position, position);
        assert!(graph.z_order.contains(&clone.id));
    }

    #[test]
    fn moving_and_framing_do_not_regenerate_code() {
        let mut graph = graph();
        let id = graph.nodes[0].id;

        // A move changes the saved project, so it reports a change.
        assert!(update(
            &mut graph,
            GraphMessage::NodesMoved(iced::Vector::new(10.0, 5.0), vec![id]),
        ));

        // Selection and camera are view state, and generate nothing.
        assert!(!update(&mut graph, GraphMessage::SelectionChanged(vec![id])));
        assert!(!update(
            &mut graph,
            GraphMessage::CameraChanged(Point::new(1.0, 2.0), 1.5),
        ));
        assert_eq!(graph.zoom, 1.5);
    }
}

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

use iced::Widget as _;
use iced::widget::{column, container, row, text};
use iced::{Alignment, Element, Length, Point};
use iced_nodegraph::prelude::*;

use super::flow_model::PortType;
use super::node_body::{self, NodeContext, NodeMessage};
use super::flow::{AppFlow, FlowTrigger};
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

/// One node: what it is, and the body carrying its controls and pins.
fn node_content<'a>(node: &'a ActionNodeData, ctx: &'a NodeContext) -> Element<'a, Message> {
    // A trigger says which trigger it is, which the kind label cannot.
    let title = node
        .kind_label_override
        .clone()
        .unwrap_or_else(|| node.kind.display_name().to_string());

    let header = container(text(title).size(12))
        .padding([3, 6])
        .width(Length::Fill);

    container(column![header, node_body::body(node, ctx).map(Message::Node)].spacing(2))
        .width(Length::Fixed(node_width(&node.kind)))
        .boxed()
}

/// Everything the editor shows: the graph, and the palette over it.
///
/// One message type rather than two nested ones, because the caller has to
/// route both to the same flow anyway.
#[derive(Debug, Clone)]
pub enum Message {
    Graph(GraphMessage),
    Palette(PaletteMessage),
    Flow(FlowMessage),
    Node(NodeMessage),
}

/// The action editor: a flow bar, and the flow's graph with right-click to
/// add a node.
pub fn editor<'a>(
    flows: &'a [AppFlow],
    state: &'a EditorState,
    ctx: &'a NodeContext,
) -> Element<'a, Message> {
    let bar = flow_bar(flows, state);

    let Some(flow) = state.selected(flows) else {
        return column![
            bar,
            container(text("No flows yet. Add one to start wiring actions.").size(14))
                .center_x(Length::Fill)
                .center_y(Length::Fill),
        ]
        .boxed();
    };

    let (graph, palette) = (&flow.graph, &state.palette);

    // The graph has no background-click callback, so the right-click that
    // opens the palette is caught out here. `mouse_area` reports the cursor
    // relative to its own bounds, which is the coordinate space
    // `PaletteState::open_at_cursor` undoes the camera in.
    let canvas = iced::widget::mouse_area(view(graph, ctx))
        .on_move(|position| Message::Palette(PaletteMessage::CursorMoved(position)))
        .on_right_press(Message::Palette(PaletteMessage::Opened));

    let Some(open) = &palette.open else {
        return column![bar, canvas].boxed();
    };

    // `layer` positions against the viewport and draws as an overlay, so it
    // sits beside the canvas rather than wrapping it. `at` puts it exactly
    // where the click landed rather than at an anchor.
    let palette = widgets::layer::layer(palette_view(open).map(Message::Palette))
        .at(open.at)
        .dismiss_on_outside_press(true)
        .dismiss_on_escape(true)
        .on_dismiss(Message::Palette(PaletteMessage::Closed))
        .padding(0.0);

    column![bar, iced::widget::stack![canvas, palette]].boxed()
}

/// Builds the widget for a whole [`ActionGraph`].
///
/// Everything the widget reports is a `Message::Graph`; everything a node's
/// own controls report is a `Message::Node`. Both arrive here so the caller
/// has one channel to route.
pub fn view<'a>(graph: &'a ActionGraph, ctx: &'a NodeContext) -> Element<'a, Message> {
    let graphed = |message: GraphMessage| Message::Graph(message);

    let mut widget = NodeGraph::<'a, ActionIds, Message>::new()
        .camera(Point::new(graph.pan[0], graph.pan[1]), graph.zoom)
        .on_connect(move |from, to| graphed(GraphMessage::Connect(from, to)))
        .on_disconnect(move |from, to| graphed(GraphMessage::Disconnect(from, to)))
        .on_move(move |delta, ids| graphed(GraphMessage::NodesMoved(delta, ids)))
        .on_select(move |ids| graphed(GraphMessage::SelectionChanged(ids)))
        .on_delete(move |ids| graphed(GraphMessage::NodesDeleted(ids)))
        .on_clone(move |ids| graphed(GraphMessage::NodesCloned(ids)))
        .on_edge_delete(move |keys| graphed(GraphMessage::EdgesDeleted(keys)))
        .on_camera(move |position, zoom| graphed(GraphMessage::CameraChanged(position, zoom)))
        // Outputs wire to inputs, and only where the port types agree — which
        // is the whole of the action system's connection rule.
        .can_connect(|from, to| from.direction() != to.direction() && from.info() == to.info());

    // `z_order` is the draw order the editor maintains, so nodes are pushed in
    // it rather than in `nodes` order.
    for id in &graph.z_order {
        let Some(data) = graph.nodes.iter().find(|node| node.id == *id) else {
            continue;
        };

        widget = widget.push_node(node(*id, data.position, node_content(data, ctx)));
    }

    for wire in &graph.edges {
        widget = widget.push_edge(edge(
            EdgeKey::of(wire),
            PinRef::new(wire.from_node, wire.from_port),
            PinRef::new(wire.to_node, wire.to_port),
        ));
    }

    widget.boxed()
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

// ─── Context palette ─────────────────────────────────────────────────────────
// `iced_nodegraph` has no palette of its own: it draws and edits a graph, and
// leaves deciding what a new node *is* to the application. So this is built
// here, out of ordinary widgets, and only needs two things from the widget --
// where the click landed and what the camera was doing -- both of which the
// host already knows.

/// Where the palette is open, and what has been typed into it.
///
/// `None` is closed, which is the whole of its state when nothing is happening.
#[derive(Debug, Clone, Default)]
pub struct PaletteState {
    open: Option<OpenPalette>,
    /// The cursor, in the graph widget's own coordinates.
    ///
    /// Tracked on every move because a right-click does not carry a position:
    /// `mouse_area::on_right_press` reports that it happened, not where.
    cursor: Point,
}

#[derive(Debug, Clone)]
struct OpenPalette {
    /// Where to draw it — the widget-relative point the user clicked.
    at: Point,
    /// Where the node goes — that point in world space.
    world: Point,
    filter: String,
}

impl PaletteState {
    pub fn is_open(&self) -> bool {
        self.open.is_some()
    }

    /// Opens the palette at the last known cursor position.
    ///
    /// `mouse_area` reports the cursor relative to its own bounds, and it wraps
    /// the graph exactly, so the widget's viewport origin is the origin here
    /// and the camera's documented transform reduces to this.
    fn open_at_cursor(&mut self, camera: Point, zoom: f32) {
        let world = Point::new(
            self.cursor.x / zoom - camera.x,
            self.cursor.y / zoom - camera.y,
        );

        self.open = Some(OpenPalette {
            at: self.cursor,
            world,
            filter: String::new(),
        });
    }

    fn close(&mut self) {
        self.open = None;
    }
}

/// What the palette reports.
#[derive(Debug, Clone)]
pub enum PaletteMessage {
    CursorMoved(Point),
    Opened,
    Closed,
    FilterChanged(String),
    /// The id of the chosen [`PaletteEntry`].
    Picked(u64),
}

/// Applies a palette message, adding a node when one was chosen.
///
/// Returns whether the graph changed, matching [`update`].
pub fn update_palette(
    graph: &mut ActionGraph,
    palette: &mut PaletteState,
    message: PaletteMessage,
) -> bool {
    match message {
        PaletteMessage::CursorMoved(position) => {
            palette.cursor = position;
            false
        }

        PaletteMessage::Opened => {
            palette.open_at_cursor(Point::new(graph.pan[0], graph.pan[1]), graph.zoom);
            false
        }

        PaletteMessage::Closed => {
            palette.close();
            false
        }

        PaletteMessage::FilterChanged(filter) => {
            if let Some(open) = &mut palette.open {
                open.filter = filter;
            }
            false
        }

        PaletteMessage::Picked(entry) => {
            let Some(open) = palette.open.take() else {
                return false;
            };

            let Some(mut node) = super::graph::action_node_from_palette_id(
                entry,
                open.world,
                graph.next_id,
            ) else {
                return false;
            };

            // Ports are derived from the kind, and a node arrives from the
            // palette with none.
            node.rebuild_ports();

            graph.next_id += 1;
            graph.z_order.push(node.id);
            graph.nodes.push(node);

            true
        }
    }
}

/// The palette itself: a search box over the node kinds that can be added.
fn palette_view<'a>(open: &'a OpenPalette) -> Element<'a, PaletteMessage> {
    let needle = open.filter.to_lowercase();

    let entries: Vec<Element<'a, PaletteMessage>> = super::graph::action_palette_entries()
        .into_iter()
        .filter(|entry| {
            needle.is_empty()
                || entry.label.to_lowercase().contains(&needle)
                || entry.kind_label.to_lowercase().contains(&needle)
        })
        .map(|entry| {
            iced::widget::button(
                row![
                    // The accent is how a kind is recognised at a glance in the
                    // graph, so the palette shows the same one.
                    container(text(""))
                        .width(Length::Fixed(3.0))
                        .height(Length::Fixed(18.0))
                        .style(move |_: &iced::Theme| container::Style {
                            background: Some(entry.accent_color.into()),
                            ..container::Style::default()
                        }),
                    text(entry.label).size(13),
                    iced::widget::space::horizontal(),
                    text(entry.kind_label).size(11),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
            )
            .on_press(PaletteMessage::Picked(entry.id))
            .style(iced::widget::button::text)
            .width(Length::Fill)
            .boxed()
        })
        .collect();

    container(
        column![
            iced::widget::text_input("Search nodes\u{2026}", &open.filter)
                .on_input(PaletteMessage::FilterChanged)
                .size(13),
            column(entries).spacing(2),
        ]
        .spacing(6),
    )
    .padding(8)
    .width(Length::Fixed(240.0))
    .style(container::bordered_box)
    .boxed()
}

#[cfg(test)]
mod palette_tests {
    use super::*;

    #[test]
    fn a_right_click_places_the_node_under_the_cursor() {
        let mut graph = ActionGraph::new_named_action();
        let mut palette = PaletteState::default();

        // A camera panned to (100, 50) at 2x zoom: a click 200px right and
        // 100px down the widget is world (0, 0).
        graph.pan = [100.0, 50.0];
        graph.zoom = 2.0;

        let _ = update_palette(
            &mut graph,
            &mut palette,
            PaletteMessage::CursorMoved(Point::new(200.0, 100.0)),
        );
        let _ = update_palette(&mut graph, &mut palette, PaletteMessage::Opened);

        assert!(palette.is_open());

        let before = graph.nodes.len();
        assert!(update_palette(
            &mut graph,
            &mut palette,
            PaletteMessage::Picked(4), // "If"
        ));

        assert_eq!(graph.nodes.len(), before + 1);
        assert!(!palette.is_open(), "picking should close the palette");

        let added = graph.nodes.last().unwrap();
        assert_eq!(added.position, Point::new(0.0, 0.0));
        assert!(
            !added.cached_ports_out.is_empty(),
            "a node from the palette arrives with no ports until they are rebuilt",
        );
        assert!(graph.z_order.contains(&added.id));
    }

    #[test]
    fn an_unknown_entry_adds_nothing() {
        let mut graph = ActionGraph::new_named_action();
        let mut palette = PaletteState::default();
        let before = graph.nodes.len();

        let _ = update_palette(&mut graph, &mut palette, PaletteMessage::Opened);
        assert!(!update_palette(
            &mut graph,
            &mut palette,
            PaletteMessage::Picked(u64::MAX),
        ));

        assert_eq!(graph.nodes.len(), before);
    }

    #[test]
    fn dismissing_closes_without_adding() {
        let mut graph = ActionGraph::new_named_action();
        let mut palette = PaletteState::default();
        let before = graph.nodes.len();

        let _ = update_palette(&mut graph, &mut palette, PaletteMessage::Opened);
        assert!(palette.is_open());

        assert!(!update_palette(&mut graph, &mut palette, PaletteMessage::Closed));
        assert!(!palette.is_open());
        assert_eq!(graph.nodes.len(), before);
    }
}

// ─── Flow bar ────────────────────────────────────────────────────────────────
// The full flow browser -- named actions, subscriptions, widget-event rows --
// is still in the parked `views::action_editor`. This is the minimum that
// makes the editor usable: somewhere to create a flow, and a way to pick which
// one is on screen.

/// Which flow the editor is showing, and the palette over it.
#[derive(Debug, Clone, Default)]
pub struct EditorState {
    pub palette: PaletteState,
    /// Index into the app's flows. Clamped on use, so removing a flow cannot
    /// leave this pointing past the end.
    selected: usize,
}

impl EditorState {
    /// The flow being edited, if there is one.
    pub fn selected<'f>(&self, flows: &'f [AppFlow]) -> Option<&'f AppFlow> {
        flows.get(self.selected.min(flows.len().saturating_sub(1)))
    }

    /// The flow being edited, to mutate. A trigger edit lands on the flow
    /// rather than on the graph, so this is public alongside the graph getter.
    pub fn selected_mut<'f>(&self, flows: &'f mut [AppFlow]) -> Option<&'f mut AppFlow> {
        let index = self.selected.min(flows.len().saturating_sub(1));
        flows.get_mut(index)
    }

    /// The selected flow's graph, which is what the widget edits.
    pub fn selected_mut_graph<'f>(
        &self,
        flows: &'f mut [AppFlow],
    ) -> Option<&'f mut ActionGraph> {
        self.selected_mut(flows).map(|flow| &mut flow.graph)
    }
}

/// What the flow bar reports.
#[derive(Debug, Clone)]
pub enum FlowMessage {
    Selected(usize),
    Added,
    Renamed(String),
}

/// Applies a flow-bar message. Returns whether the generated code changed.
pub fn update_flows(
    flows: &mut Vec<AppFlow>,
    state: &mut EditorState,
    message: FlowMessage,
) -> bool {
    match message {
        FlowMessage::Selected(index) => {
            state.selected = index;
            // Which flow is on screen is not part of the output.
            false
        }

        FlowMessage::Added => {
            let name = format!("flow_{}", flows.len() + 1);
            flows.push(AppFlow::new(name, FlowTrigger::Callable));
            state.selected = flows.len() - 1;

            true
        }

        FlowMessage::Renamed(name) => {
            let Some(flow) = state.selected_mut(flows) else {
                return false;
            };

            flow.name = name;
            true
        }
    }
}

/// A tab per flow, plus the button that makes one.
fn flow_bar<'a>(flows: &'a [AppFlow], state: &'a EditorState) -> Element<'a, Message> {
    let selected = state.selected.min(flows.len().saturating_sub(1));

    let tabs = flows.iter().enumerate().map(|(index, flow)| {
        iced::widget::button(text(&flow.name).size(12))
            .on_press(Message::Flow(FlowMessage::Selected(index)))
            .style(if index == selected {
                iced::widget::button::primary
            } else {
                iced::widget::button::text
            })
            .boxed()
    });

    let name = state.selected(flows).map(|flow| flow.name.as_str());

    container(
        row![
            row(tabs).spacing(4).wrap(),
            iced::widget::button(text("+ Flow").size(12))
                .on_press(Message::Flow(FlowMessage::Added))
                .style(iced::widget::button::secondary),
            iced::widget::space::horizontal(),
            // Renaming the selected flow, which is what the generated method
            // is named after.
            iced::widget::text_input("flow name", name.unwrap_or_default())
                .on_input(|name| Message::Flow(FlowMessage::Renamed(name)))
                .size(12)
                .width(Length::Fixed(180.0)),
        ]
        .spacing(8)
        .align_y(Alignment::Center),
    )
    .padding(6)
    .width(Length::Fill)
    .boxed()
}

#[cfg(test)]
mod flow_tests {
    use super::*;

    #[test]
    fn adding_a_flow_selects_it() {
        let mut flows: Vec<AppFlow> = Vec::new();
        let mut state = EditorState::default();

        assert!(state.selected(&flows).is_none());

        assert!(update_flows(&mut flows, &mut state, FlowMessage::Added));
        assert!(update_flows(&mut flows, &mut state, FlowMessage::Added));

        assert_eq!(flows.len(), 2);
        assert_eq!(
            state.selected(&flows).map(|flow| flow.name.as_str()),
            Some("flow_2"),
            "a new flow should be the one on screen",
        );
    }

    #[test]
    fn renaming_hits_the_selected_flow() {
        let mut flows: Vec<AppFlow> = Vec::new();
        let mut state = EditorState::default();

        let _ = update_flows(&mut flows, &mut state, FlowMessage::Added);
        let _ = update_flows(&mut flows, &mut state, FlowMessage::Added);
        let _ = update_flows(&mut flows, &mut state, FlowMessage::Selected(0));

        assert!(update_flows(
            &mut flows,
            &mut state,
            FlowMessage::Renamed("on_save".to_string()),
        ));

        assert_eq!(flows[0].name, "on_save");
        assert_eq!(flows[1].name, "flow_2");
    }

    #[test]
    fn a_stale_selection_does_not_index_past_the_end() {
        let mut flows: Vec<AppFlow> = Vec::new();
        let mut state = EditorState::default();

        let _ = update_flows(&mut flows, &mut state, FlowMessage::Added);
        let _ = update_flows(&mut flows, &mut state, FlowMessage::Added);
        let _ = update_flows(&mut flows, &mut state, FlowMessage::Selected(1));

        // Something else removed a flow behind the editor's back.
        flows.pop();

        assert!(state.selected(&flows).is_some());
        assert_eq!(state.selected(&flows).unwrap().name, "flow_1");
    }
}

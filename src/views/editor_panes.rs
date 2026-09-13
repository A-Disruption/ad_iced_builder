//! The editor window's pane layout.
//!
//! The Main view used to be a fixed `row![column![views, palette], code]`, with
//! nothing resizable and no room for a properties panel — which is why editing
//! happened in a modal that covered everything.
//!
//! It is a [`pane_grid`] now: the panes can be resized, dragged into a
//! different arrangement and maximised, and there is a place for the inspector
//! to live. [`PaneKind::Preview`] is the seat the live preview will take when
//! it moves in from the Visualizer window; until then the Visualizer is still
//! where the project is rendered.

use iced::widget::pane_grid::{self, Axis, Configuration, DragEvent, Pane, ResizeEvent};

/// What a pane is showing.
///
/// A kind appears at most once in the grid — these are the editor's fixed
/// surfaces rearranged, not a document workspace where you open many of the
/// same thing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaneKind {
    /// Views and their widget trees.
    Tree,
    /// Properties of whatever is selected.
    Inspector,
    /// The generated code.
    Code,
}

impl PaneKind {
    pub fn title(&self) -> &'static str {
        match self {
            PaneKind::Tree => "Views",
            PaneKind::Inspector => "Inspector",
            PaneKind::Code => "Code",
        }
    }
}

/// How the grid is arranged before anyone drags anything.
///
/// Tree on the left because that is where selection starts, inspector in the
/// middle next to it because that is where selection lands, code on the right
/// because it is the thing you glance at rather than work in.
pub fn default_layout() -> Configuration<PaneKind> {
    Configuration::Split {
        axis: Axis::Vertical,
        ratio: 0.30,
        a: Box::new(Configuration::Pane(PaneKind::Tree)),
        b: Box::new(Configuration::Split {
            axis: Axis::Vertical,
            ratio: 0.46,
            a: Box::new(Configuration::Pane(PaneKind::Inspector)),
            b: Box::new(Configuration::Pane(PaneKind::Code)),
        }),
    }
}

pub fn new_state() -> pane_grid::State<PaneKind> {
    pane_grid::State::with_configuration(default_layout())
}

#[derive(Debug, Clone)]
pub enum Message {
    Resized(ResizeEvent),
    Dragged(DragEvent),
    /// The title bar's maximise/restore toggle.
    ToggleMaximize(Pane),
}

pub fn update(state: &mut pane_grid::State<PaneKind>, message: Message) {
    match message {
        Message::Resized(ResizeEvent { split, ratio }) => {
            state.resize(split, ratio);
        }
        Message::Dragged(DragEvent::Dropped { pane, target }) => {
            state.drop(pane, target);
        }
        // A drag that ends nowhere leaves the layout alone.
        Message::Dragged(_) => {}
        Message::ToggleMaximize(pane) => {
            if state.maximized().is_some() {
                state.restore();
            } else {
                state.maximize(pane);
            }
        }
    }
}

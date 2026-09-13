//! Operations on a widget selection, and the rules about what can hold what.
//!
//! This module used to draw the fixed palette: a button for every
//! [`WidgetType`], forty-three of them, under the widget tree. The palette is
//! now a popover on each branch (see [`crate::views::widget_picker`]) and the
//! batch controls live in [`crate::views::inspector`], so what is left here is
//! the model behind both — [`get_available_types`] — plus the updates that
//! apply to a whole selection.

use crate::data_structures::properties::messages::PropertyChange;
use crate::data_structures::types::types::{WidgetId, WidgetType};
use crate::data_structures::widget_hierarchy::WidgetHierarchy;
use crate::enum_builder::TypeSystem;
use iced::Task;
// Application messages
#[derive(Debug, Clone)]
pub enum Message {
    // Wrapping operations
    WrapSelectedInContainer(WidgetType), // Wraps selection in Row/Column/MouseArea/Tooltip

    // Batch editing operations
    BatchPropertyChanged(PropertyChange), // Applies property to all selected widgets
}

pub fn update<'a>(
    hierarchy: &'a mut WidgetHierarchy,
    type_system: &'a mut TypeSystem,
    message: Message,
) -> Task<Message> {
    match message {
        Message::WrapSelectedInContainer(container_type) => {
            match hierarchy.wrap_selected_in_container(container_type) {
                Ok(_) => {}
                Err(_) => {}
            }
        }

        Message::BatchPropertyChanged(change) => {
            hierarchy.apply_property_to_all_selected(change, &type_system);
        }
    }
    Task::none()
}

/// Whether one of the overlay widgets can take another child.
///
/// `Popover` holds a trigger and its content and no more; `Layer` is the
/// content on its own; `Menu` holds a trigger and then one child per row, so
/// it never fills up.
fn overlay_accepts_more_children(parent: &crate::data_structures::types::types::Widget) -> bool {
    match parent.widget_type {
        WidgetType::Popover => parent.children.len() < 2,
        WidgetType::Layer => parent.children.is_empty(),
        _ => true,
    }
}

/// Returns list of widget's that can be added to the currently selected widget
pub fn get_available_types<'a>(
    hierarchy: &'a WidgetHierarchy,
    parent_id: &'a WidgetId,
) -> Vec<WidgetType> {
    let parent = hierarchy.get_widget_by_id(*parent_id);
    if parent.is_none() {
        return Vec::new();
    }
    let parent = parent.unwrap();

    let available_types = if *parent_id == hierarchy.root().id {
        if hierarchy.root().children.is_empty() {
            vec![
                WidgetType::Column,
                WidgetType::Row,
                WidgetType::Stack,
                WidgetType::Split,
                WidgetType::Responsive,
            ]
        } else {
            vec![]
        }
    } else if parent.widget_type == WidgetType::Scrollable {
        if parent.children.is_empty() {
            vec![
                WidgetType::Container,
                WidgetType::Collapsible,
                WidgetType::CollapsibleGroup,
                WidgetType::Popover,
                WidgetType::Menu,
                WidgetType::Layer,
                WidgetType::Toast,
                WidgetType::Column,
                WidgetType::Row,
                WidgetType::Stack,
                WidgetType::Split,
                WidgetType::Responsive,
                WidgetType::Float,
                WidgetType::Sensor,
                WidgetType::Themer,
                WidgetType::Grid,
                WidgetType::ViewReference,
            ]
        } else {
            vec![]
        }
    } else if parent.widget_type == WidgetType::Container
        || parent.widget_type == WidgetType::Pin
        || parent.widget_type == WidgetType::Button
        || parent.widget_type == WidgetType::Collapsible
    {
        if parent.children.is_empty() {
            vec![
                WidgetType::Container,
                WidgetType::Scrollable,
                WidgetType::Collapsible,
                WidgetType::CollapsibleGroup,
                WidgetType::Popover,
                WidgetType::Menu,
                WidgetType::Layer,
                WidgetType::Toast,
                WidgetType::Row,
                WidgetType::Column,
                WidgetType::Stack,
                WidgetType::Split,
                WidgetType::Responsive,
                WidgetType::Float,
                WidgetType::Sensor,
                WidgetType::Themer,
                WidgetType::Grid,
                WidgetType::Button,
                WidgetType::Text,
                WidgetType::TextEditor,
                WidgetType::TextInput,
                WidgetType::Checkbox,
                WidgetType::Radio,
                WidgetType::Slider,
                WidgetType::VerticalSlider,
                WidgetType::ProgressBar,
                WidgetType::Toggler,
                WidgetType::PickList,
                WidgetType::DatePicker,
                WidgetType::Space,
                WidgetType::Rule,
                WidgetType::Image,
                WidgetType::Svg,
                WidgetType::Tooltip,
                WidgetType::ComboBox,
                WidgetType::Markdown,
                WidgetType::MouseArea,
                WidgetType::Pin,
                WidgetType::Table,
                WidgetType::DataTable,
                WidgetType::QRCode,
                WidgetType::Icon,
                WidgetType::ViewReference,
            ]
        } else {
            vec![]
        }
    } else if parent.widget_type == WidgetType::MouseArea {
        if parent.children.len() < 1 {
            vec![
                WidgetType::Container,
                WidgetType::Scrollable,
                WidgetType::Collapsible,
                WidgetType::CollapsibleGroup,
                WidgetType::Popover,
                WidgetType::Menu,
                WidgetType::Layer,
                WidgetType::Toast,
                WidgetType::Row,
                WidgetType::Column,
                WidgetType::Stack,
                WidgetType::Split,
                WidgetType::Responsive,
                WidgetType::Float,
                WidgetType::Sensor,
                WidgetType::Themer,
                WidgetType::Grid,
                WidgetType::Button,
                WidgetType::Text,
                WidgetType::TextEditor,
                WidgetType::TextInput,
                WidgetType::Checkbox,
                WidgetType::Radio,
                WidgetType::Slider,
                WidgetType::VerticalSlider,
                WidgetType::ProgressBar,
                WidgetType::Toggler,
                WidgetType::PickList,
                WidgetType::DatePicker,
                WidgetType::Space,
                WidgetType::Rule,
                WidgetType::Image,
                WidgetType::Svg,
                WidgetType::Tooltip,
                WidgetType::ComboBox,
                WidgetType::Markdown,
                WidgetType::Pin,
                WidgetType::Table,
                WidgetType::DataTable,
                WidgetType::QRCode,
                WidgetType::Icon,
            ]
        } else {
            vec![]
        }
    } else if parent.widget_type == WidgetType::Tooltip {
        if parent.children.len() < 2 {
            vec![
                WidgetType::Container,
                WidgetType::Scrollable,
                WidgetType::Collapsible,
                WidgetType::CollapsibleGroup,
                WidgetType::Popover,
                WidgetType::Menu,
                WidgetType::Layer,
                WidgetType::Toast,
                WidgetType::Row,
                WidgetType::Column,
                WidgetType::Stack,
                WidgetType::Split,
                WidgetType::Responsive,
                WidgetType::Float,
                WidgetType::Sensor,
                WidgetType::Themer,
                WidgetType::Grid,
                WidgetType::Button,
                WidgetType::Text,
                WidgetType::TextEditor,
                WidgetType::TextInput,
                WidgetType::Checkbox,
                WidgetType::Radio,
                WidgetType::Slider,
                WidgetType::VerticalSlider,
                WidgetType::ProgressBar,
                WidgetType::Toggler,
                WidgetType::PickList,
                WidgetType::DatePicker,
                WidgetType::Image,
                WidgetType::Svg,
                WidgetType::ComboBox,
                WidgetType::Markdown,
                WidgetType::MouseArea,
                WidgetType::Pin,
                WidgetType::Table,
                WidgetType::DataTable,
                WidgetType::QRCode,
                WidgetType::Icon,
            ]
        } else {
            vec![]
        }
    } else if parent.widget_type == WidgetType::CollapsibleGroup {
        vec![WidgetType::Collapsible]
    } else if matches!(
        parent.widget_type,
        WidgetType::Popover | WidgetType::Menu | WidgetType::Layer | WidgetType::Toast
    ) {
        if overlay_accepts_more_children(parent) {
            vec![
                WidgetType::Container,
                WidgetType::Scrollable,
                WidgetType::Collapsible,
                WidgetType::CollapsibleGroup,
                WidgetType::Popover,
                WidgetType::Menu,
                WidgetType::Layer,
                WidgetType::Toast,
                WidgetType::Row,
                WidgetType::Column,
                WidgetType::Stack,
                WidgetType::Split,
                WidgetType::Responsive,
                WidgetType::Float,
                WidgetType::Sensor,
                WidgetType::Themer,
                WidgetType::Grid,
                WidgetType::Button,
                WidgetType::Text,
                WidgetType::TextEditor,
                WidgetType::TextInput,
                WidgetType::Checkbox,
                WidgetType::Radio,
                WidgetType::Slider,
                WidgetType::VerticalSlider,
                WidgetType::ProgressBar,
                WidgetType::Toggler,
                WidgetType::PickList,
                WidgetType::DatePicker,
                WidgetType::Space,
                WidgetType::Rule,
                WidgetType::Image,
                WidgetType::Svg,
                WidgetType::Tooltip,
                WidgetType::ComboBox,
                WidgetType::Markdown,
                WidgetType::MouseArea,
                WidgetType::Pin,
                WidgetType::Table,
                WidgetType::DataTable,
                WidgetType::QRCode,
                WidgetType::Icon,
                WidgetType::ViewReference,
            ]
        } else {
            vec![]
        }
    } else if parent.widget_type == WidgetType::Split {
        if parent.children.len() < 2 {
            vec![
                WidgetType::Container,
                WidgetType::Scrollable,
                WidgetType::Collapsible,
                WidgetType::CollapsibleGroup,
                WidgetType::Popover,
                WidgetType::Menu,
                WidgetType::Layer,
                WidgetType::Toast,
                WidgetType::Row,
                WidgetType::Column,
                WidgetType::Stack,
                WidgetType::Split,
                WidgetType::Responsive,
                WidgetType::Float,
                WidgetType::Sensor,
                WidgetType::Themer,
                WidgetType::Grid,
                WidgetType::Button,
                WidgetType::Text,
                WidgetType::TextEditor,
                WidgetType::TextInput,
                WidgetType::Checkbox,
                WidgetType::Radio,
                WidgetType::Slider,
                WidgetType::VerticalSlider,
                WidgetType::ProgressBar,
                WidgetType::Toggler,
                WidgetType::PickList,
                WidgetType::DatePicker,
                WidgetType::Space,
                WidgetType::Rule,
                WidgetType::Image,
                WidgetType::Svg,
                WidgetType::Tooltip,
                WidgetType::ComboBox,
                WidgetType::Markdown,
                WidgetType::MouseArea,
                WidgetType::Pin,
                WidgetType::Table,
                WidgetType::DataTable,
                WidgetType::QRCode,
                WidgetType::Icon,
                WidgetType::ViewReference,
            ]
        } else {
            vec![]
        }
    } else if parent.widget_type == WidgetType::Responsive {
        if parent.children.len() < 2 {
            vec![
                WidgetType::Container,
                WidgetType::Scrollable,
                WidgetType::Collapsible,
                WidgetType::CollapsibleGroup,
                WidgetType::Popover,
                WidgetType::Menu,
                WidgetType::Layer,
                WidgetType::Toast,
                WidgetType::Row,
                WidgetType::Column,
                WidgetType::Stack,
                WidgetType::Split,
                WidgetType::Responsive,
                WidgetType::Float,
                WidgetType::Sensor,
                WidgetType::Themer,
                WidgetType::Grid,
                WidgetType::Button,
                WidgetType::Text,
                WidgetType::TextEditor,
                WidgetType::TextInput,
                WidgetType::Checkbox,
                WidgetType::Radio,
                WidgetType::Slider,
                WidgetType::VerticalSlider,
                WidgetType::ProgressBar,
                WidgetType::Toggler,
                WidgetType::PickList,
                WidgetType::DatePicker,
                WidgetType::Space,
                WidgetType::Rule,
                WidgetType::Image,
                WidgetType::Svg,
                WidgetType::Tooltip,
                WidgetType::ComboBox,
                WidgetType::Markdown,
                WidgetType::MouseArea,
                WidgetType::Pin,
                WidgetType::Table,
                WidgetType::DataTable,
                WidgetType::QRCode,
                WidgetType::Icon,
                WidgetType::ViewReference,
            ]
        } else {
            vec![]
        }
    } else if parent.widget_type == WidgetType::Float || parent.widget_type == WidgetType::Sensor {
        if parent.children.is_empty() {
            vec![
                WidgetType::Container,
                WidgetType::Scrollable,
                WidgetType::Collapsible,
                WidgetType::CollapsibleGroup,
                WidgetType::Popover,
                WidgetType::Menu,
                WidgetType::Layer,
                WidgetType::Toast,
                WidgetType::Row,
                WidgetType::Column,
                WidgetType::Stack,
                WidgetType::Split,
                WidgetType::Responsive,
                WidgetType::Float,
                WidgetType::Sensor,
                WidgetType::Themer,
                WidgetType::Grid,
                WidgetType::Button,
                WidgetType::Text,
                WidgetType::TextEditor,
                WidgetType::TextInput,
                WidgetType::Checkbox,
                WidgetType::Radio,
                WidgetType::Slider,
                WidgetType::VerticalSlider,
                WidgetType::ProgressBar,
                WidgetType::Toggler,
                WidgetType::PickList,
                WidgetType::DatePicker,
                WidgetType::Space,
                WidgetType::Rule,
                WidgetType::Image,
                WidgetType::Svg,
                WidgetType::Tooltip,
                WidgetType::ComboBox,
                WidgetType::Markdown,
                WidgetType::MouseArea,
                WidgetType::Pin,
                WidgetType::Table,
                WidgetType::DataTable,
                WidgetType::QRCode,
                WidgetType::Icon,
                WidgetType::ViewReference,
            ]
        } else {
            vec![]
        }
    } else if parent.widget_type == WidgetType::Row
        || parent.widget_type == WidgetType::Column
        || parent.widget_type == WidgetType::Stack
        || parent.widget_type == WidgetType::Themer
        || parent.widget_type == WidgetType::Grid
    {
        vec![
            WidgetType::Container,
            WidgetType::Scrollable,
            WidgetType::Collapsible,
            WidgetType::CollapsibleGroup,
            WidgetType::Popover,
            WidgetType::Menu,
            WidgetType::Layer,
            WidgetType::Toast,
            WidgetType::Row,
            WidgetType::Column,
            WidgetType::Stack,
            WidgetType::Split,
            WidgetType::Responsive,
            WidgetType::Float,
            WidgetType::Sensor,
            WidgetType::Themer,
            WidgetType::Grid,
            WidgetType::Button,
            WidgetType::Text,
            WidgetType::TextEditor,
            WidgetType::TextInput,
            WidgetType::Checkbox,
            WidgetType::Radio,
            WidgetType::Slider,
            WidgetType::VerticalSlider,
            WidgetType::ProgressBar,
            WidgetType::Toggler,
            WidgetType::PickList,
            WidgetType::DatePicker,
            WidgetType::Space,
            WidgetType::Rule,
            WidgetType::Image,
            WidgetType::Svg,
            WidgetType::Tooltip,
            WidgetType::ComboBox,
            WidgetType::Markdown,
            WidgetType::MouseArea,
            WidgetType::Pin,
            WidgetType::Table,
            WidgetType::DataTable,
            WidgetType::QRCode,
            WidgetType::Icon,
            WidgetType::ViewReference,
        ]
    } else {
        vec![]
    };

    available_types
}


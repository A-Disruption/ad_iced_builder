use crate::action_system::node_kinds::TriggerPort;
use crate::action_system::state_ref::ActionValueType;
use crate::data_structures::properties::properties::Properties;
use crate::data_structures::types::types::WidgetType;

/// Returns the event names for which an action graph can be created
/// for a given widget type / properties combination.
pub fn actionable_events(widget_type: WidgetType, props: &Properties) -> Vec<&'static str> {
    match widget_type {
        WidgetType::Button => {
            if props.button_on_press_enabled
                || props.button_on_press_maybe_enabled
                || props.button_on_press_with_enabled
            {
                vec!["on_press"]
            } else {
                vec![]
            }
        }
        WidgetType::TextInput => {
            let mut events = vec!["on_input"];
            if props.text_input_on_submit {
                events.push("on_submit");
            }
            if props.text_input_on_paste {
                events.push("on_paste");
            }
            events
        }
        WidgetType::Checkbox | WidgetType::Toggler => vec!["on_toggle"],
        WidgetType::Slider | WidgetType::VerticalSlider => vec!["on_change"],
        WidgetType::PickList => vec!["on_select"],
        WidgetType::ComboBox => {
            let mut events = vec![];
            if props.combobox_use_on_input {
                events.push("on_input");
            }
            if props.combobox_use_on_option_hovered {
                events.push("on_option_hovered");
            }
            if props.combobox_use_on_open {
                events.push("on_open");
            }
            if props.combobox_use_on_close {
                events.push("on_close");
            }
            events
        }
        WidgetType::MouseArea => {
            let mut events = vec![];
            if props.mousearea_on_press {
                events.push("on_press");
            }
            if props.mousearea_on_release {
                events.push("on_release");
            }
            if props.mousearea_on_double_click {
                events.push("on_double_click");
            }
            if props.mousearea_on_right_press {
                events.push("on_right_press");
            }
            if props.mousearea_on_right_release {
                events.push("on_right_release");
            }
            if props.mousearea_on_middle_press {
                events.push("on_middle_press");
            }
            if props.mousearea_on_middle_release {
                events.push("on_middle_release");
            }
            if props.mousearea_on_scroll {
                events.push("on_scroll");
            }
            if props.mousearea_on_enter {
                events.push("on_enter");
            }
            if props.mousearea_on_move {
                events.push("on_move");
            }
            if props.mousearea_on_exit {
                events.push("on_exit");
            }
            events
        }
        _ => vec![],
    }
}

/// Returns the data output ports that a Trigger node emits for a given event.
///
/// For example, `on_input` carries a `String` named `"text"`.
pub fn trigger_ports_for_event(event_name: &str) -> Vec<TriggerPort> {
    match event_name {
        "on_input" => vec![TriggerPort {
            name: "text".to_string(),
            value_type: ActionValueType::String,
        }],
        "on_paste" => vec![TriggerPort {
            name: "text".to_string(),
            value_type: ActionValueType::String,
        }],
        "on_toggle" => vec![TriggerPort {
            name: "checked".to_string(),
            value_type: ActionValueType::Bool,
        }],
        "on_change" => vec![TriggerPort {
            name: "value".to_string(),
            value_type: ActionValueType::F32,
        }],
        "on_select" | "on_option_hovered" => vec![TriggerPort {
            name: "selected".to_string(),
            value_type: ActionValueType::String,
        }],
        "on_move" => vec![
            TriggerPort {
                name: "x".to_string(),
                value_type: ActionValueType::F32,
            },
            TriggerPort {
                name: "y".to_string(),
                value_type: ActionValueType::F32,
            },
        ],
        // Events with no payload: on_press, on_release, on_double_click, etc.
        _ => vec![],
    }
}

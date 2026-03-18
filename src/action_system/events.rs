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
        WidgetType::TextEditor => vec!["on_edit"],
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
        WidgetType::Slider | WidgetType::VerticalSlider | WidgetType::Split => vec!["on_change"],
        WidgetType::Sensor => {
            let mut events = vec![];
            if props.sensor_on_show {
                events.push("on_show");
            }
            if props.sensor_on_resize {
                events.push("on_resize");
            }
            if props.sensor_on_hide {
                events.push("on_hide");
            }
            events
        }
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
        "on_edit" => vec![TriggerPort {
            name: "text".to_string(),
            value_type: ActionValueType::String,
        }],
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
        "on_show" | "on_resize" => vec![
            TriggerPort {
                name: "width".to_string(),
                value_type: ActionValueType::F32,
            },
            TriggerPort {
                name: "height".to_string(),
                value_type: ActionValueType::F32,
            },
        ],
        // Events with no payload: on_press, on_release, on_double_click, etc.
        _ => vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::{actionable_events, trigger_ports_for_event};
    use crate::action_system::state_ref::ActionValueType;
    use crate::data_structures::properties::properties::Properties;
    use crate::data_structures::types::types::WidgetType;

    #[test]
    fn sensor_actionable_events_follow_enabled_flags() {
        let mut props = Properties::default();
        props.sensor_on_show = true;
        props.sensor_on_hide = true;

        let events = actionable_events(WidgetType::Sensor, &props);

        assert_eq!(events, vec!["on_show", "on_hide"]);
    }

    #[test]
    fn sensor_resize_event_exposes_width_and_height_ports() {
        let ports = trigger_ports_for_event("on_resize");

        assert_eq!(ports.len(), 2);
        assert_eq!(ports[0].name, "width");
        assert_eq!(ports[0].value_type, ActionValueType::F32);
        assert_eq!(ports[1].name, "height");
        assert_eq!(ports[1].value_type, ActionValueType::F32);
    }

    #[test]
    fn text_editor_edit_event_exposes_text_port() {
        let events = actionable_events(WidgetType::TextEditor, &Properties::default());
        let ports = trigger_ports_for_event("on_edit");

        assert_eq!(events, vec!["on_edit"]);
        assert_eq!(ports.len(), 1);
        assert_eq!(ports[0].name, "text");
        assert_eq!(ports[0].value_type, ActionValueType::String);
    }
}

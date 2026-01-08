use crate::code_generator::writer::{CodeWriter, to_pascal_case, to_snake_case, handle_whitespace};
use crate::code_generator::import::ImportTracker;
use crate::data_structures::types::types::{Widget, WidgetType, WidgetId};
use crate::enum_builder::TypeSystem;
use std::collections::HashMap;

pub fn generate_message_enum(
    writer: &mut CodeWriter, 
    root: &Widget, 
    names: &HashMap<WidgetId, String>,
    type_system: &TypeSystem,
) {
    writer.add_newline();
    writer.add_plain("#[derive(Debug, Clone)]");
    writer.add_newline();
    writer.add_keyword("pub enum");
    writer.add_plain(" ");
    writer.add_type("Message");
    writer.add_plain(" {");
    writer.add_newline();
    writer.increase_indent();
    
    // Recursive generation
    generate_message_variants(writer, root, names, type_system);
    
    writer.add_indent();
    writer.add_plain("Noop,"); 
    writer.add_newline();


    writer.decrease_indent(); 
    writer.add_plain("}");
}

fn generate_message_variants(writer: &mut CodeWriter, widget: &Widget, names: &HashMap<WidgetId, String>, type_system: &TypeSystem) {
    let name = names.get(&widget.id).unwrap_or(&"widget".to_string()).clone();
    let props = &widget.properties;

    match widget.widget_type {
        WidgetType::Button => {
            if props.button_on_press_enabled || props.button_on_press_maybe_enabled || props.button_on_press_with_enabled {
                writer.add_indent();
                writer.add_plain(&format!("{}Pressed", to_pascal_case(&name)));
                writer.add_plain(",");
                writer.add_newline();
            }
        }
        WidgetType::TextInput => {
            writer.add_indent();
            writer.add_plain(&format!("{}OnInput", to_pascal_case(&name)));
            writer.add_plain("(");
            writer.add_type("String");
            writer.add_plain("),");
            writer.add_newline();
            
            if props.text_input_on_submit {
                writer.add_indent();
                writer.add_plain(&format!("{}Submitted", to_pascal_case(&name)));
                writer.add_plain(",");
                writer.add_newline();
            }
            
            if props.text_input_on_paste {
                writer.add_indent();
                writer.add_plain(&format!("{}Pasted", to_pascal_case(&name)));
                writer.add_plain("(");
                writer.add_type("String");
                writer.add_plain("),");
                writer.add_newline();
            }
        }
        WidgetType::Checkbox => {
            writer.add_indent();
            writer.add_plain(&format!("{}Toggled", to_pascal_case(&name)));
            writer.add_plain("(");
            writer.add_type("bool");
            writer.add_plain("),");
            writer.add_newline();
        }
        WidgetType::Radio => {
            writer.add_indent();
            writer.add_plain(&format!("{}Selected", to_pascal_case(&name)));
            writer.add_plain("(");
            writer.add_type("usize");
            writer.add_plain("),");
            writer.add_newline();
        }
        WidgetType::Slider | WidgetType::VerticalSlider => {
            writer.add_indent();
            writer.add_plain(&format!("{}Changed", to_pascal_case(&name)));
            writer.add_plain("(");
            writer.add_type("f32");
            writer.add_plain("),");
            writer.add_newline();
        }
        WidgetType::Toggler => {
            writer.add_indent();
            writer.add_plain(&format!("{}Toggled", to_pascal_case(&name)));
            writer.add_plain("(");
            writer.add_type("bool");
            writer.add_plain("),");
            writer.add_newline();
        }
        WidgetType::PickList => {
            writer.add_indent();
            writer.add_plain(&format!("{}Selected", to_pascal_case(&name)));
            writer.add_plain("(");
            writer.add_type("String");
            writer.add_plain("),");
            writer.add_newline();
        }
        WidgetType::ComboBox => {
            
            // Determine the type parameter based on whether enum is used
            let type_name = if let Some(ref enum_id) = props.referenced_enum {
                if let Some(enum_def) = type_system.get_enum(enum_id.clone()) {
                    enum_def.name.clone()
                } else {
                    "String".to_string()
                }
            } else {
                "String".to_string()
            };
            
            // Always generate Selected message
            writer.add_indent();
            writer.add_plain(&format!("{}Selected", to_pascal_case(&name)));
            writer.add_plain("(");
            writer.add_type(&type_name);
            writer.add_plain("),");
            writer.add_newline();
            
            // Conditionally generate on_input
            if props.combobox_use_on_input {
                writer.add_indent();
                writer.add_plain(&format!("{}OnInput", to_pascal_case(&name)));
                writer.add_plain("(");
                writer.add_type("String");
                writer.add_plain("),");
                writer.add_newline();
            }
            
            // Conditionally generate on_option_hovered
            if props.combobox_use_on_option_hovered {
                writer.add_indent();
                writer.add_plain(&format!("{}OnOptionHovered", to_pascal_case(&name)));
                writer.add_plain("(");
                writer.add_type(&type_name);
                writer.add_plain("),");
                writer.add_newline();
            }
            
            // Conditionally generate on_open
            if props.combobox_use_on_open {
                writer.add_indent();
                writer.add_plain(&format!("{}OnOpen", to_pascal_case(&name)));
                writer.add_plain(",");
                writer.add_newline();
            }
            
            // Conditionally generate on_close
            if props.combobox_use_on_close {
                writer.add_indent();
                writer.add_plain(&format!("{}OnClose", to_pascal_case(&name)));
                writer.add_plain(",");
                writer.add_newline();
            }
        }
        WidgetType::MouseArea => {
            if props.mousearea_on_press {
                writer.add_indent();
                writer.add_plain(&format!("{}Pressed", to_pascal_case(&name)));
                writer.add_plain(",");
                writer.add_newline();
            }
            if props.mousearea_on_release {
                writer.add_indent();
                writer.add_plain(&format!("{}Released", to_pascal_case(&name)));
                writer.add_plain(",");
                writer.add_newline();
            }
            if props.mousearea_on_double_click {
                writer.add_indent();
                writer.add_plain(&format!("{}DoubleClicked", to_pascal_case(&name)));
                writer.add_plain(",");
                writer.add_newline();
            }
            if props.mousearea_on_right_press {
                writer.add_indent();
                writer.add_plain(&format!("{}RightPressed", to_pascal_case(&name)));
                writer.add_plain(",");
                writer.add_newline();
            }
            if props.mousearea_on_right_release {
                writer.add_indent();
                writer.add_plain(&format!("{}RightReleased", to_pascal_case(&name)));
                writer.add_plain(",");
                writer.add_newline();
            }
            if props.mousearea_on_middle_press {
                writer.add_indent();
                writer.add_plain(&format!("{}MiddlePressed", to_pascal_case(&name)));
                writer.add_plain(",");
                writer.add_newline();
            }
            if props.mousearea_on_middle_release {
                writer.add_indent();
                writer.add_plain(&format!("{}MiddleReleased", to_pascal_case(&name)));
                writer.add_plain(",");
                writer.add_newline();
            }
            if props.mousearea_on_scroll {
                writer.add_indent();
                writer.add_plain(&format!("{}Scrolled", to_pascal_case(&name)));
                writer.add_plain("(mouse::ScrollDelta),");
                writer.add_newline();
            }
            if props.mousearea_on_enter {
                writer.add_indent();
                writer.add_plain(&format!("{}Entered", to_pascal_case(&name)));
                writer.add_plain("(Point),");
                writer.add_newline();
            }
            if props.mousearea_on_move {
                writer.add_indent();
                writer.add_plain(&format!("{}Moved", to_pascal_case(&name)));
                writer.add_plain("(Point),");
                writer.add_newline();
            }
            if props.mousearea_on_exit {
                writer.add_indent();
                writer.add_plain(&format!("{}Exited", to_pascal_case(&name)));
                writer.add_plain("(Point),");
                writer.add_newline();
            }
        }
        _ => {}
    }
    
    for child in &widget.children {
        generate_message_variants(writer, child, names, type_system);
    }
}

pub fn generate_update(
    writer: &mut CodeWriter, 
    root: &Widget, 
    names: &HashMap<WidgetId, String>,
) {
    writer.add_indent();
    writer.add_keyword("pub fn");
    writer.add_function(" update");
    writer.add_plain("(&");
    writer.add_keyword("mut");
    writer.add_plain(" ");
    writer.add_keyword("self");
    writer.add_plain(", message: Message) {");
    writer.add_newline();

    writer.increase_indent();
    writer.add_indent();
    writer.add_keyword("match");
    writer.add_plain(" message {");
    writer.add_newline();

    writer.increase_indent();
    generate_match_arms(writer, root, names);
    
    // Handle Noop
    writer.add_indent();
    writer.add_type("Message");
    writer.add_operator("::");
    writer.add_plain("Noop => {}");
    writer.add_newline();
    
    writer.decrease_indent(); // match block
    writer.add_indent();
    writer.add_plain("}"); 
    writer.add_newline();
    
    writer.decrease_indent(); // fn block
    writer.add_indent();
    writer.add_plain("}");
    writer.add_newline();
}

fn generate_match_arms(writer: &mut CodeWriter, widget: &Widget, names: &HashMap<WidgetId, String>) {
    let name = names.get(&widget.id).unwrap_or(&"widget".to_string()).clone();
    let props = &widget.properties;

    match widget.widget_type {
        WidgetType::Button => {
            if props.button_on_press_enabled {
                writer.add_indent();
                writer.add_type("Message");
                writer.add_operator("::");
                writer.add_plain(&format!("{}Pressed", to_pascal_case(handle_whitespace(&name).as_str())));
                writer.add_plain(" => {");
                writer.add_newline();
                writer.increase_indent();
                writer.add_indent();
                writer.add_comment(&format!("// {} pressed", name));
                writer.add_newline();
                writer.decrease_indent();
                writer.add_indent();
                writer.add_plain("}");
                writer.add_newline();
            }
        }
        WidgetType::TextInput => {
            
            // Always generate the Changed handler
            writer.add_indent();
            writer.add_type("Message");
            writer.add_operator("::");
            writer.add_plain(&format!("{}OnInput", to_pascal_case(&name)));
            writer.add_plain("(");
            writer.add_identifier("value");
            writer.add_plain(") ");
            writer.add_operator("=>");
            writer.add_plain(" {");
            writer.add_newline();
            writer.increase_indent();
            writer.add_indent();
            writer.add_keyword("self");
            writer.add_operator(".");
            writer.add_identifier(&format!("{}_value", to_snake_case(&name)));
            writer.add_plain(" ");
            writer.add_operator("=");
            writer.add_plain(" ");
            writer.add_identifier("value");
            writer.add_plain(";");
            writer.add_newline();
            writer.decrease_indent();
            writer.add_indent();
            writer.add_plain("}");
            writer.add_newline();
            
            // Conditionally generate on_submit handler
            if props.text_input_on_submit {
                writer.add_indent();
                writer.add_type("Message");
                writer.add_operator("::");
                writer.add_plain(&format!("{}Submitted", to_pascal_case(&name)));
                writer.add_plain(" ");
                writer.add_operator("=>");
                writer.add_plain(" {");
                writer.add_newline();
                writer.increase_indent();
                writer.add_indent();
                writer.add_comment("// Handle text input submission (Enter key pressed)");
                writer.add_newline();
                writer.add_indent();
                writer.add_comment(&format!("// Current value: self.{}_value", to_snake_case(&name)));
                writer.add_newline();
                writer.decrease_indent();
                writer.add_indent();
                writer.add_plain("}");
                writer.add_newline();
            }
            
            // Conditionally generate on_paste handler
            if props.text_input_on_paste {
                writer.add_indent();
                writer.add_type("Message");
                writer.add_operator("::");
                writer.add_plain(&format!("{}Pasted", to_pascal_case(&name)));
                writer.add_plain("(");
                writer.add_identifier("pasted_text");
                writer.add_plain(") ");
                writer.add_operator("=>");
                writer.add_plain(" {");
                writer.add_newline();
                writer.increase_indent();
                writer.add_indent();
                writer.add_comment("// Handle text being pasted");
                writer.add_newline();
                writer.add_indent();
                writer.add_comment("// pasted_text contains the pasted string");
                writer.add_newline();
                writer.add_indent();
                writer.add_comment("// Note: on_input will also fire with the new combined value");
                writer.add_newline();
                writer.add_indent();
                writer.add_keyword("self");
                writer.add_operator(".");
                writer.add_identifier(&format!("{}_value", to_snake_case(&name)));
                writer.add_plain(" ");
                writer.add_operator("=");
                writer.add_plain(" ");
                writer.add_identifier("pasted_text");
                writer.add_plain(";");
                writer.add_newline();
                writer.decrease_indent();
                writer.add_indent();
                writer.add_plain("}");
                writer.add_newline();
            }
        }
        WidgetType::Checkbox => {
            writer.add_indent();
            writer.add_type("Message");
            writer.add_operator("::");
            writer.add_plain(&format!("{}Toggled", to_pascal_case(&name)));
            writer.add_plain("(");
            writer.add_identifier("checked");
            writer.add_plain(") ");
            writer.add_operator("=>");
            writer.add_plain(" {");
            writer.add_newline();
            writer.increase_indent();
            writer.add_indent();
            writer.add_keyword("self");
            writer.add_operator(".");
            writer.add_identifier(&format!("{}_checked", to_snake_case(&name)));
            writer.add_plain(" ");
            writer.add_operator("=");
            writer.add_plain(" ");
            writer.add_identifier("checked");
            writer.add_plain(";");
            writer.add_newline();
            writer.decrease_indent();
            writer.add_indent();
            writer.add_plain("}");
            writer.add_newline();
        }
        WidgetType::Radio => {
            writer.add_indent();
            writer.add_type("Message");
            writer.add_operator("::");
            writer.add_plain(&format!("{}Selected", to_pascal_case(&name)));
            writer.add_plain("(");
            writer.add_identifier("index");
            writer.add_plain(") ");
            writer.add_operator("=>");
            writer.add_plain(" {");
            writer.add_newline();
            writer.increase_indent();
            writer.add_indent();
            writer.add_keyword("self");
            writer.add_operator(".");
            writer.add_identifier(&format!("{}_selected", to_snake_case(&name)));
            writer.add_plain(" ");
            writer.add_operator("=");
            writer.add_plain(" ");
            writer.add_identifier("index");
            writer.add_plain(";");
            writer.add_newline();
            writer.decrease_indent();
            writer.add_indent();
            writer.add_plain("}");
            writer.add_newline();
        }
        WidgetType::Slider | WidgetType::VerticalSlider => {
            writer.add_indent();
            writer.add_type("Message");
            writer.add_operator("::");
            writer.add_plain(&format!("{}Changed", to_pascal_case(&name)));
            writer.add_plain("(");
            writer.add_identifier("value");
            writer.add_plain(") ");
            writer.add_operator("=>");
            writer.add_plain(" {");
            writer.add_newline();
            writer.increase_indent();
            writer.add_indent();
            writer.add_keyword("self");
            writer.add_operator(".");
            writer.add_identifier(&format!("{}_value", to_snake_case(&name)));
            writer.add_plain(" ");
            writer.add_operator("=");
            writer.add_plain(" ");
            writer.add_identifier("value");
            writer.add_plain(";");
            writer.add_newline();
            writer.decrease_indent();
            writer.add_indent();
            writer.add_plain("}");
            writer.add_newline();
        }
        WidgetType::Toggler => {
            writer.add_indent();
            writer.add_type("Message");
            writer.add_operator("::");
            writer.add_plain(&format!("{}Toggled", to_pascal_case(&name)));
            writer.add_plain("(");
            writer.add_identifier("active");
            writer.add_plain(") ");
            writer.add_operator("=>");
            writer.add_plain(" {");
            writer.add_newline();
            writer.increase_indent();
            writer.add_indent();
            writer.add_keyword("self");
            writer.add_operator(".");
            writer.add_identifier(&format!("{}_active", to_snake_case(&name)));
            writer.add_plain(" ");
            writer.add_operator("=");
            writer.add_plain(" ");
            writer.add_identifier("active");
            writer.add_plain(";");
            writer.add_newline();
            writer.decrease_indent();
            writer.add_indent();
            writer.add_plain("}");
            writer.add_newline();
        }
        WidgetType::PickList => {
            writer.add_indent();
            writer.add_type("Message");
            writer.add_operator("::");
            writer.add_plain(&format!("{}Selected", to_pascal_case(&name)));
            writer.add_plain("(");
            writer.add_identifier("value");
            writer.add_plain(") ");
            writer.add_operator("=>");
            writer.add_plain(" {");
            writer.add_newline();
            writer.increase_indent();
            writer.add_indent();
            writer.add_keyword("self");
            writer.add_operator(".");
            writer.add_identifier(&format!("{}_selected", to_snake_case(&name)));
            writer.add_plain(" ");
            writer.add_operator("=");
            writer.add_plain(" Some(");
            writer.add_identifier("value");
            writer.add_plain(");");
            writer.add_newline();
            writer.decrease_indent();
            writer.add_indent();
            writer.add_plain("}");
            writer.add_newline();
        }
        WidgetType::ComboBox => {
            // Always generate Selected handler with helpful example
            writer.add_indent();
            writer.add_type("Message");
            writer.add_operator("::");
            writer.add_plain(&format!("{}Selected", to_pascal_case(&name)));
            writer.add_plain("(");
            writer.add_identifier("value");
            writer.add_plain(") ");
            writer.add_operator("=>");
            writer.add_plain(" {");
            writer.add_newline();
            writer.increase_indent();
            
            // Add helpful println
            writer.add_indent();
            writer.add_macro("println!");
            writer.add_plain("(");
            writer.add_string(&format!("\"{} selected: {{:?}}\"", name));
            writer.add_plain(", ");
            writer.add_identifier("value");
            writer.add_plain(");");
            writer.add_newline();
            
            // Update state
            writer.add_indent();
            writer.add_keyword("self");
            writer.add_operator(".");
            writer.add_identifier(&format!("{}_value", to_snake_case(&name)));
            writer.add_plain(" ");
            writer.add_operator("=");
            writer.add_plain(" ");
            writer.add_identifier("value");
            writer.add_plain(";");
            writer.add_newline();
            
            writer.decrease_indent();
            writer.add_indent();
            writer.add_plain("}");
            writer.add_newline();
            
            // Conditionally generate on_input handler with example
            if props.combobox_use_on_input {
                writer.add_indent();
                writer.add_type("Message");
                writer.add_operator("::");
                writer.add_plain(&format!("{}OnInput", to_pascal_case(&name)));
                writer.add_plain("(");
                writer.add_identifier("text");
                writer.add_plain(") ");
                writer.add_operator("=>");
                writer.add_plain(" {");
                writer.add_newline();
                writer.increase_indent();
                
                writer.add_indent();
                writer.add_macro("println!");
                writer.add_plain("(");
                writer.add_string(&format!("\"{} input text: {{}}\"", name));
                writer.add_plain(", ");
                writer.add_identifier("text");
                writer.add_plain(");");
                writer.add_newline();
                
                writer.add_indent();
                writer.add_comment("// You can filter options, update state, etc.");
                writer.add_newline();
                
                writer.decrease_indent();
                writer.add_indent();
                writer.add_plain("}");
                writer.add_newline();
            }
            
            // Conditionally generate on_option_hovered handler with example
            if props.combobox_use_on_option_hovered {
                writer.add_indent();
                writer.add_type("Message");
                writer.add_operator("::");
                writer.add_plain(&format!("{}OnOptionHovered", to_pascal_case(&name)));
                writer.add_plain("(");
                writer.add_identifier("option");
                writer.add_plain(") ");
                writer.add_operator("=>");
                writer.add_plain(" {");
                writer.add_newline();
                writer.increase_indent();
                
                writer.add_indent();
                writer.add_macro("println!");
                writer.add_plain("(");
                writer.add_string(&format!("\"{} option hovered: {{:?}}\"", name));
                writer.add_plain(", ");
                writer.add_identifier("option");
                writer.add_plain(");");
                writer.add_newline();
                
                writer.add_indent();
                writer.add_comment("// Preview the hovered option, update UI, etc.");
                writer.add_newline();
                
                writer.decrease_indent();
                writer.add_indent();
                writer.add_plain("}");
                writer.add_newline();
            }
            
            // Conditionally generate on_open handler with example
            if props.combobox_use_on_open {
                writer.add_indent();
                writer.add_type("Message");
                writer.add_operator("::");
                writer.add_plain(&format!("{}OnOpen", to_pascal_case(&name)));
                writer.add_plain(" ");
                writer.add_operator("=>");
                writer.add_plain(" {");
                writer.add_newline();
                writer.increase_indent();
                
                writer.add_indent();
                writer.add_macro("println!");
                writer.add_plain("(");
                writer.add_string(&format!("\"{} opened!\"", name));
                writer.add_plain(");");
                writer.add_newline();
                
                writer.add_indent();
                writer.add_comment("// Refresh data, log analytics, etc.");
                writer.add_newline();
                
                writer.decrease_indent();
                writer.add_indent();
                writer.add_plain("}");
                writer.add_newline();
            }
            
            // Conditionally generate on_close handler with example
            if props.combobox_use_on_close {
                writer.add_indent();
                writer.add_type("Message");
                writer.add_operator("::");
                writer.add_plain(&format!("{}OnClose", to_pascal_case(&name)));
                writer.add_plain(" ");
                writer.add_operator("=>");
                writer.add_plain(" {");
                writer.add_newline();
                writer.increase_indent();
                
                writer.add_indent();
                writer.add_macro("println!");
                writer.add_plain("(");
                writer.add_string(&format!("\"{} closed!\"", name));
                writer.add_plain(");");
                writer.add_newline();
                
                writer.add_indent();
                writer.add_comment("// Save user choice, validate selection, etc.");
                writer.add_newline();
                
                writer.decrease_indent();
                writer.add_indent();
                writer.add_plain("}");
                writer.add_newline();
            }
        }
        WidgetType::MouseArea => {
            // Left button press
            if props.mousearea_on_press {
                writer.add_indent();
                writer.add_type("Message");
                writer.add_operator("::");
                writer.add_plain(&format!("{}Pressed", to_pascal_case(&name)));
                writer.add_plain(" ");
                writer.add_operator("=>");
                writer.add_plain(" {");
                writer.add_newline();
                writer.increase_indent();
                writer.add_indent();
                writer.add_comment("// Handle left mouse button press");
                writer.add_newline();
                writer.decrease_indent();
                writer.add_indent();
                writer.add_plain("}");
                writer.add_newline();
            }
            
            // Left button release
            if props.mousearea_on_release {
                writer.add_indent();
                writer.add_type("Message");
                writer.add_operator("::");
                writer.add_plain(&format!("{}Released", to_pascal_case(&name)));
                writer.add_plain(" ");
                writer.add_operator("=>");
                writer.add_plain(" {");
                writer.add_newline();
                writer.increase_indent();
                writer.add_indent();
                writer.add_comment("// Handle left mouse button release");
                writer.add_newline();
                writer.decrease_indent();
                writer.add_indent();
                writer.add_plain("}");
                writer.add_newline();
            }
            
            // Double click
            if props.mousearea_on_double_click {
                writer.add_indent();
                writer.add_type("Message");
                writer.add_operator("::");
                writer.add_plain(&format!("{}DoubleClicked", to_pascal_case(&name)));
                writer.add_plain(" ");
                writer.add_operator("=>");
                writer.add_plain(" {");
                writer.add_newline();
                writer.increase_indent();
                writer.add_indent();
                writer.add_comment("// Handle double click");
                writer.add_newline();
                writer.add_indent();
                writer.add_comment("// Note: on_press and on_release will also fire");
                writer.add_newline();
                writer.decrease_indent();
                writer.add_indent();
                writer.add_plain("}");
                writer.add_newline();
            }
            
            // Right button press
            if props.mousearea_on_right_press {
                writer.add_indent();
                writer.add_type("Message");
                writer.add_operator("::");
                writer.add_plain(&format!("{}RightPressed", to_pascal_case(&name)));
                writer.add_plain(" ");
                writer.add_operator("=>");
                writer.add_plain(" {");
                writer.add_newline();
                writer.increase_indent();
                writer.add_indent();
                writer.add_comment("// Handle right mouse button press");
                writer.add_newline();
                writer.decrease_indent();
                writer.add_indent();
                writer.add_plain("}");
                writer.add_newline();
            }
            
            // Right button release
            if props.mousearea_on_right_release {
                writer.add_indent();
                writer.add_type("Message");
                writer.add_operator("::");
                writer.add_plain(&format!("{}RightReleased", to_pascal_case(&name)));
                writer.add_plain(" ");
                writer.add_operator("=>");
                writer.add_plain(" {");
                writer.add_newline();
                writer.increase_indent();
                writer.add_indent();
                writer.add_comment("// Handle right mouse button release");
                writer.add_newline();
                writer.decrease_indent();
                writer.add_indent();
                writer.add_plain("}");
                writer.add_newline();
            }
            
            // Middle button press
            if props.mousearea_on_middle_press {
                writer.add_indent();
                writer.add_type("Message");
                writer.add_operator("::");
                writer.add_plain(&format!("{}MiddlePressed", to_pascal_case(&name)));
                writer.add_plain(" ");
                writer.add_operator("=>");
                writer.add_plain(" {");
                writer.add_newline();
                writer.increase_indent();
                writer.add_indent();
                writer.add_comment("// Handle middle mouse button press");
                writer.add_newline();
                writer.decrease_indent();
                writer.add_indent();
                writer.add_plain("}");
                writer.add_newline();
            }
            
            // Middle button release
            if props.mousearea_on_middle_release {
                writer.add_indent();
                writer.add_type("Message");
                writer.add_operator("::");
                writer.add_plain(&format!("{}MiddleReleased", to_pascal_case(&name)));
                writer.add_plain(" ");
                writer.add_operator("=>");
                writer.add_plain(" {");
                writer.add_newline();
                writer.increase_indent();
                writer.add_indent();
                writer.add_comment("// Handle middle mouse button release");
                writer.add_newline();
                writer.decrease_indent();
                writer.add_indent();
                writer.add_plain("}");
                writer.add_newline();
            }
            
            // Scroll with delta parameter
            if props.mousearea_on_scroll {
                writer.add_indent();
                writer.add_type("Message");
                writer.add_operator("::");
                writer.add_plain(&format!("{}Scrolled", to_pascal_case(&name)));
                writer.add_plain("(");
                writer.add_identifier("delta");
                writer.add_plain(") ");
                writer.add_operator("=>");
                writer.add_plain(" {");
                writer.add_newline();
                writer.increase_indent();
                writer.add_indent();
                writer.add_comment("// Handle scroll event");
                writer.add_newline();
                writer.add_indent();
                writer.add_comment("// delta is mouse::ScrollDelta enum:");
                writer.add_newline();
                writer.add_indent();
                writer.add_comment("//   Lines { x: f32, y: f32 } - scroll in lines");
                writer.add_newline();
                writer.add_indent();
                writer.add_comment("//   Pixels { x: f32, y: f32 } - scroll in pixels");
                writer.add_newline();
                writer.add_indent();
                writer.add_keyword("match");
                writer.add_plain(" ");
                writer.add_identifier("delta");
                writer.add_plain(" {");
                writer.add_newline();
                writer.increase_indent();
                writer.add_indent();
                writer.add_plain("mouse::ScrollDelta::Lines { ");
                writer.add_identifier("x");
                writer.add_plain(", ");
                writer.add_identifier("y");
                writer.add_plain(" } ");
                writer.add_operator("=>");
                writer.add_plain(" {");
                writer.add_newline();
                writer.increase_indent();
                writer.add_indent();
                writer.add_comment("// Handle line-based scrolling");
                writer.add_newline();
                writer.decrease_indent();
                writer.add_indent();
                writer.add_plain("}");
                writer.add_newline();
                writer.add_indent();
                writer.add_plain("mouse::ScrollDelta::Pixels { ");
                writer.add_identifier("x");
                writer.add_plain(", ");
                writer.add_identifier("y");
                writer.add_plain(" } ");
                writer.add_operator("=>");
                writer.add_plain(" {");
                writer.add_newline();
                writer.increase_indent();
                writer.add_indent();
                writer.add_comment("// Handle pixel-based scrolling");
                writer.add_newline();
                writer.decrease_indent();
                writer.add_indent();
                writer.add_plain("}");
                writer.add_newline();
                writer.decrease_indent();
                writer.add_indent();
                writer.add_plain("}");
                writer.add_newline();
                writer.decrease_indent();
                writer.add_indent();
                writer.add_plain("}");
                writer.add_newline();
            }
            
            // Mouse enter
            if props.mousearea_on_enter {
                writer.add_indent();
                writer.add_type("Message");
                writer.add_operator("::");
                writer.add_plain(&format!("{}Entered", to_pascal_case(&name)));
                writer.add_plain(" ");
                writer.add_operator("=>");
                writer.add_plain(" {");
                writer.add_newline();
                writer.increase_indent();
                writer.add_indent();
                writer.add_comment("// Handle mouse entering the area");
                writer.add_newline();
                writer.decrease_indent();
                writer.add_indent();
                writer.add_plain("}");
                writer.add_newline();
            }
            
            // Mouse move with point parameter
            if props.mousearea_on_move {
                writer.add_indent();
                writer.add_type("Message");
                writer.add_operator("::");
                writer.add_plain(&format!("{}Moved", to_pascal_case(&name)));
                writer.add_plain("(");
                writer.add_identifier("point");
                writer.add_plain(") ");
                writer.add_operator("=>");
                writer.add_plain(" {");
                writer.add_newline();
                writer.increase_indent();
                writer.add_indent();
                writer.add_comment("// Handle mouse movement within the area");
                writer.add_newline();
                writer.add_indent();
                writer.add_comment("// point is Point { x: f32, y: f32 } relative to the widget's bounds");
                writer.add_newline();
                writer.add_indent();
                writer.add_keyword("let");
                writer.add_plain(" ");
                writer.add_identifier("x");
                writer.add_plain(" ");
                writer.add_operator("=");
                writer.add_plain(" ");
                writer.add_identifier("point");
                writer.add_operator(".");
                writer.add_identifier("x");
                writer.add_plain(";");
                writer.add_newline();
                writer.add_indent();
                writer.add_keyword("let");
                writer.add_plain(" ");
                writer.add_identifier("y");
                writer.add_plain(" ");
                writer.add_operator("=");
                writer.add_plain(" ");
                writer.add_identifier("point");
                writer.add_operator(".");
                writer.add_identifier("y");
                writer.add_plain(";");
                writer.add_newline();
                writer.decrease_indent();
                writer.add_indent();
                writer.add_plain("}");
                writer.add_newline();
            }
            
            // Mouse exit
            if props.mousearea_on_exit {
                writer.add_indent();
                writer.add_type("Message");
                writer.add_operator("::");
                writer.add_plain(&format!("{}Exited", to_pascal_case(&name)));
                writer.add_plain(" ");
                writer.add_operator("=>");
                writer.add_plain(" {");
                writer.add_newline();
                writer.increase_indent();
                writer.add_indent();
                writer.add_comment("// Handle mouse leaving the area");
                writer.add_newline();
                writer.decrease_indent();
                writer.add_indent();
                writer.add_plain("}");
                writer.add_newline();
            }

            writer.add_plain(",");

        }
        _ => {}
    }

    for child in &widget.children {
        generate_match_arms(writer, child, names);
    }
}

pub fn generate_imports(writer: &mut CodeWriter, root: &Widget) {
    // Scan the entire hierarchy
    let mut tracker = ImportTracker::new();
    tracker.scan_widget(root);
    
    writer.add_keyword("use");
    writer.add_number(" iced");
    writer.add_operator("::");
    writer.add_plain("{");
    writer.add_newline();
    writer.increase_indent();
    
    // Core types - build list
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
    
    // Element, Theme, and Task are always needed
    core_imports.push("Element");
    core_imports.push("Theme");
    core_imports.push("Task");

    writer.add_indent();
    core_imports.into_iter().for_each(|import| {
        writer.add_type(import);
        writer.add_plain(", ");
    });
    writer.add_newline();
    
    // Widget imports
    if !tracker.used_widgets.is_empty() {
        writer.add_indent();
        writer.add_number("widget");
        writer.add_operator("::");
        writer.add_plain("{");
        let mut widgets: Vec<_> = tracker.used_widgets.iter().map(|s| *s).collect();
        widgets.sort();
        writer.add_plain(&widgets.join(", "));
        writer.add_plain("},");
        writer.add_newline();
    }
    
    // Mouse module - only if MouseArea is used
    if tracker.uses_mouse {
        writer.add_indent();
        writer.add_plain("mouse");
        
        let mut mouse_items = Vec::new();
        if tracker.uses_mouse_interaction {
            mouse_items.push("Interaction");
        }
        if tracker.uses_mouse_scroll_delta {
            mouse_items.push("ScrollDelta");
        }
        
        if !mouse_items.is_empty() {
            writer.add_plain("::{");
            writer.add_plain(&mouse_items.join(", "));
            writer.add_plain("}");
        }
        writer.add_plain(",");
        writer.add_newline();
    }
    
    // Text module - only if text properties are used
    if tracker.uses_text_line_height || tracker.uses_text_wrapping || 
    tracker.uses_text_shaping || tracker.uses_text_alignment {
        writer.add_indent();
        writer.add_plain("widget::text");
        
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
            writer.add_plain("::{");
            writer.add_plain(&text_items.join(", "));
            writer.add_plain("}");
        }
        writer.add_plain(",");
        writer.add_newline();
    }
    
    writer.decrease_indent();
    writer.add_plain("};");
    writer.add_newline();
}
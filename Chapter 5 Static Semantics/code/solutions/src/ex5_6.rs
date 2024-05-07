use std::collections::HashMap;

struct Transition {
    source_state: String,
    target_state: String,
    input_label: String,
}

struct StateMachine {
    transitions: Vec<Transition>,
}

impl StateMachine {
    fn new() -> Self {
        StateMachine {
            transitions: Vec::new(),
        }
    }

    fn add_transition(&mut self, source: &str, target: &str, label: &str) -> Result<(), String> {
        for t in &self.transitions {
            if t.source_state == source && t.input_label == label {
                return Err("Duplicate label for the same source state".to_string());
            }
        }
        self.transitions.push(Transition {
            source_state: source.to_string(),
            target_state: target.to_string(),
            input_label: label.to_string(),
        });
        Ok(())
    }
}

fn main() {
    let mut sm = StateMachine::new();

    sm.add_transition("state1", "state2", "label1").unwrap();
    sm.add_transition("state1", "state3", "label2").unwrap();

    match sm.add_transition("state1", "state4", "label1") {
        Ok(_) => println!("Transition added successfully."),
        Err(e) => println!("Failed to add transition: {}", e),
    }
}

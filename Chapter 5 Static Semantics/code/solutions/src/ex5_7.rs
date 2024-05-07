use std::collections::{HashMap, VecDeque};

struct StateMachine {
    transitions: HashMap<String, Vec<String>>,
    reachable_states: HashMap<String, Vec<String>>,
}

impl StateMachine {
    fn new() -> Self {
        StateMachine {
            transitions: HashMap::new(),
            reachable_states: HashMap::new(),
        }
    }

    fn add_transition(&mut self, source: &str, target: &str) {
        self.transitions.entry(source.to_string()).or_default().push(target.to_string());
        self.update_reachable_states();
    }

    fn update_reachable_states(&mut self) {
        self.reachable_states.clear();
        for (initial, _) in self.transitions.iter() {
            let reachable = self.bfs(initial);
            self.reachable_states.insert(initial.clone(), reachable);
        }
    }

    fn bfs(&self, start: &str) -> Vec<String> {
        let mut seen = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(start.to_string());
        while let Some(state) = queue.pop_front() {
            if !seen.contains(&state) {
                seen.push(state.clone());
                if let Some(targets) = self.transitions.get(&state) {
                    for target in targets {
                        queue.push_back(target.clone());
                    }
                }
            }
        }
        seen
    }

    fn is_reachable(&self, initial: &str, state: &str) -> bool {
        if let Some(reachable) = self.reachable_states.get(initial) {
            reachable.contains(&state.to_string())
        } else {
            false
        }
    }
}

fn main() {
    let mut sm = StateMachine::new();
    sm.add_transition("state1", "state2");
    sm.add_transition("state1", "state3");
    sm.add_transition("state2", "state4");

    println!("Is state4 reachable from state1? {}", sm.is_reachable("state1", "state4"));
}

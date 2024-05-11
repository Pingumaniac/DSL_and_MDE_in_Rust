use std::collections::HashMap;

struct PetriNet {
    places: HashMap<String, u32>,
    transitions: Vec<Transition>,
}

struct Transition {
    input: Vec<(String, u32)>,
    output: Vec<(String, u32)>,
}

impl Transition {
    fn fire(&self, places: &mut HashMap<String, u32>) -> bool {
        // Check if transition can fire
        for (place, count) in &self.input {
            if places.get(place).unwrap_or(&0) < count {
                return false; // Not enough tokens
            }
        }

        // Transition can fire, update tokens
        for (place, count) in &self.input {
            *places.get_mut(place).unwrap() -= count;
        }
        for (place, count) in &self.output {
            *places.entry(place.clone()).or_insert(0) += count;
        }

        true
    }
}

impl PetriNet {
    fn new() -> Self {
        Self {
            places: HashMap::new(),
            transitions: vec![],
        }
    }

    fn add_place(&mut self, name: String, tokens: u32) {
        self.places.insert(name, tokens);
    }

    fn add_transition(&mut self, input: Vec<(String, u32)>, output: Vec<(String, u32)>) {
        self.transitions.push(Transition { input, output });
    }

    fn run(&mut self) {
        let mut any_fired = true;
        while any_fired {
            any_fired = false;
            for transition in &self.transitions {
                if transition.fire(&mut self.places) {
                    any_fired = true;
                    break;
                }
            }
        }
    }
}

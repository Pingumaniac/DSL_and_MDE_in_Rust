#[derive(Debug)]
struct State {
    id: u32,
    is_initial: bool,
}

#[derive(Debug)]
struct FiniteStateMachine {
    states: Vec<State>,
}

impl FiniteStateMachine {
    fn new() -> Self {
        FiniteStateMachine {
            states: Vec::new(),
        }
    }

    fn add_state(&mut self, id: u32, is_initial: bool) {
        if is_initial {
            for state in &mut self.states {
                state.is_initial = false;
            }
        }

        let state = State { id, is_initial };
        self.states.push(state);
    }

    fn initial_state(&self) -> Option<&State> {
        self.states.iter().find(|state| state.is_initial)
    }
}

fn main() {
    let mut fsm = FiniteStateMachine::new();

    fsm.add_state(1, false);
    fsm.add_state(2, true);

    println!("Finite State Machine: {:?}", fsm);
    if let Some(initial_state) = fsm.initial_state() {
        println!("The initial state is: {:?}", initial_state);
    }
}

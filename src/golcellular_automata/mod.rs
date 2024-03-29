use crate::state::State;

#[derive(Debug, Copy, Clone)]
pub struct GOLCellularAutomata {
    state: State,
}

impl GOLCellularAutomata {
    pub fn new(state: State) -> GOLCellularAutomata {
        GOLCellularAutomata { state: state }
    }

    pub fn get_state(self: &Self) -> State {
        self.state
    }

    pub fn change_state(self: &mut Self, by: State) -> () {
        self.state = by
    }
}
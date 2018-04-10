pub struct State {
    accumulator: u8,
    program_counter: u8,
}

pub trait Instruction {
    fn execute(&self, state: State) -> State;
}

pub struct Add<'a> {
    address: &'a u8,
}

impl<'a> Instruction for Add<'a> {
    fn execute(&self, state: State) -> State {
        let new_accumulator: u8 = state.accumulator + *self.address;
        State {
            accumulator: new_accumulator,
            program_counter: state.program_counter,
        }
    }
}

pub struct Nop {
}

impl Instruction for Nop {
    fn execute(&self, state: State) -> State {
        state
    }
}

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

impl<'a> Add<'a> {
    fn new(address: &'a u8) -> Add {
        Add { address }
    }
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

impl Nop {
    fn new() -> Nop {
        Nop {}
    }
}

impl Instruction for Nop {
    fn execute(&self, state: State) -> State {
        state
    }
}

pub struct Halt {
}

impl Halt {
    fn new() -> Halt {
        Halt {}
    }
}

impl Instruction for Halt {
    fn execute(&self, state: State) -> State {
        state
    }
}

use std::error::Error;
use std::fs::File;
use std::io::Bytes;

pub fn parser(mut bytes: Bytes<File>) -> Result<Vec<Box<Instruction>>, Box<Error>> {
    let mut instructions: Vec<Box<Instruction>> = Vec::with_capacity(10);

    // let mut opcode;
    loop {
        match bytes.next() {
            Some(byte_result) => match byte_result {
                Ok(byte) => {
                    println!("byte {:x}", byte);
                    match byte {
                        0x00 => instructions.push(Box::new(Nop::new())),
                        0x30 => instructions.push(Box::new(Add::new(&0))),
                        _ => break
                    };

                },
                Err(err) => {
                    println!("err");
                },
            },
            None => {
                println!("None");
                instructions.push(Box::new(Halt::new()));
                break
            },
        };
    };

    Ok(instructions)
}

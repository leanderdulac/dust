extern crate neander_emu;

use std::fs::File;

use neander_emu::Parser;
use neander_emu::interpreter::*;
use neander_emu::instruction_set::*;

fn main() {
    let file = File::open("binary_files/program.dat").unwrap();
    let mut parser = Parser::new(file);

    for byte in parser {
        println!("byte: {:?}", byte);
        match parser.byte_to_opcode(byte) {
            Ok(opcode) => {
                match opcode {
                    OpCode::NOP => {
                        println!("NOP");
                        nop();

                    },
                    OpCode::STA(operand) => {
                        println!("sta: {:?}", operand);
                    },
                    OpCode::LDA(operand) => {
                        println!("lda: {:?}", operand);
                    },
                    OpCode::ADD(operand) => {
                        println!("add: {:?}", operand);
                    },
                    _ => {
                        println!("underscore");
                    },
                }
            },
            Err(error) => {
                println!("errrrrooo {:?}", error);
            },
        }
    }
}

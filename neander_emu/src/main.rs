extern crate neander_emu;

use std::fs::File;
use std::io::Read;
use neander_emu::parser;

fn main() {
    println!("Hello neander rewrite!");
    let file = File::open("binary_files/nop_add").unwrap();
    let instructions = parser(file.bytes());

    for instruction in instructions {
        println!("yo");

    }
}

use std::process;

pub fn nop() {
}

pub fn sta(address: &mut u8, accumulator: u8) {
   *address = accumulator;
}

pub fn lda(accumulator: &mut u8, address: &u8) {
    *accumulator = *address;
}

pub fn add(accumulator: &mut u8, address: &u8) {
    *accumulator += *address;
}

pub fn or(accumulator: &mut u8, address: &u8) {
    *accumulator = *accumulator | *address;
}

pub fn and(accumulator: &mut u8, address: &u8) {
    *accumulator = *accumulator & *address;
}

pub fn not(accumulator: &mut u8) {
    *accumulator = !*accumulator;
}

pub fn jmp(program_counter: &mut u8, literal: u8) {
    *program_counter = literal;
}

pub fn jn(program_counter: &mut u8, accumulator: u8, literal: u8) {
    if accumulator == 0x01 {
        *program_counter = literal;
    }
}

pub fn jz(program_counter: &mut u8, accumulator: u8, literal: u8) {
    if accumulator == 0x00 {
        *program_counter = literal;
    }
}

pub fn hlt() {
    process::exit(0);
}

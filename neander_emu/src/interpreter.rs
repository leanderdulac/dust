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

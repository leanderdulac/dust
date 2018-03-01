pub fn nop() {
}

pub fn sta(address: &mut u8, accumulator: u8) {
   *address = accumulator;
}

pub fn lda(accumulator: &mut u8, address: &u8) {
    *accumulator = *address;
}

pub fn nop() {
}

pub fn sta(address: &mut u8, accumulator: u8) {
   *address = accumulator;
}

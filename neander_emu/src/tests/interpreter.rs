use super::super::interpreter::*;

#[test]
fn test_nop() {
    nop();
}

#[test]
fn test_sta() {
    let mut value: u8 = 0x00;
    let mut accumulator: u8 = 0xa0;

    sta(&mut value, accumulator);

    assert_eq!(value, 0xa0);
}

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

#[test]
fn test_lda() {
    let value: u8 = 0x10;
    let mut accumulator: u8 = 0x00;

    lda(&mut accumulator, &value);

    assert_eq!(accumulator, 0x10);
}

#[test]
fn test_add() {
    let value: u8 = 0x10;
    let mut accumulator: u8 = 0x20;

    add(&mut accumulator, &value);

    assert_eq!(accumulator, 0x30);
}

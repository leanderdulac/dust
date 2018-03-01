use super::super::interpreter::*;
use std::process::Command;

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

#[test]
fn test_or() {
    let value: u8 = 0xf0;
    let mut accumulator: u8 = 0x0f;

    or(&mut accumulator, &value);

    assert_eq!(accumulator, 0xff);
}

#[test]
fn test_and() {
    let value: u8 = 0xf0;
    let mut accumulator: u8 = 0x0f;

    and(&mut accumulator, &value);

    assert_eq!(accumulator, 0x00);
}

#[test]
fn test_not() {
    let mut accumulator: u8 = 0x55;

    not(&mut accumulator);

    assert_eq!(accumulator, 0xaa);
}

#[test]
#[ignore]
fn test_hlt() {
    hlt();
}

#[test]
fn spawn_process_to_test_hlt() {
    let status = Command::new("/proc/self/exe")
        .args(&["--ignored", "real"])
        .status()
        .expect("Unable to run program");

    assert_eq!(Some(0), status.code());
}

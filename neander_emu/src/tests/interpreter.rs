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
fn test_jmp() {
    let value: u8 = 0x05;
    let mut program_counter: u8 = 0x03;

    jmp(&mut program_counter, value);

    assert_eq!(program_counter, 0x05);
}

#[test]
fn test_jn_if_one() {
    let mut program_counter: u8 = 0x05;
    let accumulator: u8 = 0x01;
    let literal: u8 = 0x0a;

    jn(&mut program_counter, accumulator, literal);

    assert_eq!(program_counter, 0x0a);
}

#[test]
fn test_jn_if_not_one() {
    let mut program_counter: u8 = 0x05;
    let accumulator: u8 = 0x00;
    let literal: u8 = 0x0a;

    jn(&mut program_counter, accumulator, literal);

    assert_eq!(program_counter, 0x05);
}

#[test]
fn test_jz_if_zero() {
    let mut program_counter: u8 = 0x05;
    let accumulator: u8 = 0x00;
    let literal: u8 = 0x0a;

    jz(&mut program_counter, accumulator, literal);

    assert_eq!(program_counter, 0x0a);
}

#[test]
fn test_jz_if_not_zero() {
    let mut program_counter: u8 = 0x05;
    let accumulator: u8 = 0x06;
    let literal: u8 = 0x0a;

    jz(&mut program_counter, accumulator, literal);

    assert_eq!(program_counter, 0x05);
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

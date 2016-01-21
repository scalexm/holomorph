use protocol::io::{ReadExt, WriteExt};
use std::io::Cursor;

#[test]
fn test_string() {
    let mut buf = Vec::new();
    buf.write_string("salut").unwrap();
    buf.write_string("helloéà❤️").unwrap();

    let mut buf = Cursor::new(buf);
    assert_eq!("salut", &buf.read_string().unwrap());
    assert_eq!("helloéà❤️", &buf.read_string().unwrap());
}

#[test]
fn test_primitive() {
    let mut buf = Vec::new();
    buf.write_u8(5).unwrap();
    buf.write_i8(-5).unwrap();
    buf.write_u16(1_234).unwrap();
    buf.write_i16(-1_234).unwrap();
    buf.write_u32(123_456).unwrap();
    buf.write_i32(-123_456).unwrap();
    buf.write_u64(12_345_678_900).unwrap();
    buf.write_i64(-12_345_678_900).unwrap();
    buf.write_f64(2.71828).unwrap();

    let mut buf = Cursor::new(buf);
    assert_eq!(5, buf.read_u8().unwrap());
    assert_eq!(-5, buf.read_i8().unwrap());
    assert_eq!(1_234, buf.read_u16().unwrap());
    assert_eq!(-1_234, buf.read_i16().unwrap());
    assert_eq!(123_456, buf.read_u32().unwrap());
    assert_eq!(-123_456, buf.read_i32().unwrap());
    assert_eq!(12_345_678_900, buf.read_u64().unwrap());
    assert_eq!(-12_345_678_900, buf.read_i64().unwrap());
    assert_eq!(2.71828, buf.read_f64().unwrap());
}

#[test]
fn test_var() {
    let mut buf = Vec::new();
    buf.write_var_i32(-200).unwrap();
    buf.write_var_i32(-400_000).unwrap();
    buf.write_var_i32(400_000).unwrap();
    buf.write_var_u32(3_000_000_000).unwrap();

    buf.write_var_i16(-4).unwrap();
    buf.write_var_i16(-4_000).unwrap();
    buf.write_var_i16(4_000).unwrap();
    buf.write_var_u16(40_000).unwrap();

    buf.write_var_i64(12).unwrap();
    buf.write_var_i64(78_878).unwrap();
    buf.write_var_i64(1_234_567_890).unwrap();
    buf.write_var_i64(12_456_456_456_465_464).unwrap();
    buf.write_var_i64(-12).unwrap();
    buf.write_var_i64(-78_878).unwrap();
    buf.write_var_i64(-1_234_567_890).unwrap();
    buf.write_var_i64(-12_456_456_456_465_464).unwrap();
    buf.write_var_u64(12_456_456_456_465_464).unwrap();
    buf.write_var_i64(1).unwrap();

    let mut buf = Cursor::new(buf);
    assert_eq!(-200, buf.read_var_i32().unwrap());
    assert_eq!(-400_000, buf.read_var_i32().unwrap());
    assert_eq!(400_000, buf.read_var_i32().unwrap());
    assert_eq!(3_000_000_000, buf.read_var_u32().unwrap());

    assert_eq!(-4, buf.read_var_i16().unwrap());
    assert_eq!(-4_000, buf.read_var_i16().unwrap());
    assert_eq!(4_000, buf.read_var_i16().unwrap());
    assert_eq!(40_000, buf.read_var_u16().unwrap());

    assert_eq!(12, buf.read_var_i64().unwrap());
    assert_eq!(78_878, buf.read_var_i64().unwrap());
    assert_eq!(1_234_567_890, buf.read_var_i64().unwrap());
    assert_eq!(12_456_456_456_465_464, buf.read_var_i64().unwrap());
    assert_eq!(-12, buf.read_var_i64().unwrap());
    assert_eq!(-78_878, buf.read_var_i64().unwrap());
    assert_eq!(-1_234_567_890, buf.read_var_i64().unwrap());
    assert_eq!(-12_456_456_456_465_464, buf.read_var_i64().unwrap());
    assert_eq!(12_456_456_456_465_464, buf.read_var_u64().unwrap());
    assert_eq!(1, buf.read_var_i64().unwrap());
}

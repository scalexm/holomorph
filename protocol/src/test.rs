use super::{Decode, Encode, Var};
use bytes::BytesMut;

#[test]
fn test_string() {
    let mut buf = BytesMut::new();
    "salut".encode(&mut buf);
    "helloéà❤️".encode(&mut buf);

    let mut buf = &buf[0..];
    assert_eq!("salut", <&str as Decode<'_>>::decode(&mut buf).unwrap());
    assert_eq!(
        "helloéà❤️",
        <&str as Decode<'_>>::decode(&mut buf).unwrap()
    );
}

#[test]
fn test_primitive() {
    let mut buf = BytesMut::new();
    5u8.encode(&mut buf);
    (-5i8).encode(&mut buf);
    1_234u16.encode(&mut buf);
    (-1_234i16).encode(&mut buf);
    123_456u32.encode(&mut buf);
    (-123_456i32).encode(&mut buf);
    2.71828f32.encode(&mut buf);
    2.71828f64.encode(&mut buf);

    let mut buf = &buf[0..];
    assert_eq!(5, u8::decode(&mut buf).unwrap());
    assert_eq!(-5, i8::decode(&mut buf).unwrap());
    assert_eq!(1_234, u16::decode(&mut buf).unwrap());
    assert_eq!(-1_234, i16::decode(&mut buf).unwrap());
    assert_eq!(123_456, u32::decode(&mut buf).unwrap());
    assert_eq!(-123_456, i32::decode(&mut buf).unwrap());
    assert_eq!(2.71828, f32::decode(&mut buf).unwrap());
    assert_eq!(2.71828, f64::decode(&mut buf).unwrap());
}

#[test]
fn test_var() {
    let mut buf = BytesMut::new();
    Var(-200i32).encode(&mut buf);
    Var(-400_000i32).encode(&mut buf);
    Var(400_000i32).encode(&mut buf);
    Var(3_000_000_000u32).encode(&mut buf);

    Var(-4i16).encode(&mut buf);
    Var(-4000i16).encode(&mut buf);
    Var(4000i16).encode(&mut buf);
    Var(40000u16).encode(&mut buf);

    Var(12i64).encode(&mut buf);
    Var(78_878i64).encode(&mut buf);
    Var(1_234_567_890i64).encode(&mut buf);
    Var(12_456_456_456_465_464i64).encode(&mut buf);
    Var(-12i64).encode(&mut buf);
    Var(-78_878i64).encode(&mut buf);
    Var(-1_234_567_890i64).encode(&mut buf);
    Var(-12_456_456_456_465_464i64).encode(&mut buf);
    Var(12_456_456_456_465_464u64).encode(&mut buf);
    Var(1i64).encode(&mut buf);

    let mut buf = &buf[0..];
    assert_eq!(-200, Var::<i32>::decode(&mut buf).unwrap().0);
    assert_eq!(-400_000, Var::<i32>::decode(&mut buf).unwrap().0);
    assert_eq!(400_000, Var::<i32>::decode(&mut buf).unwrap().0);
    assert_eq!(3_000_000_000, Var::<u32>::decode(&mut buf).unwrap().0);

    assert_eq!(-4, Var::<i16>::decode(&mut buf).unwrap().0);
    assert_eq!(-4_000, Var::<i16>::decode(&mut buf).unwrap().0);
    assert_eq!(4_000, Var::<i16>::decode(&mut buf).unwrap().0);
    assert_eq!(40_000, Var::<u16>::decode(&mut buf).unwrap().0);

    assert_eq!(12, Var::<i64>::decode(&mut buf).unwrap().0);
    assert_eq!(78_878, Var::<i64>::decode(&mut buf).unwrap().0);
    assert_eq!(1_234_567_890, Var::<i64>::decode(&mut buf).unwrap().0);
    assert_eq!(
        12_456_456_456_465_464,
        Var::<i64>::decode(&mut buf).unwrap().0
    );
    assert_eq!(-12, Var::<i64>::decode(&mut buf).unwrap().0);
    assert_eq!(-78_878, Var::<i64>::decode(&mut buf).unwrap().0);
    assert_eq!(-1_234_567_890, Var::<i64>::decode(&mut buf).unwrap().0);
    assert_eq!(
        -12_456_456_456_465_464,
        Var::<i64>::decode(&mut buf).unwrap().0
    );
    assert_eq!(
        12_456_456_456_465_464,
        Var::<i64>::decode(&mut buf).unwrap().0
    );
    assert_eq!(1, Var::<i64>::decode(&mut buf).unwrap().0);
}

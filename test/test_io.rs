use shared::io::{ReadExt, WriteExt};
use std::io::Cursor;

#[test]
fn test_io() {
    let mut buf = Vec::new();
    buf.write_string("salut").unwrap();

    buf.write_var_i32(-200).unwrap();
    buf.write_var_i32(-400_000).unwrap();
    buf.write_var_i32(400_000).unwrap();
    buf.write_var_u32(3_000_000_000).unwrap();

    buf.write_var_i16(-4).unwrap();
    buf.write_var_i16(-4_000).unwrap();
    buf.write_var_i16(4_000).unwrap();
    buf.write_var_u16(40_000).unwrap();

    let mut buf = Cursor::new(buf);
    assert_eq!("salut", &buf.read_string().unwrap());

    assert_eq!(-200, buf.read_var_i32().unwrap());
    assert_eq!(-400_000, buf.read_var_i32().unwrap());
    assert_eq!(400_000, buf.read_var_i32().unwrap());
    assert_eq!(3_000_000_000, buf.read_var_u32().unwrap());

    assert_eq!(-4, buf.read_var_i16().unwrap());
    assert_eq!(-4_000, buf.read_var_i16().unwrap());
    assert_eq!(4_000, buf.read_var_i16().unwrap());
    assert_eq!(40_000, buf.read_var_u16().unwrap());
}

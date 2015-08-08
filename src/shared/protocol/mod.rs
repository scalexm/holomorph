use std::io::{Read, Write};
use ::io::{ReadExt, WriteExt, Result};

pub mod connection;

struct VarInt(i32);
struct VarUInt(u32);
struct VarShort(i16);
struct VarUShort(u16);
struct VarLong(i64);
struct VarULong(u64);

struct VarIntVec<T>(Vec<T>);

pub trait Protocol {
    fn deserialize<R: Read>(&mut R) -> Result<Self>;
    fn serialize<W: Write>(&self, &mut W) -> Result<()>;
    fn id() -> i16;
}

macro_rules! impl_type {
    ($name: ident, $id: expr, $($field_name: ident| $field_type: ty),*) => {
        pub struct $name {
            $(
                $field_name: $field_type,
            )*
        }

        impl Protocol for $name {
            fn deserialize<R: Read>(rdr: &mut R) -> Result<$name> {
                Ok($name {
                    $(
                        $field_name: try!(<$field_type as Protocol>::deserialize(rdr)),
                    )*
                })
            }

            fn serialize<W: Write>(&self, wtr: &mut W) -> Result<()> {
                $(
                    try!(self.$field_name.serialize(wtr));
                )*
                Ok(())
            }

            fn id() -> i16 {
                $id
            }
        }
    };
}


macro_rules! impl_primitive {
    ($t: ty, $read: ident, $write: ident) => {
        impl Protocol for $t {
            fn deserialize<R: Read>(rdr: &mut R) -> Result<$t> {
                rdr.$read()
            }

            fn serialize<W: Write>(&self, wtr: &mut W) -> Result<()> {
                wtr.$write(*self)
            }

            fn id() -> i16 {
                -1
            }
        }
    };
}

macro_rules! impl_var {
    ($p: path, $read: ident, $write: ident) => {
        impl Protocol for $p {
            fn deserialize<R: Read>(rdr: &mut R) -> Result<$p> {
                rdr.$read().map($p)
            }

            fn serialize<W: Write>(&self, wtr: &mut W) -> Result<()> {
                let &$p(val) = self;
                wtr.$write(val)
            }

            fn id() -> i16 {
                -1
            }
        }
    };
}

impl_primitive!(u8, read_u8, write_u8);
impl_primitive!(u16, read_u16, write_u16);
impl_primitive!(u32, read_u32, write_u32);
impl_primitive!(u64, read_u64, write_u64);
impl_primitive!(i8, read_i8, write_i8);
impl_primitive!(i16, read_i16, write_i16);
impl_primitive!(i32, read_i32, write_i32);
impl_primitive!(i64, read_i64, write_i64);
impl_primitive!(f32, read_f32, write_f32);
impl_primitive!(f64, read_f64, write_f64);

impl_var!(VarShort, read_var_i16, write_var_i16);
impl_var!(VarUShort, read_var_u16, write_var_u16);
impl_var!(VarInt, read_var_i32, write_var_i32);
impl_var!(VarUInt, read_var_u32, write_var_u32);
impl_var!(VarLong, read_var_i64, write_var_i64);
impl_var!(VarULong, read_var_u64, write_var_u64);

impl Protocol for String {
    fn deserialize<R: Read>(rdr: &mut R) -> Result<String> {
        rdr.read_string()
    }

    fn serialize<W: Write>(&self, wtr: &mut W) -> Result<()> {
        wtr.write_string(self)
    }

    fn id() -> i16 {
        -1
    }
}

impl<P: Protocol> Protocol for Vec<P> {
    fn deserialize<R: Read>(rdr: &mut R) -> Result<Vec<P>> {
        let len = try!(rdr.read_i16());
        let mut res = Vec::new();
        for _ in (0..len) {
            res.push(try!(P::deserialize(rdr)));
        }
        Ok(res)
    }

    fn serialize<W: Write>(&self, wtr: &mut W) -> Result<()> {
        try!(wtr.write_i16(self.len() as i16));
        for v in self {
            try!(v.serialize(wtr));
        }
        Ok(())
    }

    fn id() -> i16 {
        -1
    }
}

impl<P: Protocol> Protocol for VarIntVec<P> {
    fn deserialize<R: Read>(rdr: &mut R) -> Result<VarIntVec<P>> {
        let len = try!(rdr.read_var_i32());
        let mut res = Vec::new();
        for _ in (0..len) {
            res.push(try!(P::deserialize(rdr)));
        }
        Ok(VarIntVec(res))
    }

    fn serialize<W: Write>(&self, wtr: &mut W) -> Result<()> {
        try!(wtr.write_i16(self.0.len() as i16));
        for v in &self.0 {
            try!(P::serialize(&v, wtr));
        }
        Ok(())
    }

    fn id() -> i16 {
        -1
    }
}

impl_type!(HelloConnectMessage, 3, salt| String, key| VarIntVec<u8>);

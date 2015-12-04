use std::io::{Read, Write, Result};
use io::{ReadExt, WriteExt};

#[derive(Clone, Debug)]
pub struct VarInt(pub i32);

#[derive(Clone, Debug)]
pub struct VarUInt(pub u32);

#[derive(Clone, Debug)]
pub struct VarShort(pub i16);

#[derive(Clone, Debug)]
pub struct VarUShort(pub u16);

#[derive(Clone, Debug)]
pub struct VarLong(pub i64);

#[derive(Clone, Debug)]
pub struct VarULong(pub u64);

#[derive(Clone, Debug)]
pub struct Flag(pub bool);

#[derive(Clone, Debug)]
pub struct VarIntVec<T>(pub Vec<T>);

#[derive(Clone, Debug)]
pub struct StaticVec<T>(pub Vec<T>);

fn get_flag(flag: u8, offset: u8) -> bool {
    assert!(offset < 8);
    flag & (1 << offset) != 0
}

fn set_flag(flag: u8, offset: u8, value: bool) -> u8 {
    assert!(offset < 8);
    if value {
        flag | (1 << offset)
    } else {
        flag & 255 - (1 << offset)
    }
}

pub trait Protocol: Sized {
    fn deserialize<R: Read>(&mut R) -> Result<Self>;
    fn serialize<W: Write>(&self, &mut W) -> Result<()>;

    #[inline(always)]
    fn id() -> i16;

    #[inline(always)]
    fn _deserialize<R: Read>(rdr: &mut R, _: &mut u8, _: &mut u8) -> Result<Self> {
        Self::deserialize(rdr)
    }

    #[inline(always)]
    fn _serialize<W: Write>(&self, wtr: &mut W, flag: &mut Option<u8>, _: &mut u8) -> Result<()> {
        if let Some(byte) = *flag {
            try!(wtr.write_u8(byte));
            *flag = None;
        }
        self.serialize(wtr)
    }

    fn as_packet_with_buf(&self, packet: &mut Vec<u8>) -> Result<()> {
        let mut buf = Vec::new();
        try!(self.serialize(&mut buf));
        packet.write_packet(Self::id() as u16, &buf)
    }

    fn as_packet(&self) -> Result<Vec<u8>> {
        let mut packet = Vec::new();
        try!(self.as_packet_with_buf(&mut packet));
        Ok(packet)
    }
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
impl_primitive!(bool, read_bool, write_bool);

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

impl Protocol for Flag {
    fn deserialize<R: Read>(_: &mut R) -> Result<Flag> {
        Ok(Flag(true))
    }

    fn serialize<W: Write>(&self, _: &mut W) -> Result<()> {
        Ok(())
    }

    fn _deserialize<R: Read>(rdr: &mut R, flag: &mut u8, offset: &mut u8) -> Result<Flag> {
        let real_offset = *offset % 8;
        if real_offset == 0 {
            *flag = try!(rdr.read_u8());
        }
        *offset += 1;
        Ok(Flag(get_flag(*flag, real_offset)))
    }

    fn _serialize<W: Write>(&self, wtr: &mut W, flag: &mut Option<u8>, offset: &mut u8)
                            -> Result<()> {

            let mut byte = flag.unwrap_or(0);

            if *offset == 8 {
                try!(wtr.write_u8(byte));
                *offset = 0;
            }

            byte = set_flag(byte, *offset, self.0);
            *offset += 1;
            *flag = Some(byte);
            Ok(())
    }

    fn id() -> i16 {
        -1
    }
}

impl<P: Protocol> Protocol for Vec<P> {
    fn deserialize<R: Read>(rdr: &mut R) -> Result<Vec<P>> {
        let len = try!(rdr.read_u16());
        let mut res = Vec::new();
        for _ in 0..len {
            res.push(try!(P::deserialize(rdr)));
        }
        Ok(res)
    }

    fn serialize<W: Write>(&self, wtr: &mut W) -> Result<()> {
        try!(wtr.write_u16(self.len() as u16));
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
        for _ in 0..len {
            res.push(try!(P::deserialize(rdr)));
        }
        Ok(VarIntVec(res))
    }

    fn serialize<W: Write>(&self, wtr: &mut W) -> Result<()> {
        try!(wtr.write_var_i32(self.0.len() as i32));
        for v in &self.0 {
            try!(P::serialize(&v, wtr));
        }
        Ok(())
    }

    fn id() -> i16 {
        -1
    }
}

impl<P: Protocol> Protocol for StaticVec<P> {
    fn deserialize<R: Read>(rdr: &mut R) -> Result<StaticVec<P>> {
        let mut res = Vec::new();
        for _ in 0..5 {
            res.push(try!(P::deserialize(rdr)));
        }
        Ok(StaticVec(res))
    }

    fn serialize<W: Write>(&self, wtr: &mut W) -> Result<()> {
        for v in &self.0 {
            try!(P::serialize(&v, wtr));
        }
        Ok(())
    }

    fn id() -> i16 {
        -1
    }
}

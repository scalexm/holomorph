use byteorder::{WriteBytesExt, ReadBytesExt, BigEndian};
use std::io::{self, Read, Write, Result};

macro_rules! read_var {
    ($rdr: expr, $t: ty, $size: expr) => {{
        let mut value: $t = 0;
        let mut offset = 0;

        while offset < $size {
          let b = try!(ReadExt::read_u8($rdr));
          value += ((b & 0x7F) as $t) << offset;
          if b & 0x80 != 0x80 {
              return Ok(value);
          }
          offset += 7;
        }

        Err(io::Error::new(io::ErrorKind::InvalidInput, "ill formed var integer"))
    }};
}

pub trait ReadExt: ReadBytesExt {
    #[inline]
    fn read_u8(&mut self) -> Result<u8> {
        ReadBytesExt::read_u8(self).map_err(From::from)
    }

    #[inline]
    fn read_u16(&mut self) -> Result<u16> {
        ReadBytesExt::read_u16::<BigEndian>(self).map_err(From::from)
    }

    #[inline]
    fn read_u32(&mut self) -> Result<u32> {
        ReadBytesExt::read_u32::<BigEndian>(self).map_err(From::from)
    }

    #[inline]
    fn read_u64(&mut self) -> Result<u64> {
        ReadBytesExt::read_u64::<BigEndian>(self).map_err(From::from)
    }

    #[inline]
    fn read_i8(&mut self) -> Result<i8> {
        ReadBytesExt::read_i8(self).map_err(From::from)
    }

    #[inline]
    fn read_i16(&mut self) -> Result<i16> {
        ReadBytesExt::read_i16::<BigEndian>(self).map_err(From::from)
    }

    #[inline]
    fn read_i32(&mut self) -> Result<i32> {
        ReadBytesExt::read_i32::<BigEndian>(self).map_err(From::from)
    }

    #[inline]
    fn read_i64(&mut self) -> Result<i64> {
        ReadBytesExt::read_i64::<BigEndian>(self).map_err(From::from)
    }

    #[inline]
    fn read_f32(&mut self) -> Result<f32> {
        ReadBytesExt::read_f32::<BigEndian>(self).map_err(From::from)
    }

    #[inline]
    fn read_f64(&mut self) -> Result<f64> {
        ReadBytesExt::read_f64::<BigEndian>(self).map_err(From::from)
    }

    #[inline]
    fn read_bool(&mut self) -> Result<bool> {
        ReadExt::read_u8(self).map(|val| val != 0).map_err(From::from)
    }

    fn read_var_i16(&mut self) -> Result<i16> {
        read_var!(self, i16, 16)
    }

    fn read_var_u16(&mut self) -> Result<u16> {
        self.read_var_i16().map(|val| val as u16)
    }

    fn read_var_i32(&mut self) -> Result<i32> {
        read_var!(self, i32, 32)
    }

    fn read_var_u32(&mut self) -> Result<u32> {
        self.read_var_i32().map(|val| val as u32)
    }

    fn read_var_i64(&mut self) -> Result<i64> {
        read_var!(self, i64, 64)
    }

    fn read_var_u64(&mut self) -> Result<u64> {
        self.read_var_i64().map(|val| val as u64)
    }

    fn read_string(&mut self) -> Result<String> {
        let len = try!(ReadExt::read_u16(self));
        let mut buf = vec![0; len as usize];
        try!(self.read_exact(&mut buf[0..]));

        match String::from_utf8(buf) {
            Ok(s) => Ok(s),
            Err(_) => {
                Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "bytes to utf8 conversion error"
                ))
            }
        }
    }
}

impl<R: Read + ?Sized> ReadExt for R {}

pub trait WriteExt: WriteBytesExt {
    #[inline]
    fn write_u8(&mut self, data: u8) -> Result<()> {
        WriteBytesExt::write_u8(self, data).map_err(From::from)
    }

    #[inline]
    fn write_u16(&mut self, data: u16) -> Result<()> {
        WriteBytesExt::write_u16::<BigEndian>(self, data).map_err(From::from)
    }

    #[inline]
    fn write_u32(&mut self, data: u32) -> Result<()> {
        WriteBytesExt::write_u32::<BigEndian>(self, data).map_err(From::from)
    }

    #[inline]
    fn write_u64(&mut self, data: u64) -> Result<()> {
        WriteBytesExt::write_u64::<BigEndian>(self, data).map_err(From::from)
    }

    #[inline]
    fn write_i8(&mut self, data: i8) -> Result<()> {
        WriteBytesExt::write_i8(self, data).map_err(From::from)
    }

    #[inline]
    fn write_i16(&mut self, data: i16) -> Result<()> {
        WriteBytesExt::write_i16::<BigEndian>(self, data).map_err(From::from)
    }

    #[inline]
    fn write_i32(&mut self, data: i32) -> Result<()> {
        WriteBytesExt::write_i32::<BigEndian>(self, data).map_err(From::from)
    }

    #[inline]
    fn write_i64(&mut self, data: i64) -> Result<()> {
        WriteBytesExt::write_i64::<BigEndian>(self, data).map_err(From::from)
    }

    #[inline]
    fn write_f32(&mut self, data: f32) -> Result<()> {
        WriteBytesExt::write_f32::<BigEndian>(self, data).map_err(From::from)
    }

    #[inline]
    fn write_f64(&mut self, data: f64) -> Result<()> {
        WriteBytesExt::write_f64::<BigEndian>(self, data).map_err(From::from)
    }

    #[inline]
    fn write_bool(&mut self, data: bool) -> Result<()> {
        WriteExt::write_u8(self, data as u8).map_err(From::from)
    }

    fn write_var_i16(&mut self, data: i16) -> Result<()> {
        self.write_var_u16(data as u16)
    }

    fn write_var_u16(&mut self, data: u16) -> Result<()> {
        self.write_var_u64((data as u64) & 0xFFFF)
    }

    fn write_var_i32(&mut self, data: i32) -> Result<()> {
        self.write_var_u32(data as u32)
    }

    fn write_var_u32(&mut self, data: u32) -> Result<()> {
        self.write_var_u64((data as u64) & 0xFFFFFFFF)
    }

    fn write_var_i64(&mut self, data: i64) -> Result<()> {
        self.write_var_u64(data as u64)
    }

    fn write_var_u64(&mut self, mut data: u64) -> Result<()> {
        while data >= 128 {
            try!(WriteExt::write_u8(self, (data & 127 | 128) as u8));
            data >>= 7;
        }
        WriteExt::write_u8(self, data as u8)
    }

    fn write_string(&mut self, data: &str) -> Result<()> {
        let len = data.len();
        try!(WriteExt::write_u16(self, len as u16));
        self.write_all(data.as_bytes())
    }

    fn write_packet(&mut self, id: u16, data: &Vec<u8>) -> Result<()> {
        let len = data.len();
        let nbytes: u16 = match len {
            0 => 0,
            1 ... 255 => 1,
            256 ... 65535 => 2,
            _ => 3,
        };

        try!(WriteExt::write_u16(self, id << 2 | nbytes));

        match nbytes {
            0 => (),
            1 => try!(WriteExt::write_u8(self, len as u8)),
            2 => try!(WriteExt::write_u16(self, len as u16)),
            3 => {
                try!(WriteExt::write_u8(self, ((len >> 16) & 255) as u8));
                try!(WriteExt::write_u16(self, (len & 65535) as u16))
            }
            _ => unreachable!(),
        }

        self.write_all(&data[0..])
    }
}

impl<W: Write + ?Sized> WriteExt for W {}

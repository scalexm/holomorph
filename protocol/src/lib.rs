//! A crate defining utilities for working with the Dofus protocol. Notable
//! things:
//! * the Dofus protocol is big-endian encoded
//! * strings are encoded by prefixing their length as a big-endian u16
//! * integers can be dynamically encoded on a variable number of bytes:
//!   they are partitioned in groups of 7 bits, and each group from left to
//!   right (in big-endian representation) is iteratively encoded as one byte
//!   where the 7 least significant bits are the ones being encoded, and the
//!   most significant bit is set to 1 if there are still other groups to
//!   encode and 0 otherwise. See the `Decode` and `Encode` implementations for
//!   `Var`.
//! * arrays are *usually* encoded by prefixing their length as a big-endian u16,
//!   but the length can also be encoded as a dynamic `u32`
//!
//! This crate defines the `Decode` and `Encode` traits which can be auto-derived
//! with the use of the `protocol_derive` proc-macro, as well as a `Framed` wrapper
//! to decode / encode Dofus frames from / to a `TcpStream`.
//!
//! This crate also contains the definitions of all the messages and types used
//! in the Dofus protocol, auto-generated from the decompiled sources of the Dofus
//! client with the use of a small CLI tool to be found in `protocol_gen`.

#![feature(specialization)]
#![feature(async_await)]
#![deny(rust_2018_idioms)]

pub mod constants;
pub mod frame;
pub mod messages;
#[cfg(test)]
mod test;
pub mod types;
pub mod variants;

use bytes::{Buf, BufMut, BytesMut};
use std::borrow::Cow;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
/// An error occuring when decoding a protocol object.
pub enum Error {
    NotEnoughData,
    IllFormedVarInteger,
    Utf8Error,
    UnknownVariant(u16),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NotEnoughData => write!(f, "not enough data"),
            Error::IllFormedVarInteger => write!(f, "ill formed var integer"),
            Error::Utf8Error => write!(f, "invalid utf8 string"),
            Error::UnknownVariant(id) => write!(f, "unknown variant with id {}", id),
        }
    }
}

/// Types implementing the [Decode](self::Decode) trait can be decoded
/// from a `&[u8]` with the Dofus protocol conventions.
pub trait Decode<'a>: Sized {
    const ID: u16 = 0;

    fn decode(src: &mut &'a [u8]) -> Result<Self, Error>;
}

/// Types implementing the [Encode](self::Encode) trait can be encoded to
/// a `BytesMut` with the Dofus protocol conventions.
pub trait Encode {
    const ID: u16 = 0;

    fn encode(&self, dst: &mut BytesMut);
}

impl<T: Encode + ?Sized> Encode for &T {
    const ID: u16 = T::ID;

    fn encode(&self, dst: &mut BytesMut) {
        (*self).encode(dst)
    }
}

impl<'a, T: ?Sized> Decode<'a> for std::marker::PhantomData<T> {
    #[inline]
    fn decode(_src: &mut &'a [u8]) -> Result<Self, Error> {
        Ok(std::marker::PhantomData)
    }
}

impl<T: ?Sized> Encode for std::marker::PhantomData<T> {
    #[inline]
    fn encode(&self, _dst: &mut BytesMut) {}
}

macro_rules! impl_primitive {
    ($t: ty, $read: ident, $write: ident) => {
        impl<'a> Decode<'a> for $t {
            #[inline]
            fn decode(src: &mut &'a [u8]) -> Result<Self, Error> {
                if src.len() < std::mem::size_of::<$t>() {
                    return Err(Error::NotEnoughData);
                }
                Ok(src.$read())
            }
        }

        impl Encode for $t {
            #[inline]
            fn encode(&self, dst: &mut BytesMut) {
                dst.reserve(std::mem::size_of::<$t>());
                dst.$write(*self);
            }
        }
    };
}

impl_primitive!(i8, get_i8, put_i8);
impl_primitive!(i16, get_i16, put_i16);
impl_primitive!(i32, get_i32, put_i32);
impl_primitive!(u8, get_u8, put_u8);
impl_primitive!(u16, get_u16, put_u16);
impl_primitive!(u32, get_u32, put_u32);
impl_primitive!(f32, get_f32, put_f32);
impl_primitive!(f64, get_f64, put_f64);

impl<'a> Decode<'a> for bool {
    #[inline]
    fn decode(src: &mut &'a [u8]) -> Result<Self, Error> {
        u8::decode(src).map(|v| v != 0)
    }
}

impl Encode for bool {
    #[inline]
    fn encode(&self, dst: &mut BytesMut) {
        (*self as u8).encode(dst);
    }
}

impl<'a, T: Decode<'a>> Decode<'a> for Vec<T> {
    #[inline]
    fn decode(src: &mut &'a [u8]) -> Result<Self, Error> {
        let len = u16::decode(src)? as usize;
        let mut res = Vec::with_capacity(len);
        for _ in 0..len {
            res.push(T::decode(src)?);
        }
        Ok(res)
    }
}

impl<T: Encode> Encode for [T] {
    #[inline]
    default fn encode(&self, dst: &mut BytesMut) {
        (self.len() as u16).encode(dst);
        for v in self {
            v.encode(dst);
        }
    }
}

impl<'a, T: Clone + Decode<'a>> Decode<'a> for Cow<'a, [T]> {
    #[inline]
    fn decode(src: &mut &'a [u8]) -> Result<Self, Error> {
        Vec::decode(src).map(Into::into)
    }
}

impl<'a, T: Clone + Encode> Encode for Cow<'a, [T]> {
    #[inline]
    fn encode(&self, dst: &mut BytesMut) {
        (**self).encode(dst);
    }
}

/// Special impl for `&[u8]` as it can be borrowed directly from the source
/// slice.
impl<'a> Decode<'a> for &'a [u8] {
    #[inline]
    fn decode(src: &mut &'a [u8]) -> Result<Self, Error> {
        let len = u16::decode(src)? as usize;
        if src.len() < len {
            return Err(Error::NotEnoughData);
        }
        let v = &src[0..len];
        *src = &src[len..];
        Ok(v)
    }
}

impl Encode for [u8] {
    #[inline]
    fn encode(&self, dst: &mut BytesMut) {
        (self.len() as u16).encode(dst);
        dst.extend_from_slice(self);
    }
}

/// Special impl for the same reasons as `&[u8]`.
impl<'a> Decode<'a> for &'a [i8] {
    #[inline]
    fn decode(src: &mut &'a [u8]) -> Result<Self, Error> {
        <&'a [u8] as Decode>::decode(src)
            .map(|v| unsafe { std::slice::from_raw_parts(v.as_ptr() as *const i8, v.len()) })
    }
}

impl Encode for [i8] {
    #[inline]
    fn encode(&self, dst: &mut BytesMut) {
        (self.len() as u16).encode(dst);
        dst.extend_from_slice(unsafe {
            std::slice::from_raw_parts(self.as_ptr() as *const u8, self.len())
        });
    }
}

/// Special impl for the same reasons as `&[u8]`.
impl<'a> Decode<'a> for &'a str {
    #[inline]
    fn decode(src: &mut &'a [u8]) -> Result<Self, Error> {
        let bytes: &'a [u8] = Decode::decode(src)?;
        std::str::from_utf8(bytes).map_err(|_| Error::Utf8Error)
    }
}

impl Encode for str {
    #[inline]
    fn encode(&self, dst: &mut BytesMut) {
        (self.len() as u16).encode(dst);
        dst.extend_from_slice(self.as_bytes());
    }
}

/// A type marking the need of using a dynamic number of bytes to decode or
/// encode an integer type. Internally used in the `#[derive(...)]` proc-macro.
pub struct Var<T>(pub T);

macro_rules! impl_var_primitive {
    ($t: ty, $signed: ty, $read: ident, $write: ident) => {
        impl<'a> Decode<'a> for Var<$t> {
            #[inline]
            fn decode(src: &mut &'a [u8]) -> Result<Self, Error> {
                let len = std::cmp::min(src.len(), std::mem::size_of::<$t>() * 8 / 7 + 1);
                let mut value: $signed = 0;
                let mut offset = 0;
                while offset < len {
                    let b = src.get_u8();
                    value += ((b & 0x7F) as $signed) << 7 * offset;
                    if b & 0x80 == 0 {
                        return Ok(Var(value as _));
                    }
                    offset += 1;
                }
                Err(Error::IllFormedVarInteger)
            }
        }

        impl Encode for Var<$t> {
            #[inline]
            fn encode(&self, dst: &mut BytesMut) {
                dst.reserve(std::mem::size_of::<$t>() * 8 / 7 + 1);
                let mut offset = unsafe { dst.as_mut_ptr().add(dst.len()) };
                let mut x = self.0 as $signed;
                while x >= 0x80 {
                    unsafe {
                        *offset = ((x & 0x7F) | 0x80) as u8;
                        offset = offset.add(1);
                    }
                    x >>= 7;
                }
                let new_len = offset as usize - (dst.as_ptr() as usize) + 1;
                unsafe {
                    *offset = x as u8;
                    dst.set_len(new_len);
                }
            }
        }
    };
}

impl_var_primitive!(i16, u16, read_var_i16, write_var_i16);
impl_var_primitive!(i32, u32, read_var_i32, write_var_i32);
impl_var_primitive!(i64, u64, read_var_i64, write_var_i64);
impl_var_primitive!(u16, u16, read_var_u16, write_var_u16);
impl_var_primitive!(u32, u32, read_var_u32, write_var_u32);
impl_var_primitive!(u64, u64, read_var_u64, write_var_u64);

impl<'a> Decode<'a> for Var<&'a [u8]> {
    #[inline]
    fn decode(src: &mut &'a [u8]) -> Result<Self, Error> {
        let len = Var::<u32>::decode(src)?.0 as usize;
        if src.len() < len {
            return Err(Error::NotEnoughData);
        }
        let v = &src[0..len];
        *src = &src[len..];
        Ok(Var(v))
    }
}

impl Encode for Var<&[u8]> {
    #[inline]
    fn encode(&self, dst: &mut BytesMut) {
        Var(self.0.len() as u32).encode(dst);
        dst.extend_from_slice(self.0);
    }
}

impl<'a> Decode<'a> for Var<&'a [i8]> {
    #[inline]
    fn decode(src: &mut &'a [u8]) -> Result<Self, Error> {
        <Var<&'a [u8]> as Decode>::decode(src).map(|v| unsafe {
            Var(std::slice::from_raw_parts(
                v.0.as_ptr() as *const i8,
                v.0.len(),
            ))
        })
    }
}

impl Encode for Var<&[i8]> {
    #[inline]
    fn encode(&self, dst: &mut BytesMut) {
        Var(self.0.len() as u32).encode(dst);
        dst.extend_from_slice(unsafe {
            std::slice::from_raw_parts(self.0.as_ptr() as *const u8, self.0.len())
        });
    }
}

// Other array types are handled through the special attributes
// `#[protocol(var_length)]` and `#[protocol(var_content)]` because both their
// length or their contents can be dynamically encoded, and I could not find
// an ergonomic way to deal with it efficiently with the type system only.

macro_rules! impl_array {
    ($N: expr, $($tok: ident),*) => {
        impl<'a, T: Decode<'a>> Decode<'a> for [T; $N] {
            #[inline]
            fn decode(src: &mut &'a [u8]) -> Result<Self, Error> {
                Ok([$(
                    T::decode(src).map_err(|err| {
                        let $tok: ();
                        err
                    })?,
                )*])
            }
        }

        impl<T: Encode> Encode for [T; $N] {
            #[inline]
            fn encode(&self, dst: &mut BytesMut) {
                for x in self {
                    x.encode(dst);
                }
            }
        }
    };
}

impl_array!(1, _a);
impl_array!(2, _a, _b);
impl_array!(3, _a, _b, _c);
impl_array!(4, _a, _b, _c, _d);
impl_array!(5, _a, _b, _c, _d, _e);

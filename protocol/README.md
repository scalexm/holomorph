# protocol

A crate defining utilities for working with the Dofus protocol. Notable
things:
* the Dofus protocol is big-endian encoded
* strings are encoded by prefixing their length as a big-endian u16
* integers can be dynamically encoded on a variable number of bytes:
  they are partitioned in groups of 7 bits, and each group from left to
  right (in big-endian representation) is iteratively encoded as one byte
  where the 7 least significant bits are the ones being encoded, and the
  most significant bit is set to 1 if there are still other groups to
  encode and 0 otherwise. See the `Decode` and `Encode` implementations for
  `Var`.
* arrays are *usually* encoded by prefixing their length as a big-endian u16,
  but the length can also be encoded as a dynamic `u32`

This crate defines the `Decode` and `Encode` traits which can be auto-derived
with the use of the `protocol_derive` proc-macro, as well as a `Codec` type
implementing `tokio_codec::Encoder` and `tokio_codec::Decoder` to decode /
encode frames from / to a `TcpStream`.

This crate also contains the definitions of all the messages and types used
in the Dofus protocol, auto-generated from the decompiled sources of the Dofus
client with the use of a small CLI tool to be found in `gen/main.rs`.

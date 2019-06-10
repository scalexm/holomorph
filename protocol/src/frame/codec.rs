use crate::frame::Frame;
use crate::Encode;
use bytes::{Buf, BytesMut};
use std::io;

#[derive(Debug)]
enum DecodeState {
    Header,
    Length { id: u16, nbytes: u16 },
    Payload { id: u16, length: u32 },
}

#[derive(Debug)]
pub struct Codec {
    state: DecodeState,
}

impl Codec {
    pub fn new() -> Self {
        Self {
            state: DecodeState::Header,
        }
    }
}

impl Codec {
    fn decode_length(&mut self, id: u16, nbytes: u16, src: &mut BytesMut) -> Option<Frame> {
        if src.len() < nbytes as _ {
            return None;
        }

        let length = src.get_uint(nbytes as _) as u32;

        src.reserve(length as _);
        self.state = DecodeState::Payload { id, length };
        self.decode_payload(id, length, src)
    }

    fn decode_payload(&mut self, id: u16, length: u32, src: &mut BytesMut) -> Option<Frame> {
        if src.len() < length as _ {
            return None;
        }

        src.reserve(2);
        self.state = DecodeState::Header;
        Some(Frame::new(id, src.split_to(length as _)))
    }
}

impl Codec {
    pub fn decode(&mut self, src: &mut BytesMut) -> io::Result<Option<Frame>> {
        match self.state {
            DecodeState::Header => {
                // header + sequence id
                if src.len() < 6 {
                    return Ok(None);
                }

                let header = src.get_u16();
                let id = header >> 2;
                let nbytes = header & 3;

                src.advance(4); // skip the sequence id

                if nbytes == 0 {
                    return Ok(Some(Frame::new(id, BytesMut::new())));
                } else if nbytes == 3 {
                    // Limit the maximum length to 64KiB.
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "payload too big",
                    ));
                }

                src.reserve(nbytes as _);
                self.state = DecodeState::Length { id, nbytes };
                Ok(self.decode_length(id, nbytes, src))
            }

            DecodeState::Length { id, nbytes } => Ok(self.decode_length(id, nbytes, src)),

            DecodeState::Payload { id, length } => Ok(self.decode_payload(id, length, src)),
        }
    }

    pub fn decode_eof(&mut self, buf: &mut BytesMut) -> Result<Option<Frame>, io::Error> {
        match self.decode(buf)? {
            Some(frame) => Ok(Some(frame)),
            None => {
                if buf.is_empty() {
                    Ok(None)
                } else {
                    Err(io::Error::new(io::ErrorKind::Other, "bytes remaining on stream").into())
                }
            }
        }
    }
}

impl Codec {
    pub fn encode<M: Encode>(&mut self, msg: M, dst: &mut BytesMut) -> io::Result<()> {
        use bytes::BufMut;

        // We always encode the length on two bytes because we don't care.
        dst.reserve(4);
        dst.put_u16(M::ID << 2 | 2);
        dst.put_u16(0); // keep 2 bytes of free space for the length

        // First encode the payload.
        let old_len = dst.len();
        msg.encode(dst);
        let new_len = dst.len();
        let payload_len = new_len - old_len;

        if payload_len >= 1 << 16 {
            // Limit the maximum length to 64KiB.
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "payload too big",
            ));
        }

        // Then go back in time and encode the length.
        let mut space = &mut dst[new_len - payload_len - 2..];
        space.put_u16(payload_len as _);

        Ok(())
    }
}

use bytes::{BufMut, BytesMut};

#[derive(Debug)]
enum DecodeState {
    Header,
    Length { id: u16, nbytes: u16 },
    Payload { id: u16, length: u32 },
}

#[derive(Debug)]
/// A type for encoding and decoding frames with the Dofus protocol convention,
/// implementing both [Decoder](tokio_codec::Decoder) and
/// [Encoder](tokio_codec::Encoder). A frame within the Dofus protocol is made
/// of:
/// * a two bytes header: the 14 most significant bits encode the message id,
///   while the 2 least significant ones encode how much bytes the payload
///   length is encoded on
/// * a 4 bytes sequence id
/// * the payload length (encoded on 0 to 3 bytes)
/// * the payload itself consisting of exactly one encoded message
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

enum Payload {
    Bytes(BytesMut),
    Vec(Vec<u8>),
}

pub struct Frame {
    id: u16,
    payload: Payload,
}

impl Frame {
    pub fn new(id: u16, payload: Vec<u8>) -> Self {
        Self {
            id,
            payload: Payload::Vec(payload),
        }
    }

    pub fn id(&self) -> u16 {
        self.id
    }

    pub fn payload(&self) -> &[u8] {
        match &self.payload {
            Payload::Bytes(bytes) => &bytes,
            Payload::Vec(vec) => &vec,
        }
    }
}

impl Codec {
    fn decode_length(&mut self, id: u16, nbytes: u16, src: &mut BytesMut) -> Option<Frame> {
        use bytes::{BigEndian, ByteOrder};

        if src.len() < nbytes as _ {
            return None;
        }

        let length = BigEndian::read_uint(&src, nbytes as _) as u32;

        src.advance(nbytes as _);
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
        Some(Frame {
            id,
            payload: Payload::Bytes(src.split_to(length as _)),
        })
    }
}

impl tokio_codec::Decoder for Codec {
    type Item = Frame;
    type Error = std::io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> std::io::Result<Option<Frame>> {
        use bytes::{BigEndian, ByteOrder};

        match self.state {
            DecodeState::Header => {
                // header + sequence id
                if src.len() < 6 {
                    return Ok(None);
                }

                let header = BigEndian::read_u16(&src);
                src.advance(6); // skip the sequence id
                let id = header >> 2;
                let nbytes = header & 3;

                if nbytes == 0 {
                    return Ok(Some(Frame::new(id, vec![])));
                }

                src.reserve(nbytes as _);
                self.state = DecodeState::Length { id, nbytes };
                Ok(self.decode_length(id, nbytes, src))
            }

            DecodeState::Length { id, nbytes } => Ok(self.decode_length(id, nbytes, src)),

            DecodeState::Payload { id, length } => Ok(self.decode_payload(id, length, src)),
        }
    }
}

impl tokio_codec::Encoder for Codec {
    type Item = Frame;
    type Error = std::io::Error;

    fn encode(&mut self, frame: Frame, dst: &mut BytesMut) -> std::io::Result<()> {
        let len = frame.payload().len();
        let nbytes = match len {
            0 => 0,
            1..=0xFF => 1,
            0x100..=0xFFFF => 2,
            0x10000..=0xFFFFFF => 3,
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "payload too big",
                ))
            }
        };

        dst.reserve(2 + nbytes + len);
        dst.put_u16_be(frame.id << 2 | (nbytes as u16));
        dst.put_uint_be(len as _, nbytes);
        dst.put_slice(frame.payload());

        Ok(())
    }
}

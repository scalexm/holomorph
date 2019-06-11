mod codec;

use self::codec::Codec;
use crate::Encode;
use bytes::BytesMut;
use futures::io::{AsyncRead, AsyncWrite};
use futures::stream::Stream;
use futures::task::{Context, Poll};
use pin_utils::{unsafe_pinned, unsafe_unpinned};
use std::io;
use std::pin::Pin;

const INITIAL_CAPACITY: usize = 8 * 1024;

#[derive(Debug)]
/// A type wrapping an async handle for encoding and decoding frames with the
/// Dofus protocol convention. A frame within the Dofus protocol is made of:
/// * a two bytes header: the 14 most significant bits encode the message id,
///   while the 2 least significant ones encode how much bytes the payload
///   length is encoded on
/// * a 4 bytes sequence id
/// * the payload length (encoded on 0 to 3 bytes)
/// * the payload itself consisting of exactly one encoded message
pub struct Framed<T> {
    inner: T,
    codec: Codec,
    eof: bool,
    is_readable: bool,
    read_buf: BytesMut,
    write_buf: BytesMut,
}

impl<T: Unpin> Unpin for Framed<T> {}

impl<T> Framed<T> {
    unsafe_pinned!(inner: T);
    unsafe_unpinned!(codec: Codec);
    unsafe_unpinned!(eof: bool);
    unsafe_unpinned!(is_readable: bool);
    unsafe_unpinned!(read_buf: BytesMut);
    unsafe_unpinned!(write_buf: BytesMut);
}

impl<T> Framed<T> {
    pub fn new(inner: T) -> Framed<T> {
        Framed {
            inner,
            codec: Codec::new(),
            eof: false,
            is_readable: false,
            read_buf: BytesMut::with_capacity(INITIAL_CAPACITY),
            write_buf: BytesMut::with_capacity(INITIAL_CAPACITY),
        }
    }

    pub fn get_ref(&self) -> &T {
        &self.inner
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    pub fn into_inner(self) -> T {
        self.inner
    }
}

// The `Stream` implementation is adapted from `tokio-codec`
// (https://github.com/tokio-rs/tokio) and was updated to use the newest
// futures API.

impl<T: AsyncRead> Stream for Framed<T> {
    type Item = Result<Frame, io::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        use bytes::BufMut;
        use futures::ready;

        loop {
            // Repeatedly call `decode` or `decode_eof` as long as it is
            // "readable". Readable is defined as not having returned `None`. If
            // the upstream has returned EOF, and the decoder is no longer
            // readable, it can be assumed that the decoder will never become
            // readable again, at which point the stream is terminated.
            if self.is_readable {
                if self.eof {
                    let (codec, buf) = unsafe {
                        let ref_mut = self.as_mut().get_unchecked_mut();
                        (&mut ref_mut.codec, &mut ref_mut.read_buf)
                    };
                    let frame = codec.decode_eof(buf);
                    return Poll::Ready(frame.transpose());
                }

                let (codec, buf) = unsafe {
                    let ref_mut = self.as_mut().get_unchecked_mut();
                    (&mut ref_mut.codec, &mut ref_mut.read_buf)
                };
                if let Some(frame) = codec.decode(buf).transpose() {
                    return Poll::Ready(Some(frame));
                }

                *self.as_mut().is_readable() = false;
            }

            assert!(!self.eof);

            // Otherwise, try to read more data and try again. Make sure we've
            // got room for at least one byte to read to ensure that we don't
            // get a spurious 0 that looks like EOF
            self.as_mut().read_buf().reserve(1);

            let n = unsafe {
                let (inner, buf) = {
                    let ref_mut = self.as_mut().get_unchecked_mut();
                    (
                        Pin::new_unchecked(&mut ref_mut.inner),
                        &mut ref_mut.read_buf,
                    )
                };
                let b = buf.bytes_mut();
                inner.initializer().initialize(b);
                let n = match ready!(inner.poll_read(cx, b)) {
                    Ok(n) => n,
                    Err(err) => return Poll::Ready(Some(Err(err.into()))),
                };
                buf.advance_mut(n);
                n
            };

            if n == 0 {
                *self.as_mut().eof() = true;
            }

            *self.as_mut().is_readable() = true;
        }
    }
}

impl<T> Framed<T> {
    pub fn write<M: Encode>(&mut self, msg: M) -> io::Result<()> {
        self.codec.encode(msg, &mut self.write_buf)
    }
}

impl<T: AsyncWrite + Unpin> Framed<T> {
    pub async fn send<M: Encode>(&mut self, msg: M) -> io::Result<()> {
        self.write(msg)?;
        self.flush().await
    }

    pub async fn flush(&mut self) -> io::Result<()> {
        use futures::AsyncWriteExt;

        let buf = self.write_buf.split_to(self.write_buf.len());
        self.inner.write_all(&buf).await?;
        self.inner.flush().await
    }
}

pub struct Frame {
    id: u16,
    payload: BytesMut,
}

impl Frame {
    fn new(id: u16, payload: BytesMut) -> Self {
        Self {
            id,
            payload: payload,
        }
    }

    pub fn id(&self) -> u16 {
        self.id
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
}

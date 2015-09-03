use mio::{TryRead, TryWrite, Token};
use mio::tcp::{TcpStream, Shutdown};
use std::io::{self, Read, Cursor};
use io::ReadExt;
use std::collections::VecDeque;

struct Buffer(Vec<u8>, usize);
pub struct Packet(pub u16, pub Cursor<Vec<u8>>);

fn make_buffer(len: usize) -> Buffer {
    Buffer(vec![0; len], 0)
}

pub struct Connection {
    listener_token: Token,
    socket: TcpStream,
    read_buffer: Option<Buffer>,
    write_buffer: VecDeque<Buffer>,
    state: State,
    close_on_next_write: bool,
}

enum State {
    WaitingForHeader,
    WaitingForLen(u16),
    WaitingForData(u16),
}

impl Connection {
    pub fn new(socket: TcpStream, listener_token: Token)
        -> Connection {

        Connection {
            listener_token: listener_token,
            socket: socket,
            read_buffer: Some(make_buffer(2)),
            write_buffer: VecDeque::new(),
            state: State::WaitingForHeader,
            close_on_next_write: false,
        }
    }

    pub fn listener(&self) -> Token {
        self.listener_token
    }

    pub fn socket(&self) -> &TcpStream {
        &self.socket
    }

    pub fn readable(&mut self) -> io::Result<Option<Packet>> {
        let Buffer(mut buf, pos) = self.read_buffer.take().unwrap();

        let s = match try!(self.socket.try_read(&mut buf[pos..])) {
            None | Some(0) => return Err(io::Error::new(io::ErrorKind::Other, "EOF")),
            Some(s) => s,
        };

        if pos + s != buf.len() {
            self.read_buffer = Some(Buffer(buf, pos + s));
            return Ok(None);
        }

        let mut buf = Cursor::new(buf);
        match self.state {
            State::WaitingForHeader => {
                let header = try!(buf.read_u16());
                let id = header >> 2;
                let nbytes = header & 3;

                if nbytes == 0 {
                    self.state = State::WaitingForHeader;
                    self.read_buffer = Some(make_buffer(2));

                    return Ok(Some(Packet(id, Cursor::new(Vec::new()))));
                }

                self.state = State::WaitingForLen(id);
                self.read_buffer = Some(make_buffer(nbytes as usize));
            }

            State::WaitingForLen(id) => {
                let mut len = 0u32;
                for _ in 0..buf.get_ref().len() {
                    len = (len << 8) + (try!(buf.read_u8()) as u32);
                }
                self.state = State::WaitingForData(id);
                self.read_buffer = Some(make_buffer(len as usize));
            }

            State::WaitingForData(id) => {
                self.state = State::WaitingForHeader;
                self.read_buffer = Some(make_buffer(2));

                return Ok(Some(Packet(id, buf)));
            }
        }

        Ok(None)
    }

    pub fn writable(&mut self) -> io::Result<bool> {

        if self.write_buffer.is_empty() {
            return Ok(true)
        }

        while !self.write_buffer.is_empty() {
            {
                let buf = self.write_buffer.back_mut().unwrap();
                let s = match try!(self.socket.try_write(&buf.0[buf.1..])) {
                    None => return Ok(false),
                    Some(s) => s,
                };

                if buf.1 + s != buf.0.len() {
                    buf.1 += s;
                    return Ok(false);
                }
            }

            let _ = self.write_buffer.pop_back().unwrap();
            if self.close_on_next_write {
                let _ = self.socket.shutdown(Shutdown::Both);
                return Ok(false);
            }
        }

        Ok(true)
    }

    pub fn push(&mut self, buffer: Vec<u8>, close: bool) {
        self.close_on_next_write = close;
        self.write_buffer.push_front(Buffer(buffer, 0));
    }
}

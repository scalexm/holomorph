extern crate rand;
extern crate shared;

use std::net::TcpStream;
use std::thread;
use rand::Rng;
use std::io;
use std::io::{Read, Write};
use shared::io::{ReadExt, WriteExt};

fn main() {

        let secret_number = rand::thread_rng().gen_range(1, 101);
        let mut stream = TcpStream::connect("127.0.0.1:2000").unwrap();

        let mut packet = Vec::new();
        let mut buffer = Vec::new();
        packet.write_string("salut");
        buffer.write_packet(12, &packet);
        stream.write_all(&buffer[0..]);
        stream.flush();

        let header = stream.read_u16().unwrap();
        let id = header >> 2;
        let len = stream.read_u8().unwrap();
        println!("{} {}", id, len);
        println!("{}", stream.read_string().unwrap());
}

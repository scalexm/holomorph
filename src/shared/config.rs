use rustc_serialize::json;
use rustc_serialize::{Encodable, Decodable};
use std::fs::File;
use std::io::{Read, Write};

pub trait Config {
    fn default() -> Self;
}

// uses the json format
pub fn from_file<C: Config + Decodable + Encodable>(path: &str) -> C {
    match File::open(path) {
        Ok(mut f) => {
            let mut decode = String::new();
            f.read_to_string(&mut decode).unwrap();
            json::decode(&decode).unwrap()
        }

        Err(_) => {
            let config = C::default();
            let mut encode = Vec::new();
            write!(&mut encode, "{}", json::as_pretty_json(&config)).unwrap();
            let _ = File::create(path).map(|mut f| f.write_all(&encode[0..]));

            config
        }
    }
}

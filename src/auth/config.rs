use rustc_serialize::json;
use std::fs::File;
use std::io::{Read, Write};

#[derive(RustcDecodable, RustcEncodable)]
pub struct Config {
    pub key_path: String,
    pub patch_path: String,
    pub num_threads: usize,
    pub bind_address: String,
    pub database_uri: String,
}

impl Config {
    fn new() -> Config {
        Config {
            key_path: "dofus.key".to_string(),
            patch_path: "patch.swf".to_string(),
            num_threads: 2,
            bind_address: "127.0.0.1:2000".to_string(),
            database_uri: "postgres://user:pass@localhost:5432/holomorph_auth".to_string(),
        }
    }
}

pub fn from_file(path: &str) -> Config {
    match File::open(path) {
        Ok(mut f) => {
            let mut decode = String::new();
            f.read_to_string(&mut decode).unwrap();
            json::decode(&decode).unwrap()
        }

        Err(_) => {
            let config = Config::new();
            let mut encode = Vec::new();

            write!(&mut encode, "{}", json::as_pretty_json(&config)).unwrap();
            let _ = File::create(path).map(|mut f| f.write_all(&encode[0..]));

            config
        }
    }
}

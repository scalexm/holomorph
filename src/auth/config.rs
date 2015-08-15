use shared::config;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Config {
    pub key_path: String,
    pub patch_path: String,
    pub num_threads: usize,
    pub bind_address: String,
    pub game_bind_address: String,
    pub database_uri: String,
}

impl config::Config for Config {
    fn default() -> Config {
        Config {
            key_path: "dofus.key".to_string(),
            patch_path: "patch.swf".to_string(),
            num_threads: 2,
            bind_address: "127.0.0.1:5555".to_string(),
            game_bind_address: "127.0.0.1:5556".to_string(),
            database_uri: "postgres://user:pass@localhost:5432/holomorph_auth".to_string(),
        }
    }
}

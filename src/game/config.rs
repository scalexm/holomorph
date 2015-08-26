use shared::config;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Config {
    pub auth_address: String,
    pub bind_ip: String,
    pub bind_port: i16,
    pub server_key: String,
    pub server_id: i16,
    pub auth_database_uri: String,
    pub auth_database_threads: usize,
    pub database_uri: String,
    pub database_threads: usize,
}

impl config::Config for Config {
    fn default() -> Config {
        Config {
            auth_address: "127.0.0.1:5556".to_string(),
            bind_ip: "127.0.0.1".to_string(),
            bind_port: 5557,
            server_key: "salut".to_string(),
            server_id: 1,
            auth_database_uri: "postgres://user:pass@localhost:5432/holomorph_auth".to_string(),
            auth_database_threads: 1,
            database_uri: "postgres://user:pass@localhost:5432/holomorph_game".to_string(),
            database_threads: 1,
        }
    }
}

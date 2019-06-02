use serde::Deserialize;

#[derive(Clone, PartialEq, Eq, Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

#[derive(Clone, PartialEq, Eq, Debug, Deserialize)]
pub struct ServerConfig {
    pub bind_address: String,
    pub private_key: PrivateKeyConfig,
}

#[derive(Clone, PartialEq, Eq, Debug, Deserialize)]
pub struct PrivateKeyConfig {
    pub path: String,
}

#[derive(Clone, PartialEq, Eq, Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub pool_size: u32,
}

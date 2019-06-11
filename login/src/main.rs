#![feature(async_await)]
#![deny(rust_2018_idioms)]

#[macro_use]
extern crate diesel;

mod config;
mod server;
mod session;

use config::Config;
use diesel::r2d2;
use futures::prelude::*;
use log::{error, info};
use openssl::rsa::{Padding, Rsa};
use runtime::net::TcpListener;
use server::Server;
use session::Session;
use std::sync::Arc;

const RSA_KEY_SIZE: u32 = 256;

#[runtime::main(runtime_tokio::Tokio)]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let config_path = std::env::args().nth(1).unwrap_or("config.toml".to_string());

    let config: Config = match toml::from_str(&std::fs::read_to_string(config_path)?) {
        Ok(config) => config,
        Err(err) => {
            error!("could not parse config: {}", err);
            std::process::exit(1);
        }
    };

    let signature_key =
        match Rsa::private_key_from_pem(&std::fs::read(&config.server.private_key.path)?) {
            Ok(key) => key,
            Err(err) => {
                error!("could not load RSA private key: {}", err);
                std::process::exit(1);
            }
        };

    let keypair = Rsa::generate(RSA_KEY_SIZE * 8).expect("could not generate RSA key pair");

    let mut public_key = vec![0; signature_key.size() as _];
    let n = match signature_key.private_encrypt(
        &keypair
            .public_key_to_der()
            .expect("could not DER-encode generated RSA public key"),
        &mut public_key[..],
        Padding::PKCS1,
    ) {
        Ok(n) => n,
        Err(err) => {
            error!("could not sign generated RSA public key: {}", err);
            std::process::exit(1);
        }
    };
    public_key.truncate(n);

    let database_pool = match r2d2::Builder::new()
        .max_size(config.database.pool_size)
        .build(r2d2::ConnectionManager::new(&config.database.url))
    {
        Ok(pool) => pool,
        Err(err) => {
            error!("could not build database pool: {}", err);
            std::process::exit(1);
        }
    };

    let mut server = Server {
        config,
        database_pool,
        public_key,
        private_key: keypair,
        game_servers: Default::default(),
    };

    if let Err(err) = server.load() {
        error!("could not load server: {}", err);
        std::process::exit(1);
    }

    let mut listener = TcpListener::bind(&server.config.server.bind_address)?;
    let mut incoming = listener.incoming();

    info!("listening on {}", server.config.server.bind_address);
    let server = Arc::new(server);
    while let Some(stream) = incoming.next().await {
        let server = server.clone();
        runtime::spawn(async move { Session::new(stream?, server).run().await });
    }

    Ok(())
}

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
use holomorph::{ContextualError, WithContext};
use log::{error, info};
use openssl::rsa::{Padding, Rsa};
use runtime::net::TcpListener;
use server::Server;
use session::Session;
use std::sync::Arc;

const RSA_KEY_SIZE: u32 = 256;

fn server(config_path: &str) -> Result<Server, ContextualError> {
    let config: Config = toml::from_str(
        &std::fs::read_to_string(config_path).context("could not read config file")?,
    )
    .context("could not parse config file as TOML")?;

    let signature_key = Rsa::private_key_from_pem(
        &std::fs::read(&config.server.private_key.path)
            .context("could not read private key file")?,
    )
    .context("could not parse RSA private key as PEM")?;

    let keypair = Rsa::generate(RSA_KEY_SIZE * 8).context("could not generate RSA key pair")?;

    let mut public_key = vec![0; signature_key.size() as _];
    let n = signature_key
        .private_encrypt(
            &keypair
                .public_key_to_der()
                .context("could not DER-encode generated RSA public key")?,
            &mut public_key[..],
            Padding::PKCS1,
        )
        .context("could not sign generated RSA public key")?;
    public_key.truncate(n);

    let database_pool = r2d2::Builder::new()
        .max_size(config.database.pool_size)
        .build(r2d2::ConnectionManager::new(&config.database.url))
        .context("could not build database pool")?;

    let mut server = Server {
        config,
        database_pool,
        public_key,
        private_key: keypair,
        game_servers: Default::default(),
    };

    server.load()?;
    Ok(server)
}

#[runtime::main(runtime_tokio::Tokio)]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let config_path = std::env::args().nth(1).unwrap_or("config.toml".to_string());

    let server = match server(&config_path) {
        Ok(server) => server,
        Err(err) => {
            error!("{}", err);
            std::process::exit(1);
        }
    };

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

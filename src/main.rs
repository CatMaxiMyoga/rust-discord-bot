//! Runs the discord bot
#![deny(clippy::panic)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

mod config;
mod shared_types;

pub use crate::config::CONFIG;

mod commands;
mod events;

mod setup_client;
mod setup_framework;

#[tokio::main]
async fn main() {
    let framework = setup_framework::get_framework().await;
    let mut client = setup_client::get_client(framework).await;

    client.start().await.unwrap();
}

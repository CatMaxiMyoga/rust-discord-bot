//! Runs the discord bot
#![deny(clippy::panic)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

mod config;
mod utils;

pub use crate::config::CONFIG;

mod commands;
mod events;

#[tokio::main]
async fn main() {
    let framework = utils::setup_framework::get_framework().await;
    let mut client = utils::setup_client::get_client(framework).await;

    client.start().await.unwrap();
}

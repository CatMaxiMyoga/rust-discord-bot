//! Runs the discord bot
#![deny(clippy::panic)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

mod config;
pub use crate::config::CONFIG;

mod commands;
mod events;

#[tokio::main]
async fn main() {
    let framework = utils::get_framework(
        commands::all(),
        CONFIG.guild_id,
        CONFIG.commands_synced_channel,
    )
    .await;

    let mut client = utils::get_client(
        framework,
        events::EventHandler,
        &CONFIG.token,
        CONFIG.intents,
    )
    .await;

    client.start().await.unwrap();
}

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

    let client = utils::get_client(
        framework,
        events::EventHandler,
        &CONFIG.token,
        CONFIG.intents,
    )
    .await;

    let mut client = match client {
        Err(e) => {
            CONFIG
                .logger
                .error(&format!("Error creating client: {}", e));
            return;
        }
        Ok(c) => c,
    };

    client.start().await.unwrap();
}

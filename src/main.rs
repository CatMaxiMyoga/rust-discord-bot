//! Runs the discord bot
#![deny(clippy::panic)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

mod config;
use config::CONFIG as Config;

mod shared_types;
use shared_types::{Data, Error};

mod commands;
mod events;

use poise::serenity_prelude as serenity;

#[tokio::main]
async fn main() {
    let framework = poise::Framework::<Data, Error>::builder()
        .options(poise::FrameworkOptions {
            commands: commands::all(),
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_in_guild(
                    ctx,
                    &framework.options().commands,
                    Config.guild_id
                ).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(&Config.token, Config.intents)
        .framework(framework)
        .event_handler(events::EventHandler)
        .await;

    client.unwrap().start().await.unwrap();
}

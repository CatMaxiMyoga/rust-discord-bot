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
                let old_guild_commands = Config
                    .guild_id
                    .get_commands_with_localizations(&ctx.http)
                    .await?;

                for command in old_guild_commands {
                    println!(
                        "Deleting old guild command: {} ({})",
                        command.name, command.id
                    );
                    Config
                        .guild_id
                        .delete_command(&ctx.http, command.id)
                        .await?;
                }

                let old_global_commands = serenity::Command::get_global_commands(&ctx.http).await?;

                for command in old_global_commands {
                    println!(
                        "Deleting old global command: {} ({})",
                        command.name, command.id
                    );
                    serenity::Command::delete_global_command(&ctx.http, command.id).await?;
                }

                println!("Registering commands...");
                poise::builtins::register_in_guild(
                    ctx,
                    &framework.options().commands,
                    Config.guild_id,
                )
                .await?;

                for command in &framework.options().commands {
                    println!(
                        "/{}\t  {}",
                        command.name,
                        command.description.as_deref().unwrap_or("".into())
                    );
                }

                println!("\nCommands registered.");

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

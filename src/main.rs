//! Runs the discord bot
#![deny(clippy::panic)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

mod config;
use config::CONFIG;

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
                let sync_start = std::time::Instant::now();

                let old_guild_commands = CONFIG
                    .guild_id
                    .get_commands_with_localizations(&ctx.http)
                    .await?;

                for command in old_guild_commands {
                    println!(
                        "Deleting old guild command: {} ({})",
                        command.name, command.id
                    );
                    CONFIG
                        .guild_id
                        .delete_command(&ctx.http, command.id)
                        .await?;
                }

                let old_global_commands = serenity::Command::get_global_commands(&ctx.http)
                    .await?;

                for command in old_global_commands {
                    println!(
                        "Deleting old global command: {} ({})",
                        command.name, command.id
                    );
                    serenity::Command::delete_global_command(&ctx.http, command.id).await?;
                }

                println!("Registering commands...");

                for command in &framework.options().commands {
                    println!(
                        "/{}\t  {}",
                        command.name,
                        command.description.as_deref().unwrap_or("".into())
                    );
                }

                poise::builtins::register_in_guild(
                    ctx,
                    &framework.options().commands,
                    CONFIG.guild_id,
                )
                .await?;

                let sync_duration = sync_start.elapsed();

                println!("\nCommands registered.");
                println!("Commands Synced in {:.2?}s!", sync_duration.as_secs_f64());

                let commands_synced_embed = serenity::builder::CreateEmbed::new()
                    .title("Commands Synced!")
                    .description(format!(
                        " Took {:.2?}s to sync commands.",
                        sync_duration.as_secs_f64()
                    ))
                    .color(0x00FF88);
                let commands_synced_message =
                    serenity::builder::CreateMessage::new().embed(commands_synced_embed);

                CONFIG
                    .commands_synced_channel
                    .send_message(&ctx.http, commands_synced_message)
                    .await
                    .unwrap();

                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(&CONFIG.token, CONFIG.intents)
        .framework(framework)
        .event_handler(events::EventHandler)
        .await;

    client.unwrap().start().await.unwrap();
}

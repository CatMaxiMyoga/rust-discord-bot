//! Contains helper functions and utilities.
#![deny(missing_docs)]

pub mod logging;
pub mod shared_types;

mod role_check;
pub use role_check::check_role;

use crate::shared_types::{Data, Error};
use poise::Framework;
use poise::serenity_prelude::{
    ActivityData, ChannelId, Client, ClientBuilder, Command, Context, CreateEmbed, CreateMessage,
    Error as SerenityError, EventHandler, GatewayIntents, GuildId, OnlineStatus, Ready,
};

/// Creates and returns a Serenity Client with the given framework, event handler, token, and
/// intents.
pub async fn get_client(
    framework: poise::Framework<Data, Error>,
    event_handler: impl EventHandler + 'static,
    token: impl AsRef<str>,
    intents: GatewayIntents,
) -> Result<Client, SerenityError> {
    ClientBuilder::new(&token, intents)
        .framework(framework)
        .event_handler(event_handler)
        .await
}

fn setup<'a>(
    ctx: &'a Context,
    framework: &'a Framework<Data, Error>,
    guild_id: GuildId,
    commands_synced_channel: ChannelId,
) -> poise::BoxFuture<'a, Result<Data, Error>> {
    Box::pin(async move {
        ctx.set_presence(
            Some(ActivityData::playing("playing with yarn balls")),
            OnlineStatus::Idle,
        );

        let sync_start = std::time::Instant::now();

        let old_guild_commands = guild_id.get_commands_with_localizations(&ctx.http).await?;

        for command in old_guild_commands {
            guild_id.delete_command(&ctx.http, command.id).await?;
        }

        let old_global_commands = Command::get_global_commands(&ctx.http).await?;

        for command in old_global_commands {
            Command::delete_global_command(&ctx.http, command.id).await?;
        }

        poise::builtins::register_in_guild(&ctx.http, &framework.options().commands, guild_id)
            .await?;

        let sync_duration = sync_start.elapsed();

        let commands_synced_embed = CreateEmbed::new()
            .title("Commands Synced!")
            .description(format!(
                " Took {:.2?}s to sync commands.",
                sync_duration.as_secs_f64()
            ))
            .color(0x00FF88);
        let commands_synced_message = CreateMessage::new().embed(commands_synced_embed);

        commands_synced_channel
            .send_message(&ctx.http, commands_synced_message)
            .await
            .ok();

        Ok(Data {})
    })
}

fn setup_wrapper(
    guild_id: GuildId,
    commands_synced_channel: ChannelId,
) -> impl for<'a> Fn(
    &'a Context,
    &'a Ready,
    &'a Framework<Data, Error>,
) -> poise::BoxFuture<'a, Result<Data, Error>> {
    move |ctx: &Context, _ready: &Ready, framework: &Framework<Data, Error>| {
        setup(ctx, framework, guild_id, commands_synced_channel)
    }
}

/// Creates and returns a Poise Framework with the given commands, guild ID, and commands synced
pub async fn get_framework(
    commands: Vec<poise::Command<Data, Error>>,
    guild_id: GuildId,
    commands_synced_channel: ChannelId,
) -> poise::Framework<Data, Error> {
    poise::Framework::<Data, Error>::builder()
        .options(poise::FrameworkOptions {
            commands,
            ..Default::default()
        })
        .setup(setup_wrapper(guild_id, commands_synced_channel))
        .build()
}

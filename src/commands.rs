//! Contains all the bot commands.

use crate::shared_types::{Context, Data, Error};

/// Sends back Pong! 🏓
///
/// This command has no real use, all it does is send back Pong! 🏓 when using it.
#[poise::command(slash_command, guild_only)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Pong! 🏓").await?;
    Ok(())
}

pub fn all() -> Vec<poise::Command<Data, Error>> {
    vec![ping()]
}

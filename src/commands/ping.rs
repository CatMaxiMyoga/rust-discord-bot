use crate::shared_types::{Context, Error};

/// Sends back "Pong! 🏓"
///
/// Sends back "Pong! 🏓"
#[poise::command(slash_command, guild_only)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Pong! 🏓").await?;
    Ok(())
}

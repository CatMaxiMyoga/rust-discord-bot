use utils::shared_types::{CommandsExport, Context, Error};

async fn check(ctx: Context<'_>) -> Result<bool, Error> {
    Ok(utils::check(CONFIG.commands.avatar, &ctx, &CONFIG.logger).await)
}

/// Sends back "Pong! 🏓"
///
/// Sends back "Pong! 🏓"
#[poise::command(slash_command, guild_only, check = check)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Pong! 🏓").await?;
    Ok(())
}

pub static EXPORT: CommandsExport = &[ping];

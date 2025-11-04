// :%s/ß/{name}
use crate::CONFIG;
use utils::shared_types::{Context, Error, CommandsExport};

async fn check(ctx: Context<'_>) -> Result<bool, Error> {
    Ok(utils::check_role(CONFIG.ß_roles.clone(), &ctx, &CONFIG.logger).await)
}

#[poise::command(slash_command, guild_only, check = check)]
pub async fn ß(
    ctx: Context<'_>,
) -> Result<(), Error> {
    Ok(())
}

pub static EXPORT: CommandsExport = &[ß];

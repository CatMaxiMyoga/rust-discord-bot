use crate::shared_types::Context;
use poise::{CreateReply, serenity_prelude::RoleId};

/// Checks if the command invoker has at least one of the required roles.
pub async fn check_role(
    required_roles: Vec<RoleId>,
    ctx: &Context<'_>,
    logger: &crate::logging::Logger,
) -> bool {
    let http = &ctx.http();
    let guild_id = ctx.guild_id();

    if guild_id.is_none() {
        logger.error("check_role called with non-guild context");
        return false;
    }

    // Safe to unwrap since we checked for None above
    let guild_id = guild_id.unwrap();

    let mut has_role = false;
    for role in &required_roles {
        let result = ctx
            .author()
            .has_role(http, guild_id, role)
            .await
            .unwrap_or(false);

        if result {
            has_role = true;
            break;
        }
    }

    if !has_role {
        let reply = CreateReply::default()
            .content(":x: You do not have permission to use this command!")
            .ephemeral(true);

        ctx.send(reply).await.ok();
    }

    has_role
}

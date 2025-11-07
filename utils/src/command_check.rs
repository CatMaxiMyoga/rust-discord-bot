use crate::shared_types::Context;
use poise::{
    CreateReply,
    serenity_prelude::{CacheHttp, GuildId, RoleId},
};

/// Checks if the user is allowed to invoke the command based on the given rules and context.
pub async fn check(
    command_rules: crate::config::CommandRules,
    ctx: &Context<'_>,
    logger: &crate::logging::Logger,
) -> bool {
    let Some(guild_id) = ctx.guild_id() else {
        logger.error("command check called with non-guild context");
        return false;
    };

    if let Some(roles) = command_rules.roles {
        if !check_roles(roles, &ctx, guild_id).await {
            return false;
        }
    }

    if let Some(channels) = command_rules.channels {
        if !check_channels(channels, command_rules.channel_whitelist, &ctx).await {
            return false;
        }
    }

    true
}

async fn check_roles(roles: Vec<RoleId>, ctx: &Context<'_>, guild_id: GuildId) -> bool {
    let mut has_role = false;
    for role in &roles {
        let result = ctx
            .author()
            .has_role(ctx.http(), guild_id, role)
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

async fn check_channels(
    channels: Vec<poise::serenity_prelude::ChannelId>,
    channel_whitelist: bool,
    ctx: &Context<'_>,
) -> bool {
    let channel_id = ctx.channel_id();
    let in_list = channels.contains(&channel_id);
    let allowed = if channel_whitelist { in_list } else { !in_list };
    if !allowed {
        let reply = CreateReply::default()
            .content(":x: You cannot use this command in this channel!")
            .ephemeral(true);
        ctx.send(reply).await.ok();
    }
    allowed
}

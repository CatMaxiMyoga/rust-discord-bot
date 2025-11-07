use crate::CONFIG;
use poise::serenity_prelude::{self as serenity, CreateEmbed};
use utils::shared_types::{CommandsExport, Context, Error};

async fn check(ctx: Context<'_>) -> Result<bool, Error> {
    Ok(utils::check(CONFIG.commands.avatar, &ctx, &CONFIG.logger).await)
}

/// Get the avatar of a user or yourself.
///
/// Returns the avatar of the specified user or yourself.
///
/// If no user is specified, your own avatar will be used. The avatar will be displayed directly \
/// and a direct link to the avatar will be given.
#[poise::command(slash_command, guild_only, check = check)]
pub async fn avatar(
    ctx: Context<'_>,
    #[description = "The user to get the avatar of"] user: Option<serenity::Member>,
) -> Result<(), Error> {
    let user = match user {
        Some(user) => user.user,
        None => ctx.author().clone(),
    };

    let avatar_url = user.avatar_url();
    if avatar_url.is_none() {
        let reply = poise::CreateReply::default()
            .content("The specified user does not seem to have an avatar.")
            .ephemeral(true);

        ctx.send(reply).await?;
        return Ok(());
    }
    // Safe to unwrap since we checked for None above
    let avatar_url = avatar_url.unwrap();

    let embed = CreateEmbed::default()
        .title(format!("{}'s Avatar", user.name))
        .description(format!("[Direct Link](<{}>)", avatar_url))
        .image(avatar_url)
        .color(0x00FF00)
        .timestamp(serenity::Timestamp::now());

    let reply = poise::CreateReply::default().embed(embed);
    ctx.send(reply).await?;

    Ok(())
}

pub static EXPORT: CommandsExport = &[avatar];

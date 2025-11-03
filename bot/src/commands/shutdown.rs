use crate::CONFIG;
use poise::{
    CreateReply,
    serenity_prelude::{CreateMessage, OnlineStatus, builder::CreateEmbed},
};
use utils::shared_types::{CommandsExport, Context, Error};

#[poise::command(slash_command, guild_only, owners_only)]
pub async fn shutdown(ctx: Context<'_>) -> Result<(), Error> {
    let reply = CreateReply::default()
        .content("Shutting down...")
        .ephemeral(true);
    ctx.send(reply.clone()).await?;

    let author: String = ctx.author().name.to_owned();
    let embed = CreateEmbed::new()
        .title("Bot offline!")
        .description(format!("`{}` shut down the bot", author))
        .color(0xFF0000);
    let message = CreateMessage::default().embed(embed);
    CONFIG
        .shutdown_channel
        .send_message(&ctx.http(), message)
        .await?;

    ctx.serenity_context()
        .set_presence(None, OnlineStatus::Offline);
    ctx.framework().shard_manager.shutdown_all().await;

    Ok(())
}

pub static EXPORT: CommandsExport = &[shutdown];

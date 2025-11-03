use chrono_tz::Europe::Berlin;
use poise::serenity_prelude::{CreateAttachment, GetMessages, builder::CreateMessage};

use crate::CONFIG;
use utils::shared_types::{CommandsExport, Context, Error};

/// Deletes the specified amount of messages in the current channel.
///
/// Bulk Deletes up to 100 messages in a channel.
///
/// To delete more than 100 messages, run the command multiple times. If the specified amount of \
/// messages is more than the amount of messages in the current channel, it will delete all \
/// messages in the channel and use that amount for the reply and log instead of the specified \
/// amount.
/// Note: Messages older than 14 days cannot be bulk deleted due to Discord limitations.
#[poise::command(slash_command, guild_only)]
pub async fn purge(
    ctx: Context<'_>,
    #[description = "The amount of messages to delete"]
    #[min = 1]
    #[max = 100]
    amount: u8,
) -> Result<(), Error> {
    let channel_id = ctx.channel_id();
    let get_messages = GetMessages::new().limit(amount);
    let messages = channel_id
        .messages(&ctx.serenity_context().http, get_messages)
        .await?;

    channel_id
        .delete_messages(&ctx.serenity_context().http, &messages)
        .await?;

    let message_count = messages.len();

    let reply = poise::CreateReply::default()
        .content(format!("Deleted {} messages.", message_count))
        .ephemeral(true);
    ctx.send(reply).await?;

    let mut log_content = String::new();

    for message in messages.iter().rev() {
        let timestamp = message.timestamp.to_utc().with_timezone(&Berlin);
        let timestamp: String = timestamp.format("%d/%m/%Y %I:%M:%S %p %Z").to_string();
        let author = &message.author.name;
        let content = &message.content;
        let attachment_count = message.attachments.len();
        let embed_count = message.embeds.len();

        log_content.push_str(
            format!(
                "{}\n@{}{}{}{}\n\n\n",
                timestamp,
                author,
                if content.is_empty() {
                    String::new()
                } else {
                    format!("\n{}", content)
                },
                if attachment_count > 0 {
                    format!("\n-[{} attachment(s)]", attachment_count)
                } else {
                    String::new()
                },
                if embed_count > 0 {
                    format!("\n-[{} embed(s)]", embed_count)
                } else {
                    String::new()
                }
            )
            .as_str(),
        );
    }

    let attachment = CreateAttachment::bytes(
        std::borrow::Cow::Owned(log_content.as_bytes().to_vec()),
        format!(
            "purge_log_{}.txt",
            chrono::Utc::now().format("%Y%m%d%H%M%S")
        ),
    );

    let channel_mention = format!("<#{}>", channel_id);
    let log_message = format!(
        "Purged {} messages in channel {}",
        message_count, channel_mention
    );

    let log = CreateMessage::default()
        .content(log_message)
        .add_file(attachment);

    CONFIG
        .purge_command_channel
        .send_message(&ctx.serenity_context().http, log)
        .await?;

    Ok(())
}

pub static EXPORT: CommandsExport = &[purge];

use crate::shared_types::{Context, Error};

/// Make the bot send a message.
///
/// Sends the given message.
///
/// The message is sent as the reply to the command in the channel the command was used in. To \
/// send an embed use the `embed` command instead.
/// Makes sure to handle escaped characters like `\n` properly. To include a literal backslash \
/// character, use `\\`
#[poise::command(slash_command, guild_only)]
pub async fn say(
    ctx: Context<'_>,
    #[description = "The message to send. Supports escaped characters like \\n."]
    #[min_length = 2]
    message: String,
) -> Result<(), Error> {
    println!("Raw message: {}", message);
    let unescaped_message = unescape::unescape(&message).unwrap_or(message);
    println!("Unescaped message: {}", unescaped_message);
    ctx.say(unescaped_message).await?;
    Ok(())
}

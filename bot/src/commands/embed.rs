use crate::CONFIG;
use poise::{
    CreateReply,
    serenity_prelude::{self as serenity, CreateEmbedAuthor},
};
use regex::Regex;
use serenity::builder::CreateEmbed;
use utils::shared_types::{CommandsExport, Context, Error};

type EmbedField = (String, String, bool);

async fn check(ctx: Context<'_>) -> Result<bool, Error> {
    Ok(utils::check(CONFIG.commands.embed, &ctx, &CONFIG.logger).await)
}

/// Send a message with an embed.
///
/// Sends a configurable embed message.
///
/// The message containing the embed is sent as the reply to the command in the channel the \
/// command was used in. Also lets you optionally add a normal message along with the embed. To \
/// send a normal message without an embed use the `say` command instead.
/// Some parameters make sure to handle escaped characters like `\n` properly. These are:
/// `description`, the values for `fields` and the `message`.
/// To include a literal backslash character, use `\\`
#[poise::command(slash_command, guild_only, check = check)]
#[allow(clippy::too_many_arguments)]
pub async fn embed(
    ctx: Context<'_>,

    #[description = "The author of the embed."]
    #[min_length = 1]
    #[max_length = 256]
    author: Option<String>,

    #[description = "The author URL of the embed. Requires `author` to be set."]
    #[min_length = 1]
    author_url: Option<String>,

    #[description = "The author icon URL of the embed. Requires `author` to be set. \
        Set to 'avatar' for your avatar."]
    #[min_length = 1]
    author_icon_url: Option<String>,

    #[description = "The title of the embed."]
    #[min_length = 1]
    #[max_length = 256]
    title: Option<String>,

    #[description = "The description of the embed."]
    #[min_length = 1]
    #[max_length = 4096]
    description: String,

    #[description = "The color of the embed in hexadecimal #RRGGBB format."]
    #[min_length = 7]
    #[max_length = 7]
    color: Option<String>,

    #[description = "'Name|Value|Inline(true/false);...'. Use \\| and \\; to use literals. \
        Invalid fields are ignored."]
    #[min_length = 8]
    fields: Option<String>,

    #[description = "The footer text of the embed."]
    #[min_length = 1]
    #[max_length = 2048]
    footer: Option<String>,

    #[description = "The footer icon URL of the embed. Requires `footer` to be set."]
    #[min_length = 1]
    footer_icon_url: Option<String>,

    #[description = "Whether to add the timestamp to the embed (time of sending)."]
    timestamp: Option<bool>,

    #[description = "The image URL of the embed."]
    #[min_length = 1]
    image_url: Option<String>,

    #[description = "The thumbnail URL of the embed."]
    #[min_length = 1]
    thumbnail_url: Option<String>,

    #[description = "A normal message to send along with the embed."]
    #[min_length = 1]
    message: Option<String>,
) -> Result<(), Error> {
    let mut embed = CreateEmbed::default();
    let mut error = String::new();

    if let Some(author) = author {
        let mut embed_author = CreateEmbedAuthor::new(author);
        if let Some(author_url) = author_url {
            if !check_url(&author_url) {
                error.push_str(
                    format!(
                        "Invalid author URL format '{}'. Must be a valid URL.\n",
                        author_url
                    )
                    .as_str(),
                );
            }
            embed_author = embed_author.url(author_url);
        }
        if let Some(author_icon_url) = author_icon_url {
            if !check_image_url(&author_icon_url) {
                error.push_str(
                    format!(
                        "Invalid author icon URL format '{}'. {}",
                        author_icon_url, "Must be a valid image URL or 'avatar'.\n",
                    )
                    .as_str(),
                );
            } else if author_icon_url.eq_ignore_ascii_case("avatar") {
                if let Some(avatar) = ctx.author().avatar_url() {
                    embed_author = embed_author.icon_url(avatar);
                } else {
                    error.push_str("You do not have an avatar to use as author icon.\n");
                }
            } else {
                embed_author = embed_author.icon_url(author_icon_url);
            }
        }
        embed = embed.author(embed_author);
    }

    if let Some(title) = title {
        embed = embed.title(title);
    }

    embed = embed.description(unescape::unescape(description.as_str()).expect("HEHE"));

    if let Some(color) = color {
        if let Ok(color_value) = u32::from_str_radix(color.trim_start_matches('#'), 16) {
            embed = embed.color(color_value);
        } else {
            error.push_str(
                format!(
                    "Invalid color format '{}'. Use hexadecimal #RRGGBB format.\n",
                    color
                )
                .as_str(),
            );
        }
    }

    if let Some(fields) = fields {
        let parsed_fields = parse_fields(fields);
        for (i, (name, value, inline)) in parsed_fields.iter().enumerate() {
            if name.is_empty() || value.is_empty() {
                error.push_str(
                    format!(
                        "Field {} is invalid. Name and Value cannot be empty.\n",
                        i + 1
                    )
                    .as_str(),
                );
                continue;
            }

            if name.len() > 256 {
                error.push_str(
                    format!(
                        "Field {} name is too long ({} characters). Maximum is 256.\n",
                        i + 1,
                        name.len()
                    )
                    .as_str(),
                );
                continue;
            }

            if value.len() > 1024 {
                error.push_str(
                    format!(
                        "Field {} value is too long ({} characters). Maximum is 1024.\n",
                        i + 1,
                        value.len()
                    )
                    .as_str(),
                );
                continue;
            }

            embed = embed.field(
                name,
                unescape::unescape(value.as_str()).unwrap_or(value.clone()),
                *inline,
            );
        }
    }

    if let Some(footer) = footer {
        let mut embed_footer = serenity::CreateEmbedFooter::new(footer);
        if let Some(footer_icon_url) = footer_icon_url {
            if !check_image_url(&footer_icon_url) {
                error.push_str(
                    format!(
                        "Invalid footer icon URL format '{}'. Must be a valid image URL.\n",
                        footer_icon_url
                    )
                    .as_str(),
                );
            } else if footer_icon_url.eq_ignore_ascii_case("avatar") {
                if let Some(avatar) = ctx.author().avatar_url() {
                    embed_footer = embed_footer.icon_url(avatar);
                } else {
                    error.push_str("You do not have an avatar to use as footer icon.\n");
                }
            } else {
                embed_footer = embed_footer.icon_url(footer_icon_url);
            }
        }
        embed = embed.footer(embed_footer);
    }

    if timestamp.unwrap_or(false) {
        embed = embed.timestamp(serenity::Timestamp::now());
    }

    if let Some(image_url) = image_url {
        if !check_image_url(&image_url) {
            error.push_str(
                format!(
                    "Invalid image URL format '{}'. Must be a valid image URL.\n",
                    image_url
                )
                .as_str(),
            );
        } else if image_url.eq_ignore_ascii_case("avatar") {
            if let Some(avatar) = ctx.author().avatar_url() {
                embed = embed.image(avatar);
            } else {
                error.push_str("You do not have an avatar to use as image.\n");
            }
        } else {
            embed = embed.image(image_url);
        }
    }

    if let Some(thumbnail_url) = thumbnail_url {
        if !check_image_url(&thumbnail_url) {
            error.push_str(
                format!(
                    "Invalid thumbnail URL format '{}'. Must be a valid image URL.\n",
                    thumbnail_url
                )
                .as_str(),
            );
        } else if thumbnail_url.eq_ignore_ascii_case("avatar") {
            if let Some(avatar) = ctx.author().avatar_url() {
                embed = embed.thumbnail(avatar);
            } else {
                error.push_str("You do not have an avatar to use as thumbnail.\n");
            }
        } else {
            embed = embed.thumbnail(thumbnail_url);
        }
    }

    let mut reply = CreateReply::default().embed(embed);

    if let Some(message) = message {
        let unescaped_message = unescape::unescape(&message).unwrap_or(message);
        reply = reply.content(unescaped_message);
    }

    if !error.is_empty() {
        embed = CreateEmbed::default()
            .title("Errors in embed command")
            .description(error)
            .color(0xFF0000);

        reply = CreateReply::default().embed(embed).ephemeral(true);
    }

    ctx.send(reply).await?;
    Ok(())
}

fn check_url(input: &str) -> bool {
    // Safe to unwrap since the regex is valid
    let url_regex = Regex::new(r"^(https?://[^\s]+)$").unwrap();
    url_regex.is_match(input)
}

fn check_image_url(input: &str) -> bool {
    // Safe to unwrap since the regex is valid
    let image_url_regex =
        Regex::new(r"^(https?://[^\s]+\.(png|jpg|jpeg|gif|webp|bmp))$|^avatar$").unwrap();
    image_url_regex.is_match(input)
}

fn parse_fields(input: String) -> Vec<EmbedField> {
    let semicolon_placeholder = "\u{F000}SEMICOLON528332851\u{F000}";
    let pipe_placeholder = "\u{F000}PIPE3203\u{F000}";

    input
        .replace(r"\;", semicolon_placeholder)
        .replace(r"\|", pipe_placeholder)
        .split(';')
        .filter_map(|field| {
            let parts: Vec<&str> = field.split('|').collect();

            if parts.len() == 3 {
                Some((
                    parts[0]
                        .replace(semicolon_placeholder, ";")
                        .replace(pipe_placeholder, "|")
                        .trim()
                        .to_string(),
                    parts[1]
                        .replace(semicolon_placeholder, ";")
                        .replace(pipe_placeholder, "|")
                        .trim()
                        .to_string(),
                    parts[2].trim().eq_ignore_ascii_case("true"),
                ))
            } else {
                None
            }
        })
        .collect()
}

pub static EXPORT: CommandsExport = &[embed];

//! Contains the event handler for the bot.

use crate::CONFIG;
use poise::serenity_prelude as serenity;
use serenity::Context;
use serenity::builder::{CreateEmbed, CreateMessage};

pub struct EventHandler;

#[serenity::async_trait]
impl serenity::EventHandler for EventHandler {
    async fn ready(&self, ctx: Context, ready: serenity::Ready) {
        println!("{} is connected!", ready.user.display_name());

        let embed = CreateEmbed::new()
            .title("Bot Online!")
            .description("Syncing application commands...")
            .color(0x00FF00);

        CONFIG
            .ready_event_channel
            .send_message(&ctx.http, CreateMessage::new().embed(embed))
            .await
            .unwrap();
    }
}

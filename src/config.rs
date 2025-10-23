//! Contains the Settings

use once_cell::sync::Lazy;
use poise::serenity_prelude::{ChannelId, GatewayIntents, GuildId};
use dotenv::dotenv;

#[derive(Debug)]
pub struct Config {
    pub token: String,
    pub guild_id: GuildId,
    pub intents: GatewayIntents,

    pub ready_channel: ChannelId
}

impl Config {
    fn new() -> Self {
        dotenv().ok();

        let token = std::env::var("DISCORD_TOKEN").expect("Missing token in .env file");
        let guild_id = GuildId::new(1018921751691923536);
        let intents = GatewayIntents::all();

        let ready_channel = ChannelId::new(1239935861370650634);

        Self {
            token,
            guild_id,
            intents,
            ready_channel
        }
    }
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| Config::new());

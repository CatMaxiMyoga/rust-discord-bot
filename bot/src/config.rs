//! Contains the Settings

use dotenv::dotenv;
use once_cell::sync::Lazy;
use poise::serenity_prelude::{ChannelId, GatewayIntents, GuildId, RoleId};
use utils::logging::Logger;

#[derive(Debug)]
pub struct Config {
    pub log_dir: String,
    pub logger: Logger,
    pub token: String,
    pub guild_id: GuildId,
    pub intents: GatewayIntents,

    // Event Log Channels
    pub ready_event_channel: ChannelId,

    // Command Log Channels
    pub purge_command_channel: ChannelId,

    // Misc Log Channels
    pub commands_synced_channel: ChannelId,
    pub shutdown_channel: ChannelId,

    // Command roles
    pub avatar_roles: Vec<RoleId>,
    pub embed_roles: Vec<RoleId>,
    pub purge_roles: Vec<RoleId>,
    pub say_roles: Vec<RoleId>,
}

impl Config {
    fn new() -> Self {
        dotenv().ok();

        let bot_status_channel = ChannelId::new(1239935861370650634);

        let log_dir = String::from("logs");

        let now = chrono::Utc::now().with_timezone(&chrono_tz::Europe::Berlin);
        let datetime = now.format("%Y-%m-%d_%H-%M-%S_%Z").to_string();
        let logger = Logger::builder()
            .output_file(format!("{}/{}.log", log_dir, datetime))
            .build();

        Self {
            log_dir,
            logger,
            token: std::env::var("DISCORD_TOKEN").expect("Missing token in .env file"),
            guild_id: GuildId::new(1018921751691923536),
            intents: GatewayIntents::all(),

            // Event Log Channels
            ready_event_channel: bot_status_channel,

            // Command Log Channels
            purge_command_channel: ChannelId::new(1239387297003077682),

            // Misc Log Channels
            commands_synced_channel: bot_status_channel,
            shutdown_channel: bot_status_channel,

            // Command roles
            avatar_roles: vec![RoleId::new(1233889604436754525)],
            embed_roles: vec![RoleId::new(1237741325462405223)],
            purge_roles: vec![RoleId::new(1234229041343762513)],
            say_roles: vec![RoleId::new(1053019464075063327)],
        }
    }
}

/// Global static configuration instance
pub static CONFIG: Lazy<Config> = Lazy::new(Config::new);

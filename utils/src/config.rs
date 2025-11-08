//! Contains the Settings

use dotenv::dotenv;
use once_cell::sync::Lazy;
use poise::serenity_prelude::{ChannelId, GatewayIntents, GuildId, RoleId};
use crate::logging::Logger;

/// Rules for a specific command
#[derive(Debug)]
pub struct CommandRules {
    /// Roles needed to use the command. If None, no role restriction.
    pub roles: Option<Vec<RoleId>>,
    /// Channels where the command can/cannot be used. If None, no channel restriction.
    pub channels: Option<Vec<ChannelId>>,
    /// Whether the `channels` list is a whitelist (true) or blacklist (false)
    pub channel_whitelist: bool,
}

/// Configuration for all commands
#[derive(Debug)]
pub struct CommandsConfig {
    /// Configuration for the avatar command
    pub avatar: CommandRules,
    /// Configuration for the embed command
    pub embed: CommandRules,
    /// Configuration for the purge command
    pub purge: CommandRules,
    /// Configuration for the say command
    pub say: CommandRules,
}

impl Default for CommandsConfig {
    fn default() -> Self {
        Self {
            avatar: CommandRules {
                roles: Some(vec![RoleId::new(1233889604436754525)]),
                channels: None,
                channel_whitelist: false,
            },
            embed: CommandRules {
                roles: Some(vec![RoleId::new(1237741325462405223)]),
                channels: None,
                channel_whitelist: false,
            },
            purge: CommandRules {
                roles: Some(vec![RoleId::new(1234229041343762513)]),
                channels: None,
                channel_whitelist: false,
            },
            say: CommandRules {
                roles: Some(vec![RoleId::new(1053019464075063327)]),
                channels: None,
                channel_whitelist: false,
            },
        }
    }
}

/// The main configuration struct
#[derive(Debug)]
pub struct Config {
    // ┌───────────────────────┐
    // │ General Configuration │
    // └───────────────────────┘
    /// The directory where log files are stored
    pub log_dir: String,
    /// The logger instance
    pub logger: Logger,
    /// The Discord bot token
    pub token: String,
    /// The guild ID the bot operates in
    pub guild_id: GuildId,
    /// The gateway intents the bot uses
    pub intents: GatewayIntents,
    /// Configuration for commands
    pub commands: CommandsConfig,

    // ┌────────────────────┐
    // │ Event Log Channels │
    // └────────────────────┘
    /// Channel for ready events
    pub ready_event_channel: ChannelId,

    // ┌──────────────────────┐
    // │ Command Log Channels │
    // └──────────────────────┘
    /// Channel for purge command logs
    pub purge_command_channel: ChannelId,

    // ┌───────────────────┐
    // │ Misc Log Channels │
    // └───────────────────┘
    /// Channel for command sync logs
    pub commands_synced_channel: ChannelId,
    /// Channel for shutdown logs
    pub shutdown_channel: ChannelId,
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
            commands: Default::default(),

            // Event Log Channels
            ready_event_channel: bot_status_channel,

            // Command Log Channels
            purge_command_channel: ChannelId::new(1239387297003077682),

            // Misc Log Channels
            commands_synced_channel: bot_status_channel,
            shutdown_channel: bot_status_channel,
        }
    }
}

/// Global static configuration instance
pub static CONFIG: Lazy<Config> = Lazy::new(Config::new);

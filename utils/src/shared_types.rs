//! Contains shared types

/// The shared data for the bot.
pub struct Data {}

/// The shared error type for the bot.
pub type Error = Box<dyn std::error::Error + Send + Sync>;

/// The shared context type for the bot.
pub type Context<'a> = poise::Context<'a, Data, Error>;

/// The type for exporting commands from their respective rust modules.
pub type CommandsExport = &'static [fn() -> poise::Command<Data, Error>];

//! Contains all the bot commands.

use crate::shared_types::{Data, Error};

mod embed;
mod help;
mod ping;
mod say;

/// Returns a vector of all commands in the bot.
pub fn all() -> Vec<poise::Command<Data, Error>> {
    vec![embed::embed(), ping::ping(), say::say(), help::help()]
}

//! Contains all the bot commands.

use crate::shared_types::{Data, Error};

mod avatar;
mod embed;
mod help;
mod ping;
mod purge;
mod say;

/// Returns a vector of all commands in the bot.
pub fn all() -> Vec<poise::Command<Data, Error>> {
    vec![
        avatar::avatar(),
        embed::embed(),
        help::help(),
        ping::ping(),
        purge::purge(),
        say::say(),
    ]
}

//! Contains all the bot commands.

use utils::shared_types::{Data, Error};

macro_rules! all_commands {
    [$($name:ident),* $(,)?] => {
        $(mod $name;)*

        /// Returns a vector of all commands.
        pub fn all() -> Vec<poise::Command<Data, Error>> {
            let mut v = Vec::new();
            $(
                for f in $name::EXPORT {
                    v.push(f());
                }
            )*
            v
        }
    };
}

all_commands![avatar, embed, help, ping, purge, say, shutdown];

//! Runs the discord bot
#![deny(clippy::panic)]
#![deny(unused_must_use)]

mod config;

pub use crate::config::CONFIG;

mod commands;
mod events;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut i = 1;
    while i < args.len() {
        // Safe to unwrap because of the loop condition
        let arg = args.get(i).unwrap().as_str();
        match arg {
            "--clear-logs" | "-C" => {
                for entry in std::fs::read_dir(&CONFIG.log_dir).unwrap() {
                    let entry = match entry {
                        Ok(e) => e,
                        Err(e) => {
                            CONFIG
                                .logger
                                .error(&format!("Error reading log directory: {}", e));
                            continue;
                        }
                    };
                    let path = entry.path();
                    if path.is_file() {
                        match std::fs::remove_file(path) {
                            Ok(_) => {}
                            Err(_) => {
                                CONFIG.logger.error(&format!(
                                    "Error deleting log file: {}",
                                    entry.path().display()
                                ));
                                std::process::exit(1);
                            }
                        };
                    }
                }
            }
            _ => CONFIG.logger.error(&format!("Unknown argument: '{}'", arg)),
        }
        i += 1;
    }

    let framework = utils::get_framework(
        commands::all(),
        CONFIG.guild_id,
        CONFIG.commands_synced_channel,
    )
    .await;

    let client = utils::get_client(
        framework,
        events::EventHandler,
        &CONFIG.token,
        CONFIG.intents,
    )
    .await;

    let mut client = match client {
        Err(e) => {
            CONFIG
                .logger
                .error(&format!("Error creating client: {}", e));
            return;
        }
        Ok(c) => c,
    };

    client.start().await.unwrap();
}

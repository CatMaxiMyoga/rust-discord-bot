use poise::serenity_prelude as serenity;

use crate::{CONFIG, events};
use crate::utils::shared_types::{Data, Error};

pub async fn get_client(framework: poise::Framework<Data, Error>) -> serenity::Client {
    serenity::ClientBuilder::new(&CONFIG.token, CONFIG.intents)
        .framework(framework)
        .event_handler(events::EventHandler)
        .await
        .unwrap()
}

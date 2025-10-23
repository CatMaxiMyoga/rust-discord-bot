use crate::shared_types::{Context, Data, Error};

#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Pong! :ping_pong:").await?;
    Ok(())
}

pub fn all() -> Vec<poise::Command<Data, Error>> {
    vec![]
}

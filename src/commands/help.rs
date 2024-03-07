use crate::Error;
use poise::serenity_prelude::{self as serenit, ChannelId};
// use serenity::utils::Colour;


/// Displays your or another user's account creation date
#[poise::command(slash_command)]
pub async fn help(ctx: poise::Context<'_, (), Error>) -> Result<(), Error> {
    // ctx.send(|m| {m.}).await?;
    ctx.say("Help message").await?;
    Ok(())
}
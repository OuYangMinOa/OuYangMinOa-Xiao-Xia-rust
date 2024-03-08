use poise::FrameworkBuilder;
use serenity::model::{gateway::GatewayIntents, id::GuildId};
use songbird::SerenityInit;

use crate::commands;
use crate::Error;

pub fn build(token: &str, guild_id: GuildId) -> FrameworkBuilder<(), Error> {
    poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands::get(),
            ..Default::default()
        })
        .intents(
            GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT,
        )
        .token(token)
        .client_settings(|c| c.register_songbird())
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_in_guild(ctx, &framework.options().commands, guild_id)
                    .await?;
                Ok(())
            })
        })
}
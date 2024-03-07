#[macro_use]
extern crate dotenv_codegen;

mod commands;

use poise::serenity_prelude as serenity;


type Error = Box<dyn std::error::Error + Send + Sync>;

// use std::env;

const PRIVATEGUIDID :serenity::GuildId = serenity::GuildId::new(597757976920588288);

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // dotenv!("DISCORD_TOKEN")

    let discord_token = dotenv!("DISCORD_TOKEN");
    println!("[*] Waiting for server ready ... ");
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::help::help(),
            ],..Default::default()
        })
        .setup(|ctx, _ready, framework| {Box::pin(
            async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                poise::builtins::register_in_guild(ctx, &framework.options().commands,PRIVATEGUIDID).await?;
                Ok(())
                })
            })
        .build();

    let mut client = serenity::ClientBuilder::new(discord_token, serenity::GatewayIntents::non_privileged())
        .framework(framework)
        .await
        .expect("Error creating client");
    println!("[*] Server start");
    client.start().await.unwrap();
}

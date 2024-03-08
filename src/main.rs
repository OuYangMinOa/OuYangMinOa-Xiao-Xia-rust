#[macro_use]
extern crate dotenv_codegen;

mod commands;
mod framework;
mod data;

use poise::serenity_prelude as serenity;


type Error = Box<dyn std::error::Error + Send + Sync>;

// use std::env;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // dotenv!("DISCORD_TOKEN")

    let _a = 5;
    println!("[*] Waiting for server ready ... ");
    let private_server:u64 = dotenv!("TEST_GUILD_ID").parse::<u64>().expect("Can't use private server");
    let privateguidid :serenity::GuildId = serenity::GuildId(private_server);

    

    let discord_token = dotenv!("DISCORD_TOKEN");
    

    framework::build( discord_token,privateguidid)
        .run()
        .await
        .expect("[*] Server open fail");
    println!("[*] Server start");
}
#[macro_use]
extern crate dotenv_codegen;

mod data;
mod commands;
mod framework;

use ctrlc;
use glob::glob;
use data::info::MUSICPATH;
use std::{fs, process::exit};
use poise::serenity_prelude as serenity;

type Error = Box<dyn std::error::Error + Send + Sync>;

// remove the song file when the program end
pub fn remove_file() {
    println!("[*] Removing ...");
    for entry in glob(format!("{MUSICPATH}/*.*").as_str()).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => match fs::remove_file(&path) {
                Ok(_) => {}
                Err(_) => {}
            },
            Err(e) => println!("{:?}", e),
        }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // dotenv!("DISCORD_TOKEN")

    println!("[*] Waiting for server ready ... ");
    let private_server: u64 = dotenv!("TEST_GUILD_ID")
        .parse::<u64>()
        .expect("Can't use private server");
    let privateguidid: serenity::GuildId = serenity::GuildId(private_server);

    remove_file();
    ctrlc::set_handler(|| {
        remove_file();
        exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    let discord_token = dotenv!("DISCORD_TOKEN");

    framework::build(discord_token, privateguidid)
        .run()
        .await
        .expect("[*] Server open fail");
    println!("[*] Server start");
}

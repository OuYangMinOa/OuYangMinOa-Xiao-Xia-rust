#[macro_use]
extern crate dotenv_codegen;

mod refer;
mod commands;
mod framework;

use ctrlc;
use glob::glob;
use clap::Parser;
use refer::info::MUSICPATH;
use std::{fs, process::exit};
use poise::serenity_prelude as serenity;


// pub enum MYERR{
//     Error(Box<dyn std::error::Error + Send + Sync>),
//     Empty()
// }


type Error = Box<dyn std::error::Error + Send + Sync>;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// The discord token
    #[arg(short, long)]
    discordtoken: Option<String>,


    /// The guild id for testing
    #[arg(short, long)]
    guildid: Option<u64>,
}

// serenity::event::EventType.

// remove the song file when the program end
pub fn remove_file() {
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
// serenity::ReadyEvent 


#[tokio::main(flavor = "current_thread")]
async fn main() {
    // dotenv!("DISCORD_TOKEN")
    let args = Args::parse();
    let discord_token = args.discordtoken.unwrap_or(dotenv!("DISCORD_TOKEN").to_string());
    let private_server = args.guildid.unwrap_or(dotenv!("TEST_GUILD_ID").parse::<u64>().expect("Can't use private server"));

    println!("[*] Waiting for server ready ... ");

    let privateguidid: serenity::GuildId = serenity::GuildId(private_server);

    remove_file();
    ctrlc::set_handler(|| {
        remove_file();
        exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    

    framework::build(&discord_token, privateguidid)
        .run()
        .await
        .expect("[*] Server open fail");
    println!("[*] Server start");
}

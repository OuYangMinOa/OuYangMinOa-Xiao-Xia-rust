pub mod help;
pub mod music;

use crate::Error;
use poise::Command;
// use crate::framework::Data;

pub fn get() -> Vec<Command<(), Error>>{
    vec![
        help::help(),
        music::play(),
    ]
}
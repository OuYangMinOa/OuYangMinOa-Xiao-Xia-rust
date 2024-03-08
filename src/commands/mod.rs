pub mod help;
pub mod music;
pub mod utils;

use crate::Error;
use poise::Command;
// use crate::framework::Data;

pub fn get() -> Vec<Command<(), Error>>{
    vec![
        help::help(),
        music::play(),
        music::skip(),
        music::pause(),
        music::resume(),
    ]
}
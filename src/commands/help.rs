use crate::Error;
// use serenity::utils::Colour;

use crate::data::lang::{SuppportLanguage, ENLANGMAP, TWLANGMAP};

/// Show help command with specific language
#[poise::command(slash_command)]
pub async fn help(
    ctx: poise::Context<'_, (), Error>,
    #[description = "Select a en or zhtw"] lang: SuppportLanguage,
) -> Result<(), Error> {
    match lang {
        SuppportLanguage::en => {
            ctx.say(ENLANGMAP.help).await?;
        }
        SuppportLanguage::zhtw => {
            ctx.say(TWLANGMAP.help).await?;
        }
    }
    Ok(())
}

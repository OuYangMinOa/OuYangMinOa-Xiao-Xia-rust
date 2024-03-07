
use crate::Error;

/// Play music
#[poise::command(slash_command)]
pub async fn play(ctx: poise::Context<'_, (), Error>) -> Result<(), Error> {
    ctx.say("Playing").await?;

    let guild = ctx.guild().unwrap();
    let userid = ctx.author().id;
    let voice_state = guild.voice_states.get(&userid).expect("Not in a voice channel");
    let channel_id = voice_state.channel_id.unwrap();
    let guild_id = ctx.guild_id().unwrap();


    println!("guild : {}, channel; : {}, User : {}",guild_id,channel_id,userid);
    // ;
    let serenity_context = ctx.serenity_context();
    let manager: Option<std::sync::Arc<songbird::Songbird>> = songbird::get(serenity_context).await;
    match manager {
        Some(manager) => {
            manager.join(guild_id, channel_id).await.1.unwrap();
        },
        None => {
            println!("manager : None");
        }
    }


    // if let Some(call) = manager.get(guild.id) {
    //     let handler = call.lock().await;
    //     let has_current_connection = handler.current_connection().is_some();

    //     if has_current_connection && send_reply {
    //         // bot is in another channel
    //         let bot_channel_id: ChannelId = handler.current_channel().unwrap().0.into();
    //         return Err(ParrotError::AlreadyConnected(bot_channel_id.mention()));
    //     }
    // }


    // let _call: std::sync::Arc<poise::serenity_prelude::prelude::Mutex<Call>> = manager.get(guild_id).unwrap();
    
    
    
    Ok(())
}
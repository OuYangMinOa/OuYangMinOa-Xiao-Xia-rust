
use crate::Error;
use crate::data::info;
use crate::commands::utils;

use url::Url;
use std::sync::Arc;
use async_trait::async_trait;
use songbird::{tracks, Call, Event, EventContext, EventHandler as VoiceEventHandler};
use tokio::sync::Mutex;


pub async fn get_conn(ctx: poise::Context<'_, (), Error>) -> Result<Arc<Mutex<Call>>,Error> {
    let guild = ctx.guild().expect("[*] Can't grab guild");
    let manager = songbird::get(ctx.serenity_context()).await.expect("[*] Can't grab manager");
    let conn = manager.get(guild.id).expect("[*] Can't grab conn");
    Ok(conn)
}

/// Play music with given url.
#[poise::command(slash_command, reuse_response)]
pub async fn play(ctx: poise::Context<'_, (), Error>, #[description= "url"] uri:String) -> Result<(), Error> {
    ctx.defer().await?;
    let guild   = ctx.guild().expect("[*] Grabbing guild error").clone();
    let userid = ctx.author().id;
    let voice_state_option = guild.voice_states.get(&userid);
    let voice_state ;

    // check if user in the voice channel
    match voice_state_option {
        Some(this) => {voice_state = this;}
        None => {
            ctx.send(|m|m.embed(
                     |e| e.title("Error").description("You are not in a voice channel")
                    ).ephemeral(true)
            ).await.unwrap();
            return Ok(());
        }
    }
    // grab chennel_id and guild_id
    let channel_id = voice_state.channel_id.expect("[*] Grabbing channel_id error");
    let guild_id = ctx.guild_id().expect("[*] Grabbing guild_id error");

    println!("guild : {}, channel : {}, User : {}",guild_id,channel_id,userid);
    // ;
    let serenity_context = ctx.serenity_context();
    let manager= songbird::get(serenity_context).await.expect("[*] manager error ");

    // let (conn,result) = manager.join(guild_id, channel_id);

    let (conn,result) = manager.join(guild.id, channel_id).await;
    
    match result{
        Err(err) => { 
            if err.should_leave_server() {println!("[*] Can't connect to server!!")}
        }
        Ok(_)=>{}
    }

    // handling music
    println!("[*] Handling -> {uri}");

    match Url::parse(&uri){
        Ok(_) =>{ // is a url
            if uri.contains("list"){  // is as play list
                println!("[*] is a play list");
                utils::handle_list_url(ctx, &uri, conn).await;
            }
            else {  // single song
                println!("[*] is a single song");
                utils::handle_single_video(ctx, &uri, conn).await;

            }
        }
        Err(_) => { // not a url
            println!("[*] is not a url");
            utils::handle_single_video(ctx, &uri, conn).await;
        }
    }
    Ok(())
}

/// Skip the current music
#[poise::command(slash_command, reuse_response)]
pub async fn skip(ctx: poise::Context<'_, (), Error>) -> Result<(), Error>{
    let conn: Arc<Mutex<Call>> = get_conn(ctx).await.expect("can't grab conn");
    let result = conn.lock().await.queue().skip();
    match result {
        Ok(_)  => {println!("[*] skip !!");ctx.say("skip").await.expect("[*] send message error");}
        Err(_) => {println!("[*] skip error !");ctx.say("skip error").await.expect("[*] send message error");}
    }
    Ok(())
}


/// Pause the music
#[poise::command(slash_command, reuse_response)]
pub async fn pause(ctx: poise::Context<'_, (), Error>) -> Result<(), Error>{
    let conn = get_conn(ctx).await.expect("can't grab conn");
    let result = conn.lock().await.queue().pause();
    match result {
        Ok(_)  => {println!("[*] Pause !!");ctx.say("pause").await.expect("[*] send message error");}
        Err(_) => {println!("[*] Pause error !");ctx.say("pause error").await.expect("[*] send message error");}
    }
    Ok(())
}

/// Resume the paused music
#[poise::command(slash_command, reuse_response)]
pub async fn resume(ctx: poise::Context<'_, (), Error>) -> Result<(), Error>{
    let conn = get_conn(ctx).await.expect("can't grab conn");
    let result = conn.lock().await.queue().resume();
    match result {
        Ok(_)  => {println!("[*] resume !!");ctx.say("resume").await.expect("[*] send message error");}
        Err(_) => {println!("[*] resume error !");ctx.say("resume error").await.expect("[*] send message error");}
    }
    Ok(())
}

/// stop the current music and clear the playlist
#[poise::command(slash_command, reuse_response)]
pub async fn clear(ctx: poise::Context<'_, (), Error>) -> Result<(), Error>{
    let conn: Arc<Mutex<Call>> = get_conn(ctx).await.expect("can't grab conn");
    conn.lock().await.queue().stop();
    println!("[*] stop !!");
    ctx.say("stop").await.expect("[*] send message error");
    Ok(())
}

#[poise::command(slash_command, reuse_response)]
pub async fn list(ctx: poise::Context<'_, (), Error>) -> Result<(), Error>{
    let conn: Arc<Mutex<Call>> = get_conn(ctx).await.expect("can't grab conn");
    let conn_locked = conn.lock().await;
    let (current_channel, queue) =(conn_locked.current_channel().unwrap(),conn_locked.queue().current_queue());
    ctx.send(|r| {
        r.embed(|e| {
            e.title(format!("Queue in <#{current_channel}>"));
            if queue.is_empty() {
                e.description("The queue is empty.");
            } else {
                let body = queue
                    .iter()
                    .map(|track: &tracks::TrackHandle| {
                        let metadata: &songbird::input::Metadata = track.metadata();
                        let binding = "<no_title>".to_string();
                        println!("{:?}",metadata);
                        let title = metadata.title.as_ref().unwrap_or(&binding);
                        if let Some(source_url) = &track.metadata().source_url {
                            format!("• [{title}]({source_url})")
                        } else {
                            format!("• {title}")    
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("\n");

                e.description(body);

                if let Some(thumbnail) = &queue
                    .get(0)
                    .expect("Queue is not empty right after checking")
                    .metadata()
                    .thumbnail
                {
                    e.thumbnail(thumbnail);
                }
            }

            e
        })
    })
    .await?;

    Ok(())
}


struct EndLeaver {
    pub filename: String,
}





#[async_trait]
impl VoiceEventHandler for EndLeaver {
    async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
        unsafe {
            info::FILE_TO_REMOVE.push(self.filename.clone());
        }
        // fs::remove_file(&).expect("[*] file can't delete.");
        None
    }
}
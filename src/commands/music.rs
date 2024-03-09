
use crate::Error;
use humantime::format_duration;
use std::{fs, sync::Arc};
use async_trait::async_trait;
use std::path::Path;
use crate::data::info;

use crate::commands::utils;
use poise::serenity_prelude::GuildId;
use songbird::{
    input::{ffmpeg as sf, Input}, tracks, Call, Event, EventContext, EventHandler as VoiceEventHandler, Songbird, TrackEvent
};

use tokio::sync::Mutex;


pub async fn get_conn(ctx: poise::Context<'_, (), Error>) -> Result<Arc<Mutex<Call>>,Error> {
    let guild = ctx.guild().expect("[*] Can't grab guild");
    let manager = songbird::get(ctx.serenity_context()).await.expect("[*] Can't grab manager");
    let conn = manager.get(guild.id).expect("[*] Can't grab conn");
    Ok(conn)
}

/// Play music with given url.
#[poise::command(slash_command, reuse_response)]
pub async fn play(ctx: poise::Context<'_, (), Error>, #[description= "url"] url:String) -> Result<(), Error> {

    ctx.defer().await?;
    let guild   = ctx.guild().expect("[*] Grabbing guild error").clone();
    let userid = ctx.author().id;
    let voice_state_option = guild.voice_states.get(&userid);
    let voice_state ;

    // check if user in the voice channel
    match voice_state_option {
        Some(this) => {voice_state = this;}
        None => {

            ctx.send(|m|
                m.embed(
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
    println!("[*] Handling -> {url}");

    
    let (source,music_path) = utils::build_songbird_source(url.clone()).await.unwrap();
    let metadata = source.metadata.clone();

    // let source2 = utils::get_youtube_source(url).unwrap();
    let source_result = sf(&music_path).await;

    println!("[*] {music_path} exist : {}",Path::new(&music_path).exists());
    let fsource = match source_result {
        Ok(fsource) => {
            println!("[*] use downloaded source!!");
            fsource
        }
        Err(_) => {
            println!("[*] use yt source!!");
            source
        }
    };

    // let rt = tokio::runtime::Builder::new_current_thread()
    //     .enable_all()
    //     .build()
    //     .unwrap();


    //: tokio::sync::MutexGuard<'_, Call> = conn.lock().await;// .queue().add_source(source, handler);
    
    //.enqueue_source(fsource);
    //.enqueue_source(source);
    


    tokio::spawn(async move{
        let _handle = conn.lock().await.enqueue_source(fsource);
        // self.enqueue(track);
        // handel.play(track);
        unsafe {
            info::FILE_TO_REMOVE.push(music_path);
        }
    });


    
    // let sleep = sleep(TD::from_secs(10));
    // tokio::pin!(sleep);
    // sleep.as_mut().await;


    // let _ = handle.add_event(
    //     Event::Track(TrackEvent::End),
    //     EndLeaver { manager, guild_id },
    // );




	// handle.play_only_source(source);
    // handle.enqueue_source(source);

    // handle.

    ctx.send(|r| {
        r.embed(|e| {
            e.title(format!("Queueing audio in <#{channel_id}>"));

            if let Some(title) = &metadata.title {
                e.field("Title", title, false);
            } else if let Some(track) = &metadata.track {
                e.field("Title", track, false);
            }

            if let Some(duration) = &metadata.duration {
                e.field("Duration", format_duration(*duration), true);
            }

            if let Some(source_url) = &metadata.source_url {
                e.field("Source", format!("[Open original]({source_url})"), true);
            }

            if let Some(thumbnail) = &metadata.thumbnail {
                e.thumbnail(thumbnail);
            }

            e
        })
    })
    .await.unwrap();


    Ok(())
}


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
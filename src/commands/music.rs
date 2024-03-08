use crate::Error;
use humantime::{format_duration, Duration};
use std::sync::Arc;
use async_trait::async_trait;
use std::time::Duration as STDD;
use std::path::Path;

use crate::commands::utils;
use poise::serenity_prelude::GuildId;
use songbird::{
    input::{Input,ffmpeg as sf},
    Event, EventContext, EventHandler as VoiceEventHandler, Songbird, TrackEvent,
};
use tokio::time::{sleep, Duration as TD};


/// Play music with given url.
#[poise::command(slash_command, reuse_response)]
pub async fn play(ctx: poise::Context<'_, (), Error>, #[description= "url"] url:String) -> Result<(), Error> {
    ctx.defer().await?;

    let guild = ctx.guild().unwrap().clone();
    let userid = ctx.author().id;
    let voice_state_option = guild.voice_states.get(&userid);
    let voice_state ;

    // m.embed(|e| e.title("Error").description("You are not in a voice channel"))


    // check if user in the voice channel
    match voice_state_option {
        Some(this) => {voice_state = this;}
        None => {

            ctx.send(|m|
                m.embed(
                    |e| e.title("Error").description("You are not in a voice channel")
                )
            ).await.unwrap();
            return Ok(());
        }
    }


    // grab chennel_id and guild_id
    let channel_id = voice_state.channel_id.unwrap();
    let guild_id = ctx.guild_id().unwrap();


    println!("guild : {}, channel : {}, User : {}",guild_id,channel_id,userid);
    // ;
    let serenity_context = ctx.serenity_context();
    let manager= songbird::get(serenity_context).await.unwrap();
    
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
    let mut handle = conn.lock().await.enqueue_source(fsource);//.enqueue_source(fsource);//.enqueue_source(source);
    
    
    // let sleep = sleep(TD::from_secs(10));
    // tokio::pin!(sleep);
    // sleep.as_mut().await;


    let _ = handle.add_event(
        Event::Track(TrackEvent::End),
        EndLeaver { manager, guild_id },
    );

    // handle.stop();
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


struct EndLeaver {
    pub manager: Arc<Songbird>,
    pub guild_id: GuildId,
}

#[async_trait]
impl VoiceEventHandler for EndLeaver {
    async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
        if let Some(conn) = self.manager.get(self.guild_id) {
            let should_remove = conn.lock().await.queue().is_empty();
            if should_remove {
                if let Err(err) = self.manager.remove(self.guild_id).await {
                    eprintln!("Failed to leave after track end: {err}");
                }
            }
        }
        None
    }
}
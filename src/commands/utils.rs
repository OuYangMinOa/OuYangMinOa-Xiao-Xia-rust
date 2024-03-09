
use humantime::format_duration;
use songbird::Call;
use tokio::sync::Mutex;
use youtube_dl::{SearchOptions, SingleVideo, YoutubeDl};
use url::Url;
use std::sync::Arc;
use std::{process::Stdio, time::Duration};
use songbird::input::{children_to_reader, Codec, Container, Input, Metadata};
// use songbird::input::error::Error;
use std::process::{Child, Command};
use uuid::Uuid;
use crate::Error;


use crate::data::info::MUSICPATH;
#[allow(unused)]
fn build_ytdlp_args(query:&str) -> [&str;11]{
    [
    "--print-json",
    "-f",
    "webm[abr>0]/bestaudio/best",
    "-R",
    "infinite",
    "--no-playlist",
    "--ignore-config",
    "--no-warnings",
    query,
    "-o",
    "-",
    ]
}

fn build_ffmpeg_args() -> [&'static str;9] {
    [
    "-f",
    "s16le",
    "-ac",
    "2",
    "-ar",
    "48000",
    "-acodec",
    "pcm_f32le",
    "-",
    ]
}

/// Build the songbird metadata with SingleVideo
fn build_metadata_from_source(source:SingleVideo) -> Metadata{
    Metadata{
        track:source.track,
        artist:source.artist,
        date:source.release_date,
        channels:Some(2),
        channel:source.channel,
        start_time:Some(Duration::new(0,0)),
        duration: Some(Duration::new(source.duration.unwrap().as_number().unwrap().as_u64().unwrap()   ,0) ),
        sample_rate:Some(source.asr.unwrap() as u32 ),
        source_url:source.webpage_url,
        title:source.title,
        thumbnail:source.thumbnail,
    }

}

async fn download_yt_source(url:&String,path:&str,file_option:Option<String>){
        match file_option {
            Some(filename) => {
                YoutubeDl::new(url)
                .socket_timeout("120")
                .extract_audio(true)
                .output_template(filename)
                // .download_to(path)
                .download_to_async(path)
                .await
                .unwrap();

            }
            None => {
                YoutubeDl::new(url)
                .socket_timeout("120")
                .extract_audio(true)
                // .download_to(path)
                .download_to_async(path)
                .await
                .unwrap();
            }
        }
        
        
}

async fn build_youtubedl_source(uri:String) -> SingleVideo{
    if let Ok(url) = Url::parse(&uri) {
        YoutubeDl::new(url)
            .socket_timeout("120")
            .extract_audio(true)
            // .run()
            .run_async()
            .await
            .unwrap()
            .into_single_video()
            .unwrap()
    }else {
        YoutubeDl::search_for(&SearchOptions::youtube(&uri))
        .socket_timeout("120")
        .extract_audio(true)
        .run_async()
        .await
        .unwrap()
        .into_single_video()
        .unwrap()
    }
}

async fn _build_ffmpeg_option(path:&String) ->Child{
    let child = Command::new("ffmpeg")
        .arg("-i")
        .arg(path)
        .args(build_ffmpeg_args())
        .stderr(Stdio::null())
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .spawn().unwrap();
    child
}

async fn grab_yt_list(url:&String,start:u8,end:u8) -> Vec<SingleVideo>{
    let result: Vec<SingleVideo> = YoutubeDl::new(url)
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36")
            .socket_timeout("30")
            .process_timeout(Duration::new(30,0))
            .extra_arg("-I")
            .extra_arg(format!("{start}:{end}"))
            .run_async()
            .await
            .expect("Error")
            .into_playlist()
            .unwrap()
            .entries
            .unwrap();
        result
}

async fn build_input_vec_with_single_video_vec(play_list:&Vec<SingleVideo>) -> Vec<Input>{
    let mut output :Vec<Input> = Vec::new();
    for each_video in play_list{
        let this_meta: Metadata = build_metadata_from_source(each_video.clone());
        let metaclone = this_meta.clone();
        let newurl = metaclone.source_url.unwrap();
        let thistitle = metaclone.title.unwrap() + format!("{}",Uuid::new_v4()).as_str(); // with same song title will cause the bot can't sing the same song in different guild
        // println!("{:?}",this_meta);
        let save_path = format!("{MUSICPATH}/{thistitle}.opus");
        download_yt_source(&newurl, MUSICPATH,Some(thistitle)).await;
        let command = _build_ffmpeg_option(&save_path).await;
        let this_input = Input::new(
            true,
            children_to_reader::<f32>(vec![command]), //(vec![youtube_dl, ffmpeg]),
            Codec::FloatPcm,
            Container::Raw,
            Some(this_meta)
        );
        output.push(this_input);
    };
    output
}


/// grab the youtbe source and build the input use for song bird
/// then download the video in thrread.
pub async fn handle_list_url(ctx:poise::Context<'_, (), Error>,uri:&String,conn:Arc<Mutex<Call>>){
    let source_yt_first: Vec<SingleVideo> = grab_yt_list(uri,1,3).await;

    let mut conn_locked = conn.lock().await;
    let (current_channel, _) =(conn_locked.current_channel().unwrap(),conn_locked.queue().current_queue());

    ctx.send(|r| {
        r.embed(|e| {
            e.title(format!("Queue in <#{current_channel}> (other music will be added later)"));
            let body = source_yt_first
            .iter()
            .map(|x| {
                format!("• [{}]({})",x.title.as_ref().unwrap(),x.webpage_url.as_ref().unwrap())
            })
            .collect::<Vec<_>>()
            .join("\n");
            e.description(body);
            e
        })
    }).await.unwrap();

    // download the first ten video
    for this_input in build_input_vec_with_single_video_vec(&source_yt_first).await{
        conn_locked.enqueue_source(this_input);
    }
    let source_yt_sec: Vec<SingleVideo> = grab_yt_list(&uri.clone(),4,50).await;
    for this_input in build_input_vec_with_single_video_vec(&source_yt_sec).await{
        conn_locked.enqueue_source(this_input);
    } 
    drop(source_yt_sec);
}


/// Handle single video including the case that uri is not url
pub async fn handle_single_video(ctx:poise::Context<'_, (), Error>,uri:&String,conn:Arc<Mutex<Call>>){

    let single_video = build_youtubedl_source(uri.to_string()).await;
    let this_input = build_input_vec_with_single_video_vec(&vec![single_video.clone()]).await;

    let mut conn_locked = conn.lock().await;
    let (current_channel, _) =(conn_locked.current_channel().unwrap(),conn_locked.queue().current_queue());

    for this_input in this_input{
        conn_locked.enqueue_source(this_input);
    }

    let metadata = build_metadata_from_source(single_video);
 
    ctx.send(|r| {
        r.embed(|e| {
            e.title(format!("Queueing audio in <#{current_channel}>"));
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
    }).await.unwrap();
}


//////// unused function

#[allow(unused)]
async fn build_songbird_ffmpeg_yt_dl_child(uri:&str) -> Result<(Child,Child),Error>{
    let ytdl_args = build_ytdlp_args(&uri);
    let ffmpeg_args = build_ffmpeg_args();
    let mut youtube_dl: std::process::Child = Command::new("yt-dlp")
    .args(&ytdl_args)
    .stdin(Stdio::null())
    .stderr(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn().unwrap();
    let taken_stdout = youtube_dl.stdout.take().unwrap();
    let ffmpeg = Command::new("ffmpeg")
    .arg("-i")
    .arg("-")
    .args(&ffmpeg_args)
    .stdin(taken_stdout)
    .stderr(Stdio::null())
    .stdout(Stdio::piped())
    .spawn().unwrap();

    Ok((youtube_dl,ffmpeg))
}

#[allow(unused)]
pub async fn build_songbird_source(uri:String) -> Result<(Input,String),Error>{
    println!("[*] Checking ... {uri}");
    // is a url
    let source = build_youtubedl_source(uri).await;
    let this_meta: Metadata = build_metadata_from_source(source);
    let metaclone = this_meta.clone();
    let newurl = metaclone.source_url.unwrap();
    let thistitle = metaclone.title.unwrap() + format!("{}",Uuid::new_v4()).as_str(); // with same song title will cause the bot can't sing the same song in different guild
    // println!("{:?}",this_meta);
    let save_path = format!("{MUSICPATH}/{thistitle}.opus");
    download_yt_source(&newurl, MUSICPATH,Some(thistitle)).await;

    /* 
    let _youtube_dl;
    let _ffmpeg ;
    (_youtube_dl,_ffmpeg) = build_songbird_ffmpeg_yt_dl_child(newurl.as_str()).await.unwrap();
    Ok(
        (Input::new(
        true,
        children_to_reader::<f32>(vec![youtube_dl, ffmpeg]), //(vec![youtube_dl, ffmpeg]),
        Codec::FloatPcm,
        Container::Raw,
        Some(this_meta))
        ,save_path)
    */


    // return Metadata::from_ytdl_output(value)
    let command = _build_ffmpeg_option(&save_path).await;
    Ok(
        (Input::new(
        true,
        children_to_reader::<f32>(vec![command]), //(vec![youtube_dl, ffmpeg]),
        Codec::FloatPcm,
        Container::Raw,
        Some(this_meta))
    ,save_path)

    )
}


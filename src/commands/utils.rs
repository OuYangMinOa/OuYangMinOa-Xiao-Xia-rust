use youtube_dl::{SearchOptions, SingleVideo, YoutubeDl};
use url::Url;
use std::{process::Stdio, time::Duration};
use songbird::input::{children_to_reader, Codec, Container, Input, Metadata};
use songbird::input::error::Error;
use std::process::{Child, Command};
use uuid::Uuid;

use crate::data::info::MUSICPATH;

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

async fn build_songbird_ffmpeg_yt_dl_child(uri:&str) -> Result<(Child,Child),Error>{
    let ytdl_args = build_ytdlp_args(&uri);
    let ffmpeg_args = build_ffmpeg_args();


    let mut youtube_dl: std::process::Child = Command::new("yt-dlp")
    .args(&ytdl_args)
    .stdin(Stdio::null())
    .stderr(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn().unwrap();
    let taken_stdout = youtube_dl.stdout.take().ok_or(Error::Stdout)?;
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


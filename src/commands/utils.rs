use youtube_dl::{SearchOptions, YoutubeDl};
use url::Url;
use std::{process::Stdio, time::Duration};
use songbird::input::{children_to_reader, Codec, Container, Input, Metadata};
use songbird::input::error::Error;
use std::process::Command;

pub fn get_youtube_source(uri:String) -> Result<Input,Error>{

    if let Ok(url) = Url::parse(&uri) {
        println!("[*] Downloading ... {url}");
        let source = YoutubeDl::new(url.clone())
        .socket_timeout("15")
        .extract_audio(true)
        .run()
        .unwrap()
        .into_single_video()
        .unwrap();

        YoutubeDl::new(url)
        .socket_timeout("15")
        .extract_audio(true)
        .download_to(r"data/")
        .unwrap();








        
        let this_meta: Metadata = Metadata{
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
        };
        println!("{:?}",this_meta);


        let ytdl_args = [
            "--print-json",
            "-f",
            "webm[abr>0]/bestaudio/best",
            "-R",
            "infinite",
            "--no-playlist",
            "--ignore-config",
            "--no-warnings",
            &uri,
            "-o",
            "-",
        ];
        let ffmpeg_args = [
            "-f",
            "s16le",
            "-ac",
            "2",
            "-ar",
            "48000",
            "-acodec",
            "pcm_f32le",
            "-",
        ];
        let mut youtube_dl = Command::new("yt-dlp")
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



        // return Metadata::from_ytdl_output(value)

        Ok(Input::new(
            true,
            children_to_reader::<f32>(vec![youtube_dl, ffmpeg]),
            Codec::FloatPcm,
            Container::Raw,
            Some(this_meta),
        ))
    }else{
        let source = YoutubeDl::search_for(&SearchOptions::youtube(uri))
        .socket_timeout("15")
        .extract_audio(true)
        .run()
        .unwrap()
        .into_single_video()
        .unwrap();


        
        // let newurl = source.webpage_url.unwrap().clone().as_str();

        let newurl = source.webpage_url.unwrap();
        let this_meta: Metadata = Metadata{
            track:source.track,
            artist:source.artist,
            date:source.release_date,
            channels:Some(2),
            channel:source.channel,
            start_time:Some(Duration::new(0,0)),
            duration: Some(Duration::new(source.duration.unwrap().as_number().unwrap().as_u64().unwrap()   ,0) ),
            sample_rate:Some(source.asr.unwrap() as u32 ),
            source_url:Some(newurl.clone()),
            title:source.title,
            thumbnail:source.thumbnail,
        };
        println!("{:?}",this_meta);

        let ytdl_args = [
            "--print-json",
            "-f",
            "webm[abr>0]/bestaudio/best",
            "-R",
            "infinite",
            "--no-playlist",
            "--ignore-config",
            "--no-warnings",
            newurl.as_str(),
            "-o",
            "-",
        ];
        let ffmpeg_args = [
            "-f",
            "s16le",
            "-ac",
            "2",
            "-ar",
            "48000",
            "-acodec",
            "pcm_f32le",
            "-",
        ];
        let mut youtube_dl = Command::new("yt-dlp")
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



        // return Metadata::from_ytdl_output(value)

        Ok(Input::new(
            true,
            children_to_reader::<f32>(vec![youtube_dl, ffmpeg]),
            Codec::FloatPcm,
            Container::Raw,
            Some(this_meta),
        ))
    }

}
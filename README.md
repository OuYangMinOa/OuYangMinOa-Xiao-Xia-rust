<p align="center">
  <a href="" rel="noopener">
 <img width=200px height=200px src="https://github.com/OuYangMinOa/OuYangMinOa-Xiao-Xia-rust/blob/main/icon.png" alt="Bot logo"></a>
</p>

<h3 align="center">Xiao-Xia with rust</h3>
</div>

## 🧐 About <a name = "about"></a>

Trying to rebuild <a herf="https://github.com/OuYangMinOa/Xiao-Xia">Xiao-Xia</a> with rust 

## Progress

1. Music Bot


## 🚀 Deploying your own bot <a name = "deployment"></a>

If you want to build your own bot, the bot is develop base on rust. rust is require for installation, ffmpeg for audio procession and yt-dlp for youtube video downloading. Here are the procedure to run Xiao-xia with rust locally.

1. Add personal discord [token](https://discord.com/developers/docs/topics/oauth2) 

      Edit  `.env_example` and rename the file to `.env`

2. Install ffmpeg

    For Windows: [tutorial](https://blog.gregzaal.com/how-to-install-ffmpeg-on-windows/)

    For Mac: `brew install ffmpeg`

    For Linux : `sudo apt-install ffmpeg`

4. Install yt-dlp
    
    You can use `pip install yt-dlp` if pip is installed [otherwise](https://github.com/yt-dlp/yt-dlp)

3. Run

    ```
    cargo check
    cargo run
    ```
4. Run with docker

    ```
    docker build -t xiao-xia-rust .
    docker run -d xiao-xia-rust
    ```

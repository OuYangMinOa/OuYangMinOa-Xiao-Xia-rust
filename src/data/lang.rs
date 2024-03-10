#[derive(PartialEq, poise::ChoiceParameter)]
#[allow(non_camel_case_types)]
pub enum SuppportLanguage {
    en,
    zhtw,
}

pub struct LanguageDescription<'a> {
    pub help: &'a str,
}

pub static ENLANGMAP: LanguageDescription = LanguageDescription {
    help: "* :notes: **MUSIC**
 - `/play {url}` play music.
 - `/skip` to skip the song.
 - `/pause` to pause the song.
 - `/resume` to resume the song.
 - `/list` to show the playlist.
 - `/clear` to clear the playlist.",
};

pub static TWLANGMAP: LanguageDescription = LanguageDescription {
    help: "* :notes: **MUSIC**
 - `/play {url}` 播放音樂。
 - `/skip` 跳過。
 - `/pause` 暫停。
 - `/resume` 繼續。
 - `/list` 看撥放清單。
 - `/clear` 清除播放清單。",
};

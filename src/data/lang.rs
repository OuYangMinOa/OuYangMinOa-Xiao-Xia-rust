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
    help: "Help message in taiwanese",
};

pub static TWLANGMAP: LanguageDescription = LanguageDescription {
    help: "Help message in taiwanese",
};

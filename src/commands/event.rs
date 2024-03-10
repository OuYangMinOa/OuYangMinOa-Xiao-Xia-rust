/// Handle some event

pub struct MyEventSetter;

use poise::serenity_prelude::*;


#[async_trait]
impl EventHandler for MyEventSetter {
    async fn message(&self, _ctx: Context, msg: Message) {
        println!("[*] {} : {}",msg.author.name, msg.content);
    }

    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("[*] {} is connected!", ready.user.name);
    }

}
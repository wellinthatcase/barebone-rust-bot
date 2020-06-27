use std::fs;
use serde::Deserialize;
use serenity::{
    prelude::*,
    model::{
        gateway::Activity,
        channel::Message, 
        gateway::Ready,
    },
    framework::{
        StandardFramework,
        standard::macros::group,
    },
    async_trait
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "gn ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "shut up").await {
                println!("error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Logged in on {}#{} ({}).", &ready.user.name, &ready.user.discriminator, &ready.user.id);
        ctx.set_activity(Activity::playing("with v0.1.0")).await;
    }
}

#[derive(Deserialize)]
struct Config<'a, 'b> { token: &'a str, version: &'b str }

#[group]
struct General;

#[tokio::main]
async fn main() {
    let content = fs::read_to_string("config.toml").unwrap();
    let config: Config = toml::from_str(&content).unwrap();

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("gn ")).group(&GENERAL_GROUP);

    let mut client = Client::new(&config.token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("failed creating the client");

    println!("Starting with v{}...", config.version);

    if let Err(why) = client.start().await {
        println!("error: {:?}", &why);
    }
}
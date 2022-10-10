#![allow(non_snake_case, unused_imports)]

use std::process::exit;

use serenity::{async_trait, http};
use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::model::channel::{Message, Channel};
use serenity::model::gateway::Ready;
use serenity::builder;
use serenity::model::webhook::Webhook;
use serenity::prelude::*;

use owo_colors::{
    colors::*,
    OwoColorize, Style
};
use reqwest::blocking;

mod formats;
use formats::*;
mod constants;
use constants::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            // Sending a message can fail, due to a network error, an
            // authentication error, or lack of permissions to post in the
            // channel, so log to stdout when some error happens, with a
            // description of it.
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = "MTAyNDc0NDUwMDI1OTcyOTQ2OA.GfVUbS.y2DaEldO7wth-FELlMrdZIICoTbysaJ8Gg1kaE";
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client =
        Client::builder(&token, intents)
            .event_handler(Handler)
            .framework(StandardFramework::new()).await
            .expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    let webhook = Webhook::from_url(Http::new(token), log_webhook).await.unwrap();
    webhook.execute(Http::new(token), false, |w| {
        w.content("Bot online!")
    }).await.unwrap();

    if let Err(why) = client.start().await {
        
    }
}































































































































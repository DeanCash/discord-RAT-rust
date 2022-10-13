// #![windows_subsystem = "windows"]
#![allow(non_snake_case, unused_imports)]

use std::process::exit;
use std::collections::{HashSet, HashMap};
use std::sync::Arc;

use serenity::model::prelude::Activity;
use serenity::model::user::OnlineStatus;
use serenity::{async_trait, http};
use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::model::channel::{Message, Channel};
use serenity::model::gateway::Ready;
use serenity::model::webhook::Webhook;
use serenity::model::id::UserId;
use serenity::prelude::*;
use serenity::builder;

use sysinfo::{System, SystemExt, UserExt, *};
use reqwest;
use owo_colors::{
    colors::*,
    OwoColorize, Style
};

mod formats;
mod helpers;
mod commands;
use formats::*;
use helpers::*;
use commands::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    // async fn message(&self, ctx: Context, msg: Message) {
    //     if msg.content == "aadadaw" {
    //         // Sending a message can fail, due to a network error, an
    //         // authentication error, or lack of permissions to post in the
    //         // channel, so log to stdout when some error happens, with a
    //         // description of it.
    //         if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
    //             println!("Error sending message: {:?}", why);
    //         }
    //     }
    // }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, ctx: Context, ready: Ready) {
        // send a webhook in the LOGS channel saying that the bot is now online
        let http = Http::new("");
        let webhook = Webhook::from_url(&http, LOG_WEBHOOK).await
            .expect("Couldn't create webhook")
            .execute(&http, false, |f| f
                .content(create_log_msg("Bot is now online!".to_string()))
            ).await.expect("Couldn't send OFF webhook");

        ctx.set_presence(
            Some(Activity::watching(format!("prefix {}", BOT_PREFIX))),
            OnlineStatus::DoNotDisturb
        ).await;
        
        // println!(" version : {}", c.http.create_channel(guild_id, map, audit_log_reason));


        println!(" {} {} is connected!", Pr::bot(), ready.user.tag());
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = "MTAyNDc0NDUwMDI1OTcyOTQ2OA.GfVUbS.y2DaEldO7wth-FELlMrdZIICoTbysaJ8Gg1kaE";
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::all();

    let framework = StandardFramework::new()
        .configure(|c| c.owners(
            vec![
                UserId(523990741543026689)
            ].into_iter().collect())
            .case_insensitivity(true)
            .prefix(BOT_PREFIX)
        )
        .group(&OWNER_GROUP);

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client =
        Client::builder(&token, intents)
            .event_handler(Handler)
            .framework(framework).await
            .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    // {
    //     let mut data = client.data.write().await;
    //     data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    // }

    if let Err(why) = client.start_autosharded().await {
        // if bot couldn't start up send a log message via webhook
        println!(" {} Couldn't start bot :(", Pr::err());
        let http = Http::new("");
        let webhook = Webhook::from_url(&http, LOG_WEBHOOK).await
            .expect("Couldn't create webhook")
            .execute(&http, false, |f| f
            .content(create_log_msg(format!("Bot ERROR couldn't start because: {}", why)))
        ).await.expect("Couldn't send OFF webhook");
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    fn get_ip() {
        type Store = HashMap::<String, String>;
        let res = reqwest::blocking::get("https://api.ipify.org?format=json");

        let a = res.unwrap().json::<Store>().unwrap();
        
        println!("RES: {:?}", a);
        let e = if a.contains_key("ip") {
            a.get("ip").unwrap().to_owned()
        } else {
            String::from("undefined")
        };
        println!("json: {}", e);
    }

    #[test]
    fn concat_users() {
        let system = System::new_all();
        let user_str = system.users()
            .into_iter()
            .map(|i| i.name())
            .collect::<Vec<&str>>()
            .join(" ");

        println!(" : {:?}", user_str);
        println!(" : {:?}", system.users());
    }

    #[test]
    fn prefix_msgs() {
        println!(" {} Couldn't start bot :(", Pr::app());
        println!(" {} Couldn't start bot :(", Pr::bot());
        println!(" {} Couldn't start bot :(", Pr::err());
        println!(" {} Couldn't start bot :(", Pr::event());
    }
}




























































































































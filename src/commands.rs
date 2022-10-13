#![allow(non_snake_case, unused_imports)]

use std::sync::Arc;

use serenity::builder::CreateEmbed;
use serenity::client::bridge::gateway::{ShardId, ShardManager};
use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::*;
use serenity::model::prelude::*;
use serenity::utils::Color;
use serenity::{async_trait, http};
use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::model::channel::{Message, Channel};
use serenity::model::gateway::Ready;
use serenity::model::webhook::Webhook;
use serenity::prelude::*;

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}



#[group]
#[commands(ping)]
struct Owner;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    // let data = ctx.data.read().await;
    
    // let shard_manager = match data.get::<ShardManagerContainer>() {
    //     Some(v) => v,
    //     None => {
    //         return Ok(());
    //     },
    // };
    
    // let manager = shard_manager.lock().await;
    // let runners = manager.runners.lock().await;
    
    // // Shards are backed by a "shard runner" responsible for processing events
    // // over the shard, so we'll get the information about the shard runner for
    // // the shard this command was sent over.
    // let runner = match runners.get(&ShardId(ctx.shard_id)) {
    //     Some(runner) => runner,
    //     None => {            
    //         let embed = msg.channel_id
    //             .send_message(&ctx.http, |m|
    //                 m.embed(|e| e
    //                     .color(Color::DARK_MAGENTA)
    //                     .description(format!("🏓 Pong!"))
    //                     .footer(|f| f
    //                         .text("Could't figure out the bots latency...")
    //                     )
    //                 )
    //             ).await?;
    //         return Ok(());
    //     },
    // };

    let embed = msg.channel_id
        .send_message(&ctx.http, |m|
            m.embed(|e| e
                .color(Color::DARK_MAGENTA)
                .description(format!("🏓 Pong!"))
                // .description(format!("🏓 Bots Latency: {:?}", runner.latency))
            )
        ).await;
    
    match embed {
        Ok(o) => { println!("sent Ping response!"); },
        Err(e) => { println!(" Couldn't send message: {}", e); }
    }

    Ok(())
}






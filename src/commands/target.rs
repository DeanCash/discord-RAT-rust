
use serenity::prelude::*;
use serenity::client::bridge::gateway::{ShardId, ShardManager};
use serenity::framework::standard::{
    CommandResult,
    macros::*, Args
};
use serenity::utils::Color;
use serenity::model::{
    channel::{Message,},
};
use sysinfo::{System, SystemExt, *};

use crate::formats::Pr;
use crate::utilities::{
    get_pub_ip,
    get_system_users
};

#[group]
#[commands(ping)]
struct General;


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
        Ok(o) => { println!(" {} sent Ping response!", Pr::event()); },
        Err(e) => { println!(" {} Couldn't send message: {}", Pr::err(), e); }
    }

    Ok(())
}

#[command]
async fn target(ctx: &Context, msg: &Message) -> CommandResult{


    Ok(())
}






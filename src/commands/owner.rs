#![allow(non_snake_case, unused_imports)]

use std::process;
use std::sync::Arc;

use serenity::prelude::*;
use serenity::builder::CreateEmbed;
use serenity::client::bridge::gateway::{ShardId, ShardManager};
use serenity::framework::standard::{
    CommandResult,
    macros::*, Args
};
use serenity::model::prelude::*;
use serenity::utils::Color;
use serenity::{async_trait, http};
use serenity::http::Http;
use serenity::model::{
    channel::{Message, Channel},
    gateway::Ready,
    webhook::Webhook
};
use sysinfo::{System, SystemExt, *};

use crate::formats::Pr;
use crate::utilities::{
    get_pub_ip, LOG_WEBHOOK, send_log_webhook,
    get_system_users, clear_console, create_log_msg
};

#[group]
#[owners_only]
#[commands(cls, webhook)]
struct OwnerCommands;


#[command]
async fn cls(_: &Context, _: &Message) -> CommandResult {
    clear_console();
    Ok(())
}


#[command]
async fn webhook(ctx: &Context, msg: &Message) -> CommandResult {
    send_log_webhook(LOG_WEBHOOK.to_string(), 
        format!("{}: {}", msg.author.tag(), msg.content).as_str()
    ).await;
    Ok(())
}








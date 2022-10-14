#![allow(non_snake_case, unused_imports)]

use std::process;
use std::io::{self, Write};
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
    get_pub_ip,
    get_all_users
};

#[group]
#[owners_only]
#[commands(cls)]
struct OwnerCommands;


#[command]
async fn cls(_: &Context, _: &Message) -> CommandResult {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    io::stdout().flush().expect("Couldn't CLS console");
    Ok(())
}







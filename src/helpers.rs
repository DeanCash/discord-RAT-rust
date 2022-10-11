use std::time;
use chrono::prelude::*;

pub const LOG_WEBHOOK: &str = "https://discord.com/api/webhooks/1028991227925897240/ta_VM8totTAxI02pu4IvuHutj2Aw1YJlXP3R0B9L68ahDkUDw1K2ByvX9vqzrAAiaHj0";
pub const LOG_WEBHOOK_ID: &str = "1028991227925897240";
pub const LOG_WEBHOOK_TOKEN: &str = "ta_VM8totTAxI02pu4IvuHutj2Aw1YJlXP3R0B9L68ahDkUDw1K2ByvX9vqzrAAiaHj0";

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

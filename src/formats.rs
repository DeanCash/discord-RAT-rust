use owo_colors::{
    colors::*,
    OwoColorize, Style
};

pub struct Pr;

impl Pr {
    pub fn err() -> String {
        format!(
            "{}{}{}",
            "[".bold(),
            "ERROR".red().bold(),
            "] >>".bold()
        )
    }

    pub fn bot() -> String {
        format!(
            "{}{}{}",
            "[".bold(),
            "BOT".cyan().bold(),
            "] >>".bold()
        )
    }

    pub fn app() -> String {
        format!(
            "{}{}{}",
            "[".bold(),
            "APPLICATION".blue().bold(),
            "] >>".bold()
        )
    }

    pub fn event() -> String {
        format!(
            "{}{}{}{}",
            "[".blue().bold(),
            "EVENT".bold(),
            "]".blue().bold(),
            " >>".bold()
        )
    }
}

use serde_derive::Serialize;
use std::env;
#[derive(Debug, Serialize)]
pub struct DiscordMessage {
    pub embeds: Vec<DiscordEmbed>,
}
#[derive(Debug, Serialize)]
pub struct DiscordEmbed {
    pub title: String,
    pub url: String,
    pub description: String,
    pub thumbnail: DiscordImage,
}
#[derive(Debug, Serialize)]
pub struct DiscordImage {
    url: String,
}

pub fn send_discord_message(
    client: reqwest::blocking::Client,
    message: String,
    link: String,
    desc: String,
    picture: String,
) {
    let body = DiscordMessage {
        embeds: vec![DiscordEmbed {
            title: message.to_string(),
            url: link,
            description: desc,
            thumbnail: DiscordImage { url: picture },
        }],
    };

    let discord_api = env::var("HEAD_API").expect("Unable to give head");
    let _ = client
        .post(discord_api)
        .json(&body)
        .send()
        .expect("Unable to send discord message!");
}

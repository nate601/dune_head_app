use rand::seq::SliceRandom;
use serde_derive::{Deserialize, Serialize};
use serde_xml_rs::from_str;
use std::env;

#[derive(Debug, Deserialize)]
struct Rss {
    channel: Channel,
}

#[derive(Debug, Deserialize)]
struct Channel {
    item: Vec<Item>,
}
#[derive(Debug, Deserialize)]
struct Item {
    title: String,
    news_item: Vec<NewsItem>,
    picture: String,
}
#[derive(Debug, Deserialize)]
struct NewsItem {
    news_item_title: String,
    news_item_url: String,
}
fn main() {
    let client = reqwest::blocking::Client::new();
    let res = client
        .get("https://trends.google.com/trends/trendingsearches/daily/rss?geo=US")
        .send()
        .expect("Unable to get current trends!")
        .text()
        .expect("Unable to parse response!");
    //println!("res : {}", res);
    let rss: Rss = from_str(&res).unwrap();
    // println!("{:?}",rss);
    let items = rss.channel.item;
    for i in &items {
        let title = htmlescape::decode_html(&i.title).expect("Unable to decode title");
        let meme_text = get_meme_text(&title);
        println!("{}", meme_text);
        // println!("{} this, {} that.  I need some head.", title, title);
        let news_title = htmlescape::decode_html(&i.news_item[0].news_item_title)
            .expect("Unable to decode news_title");
        println!("\t{}", news_title);
    }
    let mut rng = rand::thread_rng();
    let choice = items.choose(&mut rng).expect("Unable to randomly choose");
    send_discord_message(
        client,
        get_meme_text(&htmlescape::decode_html(&choice.title).expect("unable to decode title")),
        choice.news_item[0].news_item_url.to_string(),
        htmlescape::decode_html(&choice.news_item[0].news_item_title)
            .expect("Unable to decode news title")
            .to_string(),
        choice.picture.to_string(),
    );
}
#[derive(Debug, Serialize)]
struct DiscordMessage {
    embeds: Vec<DiscordEmbed>,
}
#[derive(Debug, Serialize)]
struct DiscordEmbed {
    title: String,
    url: String,
    description: String,
    thumbnail: DiscordImage,
}
#[derive(Debug, Serialize)]
struct DiscordImage {
    url: String,
}

fn send_discord_message(
    client: reqwest::blocking::Client,
    message: String,
    link: String,
    desc: String,
    picture: String,
) {
    /*
    let mut map:HashMap<String, String> = HashMap::new();
    map.insert("".to_string(), "".to_string());
    map.insert("content".to_string(), message);
    */
    let body = DiscordMessage {
        embeds: vec![DiscordEmbed {
            title: message.to_string(),
            url: link,
            description: desc,
            thumbnail: DiscordImage { url: picture },
        }],
    };

    let discord_api = env::var("HEAD_API").expect("Unable to give head");
    let _ = client.post(discord_api)
        .json(&body)
        .send()
        .expect("Unable to send discord message!");
}
fn get_meme_text(title: &String) -> String {
    format!("{} this, {} that.  I need some head.", title, title)
}

use rand::seq::SliceRandom;
use serde_xml_rs::from_str;
mod discord;
pub use crate::discord::*;
mod rss;
pub use crate::rss::*;

fn main()
{
    let client = reqwest::blocking::Client::new();
    let res = client
        .get("https://trends.google.com/trends/trendingsearches/daily/rss?geo=US")
        .send()
        .expect("Unable to get current trends!")
        .text()
        .expect("Unable to parse response!");
    let rss_string: Rss = from_str(&res).unwrap();
    let items = rss_string.channel.item;
    for i in &items
    {
        let title = htmlescape::decode_html(&i.title).expect("Unable to decode title");
        let meme_text = get_meme_text(&title);
        println!("{}", i.approx_traffic);
        println!("{}", meme_text);
        let news_title = htmlescape::decode_html(&i.news_item[0].news_item_title)
            .expect("Unable to decode news_title");
        println!("\t{}", news_title);
    }
    let mut rng = rand::thread_rng();
    let choice = items.choose(&mut rng).expect("Unable to randomly choose");
    let headline = items
        .iter()
        .max_by_key(|p| p.get_approx_traffic().unwrap())
        .unwrap_or(choice);
    send_discord_message(
        client,
        get_meme_text(&htmlescape::decode_html(&headline.title).expect("unable to decode title")),
        headline.news_item[0].news_item_url.to_string(),
        htmlescape::decode_html(&headline.news_item[0].news_item_title)
            .expect("Unable to decode news title")
            .to_string(),
        headline.picture.to_string(),
    );
}
fn get_meme_text(title: &String) -> String
{
    format!("{} this, {} that.  I need some head.", title, title)
}

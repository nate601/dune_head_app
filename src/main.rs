use serde_derive::Deserialize;
use std::io;
use serde_xml_rs::from_str;

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
}
#[derive(Debug, Deserialize)]
struct NewsItem {
    news_item_title: String,
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
    for i in items {
        let title = htmlescape::decode_html(&i.title).expect("Unable to decode title");
        println!("{} this, {} that.  I need some head.", title, title);
        let news_title = htmlescape::decode_html(&i.news_item[0].news_item_title)
            .expect("Unable to decode news_title");
        println!("\t{}", news_title);
    }
    let mut discard_string = String::new();
    io::stdin().read_line(&mut discard_string).ok();
}

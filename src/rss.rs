use serde_derive::Deserialize;
#[derive(Debug, Deserialize)]
pub struct Rss {
    pub channel: Channel,
}
#[derive(Debug, Deserialize)]
pub struct Channel {
    pub item: Vec<Item>,
}
#[derive(Debug, Deserialize)]
pub struct Item {
    pub title: String,
    pub news_item: Vec<NewsItem>,
    pub picture: String,
    pub approx_traffic: String,
}
#[derive(Debug, Deserialize)]
pub struct NewsItem {
    pub news_item_title: String,
    pub news_item_url: String,
}

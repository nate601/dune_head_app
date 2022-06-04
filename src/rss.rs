use serde_derive::Deserialize;
#[derive(Debug, Deserialize)]
pub struct Rss
{
    pub channel: Channel,
}
#[derive(Debug, Deserialize)]
pub struct Channel
{
    pub item: Vec<Item>,
}
#[derive(Debug, Deserialize)]
pub struct Item
{
    pub title: String,
    pub news_item: Vec<NewsItem>,
    pub picture: String,
    pub approx_traffic: String,
}
impl Item
{
    pub fn get_approx_traffic(&self) -> Result<i32, &str>
    {
        let k = self
            .approx_traffic
            .replace(",", "")
            .replace("+", "")
            .parse::<i32>();
        match k
        {
            Ok(k) => return Ok(k),
            Err(_v) => return Err("Unable to parse traffic")
        };
    }
}
#[derive(Debug, Deserialize)]
pub struct NewsItem
{
    pub news_item_title: String,
    pub news_item_url: String,
}

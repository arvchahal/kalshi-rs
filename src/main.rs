use reqwest::{Client, Error, Response, Url};
use serde::Deserialize;
#[tokio::main]
async fn main() -> Result<(), Error>{
    println!("Hello, world!");
    const KALSHI:&str = "https://api.elections.kalshi.com/trade-api/v2/markets";
    let _x = send(KALSHI).await?;
    println!("{:?}", _x);
    let body = _x.text().await?;
    println!("{:?}",body);
    Ok(())
}

async fn send(url:&str)->Result<Response, Error> {
    let client = Client::new();
    let resp = client.get(url.parse::<Url>().unwrap()).send().await?;
    Ok(resp)

}
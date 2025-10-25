use kalshi_rust_sdk::KalshiClient;
use reqwest::{Client, Response, Url};
use serde_json::Value;
use tokio::fs;
use kalshi_rust_sdk::auth::auth_loader::load_auth_from_file;
use kalshi_rust_sdk::auth::Account;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    const KALSHI: &str = "https://api.elections.kalshi.com/trade-api/v2/markets";
    let x = load_auth()?;
    let t = KalshiClient::new(x);
    // let api_ = t.get_api_keys().await;

    // println!("{:?}",api_?);
    // let announcements = t.get_exchange_announcements().await;
    // println!("{}",announcements?);
    // let sched = t.get_exchange_schedule().await?;
    // println!("{}",sched);
    // let status = t.get_exchange_status().await?;
    // println!("{}",status);
    // let user_data = t.get_user_data_timestamp().await?;
    // println!("{}",user_data);
    // let market_data = t.get_all_markets(None, None, None, None, None, None, None, None).await?;
    // println!("{}",market_data);
    // let trades = t.get_trades(None, None, None, None, None).await?;
    //     println!("{}",trades);
    // let milestone = t.get_milestones(None).await?;
    // println!("Milestones: {:?}", milestone);
    // let all_events = t.get_all_events(None, None).await?;
    // println!("All events:\n{:#?}", all_events);


    // let event = t.get_event("KXNFLPASSYDS-25OCT23MINLAC").await?;
    // println!("Single event:\n{:#?}", event);
    // let series_list = t.get_all_series(Some(5), None).await?;
    // println!("All series:\n{:#?}", series_list);

    // Example: Fetch one specific series
    // let single_series = t.get_series_by_ticker("KXNFLPASSYDS").await?;
    // println!("Single series:\n{:#?}", single_series);

    // let meta = t.get_event_metadata("KXNFLPASSYDS-25OCT23MINLAC").await?;
    // println!("Event metadata:\n{:#?}", meta);
    // let market_orderbook = t.get_market_orderbook("KXNFLPASSYDS-25OCT23MINLAC-LACJHERBERT10-225", Some(8)).await?;
    // println!("{}",market_orderbook);

//     let resp = t.get_market_candlesticks(
//     "KXNFLPASSYDS",
//     "KXNFLPASSYDS-25OCT23MINLAC-LACJHERBERT10-225",
//     1761180723,  // start_ts
//     1761267123,  // end_ts
//     1,           // period_interval (1 minute)
// ).await?;

// // quick sanity print (keeps it short)
// println!(
//     "markets: {} | adjusted_end_ts: {}",
//     resp.market_tickers.len(),
//     resp.adjusted_end_ts
// );
// for (i, candles) in resp.market_candlesticks.iter().enumerate() {
//     println!("{}: {} candles", resp.market_tickers[i], candles.len());
// }


    
    Ok(())
}

async fn send(url: &str) -> Result<Response, Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = Url::parse(url)?;                //ParseError handled by Box<dyn Error>
    let resp = client.get(url).send().await?;   //reqwest::Error also fits Box<dyn Error>
    Ok(resp)
}

fn load_auth() -> Result<Account, Box<dyn std::error::Error>> {
    let _account = load_auth_from_file()?;
    Ok(_account)
}
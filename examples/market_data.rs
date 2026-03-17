use kalshi_rs::auth::Account;
use kalshi_rs::markets::models::*;
use kalshi_rs::KalshiClient;
use rust_decimal::{Decimal, prelude::FromPrimitive};
use std::{
    str::FromStr, time::{SystemTime, UNIX_EPOCH}
};

#[tokio::main]
/// Get market data: orderbook, trades, and candlesticks
///
/// Run with: cargo run --example market_data
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key_id = std::env::var("KALSHI_API_KEY_ID").expect("KALSHI_API_KEY_ID must be set");
    let account = Account::from_file("kalshi_private.pem", api_key_id)?;
    let client = KalshiClient::new(account);
    println!("Fetching active markets...");
    let params = MarketsQuery {
        limit: Some(500),
        status: Some("active".to_string()),
        cursor: None,
        event_ticker: None,
        series_ticker: None,
        min_close_ts: None,
        max_close_ts: None,
        tickers: None,
    };
    let markets = client.get_all_markets(&params).await?;
    println!("Total active markets returned: {}", markets.markets.len());
    if markets.markets.is_empty() {
        panic!("NO MARKETS RETURNED → you are 100% in sandbox.");
    }
    println!("Checking full details for each market...");
    let mut liquid_market: Option<Market> = None;
    for m in markets.markets.iter() {
        let ticker = &m.ticker;
        let full = client.get_market(ticker).await?;
        let md = full.market;
        
        if Decimal::from_str(&md.yes_ask_dollars).unwrap() < Decimal::from_u8(1).unwrap() {
            println!("Found liquid market: {}", md.ticker);
            liquid_market = Some(md);
            break;
        }
    }
    let market = match liquid_market {
        Some(m) => m,
        None => {
            panic!("No liq markets found we are cooked");
        }
    };
    let ticker = market.ticker.clone();
    let event_ticker = market.event_ticker.clone();
    println!("Using market: {}", ticker);
    println!("Title: {}\n", market.title);
    println!("1. Market Orderbook (depth: 10)");
    let orderbook = client.get_market_orderbook(&ticker, Some(10)).await?;
    if let Some(yes_orders) = &orderbook.orderbook_fp.yes_dollars {
        println!("   YES Side:");
        for (price, volume) in yes_orders.iter().take(5) {
            println!("      {} dollars - {} contracts", price, volume);
        }
    }
    if let Some(no_orders) = &orderbook.orderbook_fp.no_dollars {
        println!("   NO Side:");
        for (price, volume) in no_orders.iter().take(5) {
            println!("      {} dollars - {} contracts", price, volume);
        }
    }
    println!("\n2. Recent Trades:");
    let trades = client
        .get_trades(Some(10), None, Some(ticker.clone()), None, None)
        .await?;
    println!("   Last {} trades:", trades.trades.len());
    for trade in trades.trades.iter().take(5) {
        println!(
            "      {} contracts @ ${} ({})",
            trade.count_fp, if trade.taker_side == "yes" { &trade.yes_price_dollars } else { &trade.no_price_dollars }, trade.taker_side
        );
    }
    println!("\n3. Market Statistics");
    let market_details = client.get_market(&ticker).await?;
    let m = &market_details.market;
    println!("   Current Prices:");
    println!("      Yes Bid: ${}", m.yes_bid_dollars);
    println!("      Yes Ask: ${}", m.yes_ask_dollars);
    println!("      Last Price: ${}", m.last_price_dollars);
    println!();
    println!("   Market Activity:");
    println!("      Volume (24h): {}", m.volume_24h_fp);
    println!("      Total Volume: {}", m.volume_fp);
    println!("      Open Interest: {}", m.open_interest_fp);
    println!("      Liquidity: ${}", m.liquidity_dollars);
    println!("\n4. Price History (Last 7 Days)");
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64;
    let seven_days_ago = now - (7 * 24 * 60 * 60);
    let period = 1440;
    let candlesticks = client
        .get_market_candlesticks(&event_ticker, &ticker, seven_days_ago, now, period)
        .await;
    match candlesticks {
        Ok(candles) => {
            println!("   Candles returned: {}", candles.candlesticks.len());
            for candle in candles.candlesticks.iter().take(5) {
                println!("      Date: {}", candle.end_period_ts);
                if let Some(open) = candle.price.open {
                    println!("         Open: {} cents", open);
                }
                if let Some(high) = candle.price.high {
                    println!("         High: {} cents", high);
                }
                if let Some(low) = candle.price.low {
                    println!("         Low: {} cents", low);
                }
                if let Some(close) = candle.price.close {
                    println!("         Close: {} cents", close);
                }
                println!("         Volume: {}", candle.volume_fp);
                println!();
            }
        }
        Err(e) => {
            println!("   Candlestick data not available: {}", e);
        }
    }
    println!("Market data analysis done!");
    Ok(())
}

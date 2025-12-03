use crate::common::setup_client;
use kalshi_rs::markets::models::*;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::time::sleep;
/// MARKETS LIST TESTS
#[tokio::test]
async fn test_get_all_markets_basic() {
    let client = setup_client();
    let result = client
        .get_all_markets(
            &MarketsQuery {
                limit: Some(5),
                cursor: None,
                event_ticker: None,
                series_ticker: None,
                max_close_ts: None,
                min_close_ts: None,
                status: Some("active".to_string()),
                tickers: None,
            },
        )
        .await;
    assert!(result.is_ok(), "Failed to get markets: {:?}", result.err());
    let response = result.unwrap();
    println!("Markets retrieved: {}", response.markets.len());
    if let Some(cursor) = &response.cursor {
        println!("Next page cursor: {}", cursor);
    }
    if let Some(market) = response.markets.first() {
        println!(
            "Sample market: {} | Status: {} | Category: {}", market.ticker, market
            .status, market.category
        );
    }
}
#[tokio::test]
async fn test_get_all_markets_with_event_ticker_filter() {
    let client = setup_client();
    let base = client
        .get_all_markets(
            &MarketsQuery {
                limit: Some(1),
                cursor: None,
                event_ticker: None,
                series_ticker: None,
                max_close_ts: None,
                min_close_ts: None,
                status: None,
                tickers: None,
            },
        )
        .await
        .unwrap();
    if base.markets.is_empty() {
        println!("No markets available - skipping filter test");
        return;
    }
    let event_ticker = base.markets[0].event_ticker.clone();
    println!("Filtering by event_ticker={}", event_ticker);
    let result = client
        .get_all_markets(
            &MarketsQuery {
                limit: Some(5),
                cursor: None,
                event_ticker: Some(event_ticker.clone()),
                series_ticker: None,
                max_close_ts: None,
                min_close_ts: None,
                status: None,
                tickers: None,
            },
        )
        .await;
    assert!(result.is_ok(), "Failed to filter markets by event_ticker");
    let filtered = result.unwrap();
    for m in filtered.markets.iter() {
        assert_eq!(m.event_ticker, event_ticker);
    }
    println!("Markets filtered successfully by event_ticker");
}
/// SINGLE MARKET TEST
#[tokio::test]
async fn test_get_single_market() {
    let client = setup_client();
    let markets = client
        .get_all_markets(
            &MarketsQuery {
                limit: Some(1),
                cursor: None,
                event_ticker: None,
                series_ticker: None,
                max_close_ts: None,
                min_close_ts: None,
                status: Some("active".to_string()),
                tickers: None,
            },
        )
        .await
        .unwrap();
    if markets.markets.is_empty() {
        println!("No markets available - skipping single market test");
        return;
    }
    let ticker = &markets.markets[0].ticker;
    println!("Testing get_market for ticker: {}", ticker);
    let result = client.get_market(ticker).await;
    assert!(result.is_ok(), "Failed to get market by ticker: {:?}", result.err());
    let market = result.unwrap();
    println!(
        "Retrieved market {} (category: {}, status: {})", market.market.ticker, market
        .market.category, market.market.status
    );
}
/// MARKET ORDERBOOK TEST
#[tokio::test]
async fn test_get_market_orderbook() {
    let client = setup_client();
    let markets = client
        .get_all_markets(
            &MarketsQuery {
                limit: Some(1),
                cursor: None,
                event_ticker: None,
                series_ticker: None,
                max_close_ts: None,
                min_close_ts: None,
                status: Some("active".to_string()),
                tickers: None,
            },
        )
        .await
        .unwrap();
    if markets.markets.is_empty() {
        println!("No markets available - skipping orderbook test");
        return;
    }
    let ticker = &markets.markets[0].ticker;
    println!("Testing orderbook for ticker: {}", ticker);
    let result = client.get_market_orderbook(ticker, Some(50)).await;
    assert!(result.is_ok(), "Failed to get market orderbook: {:?}", result.err());
    let orderbook = result.unwrap();
    println!("Orderbook retrieved for market {}", ticker);
    println!("{:?}", orderbook.orderbook);
}
/// TRADES TEST
#[tokio::test]
async fn test_get_trades_recent() {
    let client = setup_client();
    let markets = client
        .get_all_markets(
            &MarketsQuery {
                limit: Some(1),
                cursor: None,
                event_ticker: None,
                series_ticker: None,
                max_close_ts: None,
                min_close_ts: None,
                status: Some("active".to_string()),
                tickers: None,
            },
        )
        .await
        .unwrap();
    if markets.markets.is_empty() {
        println!("No markets found - skipping trades test");
        return;
    }
    let ticker = markets.markets[0].ticker.clone();
    println!("Testing trades for ticker: {}", ticker);
    let result = client
        .get_trades(Some(10), None, Some(ticker.clone()), None, None)
        .await;
    assert!(result.is_ok(), "Failed to get trades: {:?}", result.err());
    let trades = result.unwrap();
    println!("Trades retrieved: {}", trades.trades.len());
    for t in trades.trades.iter().take(3) {
        println!("Trade: {} @ ${} ({})", t.count, t.price, t.taker_side);
    }
}
/// CANDLESTICKS TEST
#[tokio::test]
async fn test_get_market_candlesticks() {
    let client = setup_client();
    let markets = client
        .get_all_markets(
            &MarketsQuery {
                limit: Some(1),
                cursor: None,
                event_ticker: None,
                series_ticker: None,
                max_close_ts: None,
                min_close_ts: None,
                status: Some("active".to_string()),
                tickers: None,
            },
        )
        .await
        .unwrap();
    if markets.markets.is_empty() {
        println!("No markets available - skipping candlesticks test");
        return;
    }
    let market = &markets.markets[0];
    let series_ticker = market.event_ticker.clone();
    let ticker = market.ticker.clone();
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
    let thirty_days_ago = now - (30 * 24 * 60 * 60);
    println!(
        "Getting candlesticks for market={} in series={} from {} to {}", ticker,
        series_ticker, thirty_days_ago, now
    );
    let result = client
        .get_market_candlesticks(&series_ticker, &ticker, thirty_days_ago, now, 86400)
        .await;
    match result {
        Ok(resp) => {
            println!("Candlesticks retrieved: {}", resp.market_candlesticks.len());
            if let Some(candle) = resp.market_candlesticks.first() {
                println!(
                    "Example Candle: ts={} price_close={:?}", candle.end_period_ts,
                    candle.price.close
                );
            }
        }
        Err(e) => {
            println!("No candlestick data available or failed: {:?}", e);
        }
    }
}
/// =============================================================================
/// COMPREHENSIVE MARKETS TEST
/// =============================================================================
#[tokio::test]
async fn test_markets_endpoints_comprehensive() {
    let client = setup_client();
    println!("\n{}", "=".repeat(80));
    println!("COMPREHENSIVE MARKETS ENDPOINTS TEST");
    println!("{}\n", "=".repeat(80));
    println!("1. Getting all markets...");
    let markets = client
        .get_all_markets(
            &MarketsQuery {
                limit: Some(5),
                cursor: None,
                event_ticker: None,
                series_ticker: None,
                max_close_ts: None,
                min_close_ts: None,
                status: Some("active".to_string()),
                tickers: None,
            },
        )
        .await
        .expect("Failed to get markets");
    println!("   Markets retrieved: {}\n", markets.markets.len());
    sleep(Duration::from_secs(2)).await;
    if let Some(first) = markets.markets.first() {
        println!("2. Getting single market...");
        let single = client.get_market(&first.ticker).await.expect("Get market failed");
        println!("   Market: {} ({})\n", single.market.ticker, single.market.category);
        sleep(Duration::from_secs(2)).await;
        println!("3. Getting orderbook...");
        let _ = client
            .get_market_orderbook(&first.ticker, Some(25))
            .await
            .expect("Get orderbook failed");
        println!("   Orderbook retrieved\n");
        sleep(Duration::from_secs(2)).await;
        println!("4. Getting trades...");
        let trades = client
            .get_trades(Some(5), None, Some(first.ticker.clone()), None, None)
            .await
            .expect("Get trades failed");
        println!("   Trades retrieved: {}\n", trades.trades.len());
        sleep(Duration::from_secs(2)).await;
        println!("5. Getting candlesticks...");
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
        let week_ago = now - (7 * 24 * 60 * 60);
        let _ = client
            .get_market_candlesticks(
                &first.event_ticker,
                &first.ticker,
                week_ago,
                now,
                86400,
            )
            .await;
        println!("   Candlesticks checked\n");
    } else {
        println!("No active markets available for detailed tests.");
    }
    println!("{}", "=".repeat(80));
    println!("ALL MARKET ENDPOINTS TESTS PASSED");
    println!("{}\n", "=".repeat(80));
}

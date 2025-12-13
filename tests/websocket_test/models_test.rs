use crate::common::setup_ws_client;
use kalshi_rs::websocket::models::*;

use super::constants::TEST_MARKET_TICKER;

#[tokio::test]
async fn test_orderbook_delta() {
    let client = setup_ws_client();
    client.connect().await.unwrap();
    // send subscription
    client
        .subscribe(vec!["orderbook_delta"], vec![TEST_MARKET_TICKER])
        .await
        .unwrap();
    // tracking if received all possible message types and how many irrelevant messages including Ping.
    let mut recv_snapshot = false;
    let mut recv_delta = false;
    let mut trys = 0;
    loop {
        // recieve next message
        let message = client.next_message().await.unwrap();
        // deser and check type
        match &message {
            KalshiSocketMessage::OrderbookSnapshot(_snap) => recv_snapshot = true,
            KalshiSocketMessage::OrderbookDelta(_delta) => recv_delta = true,
            _ => trys += 1,
        };
        // panic if waited 3 ping message
        if trys >= 3 {
            panic!("Ws sent 3 other messages before sending a SubscribedResponse");
        }
        // have received all expected messages
        if recv_delta && recv_snapshot {
            println!("{trys} {message:?}");
            return;
        }
    }
}

#[tokio::test]
async fn test_trade_update() {
    let client = setup_ws_client();
    client.connect().await.unwrap();
    // send subscription
    client
        .subscribe(vec!["trade"], vec![TEST_MARKET_TICKER])
        .await
        .unwrap();
    // tracking if received all possible message types and how many irrelevant messages including Ping.
    let mut recv_trade = false;
    let mut trys = 0;
    loop {
        // recieve next message
        let message = client.next_message().await.unwrap();
        // deser and check type
        match &message {
            KalshiSocketMessage::TradeUpdate(_trade) => recv_trade = true,
            _ => trys += 1,
        };
        // panic if waited 3 ping message
        if trys >= 7 {
            panic!("Ws sent 7 other messages before sending a SubscribedResponse");
        }
        // have received all expected messages
        if recv_trade {
            return;
        }
    }
}

#[tokio::test]
async fn test_ticker_update() {
    let client = setup_ws_client();
    client.connect().await.unwrap();
    // send subscription
    client
        .subscribe(vec!["ticker"], vec![TEST_MARKET_TICKER])
        .await
        .unwrap();
    // tracking if received all possible message types and how many irrelevant messages including Ping.
    let mut recv_tick = false;
    let mut trys = 0;
    loop {
        // recieve next message
        let message = client.next_message().await.unwrap();
        // deser and check type
        match &message {
            KalshiSocketMessage::TickerUpdate(_tick) => recv_tick = true,
            _ => trys += 1,
        };
        // panic if waited 3 ping message
        if trys >= 7 {
            panic!("Ws sent 7 other messages before sending a SubscribedResponse");
        }
        // have received all expected messages
        if recv_tick {
            return;
        }
    }
}

use crate::common::setup_ws_client;
use kalshi_rs::websocket::models::*;

use super::constants::CHANNELS;
use super::constants::TEST_ADD_MARKET_TICKER;
use super::constants::TEST_MARKET_TICKER;

#[tokio::test]
async fn test_subscribe() {
    let client = setup_ws_client();
    client.connect().await.unwrap();
    // send subscription
    client
        .subscribe(vec![CHANNELS[2]], vec![TEST_MARKET_TICKER])
        .await
        .unwrap();
    // wait for a SubscribedResponse
    let mut trys = 0;
    loop {
        // recieve next message
        let message = client.next_message().await.unwrap();
        // deser and check type
        match message {
            KalshiSocketMessage::SubscribedResponse(_) => return,
            _ => trys += 1,
        };
        // panic if waited 3 ping message
        if trys >= 3 {
            panic!("Ws sent 3 other messages before sending a SubscribedResponse");
        }
    }
}

#[tokio::test]
async fn test_unsubscribe() {
    let client = setup_ws_client();
    client.connect().await.unwrap();
    // send subscription
    client
        .subscribe(vec![CHANNELS[2]], vec![TEST_MARKET_TICKER])
        .await
        .unwrap();
    // wait for a message
    client.next_message().await.unwrap();
    // send unsubscribe
    client.unsubscribe(vec![1]).await.unwrap();
    // wait for an UnsubscribedResponse
    let mut trys = 0;
    loop {
        // recieve next message
        let message = client.next_message().await.unwrap();
        // deser and check type
        match message {
            KalshiSocketMessage::UnsubscribedResponse(_) => return,
            _ => trys += 1,
        };
        // panic if waited 3 unrelated message
        if trys >= 5 {
            panic!("Ws sent 5 pings before sending a SubscribedResponse");
        }
    }
}

// NOTE: this is not a great test. problem is that both OkResponse and ListSubscriptionsResponse
// have type field equal to "ok"
#[tokio::test]
async fn test_list_subscriptions() {
    let client = setup_ws_client();
    client.connect().await.unwrap();
    client.list_subscriptions().await.unwrap();
    // wait for a ListSubs...
    let mut trys = 0;
    loop {
        // recieve next message
        let message = client.next_message().await.unwrap();
        // deser and check type. because of ambiguous "type" field in response
        // list subscripotions response is unparseable
        match message {
            KalshiSocketMessage::Unparseable(_) => return,
            _ => trys += 1,
        };
        // panic if waited 5 ping message
        if trys >= 5 {
            panic!("Ws sent 5 other messages before sending an UnparseableResponse");
        }
    }
}

#[tokio::test]
async fn test_add_markets() {
    let client = setup_ws_client();
    client.connect().await.unwrap();
    client
        .subscribe(vec![CHANNELS[2]], vec![TEST_MARKET_TICKER])
        .await
        .unwrap();
    client
        .add_markets(vec![1], vec![TEST_ADD_MARKET_TICKER])
        .await
        .unwrap();
    // wait for an OkResponse indicatring successful operation
    let mut trys = 0;
    loop {
        // recieve next message
        let message = client.next_message().await.unwrap();
        match message {
            KalshiSocketMessage::OkResponse(_) => return,
            _ => trys += 1,
        };
        // panic if waited 5 ping message
        if trys >= 5 {
            panic!("Ws sent 5 other messages before sending an OkResponse");
        }
    }
}

#[tokio::test]
async fn test_del_markets() {
    let client = setup_ws_client();
    client.connect().await.unwrap();
    client
        .subscribe(vec![CHANNELS[2]], vec![TEST_MARKET_TICKER])
        .await
        .unwrap();
    client
        .add_markets(vec![1], vec![TEST_ADD_MARKET_TICKER])
        .await
        .unwrap();
    client
        .del_markets(vec![1], vec![TEST_ADD_MARKET_TICKER])
        .await
        .unwrap();
    // wait for an OkResponse indicatring successful operation
    let mut trys = 0;
    let mut ok_messages = 0;
    loop {
        // recieve next message
        let message = client.next_message().await.unwrap();
        match message {
            KalshiSocketMessage::OkResponse(_) => ok_messages += 1,
            _ => trys += 1,
        };
        // panic if waited 5 ping message
        if trys >= 5 {
            panic!("Ws sent 5 other messages before sending 2 OkResponse's response");
        }
        // if we have successfully added and deleted market
        // then we should get 2 OkResponse's
        if ok_messages == 2 {
            return;
        }
    }
}

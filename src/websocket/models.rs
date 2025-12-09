use serde::{Deserialize, Serialize};


// Websocket subscription responses
#[derive(Serialize, Deserialize, Debug)]
pub struct SubscribedResponse {
    pub r#type: String,
    pub id: u64,
    pub msg: SubscribedResponseMessage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubscribedResponseMessage {
    pub channel: String,
    pub sid: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub id: u64,
    pub code: u64,
    pub msg: ErrorResponseMessage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponseMessage {
    pub code: u64,
    pub msg: String,
}

// Orderbook update channel
#[derive(Serialize, Deserialize, Debug)]
pub struct OrderbookSnapshot {
    pub r#type: String,
    pub sid: u64,
    pub seq: u64,
    pub msg: OrderbookSnapshotMessage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderbookSnapshotMessage {
    pub market_ticker: String,
    pub market_id: String,
    pub yes: Option<Vec<(u8, u64)>>,
    pub no: Option<Vec<(u8, u64)>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderbookDelta {
    pub r#type: String,
    pub sid: u64,
    pub seq: u64,
    pub msg: OrderbookDeltaMessage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderbookDeltaMessage {
    pub market_ticker: String,
    pub market_id: String,
    pub price: u8,
    pub delta: i64,
    pub side: String,
    pub ts: String,
}

// Trades dhannel
#[derive(Serialize, Deserialize, Debug)]
pub struct TradeUpdate {
    pub r#type: String,
    pub sid: u64,
    pub seq: u64,
    pub msg: TradeUpdateMessage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TradeUpdateMessage {
    pub trade_id: String,
    pub market_ticker: String,
    pub yes_price: u8,
    pub no_price: u8,
    pub count: u64,
    pub taker_side: String,
    pub ts: u64,
}
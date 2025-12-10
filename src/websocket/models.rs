use serde::{Deserialize, Serialize};
use tokio_tungstenite::tungstenite;


pub enum KalshiSocketMessage {
    // Textual messages
    SubscribedResponse(SubscribedResponse), // response to a sent message indicating success
    ErrorResponse(ErrorResponse),           // response to a sent message indicating failure
    OrderbookSnapshot(OrderbookSnapshot),   // snapshot of orderbook, first message from a orderbook_delta subscription
    OrderbookDelta(OrderbookDelta),         // orderbook change
    TradeUpdate(TradeUpdate),               // trade executed between two parties
    TickerUpdate(TickerUpdate),             // tick update on market
    UserFill(UserFill),                     // user order fill update
    MarketPosition(MarketPosition),         // market position update
    // Heartbeat
    Ping,
    Pong,
    // Unexpected types from Kalshi API
    Binary(tungstenite::Bytes),
    Frame(tungstenite::protocol::frame::Frame),
    Close(Option<tungstenite::protocol::frame::CloseFrame>),
}

// Websocket subscription responses
#[derive(Serialize, Deserialize, Debug)]
pub struct SubscribedResponse {
    pub r#type: String,
    pub id: i64,
    pub msg: SubscribedResponseMessage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubscribedResponseMessage {
    pub channel: String,
    pub sid: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnsubscribedResponse {
    pub r#type: String,
    pub sid: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OkResponse {
    pub r#type: String,
    pub id: i64,
    pub sid: i64,
    pub seq: i64,
    pub market_tickers: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub r#type: String,
    pub id: i64,
    pub code: i64,
    pub msg: ErrorResponseMessage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponseMessage {
    pub code: i64,
    pub msg: String,
}

// Orderbook update channel
#[derive(Serialize, Deserialize, Debug)]
pub struct OrderbookSnapshot {
    pub r#type: String,
    pub sid: i64,
    pub seq: i64,
    pub msg: OrderbookSnapshotMessage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderbookSnapshotMessage {
    pub market_ticker: String,
    pub market_id: String,
    pub yes: Option<Vec<(u8, i64)>>,
    pub yes_dollars: Option<Vec<(String, i64)>>,
    pub no: Option<Vec<(u8, i64)>>,
    pub no_dollars: Option<Vec<(String, i64)>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderbookDelta {
    pub r#type: String,
    pub sid: i64,
    pub seq: i64,
    pub msg: OrderbookDeltaMessage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderbookDeltaMessage {
    pub market_ticker: String,
    pub market_id: String,
    pub price: u8,
    pub price_dollars: String,
    pub delta: i64,
    pub side: String,
    pub ts: String,
}

// Public trades channel
#[derive(Serialize, Deserialize, Debug)]
pub struct TradeUpdate {
    pub r#type: String,
    pub sid: i64,
    pub seq: i64,
    pub msg: TradeUpdateMessage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TradeUpdateMessage {
    pub trade_id: String,
    pub market_ticker: String,
    pub yes_price: u8,
    pub yes_price_dollars: String,
    pub no_price: u8,
    pub no_price_dollars: String,
    pub count: i64,
    pub taker_side: String,
    pub ts: i64,
}

// Ticker updates channel
#[derive(Serialize, Deserialize, Debug)]
pub struct TickerUpdate {
    pub r#type: String,
    pub sid: i64,
    pub msg: TickerUpdateMessage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TickerUpdateMessage {
    pub market_ticker: String,
    pub price: u8,
    pub yes_bid: u8,
    pub yes_ask: u8,
    pub price_dollars: String,
    pub yes_bid_dollars: String,
    pub no_bid_dollars: String,
    pub volume: i64,
    pub open_interst: i64,
    pub dollar_volume: i64,
    pub dollar_open_interest: i64,
    pub ts: i64,
    pub Clock: i64, // idk why api makes this upper case 
}

// User order fills channel
#[derive(Serialize, Deserialize, Debug)]
pub struct UserFill {
    pub r#type: String,
    pub sid: i64,
    pub msg: UserFillMessage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserFillMessage {
    pub trade_id: String,
    pub order_id: String,
    pub market_ticker: String,
    pub is_taker: bool,
    pub side: String,
    pub yes_price: u8,
    pub yes_price_dollars: String,
    pub count: i64,
    pub action: String,
    pub ts: i64,
    pub client_order_id: String,
    pub post_position: i64,
    pub purchased_side: String,
}

// Market position updates channel
#[derive(Serialize, Deserialize, Debug)]
pub struct MarketPosition {
    pub r#type: String,
    pub sid: i64,
    pub msg: UserFillMessage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MarketPositionMessage {
    pub user_id: String,
    pub market_ticker: String,
    pub position: i64,
    pub position_cost: i64,
    pub realized_pnl: i64,
    pub fees_paid: i64,
    pub volume: i64,
}


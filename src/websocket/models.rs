use serde::Deserialize;
use tokio_tungstenite::tungstenite;

use crate::errors::KalshiError;

/// Tagged enum for deserializing text websocket messages based on the "type" field
#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TextSocketMessage {
    Subscribed(SubscribedResponse),
    Unsubscribed(UnsubscribedResponse),
    Ok(OkResponse),
    Error(ErrorResponse),
    OrderbookSnapshot(OrderbookSnapshot),
    OrderbookDelta(OrderbookDelta),
    Trade(TradeUpdate),
    Ticker(TickerUpdate),
    Fill(UserFill),
    MarketPosition(MarketPosition),
}

#[derive(Debug)]
pub enum KalshiSocketMessage {
    // == TEXTUAL MESSAGES ==
    // response to a sent subscribed message indicating success
    SubscribedResponse(SubscribedResponse),
    // response to a sent unsubscribe message indicating success
    UnsubscribedResponse(UnsubscribedResponse),
    // response to a sent message indicating failure
    OkResponse(OkResponse),
    // response to a sent message indicating failure
    ErrorResponse(ErrorResponse),
    // snapshot of orderbook, first message from a orderbook_delta subscription
    OrderbookSnapshot(OrderbookSnapshot),
    // orderbook change
    OrderbookDelta(OrderbookDelta),
    // trade executed between two parties
    TradeUpdate(TradeUpdate),
    // tick update on market
    TickerUpdate(TickerUpdate),
    // user order fill update
    UserFill(UserFill),
    // market position update
    MarketPosition(MarketPosition),
    // fallback type in case not able to parse output correctly
    Unparseable(String),
    // == HEARTBEAT TYPES ==
    Ping,
    Pong,
    // == UNEXPECTED TYPES FROM KALSHI's API ==
    Binary(tungstenite::Bytes),
    Frame(tungstenite::protocol::frame::Frame),
    Close(Option<tungstenite::protocol::frame::CloseFrame>),
}

impl TryFrom<tungstenite::Message> for KalshiSocketMessage {
    type Error = KalshiError;
    fn try_from(msg: tungstenite::Message) -> Result<KalshiSocketMessage, Self::Error> {
        match msg {
            tungstenite::Message::Text(text) => Self::from_textual_message(text.to_string()),
            tungstenite::Message::Ping(_) => Ok(Self::Ping),
            tungstenite::Message::Pong(_) => Ok(Self::Pong),
            tungstenite::Message::Binary(b) => Ok(Self::Binary(b)),
            tungstenite::Message::Close(c) => Ok(Self::Close(c)),
            tungstenite::Message::Frame(f) => Ok(Self::Frame(f)),
        }
    }
}

impl From<TextSocketMessage> for KalshiSocketMessage {
    fn from(msg: TextSocketMessage) -> Self {
        match msg {
            TextSocketMessage::Subscribed(inner) => KalshiSocketMessage::SubscribedResponse(inner),
            TextSocketMessage::Unsubscribed(inner) => KalshiSocketMessage::UnsubscribedResponse(inner),
            TextSocketMessage::Ok(inner) => KalshiSocketMessage::OkResponse(inner),
            TextSocketMessage::Error(inner) => KalshiSocketMessage::ErrorResponse(inner),
            TextSocketMessage::OrderbookSnapshot(inner) => KalshiSocketMessage::OrderbookSnapshot(inner),
            TextSocketMessage::OrderbookDelta(inner) => KalshiSocketMessage::OrderbookDelta(inner),
            TextSocketMessage::Trade(inner) => KalshiSocketMessage::TradeUpdate(inner),
            TextSocketMessage::Ticker(inner) => KalshiSocketMessage::TickerUpdate(inner),
            TextSocketMessage::Fill(inner) => KalshiSocketMessage::UserFill(inner),
            TextSocketMessage::MarketPosition(inner) => KalshiSocketMessage::MarketPosition(inner),
        }
    }
}

impl KalshiSocketMessage {
    pub fn from_textual_message(s: String) -> Result<KalshiSocketMessage, KalshiError> {
        match serde_json::from_str::<TextSocketMessage>(&s) {
            Ok(msg) => Ok(msg.into()),
            Err(_) => Ok(KalshiSocketMessage::Unparseable(s)),
        }
    }
}

// Websocket subscription responses
#[derive(Deserialize, Debug)]
pub struct SubscribedResponse {
    pub id: i64,
    pub msg: SubscribedResponseMessage,
}

#[derive(Deserialize, Debug)]
pub struct SubscribedResponseMessage {
    pub channel: String,
    pub sid: i64,
}

#[derive(Deserialize, Debug)]
pub struct UnsubscribedResponse {
    pub sid: i64,
}

#[derive(Deserialize, Debug)]
pub struct OkResponse {
    pub id: i64,
    pub sid: i64,
    pub msg: OkResponseMessage,
}

#[derive(Deserialize, Debug)]
pub struct OkResponseMessage {
    pub market_tickers: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct ErrorResponse {
    pub id: i64,
    pub msg: ErrorResponseMessage,
}

#[derive(Deserialize, Debug)]
pub struct ErrorResponseMessage {
    pub code: i64,
    pub msg: String,
}

// Orderbook update channel
#[derive(Deserialize, Debug)]
pub struct OrderbookSnapshot {
    pub sid: i64,
    pub seq: i64,
    pub msg: OrderbookSnapshotMessage,
}

#[derive(Deserialize, Debug)]
pub struct OrderbookSnapshotMessage {
    pub market_ticker: String,
    pub market_id: String,
    pub yes: Option<Vec<(u8, i64)>>,
    pub yes_dollars: Option<Vec<(String, i64)>>,
    pub no: Option<Vec<(u8, i64)>>,
    pub no_dollars: Option<Vec<(String, i64)>>,
}

#[derive(Deserialize, Debug)]
pub struct OrderbookDelta {
    pub sid: i64,
    pub seq: i64,
    pub msg: OrderbookDeltaMessage,
}

#[derive(Deserialize, Debug)]
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
#[derive(Deserialize, Debug)]
pub struct TradeUpdate {
    pub sid: i64,
    pub seq: i64,
    pub msg: TradeUpdateMessage,
}

#[derive(Deserialize, Debug)]
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
#[derive(Deserialize, Debug)]
pub struct TickerUpdate {
    pub sid: i64,
    pub msg: TickerUpdateMessage,
}

#[allow(nonstandard_style)]
#[derive(Deserialize, Debug)]
pub struct TickerUpdateMessage {
    pub market_ticker: String,
    pub price: u8,
    pub yes_bid: u8,
    pub yes_ask: u8,
    pub price_dollars: String,
    pub yes_bid_dollars: String,
    pub yes_ask_dollars: String,
    pub volume: i64,
    pub open_interest: i64,
    pub dollar_volume: i64,
    pub dollar_open_interest: i64,
    pub ts: i64,
    pub Clock: i64, // idk why api makes this upper case
}

// User order fills channel
#[derive(Deserialize, Debug)]
pub struct UserFill {
    pub sid: i64,
    pub msg: UserFillMessage,
}

#[derive(Deserialize, Debug)]
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
#[derive(Deserialize, Debug)]
pub struct MarketPosition {
    pub sid: i64,
    pub msg: UserFillMessage,
}

#[derive(Deserialize, Debug)]
pub struct MarketPositionMessage {
    pub user_id: String,
    pub market_ticker: String,
    pub position: i64,
    pub position_cost: i64,
    pub realized_pnl: i64,
    pub fees_paid: i64,
    pub volume: i64,
}

use serde::{Serialize, Deserialize};
use derive_more::Display;



#[derive(Serialize, Debug, Clone)]
pub struct AmendOrderRequest {
    pub ticker: String,
    pub side: String, // "yes" | "no"
    pub action: String, // "buy" | "sell"
    pub client_order_id: String,
    pub updated_client_order_id: String,
    pub yes_price: Option<u64>,
    pub no_price: Option<u64>,
    pub yes_price_dollars: Option<String>,
    pub no_price_dollars: Option<String>,
    pub count: Option<u64>,
}

#[derive(Deserialize, Display, Debug, Clone)]
#[display("AmendOrderResponse {{ old_order: {old_order:?}, order: {order:?} }}")]
pub struct AmendOrderResponse {
    pub old_order: Order,
    pub order: Order,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Order {
    pub order_id: String,
    pub user_id: String,
    pub client_order_id: String,
    pub ticker: String,
    pub side: String,
    pub action: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub status: String,
    pub yes_price: Option<u64>,
    pub no_price: Option<u64>,
    pub yes_price_dollars: Option<String>,
    pub no_price_dollars: Option<String>,
    pub fill_count: Option<u64>,
    pub remaining_count: Option<u64>,
    pub initial_count: Option<u64>,
    pub taker_fees: Option<u64>,
    pub maker_fees: Option<u64>,
    pub taker_fill_cost: Option<u64>,
    pub maker_fill_cost: Option<u64>,
    pub taker_fill_cost_dollars: Option<String>,
    pub maker_fill_cost_dollars: Option<String>,
    pub queue_position: Option<u64>,
    pub taker_fees_dollars: Option<String>,
    pub maker_fees_dollars: Option<String>,
    pub expiration_time: Option<String>,
    pub created_time: Option<String>,
    pub last_update_time: Option<String>,
    pub self_trade_prevention_type: Option<String>,
    pub order_group_id: Option<String>,
    pub cancel_order_on_pause: Option<bool>,
    pub order_error: Option<OrderError>
}


//option if there was an error in the response for the orders
#[derive(serde::Deserialize, Debug, Clone, Serialize)]
pub struct OrderError {
    pub code: Option<String>,
    pub message: Option<String>,
    pub details: Option<String>,
    pub service: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct BatchCancelOrdersResponse{
    pub orders: Vec<Order>,


}

#[derive(serde::Serialize,serde::Deserialize)]

pub struct BatchCancelOrdersRequest{
    pub order_ids: Vec<String>,


}

#[derive(serde::Deserialize)]
pub struct BatchCreateOrdersResponse{
    pub orders: Vec<String>,
}

#[derive(serde::Serialize)]
pub struct BatchCreateOrdersRequest{
    orders: Vec<Order>
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct CancelOrderResponse {
    pub order: Order,
    pub reduced_by: Option<u64>,
}
// TODO need to build out macros for anything that has options in them  
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateOrderRequest {
    pub ticker: String,
    pub side: String, // "yes" | "no"
    pub action: String, // "buy" | "sell"
    pub count: u64,

    // Optional fields
    pub client_order_id: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>, // "limit" | "market"
    pub yes_price: Option<u64>,
    pub no_price: Option<u64>,
    pub yes_price_dollars: Option<String>,
    pub no_price_dollars: Option<String>,
    pub expiration_ts: Option<u64>,
    pub time_in_force: Option<String>, // "fill_or_kill" | "good_till_canceled" | "immediate_or_cancel"
    pub buy_max_cost: Option<u64>,
    pub post_only: Option<bool>,
    pub reduce_only: Option<bool>,
    pub self_trade_prevention_type: Option<String>, // "taker_at_cross" | "maker"
    pub order_group_id: Option<String>,
    pub cancel_order_on_pause: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateOrderResponse {
    pub order: Order,
}


#[derive(serde::Serialize)]
pub struct CreateOrderGroupRequest {
    pub contracts_limit: u64,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct CreateOrderGroupResponse {
    pub order_group_id: String,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct DecreaseOrderResponse{
    pub order: Order

}

#[derive(serde::Serialize,serde::Deserialize, Debug, Clone)]
pub struct DecreaseOrderRequest{
    pub reduce_by:u64, //>=1
    pub reduce_to: u64 //>=0

}
#[derive(serde::Serialize,serde::Deserialize, Debug, Clone)]
pub struct GetBalanceResponse{
    pub balance:u64,
    pub portfolio_value:u64,
    pub updated_ts:String,
}

#[derive(serde::Serialize, Default, Debug, Clone)]
pub struct GetFillsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ts: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_ts: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>, // default: 100, range: 1-200
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Fill {
    pub fill_id: String,
    pub trade_id: String,
    pub order_id: String,
    pub client_order_id: String,
    pub ticker: String,
    pub market_ticker: String,
    pub side: String, // "yes" | "no"
    pub action: String, // "buy" | "sell"
    pub count: u64,
    pub price: u64,
    pub yes_price: u64,
    pub no_price: u64,
    pub yes_price_fixed: String,
    pub no_price_fixed: String,
    pub is_taker: bool,
    pub created_time: String, // ISO 8601 format: "2023-11-07T05:31:56Z"
    pub ts: u64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct GetFillsResponse {
    pub fills: Vec<Fill>,
    pub cursor: String,
}

#[derive(serde::Serialize,serde::Deserialize, Debug, Clone)]
pub struct GetOrderResponse{
    pub order:Order,
}

  #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
  pub struct GetOrderGroupResponse {
    pub is_auto_cancel_enabled: bool,
    pub orders: Vec<String>
  }


  // Order group summary (for get_order_groups)
  #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
  pub struct OrderGroup{
    pub id: String,
    pub is_auto_cancel_enabled: bool
  }

  #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
  pub struct GetOrderGroupsResponse {
    pub order_groups: Vec<OrderGroup>,
    pub cursor: Option<String>  // Don't forget the cursor!
  }

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct GetOrderQueuePositionResponse{
    pub queue_position: u64,
}

#[derive(serde::Serialize, Default, Debug, Clone)]
pub struct GetOrdersParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ts: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_ts: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct GetOrdersResponse{
    pub orders: Vec<Order>,
    pub cursor: Option<String>
}

#[derive(serde::Serialize, Default, Debug, Clone)]
pub struct GetPositionsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>, // default: 100, range: 1-1000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_filter: Option<String>, // comma-separated: "position", "total_traded"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_status: Option<String>, // "all" | "unsettled" | "settled", default: "unsettled"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_ticker: Option<String>, // comma-separated list (max 10)
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct MarketPosition {
    pub market_ticker: String,
    pub position: i64,
    pub market_exposure: i64,
    pub realized_pnl: i64,
    pub fees_paid: u64, // Fees are always positive costs
    pub resting_order_count: u64,
    pub total_traded: u64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct EventPosition {
    pub event_ticker: String,
    pub position: i64,
    pub event_exposure: i64,
    pub realized_pnl: i64,
    pub fees_paid: u64, // Fees are always positive costs
    pub resting_order_count: u64,
    pub total_traded: u64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct GetPositionsResponse {
    pub cursor: Option<String>,
    pub market_positions: Vec<MarketPosition>,
    pub event_positions: Vec<EventPosition>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]

pub struct GetQueuePositionsResponse{
    queue_positions: Vec<QueuePositionObj>

}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct QueuePositionObj{
    order_id: String, 
    market_ticker: String, 
    queue_position:u64
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]

pub struct GetQueueParams{
    pub market_tickers: Option<String>,
    pub event_ticker: Option<String>
}
#[derive(serde::Serialize, Default, Debug, Clone)]
pub struct GetSettlementsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>, // default: 100, range: 1-200
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_ticker: Option<String>, // comma-separated list (max 10)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ts: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_ts: Option<u64>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Settlement {
    pub ticker: String,
    pub market_result: String, // "yes" | "no"
    pub yes_count: u64,
    pub yes_total_cost: u64,
    pub no_count: u64,
    pub no_total_cost: u64,
    pub revenue: i64, // Can be negative for losses
    pub settled_time: String, // ISO 8601 format: "2023-11-07T05:31:56Z"
    pub fee_cost: String, // Decimal string like "0.3400"
    pub value: u64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct GetSettlementsResponse {
    pub settlements: Vec<Settlement>,
    pub cursor: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct GetTotalRestingOrderValueResponse{
    total_resting_order_value: u64,//value in cents
}

#[derive(serde::Serialize,serde::Deserialize, Debug, Clone)]
pub struct DeleteOrderGroupResponse{
    pub body: Option<String>,
}

pub struct ResetOrderGroupResponse{
    
}
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

pub struct GetFillsResponse{

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
pub struct GetOrderQueuePositionResponse{

}

pub struct GetOrdersResponse{

}

pub struct GetPositionsResponse{

}

pub struct GetQueuePositionsResponse{

}

pub struct GetSettlementsResponse{

}

pub struct GetTotalRestingOrderValueResponse{

}

#[derive(serde::Serialize,serde::Deserialize, Debug, Clone)]
pub struct DeleteOrderGroupResponse{
    pub body: Option<String>,
}

pub struct ResetOrderGroupResponse{
    
}
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

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct CancelOrderResponse {
    pub order: Order,
    pub reduced_by: Option<u64>,
}

pub struct CreateOrderResponse{

}

pub struct CreateOrderGroupResponse{

}

pub struct DecreaseOrderResponse{

}

pub struct GetBalanceResponse{
    
}

pub struct GetFillsResponse{

}

pub struct GetOrderResponse{

}

pub struct GetOrderGroupResponse{

}

pub struct GetOrderGroupsResponse{

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

pub struct DeleteOrderGroupResponse{

}

pub struct ResetOrderGroupResponse{
    
}
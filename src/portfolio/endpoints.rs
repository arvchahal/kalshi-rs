use crate::client::KalshiClient;
use crate::errors::KalshiError;
use crate::helpers::build_url_with_query;
use crate::portfolio::models::{
    AmendOrderResponse, BatchCancelOrdersResponse, BatchCreateOrdersResponse, 
    CancelOrderResponse, CreateOrderResponse, CreateOrderGroupResponse, 
    DecreaseOrderResponse, DeleteOrderGroupResponse, GetBalanceResponse, 
    GetFillsResponse, GetOrderResponse, GetOrderGroupResponse, 
    GetOrderGroupsResponse, GetOrderQueuePositionResponse, GetOrdersResponse, 
    GetPositionsResponse, GetQueuePositionsResponse, GetSettlementsResponse, 
    GetTotalRestingOrderValueResponse, ResetOrderGroupResponse, BatchCancelOrdersRequest, BatchCreateOrdersRequest,AmendOrderRequest
};
const AMEND_ORDER: &str = "/trade-api/v2/portfolio/orders//amend"; // Post
const BATCH_CANCEL_ORDERS: &str = "/trade-api/v2/portfolio/orders/batched"; // Delete
const BATCH_CREATE_ORDERS: &str = "/trade-api/v2/portfolio/orders/batched"; // Post
const CANCEL_ORDER: &str = "/trade-api/v2/portfolio/orders/"; // Delete
const CREATE_ORDER: &str = "/trade-api/v2/portfolio/orders"; // Post
const CREATE_ORDER_GROUP: &str = "/trade-api/v2/portfolio/order_groups/create"; // Post
const DECREASE_ORDER: &str = "/trade-api/v2/portfolio/orders//decrease"; // Post
const DELETE_ORDER_GROUP: &str = "/trade-api/v2/portfolio/order_groups/"; // Delete
const GET_BALANCE: &str = "/trade-api/v2/portfolio/balance"; // Get
const GET_FILLS: &str = "/trade-api/v2/portfolio/fills"; // Get
const GET_ORDER: &str = "/trade-api/v2/portfolio/orders/"; // Get
const GET_ORDER_GROUP: &str = "/trade-api/v2/portfolio/order_groups/"; // Get
const GET_ORDER_GROUPS: &str = "/trade-api/v2/portfolio/order_groups"; // Get
const GET_ORDER_QUEUE_POSITION: &str = "/trade-api/v2/portfolio/orders//queue_position"; // Get
const GET_ORDERS: &str = "/trade-api/v2/portfolio/orders"; // Get
const GET_POSITIONS: &str = "/trade-api/v2/portfolio/positions"; // Get
const GET_QUEUE_POSITIONS: &str = "/trade-api/v2/portfolio/orders/queue_positions"; // Post
const GET_SETTLEMENTS: &str = "/trade-api/v2/portfolio/settlements"; // Get
const GET_TOTAL_RESTING_ORDER_VALUE: &str = "/trade-api/v2/portfolio/summary/total_resting_order_value"; // Get
const RESET_ORDER_GROUP: &str = "/trade-api/v2/portfolio/order_groups//reset"; // Put

//need to test
impl KalshiClient{
    pub async fn amend_order(
        &self,
        order_id: &str,
        body: &AmendOrderRequest,
    ) -> Result<AmendOrderResponse, KalshiError> {
        let url = AMEND_ORDER.replace("{}", order_id);
        let json_body = serde_json::to_string(body).map_err(|e| {
            KalshiError::Other(format!("Failed to serialize request body: {}", e))
        })?;

        let resp = self.authenticated_post(&url, Some(&json_body)).await?;
        let data: AmendOrderResponse = serde_json::from_str(&resp).map_err(|e| {
            KalshiError::Other(format!("Parse error: {e}. Response: {resp}"))
        })?;
        Ok(data)
    }
    pub async fn batch_cancel_orders(&self,body:&BatchCancelOrdersRequest)-> Result<BatchCancelOrdersResponse,KalshiError>{
            let json_body = serde_json::to_string(body).map_err(|e| {
                KalshiError::Other(format!("Failed to serialize request body: {}", e))
            })?;
            let (status, resp) = self.authenticated_delete(BATCH_CANCEL_ORDERS, Some(&json_body)).await?;
            let data: BatchCancelOrdersResponse = serde_json::from_str(&resp).map_err(|e| {
                KalshiError::Other(format!("Parse error: {e}. Response: {resp}, status{status}"))
            })?;
            Ok(data)

    }

    pub async fn batch_create_orders(&self,body:&BatchCreateOrdersRequest)-> Result<BatchCreateOrdersResponse,KalshiError>{
            let json_body = serde_json::to_string(body).map_err(|e| {
                KalshiError::Other(format!("Failed to serialize request body: {}", e))
            })?;
            let resp = self.authenticated_post(BATCH_CREATE_ORDERS, Some(&json_body)).await?;
            let data: BatchCreateOrdersResponse = serde_json::from_str(&resp).map_err(|e| {
                KalshiError::Other(format!("Parse error: {e}. Response: {resp}"))
            })?;
            Ok(data)
    }
    pub async fn cancel_order(&self, order_id:String)-> Result<CancelOrderResponse, KalshiError>{
        let url:&str = &CANCEL_ORDER.replace("{}",&order_id);
        let (status,resp) = self.authenticated_delete::<str>(url,None).await?;
        let data: CancelOrderResponse = serde_json::from_str(&resp).map_err(|e| {
            KalshiError::Other(format!("Parse error: {e}. Response: {resp}"))})?;
        Ok(data)
    }

    pub async fn create_order(){

    }

    pub async fn create_order_group(){

    }
    pub async fn decrease_order(){

    }

    pub async fn delete_order_group(){

    }
    pub async fn get_balance(){

    }
    pub async fn get_fills(){

    }
    pub async fn get_order(){

    }
    pub async fn get_order_group(){

    }
    pub async fn get_order_groups(){

    }
    pub async fn get_order_queue_position(){
        
    }


    pub async fn get_orders(){
        
    }

    pub async fn get_positions(){

    }
    pub async fn get_queue_positions(){

    }

    pub async fn get_settlements(){

    }
    pub async fn get_total_resting_order_value(){

    }

    pub async fn reset_order_group(){

    }



}
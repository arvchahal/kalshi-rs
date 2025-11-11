use crate::client::KalshiClient;
use crate::errors::KalshiError;
use crate::helpers::build_url_with_query;
use crate::portfolio::models::{
    AmendOrderResponse, BatchCancelOrdersResponse, BatchCreateOrdersResponse,
    CancelOrderResponse, CreateOrderResponse, CreateOrderGroupResponse,
    DecreaseOrderResponse, DeleteOrderGroupResponse, GetBalanceResponse,
    GetFillsResponse, GetOrderResponse, GetOrderGroupResponse,
    GetOrderGroupsResponse, GetOrderQueuePositionResponse, GetOrdersResponse,
    GetOrdersParams, GetPositionsResponse, GetQueuePositionsResponse, GetSettlementsResponse,
    GetTotalRestingOrderValueResponse, ResetOrderGroupResponse, BatchCancelOrdersRequest,
    BatchCreateOrdersRequest,AmendOrderRequest,CreateOrderRequest, CreateOrderGroupRequest,
    DecreaseOrderRequest,
};
const AMEND_ORDER: &str = "/trade-api/v2/portfolio/orders//amend"; // Post
const BATCH_CANCEL_ORDERS: &str = "/trade-api/v2/portfolio/orders/batched"; // Delete
const BATCH_CREATE_ORDERS: &str = "/trade-api/v2/portfolio/orders/batched"; // Post
const CANCEL_ORDER: &str = "/trade-api/v2/portfolio/orders/"; // Delete
const CREATE_ORDER: &str = "/trade-api/v2/portfolio/orders"; // Post
const CREATE_ORDER_GROUP: &str = "/trade-api/v2/portfolio/order_groups/create"; // Post
const DECREASE_ORDER: &str = "/trade-api/v2/portfolio/orders/{}/decrease"; // Post
const DELETE_ORDER_GROUP: &str = "/trade-api/v2/portfolio/order_groups/{}"; // Delete
const GET_BALANCE: &str = "/trade-api/v2/portfolio/balance"; // Get
const GET_FILLS: &str = "/trade-api/v2/portfolio/fills"; // Get
const GET_ORDER: &str = "/trade-api/v2/portfolio/orders/"; // Get
const GET_ORDER_GROUP: &str = "/trade-api/v2/portfolio/order_groups/{}"; // Get
const GET_ORDER_GROUPS: &str = "/trade-api/v2/portfolio/order_groups"; // Get
const GET_ORDER_QUEUE_POSITION: &str = "/trade-api/v2/portfolio/orders/{}/queue_position"; // Get
const GET_ORDERS: &str = "/trade-api/v2/portfolio/orders"; // Get
const GET_POSITIONS: &str = "/trade-api/v2/portfolio/positions"; // Get
const GET_QUEUE_POSITIONS: &str = "/trade-api/v2/portfolio/orders/queue_positions"; // Post
const GET_SETTLEMENTS: &str = "/trade-api/v2/portfolio/settlements"; // Get
const GET_TOTAL_RESTING_ORDER_VALUE: &str = "/trade-api/v2/portfolio/summary/total_resting_order_value"; // Get
const RESET_ORDER_GROUP: &str = "/trade-api/v2/portfolio/order_groups//reset"; // Put

//need to test

// TODO need to build out macros for anything that has options in them  

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

    // TODO refactor need to see if we need status or can just keep as _
    pub async fn cancel_order(&self, order_id:String)-> Result<CancelOrderResponse, KalshiError>{
        let url:&str = &CANCEL_ORDER.replace("{}",&order_id);
        let (_,resp) = self.authenticated_delete::<str>(url,None).await?;
        let data: CancelOrderResponse = serde_json::from_str(&resp).map_err(|e| {
            KalshiError::Other(format!("Parse error: {e}. Response: {resp}"))})?;
        Ok(data)
    }

    pub async fn create_order(&self, body:&CreateOrderRequest)-> Result<CreateOrderResponse, KalshiError>{
        let json_body = serde_json::to_string(body).map_err(|e| {
                KalshiError::Other(format!("Failed to serialize request body: {}", e))
            })?;
        let resp = self.authenticated_post(CREATE_ORDER, Some(&json_body)).await?;
        let data: CreateOrderResponse = serde_json::from_str(&resp).map_err(|e| {
            KalshiError::Other(format!("Parse error: {e}. Response: {resp}"))})?;
        Ok(data)
    }

    pub async fn create_order_group(&self, body: &CreateOrderGroupRequest)-> Result< CreateOrderGroupResponse, KalshiError>{
        // let json_body = serde_json::to_string(body).map_err(|e| {
        //         KalshiError::Other(format!("Failed to serialize request body: {}", e))
        //     })?;
        let resp = self.authenticated_post(CREATE_ORDER_GROUP, Some(&body)).await?;
        let data: CreateOrderGroupResponse = serde_json::from_str(&resp).map_err(|e|{
            KalshiError::Other(format!("Failed to serialize request body: {}", e))
        })?;
        Ok(data)

    }
    pub async fn decrease_order(&self, order_id: &str, body:&DecreaseOrderRequest)-> Result<DecreaseOrderResponse,KalshiError>{
        let json_body = serde_json::to_string(body).map_err(|e| {
                KalshiError::Other(format!("Failed to serialize request body: {}", e))
            })?;
        let url = DECREASE_ORDER.replace("{}",order_id);
        let resp = self.authenticated_post(&url, Some(&json_body)).await?;
        let data: DecreaseOrderResponse = serde_json::from_str(&resp).map_err(|e|{
            KalshiError::Other(format!("Failed to serialize request body: {}", e))
        })?;
        Ok(data)

    }

    pub async fn delete_order_group(&self, order_group_id: &str)->Result<DeleteOrderGroupResponse,KalshiError>{
        let url = DELETE_ORDER_GROUP.replace("{}", order_group_id);
        let (_,resp) = self.authenticated_delete::<str>(&url, None).await?;
        let data: DeleteOrderGroupResponse = serde_json::from_str(&resp).map_err(|e|{
            KalshiError::Other(format!("Failed to serialize request body: {}", e))
        })?;
        Ok(data)
    }
    pub async fn get_balance(&self)-> Result<GetBalanceResponse,KalshiError>{
        let resp = self.authenticated_get(GET_BALANCE).await?;
        let data: GetBalanceResponse = serde_json::from_str(&resp).map_err(|e|{
            KalshiError::Other(format!("Failed to serialize request body: {}", e))
        })?;
        Ok(data)
    }

    pub async fn get_fills(){

    }
    pub async fn get_order(&self, order_id:&str)-> Result<GetOrderResponse,KalshiError>{
        let url = GET_ORDER.replace("{}", order_id);
        let resp = self.authenticated_get(&url).await?;
        let data: GetOrderResponse = serde_json::from_str(&resp).map_err(|e|{
            KalshiError::Other(format!("Failed to serialize request body: {}", e))
        })?;
        Ok(data)

    }
    pub async fn get_order_group(&self, order_group_id:&str)-> Result<GetOrderGroupResponse,KalshiError>{
        let url = GET_ORDER_GROUP.replace("{}", order_group_id);
        let resp = self.authenticated_get(&url).await?;
        let data:GetOrderGroupResponse = serde_json::from_str(&resp).map_err(|e|{
            KalshiError::Other(format!("Failed to serialize request body: {}", e))
        })?;
        Ok(data)
    }
    pub async fn get_order_groups(&self)->Result<GetOrderGroupsResponse,KalshiError>{
        let resp = self.authenticated_get(GET_ORDER_GROUPS).await?;
        let data:GetOrderGroupsResponse = serde_json::from_str(&resp).map_err(|e|{
            KalshiError::Other(format!("Failed to serialize request body: {}", e))
        })?;
        Ok(data)
    }
    pub async fn get_order_queue_position(&self, order_id:&str)-> Result<GetOrderQueuePositionResponse, KalshiError>{
        let url = GET_ORDER_QUEUE_POSITION.replace("{}", order_id);
        let resp = self.authenticated_get(&url).await?;
        let data:GetOrderQueuePositionResponse = serde_json::from_str(&resp).map_err(|e|{
            KalshiError::Other(format!("Failed to serialize request body: {}", e))
        })?;
        Ok(data)
    }


    pub async fn get_orders(&self, params: &GetOrdersParams) -> Result<GetOrdersResponse, KalshiError> {
        let query = serde_urlencoded::to_string(params)
            .map_err(|e| KalshiError::Other(format!("Failed to serialize params: {}", e)))?;

        let url = if query.is_empty() {
            GET_ORDERS.to_string()
        } else {
            format!("{}?{}", GET_ORDERS, query)
        };

        let resp = self.authenticated_get(&url).await?;
        let data: GetOrdersResponse = serde_json::from_str(&resp).map_err(|e| {
            KalshiError::Other(format!("Failed to deserialize response: {}", e))
        })?;
        Ok(data)
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
//! Portfolio module endpoints.
//!
//! This module implements API endpoints for portfolio operations.

use crate::client::KalshiClient;
use crate::errors::KalshiError;
use crate::portfolio::models::{
    AmendOrderRequest, AmendOrderResponse, BatchCancelOrdersRequest,
    BatchCancelOrdersResponse, BatchCreateOrdersRequest, BatchCreateOrdersResponse,
    CancelOrderResponse, CreateOrderGroupRequest, CreateOrderGroupResponse,
    CreateOrderRequest, CreateOrderResponse, DecreaseOrderRequest, DecreaseOrderResponse,
    DeleteOrderGroupResponse, GetBalanceResponse, GetFillsParams, GetFillsResponse,
    GetOrderGroupResponse, GetOrderGroupsResponse, GetOrderQueuePositionResponse,
    GetOrderResponse, GetOrdersParams, GetOrdersResponse, GetPositionsParams,
    GetPositionsResponse, GetQueueParams, GetQueuePositionsResponse,
    GetSettlementsParams, GetSettlementsResponse, GetTotalRestingOrderValueResponse,
    ResetOrderGroupResponse,
};


const AMEND_ORDER: &str = "/trade-api/v2/portfolio/orders/{}/amend";
const BATCH_CANCEL_ORDERS: &str = "/trade-api/v2/portfolio/orders/batched";
const BATCH_CREATE_ORDERS: &str = "/trade-api/v2/portfolio/orders/batched";
const CANCEL_ORDER: &str = "/trade-api/v2/portfolio/orders/{}";
const CREATE_ORDER: &str = "/trade-api/v2/portfolio/orders";
const CREATE_ORDER_GROUP: &str = "/trade-api/v2/portfolio/order_groups/create";
const DECREASE_ORDER: &str = "/trade-api/v2/portfolio/orders/{}/decrease";
const DELETE_ORDER_GROUP: &str = "/trade-api/v2/portfolio/order_groups/{}";
const GET_BALANCE: &str = "/trade-api/v2/portfolio/balance";
const GET_FILLS: &str = "/trade-api/v2/portfolio/fills";
const GET_ORDER: &str = "/trade-api/v2/portfolio/orders/{}";
const GET_ORDER_GROUP: &str = "/trade-api/v2/portfolio/order_groups/{}";
const GET_ORDER_GROUPS: &str = "/trade-api/v2/portfolio/order_groups";
const GET_ORDER_QUEUE_POSITION: &str = "/trade-api/v2/portfolio/orders/{}/queue_position";
const GET_ORDERS: &str = "/trade-api/v2/portfolio/orders";
const GET_POSITIONS: &str = "/trade-api/v2/portfolio/positions";
const GET_QUEUE_POSITIONS: &str = "/trade-api/v2/portfolio/orders/queue_positions";
const GET_SETTLEMENTS: &str = "/trade-api/v2/portfolio/settlements";
const GET_TOTAL_RESTING_ORDER_VALUE: &str = "/trade-api/v2/portfolio/summary/total_resting_order_value";
const RESET_ORDER_GROUP: &str = "/trade-api/v2/portfolio/order_groups/{}/reset";


impl KalshiClient {

    /// Amend Order.
    ///
    /// **Endpoint:** `PUT /portfolio/orders/{}/amend`
    ///
    /// # Returns
    /// Result with response data or error
    pub async fn amend_order(
        &self,
        order_id: &str,
        body: &AmendOrderRequest,
    ) -> Result<AmendOrderResponse, KalshiError> {
        let url = AMEND_ORDER.replace("{}", order_id);
        let resp = self.authenticated_post(&url, Some(&body)).await?;
        let data: AmendOrderResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(
                format!("Parse error: {e}. Response: {resp}"),
            ))?;
        Ok(data)
    }


    /// Batch Cancel Orders.
    ///
    /// **Endpoint:** `DELETE /portfolio/orders/batched`
    ///
    /// # Returns
    /// Result with response data or error
    pub async fn batch_cancel_orders(
        &self,
        body: &BatchCancelOrdersRequest,
    ) -> Result<BatchCancelOrdersResponse, KalshiError> {
        let (status, resp) = self
            .authenticated_delete(BATCH_CANCEL_ORDERS, Some(&body))
            .await?;
        if status.as_u16() != 200 {
            return Err(
                KalshiError::Other(format!("Expected 200 OK, got {}: {}", status, resp)),
            );
        }
        let data: BatchCancelOrdersResponse = serde_json::from_str(&resp)
            .map_err(|e| {
                KalshiError::Other(
                    format!("Parse error: {e}. Response: {resp}, status: {status}"),
                )
            })?;
        Ok(data)
    }


    /// Batch Create Orders.
    ///
    /// **Endpoint:** `POST /portfolio/orders/batched`
    ///
    /// # Returns
    /// Result with response data or error
    pub async fn batch_create_orders(
        &self,
        body: &BatchCreateOrdersRequest,
    ) -> Result<BatchCreateOrdersResponse, KalshiError> {
        let resp = self.authenticated_post(BATCH_CREATE_ORDERS, Some(&body)).await?;
        let data: BatchCreateOrdersResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(
                format!("Parse error: {e}. Response: {resp}"),
            ))?;
        Ok(data)
    }


    /// Cancel Order.
    ///
    /// **Endpoint:** `DELETE /portfolio/orders/{}`
    ///
    /// # Returns
    /// Result with response data or error
    pub async fn cancel_order(
        &self,
        order_id: String,
    ) -> Result<CancelOrderResponse, KalshiError> {
        let url: &str = &CANCEL_ORDER.replace("{}", &order_id);
        let (status, resp) = self.authenticated_delete::<str>(url, None).await?;
        // API sometimes returns empty body on success, which would fail JSON parsing
        if resp.trim().is_empty() {
            return Err(
                KalshiError::Other(
                    format!(
                        "Empty response from cancel_order API (status: {}). This is unexpected - the API should return the canceled order.",
                        status
                    ),
                ),
            );
        }
        let data: CancelOrderResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(
                format!("Parse error: {e}. Response: {resp}"),
            ))?;
        Ok(data)
    }


    /// Create Order.
    ///
    /// **Endpoint:** `POST /portfolio/orders`
    ///
    /// # Returns
    /// Result with response data or error
    pub async fn create_order(
        &self,
        body: &CreateOrderRequest,
    ) -> Result<CreateOrderResponse, KalshiError> {
        let resp = self.authenticated_post(CREATE_ORDER, Some(&body)).await?;
        let data: CreateOrderResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(
                format!("Parse error: {e}. Response: {resp}"),
            ))?;
        Ok(data)
    }


    /// Create Order Group.
    ///
    /// **Endpoint:** `POST /portfolio/orders`
    ///
    /// # Returns
    /// Result with response data or error
    pub async fn create_order_group(
        &self,
        body: &CreateOrderGroupRequest,
    ) -> Result<CreateOrderGroupResponse, KalshiError> {
        let resp = self.authenticated_post(CREATE_ORDER_GROUP, Some(&body)).await?;
        let data: CreateOrderGroupResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(
                format!("Failed to serialize request body: {}", e),
            ))?;
        Ok(data)
    }


    /// Decrease Order.
    ///
    /// **Endpoint:** `GET /portfolio/orders/{}/decrease`
    ///
    /// # Returns
    /// Result with response data or error
    pub async fn decrease_order(
        &self,
        order_id: &str,
        body: &DecreaseOrderRequest,
    ) -> Result<DecreaseOrderResponse, KalshiError> {
        let url = DECREASE_ORDER.replace("{}", order_id);
        let resp = self.authenticated_post(&url, Some(&body)).await?;
        let data: DecreaseOrderResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(
                format!("Failed to serialize request body: {}", e),
            ))?;
        Ok(data)
    }


    /// Delete Order Group.
    ///
    /// **Endpoint:** `DELETE /portfolio/order_groups/{}`
    ///
    /// # Returns
    /// Result with response data or error
    pub async fn delete_order_group(
        &self,
        order_group_id: &str,
    ) -> Result<DeleteOrderGroupResponse, KalshiError> {
        let url = DELETE_ORDER_GROUP.replace("{}", order_group_id);
        let (status, resp) = self.authenticated_delete::<str>(&url, None).await?;
        // Delete operations might return empty body with 204 status, which is valid
        if status.as_u16() == 204 || resp.trim().is_empty() {
            return Ok(DeleteOrderGroupResponse {
                body: None,
            });
        }
        let data: DeleteOrderGroupResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(
                format!("Failed to serialize request body: {}", e),
            ))?;
        Ok(data)
    }


    /// Get Balance.
    ///
    /// **Endpoint:** `GET /portfolio/balance`
    ///
    /// # Returns
    /// Result with response data or error
    pub async fn get_balance(&self) -> Result<GetBalanceResponse, KalshiError> {
        let resp = self.authenticated_get::<str>(GET_BALANCE, None).await?;
        let data: GetBalanceResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(
                format!("Failed to serialize request body: {}", e),
            ))?;
        Ok(data)
    }


    /// Get Fills.
    ///
    /// **Endpoint:** `GET /portfolio/fills`
    ///
    /// # Returns
    /// Result with response data or error
    pub async fn get_fills(
        &self,
        params: &GetFillsParams,
    ) -> Result<GetFillsResponse, KalshiError> {
        let query = serde_urlencoded::to_string(params)
            .map_err(|e| KalshiError::Other(
                format!("Failed to serialize params: {}", e),
            ))?;
        let url = if query.is_empty() {
            GET_FILLS.to_string()
        } else {
            format!("{}?{}", GET_FILLS, query)
        };
        let resp = self.authenticated_get::<str>(&url, None).await?;
        let data: GetFillsResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(
                format!("Failed to deserialize response: {}", e),
            ))?;
        Ok(data)
    }


    /// Get Order.
    ///
    /// **Endpoint:** `GET /portfolio/orders/{}`
    ///
    /// # Returns
    /// Result with response data or error
    pub async fn get_order(
        &self,
        order_id: &str,
    ) -> Result<GetOrderResponse, KalshiError> {
        let url = GET_ORDER.replace("{}", order_id);
        let resp = self.authenticated_get::<str>(&url, None).await?;
        let data: GetOrderResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(
                format!("Failed to serialize request body: {}", e),
            ))?;
        Ok(data)
    }


    /// Get Order Group.
    ///
    /// **Endpoint:** `GET /portfolio/orders/{}`
    ///
    /// # Returns
    /// Result with response data or error
    pub async fn get_order_group(
        &self,
        order_group_id: &str,
    ) -> Result<GetOrderGroupResponse, KalshiError> {
        let url = GET_ORDER_GROUP.replace("{}", order_group_id);
        let resp = self.authenticated_get::<str>(&url, None).await?;
        let data: GetOrderGroupResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(
                format!("Failed to serialize request body: {}", e),
            ))?;
        Ok(data)
    }


    /// Get Order Groups.
    ///
    /// **Endpoint:** `GET /portfolio/orders/{}`
    ///
    /// # Returns
    /// Result with response data or error
    pub async fn get_order_groups(&self) -> Result<GetOrderGroupsResponse, KalshiError> {
        let resp = self.authenticated_get::<str>(GET_ORDER_GROUPS, None).await?;
        let data: GetOrderGroupsResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(
                format!("Failed to serialize request body: {}", e),
            ))?;
        Ok(data)
    }


    /// Get Order Queue Position.
    ///
    /// **Endpoint:** `GET /portfolio/orders/{}`
    ///
    /// # Returns
    /// Result with response data or error
    pub async fn get_order_queue_position(
        &self,
        order_id: &str,
    ) -> Result<GetOrderQueuePositionResponse, KalshiError> {
        let url = GET_ORDER_QUEUE_POSITION.replace("{}", order_id);
        let resp = self.authenticated_get::<str>(&url, None).await?;
        let data: GetOrderQueuePositionResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(
                format!("Failed to serialize request body: {}", e),
            ))?;
        Ok(data)
    }


    /// Get Orders.
    ///
    /// **Endpoint:** `GET /portfolio/orders/{}`
    ///
    /// # Returns
    /// Result with response data or error
    pub async fn get_orders(
        &self,
        params: &GetOrdersParams,
    ) -> Result<GetOrdersResponse, KalshiError> {
        let query = serde_urlencoded::to_string(params)
            .map_err(|e| KalshiError::Other(
                format!("Failed to serialize params: {}", e),
            ))?;
        let url = if query.is_empty() {
            GET_ORDERS.to_string()
        } else {
            format!("{}?{}", GET_ORDERS, query)
        };
        let resp = self.authenticated_get::<str>(&url, None).await?;
        let data: GetOrdersResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(
                format!("Failed to deserialize response: {}", e),
            ))?;
        Ok(data)
    }


    /// Get Positions.
    ///
    /// **Endpoint:** `GET /portfolio/positions`
    ///
    /// # Returns
    /// Result with response data or error
    pub async fn get_positions(
        &self,
        params: &GetPositionsParams,
    ) -> Result<GetPositionsResponse, KalshiError> {
        let query = serde_urlencoded::to_string(params)
            .map_err(|e| KalshiError::Other(
                format!("Failed to serialize params: {}", e),
            ))?;
        let url = if query.is_empty() {
            GET_POSITIONS.to_string()
        } else {
            format!("{}?{}", GET_POSITIONS, query)
        };
        let resp = self.authenticated_get::<str>(&url, None).await?;
        let data: GetPositionsResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(
                format!("Failed to deserialize response: {}", e),
            ))?;
        Ok(data)
    }


    /// Get Queue Positions.
    ///
    /// **Endpoint:** `GET /portfolio/orders/queue_positions`
    ///
    /// # Returns
    /// Result with response data or error
    pub async fn get_queue_positions(
        &self,
        params: &GetQueueParams,
    ) -> Result<GetQueuePositionsResponse, KalshiError> {
        let query = serde_urlencoded::to_string(params)
            .map_err(|e| KalshiError::Other(
                format!("Failed to serialize params: {}", e),
            ))?;
        let url = if query.is_empty() {
            GET_QUEUE_POSITIONS.to_string()
        } else {
            format!("{}?{}", GET_QUEUE_POSITIONS, query)
        };
        let resp = self.authenticated_get::<str>(&url, None).await?;
        let data: GetQueuePositionsResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(
                format!("Failed to deserialize response: {}", e),
            ))?;
        Ok(data)
    }


    /// Get Settlements.
    ///
    /// **Endpoint:** `GET /portfolio/settlements`
    ///
    /// # Returns
    /// Result with response data or error
    pub async fn get_settlements(
        &self,
        params: &GetSettlementsParams,
    ) -> Result<GetSettlementsResponse, KalshiError> {
        let query = serde_urlencoded::to_string(params)
            .map_err(|e| KalshiError::Other(
                format!("Failed to serialize params: {}", e),
            ))?;
        let url = if query.is_empty() {
            GET_SETTLEMENTS.to_string()
        } else {
            format!("{}?{}", GET_SETTLEMENTS, query)
        };
        let resp = self.authenticated_get::<str>(&url, None).await?;
        let data: GetSettlementsResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(
                format!("Failed to deserialize response: {}", e),
            ))?;
        Ok(data)
    }


    ///Endpoint for getting the total value, in cents, of resting orders. This endpoint is only intended for use by FCM members (rare). Note: If you’re uncertain about this endpoint, it likely does not apply to you.
    pub async fn get_total_resting_order_value(
        &self,
    ) -> Result<GetTotalRestingOrderValueResponse, KalshiError> {
        let resp = self
            .authenticated_get::<str>(&GET_TOTAL_RESTING_ORDER_VALUE, None)
            .await?;
        let data: GetTotalRestingOrderValueResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(
                format!("Parse error: {e}. Response: {resp}"),
            ))?;
        Ok(data)
    }

    
    /// Reset Order Group.
    ///
    /// **Endpoint:** `GET /portfolio/order_groups/{}/reset`
    ///
    /// # Returns
    /// Result with response data or error
    pub async fn reset_order_group(
        &self,
        order_group_id: &str,
    ) -> Result<ResetOrderGroupResponse, KalshiError> {
        let url = RESET_ORDER_GROUP.replace("{}", order_group_id);
        // API requires a body even though reset doesn't need any parameters
        let empty_body = serde_json::json!({});
        let (status, resp) = self.authenticated_put(&url, Some(&empty_body)).await?;
        if status.is_success() {
            Ok(ResetOrderGroupResponse {})
        } else {
            Err(KalshiError::Other(format!("Unexpected status {}: {}", status, resp)))
        }
    }
}

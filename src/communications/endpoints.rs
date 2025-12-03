//! Communications module endpoints.
//!
//! This module implements API endpoints for communications operations.

use crate::client::KalshiClient;
use crate::communications::models::{
    Accept, AcceptQuoteResponse, ConfirmQuoteResponse, CreateQuoteRequest, CreateQuoteResponse,
    CreateRFQRequest, CreateRFQResponse, DeleteQuoteResponse, DeleteRFQResponse,
    GetCommunicationsIDResponse, GetQuoteResponse, GetQuotesQuery, GetQuotesResponse,
    GetRFQResponse, GetRFQsResponse,
};
use crate::errors::KalshiError;

const ACCEPT_QUOTE: &str = "/trade-api/v2/communications/quotes/{quote_id}/accept";
const CONFIRM_QUOTE: &str = "/trade-api/v2/communications/quotes/{quote_id}/confirm";
const CREATE_QUOTE: &str = "/trade-api/v2/communications/quotes";
const GET_RFQ: &str = "/trade-api/v2/communications/rfqs/{}";
const DELETE_RFQ: &str = "/trade-api/v2/communications/rfqs/{}";
const GET_QUOTES: &str = "/trade-api/v2/communications/quotes";
const DELETE_QUOTE: &str = "/trade-api/v2/communications/quotes/{}";
const GET_QUOTE: &str = "/trade-api/v2/communications/quotes/{}";
const GET_COMMUNICATIONS_ID: &str = "/trade-api/v2/communications/id";
const GET_RFQS: &str = "/trade-api/v2/communications/rfqs";
const CREATE_RFQ: &str = "/trade-api/v2/communications/rfqs";

impl KalshiClient {
    /// Accept Quote.
    ///
    /// **Endpoint:** `GET /communications/quotes/{quote_id}/accept`
    ///
    /// # Returns
    /// Result with response data or error
    pub async fn accept_quote(
        &self,
        quote_id: &str,
        accepted_side: &str,
    ) -> Result<AcceptQuoteResponse, KalshiError> {
        let accept: Accept = Accept::from_str_helper(accepted_side).unwrap();
        let url = ACCEPT_QUOTE.replace("{}", quote_id);
        let resp = self
            .authenticated_get::<Accept>(&url, Some(&accept))
            .await?;
        let data: AcceptQuoteResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(format!("Parse error: {e}")))?;
        Ok(data)
    }

    /// Confirm Quote.
    ///
    /// **Endpoint:** `GET /communications/quotes/{quote_id}/confirm`
    ///
    /// # Returns
    /// Result with response data or error
    pub async fn confirm_quote(&self, quote_id: &str) -> Result<ConfirmQuoteResponse, KalshiError> {
        let url = CONFIRM_QUOTE.replace("{}", quote_id);
        let resp = self.authenticated_get::<str>(&url, None).await?;
        let data: ConfirmQuoteResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(format!("Parse error: {e}")))?;
        Ok(data)
    }

    /// Create Quote.
    ///
    /// **Endpoint:** `POST /communications/quotes`
    ///
    /// # Returns
    /// Result with response data or error
    pub async fn create_quote(
        &self,
        body: CreateQuoteRequest,
    ) -> Result<CreateQuoteResponse, KalshiError> {
        let resp = self
            .authenticated_get::<CreateQuoteRequest>(CREATE_QUOTE, Some(&body))
            .await?;
        let data: CreateQuoteResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(format!("Parse error: {e}")))?;
        Ok(data)
    }

    /// Get Rfq.
    ///
    /// **Endpoint:** `GET /communications/rfqs/{}`
    ///
    /// # Returns
    /// Result with response data or error
    pub async fn get_rfq(&self, rfq_id: &str) -> Result<GetRFQResponse, KalshiError> {
        let url = GET_RFQ.replace("{}", rfq_id);
        let resp = self.authenticated_get::<str>(&url, None).await?;
        let data: GetRFQResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(format!("Parse error: {e}")))?;
        Ok(data)
    }

    /// GET /trade-api/v2/communications/quotes
    /// Returns all quotes matching the query criteria
    pub async fn get_quotes(
        &self,
        params: &GetQuotesQuery,
    ) -> Result<GetQuotesResponse, KalshiError> {
        let query = serde_urlencoded::to_string(params)
            .map_err(|e| KalshiError::Other(format!("Failed to serialize params: {}", e)))?;
        let url = if query.is_empty() {
            GET_QUOTES.to_string()
        } else {
            format!("{}?{}", GET_QUOTES, query)
        };
        let resp = self.authenticated_get::<str>(&url, None).await?;
        let data: GetQuotesResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(format!("Parse error: {e}. Response: {resp}")))?;
        Ok(data)
    }

    /// Delete Quote.
    ///
    /// **Endpoint:** `DELETE /communications/quotes/{}`
    ///
    /// # Returns
    /// Result with response data or error
    pub async fn delete_quote(&self, quote_id: &str) -> Result<DeleteQuoteResponse, KalshiError> {
        let url = DELETE_QUOTE.replace("{}", quote_id);
        let (status, resp) = self.authenticated_delete::<str>(&url, None).await?;
        if status.as_u16() == 204 || resp.trim().is_empty() {
            return Ok(DeleteQuoteResponse { body: None });
        }
        let data: DeleteQuoteResponse =
            serde_json::from_str(&resp).map_err(KalshiError::ParseError)?;
        Ok(data)
    }
    /// Delete Rfq.
    ///
    /// **Endpoint:** `DELETE /communications/rfqs/{}`
    ///
    /// # Returns
    /// Result with response data or error
    pub async fn delete_rfq(&self, rfq_id: &str) -> Result<DeleteRFQResponse, KalshiError> {
        let url = DELETE_RFQ.replace("{}", rfq_id);
        let (status, resp) = self.authenticated_delete::<str>(&url, None).await?;
        if status.as_u16() == 204 || resp.trim().is_empty() {
            return Ok(DeleteRFQResponse { body: None });
        }
        let data: DeleteRFQResponse =
            serde_json::from_str(&resp).map_err(KalshiError::ParseError)?;
        Ok(data)
    }

    /// Get Communications Id.
    ///
    /// **Endpoint:** `GET /communications/id`
    ///
    /// # Returns
    /// Result with response data or error
    pub async fn get_communications_id(&self) -> Result<GetCommunicationsIDResponse, KalshiError> {
        let resp = self
            .authenticated_get::<str>(GET_COMMUNICATIONS_ID, None)
            .await?;
        let data: GetCommunicationsIDResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(format!("Parse error: {e}")))?;
        Ok(data)
    }

    /// GET /trade-api/v2/communications/rfqs
    /// Returns all RFQs for the authenticated user
    pub async fn get_rfqs(&self) -> Result<GetRFQsResponse, KalshiError> {
        let resp = self.authenticated_get::<str>(GET_RFQS, None).await?;
        let data: GetRFQsResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(format!("Parse error: {e}. Response: {resp}")))?;
        Ok(data)
    }

    /// POST /trade-api/v2/communications/rfqs
    /// Creates a new RFQ. You can have a maximum of 100 open RFQs at a time.
    pub async fn create_rfq(
        &self,
        body: &CreateRFQRequest,
    ) -> Result<CreateRFQResponse, KalshiError> {
        let resp = self.authenticated_post(CREATE_RFQ, Some(body)).await?;
        let data: CreateRFQResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(format!("Parse error: {e}. Response: {resp}")))?;
        Ok(data)
    }

    /// GET /trade-api/v2/communications/quotes/{quote_id}
    /// Gets a single quote by its ID
    pub async fn get_quote(&self, quote_id: &str) -> Result<GetQuoteResponse, KalshiError> {
        let url = GET_QUOTE.replace("{}", quote_id);
        let resp = self.authenticated_get::<str>(&url, None).await?;
        let data: GetQuoteResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(format!("Parse error: {e}. Response: {resp}")))?;
        Ok(data)
    }
}

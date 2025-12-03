//! Markets module endpoints.
//!
//! This module implements API endpoints for retrieving market data from Kalshi,
//! including market lists, individual market details, orderbooks, candlesticks, and trades.
//!
//! # Usage
//!
//! All endpoint methods are available on [`KalshiClient`](crate::client::KalshiClient).
//! See the client documentation for a complete list of available methods.

use crate::client::KalshiClient;
use crate::errors::KalshiError;
use crate::markets::models::{
    CandlesticksQuery, GetMarketCandlesticksResponse, GetMarketOrderbookResponse,
    GetMarketResponse, GetMarketsResponse, GetTradesQuery, GetTradesResponse,
    MarketsQuery, OrderbookQuery,
};

const GET_MARKETS: &str = "/trade-api/v2/markets";
const GET_MARKET: &str = "/trade-api/v2/markets/{}";
const GET_TRADES: &str = "/trade-api/v2/markets/trades";
const GET_MARKET_ORDERBOOK: &str = "/trade-api/v2/markets/{}/orderbook";
const GET_MARKET_CANDLESTICKS: &str = "/trade-api/v2/series/{}/markets/{}/candlesticks";

impl KalshiClient {
    /// Retrieves a list of markets from Kalshi.
    ///
    /// **Endpoint:** `GET /markets`
    ///
    /// # Query Parameters
    /// - `limit` - Maximum number of markets to return
    /// - `cursor` - Pagination cursor for retrieving additional results
    /// - `event_ticker` - Filter by event ticker
    /// - `series_ticker` - Filter by series ticker
    /// - `max_close_ts` - Filter markets closing before this timestamp
    /// - `min_close_ts` - Filter markets closing after this timestamp
    /// - `status` - Filter by market status (e.g., "open", "closed")
    /// - `tickers` - Comma-separated list of specific market tickers
    ///
    /// # Returns
    /// [`GetMarketsResponse`] containing a vector of Market objects and optional pagination cursor
    ///
    /// # Models Used
    /// - Query: [`MarketsQuery`]
    /// - Response: [`GetMarketsResponse`]
    pub async fn get_all_markets(
        &self,
        params: &MarketsQuery,
    ) -> Result<GetMarketsResponse, KalshiError> {
        // Only append '?' if there are actual query params to avoid malformed URLs
        let query = serde_urlencoded::to_string(&params)
            .map_err(|e| KalshiError::Other(
                format!("Failed to serialize params: {}", e),
            ))?;
        let url = if query.is_empty() {
            GET_MARKETS.to_string()
        } else {
            format!("{}?{}", GET_MARKETS, query)
        };
        let resp = self.unauthenticated_get(&url).await?;
        let data: GetMarketsResponse = serde_json::from_str(&resp)
            .map_err(|e| {
                KalshiError::Other(
                    format!(
                        "Invalid Parsing response format: Parse error: {e}. Response: {resp}"
                    ),
                )
            })?;
        Ok(data)
    }


    /// Retrieves detailed information for a specific market.
    ///
    /// **Endpoint:** `GET /markets/{ticker}`
    ///
    /// # Parameters
    /// - `ticker` - The unique market ticker identifier
    ///
    /// # Returns
    /// [`GetMarketResponse`] containing detailed Market information
    ///
    /// # Models Used
    /// - Response: [`GetMarketResponse`]
    pub async fn get_market(
        &self,
        ticker: &str,
    ) -> Result<GetMarketResponse, KalshiError> {
        let url = GET_MARKET.replace("{}", ticker);
        let resp = self.unauthenticated_get(&url).await?;
        let data: GetMarketResponse = serde_json::from_str(&resp)
            .map_err(|e| {
                KalshiError::Other(
                    format!(
                        "Invalid Parsing response format: Parse error: {e}. Response: {resp}"
                    ),
                )
            })?;
        Ok(data)
    }


    /// Retrieves recent trade history across markets.
    ///
    /// **Endpoint:** `GET /markets/trades`
    ///
    /// # Query Parameters
    /// - `limit` - Maximum number of trades to return
    /// - `cursor` - Pagination cursor for additional results
    /// - `ticker` - Filter trades for a specific market ticker
    /// - `min_ts` - Filter trades after this timestamp
    /// - `max_ts` - Filter trades before this timestamp
    ///
    /// # Returns
    /// [`GetTradesResponse`] containing a vector of Trade objects
    ///
    /// # Models Used
    /// - Query: [`GetTradesQuery`]
    /// - Response: [`GetTradesResponse`]
    pub async fn get_trades(
        &self,
        limit: Option<u16>,
        cursor: Option<String>,
        ticker: Option<String>,
        min_ts: Option<u64>,
        max_ts: Option<u64>,
    ) -> Result<GetTradesResponse, KalshiError> {
        let params = GetTradesQuery {
            limit,
            cursor,
            ticker,
            min_ts,
            max_ts,
        };
        let query = serde_urlencoded::to_string(&params)
            .map_err(|e| KalshiError::Other(
                format!("Failed to serialize params: {}", e),
            ))?;
        let url = if query.is_empty() {
            GET_TRADES.to_string()
        } else {
            format!("{}?{}", GET_TRADES, query)
        };
        let resp = self.unauthenticated_get(&url).await?;
        let data: GetTradesResponse = serde_json::from_str(&resp)
            .map_err(|e| {
                KalshiError::Other(
                    format!(
                        "Invalid Parsing response format: Parse error: {e}. Response: {resp}"
                    ),
                )
            })?;
        Ok(data)
    }


    /// Retrieves the current orderbook for a specific market.
    ///
    /// **Endpoint:** `GET /markets/{ticker}/orderbook`
    ///
    /// # Parameters
    /// - `ticker` - The market ticker identifier
    ///
    /// # Query Parameters
    /// - `depth` - Number of price levels to return (capped at 100)
    ///
    /// # Returns
    /// [`GetMarketOrderbookResponse`] containing the Orderbook with bid/ask levels
    ///
    /// # Models Used
    /// - Query: [`OrderbookQuery`]
    /// - Response: [`GetMarketOrderbookResponse`]
    pub async fn get_market_orderbook(
        &self,
        ticker: &str,
        depth: Option<u128>,
    ) -> Result<GetMarketOrderbookResponse, KalshiError> {
        let base_url = GET_MARKET_ORDERBOOK.replace("{}", ticker);
        let capped_depth = depth.map(|d| if d > 100 { 100 } else { d });
        let params = OrderbookQuery {
            depth: capped_depth,
        };
        let query = serde_urlencoded::to_string(&params)
            .map_err(|e| KalshiError::Other(
                format!("Failed to serialize params: {}", e),
            ))?;
        let url = if query.is_empty() {
            base_url
        } else {
            format!("{}?{}", base_url, query)
        };
        let resp = self.unauthenticated_get(&url).await?;
        let data: GetMarketOrderbookResponse = serde_json::from_str(&resp)
            .map_err(|e| {
                KalshiError::Other(
                    format!(
                        "Invalid Parsing response format: Parse error: {e}. Response: {resp}"
                    ),
                )
            })?;
        Ok(data)
    }


    /// Retrieves historical candlestick (OHLC) data for a market.
    ///
    /// **Endpoint:** `GET /series/{series_ticker}/markets/{ticker}/candlesticks`
    ///
    /// # Parameters
    /// - `series_ticker` - The series ticker identifier
    /// - `ticker` - The market ticker identifier
    ///
    /// # Query Parameters
    /// - `start_ts` - Start timestamp for candlestick data
    /// - `end_ts` - End timestamp for candlestick data
    /// - `period_interval` - Time period for each candlestick in seconds
    ///
    /// # Returns
    /// [`GetMarketCandlesticksResponse`] containing a vector of Candlestick data points
    ///
    /// # Models Used
    /// - Query: [`CandlesticksQuery`]
    /// - Response: [`GetMarketCandlesticksResponse`]
    pub async fn get_market_candlesticks(
        &self,
        series_ticker: &str,
        ticker: &str,
        start_ts: i64,
        end_ts: i64,
        period_interval: u32,
    ) -> Result<GetMarketCandlesticksResponse, KalshiError> {
        // First {} is series_ticker, second {} is market ticker - order matters here
        let base_url = GET_MARKET_CANDLESTICKS
            .replacen("{}", series_ticker, 1)
            .replacen("{}", ticker, 1);
        let params = CandlesticksQuery {
            start_ts,
            end_ts,
            period_interval,
        };
        let query = serde_urlencoded::to_string(&params)
            .map_err(|e| KalshiError::Other(
                format!("Failed to serialize params: {}", e),
            ))?;
        let url = if query.is_empty() {
            base_url
        } else {
            format!("{}?{}", base_url, query)
        };
        let resp = self.unauthenticated_get(&url).await?;
        let data: GetMarketCandlesticksResponse = serde_json::from_str(&resp)
            .map_err(|e| {
                KalshiError::Other(
                    format!(
                        "Invalid Parsing response format: Parse error: {e}. Response: {resp}"
                    ),
                )
            })?;
        Ok(data)
    }
}

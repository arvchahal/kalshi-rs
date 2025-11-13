use crate::client::KalshiClient;
use crate::errors::KalshiError;
use crate::markets::models::{
    GetMarketOrderbookResponse, GetMarketResponse, GetMarketsResponse,
    MarketsQuery, GetMarketCandlesticksResponse, CandlesticksQuery,
    GetTradesQuery, GetTradesResponse, OrderbookQuery
};

const GET_MARKETS: &str = "/trade-api/v2/markets"; //no auth GET
const GET_MARKET: &str = "/trade-api/v2/markets/{}";//no auth GET the {} with ticker
const GET_TRADES: &str = "/trade-api/v2/markets/trades";// noauth GET
const GET_MARKET_ORDERBOOK: &str = "/trade-api/v2/markets/{}/orderbook"; // no auth get replace the {} with ticker
const GET_MARKET_CANDLESTICKS: &str = "/trade-api/v2/series/{}/markets/{}/candlesticks";//first replacement is series ticker, second is market ticker

impl KalshiClient {
    ///might need to refactor into a macro for better function overloading not currently supported in rust
    pub async fn get_all_markets(
        &self,
        limit: Option<u16>,
        cursor: Option<&str>,
        event_ticker: Option<&str>,
        series_ticker: Option<&str>,
        max_close_ts: Option<i64>,
        min_close_ts: Option<i64>,
        status: Option<&str>,
        tickers: Option<&[&str]>, // accept a slice, join below
    ) -> Result<GetMarketsResponse, KalshiError> {
        let params = MarketsQuery {
            limit,
            cursor,
            event_ticker,
            series_ticker,
            max_close_ts,
            min_close_ts,
            status,
            tickers: tickers.map(|t| t.join(",")),
        };

        let query = serde_urlencoded::to_string(&params)
            .map_err(|e| KalshiError::Other(format!("Failed to serialize params: {}", e)))?;

        let url = if query.is_empty() {
            GET_MARKETS.to_string()
        } else {
            format!("{}?{}", GET_MARKETS, query)
        };

        let resp = self.unauthenticated_get(&url).await?;
        let data: GetMarketsResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(format!("Invalid Parsing response format: Parse error: {e}. Response: {resp}")))?;
        Ok(data)
    }

    pub async fn get_market(&self, ticker: &str) -> Result<GetMarketResponse, KalshiError> {
        let url = GET_MARKET.replace("{}", ticker);
        let resp = self.unauthenticated_get(&url).await?;
        let data: GetMarketResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(format!("Invalid Parsing response format: Parse error: {e}. Response: {resp}")))?;
        Ok(data)
    }

    pub async fn get_trades(&self, limit: Option<u16>, cursor: Option<String>, ticker: Option<String>, min_ts: Option<u64>, max_ts: Option<u64>) -> Result<GetTradesResponse, KalshiError> {
        let params = GetTradesQuery { limit, cursor, ticker, min_ts, max_ts };
        let query = serde_urlencoded::to_string(&params)
            .map_err(|e| KalshiError::Other(format!("Failed to serialize params: {}", e)))?;

        let url = if query.is_empty() {
            GET_TRADES.to_string()
        } else {
            format!("{}?{}", GET_TRADES, query)
        };

        let resp = self.unauthenticated_get(&url).await?;
        let data: GetTradesResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(format!("Invalid Parsing response format: Parse error: {e}. Response: {resp}")))?;
        Ok(data)
    }

    pub async fn get_market_orderbook(&self, ticker: &str, depth: Option<u128>) -> Result<GetMarketOrderbookResponse, KalshiError> {
        let base_url = GET_MARKET_ORDERBOOK.replace("{}", ticker);

        // Cap depth at 100 if provided
        let capped_depth = depth.map(|d| if d > 100 { 100 } else { d });

        let params = OrderbookQuery { depth: capped_depth };
        let query = serde_urlencoded::to_string(&params)
            .map_err(|e| KalshiError::Other(format!("Failed to serialize params: {}", e)))?;

        let url = if query.is_empty() {
            base_url
        } else {
            format!("{}?{}", base_url, query)
        };

        println!("{}", url);
        let resp = self.unauthenticated_get(&url).await?;
        let data: GetMarketOrderbookResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(format!("Invalid Parsing response format: Parse error: {e}. Response: {resp}")))?;
        Ok(data)
    }

    pub async fn get_market_candlesticks(&self, series_ticker: &str, ticker: &str, start_ts: i64, end_ts: i64, period_interval: u32) -> Result<GetMarketCandlesticksResponse, KalshiError> {
        let base_url = GET_MARKET_CANDLESTICKS
            .replacen("{}", series_ticker, 1)
            .replacen("{}", ticker, 1);

        let params = CandlesticksQuery { start_ts, end_ts, period_interval };
        let query = serde_urlencoded::to_string(&params)
            .map_err(|e| KalshiError::Other(format!("Failed to serialize params: {}", e)))?;

        let url = if query.is_empty() {
            base_url
        } else {
            format!("{}?{}", base_url, query)
        };

        println!("{}", url);
        let resp = self.unauthenticated_get(&url).await?;
        let data: GetMarketCandlesticksResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(format!("Invalid Parsing response format: Parse error: {e}. Response: {resp}")))?;
        Ok(data)
    }
}


use crate::client::KalshiClient;
use crate::errors::KalshiError;
use crate::helpers::build_url_with_query;

// use crate::exchange::models::{GetExcahngeStatus, GetExchangeAnnouncementsResponse, GetExchangeScheduleResponse, GetUserDataTimestampResponse, GetMarketRespose};
use crate::markets::models::{
    GetMarketOrderbookResponse, GetMarketResponse, GetMarketsResponse, 
    MarketsQuery, GetMarketCandlesticksResponse, CandlesticksQuery,
    GetTradesQuery, GetTradesResponse};

const GET_MARKETS:&str = "/trade-api/v2/markets"; //no auth GET
const GET_MARKET:&str = "/trade-api/v2/markets/{}";//no auth GET the {} with ticker
const GET_TRADES:&str = "/trade-api/v2/markets/trades";// noauth GET
const GET_MARKET_ORDERBOOK:&str ="/trade-api/v2/markets/{}/orderbook"; // no auth get replace the {} with ticker
const GET_MARKET_CANDLESTICKS: &str = "/trade-api/v2/series/{}/markets/{}/candlesticks";//first replacement is series ticker, second is market ticker

impl KalshiClient{
    ///might need to refactor into a macro for better function overloading not curretnly suppported in rust
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
        let q = MarketsQuery {
            limit,
            cursor,
            event_ticker,
            series_ticker,
            max_close_ts,
            min_close_ts,
            status,
            tickers: tickers.map(|t| t.join(",")),
        };

        // build "/trade-api/v2/markets?..."
        let url = build_url_with_query(GET_MARKETS.to_string(), &q);

        let resp = self.unauthenticated_get(&url).await?;
        let data: GetMarketsResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(format!("Invalid Parsing response format: Parse error: {e}. Response: {resp}")))?;
        Ok(data)
    }
    pub async fn get_market(&self, ticker:&str)->Result<GetMarketResponse,KalshiError>{
        let url = GET_MARKET.replace("{}", ticker);
        let resp = self.unauthenticated_get(&url).await?;
        let data: GetMarketResponse = serde_json::from_str(&resp).map_err(|e| KalshiError::Other(format!("Invalid Parsing response format: Parse error: {e}. Response: {resp}")))?;
        Ok(data)
    }
    pub async fn get_trades(&self, limit: Option<u16>,cursor:Option<String>,ticker:Option<String>,min_ts:Option<u64>,max_ts:Option<u64>)->Result<GetTradesResponse, KalshiError>{
        let q: GetTradesQuery = GetTradesQuery{limit, cursor, ticker, min_ts, max_ts};
        let url = build_url_with_query(GET_TRADES.to_string(), &q);
        let resp = self.unauthenticated_get(&url).await?;
        let data: GetTradesResponse = serde_json::from_str(&resp).map_err(|e| KalshiError::Other(format!("Invalid Parsing response format: Parse error: {e}. Response: {resp}")))?;
        Ok(data)
    }
    pub async fn get_market_orderbook(&self, ticker:&str, depth: Option<u128>)->Result<GetMarketOrderbookResponse, KalshiError>{
        let mut url:String= GET_MARKET_ORDERBOOK.to_string().replace("{}", ticker);
        let mut actual_depth = depth.unwrap_or(0);
        if actual_depth>100{
            actual_depth = 100;
            url = build_url_with_query(url.clone(), &actual_depth);
        }
        println!("{}", url);
        let resp = self.unauthenticated_get(&url).await?;
        let data: GetMarketOrderbookResponse = serde_json::from_str(&resp).map_err(|e| KalshiError::Other(format!("Invalid Parsing response format: Parse error: {e}. Response: {resp}")))?;
        Ok(data)

    }
    pub async fn get_market_candlesticks(&self, series_ticker:&str, ticker:&str, start_ts: i64, end_ts: i64, period_interval: u32,)->Result<GetMarketCandlesticksResponse,KalshiError>{
        let url = GET_MARKET_CANDLESTICKS
            .replacen("{}", series_ticker,1)
            .replacen("{}", ticker,1);
        let query: CandlesticksQuery = CandlesticksQuery{start_ts, end_ts, period_interval};
        let url_query = build_url_with_query(url, &query);
        println!("{}", url_query);
        let resp = self.unauthenticated_get(&url_query).await?;
        let data: GetMarketCandlesticksResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(format!("Invalid Parsing response format: Parse error: {e}. Response: {resp}")))?;
        Ok(data)

    }

    

}

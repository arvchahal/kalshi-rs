
use crate::client::KalshiClient;
use crate::errors::KalshiError;
use chrono::{DateTime, Utc};

// use crate::exchange::models::{GetExcahngeStatus, GetExchangeAnnouncementsResponse, GetExchangeScheduleResponse, GetUserDataTimestampResponse, GetMarketRespose};
use crate::markets::models::{GetMarketResponse,GetMarketsResponse, MarketsQuery};

const GET_MARKETS:&str = "/trade-api/v2/markets"; //no auth GET
const GET_MARKET:&str = "/trade-api/v2/markets/{}";//no auth GET
const GET_TRADES:&str = "/trade-api/v2/markets/trades";// auth GET
const GET_MARKET_ORDERBOOK:&str ="/trade-api/v2/markets/{ticker}/orderbook";
const GET_MARKET_CANDLESTICKS: &str = "/trade-api/v2/series/{}/markets/{}/candlesticks";//first replacement is series ticker, second is market ticker

impl KalshiClient{
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
        let qs = serde_urlencoded::to_string(q).unwrap();
        let url = if qs.is_empty() {
            GET_MARKETS.to_string()
        } else {
            format!("{}?{}", GET_MARKETS, qs)
        };

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
    // pub fn get_trades(&self)->Result<>{

    // }
    // pub fn get_market_orderbook(&self)->Result<>{

    // }
    // pub fn get_market_candlesticks(&self)->{

    // }

}

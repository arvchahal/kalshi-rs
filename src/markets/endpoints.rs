
use crate::client::KalshiClient;
use crate::errors::KalshiError;
use crate::exchange::models::{GetExcahngeStatus, GetExchangeAnnouncementsResponse, GetExchangeScheduleResponse, GetUserDataTimestampResponse};
const GET_MARKETS:&str = "/trade-api/v2/markets"; //no auth GET
const GET_MARKET:&str = "/trade-api/v2/markets/{}";//no auth GET
const GET_TRADES:&str = "/trade-api/v2/markets/trades";// auth GET
const GET_MARKET_ORDERBOOK:&str ="/trade-api/v2/markets/{ticker}/orderbook";
const GET_MARKET_CANDLESTICKS: &str = "/trade-api/v2/series/{}/markets/{}/candlesticks";//first replacement is series ticker, second is market ticker

impl KalshiClient{
    pub fn get_market(&self)->Result<>{
        
    }
    pub fn get_markets(&self)->Result<>{

    }
    pub fn get_trades(&self)->Result<>{

    }
    pub fn get_market_orderbook(&self)->Result<>{

    }
    pub fn get_market_candlesticks(&self)->{

    }

}

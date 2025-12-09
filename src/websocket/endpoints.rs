use futures_util::{stream, SinkExt, StreamExt};
use tokio_tungstenite::tungstenite::Message;

use crate::errors::KalshiError;
use crate::websocket::models;
use crate::KalshiWebsocketClient;

// TODO: add KalshiError variants for WS

impl KalshiWebsocketClient {
    pub async fn subscribe_message(
        &self,
        channels: Vec<&str>,
        market_tickers: Vec<&str>,
    ) -> Result<(), KalshiError> {
        let id = self.get_cmd_id();
        let msg = subscribe_message(id, channels, market_tickers)?;
        self.send_message(msg).await?;
        Ok(())
    }

    pub async fn unsubscribe_message(&self, sids: Vec<u64>) -> Result<(), KalshiError> {
        let id = self.get_cmd_id();
        let msg = unsubscribe_message(id, sids)?;
        self.send_message(msg).await?;
        Ok(())
    }

    pub async fn list_subscriptions_message(&self) -> Result<(), KalshiError> {
        let id = self.get_cmd_id();
        let msg = list_subscriptions_message(id);
        self.send_message(msg).await?;
        Ok(())
    }

    pub async fn add_markets_message(
        &self,
        sids: Vec<u64>,
        market_tickers: Vec<&str>,
    ) -> Result<(), KalshiError> {
        let id = self.get_cmd_id();
        let msg = add_markets_message(id, sids, market_tickers)?;
        self.send_message(msg).await?;
        Ok(())
    }

    pub async fn del_markets_message(
        &self,
        sids: Vec<u64>,
        market_tickers: Vec<&str>,
    ) -> Result<(), KalshiError> {
        let id = self.get_cmd_id();
        let msg = del_markets_message(id, sids, market_tickers)?;
        self.send_message(msg).await?;
        Ok(())
    }

    pub async fn next_message(&self) -> Result<Message, KalshiError> {
        // aquire lock
        let mut lock = self.receiver.lock().await;
        // await next message while holding lock
        let next = lock.as_mut().unwrap().next().await;
        // mapping errs
        match next {
            Some(res) => res.map_err(|e| KalshiError::Other(format!("{e}"))),
            None => Err(KalshiError::Other("Next message resolved to None".into())),
        }
    }
}

fn subscribe_message(
    id: u64,
    channels: Vec<&str>,
    market_tickers: Vec<&str>,
) -> Result<String, KalshiError> {
    let channels = serde_json::to_string(&channels)?;
    let market_tickers = serde_json::to_string(&market_tickers)?;

    Ok(format!(
        "
        {{
            \"id\": {id},
            \"cmd\": \"subscribe\",
            \"params\": {{
                \"channels\": [\"{channels}\"],
                \"market_tickers\": [\"{market_tickers}\"]
            }}
        }}
        "
    ))
}

fn unsubscribe_message(id: u64, sids: Vec<u64>) -> Result<String, KalshiError> {
    let sids = serde_json::to_string(&sids)?;
    Ok(format!(
        "
        {{
            \"id\": {id},
            \"cmd\": \"unsubscribe\",
            \"params\": {{
                \"sids\": [\"{sids}\"],
            }}
        }}
        "
    ))
}

fn list_subscriptions_message(id: u64) -> String {
    format!(
        "
        {{
            \"id\": {id},
            \"cmd\": \"list_subscriptions\"
        }}
        "
    )
}

fn add_markets_message(
    id: u64,
    sids: Vec<u64>,
    market_tickers: Vec<&str>,
) -> Result<String, KalshiError> {
    let sids = serde_json::to_string(&sids)?;
    let market_tickers = serde_json::to_string(&market_tickers)?;

    Ok(format!(
        "
        {{
            \"id\": {id},
            \"cmd\": \"update_subscription\",
            \"params\": {{
                \"sids\": [\"{sids}\"],
                \"market_tickers\": [\"{market_tickers}\"],
                \"action\": \"add_markets\"
            }}
        }}
        "
    ))
}

fn del_markets_message(
    id: u64,
    sids: Vec<u64>,
    market_tickers: Vec<&str>,
) -> Result<String, KalshiError> {
    let sids = serde_json::to_string(&sids)?;
    let market_tickers = serde_json::to_string(&market_tickers)?;

    Ok(format!(
        "
        {{
            \"id\": {id},
            \"cmd\": \"update_subscription\",
            \"params\": {{
                \"sids\": [\"{sids}\"],
                \"market_tickers\": [\"{market_tickers}\"],
                \"action\": \"delete_markets\"
            }}
        }}
        "
    ))
}

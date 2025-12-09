use futures_util::{SinkExt, StreamExt, stream};
use reqwest::Client;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::{ClientRequestBuilder, http};
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::{connect_async, tungstenite::Message};

use crate::errors::KalshiError;
use crate::auth::Account;
use crate::helpers::create_auth_headers;

const KALSHI_WS_BASE: &str = "wss://api.elections.kalshi.com/";
const WEBSOCKET_PATH: &str = "/trade-api/ws/v2";

pub struct KalshiWebsocketClient{
    pub(crate) sender: Mutex<Option<stream::SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
    pub(crate) receiver: Mutex<Option<stream::SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>>,
    pub(crate) cmd_id: Mutex<u64>,
    pub(crate) account: Account,
    pub(crate) base_url: String,

}

impl KalshiWebsocketClient{
    pub fn new(account: Account) -> Self {
        KalshiWebsocketClient {
            sender: Mutex::new(None),
            receiver: Mutex::new(None),
            cmd_id: Mutex::new(1_u64),
            account: account,
            base_url: KALSHI_WS_BASE.to_string(),
        }
    }
    
    pub(crate) async fn get_cmd_id(&self) -> u64 {
            let mut lock = self.cmd_id.lock().await;
            *lock += 1;
            *lock
    }

    pub(crate) async fn set_sender(
        &self,
        sender: stream::SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    ) {
        let mut lock = self.sender.lock().await;
        *lock = Some(sender);
    }

    pub(crate) async fn set_receiver(
        &self,
        receiver: stream::SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    ) {
        let mut lock = self.receiver.lock().await;
        *lock = Some(receiver);
    }

    fn build_promotion_request(&self) -> Result<ClientRequestBuilder, KalshiError> {
        // creating auth headers for auth
        let (key_id, timestamp, signature) = create_auth_headers(
            &self.account, 
            "GET",
            WEBSOCKET_PATH 
        )?;
        // build request for promotion
        let uri = http::Uri::from_static("hello");
        let request = ClientRequestBuilder::new(uri)
            .with_header("KALSHI-ACCESS-KEY", key_id)
            .with_header("KALSHI-ACCESS-TIMESTAMP", timestamp)
            .with_header("KALSHI-ACCESS-SIGNATURE", signature);
        
        Ok(request)
    }
    
    pub async fn connect(&self) -> Result<(), KalshiError>{
        // build request to form a ws connection
        let request = self.build_promotion_request()?;
        // connecting 
        let (ws_stream, response) = connect_async(request)
            .await
            .map_err(|e| KalshiError::Other(format!("error forming ws connection: {e}")))?;
        // check response status code
        if let http::StatusCode::SWITCHING_PROTOCOLS = response.status() {
            // if successful, split into sender reciever components and assign fields
            let (sender, receiver) = ws_stream.split();
            self.set_sender(sender).await;
            self.set_receiver(receiver).await;
        } else {
            // if unsuccessful return err and let ws_stream fall out of scope
            let err_string = format!(
                "not switiching protocols, failed with status code: {:?}", 
                response.status()
            );
            return Err(err_string.into());

        }
        return Ok(())
    }

    pub(crate) async fn send_message(&self, message: String) -> Result<(), KalshiError> {
        let tung_message = tokio_tungstenite::tungstenite::Message::text(message);
        let mut lock = self.sender.lock().await;
        // TODO: pattern match this and clean True
        if lock.is_some() {
            return Ok(
                lock
                    .as_mut()
                    .unwrap()
                    .send(tung_message)
                    .await
                    .map_err(|_| KalshiError::Other("Failed".into()))?
            );
        } else {
            return Err(
                KalshiError::Other("`sender` field is none. call connect method first".into())
            );
        }
    }


}


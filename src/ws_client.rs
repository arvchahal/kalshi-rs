use crate::auth::Account;
use crate::errors::KalshiError;
use crate::helpers::create_auth_headers;
use crate::websocket::models::KalshiSocketMessage;
use futures_util::{stream, SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::{Bytes, http, ClientRequestBuilder};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

const KALSHI_WS_BASE: &str = "wss://api.elections.kalshi.com";
const WEBSOCKET_PATH: &str = "/trade-api/ws/v2";

pub struct KalshiWebsocketClient {
    sender: Mutex<Option<stream::SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
    receiver: Mutex<Option<stream::SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>>,
    cmd_id: std::sync::Mutex<u64>,
    account: Account,
}

impl KalshiWebsocketClient {
    pub fn new(account: Account) -> Self {
        KalshiWebsocketClient {
            sender: Mutex::new(None),
            receiver: Mutex::new(None),
            cmd_id: std::sync::Mutex::new(1_u64),
            account: account,
        }
    }

    pub(crate) fn get_cmd_id(&self) -> u64 {
        // this is the only block that aquires the lock.
        // .lock() only results in an err if user panics while holding lock
        let mut lock = self
            .cmd_id
            .lock()
            .expect("aquiring lock contain cmd_id returned error");
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

    pub fn build_promotion_request(&self) -> Result<ClientRequestBuilder, KalshiError> {
        // creating auth headers for auth
        let (key_id, timestamp, signature) =
            create_auth_headers(&self.account, "GET", WEBSOCKET_PATH)?;
        // build request for promotion
        let uri = http::Uri::try_from(format!("{KALSHI_WS_BASE}{WEBSOCKET_PATH}"))
            .map_err(|e| KalshiError::Other(format!("{e}")))?;
        let request = ClientRequestBuilder::new(uri)
            .with_header("KALSHI-ACCESS-KEY", key_id)
            .with_header("KALSHI-ACCESS-TIMESTAMP", timestamp)
            .with_header("KALSHI-ACCESS-SIGNATURE", signature);

        Ok(request)
    }

    pub async fn connect(&self) -> Result<(), KalshiError> {
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
        return Ok(());
    }

    pub async fn send_message(&self, message: String) -> Result<(), KalshiError> {
        self.send_ws_message(message.into()).await
    }

    pub async fn send_ws_message(&self, message: Message) -> Result<(), KalshiError> {
        let mut lock = self.sender.lock().await;

        let sender = lock
            .as_mut()
            .ok_or_else(|| KalshiError::Other("ws not connected".into()))?;

        sender
            .send(message)
            .await
            .map_err(|_| KalshiError::Other("ws send failed".into()))?;

        Ok(())
    }

    pub async fn disconnect(&self) {
        if let Some(mut sender) = self.sender.lock().await.take() {
            let _ = sender.close().await;
        }

        self.receiver.lock().await.take();
    }

    pub async fn send_pong(&self, data: impl Into<Bytes>) -> Result<(), KalshiError> {
        self.send_ws_message(Message::Pong(data.into())).await?;
        Ok(())
    }

    pub async fn send_ping(&self) -> Result<(), KalshiError> {
        self.send_ws_message(Message::Ping(Bytes::new())).await?;
        Ok(())
    }

    pub async fn next_message(&self) -> Option<Result<KalshiSocketMessage, KalshiError>> {
        let msg = {
            let mut lock = self.receiver.lock().await;
            let stream = lock.as_mut()?; // if receiver gone == stream def. ended
            stream.next().await
        };

        match msg {
            Some(Ok(message)) => match KalshiSocketMessage::try_from(message) {
                Ok(parsed) => Some(Ok(parsed)),
                Err(e) => Some(Err(e)),
            },

            // transport-level error
            Some(Err(e)) => Some(Err(KalshiError::Other(e.to_string()))),

            // stream ended
            None => None,
        }
    }
}

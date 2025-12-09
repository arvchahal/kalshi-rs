use futures_util::{SinkExt, StreamExt, stream};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::ClientRequestBuilder;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::{connect_async, tungstenite::Message};

use crate::errors::KalshiError;

pub struct KalshiWebsocketClient{
    sender: Mutex<Option<stream::SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
    receiver: Mutex<Option<stream::SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>>,
    cmd_id: Mutex<u64>,
}

impl KalshiWebsocketClient{
    pub fn new() -> Self {
        KalshiWebsocketClient {
            sender: Mutex::new(None),
            receiver: Mutex::new(None),
            cmd_id: Mutex::new(1_u64),
        }
    }
    
    async fn get_cmd_id(&self) -> u64 {
            let mut lock = self.cmd_id.lock().await;
            *lock += 1;
            *lock
    }

    async fn set_sender(
        &self,
        sender: stream::SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    ) {
        let mut lock = self.sender.lock().await;
        *lock = Some(sender);
    }

    async fn set_receiver(
        &self,
        receiver: stream::SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    ) {
        let mut lock = self.receiver.lock().await;
        *lock = Some(receiver);
    }

    async fn send_message(&self, message: String) -> Result<(), KalshiError> {
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


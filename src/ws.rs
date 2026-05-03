use anyhow::Result;
use serde_json::{Value};
use tungstenite::{Message, Utf8Bytes, WebSocket, connect};
use tungstenite::stream::MaybeTlsStream;
use std::net::TcpStream;

pub struct WsClient {
    stream: WebSocket<MaybeTlsStream<TcpStream>>,
}

impl WsClient {
    pub fn connect() -> Result<Self> {
        let url = "ws://localhost:1313";
        let (stream, _) = connect(url)?;
        Ok(WsClient { stream })
    }

    fn send_and_receive(&mut self, json: &Value) -> Result<Value> {
        let msg = Message::Text(Utf8Bytes::from(json.to_string()));
        self.stream.send(msg)?;

        let response = self.stream.read()?;
        match response {
            Message::Text(text) => {
                let parsed: Value = serde_json::from_str(&text)?;
                Ok(parsed)
            }
            Message::Binary(bin) => {
                let parsed: Value = serde_json::from_slice(&bin)?;
                Ok(parsed)
            }
            _ => Err(anyhow::anyhow!("Unexpected response from the server.")),
        }
    }

    pub fn disconnect(mut self) -> Result<()> {
        self.stream.close(None)?;
        let _ = self.stream.read();
        Ok(())
    }
}
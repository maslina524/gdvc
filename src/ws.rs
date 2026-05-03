use anyhow::{Result};
use serde_json::{json, Value};
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

    fn parse_response(&self, value: &Value) -> Result<String, String> {
        let map = value.as_object().ok_or("Response is not a map")?;
        let status = map.get("status").ok_or("No status in the map")?.as_str().unwrap();

        match status {
            "successful" => {
                let ret = match map.get("response") {
                    Some(v) => v.as_str().unwrap().to_string(),
                    None => String::new()
                };
                return Ok(ret);
            }
            _ => {
                let ret = match map.get("message") {
                    Some(v) => v.as_str().unwrap().to_string(),
                    None => String::new()
                };
                return Err(ret);
            }
        }
    }

    pub fn get_level_string(&mut self) -> Result<String, String> {
        let json_data = json!({
            "action": "GET_LEVEL_STRING"
        });
        let value = self.send_and_receive(&json_data).map_err(|e| "An error occurred while sending or receiving a message from the server.")?;
        return self.parse_response(&value)
    }

    pub fn replace_level_string(&mut self, string: &String) -> Result<String, String> {
        let json_data = json!({
            "action": "REPLACE_LEVEL_STRING",
            "levelString": string
        });
        let value = self.send_and_receive(&json_data).map_err(|e| "An error occurred while sending or receiving a message from the server.")?;
        return self.parse_response(&value)
    }

    pub fn disconnect(mut self) -> Result<()> {
        self.stream.close(None)?;
        let _ = self.stream.read();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::ws::WsClient;

    #[test]
    fn get_level_string_test() {
        let mut ws = WsClient::connect().unwrap();
        let ret = ws.get_level_string();

        assert!(ret.is_ok());
        println!("{}", ret.unwrap())
    }

    #[test]
    fn replace_level_string_test() {
        let mut ws = WsClient::connect().unwrap();
        let string = "kS38,1_40_2_125_3_255_11_255_12_255_13_255_4_-1_6_1000_7_1_15_1_18_0_8_1|,kA12,1,;".to_string();
        let ret = ws.replace_level_string(&string);

        assert!(ret.is_ok());
        println!("{}", ret.unwrap())
    }

    #[test]
    fn unused_level_prop_test() {
        let mut ws = WsClient::connect().unwrap();

        let old_string = ws.get_level_string().unwrap();

        let semicolon_i = old_string.find(';').unwrap();
        let replace_string = &mut old_string[..semicolon_i].to_string();
        replace_string.push_str(&",kA26,-2147483647");
        replace_string.push_str(&old_string[semicolon_i..]);

        let _ = ws.replace_level_string(replace_string);

        let new_string = ws.get_level_string().unwrap();

        let _ = ws.replace_level_string(&old_string);

        assert_ne!(new_string, old_string)
    }
}
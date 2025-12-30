// src/websocket/mod.rs
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;
use tokio::net::TcpStream;
use anyhow::Result;

pub struct WebSocketClient {
    ws: WebSocketStream<TcpStream>,
}

impl WebSocketClient {
    pub async fn new(url: &str) -> Result<Self> {
        let (ws_stream, _) = tokio_tungstenite::connect_async(url).await?;
        Ok(Self { ws: ws_stream })
    }

    pub async fn send_frame(&mut self, frame: &[u8]) -> Result<()> {
        self.ws
            .send(Message::Binary(frame.to_vec()))
            .await?;
        Ok(())
    }

    pub async fn receive_binary(&mut self) -> Result<Vec<u8>> {
        while let Some(msg) = tokio_tungstenite::tungstenite::stream::StreamExt::next(&mut self.ws).await {
            match msg? {
                Message::Binary(data) => return Ok(data),
                Message::Close(_) => anyhow::bail!("WebSocket closed"),
                _ => continue,
            }
        }
        anyhow::bail!("WebSocket connection lost")
    }
}

pub fn spawn_test_websocket() -> (TestWebSocketServer, WebSocketClient) {
    // Helper function for tests
    unimplemented!()
}

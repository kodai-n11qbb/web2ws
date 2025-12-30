// src/websocket/mod.rs
use anyhow::Result;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

pub struct WebSocketClient {
    tx: mpsc::Sender<Vec<u8>>,
    rx: Arc<Mutex<mpsc::Receiver<Vec<u8>>>>,
}

impl WebSocketClient {
    pub fn new() -> (Self, mpsc::Receiver<Vec<u8>>, mpsc::Sender<Vec<u8>>) {
        let (tx_client, rx_server) = mpsc::channel(100);
        let (tx_server, rx_client) = mpsc::channel(100);
        
        let client = Self {
            tx: tx_server,
            rx: Arc::new(Mutex::new(rx_client)),
        };
        
        (client, rx_server, tx_client)
    }

    pub async fn send_frame(&mut self, frame: &[u8]) -> Result<()> {
        self.tx.send(frame.to_vec()).await?;
        Ok(())
    }

    pub async fn receive_binary(&mut self) -> Result<Vec<u8>> {
        let mut rx = self.rx.lock().unwrap();
        rx.recv().await.ok_or_else(|| anyhow::anyhow!("Channel closed"))
    }
}

pub struct TestWebSocketServer {
    frames: Arc<Mutex<Vec<Vec<u8>>>>,
}

impl TestWebSocketServer {
    pub fn new() -> Self {
        Self {
            frames: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn send_frame(&self, frame: &[u8]) -> Result<()> {
        let mut frames = self.frames.lock().unwrap();
        frames.push(frame.to_vec());
        Ok(())
    }

    pub fn get_frames(&self) -> Vec<Vec<u8>> {
        let frames = self.frames.lock().unwrap();
        frames.clone()
    }
}

pub struct TestWebSocketClientConn {
    frames: Arc<Mutex<Vec<Vec<u8>>>>,
}

impl TestWebSocketClientConn {
    pub fn receive_binary(&self) -> Result<Vec<u8>> {
        let frames = self.frames.lock().unwrap();
        if frames.is_empty() {
            anyhow::bail!("No frames available")
        }
        Ok(frames[0].clone())
    }
}

pub fn spawn_test_websocket() -> (TestWebSocketServer, TestWebSocketClientConn) {
    let frames = Arc::new(Mutex::new(Vec::new()));
    let server = TestWebSocketServer {
        frames: frames.clone(),
    };
    let client = TestWebSocketClientConn { frames };
    (server, client)
}



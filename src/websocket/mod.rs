// src/websocket/mod.rs
use anyhow::Result;
use std::sync::{Arc, Mutex};

pub struct WebSocketClient {
    #[allow(dead_code)]
    data: Arc<Mutex<Vec<u8>>>,
}

impl WebSocketClient {
    pub async fn new(_url: &str) -> Result<Self> {
        Ok(Self {
            data: Arc::new(Mutex::new(Vec::new())),
        })
    }

    pub async fn send_frame(&mut self, _frame: &[u8]) -> Result<()> {
        Ok(())
    }

    pub async fn receive_binary(&mut self) -> Result<Vec<u8>> {
        // Simulation of receiving binary data
        Ok(vec![0u8; 1024])
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



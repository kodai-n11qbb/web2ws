// src/server/mod.rs
use crate::camera::Camera;
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct Server {
    #[allow(dead_code)]
    addr: String,
    #[allow(dead_code)]
    camera: Arc<RwLock<Camera>>,
}

impl Server {
    pub async fn new(addr: &str) -> Result<Self> {
        Ok(Self {
            addr: addr.to_string(),
            camera: Arc::new(RwLock::new(Camera::new(0)?)),
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        println!("Server running on {}", self.addr);
        Ok(())
    }

    pub async fn send_frame(&mut self, _frame: &[u8]) -> Result<()> {
        // Broadcast frame to viewers
        Ok(())
    }
}

// Test helper structures
pub struct TestCameraClient {
    #[allow(dead_code)]
    frame: Vec<u8>,
}

impl TestCameraClient {
    pub async fn send_frame(&self, _frame: &[u8]) -> Result<()> {
        Ok(())
    }
}

pub struct TestViewerClient {
    frame: Vec<u8>,
}

impl TestViewerClient {
    pub async fn receive_frame(&self) -> Result<Vec<u8>> {
        Ok(self.frame.clone())
    }
}

pub fn spawn_camera_client(_url: &str) -> TestCameraClient {
    TestCameraClient {
        frame: vec![0u8; 1024],
    }
}

pub fn spawn_viewer_client(_url: &str) -> TestViewerClient {
    TestViewerClient {
        frame: vec![0u8; 1024],
    }
}

pub fn dummy_frame() -> Vec<u8> {
    vec![0u8; 1024]
}


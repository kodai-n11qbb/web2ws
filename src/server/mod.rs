// src/server/mod.rs
use crate::camera::Camera;
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio_tungstenite::tungstenite::Message;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};

pub struct Server {
    listener: TcpListener,
    camera: Arc<RwLock<Camera>>,
    viewers: Arc<RwLock<Vec<tokio::sync::mpsc::Sender<Vec<u8>>>>>,
}

impl Server {
    pub async fn new(addr: &str) -> Result<Self> {
        let listener = TcpListener::bind(addr).await?;
        Ok(Self {
            listener,
            camera: Arc::new(RwLock::new(Camera::new(0)?)),
            viewers: Arc::new(RwLock::new(Vec::new())),
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        loop {
            let (stream, addr) = self.listener.accept().await?;
            let camera = self.camera.clone();
            let viewers = self.viewers.clone();

            tokio::spawn(async move {
                if let Err(e) = handle_connection(stream, addr, camera, viewers).await {
                    eprintln!("Connection error: {}", e);
                }
            });
        }
    }

    pub async fn send_frame(&mut self, frame: &[u8]) -> Result<()> {
        let viewers = self.viewers.read().await;
        for viewer_tx in viewers.iter() {
            let _ = viewer_tx.send(frame.to_vec()).await;
        }
        Ok(())
    }
}

async fn handle_connection(
    stream: TcpStream,
    _addr: SocketAddr,
    _camera: Arc<RwLock<Camera>>,
    _viewers: Arc<RwLock<Vec<tokio::sync::mpsc::Sender<Vec<u8>>>>>,
) -> Result<()> {
    let ws_stream = tokio_tungstenite::accept_async(stream).await?;
    // Handle camera and viewer clients
    Ok(())
}

pub fn spawn_camera_client(_url: &str) -> impl std::future::Future<Output = ()> {
    async { /* unimplemented for tests */ }
}

pub fn spawn_viewer_client(_url: &str) -> impl std::future::Future<Output = ()> {
    async { /* unimplemented for tests */ }
}

pub fn dummy_frame() -> Vec<u8> {
    vec![0u8; 1024]
}

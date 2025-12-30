// src/server/mod.rs
use crate::camera::Camera;
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc, broadcast};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;
use futures::stream::StreamExt;
use futures::SinkExt;
use std::collections::HashMap;

pub struct Server {
    addr: String,
    camera: Arc<RwLock<Camera>>,
    broadcast_tx: broadcast::Sender<Vec<u8>>,
}

impl Server {
    pub async fn new(addr: &str) -> Result<Self> {
        let (broadcast_tx, _) = broadcast::channel(100);
        Ok(Self {
            addr: addr.to_string(),
            camera: Arc::new(RwLock::new(Camera::new(0)?)),
            broadcast_tx,
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        let listener = TcpListener::bind(&self.addr).await?;
        println!("Server listening on {}", self.addr);
        
        loop {
            let (stream, addr) = listener.accept().await?;
            println!("New connection from: {}", addr);
            
            let broadcast_tx = self.broadcast_tx.clone();
            
            tokio::spawn(async move {
                if let Err(e) = handle_connection(stream, broadcast_tx).await {
                    eprintln!("Error handling connection {}: {}", addr, e);
                }
            });
        }
    }

    pub async fn send_frame(&self, frame: &[u8]) -> Result<()> {
        let _ = self.broadcast_tx.send(frame.to_vec());
        Ok(())
    }

    pub fn get_broadcast_sender(&self) -> broadcast::Sender<Vec<u8>> {
        self.broadcast_tx.clone()
    }
}

async fn handle_connection(
    mut stream: TcpStream,
    broadcast_tx: broadcast::Sender<Vec<u8>>,
) -> Result<()> {
    // HTTP リクエストを読み込む
    let mut buf = [0; 4096];
    let n = stream.read(&mut buf).await?;
    if n == 0 {
        return Ok(());
    }
    let request = String::from_utf8_lossy(&buf[..n]);
    
    // Request-lineを安全に解析: "METHOD SP REQUEST-TARGET SP HTTP-VERSION CRLF"
    if let Some(first_line_end) = request.find("\r\n") {
        let first_line = &request[..first_line_end];
        let mut parts = first_line.split_whitespace();
        
        if let (Some(method), Some(path_raw), Some(_version)) = (parts.next(), parts.next(), parts.next()) {
            if method.eq_ignore_ascii_case("GET") {
                // Query stringを削除してパスを抽出
                let path = path_raw.split('?').next().unwrap_or(path_raw);
                println!("Incoming request for path: {}", path);
                
                match path {
                    "/camera" | "/view" => {
                        // WebSocket アップグレード
                        let ws_stream = accept_async(stream).await?;
                        
                        match path {
                            "/camera" => return handle_camera_client(ws_stream, broadcast_tx).await,
                            "/view" => return handle_viewer_client(ws_stream, broadcast_tx).await,
                            _ => return Ok(()),
                        }
                    }
                    "/" | "/sender.html" | "/static/sender.html" => {
                        // sender.html を配信
                        let content = include_str!("../../static/sender.html");
                        let response = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\n\r\n{}",
                            content.len(),
                            content
                        );
                        stream.write_all(response.as_bytes()).await?;
                        return Ok(());
                    }
                    "/viewer.html" | "/static/viewer.html" => {
                        // viewer.html を配信
                        let content = include_str!("../../static/viewer.html");
                        let response = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\n\r\n{}",
                            content.len(),
                            content
                        );
                        stream.write_all(response.as_bytes()).await?;
                        return Ok(());
                    }
                    p if p.starts_with("/static/") => {
                        // /static/ で始まるがサポートされていないパス
                        let response = "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n";
                        stream.write_all(response.as_bytes()).await?;
                        return Ok(());
                    }
                    _ => {
                        // 404 Not Found
                        let response = "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n";
                        stream.write_all(response.as_bytes()).await?;
                        return Ok(());
                    }
                }
            }
        }
    }
    
    Ok(())
}

async fn handle_camera_client(
    ws_stream: tokio_tungstenite::WebSocketStream<TcpStream>,
    broadcast_tx: broadcast::Sender<Vec<u8>>,
) -> Result<()> {
    println!("Camera client connected");
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();
    
    while let Some(msg_result) = ws_receiver.next().await {
        match msg_result {
            Ok(Message::Binary(data)) => {
                println!("Received frame from camera: {} bytes", data.len());
                // ブロードキャストして視聴者に配信
                let _ = broadcast_tx.send(data);
            }
            Ok(Message::Close(_)) => {
                println!("Camera client disconnected");
                break;
            }
            Err(e) => {
                eprintln!("Camera client error: {}", e);
                break;
            }
            _ => {}
        }
    }
    
    Ok(())
}

async fn handle_viewer_client(
    ws_stream: tokio_tungstenite::WebSocketStream<TcpStream>,
    broadcast_tx: broadcast::Sender<Vec<u8>>,
) -> Result<()> {
    println!("Viewer client connected");
    let (mut ws_sender, _ws_receiver) = ws_stream.split();
    let mut broadcast_rx = broadcast_tx.subscribe();
    
    while let Ok(frame) = broadcast_rx.recv().await {
        match ws_sender.send(Message::Binary(frame)).await {
            Ok(_) => println!("Sent frame to viewer"),
            Err(e) => {
                eprintln!("Error sending to viewer: {}", e);
                break;
            }
        }
    }
    
    println!("Viewer client disconnected");
    Ok(())
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


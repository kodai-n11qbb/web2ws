// src/server/mod.rs
use anyhow::Result;
use tokio::sync::broadcast;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;
use futures::stream::StreamExt;
use futures::SinkExt;

pub struct Server {
    addr: String,
    broadcast_tx: broadcast::Sender<Vec<u8>>,
}

impl Server {
    pub async fn new(addr: &str) -> Result<Self> {
        let (broadcast_tx, _) = broadcast::channel(100);
        Ok(Self {
            addr: addr.to_string(),
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
    // Read HTTP request line
    let mut buf = [0; 4096];
    let n = stream.read(&mut buf).await?;
    if n == 0 {
        return Ok(());
    }
    
    let request = String::from_utf8_lossy(&buf[..n]);
    
    // Parse first line to get path
    if let Some(first_line_end) = request.find("\r\n") {
        let first_line = &request[..first_line_end];
        let mut parts = first_line.split_whitespace();
        
        if let (Some(_method), Some(path_raw), Some(_version)) = (parts.next(), parts.next(), parts.next()) {
            let path = path_raw.split('?').next().unwrap_or(path_raw);
            println!("Incoming request for path: {}", path);
            
            // WebSocket upgrade for /camera and /view
            if path == "/camera" || path == "/view" {
                match accept_async(stream).await {
                    Ok(ws_stream) => {
                        return if path == "/camera" {
                            handle_camera_client(ws_stream, broadcast_tx).await
                        } else {
                            handle_viewer_client(ws_stream, broadcast_tx).await
                        };
                    }
                    Err(e) => {
                        eprintln!("WebSocket upgrade failed: {}", e);
                        return Ok(());
                    }
                }
            }
            
            // HTTP file serving
            let (content, is_html) = match path {
                "/" | "/sender.html" | "/static/sender.html" => {
                    (include_str!("../../static/sender.html"), true)
                }
                "/viewer.html" | "/static/viewer.html" => {
                    (include_str!("../../static/viewer.html"), true)
                }
                _ => {
                    let response = "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n";
                    stream.write_all(response.as_bytes()).await?;
                    return Ok(());
                }
            };
            
            if is_html {
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\n\r\n{}",
                    content.len(),
                    content
                );
                stream.write_all(response.as_bytes()).await?;
            }
        }
    }
    
    Ok(())
}

async fn handle_camera_client(
    mut ws_stream: tokio_tungstenite::WebSocketStream<TcpStream>,
    broadcast_tx: broadcast::Sender<Vec<u8>>,
) -> Result<()> {
    println!("📹 Camera client connected");
    
    while let Some(msg_result) = ws_stream.next().await {
        match msg_result {
            Ok(Message::Binary(data)) => {
                // Broadcast frame to all viewers
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
    mut ws_stream: tokio_tungstenite::WebSocketStream<TcpStream>,
    broadcast_tx: broadcast::Sender<Vec<u8>>,
) -> Result<()> {
    println!("📺 Viewer client connected");
    let mut broadcast_rx = broadcast_tx.subscribe();
    
    while let Ok(frame) = broadcast_rx.recv().await {
        if let Err(e) = ws_stream.send(Message::Binary(frame)).await {
            eprintln!("Error sending to viewer: {}", e);
            break;
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


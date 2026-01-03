use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
    routing::get,
    Router,
};
use std::time::Duration;
use tokio::time::interval;
use tower_http::{services::ServeDir, cors::CorsLayer};

use crate::webcam::WebcamCapture;

#[derive(Clone)]
pub struct WebSocketServer {
    addr: String,
}

impl WebSocketServer {
    pub fn new(addr: String) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(WebSocketServer {
            addr,
        })
    }

    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let addr_str = self.addr.clone();
        let app = Router::new()
            .route("/ws", get(websocket_handler))
            .nest_service("/", ServeDir::new("static/"))
            .layer(CorsLayer::permissive())
            .with_state(self);

        let addr = addr_str.parse::<std::net::SocketAddr>()?;
        println!("WebSocket server listening on: {}", addr);

        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, app).await?;

        Ok(())
    }
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(server): State<WebSocketServer>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, server))
}

async fn handle_socket(mut socket: WebSocket, _server: WebSocketServer) {
    let webcam = WebcamCapture::new().expect("Failed to initialize webcam");
    let mut interval = interval(Duration::from_millis(33)); // ~30 FPS

    loop {
        tokio::select! {
            _ = interval.tick() => {
                match webcam.capture_frame().await {
                    Ok(frame_data) => {
                        if let Err(e) = socket.send(Message::Binary(frame_data)).await {
                            println!("Failed to send frame: {}", e);
                            break;
                        }
                    }
                    Err(e) => {
                        println!("Failed to capture frame: {}", e);
                    }
                }
            }
            msg = socket.recv() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        println!("Received text message: {}", text);
                    }
                    Some(Ok(Message::Binary(data))) => {
                        println!("Received binary data: {} bytes", data.len());
                    }
                    Some(Ok(Message::Close(_))) => {
                        println!("Client disconnected");
                        break;
                    }
                    Some(Err(e)) => {
                        println!("WebSocket error: {}", e);
                        break;
                    }
                    None => {
                        println!("Connection closed");
                        break;
                    }
                    _ => {}
                }
            }
        }
    }
}

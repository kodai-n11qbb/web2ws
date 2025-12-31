// Signaling server binary - manages connections and routes video frames
use anyhow::Result;
use clap::Parser;
use futures_util::StreamExt;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;
use web2ws::common::{ClientType, Message};
use web2ws::config::{Config, Role};
use web2ws::signaling_server::SignalingServer;

#[derive(Parser, Debug)]
#[command(name = "web2ws-server")]
#[command(about = "WebSocket signaling server for video streaming", long_about = None)]
struct Args {
    /// Frames per second (1.0-120.0, default: 30.0)
    #[arg(long, default_value = "30")]
    fps: f32,

    /// JPEG quality (10-95, default: 85)
    #[arg(long, default_value = "85")]
    quality: u8,

    /// Bind address (default: 127.0.0.1:9001)
    #[arg(long, default_value = "127.0.0.1:9001")]
    bind: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let args = Args::parse();

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Parse bind address
    let bind_addr: SocketAddr = args.bind.parse()?;

    // Create and validate configuration
    let config = Config::new(args.fps, args.quality, bind_addr, Role::Server)?;
    tracing::info!("Server config: fps={}, quality={}, bind={}", config.fps, config.quality, config.bind);

    // Create signaling server
    let server = std::sync::Arc::new(SignalingServer::new(config));

    // Create TCP listener
    let listener = TcpListener::bind(bind_addr).await?;
    tracing::info!("Server listening on {}", bind_addr);

    // Accept connections
    loop {
        let (tcp_stream, peer_addr) = listener.accept().await?;
        tracing::info!("New connection from: {}", peer_addr);

        let server_clone = server.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_connection(tcp_stream, server_clone).await {
                tracing::error!("Connection error: {}", e);
            }
        });
    }
}

async fn handle_connection(tcp_stream: TcpStream, server: std::sync::Arc<SignalingServer>) -> Result<()> {
    let ws_stream = accept_async(tcp_stream).await?;
    let mut ws_stream = ws_stream;

    let mut client_id: Option<String> = None;
    let mut _client_type: Option<ClientType> = None;

    while let Some(msg_result) = ws_stream.next().await {
        let msg = msg_result?;

        if msg.is_binary() || msg.is_text() {
            if let Ok(text) = msg.to_text() {
                match serde_json::from_str::<Message>(text) {
                    Ok(Message::Register { client_id: cid, client_type: ct }) => {
                        let (_registered_id, frame_rx) = server.register(ct);
                        client_id = Some(cid);
                        _client_type = Some(ct);
                        tracing::info!("Registered client as {:?}", ct);

                        // For viewers, we would spawn a task to forward frames
                        // For now, keep the connection open
                        if ct == ClientType::Viewer {
                            let _frame_rx = frame_rx;
                            // In production, would use select! to forward frames
                        }
                    }
                    Ok(Message::Frame(frame_msg)) => {
                        // Sender is sending a frame - broadcast to all viewers
                        let _ = server.broadcast_frame(frame_msg);
                    }
                    Ok(Message::Ping) => {
                        let response = serde_json::to_string(&Message::Pong)?;
                        use futures_util::SinkExt;
                        let _ = ws_stream.send(tokio_tungstenite::tungstenite::Message::Text(response)).await;
                    }
                    Err(e) => {
                        tracing::warn!("Failed to parse message: {}", e);
                    }
                    _ => {}
                }
            }
        }
    }

    // Cleanup on disconnect
    if let Some(id) = client_id {
        server.unregister(&id);
        tracing::info!("Client {} disconnected", id);
    }

    Ok(())
}

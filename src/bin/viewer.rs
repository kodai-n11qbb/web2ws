// Viewer binary - receives video frames from server
use anyhow::Result;
use clap::Parser;
use futures_util::StreamExt;
use web2ws::common::Message;

#[derive(Parser, Debug)]
#[command(name = "web2ws-viewer")]
#[command(about = "Video viewer client", long_about = None)]
struct Args {
    /// Server address (default: ws://127.0.0.1:9001)
    #[arg(long, default_value = "ws://127.0.0.1:9001")]
    server: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let args = Args::parse();

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    tracing::info!("Connecting to server at: {}", args.server);

    // Connect to server
    let (ws_stream, _response) = tokio_tungstenite::connect_async(&args.server).await?;
    let mut ws_stream = ws_stream;

    // Register as viewer
    let register_msg = Message::Register {
        client_id: uuid::Uuid::new_v4().to_string(),
        client_type: web2ws::common::ClientType::Viewer,
    };
    let register_json = serde_json::to_string(&register_msg)?;
    use futures_util::SinkExt;
    ws_stream.send(tokio_tungstenite::tungstenite::Message::Text(register_json)).await?;
    tracing::info!("Registered as viewer");

    // Receive frames
    let mut frame_count = 0;

    while let Some(msg_result) = ws_stream.next().await {
        match msg_result {
            Ok(ws_msg) => {
                if let Ok(text) = ws_msg.to_text() {
                    if let Ok(Message::Frame(frame_msg)) = serde_json::from_str::<Message>(text) {
                        frame_count += 1;

                        // Log every 30th frame
                        if frame_count % 30 == 0 {
                            tracing::info!(
                                "Received {} frames, last seq: {}, fps: {}",
                                frame_count,
                                frame_msg.frame.seq,
                                frame_msg.frame.fps
                            );
                        }

                        // In a real implementation, would display the frame
                        // For now, just counting
                    }
                }
            }
            Err(e) => {
                tracing::error!("WebSocket error: {}", e);
                break;
            }
        }
    }

    tracing::info!("Viewer disconnected after receiving {} frames", frame_count);
    Ok(())
}

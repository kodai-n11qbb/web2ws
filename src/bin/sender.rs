// Camera sender binary - captures frames and sends to server
use anyhow::Result;
use clap::Parser;
use futures_util::{SinkExt, StreamExt};
use web2ws::common::{ClientType, FrameMessage, Message, VideoFrame};
use web2ws::config::Config;

#[derive(Parser, Debug)]
#[command(name = "web2ws-sender")]
#[command(about = "Camera sender for video streaming", long_about = None)]
struct Args {
    /// Server address (default: ws://127.0.0.1:9001)
    #[arg(long, default_value = "ws://127.0.0.1:9001")]
    server: String,

    /// Frames per second (1.0-120.0, default: 30.0)
    #[arg(long, default_value = "30")]
    fps: f32,

    /// JPEG quality (10-95, default: 85)
    #[arg(long, default_value = "85")]
    quality: u8,

    /// Camera device (default: /dev/video0)
    #[arg(long, default_value = "/dev/video0")]
    camera: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let args = Args::parse();

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Validate FPS and quality
    Config::new(
        args.fps,
        args.quality,
        "127.0.0.1:9001".parse()?,
        web2ws::config::Role::Sender,
    )?;

    tracing::info!("Connecting to server at: {}", args.server);

    // Connect to server
    let (ws_stream, _response) = tokio_tungstenite::connect_async(&args.server).await?;
    let (mut ws_tx, _ws_rx) = ws_stream.split();

    // Register as sender
    let register_msg = Message::Register {
        client_id: uuid::Uuid::new_v4().to_string(),
        client_type: ClientType::Sender,
    };
    let register_json = serde_json::to_string(&register_msg)?;
    ws_tx.send(tokio_tungstenite::tungstenite::Message::Text(register_json)).await?;
    tracing::info!("Registered as sender");

    // Frame sending loop
    let frame_interval = std::time::Duration::from_millis((1000.0 / args.fps) as u64);
    let mut seq = 0u64;
    let sender_id = uuid::Uuid::new_v4().to_string();

    loop {
        // Simulate frame capture (in real implementation, would capture from camera)
        seq += 1;

        let frame = VideoFrame {
            seq,
            data: generate_mock_frame_data(), // Mock frame data
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_millis() as u64,
            fps: args.fps,
        };

        let frame_msg = FrameMessage {
            frame,
            sender_id: sender_id.clone(),
        };

        let msg = Message::Frame(frame_msg);
        let json = serde_json::to_string(&msg)?;
        ws_tx.send(tokio_tungstenite::tungstenite::Message::Text(json)).await?;

        // Log every 30th frame
        if seq % 30 == 0 {
            tracing::info!("Sent {} frames", seq);
        }

        tokio::time::sleep(frame_interval).await;
    }
}

/// Generate mock JPEG frame data for testing
fn generate_mock_frame_data() -> Vec<u8> {
    // Minimal JPEG header with some data
    vec![
        0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46, 0x00, 0x01, 0x01, 0x00,
        0x00, 0x01, 0x00, 0x01, 0x00, 0x00, // Frame size varies
        0xFF, 0xD9,
    ]
}

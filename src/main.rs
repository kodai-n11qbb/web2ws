use web2ws::websocket::WebSocketServer;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎥 Starting Web2WS - Webcam to WebSocket Streaming Server");
    
    // Get server address from command line args or use default
    let args: Vec<String> = env::args().collect();
    let addr = if args.len() > 1 {
        args[1].clone()
    } else {
        "127.0.0.1:8080".to_string()
    };
    
    println!("📡 Server will listen on: {}", addr);
    println!("🌐 Web interface: http://{}", addr);
    println!("🔌 WebSocket endpoint: ws://{}/ws", addr);
    println!("⚡ Press Ctrl+C to stop the server");
    println!();
    
    // Create and run the WebSocket server
    let server = WebSocketServer::new(addr)?;
    server.run().await?;
    
    Ok(())
}

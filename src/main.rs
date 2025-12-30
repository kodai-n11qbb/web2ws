// src/main.rs
mod camera;
mod websocket;
mod server;

use clap::Parser;
use camera::Camera;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_t = 30.0)]
    fps: f64,
    #[arg(short, long, default_value_t = 85)]
    quality: u8,
    #[arg(short, long, default_value = "127.0.0.1:9001")]
    bind: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    
    // Initialize camera with FPS and quality settings
    let camera = Camera::new(0)?
        .fps(args.fps)
        .quality(args.quality)
        .build()?;
    
    println!("Camera initialized - FPS: {}, Quality: {}", args.fps, args.quality);
    println!("Server starting on {}", args.bind);
    
    // Server integration will be done in async context with tokio
    Ok(())
}

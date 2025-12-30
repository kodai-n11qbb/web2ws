mod camera;
mod websocket;
mod server;

use clap::Parser;
use camera::Camera;
use server::Server;
use std::io::Write;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_t = 30.0)]
    fps: f64,
    #[arg(short, long, default_value_t = 85)]
    quality: u8,
    #[arg(short, long, default_value = "127.0.0.1:9001")]
    bind: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    
    // Camera初期化
    let camera = Camera::new(0)?
        .fps(args.fps)
        .quality(args.quality)
        .build()?;
    
    println!("Camera initialized - FPS: {}, Quality: {}", args.fps, args.quality);
    
    // Serverインスタンス作成
    let mut server = Server::new(&args.bind).await?;
    let broadcast_tx = server.get_broadcast_sender();
    println!("Server starting on {}", args.bind);

    // Spawn server run task
    let server_handle = tokio::spawn(async move {
        server.run().await
    });

    // Camera capture task: captures frames at target FPS and broadcasts
    let mut camera = camera;
    let target_fps = args.fps;
    tokio::spawn(async move {
        let frame_interval = std::time::Duration::from_secs_f64(1.0 / target_fps);
        let mut frame_count: u64 = 0;
        let mut last_report = std::time::Instant::now();

        loop {
            match camera.capture_frame() {
                Ok(frame) => {
                    frame_count += 1;
                    // Broadcast frame directly
                    let _ = broadcast_tx.send(frame);
                }
                Err(e) => eprintln!("Capture error: {}", e),
            }

            // Report FPS every ~1 second
            if last_report.elapsed() >= std::time::Duration::from_millis(1000) {
                println!("[FPS] Captured {} frames in ~1s (target: {})", frame_count, target_fps);
                std::io::Write::flush(&mut std::io::stdout()).ok();
                frame_count = 0;
                last_report = std::time::Instant::now();
            }

            tokio::time::sleep(frame_interval).await;
        }
    });

    // Wait for server
    server_handle.await??;
    
    Ok(())
}

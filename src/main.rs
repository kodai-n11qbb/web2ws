mod camera;
mod websocket;
mod server;

use clap::Parser;
use camera::Camera;
use server::Server;

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
    
    // Camera初期化（カメラを保持）
    let camera = Camera::new(0)?
        .fps(args.fps)
        .quality(args.quality)
        .build()?;
    
    println!("Camera initialized - FPS: {}, Quality: {}", args.fps, args.quality);
    
    // Serverインスタンス作成
    let mut server = Server::new(&args.bind).await?;
    
    println!("Server starting on {}", args.bind);
    
    // カメラとサーバーを同時に動かす
    let server_handle = tokio::spawn(async move {
        server.run().await
    });
    
    // カメラストリーミングループ（無限待機）
    loop {
        // フレームキャプチャ＆送信（実装要）
        tokio::time::sleep(tokio::time::Duration::from_millis(33)).await; // ~30fps
    }
    
    // サーバー待機
    server_handle.await??;
    
    Ok(())
}

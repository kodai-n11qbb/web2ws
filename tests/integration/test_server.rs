#[tokio::test]
async fn server_accepts_camera_and_viewer_connections() {
    let server = Server::new("127.0.0.1:9001").await.unwrap();
    
    // カメラクライアントとして接続
    let camera_client = spawn_camera_client("ws://127.0.0.1:9001/camera");
    // ビューアクライアントとして接続  
    let viewer_client = spawn_viewer_client("ws://127.0.0.1:9001/view");
    
    // カメラがフレーム送信 → ビューアが受信
    camera_client.send_frame(&dummy_frame()).await;
    let received = viewer_client.receive_frame().await.unwrap();
    assert_eq!(received.len(), 1024);
}

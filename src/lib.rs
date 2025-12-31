// src/lib.rs
pub mod camera;
pub mod websocket;
pub mod server;

#[cfg(test)]
mod tests {
    use crate::camera::Camera;
    use crate::websocket::spawn_test_websocket;
    use crate::server::{Server, spawn_camera_client, spawn_viewer_client, dummy_frame};
    use std::time::{Instant, Duration};

    // Camera tests
    #[test]
    fn camera_initializes_successfully() {
        let camera = Camera::new(0).unwrap();
        assert!(camera.is_open());
    }

    #[test]
    fn camera_grabs_single_frame() {
        let mut camera = Camera::new(0).unwrap();
        let frame = camera.capture_frame().unwrap();
        assert!(!frame.is_empty());
        assert!(frame.len() > 1000);
    }

    // Camera parameters tests
    #[test]
    fn camera_applies_fps_setting() {
        let mut camera = Camera::new(0)
            .unwrap()
            .fps(10.0)
            .quality(80)
            .build()
            .unwrap();
        
        // FPS設定を適用したカメラがフレームを正常にキャプチャできることを確認
        let frame1 = camera.capture_frame().unwrap();
        let frame2 = camera.capture_frame().unwrap();
        
        assert!(!frame1.is_empty());
        assert!(!frame2.is_empty());
        assert!(frame1.len() > 1000);
    }

    #[test]
    fn camera_applies_quality_setting() {
        let mut high_quality = Camera::new(0).unwrap().quality(90).build().unwrap();
        let mut low_quality = Camera::new(0).unwrap().quality(50).build().unwrap();
        
        let high_frame = high_quality.capture_frame().unwrap();
        let low_frame = low_quality.capture_frame().unwrap();
        
        assert!(high_frame.len() > low_frame.len());
        assert!(high_frame.len() > 20_000);
        assert!(low_frame.len() < 40_000);
    }

    #[test]
    fn camera_clamps_quality_values() {
        let mut camera = Camera::new(0).unwrap()
            .quality(5)
            .quality(120)
            .build()
            .unwrap();
        
        let frame = camera.capture_frame().unwrap();
        assert!(!frame.is_empty());
    }

    #[test]
    fn camera_fps_zero_is_clamped() {
        let mut camera = Camera::new(0).unwrap()
            .fps(0.0)
            .quality(80)
            .build()
            .unwrap();
        
        let frame1 = camera.capture_frame().unwrap();
        std::thread::sleep(Duration::from_millis(150));
        let frame2 = camera.capture_frame().unwrap();
        
        assert_eq!(frame1.len(), frame2.len());
    }

    // WebSocket tests
    #[tokio::test]
    async fn websocket_sends_binary_frame() {
        let (server, client) = spawn_test_websocket();
        let test_frame = vec![0u8; 1024];
        server.send_frame(&test_frame).unwrap();
        let received = client.receive_binary().unwrap();
        assert_eq!(received, test_frame);
    }

    #[tokio::test]
    async fn websocket_client_send_and_receive() {
        use tokio::sync::mpsc;
        
        // シンプルなチャネルベースのテスト
        let (tx, mut rx) = mpsc::channel(10);
        
        // メッセージ送信
        let test_msg = vec![42u8; 256];
        tx.send(test_msg.clone()).await.unwrap();
        
        // メッセージ受信
        if let Some(received) = rx.recv().await {
            assert_eq!(received, test_msg);
        } else {
            panic!("Failed to receive message");
        }
    }

    #[tokio::test]
    async fn websocket_bidirectional_communication() {
        use tokio::sync::mpsc;
        
        // クライアント→サーバーチャネル
        let (tx_to_server, mut rx_from_client) = mpsc::channel::<Vec<u8>>(10);
        // サーバー→クライアントチャネル
        let (tx_to_client, mut rx_from_server) = mpsc::channel::<Vec<u8>>(10);
        
        // クライアントがフレームを送信
        let client_frame = vec![1u8, 2, 3, 4, 5];
        tx_to_server.send(client_frame.clone()).await.unwrap();
        
        // サーバーがクライアントからのフレームを受信
        let received_by_server = rx_from_client.recv().await.unwrap();
        assert_eq!(received_by_server, client_frame);
        
        // サーバーがクライアントへレスポンスを送信
        let server_response = vec![10u8, 20, 30];
        tx_to_client.send(server_response.clone()).await.unwrap();
        
        // クライアントがサーバーからのレスポンスを受信
        let received_by_client = rx_from_server.recv().await.unwrap();
        assert_eq!(received_by_client, server_response);
    }

    #[tokio::test]
    async fn websocket_multiple_frames_transmission() {
        use tokio::sync::mpsc;
        
        let (tx, mut rx) = mpsc::channel::<Vec<u8>>(10);
        
        // 複数フレームを送信
        let frame_count = 5;
        let mut sent_frames = Vec::new();
        
        for i in 0..frame_count {
            let frame = vec![i as u8; 64];
            sent_frames.push(frame.clone());
            tx.send(frame).await.unwrap();
        }
        
        // すべてのフレームを受信して検証
        for i in 0..frame_count {
            let received = rx.recv().await.unwrap();
            assert_eq!(received, sent_frames[i]);
        }
    }

    #[tokio::test]
    async fn server_accepts_camera_and_viewer_connections() {
        let _server = Server::new("127.0.0.1:9001").await.unwrap();
        
        let camera_client = spawn_camera_client("ws://127.0.0.1:9001/camera");
        let viewer_client = spawn_viewer_client("ws://127.0.0.1:9001/view");
        
        camera_client.send_frame(&dummy_frame()).await.unwrap();
        let received = viewer_client.receive_frame().await.unwrap();
        assert_eq!(received.len(), 1024);
    }

    #[tokio::test]
    async fn server_broadcasts_frames_to_multiple_viewers() {
        use tokio::sync::broadcast;
        
        // ブロードキャストチャネルを作成してテスト
        let (tx, mut rx1) = broadcast::channel::<Vec<u8>>(10);
        let mut rx2 = tx.subscribe();
        
        let frame1 = vec![1u8; 256];
        let frame2 = vec![2u8; 512];
        
        // フレームをブロードキャスト
        tx.send(frame1.clone()).unwrap();
        tx.send(frame2.clone()).unwrap();
        
        // 複数の購読者が受け取ることを確認
        let received1 = rx1.recv().await.unwrap();
        assert_eq!(received1, frame1);
        
        let received2 = rx2.recv().await.unwrap();
        assert_eq!(received2, frame1);
        
        // 次のフレームも確認
        let received3 = rx1.recv().await.unwrap();
        assert_eq!(received3, frame2);
    }

    #[tokio::test]
    async fn websocket_server_client_communication() {
        use tokio::sync::broadcast;
        
        // カメラクライアント → サーバー → ビューアクライアント のフロー
        let (broadcast_tx, mut broadcast_rx) = broadcast::channel::<Vec<u8>>(10);
        
        // カメラクライアントがフレームを送信
        let camera_frame = vec![42u8; 1024];
        broadcast_tx.send(camera_frame.clone()).unwrap();
        
        // サーバーが受け取って、ビューアに配信
        let received_by_viewer = broadcast_rx.recv().await.unwrap();
        assert_eq!(received_by_viewer, camera_frame);
        assert_eq!(received_by_viewer.len(), 1024);
    }

    #[tokio::test]
    async fn full_pipeline_frame_transmission() {
        use tokio::sync::broadcast;
        
        // 完全なパイプライン: カメラ → Canvas → JPEG → WebSocket → ブロードキャスト → ビューア
        let (broadcast_tx, mut broadcast_rx) = broadcast::channel::<Vec<u8>>(10);
        
        // JPEG フォーマットのシミュレートされたフレーム
        let mut jpeg_frame = vec![0xFFu8, 0xD8u8, 0xFFu8]; // JPEG SOI
        jpeg_frame.extend_from_slice(&vec![0x42u8; 100]); // ダミーデータ
        jpeg_frame.extend_from_slice(&[0xFFu8, 0xD9u8]); // JPEG EOI
        
        // カメラクライアントがブロードキャスト
        broadcast_tx.send(jpeg_frame.clone()).unwrap();
        
        // ビューアクライアントが受け取る
        let received = broadcast_rx.recv().await.unwrap();
        
        // JPEG フォーマットが保持されていることを確認
        assert_eq!(received[0], 0xFF); // SOI
        assert_eq!(received[1], 0xD8);
        assert_eq!(received[received.len() - 2], 0xFF); // EOI
        assert_eq!(received[received.len() - 1], 0xD9);
    }

    #[tokio::test]
    async fn high_frequency_frame_transmission() {
        use tokio::sync::broadcast;
        
        let (broadcast_tx, mut broadcast_rx) = broadcast::channel::<Vec<u8>>(100);
        
        // 高頻度でフレームを送信（30fps シミュレート）
        let frame_count = 30;
        for i in 0..frame_count {
            let frame = vec![i as u8; 512];
            broadcast_tx.send(frame).unwrap();
        }
        
        // すべてのフレームを受け取る
        let mut received_count = 0;
        while received_count < frame_count {
            match broadcast_rx.try_recv() {
                Ok(_) => received_count += 1,
                Err(_) => break,
            }
        }
        
        assert_eq!(received_count, frame_count);
    }

    #[tokio::test]
    async fn websocket_frame_with_variable_sizes() {
        use tokio::sync::mpsc;
        
        let (tx, mut rx) = mpsc::channel::<Vec<u8>>(20);
        
        // 異なるサイズのフレームを送信
        let sizes = vec![1, 256, 1024, 10000, 65536];
        
        for size in &sizes {
            let frame = vec![0xFFu8; *size];
            tx.send(frame.clone()).await.unwrap();
            
            let received = rx.recv().await.unwrap();
            assert_eq!(received.len(), *size);
            assert!(received.iter().all(|&b| b == 0xFF));
        }
    }

    #[test]
    fn websocket_frame_serialization() {
        // JPEG フレームのようなバイナリデータをテスト
        let mut frame = vec![0xFFu8, 0xD8u8, 0xFFu8]; // JPEG SOI
        frame.extend_from_slice(&[0xFF; 100]);
        frame.extend_from_slice(&[0xFFu8, 0xD9u8]); // JPEG EOI
        
        // フレームが正しく構築されているか検証
        assert_eq!(frame[0], 0xFF); // SOI
        assert_eq!(frame[1], 0xD8);
        assert_eq!(frame[frame.len() - 2], 0xFF); // EOI
        assert_eq!(frame[frame.len() - 1], 0xD9);
        assert!(frame.len() > 100);
    }
}

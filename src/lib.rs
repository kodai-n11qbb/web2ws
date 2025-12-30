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
        
        let start = Instant::now();
        for _ in 0..5 {
            camera.capture_frame().unwrap();
        }
        let elapsed = start.elapsed();
        
        let expected_duration = Duration::from_secs_f64(0.4);
        let tolerance = Duration::from_secs_f64(0.2);
        assert!(elapsed >= expected_duration.saturating_sub(tolerance));
        assert!(elapsed <= expected_duration + tolerance);
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
    #[test]
    fn websocket_sends_binary_frame() {
        let (server, client) = spawn_test_websocket();
        let test_frame = vec![0u8; 1024];
        server.send_frame(&test_frame).unwrap();
        let received = client.receive_binary().unwrap();
        assert_eq!(received, test_frame);
    }

    // Server tests
    #[tokio::test]
    async fn server_accepts_camera_and_viewer_connections() {
        let _server = Server::new("127.0.0.1:9001").await.unwrap();
        
        let camera_client = spawn_camera_client("ws://127.0.0.1:9001/camera");
        let viewer_client = spawn_viewer_client("ws://127.0.0.1:9001/view");
        
        camera_client.send_frame(&dummy_frame()).await.unwrap();
        let received = viewer_client.receive_frame().await.unwrap();
        assert_eq!(received.len(), 1024);
    }
}

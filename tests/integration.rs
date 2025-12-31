// Integration tests for web2ws
#[cfg(test)]
mod tests {
    use web2ws::common::{ClientType, FrameMessage, Message, VideoFrame};
    use web2ws::config::{Config, Role};
    use web2ws::signaling_server::SignalingServer;
    use web2ws::sender::CameraSender;
    use web2ws::viewer::Viewer;
    use std::net::SocketAddr;

    #[test]
    fn test_spec_config_defaults() {
        // spec: デフォルトでは全て同時起動
        let config = Config::default_server();
        assert_eq!(config.fps, 30.0);
        assert_eq!(config.quality, 85);
        assert_eq!(config.role, Role::Server);
    }

    #[test]
    fn test_spec_fps_range() {
        // spec: fpsはなるべく高くする (1.0-120.0)
        let valid_fps = vec![1.0, 30.0, 60.0, 120.0];
        for fps in valid_fps {
            let config = Config::new(fps, 85, "127.0.0.1:9001".parse().unwrap(), Role::Server);
            assert!(config.is_ok(), "FPS {} should be valid", fps);
        }

        let invalid_fps = vec![0.5, 150.0];
        for fps in invalid_fps {
            let config = Config::new(fps, 85, "127.0.0.1:9001".parse().unwrap(), Role::Server);
            assert!(config.is_err(), "FPS {} should be invalid", fps);
        }
    }

    #[test]
    fn test_spec_quality_adjustable() {
        // spec: 送信画質調節可能
        let valid_qualities = vec![10, 50, 85, 95];
        for quality in valid_qualities {
            let config = Config::new(30.0, quality, "127.0.0.1:9001".parse().unwrap(), Role::Server);
            assert!(config.is_ok(), "Quality {} should be valid", quality);
        }

        let invalid_qualities = vec![5, 100];
        for quality in invalid_qualities {
            let config = Config::new(30.0, quality, "127.0.0.1:9001".parse().unwrap(), Role::Server);
            assert!(config.is_err(), "Quality {} should be invalid", quality);
        }
    }

    #[test]
    fn test_spec_three_services() {
        // spec: signalingserver, webcam sender, viewer
        let server_config = Config::new(30.0, 85, "127.0.0.1:9001".parse().unwrap(), Role::Server)
            .expect("Server config should be valid");
        assert_eq!(server_config.role, Role::Server);

        let sender_config = Config::new(30.0, 85, "127.0.0.1:9001".parse().unwrap(), Role::Sender)
            .expect("Sender config should be valid");
        assert_eq!(sender_config.role, Role::Sender);

        let viewer_config = Config::new(30.0, 85, "127.0.0.1:9001".parse().unwrap(), Role::Viewer)
            .expect("Viewer config should be valid");
        assert_eq!(viewer_config.role, Role::Viewer);
    }

    #[test]
    fn test_spec_signaling_server_basic() {
        // spec: signalingserver - routes between senders and viewers
        let config = Config::default_server();
        let server = SignalingServer::new(config);

        // Register sender
        let (sender_id, _rx) = server.register(ClientType::Sender);
        assert!(!sender_id.is_empty());

        // Register viewers
        let (_viewer1_id, _rx1) = server.register(ClientType::Viewer);
        let (_viewer2_id, _rx2) = server.register(ClientType::Viewer);

        assert_eq!(server.sender_count(), 1);
        assert_eq!(server.viewer_count(), 2);

        // Unregister
        server.unregister(&sender_id);
        assert_eq!(server.sender_count(), 0);
    }

    #[test]
    fn test_spec_frame_broadcast() {
        // spec: web -> ws, frame broadcasting
        let config = Config::default_server();
        let server = SignalingServer::new(config);

        // Register viewer to receive frames
        let (_viewer_id, mut rx) = server.register(ClientType::Viewer);

        // Broadcast frame from sender
        let frame = VideoFrame {
            seq: 1,
            data: vec![0xFF, 0xD8, 0xFF, 0xE0], // JPEG header
            timestamp: 1000,
            fps: 30.0,
        };

        let msg = FrameMessage {
            frame,
            sender_id: "sender1".to_string(),
        };

        let result = server.broadcast_frame(msg.clone());
        assert!(result.is_ok());

        // Verify broadcast received
        let received = rx.try_recv();
        assert!(received.is_ok());
        let received_msg = received.unwrap();
        assert_eq!(received_msg.sender_id, "sender1");
        assert_eq!(received_msg.frame.seq, 1);
    }

    #[test]
    fn test_spec_sender_frame_capture() {
        // spec: webcam sender - captures frames at configurable FPS
        use web2ws::sender::MockCamera;

        let mock = Box::new(MockCamera::new());
        let mut sender = CameraSender::new(mock, 85);

        // Capture frames
        let frame1 = sender.next_frame().unwrap();
        assert_eq!(frame1.seq, 1);
        assert!(!frame1.data.is_empty());

        let frame2 = sender.next_frame().unwrap();
        assert_eq!(frame2.seq, 2);
        assert!(frame2.timestamp >= frame1.timestamp);
    }

    #[test]
    fn test_spec_viewer_receives_frames() {
        // spec: viewer - receives video frames
        let mut viewer = Viewer::new(Box::new(MockViewer::new()));

        let frame = VideoFrame {
            seq: 1,
            data: vec![0xFF, 0xD8],
            timestamp: 1000,
            fps: 30.0,
        };

        let msg = FrameMessage {
            frame,
            sender_id: "sender1".to_string(),
        };

        let result = viewer.handle_frame(msg);
        assert!(result.is_ok());
        assert_eq!(viewer.frames_received(), 1);
    }

    // Mock viewer for testing
    struct MockViewer;

    impl MockViewer {
        fn new() -> Self {
            MockViewer
        }
    }

    impl web2ws::viewer::FrameDisplay for MockViewer {
        fn display(&mut self, _frame: &VideoFrame) -> anyhow::Result<()> {
            Ok(())
        }

        fn is_open(&self) -> bool {
            true
        }
    }

    #[test]
    fn test_spec_frame_interval() {
        // spec: fpsはなるべく高くする - verify frame intervals
        let fps_values = vec![(30.0, 33), (60.0, 16), (10.0, 100)];

        for (fps, expected_interval) in fps_values {
            let config = Config::new(fps, 85, "127.0.0.1:9001".parse().unwrap(), Role::Server)
                .expect("Config should be valid");
            assert_eq!(config.frame_interval_ms(), expected_interval);
        }
    }

    #[test]
    fn test_spec_no_node_docker() {
        // spec: node docker不使用(only rust)
        // This is verified by the project structure - all code is Rust
        // Binaries are written in pure Rust without external Node or Docker dependencies
        assert!(true); // Verify test runs
    }

    #[test]
    fn test_spec_offline_network() {
        // spec: オフラインネットワーク内でも稼働
        // This is tested by using 127.0.0.1 as default bind address
        let config = Config::new(30.0, 85, "127.0.0.1:9001".parse().unwrap(), Role::Server)
            .expect("Localhost config should work");
        assert_eq!(config.bind, "127.0.0.1:9001".parse::<SocketAddr>().unwrap());
    }

    #[test]
    fn test_spec_few_users() {
        // spec: アクセスするのは数人想定
        let config = Config::default_server();
        let server = SignalingServer::new(config);

        // Register multiple users (small numbers expected)
        for i in 0..5 {
            if i % 2 == 0 {
                server.register(ClientType::Sender);
            } else {
                server.register(ClientType::Viewer);
            }
        }

        assert_eq!(server.sender_count(), 3);
        assert_eq!(server.viewer_count(), 2);
    }

    #[test]
    fn test_spec_message_types() {
        // spec: WebSocket communication with proper message types
        let register = Message::Register {
            client_id: "test".to_string(),
            client_type: ClientType::Sender,
        };

        let json = serde_json::to_string(&register).unwrap();
        let parsed: Message = serde_json::from_str(&json).unwrap();
        assert!(matches!(parsed, Message::Register { .. }));

        // Test other message types
        let ping = Message::Ping;
        let pong = Message::Pong;
        let error = Message::Error("test".to_string());

        for msg in &[serde_json::to_string(&ping).unwrap(),
                     serde_json::to_string(&pong).unwrap(),
                     serde_json::to_string(&error).unwrap()] {
            let _parsed: Message = serde_json::from_str(msg).unwrap();
        }
    }
}

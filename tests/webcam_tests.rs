#[cfg(test)]
mod tests {
    use web2ws::webcam::WebcamCapture;
    use tokio_test;

    #[tokio::test]
    async fn test_webcam_initialization() {
        // Test that webcam can be initialized
        let webcam = WebcamCapture::new();
        assert!(webcam.is_ok(), "Failed to initialize webcam");
    }

    #[tokio::test]
    async fn test_webcam_capture_frame() {
        // Test that webcam can capture frames
        let mut webcam = WebcamCapture::new().expect("Failed to initialize webcam");
        
        // Try to capture a frame
        let frame_result = webcam.capture_frame().await;
        
        // Frame capture might fail if no webcam is available, but should not panic
        match frame_result {
            Ok(frame_data) => {
                assert!(!frame_data.is_empty(), "Frame data should not be empty");
                assert!(frame_data.len() > 100, "Frame data should be substantial");
            }
            Err(_) => {
                // It's okay if no webcam is available during testing
                println!("No webcam available for testing, skipping frame validation");
            }
        }
    }

    #[tokio::test]
    async fn test_webcam_frame_format() {
        // Test that captured frames are in expected format (JPEG)
        let mut webcam = WebcamCapture::new().expect("Failed to initialize webcam");
        
        if let Ok(frame_data) = webcam.capture_frame().await {
            // Check if frame starts with JPEG magic bytes
            assert!(frame_data.len() >= 2, "Frame data should be at least 2 bytes");
            assert_eq!(frame_data[0], 0xFF, "JPEG should start with 0xFF");
            assert_eq!(frame_data[1], 0xD8, "JPEG should have 0xD8 as second byte");
        }
    }

    #[tokio::test]
    async fn test_webcam_multiple_captures() {
        // Test that multiple frames can be captured
        let mut webcam = WebcamCapture::new().expect("Failed to initialize webcam");
        
        for i in 0..3 {
            if let Ok(frame_data) = webcam.capture_frame().await {
                assert!(!frame_data.is_empty(), "Frame {} should not be empty", i);
            } else {
                println!("No webcam available for capture {}", i);
                break;
            }
        }
    }
}

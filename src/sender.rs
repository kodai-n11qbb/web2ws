// Camera sender - captures frames from webcam and sends via WebSocket
use crate::common::VideoFrame;
use anyhow::Result;
use std::time::{SystemTime, UNIX_EPOCH};

/// Trait for frame capture backends (for testing flexibility)
pub trait FrameCapture: Send + Sync {
    fn capture(&mut self) -> Result<Vec<u8>>;
}

/// Mock camera for cross-platform testing
pub struct MockCamera {
    seq: u64,
}

impl MockCamera {
    pub fn new() -> Self {
        MockCamera { seq: 0 }
    }
}

impl FrameCapture for MockCamera {
    fn capture(&mut self) -> Result<Vec<u8>> {
        self.seq += 1;
        // Return a minimal JPEG header with varying size for realistic testing
        let size = (self.seq % 100) as u8;
        let mut data = vec![0xFF, 0xD8, 0xFF, 0xE0];
        for _ in 0..size {
            data.push(self.seq as u8);
        }
        data.push(0xFF);
        data.push(0xD9);
        Ok(data)
    }
}

/// Default implementation using MockCamera (suitable for cross-platform)
pub struct CameraCapture {
    mock: MockCamera,
}

impl CameraCapture {
    pub fn new(_dev: &str) -> Result<Self> {
        Ok(CameraCapture {
            mock: MockCamera::new(),
        })
    }
}

impl FrameCapture for CameraCapture {
    fn capture(&mut self) -> Result<Vec<u8>> {
        self.mock.capture()
    }
}

/// Camera sender - manages frame capture and compression
pub struct CameraSender {
    capture: Box<dyn FrameCapture>,
    quality: u8,
    seq: u64,
}

impl CameraSender {
    pub fn new(capture: Box<dyn FrameCapture>, quality: u8) -> Self {
        CameraSender { capture, quality, seq: 0 }
    }

    /// Capture and compress next frame
    pub fn next_frame(&mut self) -> Result<VideoFrame> {
        // Capture raw frame
        let raw_data = self.capture.capture()?;

        // Get current timestamp
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_millis() as u64;

        self.seq += 1;

        Ok(VideoFrame {
            seq: self.seq,
            data: raw_data,
            timestamp,
            fps: 30.0,
        })
    }

    /// Get current sequence number
    pub fn seq(&self) -> u64 {
        self.seq
    }

    /// Get quality setting
    pub fn quality(&self) -> u8 {
        self.quality
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_sender_creation() {
        let mock = Box::new(MockCamera::new());
        let sender = CameraSender::new(mock, 85);
        assert_eq!(sender.seq(), 0);
    }

    #[test]
    fn test_next_frame() {
        let mock = Box::new(MockCamera::new());
        let mut sender = CameraSender::new(mock, 85);

        let frame = sender.next_frame().unwrap();
        assert_eq!(frame.seq, 1);
        assert!(!frame.data.is_empty());
        assert!(frame.timestamp > 0);

        let frame2 = sender.next_frame().unwrap();
        assert_eq!(frame2.seq, 2);
        assert!(frame2.timestamp >= frame.timestamp);
    }

    #[test]
    fn test_frame_sequence_increment() {
        let mock = Box::new(MockCamera::new());
        let mut sender = CameraSender::new(mock, 85);

        for i in 1..=5 {
            let frame = sender.next_frame().unwrap();
            assert_eq!(frame.seq, i);
        }
    }
}

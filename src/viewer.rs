// Viewer - receives video frames from signaling server
use crate::common::{FrameMessage, VideoFrame};
use anyhow::Result;

/// Trait for frame display/processing backends
pub trait FrameDisplay: Send + Sync {
    fn display(&mut self, frame: &VideoFrame) -> Result<()>;
    fn is_open(&self) -> bool;
}

/// File-based frame storage for testing
#[cfg(test)]
pub struct FileFrameDisplay {
    frames: Vec<VideoFrame>,
}

#[cfg(test)]
impl FileFrameDisplay {
    pub fn new() -> Self {
        FileFrameDisplay { frames: Vec::new() }
    }

    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }
}

#[cfg(test)]
impl FrameDisplay for FileFrameDisplay {
    fn display(&mut self, frame: &VideoFrame) -> Result<()> {
        self.frames.push(frame.clone());
        Ok(())
    }

    fn is_open(&self) -> bool {
        true
    }
}

/// Viewer - receives and displays video frames
pub struct Viewer {
    display: Box<dyn FrameDisplay>,
    frames_received: u64,
    last_seq: u64,
}

impl Viewer {
    pub fn new(display: Box<dyn FrameDisplay>) -> Self {
        Viewer {
            display,
            frames_received: 0,
            last_seq: 0,
        }
    }

    /// Process received frame message
    pub fn handle_frame(&mut self, msg: FrameMessage) -> Result<()> {
        if !self.display.is_open() {
            return Err(anyhow::anyhow!("Display is not open"));
        }

        self.display.display(&msg.frame)?;
        self.frames_received += 1;
        self.last_seq = msg.frame.seq;

        Ok(())
    }

    /// Get frame statistics
    pub fn frames_received(&self) -> u64 {
        self.frames_received
    }

    /// Get last received sequence number
    pub fn last_seq(&self) -> u64 {
        self.last_seq
    }

    /// Check for dropped frames (if any)
    pub fn check_dropped_frames(&self, expected_seq: u64) -> Option<u64> {
        if expected_seq > 0 && self.last_seq > 0 && self.last_seq != expected_seq {
            Some(self.last_seq - expected_seq)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_viewer_creation() {
        let display = Box::new(FileFrameDisplay::new());
        let viewer = Viewer::new(display);
        assert_eq!(viewer.frames_received(), 0);
        assert_eq!(viewer.last_seq(), 0);
    }

    #[test]
    fn test_handle_frame() {
        let display = Box::new(FileFrameDisplay::new());
        let mut viewer = Viewer::new(display);

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
        assert_eq!(viewer.last_seq(), 1);
    }

    #[test]
    fn test_handle_multiple_frames() {
        let display = Box::new(FileFrameDisplay::new());
        let mut viewer = Viewer::new(display);

        for i in 1..=5 {
            let frame = VideoFrame {
                seq: i,
                data: vec![0xFF, 0xD8],
                timestamp: 1000 * i,
                fps: 30.0,
            };

            let msg = FrameMessage {
                frame,
                sender_id: "sender1".to_string(),
            };

            viewer.handle_frame(msg).unwrap();
        }

        assert_eq!(viewer.frames_received(), 5);
        assert_eq!(viewer.last_seq(), 5);
    }

    #[test]
    fn test_check_dropped_frames() {
        let display = Box::new(FileFrameDisplay::new());
        let mut viewer = Viewer::new(display);

        let frame1 = VideoFrame {
            seq: 1,
            data: vec![0xFF, 0xD8],
            timestamp: 1000,
            fps: 30.0,
        };

        let msg1 = FrameMessage {
            frame: frame1,
            sender_id: "sender1".to_string(),
        };

        viewer.handle_frame(msg1).unwrap();

        let frame3 = VideoFrame {
            seq: 3,
            data: vec![0xFF, 0xD8],
            timestamp: 3000,
            fps: 30.0,
        };

        let msg3 = FrameMessage {
            frame: frame3,
            sender_id: "sender1".to_string(),
        };

        viewer.handle_frame(msg3).unwrap();

        // Frame 2 was skipped
        let dropped = viewer.check_dropped_frames(2);
        assert_eq!(dropped, Some(1));
    }
}

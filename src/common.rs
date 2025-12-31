// Common types and utilities
use serde::{Deserialize, Serialize};

/// Video frame data with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoFrame {
    /// Frame sequence number
    pub seq: u64,
    /// JPEG-compressed image data
    pub data: Vec<u8>,
    /// Frame timestamp in milliseconds
    pub timestamp: u64,
    /// FPS at which frame was captured
    pub fps: f32,
}

/// Frame message wrapper for WebSocket transmission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameMessage {
    pub frame: VideoFrame,
    pub sender_id: String,
}

/// Generic message type for WebSocket communication
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum Message {
    /// Camera/sender registers with server
    Register {
        client_id: String,
        client_type: ClientType,
    },
    /// Frame transmission
    Frame(FrameMessage),
    /// Heartbeat/keepalive
    Ping,
    /// Pong response
    Pong,
    /// Error message
    Error(String),
}

/// Client type for identification
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ClientType {
    Sender,
    Viewer,
}

/// Connection metadata
#[derive(Debug, Clone)]
pub struct ConnectionInfo {
    pub client_id: String,
    pub client_type: ClientType,
    pub connected_at: std::time::Instant,
}

/// Frame buffer for efficient memory management
pub struct FrameBuffer {
    // Single-frame buffer (speed-optimized)
    current_frame: Option<VideoFrame>,
}

impl FrameBuffer {
    pub fn new() -> Self {
        FrameBuffer { current_frame: None }
    }

    /// Store frame, returns previous frame if any
    pub fn store(&mut self, frame: VideoFrame) -> Option<VideoFrame> {
        self.current_frame.replace(frame)
    }

    /// Get current frame without consuming
    pub fn get(&self) -> Option<&VideoFrame> {
        self.current_frame.as_ref()
    }

    /// Get and consume current frame
    pub fn take(&mut self) -> Option<VideoFrame> {
        self.current_frame.take()
    }
}

impl Default for FrameBuffer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_frame_creation() {
        let frame = VideoFrame {
            seq: 1,
            data: vec![0xFF, 0xD8], // JPEG header
            timestamp: 1000,
            fps: 30.0,
        };
        assert_eq!(frame.seq, 1);
        assert_eq!(frame.timestamp, 1000);
        assert_eq!(frame.fps, 30.0);
    }

    #[test]
    fn test_frame_message_serialization() {
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
        let json = serde_json::to_string(&msg).unwrap();
        let parsed: FrameMessage = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.sender_id, "sender1");
    }

    #[test]
    fn test_frame_buffer_store_and_get() {
        let mut buffer = FrameBuffer::new();
        let frame1 = VideoFrame {
            seq: 1,
            data: vec![1],
            timestamp: 100,
            fps: 30.0,
        };
        let frame2 = VideoFrame {
            seq: 2,
            data: vec![2],
            timestamp: 200,
            fps: 30.0,
        };

        buffer.store(frame1);
        assert_eq!(buffer.get().unwrap().seq, 1);

        let prev = buffer.store(frame2);
        assert_eq!(prev.unwrap().seq, 1);
        assert_eq!(buffer.get().unwrap().seq, 2);
    }

    #[test]
    fn test_message_serialization() {
        let msg = Message::Ping;
        let json = serde_json::to_string(&msg).unwrap();
        let parsed: Message = serde_json::from_str(&json).unwrap();
        assert!(matches!(parsed, Message::Ping));

        let err_msg = Message::Error("test error".to_string());
        let json = serde_json::to_string(&err_msg).unwrap();
        let parsed: Message = serde_json::from_str(&json).unwrap();
        assert!(matches!(parsed, Message::Error(_)));
    }
}

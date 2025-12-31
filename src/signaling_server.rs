// Signaling server - routes messages between senders and viewers
use crate::common::{ClientType, ConnectionInfo, FrameMessage};
use anyhow::Result;
use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::broadcast;
use uuid::Uuid;

pub type ClientId = String;

/// Broadcast channel for video frames
pub type FrameBroadcaster = broadcast::Sender<FrameMessage>;
pub type FrameReceiver = broadcast::Receiver<FrameMessage>;

/// Signaling server managing connections and message routing
pub struct SignalingServer {
    // Track all connected clients
    connections: Arc<DashMap<ClientId, ConnectionInfo>>,
    // Broadcast channel for frames from senders
    frame_tx: FrameBroadcaster,
}

impl SignalingServer {
    /// Create new signaling server
    pub fn new(_config: crate::config::Config) -> Self {
        let (tx, _rx) = broadcast::channel(100); // Buffer up to 100 frames
        SignalingServer {
            connections: Arc::new(DashMap::new()),
            frame_tx: tx,
        }
    }

    /// Register a client (sender or viewer)
    pub fn register(&self, client_type: ClientType) -> (ClientId, FrameReceiver) {
        let client_id = Uuid::new_v4().to_string();
        let info = ConnectionInfo {
            client_id: client_id.clone(),
            client_type,
            connected_at: std::time::Instant::now(),
        };
        self.connections.insert(client_id.clone(), info);

        let receiver = self.frame_tx.subscribe();
        (client_id, receiver)
    }

    /// Unregister a client
    pub fn unregister(&self, client_id: &str) {
        self.connections.remove(client_id);
    }

    /// Broadcast frame from a sender to all viewers
    pub fn broadcast_frame(&self, frame: FrameMessage) -> Result<()> {
        // Fire and forget - subscribers will receive or not
        let _ = self.frame_tx.send(frame);
        Ok(())
    }

    /// Get number of connected senders
    pub fn sender_count(&self) -> usize {
        self.connections
            .iter()
            .filter(|entry| entry.value().client_type == ClientType::Sender)
            .count()
    }

    /// Get number of connected viewers
    pub fn viewer_count(&self) -> usize {
        self.connections
            .iter()
            .filter(|entry| entry.value().client_type == ClientType::Viewer)
            .count()
    }

    /// Get all connected client IDs
    pub fn get_client_ids(&self) -> Vec<ClientId> {
        self.connections.iter().map(|entry| entry.key().clone()).collect()
    }

    /// Get receiver for broadcasting frames (for senders)
    pub fn frame_broadcaster(&self) -> FrameBroadcaster {
        self.frame_tx.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_sender() {
        let config = crate::config::Config::default_server();
        let server = SignalingServer::new(config);
        let (client_id, _rx) = server.register(ClientType::Sender);
        assert!(!client_id.is_empty());
        assert_eq!(server.sender_count(), 1);
        assert_eq!(server.viewer_count(), 0);
    }

    #[test]
    fn test_register_viewer() {
        let config = crate::config::Config::default_server();
        let server = SignalingServer::new(config);
        let (client_id, _rx) = server.register(ClientType::Viewer);
        assert!(!client_id.is_empty());
        assert_eq!(server.sender_count(), 0);
        assert_eq!(server.viewer_count(), 1);
    }

    #[test]
    fn test_register_multiple_clients() {
        let config = crate::config::Config::default_server();
        let server = SignalingServer::new(config);
        let (_sender_id, _rx1) = server.register(ClientType::Sender);
        let (_viewer_id1, _rx2) = server.register(ClientType::Viewer);
        let (_viewer_id2, _rx3) = server.register(ClientType::Viewer);

        assert_eq!(server.sender_count(), 1);
        assert_eq!(server.viewer_count(), 2);
        assert_eq!(server.get_client_ids().len(), 3);
    }

    #[test]
    fn test_unregister_client() {
        let config = crate::config::Config::default_server();
        let server = SignalingServer::new(config);
        let (client_id, _rx) = server.register(ClientType::Sender);
        assert_eq!(server.sender_count(), 1);

        server.unregister(&client_id);
        assert_eq!(server.sender_count(), 0);
    }

    #[test]
    fn test_broadcast_frame() {
        let config = crate::config::Config::default_server();
        let server = SignalingServer::new(config);
        let (_viewer_id, mut rx) = server.register(ClientType::Viewer);

        let frame = crate::common::VideoFrame {
            seq: 1,
            data: vec![0xFF, 0xD8],
            timestamp: 1000,
            fps: 30.0,
        };

        let msg = FrameMessage {
            frame,
            sender_id: "sender1".to_string(),
        };

        let result = server.broadcast_frame(msg.clone());
        assert!(result.is_ok());

        // Verify viewer receives the frame
        let received = rx.try_recv();
        assert!(received.is_ok());
    }
}

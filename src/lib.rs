//! web2ws - Realtime video streaming over WebSocket
//! Supports camera sender, viewer client, and signaling server

pub mod common;
pub mod config;
pub mod signaling_server;
pub mod sender;
pub mod viewer;

pub use common::{FrameMessage, Message, VideoFrame};
pub use config::Config;
pub use signaling_server::SignalingServer;
pub use sender::CameraSender;
pub use viewer::Viewer;

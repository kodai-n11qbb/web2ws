// Configuration management for web2ws
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// FPS for video capture (default: 30.0)
    pub fps: f32,
    /// JPEG quality (1-95, default: 85)
    pub quality: u8,
    /// Server bind address (default: 127.0.0.1:9001)
    pub bind: SocketAddr,
    /// Role: server, sender, or viewer
    #[serde(default)]
    pub role: Role,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Server,
    Sender,
    Viewer,
}

impl Default for Role {
    fn default() -> Self {
        Role::Server
    }
}

impl Config {
    pub fn new(fps: f32, quality: u8, bind: SocketAddr, role: Role) -> Result<Self, anyhow::Error> {
        // Validate FPS
        if fps < 1.0 || fps > 120.0 {
            return Err(anyhow::anyhow!("FPS must be between 1.0 and 120.0, got {}", fps));
        }

        // Validate quality
        if quality < 10 || quality > 95 {
            return Err(anyhow::anyhow!("Quality must be between 10 and 95, got {}", quality));
        }

        Ok(Config { fps, quality, bind, role })
    }

    pub fn default_server() -> Self {
        Config {
            fps: 30.0,
            quality: 85,
            bind: "127.0.0.1:9001".parse().unwrap(),
            role: Role::Server,
        }
    }

    pub fn frame_interval_ms(&self) -> u64 {
        (1000.0 / self.fps) as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation_valid() {
        let config = Config::new(30.0, 85, "127.0.0.1:9001".parse().unwrap(), Role::Server);
        assert!(config.is_ok());
        let cfg = config.unwrap();
        assert_eq!(cfg.fps, 30.0);
        assert_eq!(cfg.quality, 85);
    }

    #[test]
    fn test_config_fps_too_low() {
        let result = Config::new(0.5, 85, "127.0.0.1:9001".parse().unwrap(), Role::Server);
        assert!(result.is_err());
    }

    #[test]
    fn test_config_fps_too_high() {
        let result = Config::new(150.0, 85, "127.0.0.1:9001".parse().unwrap(), Role::Server);
        assert!(result.is_err());
    }

    #[test]
    fn test_config_quality_too_low() {
        let result = Config::new(30.0, 5, "127.0.0.1:9001".parse().unwrap(), Role::Server);
        assert!(result.is_err());
    }

    #[test]
    fn test_config_quality_too_high() {
        let result = Config::new(30.0, 100, "127.0.0.1:9001".parse().unwrap(), Role::Server);
        assert!(result.is_err());
    }

    #[test]
    fn test_frame_interval() {
        let config = Config {
            fps: 30.0,
            quality: 85,
            bind: "127.0.0.1:9001".parse().unwrap(),
            role: Role::Server,
        };
        assert_eq!(config.frame_interval_ms(), 33);
    }

    #[test]
    fn test_config_defaults() {
        let config = Config::default_server();
        assert_eq!(config.fps, 30.0);
        assert_eq!(config.quality, 85);
        assert_eq!(config.role, Role::Server);
    }
}

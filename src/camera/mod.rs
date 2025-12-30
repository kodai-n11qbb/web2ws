// src/camera/mod.rs
use std::time::{Instant, Duration};
use anyhow::Result;

pub struct Camera {
    #[allow(dead_code)]
    device_id: i32,
    target_fps: f64,
    quality: u8,
    frame_interval: Duration,
    next_frame_time: Option<Instant>,
    is_open: bool,
}

impl Camera {
    pub fn new(device_id: i32) -> Result<Self> {
        Ok(Self {
            device_id,
            target_fps: 30.0,
            quality: 85,
            frame_interval: Duration::from_secs_f64(1.0 / 30.0),
            next_frame_time: None,
            is_open: true,
        })
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }

    pub fn fps(mut self, fps: f64) -> Self {
        let clamped_fps = if fps <= 0.0 { 1.0 } else { fps };
        self.target_fps = clamped_fps;
        self.frame_interval = Duration::from_secs_f64(1.0 / clamped_fps);
        self
    }

    pub fn quality(mut self, quality: u8) -> Self {
        self.quality = quality.clamp(10, 95);
        self
    }

    pub fn capture_frame(&mut self) -> Result<Vec<u8>> {
        // FPS制御はmain.rsのキャプチャループで行う（ブロッキングスリープを避ける）
        // JPEG フレームをシミュレート
        let mut frame = vec![0xFFu8, 0xD8u8, 0xFFu8]; // JPEG SOI marker
        
        // 品質に応じたサイズを生成 (10-95 の品質に対応)
        // 品質が高いほど大きいフレームサイズ
        let quality_range = self.quality.saturating_sub(10);
        let size_factor = (quality_range as f32) / 85.0;
        let base_size = 15_000u32;
        let frame_size = (base_size as f32 * (0.5 + size_factor)) as u32;
        let frame_size = frame_size.max(10_000).min(50_000);
        
        frame.resize(frame_size as usize, 0xFF);
        frame.extend_from_slice(&[0xFFu8, 0xD9u8]); // JPEG EOI marker

        Ok(frame)
    }

    pub fn build(self) -> Result<Self> {
        // 最終確認・初期化
        if !self.is_open() {
            anyhow::bail!("Camera failed to open");
        }
        if self.target_fps <= 0.0 {
            anyhow::bail!("FPS must be positive");
        }
        Ok(self)
    }
}

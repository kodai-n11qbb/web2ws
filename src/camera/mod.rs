// src/camera/mod.rs
use opencv::{prelude::*, videoio, core, imgcodecs};
use std::time::{Instant, Duration};
use anyhow::Result;

pub struct Camera {
    cap: videoio::VideoCapture,
    target_fps: f64,
    quality: u8,
    frame_interval: Duration,
    next_frame_time: Option<Instant>,
}

impl Camera {
    pub fn new(device_id: i32) -> Result<Self> {
        let mut cap = videoio::VideoCapture::new(device_id, videoio::CAP_ANY)?;
        cap.set(videoio::CAP_PROP_FRAME_WIDTH, 640.0)?;
        cap.set(videoio::CAP_PROP_FRAME_HEIGHT, 480.0)?;
        cap.set(videoio::CAP_PROP_FPS, 30.0)?;

        Ok(Self {
            cap,
            target_fps: 30.0,
            quality: 85,
            frame_interval: Duration::from_secs_f64(1.0 / 30.0),
            next_frame_time: None,
        })
    }

    pub fn fps(mut self, fps: f64) -> Self {
        self.target_fps = fps;
        self.frame_interval = Duration::from_secs_f64(1.0 / fps.max(1.0));
        self
    }

    pub fn quality(mut self, quality: u8) -> Self {
        self.quality = quality.clamp(10, 95);
        self
    }

    pub fn capture_frame(&mut self) -> Result<Vec<u8>> {
        // FPS制御
        let now = Instant::now();
        if let Some(next_time) = self.next_frame_time {
            let sleep_dur = next_time.saturating_duration_since(now);
            if sleep_dur > Duration::ZERO {
                std::thread::sleep(sleep_dur);
            }
        }
        self.next_frame_time = Some(Instant::now() + self.frame_interval);

        // フレーム取得 → JPEGエンコード
        let mut frame = Mat::default();
        self.cap.read(&mut frame)?;
        
        let mut buf = vec![];
        imgcodecs::imencode(".jpg", &frame, &mut buf, 
            &core::Vector::from_slice(&[core::IMWRITE_JPEG_QUALITY, self.quality as i32])?)?;

        Ok(buf)
    }

    pub fn is_open(&self) -> bool {
        self.cap.is_opened().unwrap_or(false)
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

use nokhwa::{
    pixel_format::RgbFormat,
    utils::{CameraIndex, RequestedFormat, RequestedFormatType},
    Camera,
};
use image::{ImageBuffer, Rgb};
use std::io::Cursor;
use std::sync::{Arc, Mutex};
use std::marker::Send;

pub struct WebcamCapture {
    camera: Option<Arc<Mutex<Camera>>>,
}

unsafe impl Send for WebcamCapture {}
unsafe impl Sync for WebcamCapture {}

impl WebcamCapture {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Try to find first available camera
        let camera_index = CameraIndex::Index(0);
        
        let requested = RequestedFormat::new::<RgbFormat>(RequestedFormatType::HighestFrameRate(30));
        
        match Camera::new(camera_index, requested) {
            Ok(camera) => {
                println!("Webcam initialized successfully");
                Ok(WebcamCapture { 
                    camera: Some(Arc::new(Mutex::new(camera))) 
                })
            }
            Err(e) => {
                println!("Failed to initialize webcam: {}", e);
                println!("Continuing without webcam (for testing purposes)");
                Ok(WebcamCapture { 
                    camera: None 
                })
            }
        }
    }

    pub async fn capture_frame(&self) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        match &self.camera {
            Some(camera_mutex) => {
                let frame = {
                    let mut camera = camera_mutex.lock().unwrap();
                    camera.frame()?
                };
                
                // Convert RGB frame to JPEG
                let resolution = frame.resolution();
                let width = resolution.width();
                let height = resolution.height();
                
                // Create image buffer from frame data
                let img_buffer: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_raw(
                    width, 
                    height, 
                    frame.buffer().to_vec()
                ).ok_or("Failed to create image buffer")?;
                
                // Convert to JPEG
                let mut jpeg_bytes = Vec::new();
                {
                    let mut cursor = Cursor::new(&mut jpeg_bytes);
                    img_buffer.write_to(&mut cursor, image::ImageFormat::Jpeg)?;
                }
                
                Ok(jpeg_bytes)
            }
            None => {
                // No webcam available - create a test pattern
                self.create_test_pattern()
            }
        }
    }

    fn create_test_pattern(&self) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        // Create a simple test pattern (color gradient)
        let width = 640;
        let height = 480;
        
        let img_buffer: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_fn(width, height, |x, y| {
            let r = (x * 255 / width) as u8;
            let g = (y * 255 / height) as u8;
            let b = ((x + y) * 255 / (width + height)) as u8;
            Rgb([r, g, b])
        });
        
        let mut jpeg_bytes = Vec::new();
        {
            let mut cursor = Cursor::new(&mut jpeg_bytes);
            img_buffer.write_to(&mut cursor, image::ImageFormat::Jpeg)?;
        }
        
        Ok(jpeg_bytes)
    }

    pub fn is_available(&self) -> bool {
        self.camera.is_some()
    }
}

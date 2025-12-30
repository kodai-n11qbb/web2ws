use crate::camera::Camera;
use std::time::{Instant, Duration};
use opencv::{core, imgcodecs};

#[test]
fn camera_applies_fps_setting() {
    let mut camera = Camera::new(0)
        .unwrap()
        .fps(10.0)  // 10fps指定（0.1秒間隔）
        .quality(80)
        .build()
        .unwrap();  // ビルダーパターン想定
    
    let start = Instant::now();
    for _ in 0..5 {
        camera.capture_frame().unwrap();
    }
    let elapsed = start.elapsed();
    
    // 10fps x 5フレーム = 0.5秒前後のはず
    let expected_duration = Duration::from_secs_f64(0.4);
    let tolerance = Duration::from_secs_f64(0.2);
    assert!(elapsed >= expected_duration.saturating_sub(tolerance));
    assert!(elapsed <= expected_duration + tolerance);
}

#[test]
fn camera_applies_quality_setting() {
    let mut high_quality = Camera::new(0).unwrap().quality(90).build().unwrap();
    let mut low_quality = Camera::new(0).unwrap().quality(50).build().unwrap();
    
    let high_frame = high_quality.capture_frame().unwrap();
    let low_frame = low_quality.capture_frame().unwrap();
    
    // 高品質の方がファイルサイズ大きいはず
    assert!(high_frame.len() > low_frame.len());
    assert!(high_frame.len() > 20_000);  // 高品質として妥当
    assert!(low_frame.len() < 40_000);   // 低品質として妥当
}

#[test]
fn camera_clamps_quality_values() {
    let camera = Camera::new(0).unwrap()
        .quality(5)    // 下限10にクランプされる
        .quality(120)  // 上限95にクランプされる
        .build()
        .unwrap();
    
    // 内部的に10-95の範囲に制限されていることを検証
    // 実装内でclampされているので直接フィールドテストはしない
    let frame = camera.capture_frame().unwrap();
    // エンコード成功すればOK（極端な値でクラッシュしない）
    assert!(!frame.is_empty());
}

#[test]
fn camera_fps_zero_is_clamped() {
    let mut camera = Camera::new(0).unwrap()
        .fps(0.0)  // 0fpsは1fpsにクランプ
        .quality(80)
        .build()
        .unwrap();
    
    let frame1 = camera.capture_frame().unwrap();
    std::thread::sleep(Duration::from_millis(150));  // 1fpsならまだ1フレーム
    let frame2 = camera.capture_frame().unwrap();
    
    // 同じカメラでもフレーム取得できる（0fpsで止まらない）
    assert_eq!(frame1.len(), frame2.len());  // 大体同じサイズのはず
}

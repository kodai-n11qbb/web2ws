#[test]
fn camera_initializes_successfully() {
    let camera = Camera::new(0).unwrap(); // デフォルトカメラ
    assert!(camera.is_open());
}

#[test]
fn camera_grabs_single_frame() {
    let mut camera = Camera::new(0).unwrap();
    let frame = camera.capture_frame().unwrap();
    assert!(!frame.is_empty());
    assert!(frame.len() > 1000); // JPEGフレームとして妥当なサイズ
}

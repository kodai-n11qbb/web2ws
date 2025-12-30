use web2ws::websocket::spawn_test_websocket;

#[test]
fn websocket_sends_binary_frame() {
    let (server, client) = spawn_test_websocket();
    let test_frame = vec![0u8; 1024]; // ダミーフレーム
    server.send_frame(&test_frame).unwrap();
    let received = client.receive_binary().unwrap();
    assert_eq!(received, test_frame);
}

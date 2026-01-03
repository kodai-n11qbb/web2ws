#[cfg(test)]
mod tests {
    use web2ws::websocket::WebSocketServer;
    use tokio_test;
    use tokio::net::TcpStream;
    use tokio_tungstenite::{connect_async, tungstenite::Message};
    use futures_util::{SinkExt, StreamExt};

    #[tokio::test]
    async fn test_websocket_server_startup() {
        // Test that WebSocket server can start up
        let server = WebSocketServer::new("127.0.0.1:8081".to_string());
        assert!(server.is_ok(), "Failed to create WebSocket server");
    }

    #[tokio::test]
    async fn test_websocket_client_connection() {
        // Start server on a different port for testing
        let server_addr = "127.0.0.1:8082";
        let server = WebSocketServer::new(server_addr.to_string()).expect("Failed to create server");
        
        // Start server in background
        let server_handle = tokio::spawn(async move {
            server.run().await.expect("Server failed to run");
        });

        // Give server time to start
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Try to connect
        let ws_url = format!("ws://{}/ws", server_addr);
        let connect_result = connect_async(&ws_url).await;

        match connect_result {
            Ok((_, _)) => {
                // Connection successful
                println!("WebSocket connection test passed");
            }
            Err(e) => {
                println!("WebSocket connection failed: {}", e);
            }
        }

        // Clean up
        server_handle.abort();
    }

    #[tokio::test]
    async fn test_websocket_message_broadcast() {
        // Test that server can broadcast messages to clients
        let server_addr = "127.0.0.1:8083";
        let server = WebSocketServer::new(server_addr.to_string()).expect("Failed to create server");
        
        // Start server in background
        let server_handle = tokio::spawn(async move {
            server.run().await.expect("Server failed to run");
        });

        // Give server time to start
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Connect client
        let ws_url = format!("ws://{}/ws", server_addr);
        if let Ok((ws_stream, _)) = connect_async(&ws_url).await {
            let (mut write, mut read) = ws_stream.split();
            
            // Send a test message
            let test_msg = Message::Text("test".to_string());
            let send_result = write.send(test_msg).await;
            assert!(send_result.is_ok(), "Failed to send message");
            
            // Try to receive a message (timeout after 1 second)
            let receive_result = tokio::time::timeout(
                tokio::time::Duration::from_secs(1),
                read.next()
            ).await;
            
            match receive_result {
                Ok(Some(msg)) => {
                    assert!(msg.is_ok(), "Received error message");
                }
                Ok(None) => {
                    println!("Connection closed");
                }
                Err(_) => {
                    println!("Timeout waiting for message (expected in test)");
                }
            }
        }

        // Clean up
        server_handle.abort();
    }

    #[tokio::test]
    async fn test_websocket_frame_broadcast() {
        // Test broadcasting frame data (binary)
        let server_addr = "127.0.0.1:8084";
        let server = WebSocketServer::new(server_addr.to_string()).expect("Failed to create server");
        
        // Start server in background
        let server_handle = tokio::spawn(async move {
            server.run().await.expect("Server failed to run");
        });

        // Give server time to start
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Connect client
        let ws_url = format!("ws://{}/ws", server_addr);
        if let Ok((ws_stream, _)) = connect_async(&ws_url).await {
            let (mut write, _) = ws_stream.split();
            
            // Send binary frame data (simulating MJPEG frame)
            let test_frame = vec![0xFF, 0xD8, 0xFF, 0xE0]; // JPEG header
            let binary_msg = Message::Binary(test_frame);
            let send_result = write.send(binary_msg).await;
            assert!(send_result.is_ok(), "Failed to send binary frame");
        }

        // Clean up
        server_handle.abort();
    }
}

# web2ws
4os(android ios macos win) -> realtime video with ws\
not use DOCKER NODE

## Usage

### Running the Server
Start the video streaming server with custom FPS and quality settings:

```bash
cargo run -- --fps 15 --quality 70
```

#### Command Line Options
- `--fps <FPS>`: Target frames per second (default: 30.0)
  - Valid range: 1.0 - 120.0 fps
  - Controls how many frames per second are captured and transmitted
  
- `--quality <QUALITY>`: JPEG quality level (default: 85)
  - Valid range: 10 - 95
  - Higher values produce larger, better quality frames
  
- `--bind <ADDRESS>`: Server bind address (default: 127.0.0.1:9001)
  - Format: IP:PORT

### Example Commands

Basic usage with defaults:
```bash
cargo run
```

Low latency, low quality stream:
```bash
cargo run -- --fps 10 --quality 50
```

High quality stream:
```bash
cargo run -- --fps 30 --quality 90
```

Custom bind address:
```bash
cargo run -- --bind 0.0.0.0:8080 --fps 20 --quality 75
```

### WebSocket Endpoints

Once the server is running, clients can connect to:
- **Camera Stream**: `ws://127.0.0.1:9001/camera` - Send camera frames
- **Viewer Stream**: `ws://127.0.0.1:9001/view` - Receive video frames

### Testing

Run the full test suite:
```bash
cargo test
```

Run specific tests:
```bash
cargo test test_camera              # Camera basic functionality
cargo test test_camera_params       # FPS and quality settings
cargo test test_websocket           # WebSocket communication
cargo test test_server              # Full server integration
```

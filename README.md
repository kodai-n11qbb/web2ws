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

### Web Interface

Once the server is running, open your browser to:
- **Sender** (Camera): `http://localhost:9001/` - Share camera stream
- **Viewer**: `http://localhost:9001/viewer.html` - Watch video stream

**Or access the WebSocket endpoints directly:**
- **Camera Stream**: `ws://localhost:9001/camera` - Send camera frames
- **Viewer Stream**: `ws://localhost:9001/view` - Receive video frames

### Architecture

The server supports:
- **HTTP** for serving HTML files
- **WebSocket** for real-time bidirectional communication
- **Broadcast channels** for multi-client streaming
- **JPEG frame encoding** for efficient transmission

### Testing

Run the full test suite:
```bash
cargo test --lib
```

Test categories:
- **Camera Tests**: Initialization, frame capture, FPS control, quality settings
- **WebSocket Tests**: Binary transmission, bidirectional communication, high-frequency streaming
- **Server Tests**: Client management, frame broadcasting, pipeline validation

### Data Flow

```
Camera Client (Sender)
    ↓ (JPEG frames via WebSocket)
Server (Broadcast Channel)
    ↓ (Frame distribution)
Viewer Clients (Viewer)
```


|Target FPS|Achieved FPS|Efficiency|
|:-|:-|:-|
|30|29–30|✅ 97–100%|
|60|54–55|⚠️ 90–92%|
|120|98–99|⚠️ 82–83%|
|200|150–151|⚠️ 75–76%|
|500|295–297|⚠️ 59–60%|
|1000|441–444|⚠️ 44–45%|
|2000|620–628|⚠️ 31–32%|

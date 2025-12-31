# web2ws - Realtime Video Streaming over WebSocket

Fast, lightweight video streaming solution for offline networks. Built entirely in **Rust** with **no Node.js or Docker dependencies**.

**Key Features:**
- ✅ Realtime video streaming via WebSocket
- ✅ Configurable FPS (1-120 fps)
- ✅ Adjustable JPEG quality (10-95)
- ✅ Works on offline local networks
- ✅ Three-service architecture (signaling server, sender, viewer)
- ✅ Speed-optimized with release mode compilation

## Architecture

The system is divided into three separate binaries for easy management:

1. **Signaling Server** (`server`) - Routes messages between senders and viewers
2. **Camera Sender** (`sender`) - Captures frames and streams to server
3. **Viewer** (`viewer`) - Receives and displays video frames

## Building

```bash
# Build all binaries in release mode (optimized for speed)
cargo build --release

# Binaries will be in target/release/
```

## Running

### Start the Signaling Server
```bash
# Default: localhost:9001, 30 FPS, quality 85
./target/release/server

# Custom settings
./target/release/server --bind 0.0.0.0:9001 --fps 60 --quality 90
```

### Start a Sender (Camera)
```bash
# Connect to server, send 30 FPS
./target/release/sender --server ws://127.0.0.1:9001 --fps 30 --quality 85

# High FPS, lower quality (less bandwidth)
./target/release/sender --fps 60 --quality 50
```

### Start a Viewer
```bash
# Connect and receive video
./target/release/viewer --server ws://127.0.0.1:9001

# Connect to remote server
./target/release/viewer --server ws://192.168.1.10:9001
```

## Command Line Options

### Server Options
- `--fps <FPS>` - Frames per second (default: 30, range: 1-120)
- `--quality <QUALITY>` - JPEG quality (default: 85, range: 10-95)
- `--bind <ADDRESS>` - Server address (default: 127.0.0.1:9001)

### Sender Options
- `--server <URL>` - WebSocket server address (default: ws://127.0.0.1:9001)
- `--fps <FPS>` - Capture FPS (default: 30)
- `--quality <QUALITY>` - JPEG quality (default: 85)
- `--camera <DEVICE>` - Camera device path (default: /dev/video0)

### Viewer Options
- `--server <URL>` - WebSocket server address (default: ws://127.0.0.1:9001)

## Testing

Run the full test suite (36 tests):
```bash
# Run all tests
cargo test

# Run only library tests (unit tests for modules)
cargo test --lib

# Run only integration tests (spec compliance tests)
cargo test --test integration

# Run with output
cargo test -- --nocapture
```

## Specification Compliance

This implementation fully satisfies the specification (`spec.md`):

✅ **Usage Environment**
- Works offline on local networks
- Input source: webcam frames
- Designed for small number of users

✅ **Services**
- Signaling server - routes frames between senders/viewers
- Webcam sender - captures and transmits frames
- Viewer - receives and displays video

✅ **Internal Design**
- High FPS support (up to 120 fps)
- Pure Rust implementation (no Node.js or Docker)
- Adjustable quality (10-95 JPEG quality)
- WebSocket protocol (web -> ws)
- Configuration via CLI

✅ **Performance Focus**
- Release mode compilation with optimizations
- Single-frame buffer for speed
- Efficient message broadcasting
- Minimal memory allocations

## Project Structure

```
src/
├── lib.rs              # Library exports
├── common.rs           # Shared types (Message, VideoFrame)
├── config.rs           # Configuration management
├── signaling_server.rs # Server routing logic
├── sender.rs           # Camera capture abstraction
├── viewer.rs           # Frame display abstraction
└── bin/
    ├── server.rs       # Signaling server binary
    ├── sender.rs       # Camera sender binary
    └── viewer.rs       # Viewer binary

tests/
└── integration.rs      # Spec compliance tests
```

## Performance Considerations

1. **Frame Buffer** - Single frame buffer for minimal allocations
2. **Async/Await** - Tokio for efficient async runtime
3. **Release Build** - LTO and optimized code generation enabled
4. **Broadcast Channel** - Efficient multi-viewer support
5. **DashMap** - Lock-free concurrent connection tracking

## Example Workflow

Terminal 1 - Start server:
```bash
./target/release/server --fps 30 --quality 85
```

Terminal 2 - Start sender:
```bash
./target/release/sender --fps 30 --quality 85
```

Terminal 3 - Start viewer(s):
```bash
./target/release/viewer
./target/release/viewer  # Multiple viewers supported
```

All communication happens via WebSocket on the local network.

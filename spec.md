```
.
├── Cargo.toml
├── README.md
├── spec.md
├── src
│   ├── camera
│   │   └── mod.rs
│   └── main.rs
└── tests
    ├── integration
    │   └── test_server.rs
    └── unit
        ├── camera
        │   ├── test_camera.rs
        │   └── test_camera_params.rs
        └── websocket
            └── test_websocket.rs
```

```
1. cargo test test_camera              # カメラ基本動作 → src/camera/mod.rs実装
2. cargo test test_camera_params       # FPS/画質 → Camera拡張
3. cargo test test_websocket           # WebSocket → src/websocket/mod.rs実装  
4. cargo test test_server              # 全体連携 → src/server/mod.rs実装
5. cargo run -- --fps 15 --quality 70  # MVP完成！
```
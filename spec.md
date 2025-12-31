# spec
- オフライン稼働
- node docker不使用(only rust)
- web -> ws
- 画質調節可能

```
.
├── README.md
├── spec.md
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
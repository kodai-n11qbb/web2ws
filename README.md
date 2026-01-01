# web2ws - Webcam to WebSocket Streaming

webcamを通してWebSocketで高速配信するRustアプリケーション（オフライン対応）

## 🚀 機能

- Webcamから映像を取得
- WebSocket経由で同一ネットワーク内のクライアントへ配信
- 低遅延での映像配信（約30fps）
- Python受信クライアント付き

## 📋 要件

- Rust 1.70以上
- Python 3.8以上（受信クライアント用）
- Webcamデバイス

## 🛠️ セットアップ

### Rustアプリケーションのビルド

```bash
cargo build --release
```

### Python受信クライアントのセットアップ

```bash
cd python
pip install -r requirements.txt
```

## 🎯 使用方法

### 1. サーバー（配信側）の起動

```bash
cargo run --release
```

サーバーは `127.0.0.1:8080` で起動し、WebSocketエンドポイントは `/ws` です。

**注意**: Webcamがない場合でもサーバーは起動します。その場合、ブラウザでプレビューを表示できますが、映像は配信されません。

### 2. ブラウザでプレビュー表示

ブラウザで以下のURLにアクセス：
```
http://127.0.0.1:8080
```

WebSocket経由でwebcam映像が表示されます。

### 3. Pythonクライアント（受信側）の起動

```bash
cd python
python receiver.py
```

デフォルトでは `ws://127.0.0.1:8080/ws` に接続します。別のアドレスを指定する場合：

```bash
python receiver.py ws://192.168.1.100:8080/ws
```

### 操作方法

- ブラウザ: 自動的に接続し、映像を表示
- Pythonクライアント: `q`キーで終了

## 📁 プロジェクト構造

```
web2ws/
├── src/           # Rustソースコード
│   ├── main.rs    # メインアプリケーション
│   ├── lib.rs     # ライブラリエントリーポイント
│   ├── webcam.rs  # Webcam取得機能
│   └── websocket.rs # WebSocket配信機能
├── tests/         # TDDテストコード
├── python/        # Python受信クライアント
│   ├── receiver.py
│   └── requirements.txt
└── mds/           # ドキュメント
    ├── REQUIREMENTS.md
    ├── backlog.md
    └── config_conventions.md
```

## 🧪 テスト

```bash
cargo test
```

## 📝 開発原則

このプロジェクトはアジャイルTDDを厳守しています：

- Red-Green-Refactorサイクル必須
- `/tests/` で実装されたものを中心にプロジェクトを進める
- `/mds/backlog.md` でtodoを管理
- `/mds/config_conventions.md` のコード規約に従う

詳細は `/mds/REQUIREMENTS.md` を参照してください。

## 🔧 技術スタック

- **Rust**: メインアプリケーション
- **nokhwa**: Webcam取得
- **axum**: WebサーバーとWebSocket
- **tokio**: 非同期ランタイム
- **Python**: 受信クライアント
- **websockets**: Python WebSocketクライアント
- **opencv-python**: 映像表示

## 📄 ライセンス

（ライセンス情報を追加）

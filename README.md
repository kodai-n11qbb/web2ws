# web2ws - Webcam to WebSocket Streaming

WebcamからWebSocketで高速配信するRustアプリケーション（オフライン対応）

## 🚀 機能

- Webcamから映像を取得
- WebSocket経由で同一ネットワーク内のクライアントへ配信
- 低遅延での映像配信（約30fps）
- HTML/JavaScriptクライアント付き
- リアルタイム統計（FPS、遅延、データ転送率）
- 画質・FPS調整機能
- テストパターン生成（Webcam未接続時）

## 📋 要件

- Rust 1.70以上
- Webcamデバイス（任意）
- 最新のChromeブラウザ

## 🛠️ セットアップ

### Rustアプリケーションのビルド

```bash
cargo build --release
```

## 🎯 使用方法

### 1. サーバー（配信側）の起動

```bash
cargo run --release
```

サーバーは `127.0.0.1:8080` で起動し、WebSocketエンドポイントは `/ws` です。

**注意**: Webcamがない場合でもサーバーは起動します。テストパターンが配信されます。

### 2. ブラウザで受信表示

ブラウザで以下のURLにアクセス：
```
http://127.0.0.1:8080
```

WebSocket経由でwebcam映像（またはテストパターン）が表示されます。

### 操作方法

- **接続/切断**: ボタンで制御
- **画質調整**: スライダーで品質（10-100%）を調整
- **FPS調整**: スライダーで目標FPS（10-60）を設定
- **統計表示**: リアルタイムでFPS、遅延、データ転送率を表示

## 📁 プロジェクト構造

```
web2ws/
├── src/           # Rustソースコード
│   ├── main.rs    # メインアプリケーション
│   ├── lib.rs     # ライブラリエントリーポイント
│   ├── webcam.rs  # Webcam取得機能
│   └── websocket.rs # WebSocket配信機能
├── tests/         # TDDテストコード
│   ├── webcam_tests.rs
│   └── websocket_tests.rs
├── static/        # HTML/JSクライアント
│   ├── index.html
│   └── app.js
└── mds/           # ドキュメント
    ├── REQUIREMENTS.md
    ├── backlog.md
    └── config_conventions.md
```

## 🧪 テスト

```bash
cargo test
```

TDDアプローチで実装されており、以下のテストが含まれます：
- Webcam初期化テスト
- フレームキャプチャテスト
- WebSocketサーバー起動テスト
- クライアント接続テスト

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
- **image**: JPEG変換
- **HTML/JavaScript**: クライアント側実装
- **WebSocket**: リアルタイム通信

## 🌐 機能詳細

### Webcamキャプチャ
- 自動カメラ検出
- RGBからJPEGへのリアルタイム変換
- カメラ未接続時のテストパターン生成

### WebSocketストリーミング
- バイナリフレーム配信
- 複数クライアント対応
- 自動再接続機能

### クライアントインターフェース
- レスポンシブデザイン
- リアルタイム統計表示
- 品質・FPS調整
- クロスブラウザ対応

## 📄 ライセンス

（ライセンス情報を追加）
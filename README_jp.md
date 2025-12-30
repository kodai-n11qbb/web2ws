web2ws
4OS(Android/iOS/macOS/Windows) → WebSocketリアルタイムビデオ
（Docker/Node.js不使用）

使い方
サーバー起動
カスタムFPSと品質設定でビデオストリーミングサーバーを開始：

``` bash
cargo run -- --fps 15 --quality 70
```
コマンドラインオプション
`--fps <FPS>`: 目標フレームレート（デフォルト: 30.0）

有効範囲: 1.0 - 120.0 fps

1秒間にキャプチャ・送信するフレーム数

`--quality <QUALITY>`: JPEG品質レベル（デフォルト: 85）

有効範囲: 10 - 95

数値が高いほどデータ量増・品質向上

`--bind <ADDRESS>`: サーバーバインドアドレス（デフォルト: 127.0.0.1:9001）

形式: IP:PORT

実行例
基本使用（デフォルト設定）:

``` bash
cargo run
```
低遅延・低品質ストリーム:

``` bash
cargo run -- --fps 10 --quality 50
```
高品質ストリーム:

``` bash
cargo run -- --fps 30 --quality 90
```
カスタムバインドアドレス:

``` bash
cargo run -- --bind 0.0.0.0:8080 --fps 20 --quality 75
```
WebSocketエンドポイント
サーバー起動後、クライアントは以下に接続：

カメラストリーム: ws://127.0.0.1:9001/camera - カメラフレーム送信

視聴ストリーム: ws://127.0.0.1:9001/view - ビデオフレーム受信

テスト実行
全テストスイート実行:

``` bash
cargo test
```
個別テスト実行:

``` bash
cargo test test_camera              # カメラ基本機能
cargo test test_camera_params       # FPS/品質設定
cargo test test_websocket           # WebSocket通信
cargo test test_server              # サーバー統合テスト
tests/unit/camera/test_camera_params.rs で設定テスト確認済み！
```
# Requirements.md - アジャイルTDDプロジェクト厳守ルール

## 1. 開発原則(これは絶対に厳守)
- アジャイルTDDを厳守。Red-Green-Refactorサイクル必須。
- 逸脱時は即拒否。再生成を要求。
- TDD(/tests/)で実装されたものを中心にプロジェクトを進めること
- /mds/backlog.md より、todoを書き示し更新し続け、透明性を保つこと
- /mds/config_conventions.md より、コード規約を定めて従うこと

## 2. TDD実装
- /tests/ に実装する。

## 3. 人間定義項目（圧縮版）
- **目的**: オフライン環境で、Webcam映像をWebSocket経由で高速配信・受信・表示。  
- **範囲**: 同一ネットワーク内での映像取得・配信・確認。  
- **完結基準**: Webcam → WS配信 → 受信・表示。
- **業務要件**:  
  - 現状、Web経由で簡単・高速に映像を配信できる仕組みがない。  
  - フロー: 端末アクセス → 配信 → 他端末で受信。
- **機能要件**:  
  - Rust中心実装（将来ライブラリ化可能）。  
  - 遅延最小・最大fpsを追求。  
  - 配信端末でプレビュー表示。  
  - WebSocketで映像配信。  
  - 画質調整機能。  
  - /static/ にHTML+VanillaJSで受信表示ページを用意。
- **非機能要件**: 未定義（速度最優先想定）。
- **外部インタフェース**:  
  - Chrome対応（Win/macOS/iOS/Android）。  
  - オフライン動作必須。
- **制約・前提**: Rust中心開発。
- **禁止技術**: Docker、npm、オンライン依存。

## 4. 参照ファイル
- coding_conventions.md: [./mds/coding_conventions.md]
- backlog.md: [./mds/backlog.md]

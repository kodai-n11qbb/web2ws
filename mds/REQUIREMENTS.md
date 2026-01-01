# Requirements.md - アジャイルTDDプロジェクト厳守ルール

## 1. 開発原則
- アジャイルTDDを厳守。Red-Green-Refactorサイクル必須。
- 逸脱時は即拒否。再生成を要求。
- TDD(/tests/)で実装されたものを中心にプロジェクトを進めること
- /mds/backlog.md より、todoを書き示し、透明性を保つこと
- /mds/config_conventions.md より、コード規約を定めて従うこと

## 2. TDD実装
- /tests/ に実装する。

## 3. 人間定義項目
- 目的と範囲（Goal & Scope）
    - 目的: webcamを通してwsで高速配信する（オフラインで）
    - 範囲: 映像配信(と確認用の受信と表示)
    - 完結基準: 同ネットワーク内での映像取得・配信（webcamをwsに流す部分）+映像確認用の受信・表示
- 業務要件（Business Requirements）
    - 現状: ウェブ経由でwsに映像を簡単に高速配信するものが存在しない
    - 主要フロー: 端末でウェブアクセス->端末から配信->別端末から受信
- 機能要件（Functional Requirements）
    - 主要機能はrustとし、のちにライブラリとして公開できるものとする
    - 遅延が少ないこと(常に最高fpsを目指す)
    - 配信端末からwebcam映像をブラウザ上でプレビュー表示できること
    - 配信端末から取得した映像をwebsocketで同一ネットワーク内のクライアントへ配信する
    - 配信する画質の調節ができる
    - 映像を受信する簡素なものを別途で用意する(pythonで独立したものとして作成を希望)
- 非機能要件（Non-functional Requirements）
    - 
- 外部インタフェース（External Interfaces）
    - 4OS(win macos ios android)のchrome対応し、速度を優先かつオフラインで動作すればなんでも良い
- 制約条件と前提（Constraints & Assumptions）
    - rustを中心とした開発にすること
- 禁止技術
    - docker, npm

## 4. 参照ファイル
- coding_conventions.md: [./mds/coding_conventions.md]
- backlog.md: [./mds/backlog.md]

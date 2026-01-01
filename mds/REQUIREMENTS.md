# Requirements.md - アジャイルTDDプロジェクト厳守ルール

## 1. 開発原則
- アジャイルTDDを厳守。Red-Green-Refactorサイクル必須。
- 逸脱時は即拒否。再生成を要求。
- /mds/REQUIREMENTS.md及び./mds/に含まれるmdを参照する
- /mds/ に全MD保存 (coding_conventions.md, backlog.md)。
- 更新時は差分明記し、厳守。

## 2. TDD実装
- /tests/ に実装する。

## 3. 人間定義項目
- 目的と範囲（Goal & Scope）
    - 目的: 高速AI推論を簡素にする（オフラインで）
    - 範囲: 映像配信(と確認用の受信と表示)
    - 完結基準: 同ネットワーク内での映像取得・配信（webcamをwsに流す部分）+映像確認用の受信・表示
- 業務要件（Business Requirements）
    - 現状: AI推論をws経由で映像を取得する手段が面倒で遅延がある
    - 主要フロー: 端末でウェブアクセス->端末から配信->別端末から受信->同または別端末でAI推論
- 機能要件（Functional Requirements）
    - 遅延が少ないこと(常に最高fpsを目指す)
    - 配信端末からwebcam映像をブラウザ上でプレビュー表示できること
    - 配信端末から取得した映像をwebsocketで同一ネットワーク内のクライアントへ配信する
    - 配信する画質の調節ができる
    - 映像を受信する簡素なものを別途で用意する
- 非機能要件（Non-functional Requirements）
    - このプロジェクトコードを人間が見ても理解できるものであり続けさせる
    - AIがこのコードを改変する可能性もあるためREQUIREMENTS.mdや/mds/の中身に準拠する
- 外部インタフェース（External Interfaces）
    - 4OS(win macos ios android)のchrome対応し、速度を優先かつオフラインで動作すればなんでも良い
- 制約条件と前提（Constraints & Assumptions）
    - 
- 禁止技術
    - docker, node

## 4. 参照ファイル
- coding_conventions.md: [./mds/coding_conventions.md]
- backlog.md: [./mds/backlog.md]

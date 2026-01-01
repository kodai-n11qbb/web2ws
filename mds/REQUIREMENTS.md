# Requirements.md - アジャイルTDDプロジェクト厳守ルール

## 1. 開発原則
- アジャイルTDDを厳守。Red-Green-Refactorサイクル必須。
- 逸脱時は即拒否。再生成を要求。
- REQUIREMENTS.md及び./mds/に含まれるmdを参照する

## 2. TDD実装
- 全てテストをGitHub Actions (.github/workflows/test.yml)で実行。
- PRマージ前にテスト通過必須 (Branch Protection有効)。

## 3. 人間定義項目
- 目的と範囲（Goal & Scope）
    - 
- 業務要件（Business Requirements）
    - 
- 機能要件（Functional Requirements）
    - 
- 非機能要件（Non-functional Requirements）
    - 
- 外部インタフェース（External Interfaces）
    - 
- 制約条件と前提（Constraints & Assumptions）
    - 
- 禁止技術: [例: 同期処理禁止、特定ライブラリNG]
    - 

## 4. ドキュメント管理
- /mds/ に全MD保存 (coding_conventions.md, backlog.md)。
- 更新時は差分明記し、厳守。

## 5. 参照ファイル
- coding_conventions.md: [./mds/coding_conventions.md]
- backlog.md: [./mds/backlog.md]

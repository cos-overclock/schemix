# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## プロジェクト概要

Schemixは、AtopileライクなコードベースEDA（Electronic Design Automation）フレームワークのRust実装です。DSL（Domain Specific Language）を使用して回路設計を記述し、KiCadとの統合により実際のPCBレイアウトファイルを生成します。

## 共通コマンド

```bash
# プロジェクトのビルド
cargo build

# プロジェクトの実行
cargo run

# テスト実行
cargo test

# 特定のテスト実行
cargo test test_name

# リント実行
cargo clippy

# コードフォーマット
cargo fmt

# ドキュメント生成
cargo doc --open
```

## アーキテクチャ

Schemixは以下のコアモジュール構成で設計されています：

### 想定されるモジュール構造

- `parser/` - DSL構文解析器（nomクレート使用）
- `ast/` - 抽象構文木定義と操作
- `compiler/` - コンパイラとバリデーション
- `kicad/` - KiCadファイル形式生成（.kicad_sch, .kicad_pcb）
- `components/` - 電子コンポーネント管理とライブラリ
- `project/` - プロジェクト設定とワークスペース管理
- `cli/` - CLI実装（clapクレート使用）

### データフロー

1. `.smx`ファイル（DSL）→ パーサー → AST
2. AST → コンパイラ → バリデーション済み回路定義
3. 回路定義 → KiCad生成器 → `.kicad_sch`, `.kicad_pcb`ファイル

### 想定されるプロジェクト構造

```file
project/
├── schemix.yaml          # プロジェクト設定
├── src/
│   ├── main.smx         # メイン回路定義
│   └── components/      # カスタムコンポーネント
├── build/               # 生成されたKiCadファイル
└── lib/                 # ローカルライブラリ
```

## 実装フェーズ

### Phase 1 (MVP)

- 基本DSL構文解析器
- シンプルなKiCad出力
- CLI基本機能

### Phase 2 (機能拡張)

- コンポーネントライブラリ
- パッケージ管理
- 型検査・バリデーション

### Phase 3 (最適化)

- IDE統合（LSP）
- 高度なKiCad機能
- テスト・デバッグツール

## 技術スタック

- **パーサー**: `nom` - 高性能パーサーコンビネータ
- **CLI**: `clap` - コマンドライン引数解析
- **シリアライゼーション**: `serde` + `serde_yaml` - 設定ファイル処理
- **エラーハンドリング**: `anyhow` - エラー処理
- **非同期処理**: `tokio` - 非同期I/O（必要に応じて）

## KiCad統合

KiCadのS式形式ファイルを生成：

- `.kicad_sch` - 回路図ファイル
- `.kicad_pcb` - PCBレイアウトファイル
- `.kicad_pro` - プロジェクト設定ファイル

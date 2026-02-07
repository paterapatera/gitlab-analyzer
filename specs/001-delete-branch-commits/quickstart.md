# Quickstart: Delete Collected Branch Commits

## 前提

- Rust toolchain（Rust 2021 / Tauri 2）
- Bun（`tauri.conf.json` の `beforeDevCommand/beforeBuildCommand` が `bun run ...`）

## セットアップ

1. 依存をインストール
   - `bun install`
2. 開発起動
   - `bun run tauri dev`

## テスト

- フロントエンド（Vitest）: `bun run test`
- Rust: `Push-Location src-tauri; cargo test; Pop-Location`

## ビルド

- `bun run tauri build`

---

## 手動検証手順

### US1: ブランチ削除フロー

1. アプリを起動する（`bun run tauri dev`）
2. プロジェクトとブランチを選択
3. ゴミ箱アイコンの削除ボタンをクリック
4. 影響サマリ（削除件数・影響ビュー）を確認
5. 「取り消し不可」文言の確認ダイアログで確定
6. **確認事項**:
   - 削除後、集計ビューから対象ブランチが除外される
   - 取り消し不可の文言が表示される

### US2: キャンセル時の挙動

1. 影響サマリ表示後、確認ダイアログでキャンセル
2. **確認事項**:
   - 何も削除されない
   - 集計結果が変化しない

### Edge: 収集中のブランチ削除

1. 収集中の同一ブランチに対して削除を試みる
2. **確認事項**:
   - 削除がブロックされ、理由が表示される

### Edge: 収集件数ゼロ

1. コミットが存在しないブランチで削除を試みる
2. **確認事項**:
   - 非破壊メッセージが表示される
   - データは変化しない

### セキュリティ確認

1. DevTools > Console でログを確認
2. **確認事項**:
   - `accessToken`/`authorEmail` がどこにも出力されていない

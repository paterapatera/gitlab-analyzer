# Quickstart: GitLab 月次コミット行数分析

## 前提

- Rust toolchain（このリポジトリは Rust 2021 / Tauri 2）
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

## メモ

- `accessToken` と `authorEmail` は UI/ログに出さない（仕様要件）。

---

## 手動検証手順

### US1: GitLab 接続設定とプロジェクト同期

1. アプリを起動する（`bun run tauri dev`）
2. 接続設定画面で GitLab Base URL とアクセストークンを入力
3. 保存ボタンをクリック
4. 「プロジェクト同期」ボタンをクリック
5. **確認事項**:
   - プロジェクト一覧が表示される
   - 無効なトークンの場合、エラーメッセージに「次にすべき行動」が含まれる
   - ブラウザ DevTools のコンソールにトークンが出力されていない
   - 画面にトークンが表示されていない

### US2: コミット収集

1. US1 完了後、プロジェクトを選択
2. ブランチを選択
3. 期間（任意）を指定して「収集」ボタンをクリック
4. **確認事項**:
   - 収集結果（挿入件数/スキップ件数/欠損件数）が表示される
   - 再収集しても重複が増えない（skippedDuplicateCount が増加）
   - エラー時も途中まで保存されている

### US3: 月次集計表示

1. US2 完了後、「集計」タブに移動
2. フィルタ（年/ユーザー/プロジェクト）を変更
3. **確認事項**:
   - フィルタ変更から 2 秒以内にグラフ/表が更新される
   - 画面に作者メールが表示されていない
   - 欠損件数が正しく表示される

### セキュリティ確認

1. DevTools > Console でログを確認
2. DevTools > Network でリクエスト/レスポンスを確認
3. **確認事項**:
   - `accessToken` がどこにも出力されていない
   - `authorEmail` がどこにも出力されていない

# Quickstart: Project Autocomplete

## 前提

- Bun（`tauri.conf.json` の `beforeDevCommand/beforeBuildCommand` が `bun run ...`）
- Tauri 2 / Rust toolchain（バックエンドの既存コマンドを利用）

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

### US1: コミット収集画面のプロジェクト検索

1. アプリを起動する（`bun run tauri dev`）
2. コミット収集タブを開く
3. プロジェクト検索入力に「ana」を入力する
4. **確認事項**:
   - プロジェクト名/パスに「ana」を含む候補だけが表示される
   - 入力を空にすると全件が表示される
   - 該当が無い場合は「該当するプロジェクトがありません」が表示される

### US2: 集計表示（プロジェクト別）の検索

1. 集計タブを開く（プロジェクト別ビュー）
2. プロジェクト検索入力に「group/」を入力する
3. **確認事項**:
   - `pathWithNamespace` に一致する候補だけが表示される
   - 大文字/小文字を区別しない

### US3: キーボード操作

1. 入力欄にフォーカスして文字を入力
2. 下矢印で最初の候補を選択
3. 上下矢印でハイライト移動
4. Enterで選択、Escで閉じる
5. **確認事項**:
   - 選択後にブランチ選択など関連UIが有効になる

### 追加確認

- 100件を超える場合は「さらに絞り込み」メッセージが表示される
- 150ms デバウンス後に候補が更新される

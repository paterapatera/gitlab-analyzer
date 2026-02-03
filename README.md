# GitLab 月次コミット行数分析

GitLab からコミットを取得し、年/月/ユーザー単位で（追加行 + 削除行）を集計・可視化するデスクトップアプリケーション。

## ドキュメント

- **仕様書**: [specs/001-gitlab-commit-lines/spec.md](specs/001-gitlab-commit-lines/spec.md)
- **実装計画**: [specs/001-gitlab-commit-lines/plan.md](specs/001-gitlab-commit-lines/plan.md)
- **データモデル**: [specs/001-gitlab-commit-lines/data-model.md](specs/001-gitlab-commit-lines/data-model.md)
- **APIコントラクト**: [specs/001-gitlab-commit-lines/contracts/tauri-commands.openapi.yaml](specs/001-gitlab-commit-lines/contracts/tauri-commands.openapi.yaml)
- **クイックスタート**: [specs/001-gitlab-commit-lines/quickstart.md](specs/001-gitlab-commit-lines/quickstart.md)

## 技術スタック

- **Frontend**: TypeScript 5.6 / React 18 / Vite 6 / Tailwind CSS / shadcn/ui
- **Backend**: Rust 2021 / Tauri 2
- **Testing**: Vitest (Frontend) / cargo test (Backend)
- **Runtime**: Bun

## 推奨 IDE 設定

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## セットアップ

```bash
# 依存インストール
bun install

# 開発起動
bun run tauri dev

# テスト
bun run test                                           # Frontend
Push-Location src-tauri; cargo test; Pop-Location     # Backend
```

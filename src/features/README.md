# Features ディレクトリ

機能（ユーザーストーリー）単位でコンポーネントとロジックをまとめるディレクトリ。

## 構成指針

各 feature は以下の構造を持つ:

```text
src/features/
├── <featureName>/
│   ├── <ComponentName>.tsx       # UI コンポーネント
│   ├── <ComponentName>.test.tsx  # テスト（Vitest）
│   ├── use<HookName>.ts          # カスタムフック（必要に応じて）
│   └── types.ts                  # feature 固有の型定義（必要に応じて）
└── ui/                           # 複数 feature で共有する UI ユーティリティ
```

## 実装済み Feature

- `gitlabConnection/` - GitLab 接続設定（US1）
- `projects/` - プロジェクト同期・一覧（US1）
- `collect/` - コミット収集（US2）
- `stats/` - 月次集計・グラフ・表（US3）
- `ui/` - 共通 UI（エラー表示、非同期状態管理など）

## 命名規則

- コンポーネント: PascalCase（例: `ConnectionForm.tsx`）
- フック: camelCase + `use` prefix（例: `useAsyncState.ts`）
- テスト: `<ComponentName>.test.tsx`

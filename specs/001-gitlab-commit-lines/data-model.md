# Data Model: GitLab 月次コミット行数分析

## 概要

本機能は、GitLab から取得したコミットデータを端末ローカルに保存し、年/月/ユーザー単位で（追加行 + 削除行）を集計する。

## エンティティ

### GitLabConnection（接続設定）

- `baseUrl: string`（必須）
- `accessToken: string`（必須）
- `updatedAtUtc: string`（ISO8601）

Validation:

- `baseUrl` は `http(s)://` で始まる
- `accessToken` は空でない

Security:

- UI とログに `accessToken` を出さない（FR-016/FR-018）

---

### Project（GitLab プロジェクト）

- `projectId: number`（GitLab の project id）
- `name: string`
- `pathWithNamespace: string`
- `webUrl: string`

---

### Branch（ブランチ）

- `projectId: number`
- `name: string`
- `isDefault?: boolean`

Unique:

- `(projectId, name)`

---

### Commit（コミット）

- `projectId: number`
- `branchName: string`
- `sha: string`
- `message: string`
- `committedDateUtc: string`（ISO8601; UTC として扱う）
- `authorName: string`
- `authorEmail?: string`（取得できない場合あり）
- `additions?: number`（取得不能なら欠損）
- `deletions?: number`（取得不能なら欠損）
- `statsMissing: boolean`（`additions/deletions` が欠損なら true）

Unique:

- `(projectId, branchName, sha)`（FR-011）

Security:

- UI とログに `authorEmail` を出さない（FR-017/FR-019）

---

### MonthlyUserStats（集計結果：派生/計算）

保存は必須ではなく、表示時に保存済み `Commit` から集計する。

- `year: number`
- `month: number`（1-12）
- `userKey: string`（内部キー。`authorEmail` 優先、無い場合 `authorName`）
- `displayName: string`（表示名。`authorName`）
- `totalLines: number`（`additions + deletions` の合計。欠損は 0）
- `missingCount: number`（欠損コミット件数）

Validation:

- `month` は 1..12

## 状態遷移（収集ジョブの概念）

必要に応じて（UX/進捗/再実行のため）、収集実行を「ジョブ」として扱う。

- `idle` → `running` → (`completed` | `failed` | `canceled`)

最小 MVP では、running 中の進捗（ページ数/コミット数）だけを返し、永続ジョブは後回しでも良い。

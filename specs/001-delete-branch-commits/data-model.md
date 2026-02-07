# Data Model: Delete Collected Branch Commits

## 概要

本機能は、プロジェクト/ブランチ単位で保存済みコミットを削除し、集計ビューから除外する。
削除の影響件数は既存のコミットデータから算出する。

## エンティティ

### Project（プロジェクト）

- `projectId: number`
- `name: string`
- `pathWithNamespace: string`

---

### Branch（ブランチ）

- `projectId: number`
- `name: string`

Unique:

- `(projectId, name)`

---

### Commit（収集済みコミット）

- `projectId: number`
- `branchName: string`
- `sha: string`
- `committedDateUtc: string`
- `authorName: string`
- `authorEmail?: string`
- `additions?: number`
- `deletions?: number`
- `statsMissing: boolean`

Unique:

- `(projectId, branchName, sha)`

Indexes:

- `(projectId, branchName)`（削除と影響件数の集計に使用）

Security:

- UI とログに `authorEmail` を出さない。

---

### BranchDeleteImpact（削除影響・派生）

保存は必須ではなく、表示時に `Commit` から計算する。

- `projectId: number`
- `branchName: string`
- `commitCount: number`
- `affectedViews: string[]`（例: `project-view`, `cross-view`）
- `canDelete: boolean`
- `blockReason?: string`

## 状態遷移

削除フローは以下の状態を持つ。

- `idle` → `impact-ready` → (`confirmed` | `canceled`)
- `confirmed` 実行時に `blocked`（収集中）または `completed`（削除完了）へ遷移

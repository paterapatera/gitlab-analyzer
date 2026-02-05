# Data Model: 集計表示におけるユーザーフィルタリング

**Feature**: [spec.md](spec.md) | **Plan**: [plan.md](plan.md) | **Research**: [research.md](research.md)  
**Date**: 2026-02-05

## Entities

### 1. UserFilterState（ユーザー選択状態）

**Purpose**: プロジェクト別ビューまたは横断ビューにおいて、どのユーザーが選択されているかを表す永続化データ

**Attributes**:

- `view_type`: ビュー種別（`"project-view"` | `"cross-view"`）
- `context_key`: 選択状態のコンテキストキー
  - プロジェクト別: `"<project_name>/<branch_name>/<year>"`（例: `"my-project/main/2025"`）
  - 横断: `"<year>"`（例: `"2025"`）
- `selected_users`: 選択されたユーザーの識別子配列（`string[]`）
  - 識別子は既存の集計キー（authorEmail優先、フォールバックauthorName）

**Relationships**:

- 既存のCommitデータ（authorEmail/authorName）と関連
- StatsFiltersの選択条件（project/branch/year）に依存

**Validation Rules**:

- `view_type` は `"project-view"` または `"cross-view"` のみ
- `context_key` は空文字列不可
- `selected_users` は空配列も許容（全解除状態）
- 同一の `view_type` + `context_key` の組み合わせは一意

**State Transitions**:

```
初期状態（未保存） → 全ユーザー選択状態（初回表示時のデフォルト）
全ユーザー選択状態 → 一部選択状態（チェックボックス解除）
一部選択状態 → 全解除状態（全解除ボタン）
全解除状態 → 全選択状態（全選択ボタン）
任意の状態 → 保存（自動永続化）
```

### 2. FilteredUserData（フィルタ済みユーザーデータ）

**Purpose**: ユーザーフィルタリング適用後の月次集計データ

**Attributes**:

- `user_id`: ユーザー識別子（authorEmail優先、フォールバックauthorName）
- `user_name`: 表示用ユーザー名（authorName）
- `monthly_data`: 月次データ配列（既存のMonthlyStatsと同構造）

**Relationships**:

- UserFilterStateの`selected_users`に基づいてフィルタリングされる
- 既存のMonthlyStats（月次集計結果）から派生

**Validation Rules**:

- `user_id` は `selected_users` に含まれるユーザーのみ
- `monthly_data` は既存のMonthlyStats構造に準拠

## Storage Schema

### user_filter_state.json

```json
{
  "project-view": {
    "my-project/main/2025": ["user1@example.com", "user2@example.com"],
    "my-project/develop/2025": ["user1@example.com"],
    "other-project/main/2024": ["user3"]
  },
  "cross-view": {
    "2025": ["user1@example.com", "user2@example.com", "user3"],
    "2024": ["user1@example.com"]
  }
}
```

**Structure**:

- Top-level keys: `"project-view"`, `"cross-view"`
- Nested keys: context_key（`"<project>/<branch>/<year>"` or `"<year>"`）
- Values: 選択されたユーザーID配列（`string[]`）

**Notes**:

- プロジェクト名/ブランチ名はURLエンコード不要（JSONキーとして有効な文字列）
- 空配列は「全解除」状態を表す（キー自体を削除しない）
- 未保存のcontext_keyは「全選択（デフォルト）」として扱う

## TypeScript Types

```typescript
// Frontend types (src/lib/contracts/tauriCommands.ts)

/**
 * ユーザーフィルタ選択状態のビュー種別
 */
export type UserFilterViewType = 'project-view' | 'cross-view'

/**
 * ユーザーフィルタ選択状態のコンテキストキー
 * - プロジェクト別: "<project>/<branch>/<year>"
 * - 横断: "<year>"
 */
export type UserFilterContextKey = string

/**
 * ユーザー識別子の配列（authorEmail優先、フォールバックauthorName）
 */
export type SelectedUsers = string[]

/**
 * ユーザーフィルタ選択状態取得コマンドの引数
 */
export interface UserFilterGetArgs {
  viewType: UserFilterViewType
  contextKey: UserFilterContextKey
}

/**
 * ユーザーフィルタ選択状態保存コマンドの引数
 */
export interface UserFilterSetArgs {
  viewType: UserFilterViewType
  contextKey: UserFilterContextKey
  selectedUsers: SelectedUsers
}
```

## Rust Structs

```rust
// Backend types (src-tauri/src/storage/user_filter_repository.rs)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ユーザーフィルタ選択状態のビュー種別
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum UserFilterViewType {
    ProjectView,
    CrossView,
}

/// ユーザーフィルタ選択状態のコンテキストキー
pub type UserFilterContextKey = String;

/// ユーザー識別子の配列
pub type SelectedUsers = Vec<String>;

/// ユーザーフィルタ選択状態全体のストレージ構造
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFilterStorage {
    #[serde(rename = "project-view")]
    pub project_view: HashMap<UserFilterContextKey, SelectedUsers>,
    #[serde(rename = "cross-view")]
    pub cross_view: HashMap<UserFilterContextKey, SelectedUsers>,
}

impl Default for UserFilterStorage {
    fn default() -> Self {
        Self {
            project_view: HashMap::new(),
            cross_view: HashMap::new(),
        }
    }
}
```

## Data Flow

```
┌─────────────────────────────────────────────────────────────────┐
│ 1. 初回表示                                                      │
│    StatsFilters → useUserFilter → Tauri: user_filter_get       │
│                                 → 未保存 → 全ユーザー選択       │
└─────────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────────┐
│ 2. チェックボックス操作                                         │
│    UserFilter → useUserFilter(setState)                         │
│              → Tauri: user_filter_set → JSON保存                │
│              → MonthlyBarChart/Table(フィルタ済みデータ再計算)  │
└─────────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────────┐
│ 3. フィルタ条件変更                                             │
│    StatsFilters(project/branch/year変更)                        │
│              → useUserFilter → Tauri: user_filter_get           │
│                             → 保存済み選択状態復元               │
│                             → MonthlyBarChart/Table更新          │
└─────────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────────┐
│ 4. アプリ再起動                                                 │
│    同じプロジェクト/ブランチ/年を選択                           │
│              → useUserFilter → Tauri: user_filter_get           │
│                             → 前回の選択状態復元                 │
└─────────────────────────────────────────────────────────────────┘
```

## Migration Notes

既存データへの影響なし。新規ファイル（`user_filter_state.json`）を作成するのみ。

## Testing Considerations

### データ整合性テスト

- 同一context_keyで複数回保存→最新値で上書き
- 存在しないcontext_keyで取得→空配列（デフォルト全選択）
- 無効なview_type→エラー

### 境界値テスト

- selected_users空配列（全解除）
- selected_users大量（100名以上）
- context_keyに特殊文字（スラッシュ、スペース）

### 状態遷移テスト

- 全選択→一部選択→全解除→全選択のサイクル
- ビュー切り替え（project-view ⇔ cross-view）で独立性確認

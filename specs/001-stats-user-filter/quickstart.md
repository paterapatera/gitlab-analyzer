# Quickstart: 集計表示におけるユーザーフィルタリング

**Feature**: [spec.md](spec.md) | **Plan**: [plan.md](plan.md)  
**Date**: 2026-02-05

このガイドは、ユーザーフィルタリング機能の実装手順を段階的に示します。

## Prerequisites

- Node.js 18+ + npm
- Rust 1.70+
- Tauri CLI
- shadcn/ui設定済み

## Implementation Steps

### Phase 1: shadcn/uiコンポーネント追加

```bash
# Checkboxコンポーネントを追加（存在しない場合）
npx shadcn@latest add checkbox

# ScrollAreaコンポーネントを追加（存在しない場合）
npx shadcn@latest add scroll-area
```

**確認**:

- `src/components/ui/checkbox.tsx` が存在
- `src/components/ui/scroll-area.tsx` が存在

---

### Phase 2: Backend実装（Rust）

#### 2-1. データ構造定義

**ファイル**: `src-tauri/src/storage/user_filter_repository.rs`（新規作成）

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ユーザーフィルタ選択状態のビュー種別
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum UserFilterViewType {
    ProjectView,
    CrossView,
}

/// コンテキストキー（プロジェクト/ブランチ/年 or 年のみ）
pub type UserFilterContextKey = String;

/// 選択されたユーザーID配列
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

#### 2-2. Repository実装

**同ファイル続き**: `src-tauri/src/storage/user_filter_repository.rs`

```rust
use super::json_store::JsonStore;
use super::repository::Result;

/// ユーザーフィルタ選択状態のリポジトリ
pub struct UserFilterRepository {
    store: JsonStore<UserFilterStorage>,
}

impl UserFilterRepository {
    /// 新しいリポジトリインスタンスを作成
    /// NOTE: user_filter_state.json ファイルを読み書きする
    pub fn new() -> Result<Self> {
        let store = JsonStore::new("user_filter_state.json")?;
        Ok(Self { store })
    }

    /// 選択状態を取得
    /// NOTE: 存在しない場合は空配列を返す（全選択として扱う）
    pub fn get(
        &self,
        view_type: &UserFilterViewType,
        context_key: &str,
    ) -> Result<SelectedUsers> {
        let storage = self.store.load()?;
        let map = match view_type {
            UserFilterViewType::ProjectView => &storage.project_view,
            UserFilterViewType::CrossView => &storage.cross_view,
        };
        Ok(map.get(context_key).cloned().unwrap_or_default())
    }

    /// 選択状態を保存
    /// NOTE: 同一のview_type+context_keyが存在する場合は上書き
    pub fn set(
        &mut self,
        view_type: &UserFilterViewType,
        context_key: &str,
        selected_users: SelectedUsers,
    ) -> Result<()> {
        let mut storage = self.store.load()?;
        let map = match view_type {
            UserFilterViewType::ProjectView => &mut storage.project_view,
            UserFilterViewType::CrossView => &mut storage.cross_view,
        };
        map.insert(context_key.to_string(), selected_users);
        self.store.save(&storage)?;
        Ok(())
    }
}
```

#### 2-3. Tauriコマンド実装

**ファイル**: `src-tauri/src/commands/user_filter_get.rs`（新規作成）

```rust
use crate::storage::user_filter_repository::{UserFilterRepository, UserFilterViewType, SelectedUsers};

/// ユーザーフィルタ選択状態を取得
///
/// # Arguments
/// * `view_type` - ビュー種別（"project-view" or "cross-view"）
/// * `context_key` - コンテキストキー（プロジェクト/ブランチ/年 or 年のみ）
///
/// # Returns
/// 選択されたユーザーID配列（存在しない場合は空配列）
#[tauri::command]
pub async fn user_filter_get(
    view_type: String,
    context_key: String,
) -> Result<SelectedUsers, String> {
    let view_type = serde_json::from_str::<UserFilterViewType>(&format!("\"{}\"", view_type))
        .map_err(|e| format!("Invalid view_type: {}", e))?;

    let repo = UserFilterRepository::new()
        .map_err(|e| format!("Failed to initialize repository: {}", e))?;

    repo.get(&view_type, &context_key)
        .map_err(|e| format!("Failed to get user filter state: {}", e))
}
```

**ファイル**: `src-tauri/src/commands/user_filter_set.rs`（新規作成）

```rust
use crate::storage::user_filter_repository::{UserFilterRepository, UserFilterViewType, SelectedUsers};

/// ユーザーフィルタ選択状態を保存
///
/// # Arguments
/// * `view_type` - ビュー種別（"project-view" or "cross-view"）
/// * `context_key` - コンテキストキー（プロジェクト/ブランチ/年 or 年のみ）
/// * `selected_users` - 選択されたユーザーID配列
#[tauri::command]
pub async fn user_filter_set(
    view_type: String,
    context_key: String,
    selected_users: SelectedUsers,
) -> Result<(), String> {
    let view_type = serde_json::from_str::<UserFilterViewType>(&format!("\"{}\"", view_type))
        .map_err(|e| format!("Invalid view_type: {}", e))?;

    let mut repo = UserFilterRepository::new()
        .map_err(|e| format!("Failed to initialize repository: {}", e))?;

    repo.set(&view_type, &context_key, selected_users)
        .map_err(|e| format!("Failed to set user filter state: {}", e))
}
```

#### 2-4. コマンド登録

**ファイル**: `src-tauri/src/commands/mod.rs`（既存ファイルに追加）

```rust
mod user_filter_get;
mod user_filter_set;

pub use user_filter_get::user_filter_get;
pub use user_filter_set::user_filter_set;
```

**ファイル**: `src-tauri/src/main.rs`（既存ファイルに追加）

```rust
// invoke_handlerにコマンド追加
.invoke_handler(tauri::generate_handler![
    // ...既存のコマンド...
    user_filter_get,
    user_filter_set,
])
```

**ファイル**: `src-tauri/src/storage/mod.rs`（既存ファイルに追加）

```rust
pub mod user_filter_repository;
```

---

### Phase 3: Frontend実装（React/TypeScript）

#### 3-1. Tauri契約型定義

**ファイル**: `src/lib/contracts/tauriCommands.ts`（既存ファイルに追加）

```typescript
/**
 * ユーザーフィルタ選択状態のビュー種別
 */
export type UserFilterViewType = 'project-view' | 'cross-view'

/**
 * ユーザーフィルタ選択状態のコンテキストキー
 */
export type UserFilterContextKey = string

/**
 * 選択されたユーザーID配列
 */
export type SelectedUsers = string[]

/**
 * ユーザーフィルタ選択状態を取得
 */
export async function getUserFilterState(
  viewType: UserFilterViewType,
  contextKey: UserFilterContextKey,
): Promise<SelectedUsers> {
  return invoke<SelectedUsers>('user_filter_get', {
    viewType,
    contextKey,
  })
}

/**
 * ユーザーフィルタ選択状態を保存
 */
export async function setUserFilterState(
  viewType: UserFilterViewType,
  contextKey: UserFilterContextKey,
  selectedUsers: SelectedUsers,
): Promise<void> {
  return invoke('user_filter_set', {
    viewType,
    contextKey,
    selectedUsers,
  })
}
```

#### 3-2. カスタムhook実装

**ファイル**: `src/features/stats/useUserFilter.ts`（新規作成）

```typescript
import { useState, useEffect, useCallback } from 'react'
import {
  getUserFilterState,
  setUserFilterState,
  type UserFilterViewType,
} from '@/lib/contracts/tauriCommands'

/**
 * ユーザーフィルタリング状態を管理するカスタムhook
 *
 * NOTE: 選択状態はプロジェクト/ブランチ/年の組み合わせごとに永続化される
 *
 * @param viewType - ビュー種別（"project-view" or "cross-view"）
 * @param contextKey - コンテキストキー（プロジェクト/ブランチ/年 or 年のみ）
 * @param availableUsers - 表示可能なユーザーID配列
 */
export function useUserFilter(
  viewType: UserFilterViewType,
  contextKey: string,
  availableUsers: string[],
) {
  const [selectedUsers, setSelectedUsersState] = useState<string[]>([])
  const [isLoading, setIsLoading] = useState(true)

  // 初回読み込み: 保存済み選択状態を復元
  useEffect(() => {
    let isCancelled = false

    async function loadState() {
      try {
        setIsLoading(true)
        const saved = await getUserFilterState(viewType, contextKey)

        if (!isCancelled) {
          // 保存済みデータが存在しない場合は全選択
          if (saved.length === 0) {
            setSelectedUsersState(availableUsers)
          } else {
            // 保存済みデータを復元（存在するユーザーのみ）
            const validUsers = saved.filter((id) => availableUsers.includes(id))
            setSelectedUsersState(validUsers)
          }
        }
      } catch (error) {
        console.error('Failed to load user filter state:', error)
        // エラー時は全選択
        if (!isCancelled) {
          setSelectedUsersState(availableUsers)
        }
      } finally {
        if (!isCancelled) {
          setIsLoading(false)
        }
      }
    }

    loadState()

    return () => {
      isCancelled = true
    }
  }, [viewType, contextKey, availableUsers])

  // 選択状態を変更して保存
  const setSelectedUsers = useCallback(
    async (users: string[]) => {
      setSelectedUsersState(users)
      try {
        await setUserFilterState(viewType, contextKey, users)
      } catch (error) {
        console.error('Failed to save user filter state:', error)
      }
    },
    [viewType, contextKey],
  )

  // 全選択
  const selectAll = useCallback(() => {
    setSelectedUsers(availableUsers)
  }, [availableUsers, setSelectedUsers])

  // 全解除
  const deselectAll = useCallback(() => {
    setSelectedUsers([])
  }, [setSelectedUsers])

  // トグル（1ユーザー）
  const toggleUser = useCallback(
    (userId: string) => {
      const newSelection = selectedUsers.includes(userId)
        ? selectedUsers.filter((id) => id !== userId)
        : [...selectedUsers, userId]
      setSelectedUsers(newSelection)
    },
    [selectedUsers, setSelectedUsers],
  )

  return {
    selectedUsers,
    isLoading,
    setSelectedUsers,
    selectAll,
    deselectAll,
    toggleUser,
  }
}
```

#### 3-3. UserFilterコンポーネント実装

**ファイル**: `src/features/stats/UserFilter.tsx`（新規作成）

```typescript
import { Checkbox } from "@/components/ui/checkbox";
import { Button } from "@/components/ui/button";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Label } from "@/components/ui/label";

interface UserFilterProps {
  /** 表示可能な全ユーザー（ユーザーID配列） */
  availableUsers: Array<{ id: string; name: string }>;
  /** 選択されたユーザーID配列 */
  selectedUsers: string[];
  /** チェックボックストグル時のコールバック */
  onToggleUser: (userId: string) => void;
  /** 全選択時のコールバック */
  onSelectAll: () => void;
  /** 全解除時のコールバック */
  onDeselectAll: () => void;
  /** ローディング中フラグ */
  isLoading?: boolean;
}

/**
 * ユーザーフィルタリングUI
 *
 * NOTE: チェックボックスはアルファベット順（昇順）で表示される
 * NOTE: 50名以上の場合はスクロール可能なコンテナに配置される
 */
export function UserFilter({
  availableUsers,
  selectedUsers,
  onToggleUser,
  onSelectAll,
  onDeselectAll,
  isLoading = false,
}: UserFilterProps) {
  // ユーザー名でアルファベット順ソート
  const sortedUsers = [...availableUsers].sort((a, b) =>
    a.name.localeCompare(b.name)
  );

  // 50名以上の場合はスクロール可能にする
  const needsScroll = sortedUsers.length >= 50;

  const checkboxList = (
    <div className="space-y-2">
      {sortedUsers.map((user) => (
        <div key={user.id} className="flex items-center space-x-2">
          <Checkbox
            id={`user-${user.id}`}
            checked={selectedUsers.includes(user.id)}
            onCheckedChange={() => onToggleUser(user.id)}
            disabled={isLoading}
          />
          <Label
            htmlFor={`user-${user.id}`}
            className="text-sm font-normal cursor-pointer"
          >
            {user.name}
          </Label>
        </div>
      ))}
    </div>
  );

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <h3 className="text-sm font-medium">ユーザー選択</h3>
        <div className="flex gap-2">
          <Button
            variant="outline"
            size="sm"
            onClick={onSelectAll}
            disabled={isLoading}
          >
            全選択
          </Button>
          <Button
            variant="outline"
            size="sm"
            onClick={onDeselectAll}
            disabled={isLoading}
          >
            全解除
          </Button>
        </div>
      </div>

      {selectedUsers.length === 0 && (
        <p className="text-sm text-muted-foreground">
          ユーザーを選択してください
        </p>
      )}

      {needsScroll ? (
        <ScrollArea className="h-[400px] pr-4">
          {checkboxList}
        </ScrollArea>
      ) : (
        checkboxList
      )}
    </div>
  );
}
```

#### 3-4. StatsFiltersに統合

**ファイル**: `src/features/stats/StatsFilters.tsx`（既存ファイルに追加）

```typescript
import { UserFilter } from "./UserFilter";
import { useUserFilter } from "./useUserFilter";

// 既存のStatsFiltersコンポーネントに以下を追加

// propsに追加
interface StatsFiltersProps {
  // ...既存のprops...
  availableUsers: Array<{ id: string; name: string }>; // 追加
  onUserFilterChange: (selectedUsers: string[]) => void; // 追加
  viewType: "project-view" | "cross-view"; // 追加
  contextKey: string; // 追加（プロジェクト/ブランチ/年 or 年のみ）
}

export function StatsFilters({
  // ...既存のprops...
  availableUsers,
  onUserFilterChange,
  viewType,
  contextKey,
}: StatsFiltersProps) {
  // ユーザーフィルタhook
  const {
    selectedUsers,
    isLoading: isFilterLoading,
    selectAll,
    deselectAll,
    toggleUser,
  } = useUserFilter(
    viewType,
    contextKey,
    availableUsers.map((u) => u.id)
  );

  // 選択変更時に親に通知
  useEffect(() => {
    if (!isFilterLoading) {
      onUserFilterChange(selectedUsers);
    }
  }, [selectedUsers, isFilterLoading, onUserFilterChange]);

  return (
    <div className="space-y-4">
      {/* 既存のフィルタ（年選択など） */}

      {/* ユーザーフィルタ追加 */}
      <UserFilter
        availableUsers={availableUsers}
        selectedUsers={selectedUsers}
        onToggleUser={toggleUser}
        onSelectAll={selectAll}
        onDeselectAll={deselectAll}
        isLoading={isFilterLoading}
      />
    </div>
  );
}
```

---

### Phase 4: テスト実装

#### 4-1. Backend テスト

**ファイル**: `src-tauri/src/storage/user_filter_repository.rs`（テスト追加）

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_get_set_project_view() {
        // テスト用リポジトリ作成
        let mut repo = UserFilterRepository::new().unwrap();

        // 初期状態は空配列
        let result = repo.get(&UserFilterViewType::ProjectView, "test/main/2025").unwrap();
        assert_eq!(result, Vec::<String>::new());

        // 保存
        let users = vec!["user1@example.com".to_string(), "user2@example.com".to_string()];
        repo.set(&UserFilterViewType::ProjectView, "test/main/2025", users.clone()).unwrap();

        // 取得確認
        let result = repo.get(&UserFilterViewType::ProjectView, "test/main/2025").unwrap();
        assert_eq!(result, users);
    }

    #[test]
    fn test_independent_views() {
        let mut repo = UserFilterRepository::new().unwrap();

        // プロジェクト別ビュー
        repo.set(&UserFilterViewType::ProjectView, "test/main/2025", vec!["user1".to_string()]).unwrap();

        // 横断ビュー
        repo.set(&UserFilterViewType::CrossView, "2025", vec!["user2".to_string()]).unwrap();

        // 独立性確認
        let project = repo.get(&UserFilterViewType::ProjectView, "test/main/2025").unwrap();
        let cross = repo.get(&UserFilterViewType::CrossView, "2025").unwrap();
        assert_eq!(project, vec!["user1"]);
        assert_eq!(cross, vec!["user2"]);
    }
}
```

#### 4-2. Frontend テスト

**ファイル**: `src/features/stats/UserFilter.test.tsx`（新規作成）

```typescript
import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { UserFilter } from "./UserFilter";

describe("UserFilter", () => {
  const availableUsers = [
    { id: "user1@example.com", name: "Alice" },
    { id: "user2@example.com", name: "Bob" },
    { id: "user3", name: "Charlie" },
  ];

  it("アルファベット順で表示される", () => {
    const { container } = render(
      <UserFilter
        availableUsers={availableUsers}
        selectedUsers={[]}
        onToggleUser={vi.fn()}
        onSelectAll={vi.fn()}
        onDeselectAll={vi.fn()}
      />
    );

    const labels = container.querySelectorAll("label");
    expect(labels[0].textContent).toBe("Alice");
    expect(labels[1].textContent).toBe("Bob");
    expect(labels[2].textContent).toBe("Charlie");
  });

  it("全選択ボタンで全ユーザーが選択される", async () => {
    const onSelectAll = vi.fn();
    render(
      <UserFilter
        availableUsers={availableUsers}
        selectedUsers={[]}
        onToggleUser={vi.fn()}
        onSelectAll={onSelectAll}
        onDeselectAll={vi.fn()}
      />
    );

    const selectAllBtn = screen.getByText("全選択");
    await userEvent.click(selectAllBtn);

    expect(onSelectAll).toHaveBeenCalledTimes(1);
  });

  it("チェックボックスクリックでトグルされる", async () => {
    const onToggleUser = vi.fn();
    render(
      <UserFilter
        availableUsers={availableUsers}
        selectedUsers={["user1@example.com"]}
        onToggleUser={onToggleUser}
        onSelectAll={vi.fn()}
        onDeselectAll={vi.fn()}
      />
    );

    const aliceCheckbox = screen.getByLabelText("Alice");
    await userEvent.click(aliceCheckbox);

    expect(onToggleUser).toHaveBeenCalledWith("user1@example.com");
  });
});
```

---

## Testing

```bash
# Frontend テスト
npm test

# Backend テスト
cd src-tauri
cargo test

# 開発モード起動
npm run tauri dev
```

## Verification Checklist

- [ ] shadcn Checkbox/ScrollArea コンポーネント追加完了
- [ ] Backend: UserFilterRepository 実装・テスト完了
- [ ] Backend: Tauri コマンド登録完了
- [ ] Frontend: useUserFilter hook 実装・テスト完了
- [ ] Frontend: UserFilter コンポーネント実装・テスト完了
- [ ] StatsFilters に統合完了
- [ ] 選択状態がプロジェクト/ブランチ/年ごとに永続化されることを確認
- [ ] アプリ再起動後も選択状態が復元されることを確認
- [ ] チェックボックス操作後1秒以内にグラフ/テーブル更新を確認

## Troubleshooting

### 選択状態が保存されない

- Tauri コマンドが正しく登録されているか確認
- `user_filter_state.json` の権限を確認
- ブラウザコンソールでエラーログを確認

### パフォーマンスが遅い

- React.memo を UserFilter に適用
- ScrollArea の高さ制限を調整
- 100名以上で問題がある場合は仮想化を検討

### テストが失敗する

- Tauri のモック設定を確認（`__mocks__/@tauri-apps/api.ts`）
- テストファイルで invoke がモックされているか確認

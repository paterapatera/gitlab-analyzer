/**
 * ユーザーフィルタリング状態を管理するカスタムhook
 *
 * 選択状態はプロジェクト/ブランチ/年の組み合わせごとに永続化される。
 * ビュー種別（project-view / cross-view）で独立した選択状態を持つ。
 */

import { useState, useEffect, useCallback, useMemo } from 'react'
import {
  getUserFilterState,
  setUserFilterState,
  type UserFilterViewType,
  type SelectedUsers,
} from '@/lib/contracts/tauriCommands'

/** useUserFilter hookの引数 */
export interface UseUserFilterOptions {
  /** ビュー種別（"project-view" or "cross-view"） */
  viewType: UserFilterViewType
  /** コンテキストキー（プロジェクト/ブランチ/年 or 年のみ） */
  contextKey: string
  /** 表示可能なユーザーの配列 */
  availableUsers: Array<{ userKey: string; displayName: string }>
}

/** useUserFilter hookの戻り値 */
export interface UseUserFilterResult {
  /** 選択されたユーザーKey配列 */
  selectedUsers: SelectedUsers
  /** ローディング中フラグ */
  isLoading: boolean
  /** 選択状態を直接設定 */
  setSelectedUsers: (users: SelectedUsers) => Promise<void>
  /** 全選択 */
  selectAll: () => Promise<void>
  /** 全解除 */
  deselectAll: () => Promise<void>
  /** 1ユーザーをトグル */
  toggleUser: (userKey: string) => Promise<void>
}

/**
 * ユーザーフィルタリング状態を管理するカスタムhook
 *
 * NOTE: 初回読み込み時に保存済みデータを復元
 * NOTE: 選択変更時に自動で永続化
 * NOTE: 未保存の場合は全選択がデフォルト
 *
 * @param options - hook のオプション
 * @returns フィルタ状態と操作関数
 */
export function useUserFilter({
  viewType,
  contextKey,
  availableUsers,
}: UseUserFilterOptions): UseUserFilterResult {
  const [selectedUsers, setSelectedUsersState] = useState<SelectedUsers>([])
  const [isLoading, setIsLoading] = useState(true)

  // 利用可能なユーザーKeyの配列をメモ化
  const availableUserKeys = useMemo(() => availableUsers.map((u) => u.userKey), [availableUsers])

  // 初回読み込み: 保存済み選択状態を復元
  useEffect(() => {
    let isCancelled = false

    async function loadState() {
      try {
        setIsLoading(true)
        const saved = await getUserFilterState(viewType, contextKey)

        if (!isCancelled) {
          if (saved.length === 0) {
            // 未保存の場合は全選択
            setSelectedUsersState(availableUserKeys)
          } else {
            // 保存済みデータを復元（現在も存在するユーザーのみ）
            const validUsers = saved.filter((key) => availableUserKeys.includes(key))
            setSelectedUsersState(validUsers.length > 0 ? validUsers : availableUserKeys)
          }
        }
      } catch (error) {
        console.error('Failed to load user filter state:', error)
        // エラー時は全選択
        if (!isCancelled) {
          setSelectedUsersState(availableUserKeys)
        }
      } finally {
        if (!isCancelled) {
          setIsLoading(false)
        }
      }
    }

    if (availableUserKeys.length > 0) {
      loadState()
    } else {
      setSelectedUsersState([])
      setIsLoading(false)
    }

    return () => {
      isCancelled = true
    }
  }, [viewType, contextKey, availableUserKeys])

  // 選択状態を変更して保存
  const setSelectedUsers = useCallback(
    async (users: SelectedUsers) => {
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
  const selectAll = useCallback(async () => {
    await setSelectedUsers(availableUserKeys)
  }, [availableUserKeys, setSelectedUsers])

  // 全解除
  const deselectAll = useCallback(async () => {
    await setSelectedUsers([])
  }, [setSelectedUsers])

  // 1ユーザーをトグル
  const toggleUser = useCallback(
    async (userKey: string) => {
      const newSelection = selectedUsers.includes(userKey)
        ? selectedUsers.filter((key) => key !== userKey)
        : [...selectedUsers, userKey]
      await setSelectedUsers(newSelection)
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

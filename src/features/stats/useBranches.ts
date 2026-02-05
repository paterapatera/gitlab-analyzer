/**
 * ブランチ一覧を取得・管理するカスタムhook
 *
 * プロジェクト選択時に自動的にブランチ一覧を読み込み、
 * デフォルトブランチを自動選択する。
 */

import { useState, useEffect, useCallback } from 'react'
import { invokeCommand } from '@/lib/tauri'
import type { Branch } from '@/lib/contracts/tauriCommands'

/** useBranches hookの引数 */
export interface UseBranchesOptions {
  /** 対象プロジェクトID（null/undefinedで未選択） */
  projectId: number | null | undefined
}

/** useBranches hookの戻り値 */
export interface UseBranchesResult {
  /** ブランチ一覧 */
  branches: Branch[]
  /** 選択中のブランチ名 */
  selectedBranch: string
  /** ブランチ選択を変更 */
  setSelectedBranch: (branchName: string) => void
  /** 読み込み中フラグ */
  isLoading: boolean
  /** エラーメッセージ */
  error: string | null
}

/**
 * ブランチ一覧を取得・管理するカスタムhook
 *
 * @param options - hook のオプション
 * @returns ブランチ状態と操作関数
 */
export function useBranches({ projectId }: UseBranchesOptions): UseBranchesResult {
  const [branches, setBranches] = useState<Branch[]>([])
  const [selectedBranch, setSelectedBranch] = useState('')
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  // プロジェクト選択時にブランチ一覧を読み込む
  useEffect(() => {
    if (!projectId) {
      setBranches([])
      setSelectedBranch('')
      setError(null)
      return
    }

    let isCancelled = false

    const loadBranches = async () => {
      setIsLoading(true)
      setError(null)

      try {
        const result = await invokeCommand<Branch[]>('list_branches', {
          projectId,
        })

        if (isCancelled) return

        if (!result.ok) {
          setError(result.error)
          setBranches([])
          setSelectedBranch('')
          return
        }

        setBranches(result.data)

        // デフォルトブランチを自動選択
        const defaultBranch = result.data.find((b) => b.isDefault)
        if (defaultBranch) {
          setSelectedBranch(defaultBranch.name)
        } else if (result.data.length > 0) {
          setSelectedBranch(result.data[0].name)
        } else {
          setSelectedBranch('')
        }
      } finally {
        if (!isCancelled) {
          setIsLoading(false)
        }
      }
    }

    loadBranches()

    return () => {
      isCancelled = true
    }
  }, [projectId])

  const handleSetSelectedBranch = useCallback((branchName: string) => {
    setSelectedBranch(branchName)
  }, [])

  return {
    branches,
    selectedBranch,
    setSelectedBranch: handleSetSelectedBranch,
    isLoading,
    error,
  }
}

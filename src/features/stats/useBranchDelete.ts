/**
 * ブランチ削除アクションフック
 *
 * 指定プロジェクト/ブランチのコミットを削除し、結果を返す。
 */

import { useState, useCallback } from 'react'
import {
  deleteBranchCommits,
  type DeleteBranchRequest,
  type DeleteBranchResponse,
} from '@/lib/contracts/tauriCommands'

/** フックの戻り値 */
export interface UseBranchDeleteReturn {
  /** 削除を実行 */
  execute: (request: DeleteBranchRequest) => Promise<DeleteBranchResponse>
  /** 実行中フラグ */
  isDeleting: boolean
  /** エラーメッセージ（失敗時） */
  error: string | null
  /** エラーをクリア */
  clearError: () => void
}

/**
 * ブランチ削除フック
 *
 * @returns 削除アクションの状態と実行関数
 */
export function useBranchDelete(): UseBranchDeleteReturn {
  const [isDeleting, setIsDeleting] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const execute = useCallback(async (request: DeleteBranchRequest) => {
    setIsDeleting(true)
    setError(null)
    try {
      const result = await deleteBranchCommits(request)
      return result
    } catch (err) {
      const message = err instanceof Error ? err.message : '削除に失敗しました'
      setError(message)
      throw err
    } finally {
      setIsDeleting(false)
    }
  }, [])

  const clearError = useCallback(() => setError(null), [])

  return { execute, isDeleting, error, clearError }
}

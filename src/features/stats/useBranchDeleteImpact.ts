/**
 * ブランチ削除影響サマリ取得フック
 *
 * 削除ダイアログを開いた際に、影響件数とブロック理由を取得する。
 */

import { useState, useCallback } from 'react'
import {
  getBranchDeleteImpact,
  type DeleteBranchImpactRequest,
  type DeleteBranchImpactResponse,
} from '@/lib/contracts/tauriCommands'

/** フックの戻り値 */
export interface UseBranchDeleteImpactReturn {
  /** 影響サマリ */
  impact: DeleteBranchImpactResponse | null
  /** 取得中フラグ */
  isLoading: boolean
  /** エラーメッセージ */
  error: string | null
  /** 影響サマリを取得 */
  fetchImpact: (request: DeleteBranchImpactRequest) => Promise<void>
}

/**
 * ブランチ削除影響サマリフック
 *
 * @returns 影響サマリの状態と取得関数
 */
export function useBranchDeleteImpact(): UseBranchDeleteImpactReturn {
  const [impact, setImpact] = useState<DeleteBranchImpactResponse | null>(null)
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const fetchImpact = useCallback(async (request: DeleteBranchImpactRequest) => {
    setIsLoading(true)
    setError(null)
    setImpact(null)
    try {
      const result = await getBranchDeleteImpact(request)
      setImpact(result)
    } catch (err) {
      const message = err instanceof Error ? err.message : '影響サマリの取得に失敗しました'
      setError(message)
    } finally {
      setIsLoading(false)
    }
  }, [])

  return { impact, isLoading, error, fetchImpact }
}

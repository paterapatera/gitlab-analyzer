/**
 * 集計データ取得・管理用カスタムhook
 *
 * プロジェクトビュー / 横断ビューの集計データを取得し、
 * フィルタ済みデータを提供する。
 */

import { useState, useCallback, useMemo } from 'react'
import { invokeCommand } from '@/lib/tauri'
import type {
  MonthlyStatsResponse,
  ProjectViewStatsRequest,
  CrossViewStatsRequest,
} from '@/lib/contracts/tauriCommands'

/** ビューの種類 */
export type StatsView = 'project' | 'cross'

/** useStatsData hookの引数 */
export interface UseStatsDataOptions {
  /** ビュー種別 */
  statsView: StatsView
  /** 選択中のプロジェクトID */
  projectId: number | null | undefined
  /** 選択中のブランチ名 */
  branchName: string
  /** 年 */
  year: number
}

/** 利用可能なユーザー情報 */
export interface AvailableUser {
  userKey: string
  displayName: string
}

/** useStatsData hookの戻り値 */
export interface UseStatsDataResult {
  /** 集計データ（フィルタ前） */
  statsData: MonthlyStatsResponse | null
  /** 読み込み中フラグ */
  isLoading: boolean
  /** データを再読み込み */
  loadStats: () => Promise<void>
  /** 利用可能なユーザー一覧 */
  availableUsers: AvailableUser[]
  /** 選択ユーザーでフィルタ済みデータを取得 */
  getFilteredData: (selectedUsers: string[]) => MonthlyStatsResponse | null
}

/**
 * 集計データ取得・管理用カスタムhook
 *
 * @param options - hook のオプション
 * @returns 集計データと操作関数
 */
export function useStatsData({
  statsView,
  projectId,
  branchName,
  year,
}: UseStatsDataOptions): UseStatsDataResult {
  const [statsData, setStatsData] = useState<MonthlyStatsResponse | null>(null)
  const [isLoading, setIsLoading] = useState(false)

  // 利用可能なユーザー（集計データから抽出）
  const availableUsers = useMemo<AvailableUser[]>(() => {
    if (!statsData) return []
    return statsData.series.map((s) => ({
      userKey: s.userKey,
      displayName: s.displayName,
    }))
  }, [statsData])

  // 集計データを取得
  const loadStats = useCallback(async () => {
    // プロジェクトビューで必要なパラメータがない場合はスキップ
    if (statsView === 'project' && (!projectId || !branchName)) {
      return
    }

    setIsLoading(true)

    try {
      if (statsView === 'project' && projectId && branchName) {
        const request: ProjectViewStatsRequest = {
          projectId,
          branchName,
          year,
        }
        const result = await invokeCommand<MonthlyStatsResponse>('get_monthly_stats_project_view', {
          request,
        })
        if (result.ok) {
          setStatsData(result.data)
        }
      } else if (statsView === 'cross') {
        const request: CrossViewStatsRequest = {
          year,
        }
        const result = await invokeCommand<MonthlyStatsResponse>('get_monthly_stats_cross_view', {
          request,
        })
        if (result.ok) {
          setStatsData(result.data)
        }
      }
    } finally {
      setIsLoading(false)
    }
  }, [statsView, projectId, branchName, year])

  // フィルタ済みデータを取得
  const getFilteredData = useCallback(
    (selectedUsers: string[]): MonthlyStatsResponse | null => {
      if (!statsData) return null
      return {
        months: statsData.months,
        series: statsData.series.filter((s) => selectedUsers.includes(s.userKey)),
      }
    },
    [statsData],
  )

  return {
    statsData,
    isLoading,
    loadStats,
    availableUsers,
    getFilteredData,
  }
}

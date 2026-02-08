/**
 * 集計表示タブ
 *
 * プロジェクト別ビュー / 横断ビューの月次集計を表示する。
 * フィルタ、グラフ、テーブルを含む。
 */

import { useState, useEffect, useMemo, useCallback } from 'react'
import type { Project, UserFilterViewType } from '@/lib/contracts/tauriCommands'
import { MonthlyBarChart } from '@/features/stats/MonthlyBarChart'
import { MonthlyTable } from '@/features/stats/MonthlyTable'
import { MissingStatsNotice } from '@/features/stats/MissingStatsNotice'
import { UserFilter } from '@/features/stats/UserFilter'
import { useUserFilter } from '@/features/stats/useUserFilter'
import { useBranches } from '@/features/stats/useBranches'
import { useStatsData, type StatsView } from '@/features/stats/useStatsData'
import { useBranchDeleteImpact } from '@/features/stats/useBranchDeleteImpact'
import { StatsFilterCard } from '@/features/stats/StatsFilterCard'
import { SelectionAlerts } from '@/features/stats/SelectionAlerts'
import { Card, CardAction, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert'
import { Button } from '@/components/ui/button'
import { buildMonthlyTableTsv } from '@/features/stats/buildMonthlyTableTsv'
import { Copy } from 'lucide-react'

/** StatsTab のプロパティ */
export interface StatsTabProps {
  /** プロジェクト一覧（選択用） */
  projects: Project[]
}

/**
 * 集計表示タブコンポーネント
 */
export function StatsTab({ projects }: StatsTabProps) {
  // 状態管理
  const [statsView, setStatsView] = useState<StatsView>('project')
  const [selectedProject, setSelectedProject] = useState<Project | null>(null)
  const [statsYear, setStatsYear] = useState(new Date().getFullYear())
  const [copySuccess, setCopySuccess] = useState<{
    at: number
    headerOnly: boolean
  } | null>(null)

  // ブランチ管理
  const { branches, selectedBranch, setSelectedBranch } = useBranches({
    projectId: selectedProject?.projectId,
  })

  // 集計データ管理
  const {
    statsData,
    isLoading: isLoadingStats,
    loadStats,
    availableUsers,
    getFilteredData,
  } = useStatsData({
    statsView,
    projectId: selectedProject?.projectId,
    branchName: selectedBranch,
    year: statsYear,
  })

  // ユーザーフィルタ用のcontextKey生成
  const userFilterViewType: UserFilterViewType =
    statsView === 'project' ? 'project-view' : 'cross-view'

  const userFilterContextKey = useMemo(() => {
    if (statsView === 'project' && selectedProject && selectedBranch) {
      return `${selectedProject.name}/${selectedBranch}/${statsYear}`
    }
    return `${statsYear}`
  }, [statsView, selectedProject, selectedBranch, statsYear])

  // ユーザーフィルタhook
  const {
    selectedUsers,
    isLoading: isLoadingUserFilter,
    selectAll,
    deselectAll,
    setSelectedUsers,
  } = useUserFilter({
    viewType: userFilterViewType,
    contextKey: userFilterContextKey,
    availableUsers,
  })

  // フィルタ済み集計データ
  const filteredStatsData = useMemo(
    () => getFilteredData(selectedUsers),
    [getFilteredData, selectedUsers],
  )

  // 初回読み込み
  useEffect(() => {
    loadStats()
  }, [loadStats])

  useEffect(() => {
    if (copySuccess === null) {
      return
    }

    const timer = window.setTimeout(() => {
      setCopySuccess(null)
    }, 2000)

    return () => window.clearTimeout(timer)
  }, [copySuccess])

  // プロジェクト変更ハンドラ
  const handleProjectSelect = useCallback((project: Project) => {
    setSelectedProject(project)
  }, [])

  // ブランチ削除影響サマリ
  const {
    impact: branchDeleteImpact,
    isLoading: isLoadingBranchDeleteImpact,
    fetchImpact: fetchBranchDeleteImpact,
  } = useBranchDeleteImpact()

  /** 影響サマリ取得をリクエスト */
  const handleRequestBranchDeleteImpact = useCallback(() => {
    if (selectedProject && selectedBranch) {
      fetchBranchDeleteImpact({
        projectId: selectedProject.projectId,
        branchName: selectedBranch,
      })
    }
  }, [selectedProject, selectedBranch, fetchBranchDeleteImpact])

  /** 削除完了後に集計を再取得 */
  const handleBranchDeleted = useCallback(() => {
    loadStats()
  }, [loadStats])

  const handleCopyMonthlyTable = useCallback(async () => {
    if (!filteredStatsData) {
      return
    }

    try {
      const tsv = buildMonthlyTableTsv(filteredStatsData)
      await navigator.clipboard.writeText(tsv)
      const isHeaderOnly = filteredStatsData.series.length === 0
      setCopySuccess({ at: Date.now(), headerOnly: isHeaderOnly })
    } catch (error) {
      console.error('Failed to copy monthly table TSV.', error)
    }
  }, [filteredStatsData])

  return (
    <div className="space-y-6">
      {/* フィルタ */}
      <StatsFilterCard
        statsView={statsView}
        onStatsViewChange={setStatsView}
        projects={projects}
        selectedProject={selectedProject}
        onProjectSelect={handleProjectSelect}
        branches={branches}
        selectedBranch={selectedBranch}
        onBranchChange={(e) => setSelectedBranch(e.target.value)}
        statsYear={statsYear}
        onYearChange={setStatsYear}
        isLoadingStats={isLoadingStats}
        onRefresh={loadStats}
        onBranchDeleted={handleBranchDeleted}
        branchDeleteImpact={branchDeleteImpact}
        isLoadingBranchDeleteImpact={isLoadingBranchDeleteImpact}
        onRequestBranchDeleteImpact={handleRequestBranchDeleteImpact}
      />

      {/* プロジェクト・ブランチ選択案内 */}
      <SelectionAlerts
        statsView={statsView}
        selectedProject={selectedProject}
        selectedBranch={selectedBranch}
      />

      {/* 欠損通知 */}
      {statsData && <MissingStatsNotice data={statsData} />}

      {/* ユーザーフィルタ + グラフ + 表 */}
      {statsData && (
        <div className="grid gap-6 lg:grid-cols-[280px_1fr]">
          {/* ユーザーフィルタ */}
          <Card>
            <CardContent className="pt-6">
              <UserFilter
                availableUsers={availableUsers}
                selectedUsers={selectedUsers}
                onSelectionChange={setSelectedUsers}
                onSelectAll={selectAll}
                onDeselectAll={deselectAll}
                isLoading={isLoadingUserFilter}
              />
            </CardContent>
          </Card>

          <div className="space-y-6">
            {/* グラフ */}
            <Card>
              <CardHeader>
                <CardTitle>月次コミット行数</CardTitle>
              </CardHeader>
              <CardContent>
                <MonthlyBarChart data={filteredStatsData!} />
              </CardContent>
            </Card>

            {/* 表 */}
            <Card>
              <CardHeader className="grid-rows-[auto_auto_auto]">
                {copySuccess !== null && (
                  <Alert>
                    <AlertTitle>コピー完了</AlertTitle>
                    <AlertDescription>
                      {copySuccess.headerOnly
                        ? 'ヘッダーのみコピーしました。'
                        : '詳細データをクリップボードにコピーしました。'}
                    </AlertDescription>
                  </Alert>
                )}
                <CardTitle>詳細データ</CardTitle>
                <CardAction>
                  <Button
                    variant="ghost"
                    size="icon"
                    aria-label="詳細データをコピー"
                    title="詳細データをコピー"
                    onClick={handleCopyMonthlyTable}
                  >
                    <Copy className="h-4 w-4" />
                  </Button>
                </CardAction>
              </CardHeader>
              <CardContent>
                <MonthlyTable data={filteredStatsData!} />
              </CardContent>
            </Card>
          </div>
        </div>
      )}
    </div>
  )
}

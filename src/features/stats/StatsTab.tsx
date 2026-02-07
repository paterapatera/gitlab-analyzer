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
import { StatsFilterCard } from '@/features/stats/StatsFilterCard'
import { SelectionAlerts } from '@/features/stats/SelectionAlerts'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'

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

  // プロジェクト変更ハンドラ
  const handleProjectSelect = useCallback((project: Project) => {
    setSelectedProject(project)
  }, [])

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
              <CardHeader>
                <CardTitle>詳細データ</CardTitle>
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

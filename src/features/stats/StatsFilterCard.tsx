/**
 * 集計フィルタカードコンポーネント
 *
 * ビュー選択、プロジェクト/ブランチ選択、年度選択、更新ボタンを含むフィルタUI。
 */

import type { Project } from '@/lib/contracts/tauriCommands'
import type { StatsView } from '@/features/stats/useStatsData'
import { StatsFilters } from '@/features/stats/StatsFilters'
import { ViewSelector } from '@/features/stats/ViewSelector'
import { ProjectBranchSelector } from '@/features/stats/ProjectBranchSelector'
import { BranchDeleteDialog } from '@/features/stats/BranchDeleteDialog'
import { Card, CardContent } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import type { DeleteBranchImpactResponse } from '@/lib/contracts/tauriCommands'

/** フィルタカードのプロパティ */
export interface StatsFilterCardProps {
  statsView: StatsView
  onStatsViewChange: (view: StatsView) => void
  projects: Project[]
  selectedProject: Project | null
  onProjectSelect: (project: Project) => void
  branches: Array<{ name: string }>
  selectedBranch: string
  onBranchChange: (e: React.ChangeEvent<HTMLSelectElement>) => void
  statsYear: number
  onYearChange: (year: number) => void
  isLoadingStats: boolean
  onRefresh: () => void
  /** 削除後のコールバック */
  onBranchDeleted?: () => void
  /** 影響サマリ（US2） */
  branchDeleteImpact?: DeleteBranchImpactResponse | null
  /** 影響サマリ取得中フラグ */
  isLoadingBranchDeleteImpact?: boolean
  /** 影響サマリ取得コールバック */
  onRequestBranchDeleteImpact?: () => void
}

/** フィルタカードコンポーネント */
export function StatsFilterCard({
  statsView,
  onStatsViewChange,
  projects,
  selectedProject,
  onProjectSelect,
  branches,
  selectedBranch,
  onBranchChange,
  statsYear,
  onYearChange,
  isLoadingStats,
  onRefresh,
  onBranchDeleted,
  branchDeleteImpact,
  isLoadingBranchDeleteImpact,
  onRequestBranchDeleteImpact,
}: StatsFilterCardProps) {
  return (
    <Card>
      <CardContent className="flex flex-wrap items-center justify-between gap-4 pt-6">
        <div className="flex flex-wrap items-center gap-6">
          {/* ビュー選択 */}
          <ViewSelector statsView={statsView} onStatsViewChange={onStatsViewChange} />

          {/* プロジェクト・ブランチ選択（プロジェクトビュー時のみ） */}
          {statsView === 'project' && (
            <ProjectBranchSelector
              projects={projects}
              selectedProject={selectedProject}
              onProjectSelect={onProjectSelect}
              branches={branches}
              selectedBranch={selectedBranch}
              onBranchChange={onBranchChange}
            />
          )}

          <StatsFilters year={statsYear} onYearChange={onYearChange} />
        </div>

        <div className="flex items-center gap-2">
          {/* ブランチ削除ボタン（プロジェクトビュー + ブランチ選択済みの場合のみ） */}
          {statsView === 'project' && selectedProject && selectedBranch && (
            <BranchDeleteDialog
              projectId={selectedProject.projectId}
              branchName={selectedBranch}
              impact={branchDeleteImpact}
              isLoadingImpact={isLoadingBranchDeleteImpact}
              onDeleted={onBranchDeleted}
              onOpen={onRequestBranchDeleteImpact}
            />
          )}

          <Button onClick={onRefresh} disabled={isLoadingStats}>
            {isLoadingStats ? '読み込み中...' : '更新'}
          </Button>
        </div>
      </CardContent>
    </Card>
  )
}

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
import { Card, CardContent } from '@/components/ui/card'
import { Button } from '@/components/ui/button'

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

        <Button onClick={onRefresh} disabled={isLoadingStats}>
          {isLoadingStats ? '読み込み中...' : '更新'}
        </Button>
      </CardContent>
    </Card>
  )
}

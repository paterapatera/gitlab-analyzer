/**
 * 選択案内アラートコンポーネント
 *
 * プロジェクト・ブランチ未選択時の案内メッセージを表示する。
 */

import type { Project } from '@/lib/contracts/tauriCommands'
import type { StatsView } from '@/features/stats/useStatsData'
import { Alert, AlertDescription } from '@/components/ui/alert'
import { AlertTriangle } from 'lucide-react'

/** 選択案内アラートのプロパティ */
export interface SelectionAlertsProps {
  statsView: StatsView
  selectedProject: Project | null
  selectedBranch: string
}

/** 選択案内アラートコンポーネント */
export function SelectionAlerts({
  statsView,
  selectedProject,
  selectedBranch,
}: SelectionAlertsProps) {
  if (statsView !== 'project') return null

  if (!selectedProject) {
    return (
      <Alert>
        <AlertTriangle className="h-4 w-4" />
        <AlertDescription>上記からプロジェクトを選択してください</AlertDescription>
      </Alert>
    )
  }

  if (!selectedBranch) {
    return (
      <Alert>
        <AlertTriangle className="h-4 w-4" />
        <AlertDescription>上記からブランチを選択してください</AlertDescription>
      </Alert>
    )
  }

  return null
}

/**
 * 設定タブコンポーネント
 *
 * GitLab接続設定とプロジェクト管理を行う。
 */

import type { Project } from '@/lib/contracts/tauriCommands'
import { ConnectionForm } from '@/features/gitlabConnection/ConnectionForm'
import { ProjectsPanel } from '@/features/projects/ProjectsPanel'

/** 設定タブのプロパティ */
export interface SettingsTabProps {
  /** 接続設定保存時のコールバック */
  onConnectionSaved: () => void
  /** プロジェクト選択時のコールバック */
  onProjectSelect: (project: Project) => void
  /** 選択中のプロジェクトID */
  selectedProjectId?: number
}

/** 設定タブコンポーネント */
export function SettingsTab({
  onConnectionSaved,
  onProjectSelect,
  selectedProjectId,
}: SettingsTabProps) {
  return (
    <div className="grid gap-6 md:grid-cols-2">
      <ConnectionForm onSaved={onConnectionSaved} />
      <ProjectsPanel onProjectSelect={onProjectSelect} selectedProjectId={selectedProjectId} />
    </div>
  )
}

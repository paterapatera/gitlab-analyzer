/**
 * コミット収集タブコンポーネント
 *
 * プロジェクトを選択し、コミットを収集する。
 */

import type { Project, CollectCommitsResult } from '@/lib/contracts/tauriCommands'
import { ProjectsPanel } from '@/features/projects/ProjectsPanel'
import { CollectForm } from '@/features/collect/CollectForm'
import { CollectResult } from '@/features/collect/CollectResult'
import { BulkCollectCard } from '@/features/collect/BulkCollectCard'
import { Card, CardContent } from '@/components/ui/card'

/** コミット収集タブのプロパティ */
export interface CollectTabProps {
  /** プロジェクト選択時のコールバック */
  onProjectSelect: (project: Project) => void
  /** 選択中のプロジェクト */
  selectedProject: Project | null
  /** 収集結果 */
  collectResult: CollectCommitsResult | null
  /** 収集完了時のコールバック */
  onCollected: (result: CollectCommitsResult) => void
}

/** コミット収集タブコンポーネント */
export function CollectTab({
  onProjectSelect,
  selectedProject,
  collectResult,
  onCollected,
}: CollectTabProps) {
  return (
    <div className="grid gap-6 md:grid-cols-2">
      <ProjectsPanel
        onProjectSelect={onProjectSelect}
        selectedProjectId={selectedProject?.projectId}
      />
      <div className="space-y-6">
        <BulkCollectCard />
        {selectedProject ? (
          <CollectContent
            projectId={selectedProject.projectId}
            collectResult={collectResult}
            onCollected={onCollected}
          />
        ) : (
          <ProjectNotSelectedCard />
        )}
      </div>
    </div>
  )
}

/** 収集コンテンツのプロパティ */
interface CollectContentProps {
  projectId: number
  collectResult: CollectCommitsResult | null
  onCollected: (result: CollectCommitsResult) => void
}

/** 収集コンテンツ */
function CollectContent({ projectId, collectResult, onCollected }: CollectContentProps) {
  return (
    <>
      <CollectForm projectId={projectId} onCollected={onCollected} />
      {collectResult && <CollectResult result={collectResult} />}
    </>
  )
}

/** プロジェクト未選択のカード */
function ProjectNotSelectedCard() {
  return (
    <Card>
      <CardContent className="pt-6 text-muted-foreground">
        左のパネルからプロジェクトを選択してください
      </CardContent>
    </Card>
  )
}

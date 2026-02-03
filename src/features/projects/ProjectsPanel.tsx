/**
 * プロジェクト一覧パネル
 *
 * プロジェクトの同期と一覧表示を行う。
 */

import { useState, useCallback, useEffect } from 'react'
import { invokeCommand } from '@/lib/tauri'
import type { Project } from '@/lib/contracts/tauriCommands'
import { ErrorAlert } from '@/features/ui/ErrorAlert'
import { cn } from '@/lib/utils'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle, CardAction } from '@/components/ui/card'

/** プロジェクトパネルのプロパティ */
export interface ProjectsPanelProps {
  /** プロジェクト選択時のコールバック */
  onProjectSelect?: (project: Project) => void
  /** 選択中のプロジェクト ID */
  selectedProjectId?: number
  /** 追加のCSSクラス */
  className?: string
}

/**
 * プロジェクト一覧パネル
 */
export function ProjectsPanel({
  onProjectSelect,
  selectedProjectId,
  className,
}: ProjectsPanelProps) {
  const [projects, setProjects] = useState<Project[]>([])
  const [isLoadingProjects, setIsLoadingProjects] = useState(false)
  const [isSyncing, setIsSyncing] = useState(false)
  const [error, setError] = useState<string | null>(null)

  // ローカルに保存されたプロジェクトを読み込む（初回のみ）
  useEffect(() => {
    const loadProjects = async () => {
      setIsLoadingProjects(true)
      setError(null)

      try {
        const result = await invokeCommand<Project[]>('get_projects')

        if (!result.ok) {
          setError(result.error)
          return
        }

        setProjects(result.data)
      } finally {
        setIsLoadingProjects(false)
      }
    }

    loadProjects()
  }, [])

  const handleSync = useCallback(async () => {
    setError(null)
    setIsSyncing(true)

    try {
      const result = await invokeCommand<Project[]>('sync_projects')

      if (!result.ok) {
        setError(result.error)
        return
      }

      setProjects(result.data)
    } finally {
      setIsSyncing(false)
    }
  }, [])

  return (
    <Card className={className}>
      <CardHeader>
        <CardTitle>プロジェクト一覧</CardTitle>
        <CardAction>
          <Button variant="default" onClick={handleSync} disabled={isSyncing}>
            {isSyncing ? '同期中...' : 'プロジェクト同期'}
          </Button>
        </CardAction>
      </CardHeader>
      <CardContent>
        {error && <ErrorAlert message={error} onDismiss={() => setError(null)} />}

        {projects.length === 0 && !error && !isLoadingProjects && (
          <p className="text-muted-foreground">
            プロジェクトがありません。「プロジェクト同期」をクリックして取得してください。
          </p>
        )}

        {projects.length > 0 && (
          <ul className="divide-y divide-border rounded-md border">
            {projects.map((project) => (
              <li key={project.projectId}>
                <button
                  type="button"
                  onClick={() => onProjectSelect?.(project)}
                  className={cn(
                    'w-full px-4 py-3 text-left transition-colors',
                    'hover:bg-accent focus:bg-accent focus:outline-none',
                    selectedProjectId === project.projectId && 'bg-accent',
                  )}
                >
                  <div className="font-medium">{project.name}</div>
                  <div className="text-sm">{project.pathWithNamespace}</div>
                </button>
              </li>
            ))}
          </ul>
        )}
      </CardContent>
    </Card>
  )
}

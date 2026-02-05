/**
 * GitLab 月次コミット行数分析アプリ
 *
 * メインのアプリケーションコンポーネント。
 * タブで画面を切り替え、接続設定/プロジェクト/収集/集計の各機能を提供する。
 */

import { useState, useCallback, useEffect } from 'react'
import { invokeCommand } from '@/lib/tauri'
import type { Project, CollectCommitsResult } from '@/lib/contracts/tauriCommands'
import { SettingsTab } from '@/features/settings/SettingsTab'
import { CollectTab } from '@/features/collect/CollectTab'
import { StatsTab } from '@/features/stats/StatsTab'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import './App.css'

/** タブの種類 */
type Tab = 'settings' | 'collect' | 'stats'

function App() {
  const [activeTab, setActiveTab] = useState<Tab>('settings')
  const [selectedProject, setSelectedProject] = useState<Project | null>(null)
  const [collectResult, setCollectResult] = useState<CollectCommitsResult | null>(null)

  // プロジェクト一覧（各タブでの選択用）
  const [projects, setProjects] = useState<Project[]>([])

  // 初回起動時に保存済みプロジェクト一覧を読み込む
  useEffect(() => {
    const loadProjects = async () => {
      const result = await invokeCommand<Project[]>('get_projects')
      if (result.ok) {
        setProjects(result.data)
      }
    }
    loadProjects()
  }, [])

  const handleProjectSelect = useCallback((project: Project) => {
    setSelectedProject(project)
    setActiveTab('collect')
  }, [])

  const handleCollected = useCallback((result: CollectCommitsResult) => {
    setCollectResult(result)
  }, [])

  const handleConnectionSaved = useCallback(() => {
    // 接続設定保存後、自動的にプロジェクト同期を促す
  }, [])

  return (
    <div className="min-h-screen bg-background">
      {/* ヘッダー */}
      <header className="bg-card shadow-sm border-b">
        <div className="mx-auto max-w-6xl px-4 py-4">
          <h1 className="text-xl font-bold">GitLab 月次コミット行数分析</h1>
        </div>
      </header>

      {/* メインコンテンツ */}
      <main className="mx-auto max-w-6xl px-4 py-6">
        <Tabs value={activeTab} onValueChange={(value) => setActiveTab(value as Tab)}>
          <TabsList className="mb-6 gap-2">
            <TabsTrigger value="settings">設定</TabsTrigger>
            <TabsTrigger value="collect">コミット収集</TabsTrigger>
            <TabsTrigger value="stats">集計表示</TabsTrigger>
          </TabsList>

          {/* 設定タブ */}
          <TabsContent value="settings">
            <SettingsTab
              onConnectionSaved={handleConnectionSaved}
              onProjectSelect={handleProjectSelect}
              selectedProjectId={selectedProject?.projectId}
            />
          </TabsContent>

          {/* コミット収集タブ */}
          <TabsContent value="collect">
            <CollectTab
              onProjectSelect={handleProjectSelect}
              selectedProject={selectedProject}
              collectResult={collectResult}
              onCollected={handleCollected}
            />
          </TabsContent>

          {/* 集計表示タブ */}
          <TabsContent value="stats">
            <StatsTab projects={projects} />
          </TabsContent>
        </Tabs>
      </main>
    </div>
  )
}

export default App

/**
 * GitLab 月次コミット行数分析アプリ
 *
 * メインのアプリケーションコンポーネント。
 * タブで画面を切り替え、接続設定/プロジェクト/収集/集計の各機能を提供する。
 */

import { useState, useCallback, useEffect, useMemo } from 'react'
import { invokeCommand } from '@/lib/tauri'
import type {
  Project,
  Branch,
  CollectCommitsResult,
  MonthlyStatsResponse,
  ProjectViewStatsRequest,
  CrossViewStatsRequest,
  UserFilterViewType,
} from '@/lib/contracts/tauriCommands'
import { ConnectionForm } from '@/features/gitlabConnection/ConnectionForm'
import { ProjectsPanel } from '@/features/projects/ProjectsPanel'
import { CollectForm } from '@/features/collect/CollectForm'
import { CollectResult } from '@/features/collect/CollectResult'
import { StatsFilters } from '@/features/stats/StatsFilters'
import { MonthlyBarChart } from '@/features/stats/MonthlyBarChart'
import { MonthlyTable } from '@/features/stats/MonthlyTable'
import { MissingStatsNotice } from '@/features/stats/MissingStatsNotice'
import { UserFilter } from '@/features/stats/UserFilter'
import { useUserFilter } from '@/features/stats/useUserFilter'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Label } from '@/components/ui/label'
import { NativeSelect, NativeSelectOption } from '@/components/ui/native-select'
import { Alert, AlertDescription } from '@/components/ui/alert'
import { AlertTriangle } from 'lucide-react'
import './App.css'

/** タブの種類 */
type Tab = 'settings' | 'collect' | 'stats'

/** ビューの種類 */
type StatsView = 'project' | 'cross'

function App() {
  const [activeTab, setActiveTab] = useState<Tab>('settings')
  const [selectedProject, setSelectedProject] = useState<Project | null>(null)
  const [collectResult, setCollectResult] = useState<CollectCommitsResult | null>(null)

  // プロジェクト一覧（集計タブでの選択用）
  const [projects, setProjects] = useState<Project[]>([])
  // ブランチ一覧（集計タブでの選択用）
  const [branches, setBranches] = useState<Branch[]>([])
  const [selectedBranch, setSelectedBranch] = useState<string>('')

  // 集計関連の状態
  const [statsView, setStatsView] = useState<StatsView>('project')
  const [statsYear, setStatsYear] = useState(new Date().getFullYear())
  const [statsData, setStatsData] = useState<MonthlyStatsResponse | null>(null)
  const [isLoadingStats, setIsLoadingStats] = useState(false)

  // ユーザーフィルタ用のcontextKey生成
  const userFilterViewType: UserFilterViewType =
    statsView === 'project' ? 'project-view' : 'cross-view'
  const userFilterContextKey = useMemo(() => {
    if (statsView === 'project' && selectedProject && selectedBranch) {
      return `${selectedProject.name}/${selectedBranch}/${statsYear}`
    }
    return `${statsYear}`
  }, [statsView, selectedProject, selectedBranch, statsYear])

  // 利用可能なユーザー（集計データから抽出）
  const availableUsers = useMemo(() => {
    if (!statsData) return []
    return statsData.series.map((s) => ({
      userKey: s.userKey,
      displayName: s.displayName,
    }))
  }, [statsData])

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
  const filteredStatsData = useMemo<MonthlyStatsResponse | null>(() => {
    if (!statsData) return null
    return {
      months: statsData.months,
      series: statsData.series.filter((s) => selectedUsers.includes(s.userKey)),
    }
  }, [statsData, selectedUsers])

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

  // プロジェクト選択時にブランチ一覧を読み込む
  useEffect(() => {
    if (!selectedProject) {
      setBranches([])
      setSelectedBranch('')
      return
    }

    const loadBranches = async () => {
      const result = await invokeCommand<Branch[]>('list_branches', {
        projectId: selectedProject.projectId,
      })
      if (result.ok) {
        setBranches(result.data)
        // デフォルトブランチを自動選択
        const defaultBranch = result.data.find((b) => b.isDefault)
        if (defaultBranch) {
          setSelectedBranch(defaultBranch.name)
        } else if (result.data.length > 0) {
          setSelectedBranch(result.data[0].name)
        }
      }
    }
    loadBranches()
  }, [selectedProject])

  // 集計データを取得
  const loadStats = useCallback(async () => {
    if (statsView === 'project' && (!selectedProject || !selectedBranch)) {
      return
    }

    setIsLoadingStats(true)

    try {
      if (statsView === 'project' && selectedProject && selectedBranch) {
        const request: ProjectViewStatsRequest = {
          projectId: selectedProject.projectId,
          branchName: selectedBranch,
          year: statsYear,
        }
        const result = await invokeCommand<MonthlyStatsResponse>('get_monthly_stats_project_view', {
          request,
        })
        if (result.ok) {
          setStatsData(result.data)
        }
      } else {
        const request: CrossViewStatsRequest = {
          year: statsYear,
        }
        const result = await invokeCommand<MonthlyStatsResponse>('get_monthly_stats_cross_view', {
          request,
        })
        if (result.ok) {
          setStatsData(result.data)
        }
      }
    } finally {
      setIsLoadingStats(false)
    }
  }, [statsView, selectedProject, selectedBranch, statsYear])

  // 集計タブに切り替えたときにデータを読み込む
  useEffect(() => {
    if (activeTab === 'stats') {
      loadStats()
    }
  }, [activeTab, loadStats])

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
            <div className="grid gap-6 md:grid-cols-2">
              <ConnectionForm onSaved={handleConnectionSaved} />
              <ProjectsPanel
                onProjectSelect={handleProjectSelect}
                selectedProjectId={selectedProject?.projectId}
              />
            </div>
          </TabsContent>

          {/* コミット収集タブ */}
          <TabsContent value="collect">
            <div className="grid gap-6 md:grid-cols-2">
              <ProjectsPanel
                onProjectSelect={handleProjectSelect}
                selectedProjectId={selectedProject?.projectId}
              />
              <div className="space-y-6">
                {selectedProject ? (
                  <>
                    <CollectForm
                      projectId={selectedProject.projectId}
                      onCollected={handleCollected}
                    />
                    {collectResult && <CollectResult result={collectResult} />}
                  </>
                ) : (
                  <Card>
                    <CardContent className="pt-6 text-muted-foreground">
                      左のパネルからプロジェクトを選択してください
                    </CardContent>
                  </Card>
                )}
              </div>
            </div>
          </TabsContent>

          {/* 集計表示タブ */}
          <TabsContent value="stats">
            <div className="space-y-6">
              {/* フィルタ */}
              <Card>
                <CardContent className="flex flex-wrap items-center justify-between gap-4 pt-6">
                  <div className="flex flex-wrap items-center gap-6">
                    <div className="flex items-center gap-2">
                      <Label>ビュー:</Label>
                      <NativeSelect
                        value={statsView}
                        onChange={(e) => setStatsView(e.target.value as StatsView)}
                        size="sm"
                      >
                        <NativeSelectOption value="project">プロジェクト別</NativeSelectOption>
                        <NativeSelectOption value="cross">横断</NativeSelectOption>
                      </NativeSelect>
                    </div>
                    {statsView === 'project' && (
                      <>
                        <div className="flex items-center gap-2">
                          <Label>プロジェクト:</Label>
                          <NativeSelect
                            value={selectedProject?.projectId.toString() || ''}
                            onChange={(e) => {
                              const project = projects.find(
                                (p) => p.projectId.toString() === e.target.value,
                              )
                              setSelectedProject(project || null)
                            }}
                            size="sm"
                            className="min-w-[200px]"
                          >
                            <NativeSelectOption value="">選択してください</NativeSelectOption>
                            {projects.map((project) => (
                              <NativeSelectOption
                                key={project.projectId}
                                value={project.projectId.toString()}
                              >
                                {project.name}
                              </NativeSelectOption>
                            ))}
                          </NativeSelect>
                        </div>
                        {selectedProject && (
                          <div className="flex items-center gap-2">
                            <Label>ブランチ:</Label>
                            <NativeSelect
                              value={selectedBranch}
                              onChange={(e) => setSelectedBranch(e.target.value)}
                              size="sm"
                              className="min-w-[150px]"
                            >
                              {branches.map((branch) => (
                                <NativeSelectOption key={branch.name} value={branch.name}>
                                  {branch.name}
                                </NativeSelectOption>
                              ))}
                            </NativeSelect>
                          </div>
                        )}
                      </>
                    )}
                    <StatsFilters year={statsYear} onYearChange={setStatsYear} />
                  </div>
                  <Button onClick={loadStats} disabled={isLoadingStats}>
                    {isLoadingStats ? '読み込み中...' : '更新'}
                  </Button>
                </CardContent>
              </Card>

              {/* プロジェクト・ブランチ選択（プロジェクトビュー時のみ） */}
              {statsView === 'project' && !selectedProject && (
                <Alert>
                  <AlertTriangle className="h-4 w-4" />
                  <AlertDescription>上記からプロジェクトを選択してください</AlertDescription>
                </Alert>
              )}
              {statsView === 'project' && selectedProject && !selectedBranch && (
                <Alert>
                  <AlertTriangle className="h-4 w-4" />
                  <AlertDescription>上記からブランチを選択してください</AlertDescription>
                </Alert>
              )}

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
          </TabsContent>
        </Tabs>
      </main>
    </div>
  )
}

export default App

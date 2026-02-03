/**
 * コミット収集フォーム
 *
 * プロジェクト/ブランチ/期間を指定してコミットを収集する。
 */

import { useState, useCallback, useEffect } from 'react'
import { invokeCommand } from '@/lib/tauri'
import type {
  Branch,
  CollectCommitsRequest,
  CollectCommitsResult,
} from '@/lib/contracts/tauriCommands'
import { ErrorAlert } from '@/features/ui/ErrorAlert'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { NativeSelect, NativeSelectOption } from '@/components/ui/native-select'
import { Skeleton } from '@/components/ui/skeleton'

/** 収集フォームのプロパティ */
export interface CollectFormProps {
  /** 対象プロジェクト ID */
  projectId: number
  /** 収集完了時のコールバック */
  onCollected?: (result: CollectCommitsResult) => void
  /** 追加のCSSクラス */
  className?: string
}

/**
 * コミット収集フォーム
 */
export function CollectForm({ projectId, onCollected, className }: CollectFormProps) {
  const [branches, setBranches] = useState<Branch[]>([])
  const [selectedBranch, setSelectedBranch] = useState('')
  const [sinceDate, setSinceDate] = useState('')
  const [untilDate, setUntilDate] = useState('')
  const [isLoadingBranches, setIsLoadingBranches] = useState(false)
  const [isCollecting, setIsCollecting] = useState(false)
  const [error, setError] = useState<string | null>(null)

  // ブランチ一覧を読み込む
  useEffect(() => {
    const loadBranches = async () => {
      setIsLoadingBranches(true)
      setError(null)

      try {
        const result = await invokeCommand<Branch[]>('list_branches', {
          projectId,
        })

        if (!result.ok) {
          setError(result.error)
          return
        }

        setBranches(result.data)

        // デフォルトブランチを選択
        const defaultBranch = result.data.find((b) => b.isDefault)
        if (defaultBranch) {
          setSelectedBranch(defaultBranch.name)
        } else if (result.data.length > 0) {
          setSelectedBranch(result.data[0].name)
        }
      } finally {
        setIsLoadingBranches(false)
      }
    }

    loadBranches()
  }, [projectId])

  const handleCollect = useCallback(async () => {
    if (!selectedBranch) {
      setError('ブランチを選択してください')
      return
    }

    // 日付バリデーション
    if (sinceDate && untilDate && sinceDate > untilDate) {
      setError('開始日は終了日より前にしてください')
      return
    }

    setError(null)
    setIsCollecting(true)

    try {
      const request: CollectCommitsRequest = {
        projectId,
        branchName: selectedBranch,
        sinceUtc: sinceDate ? `${sinceDate}T00:00:00Z` : null,
        untilUtc: untilDate ? `${untilDate}T23:59:59Z` : null,
      }

      const result = await invokeCommand<CollectCommitsResult>('collect_commits', {
        request,
      })

      if (!result.ok) {
        setError(result.error)
        return
      }

      onCollected?.(result.data)
    } finally {
      setIsCollecting(false)
    }
  }, [projectId, selectedBranch, sinceDate, untilDate, onCollected])

  if (isLoadingBranches) {
    return (
      <Card className={className}>
        <CardHeader>
          <Skeleton className="h-6 w-32" />
        </CardHeader>
        <CardContent className="space-y-4">
          <Skeleton className="h-9 w-full" />
          <div className="grid grid-cols-2 gap-4">
            <Skeleton className="h-9 w-full" />
            <Skeleton className="h-9 w-full" />
          </div>
          <Skeleton className="h-9 w-full" />
        </CardContent>
      </Card>
    )
  }

  return (
    <Card className={className}>
      <CardHeader>
        <CardTitle>コミット収集</CardTitle>
      </CardHeader>
      <CardContent className="space-y-4">
        {error && <ErrorAlert message={error} onDismiss={() => setError(null)} />}

        <div className="space-y-2">
          <Label htmlFor="branch">ブランチ</Label>
          <NativeSelect
            id="branch"
            value={selectedBranch}
            onChange={(e) => setSelectedBranch(e.target.value)}
            disabled={isCollecting}
            className="w-full"
          >
            {branches.map((branch) => (
              <NativeSelectOption key={branch.name} value={branch.name}>
                {branch.name}
                {branch.isDefault ? ' (default)' : ''}
              </NativeSelectOption>
            ))}
          </NativeSelect>
        </div>

        <div className="grid grid-cols-2 gap-4">
          <div className="space-y-2">
            <Label htmlFor="sinceDate">開始日（任意）</Label>
            <Input
              id="sinceDate"
              type="date"
              value={sinceDate}
              onChange={(e) => setSinceDate(e.target.value)}
              disabled={isCollecting}
            />
          </div>

          <div className="space-y-2">
            <Label htmlFor="untilDate">終了日（任意）</Label>
            <Input
              id="untilDate"
              type="date"
              value={untilDate}
              onChange={(e) => setUntilDate(e.target.value)}
              disabled={isCollecting}
            />
          </div>
        </div>

        <Button
          type="button"
          onClick={handleCollect}
          disabled={!selectedBranch || isCollecting}
          variant="secondary"
          className="w-full"
        >
          {isCollecting ? '収集中...' : 'コミット収集'}
        </Button>
      </CardContent>
    </Card>
  )
}

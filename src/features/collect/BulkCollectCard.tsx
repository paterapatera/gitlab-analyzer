/**
 * 一括コミット収集カード
 *
 * 収集履歴がある全対象の続きをまとめて収集する。
 */

import { useCallback, useEffect, useMemo, useState } from 'react'
import { listen } from '@tauri-apps/api/event'
import { invokeCommand } from '@/lib/tauri'
import type {
  BulkCollectionProgress,
  BulkCollectionStarted,
  BulkCollectionStatus,
} from '@/lib/contracts/tauriCommands'
import { ErrorAlert } from '@/features/ui/ErrorAlert'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Progress } from '@/components/ui/progress'
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table'

export function BulkCollectCard() {
  const [runId, setRunId] = useState<string | null>(null)
  const [status, setStatus] = useState<BulkCollectionStatus | null>(null)
  const [progress, setProgress] = useState<BulkCollectionProgress | null>(null)
  const [isRunning, setIsRunning] = useState(false)
  const [isLoadingStatus, setIsLoadingStatus] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const refreshStatus = useCallback(
    async (targetRunId?: string) => {
      const id = targetRunId ?? runId
      if (!id) return

      setIsLoadingStatus(true)
      const result = await invokeCommand<BulkCollectionStatus>('get_bulk_collection_status', {
        runId: id,
      })
      setIsLoadingStatus(false)

      if (!result.ok) {
        setError(result.error)
        return
      }

      setStatus(result.data)
      setProgress({
        runId: result.data.runId,
        totalTargets: result.data.totalTargets,
        completedCount: result.data.completedCount,
        successCount: result.data.successCount,
        failedCount: result.data.failedCount,
        currentTarget: null,
      })
      setIsRunning(result.data.status === 'running')
    },
    [runId],
  )

  useEffect(() => {
    const unlisten = listen<BulkCollectionProgress>('bulk-collection-progress', (event) => {
      setProgress(event.payload)
      setIsRunning(event.payload.completedCount < event.payload.totalTargets)

      if (event.payload.completedCount >= event.payload.totalTargets) {
        refreshStatus(event.payload.runId)
      }
    })

    return () => {
      unlisten.then((fn) => fn())
    }
  }, [refreshStatus])

  const handleStart = useCallback(async () => {
    setError(null)
    const result = await invokeCommand<BulkCollectionStarted>('collect_commits_bulk')
    if (!result.ok) {
      setError(result.error)
      return
    }

    setRunId(result.data.runId)
    setIsRunning(true)
    setProgress({
      runId: result.data.runId,
      totalTargets: result.data.totalTargets,
      completedCount: 0,
      successCount: 0,
      failedCount: 0,
      currentTarget: null,
    })
    refreshStatus(result.data.runId)
  }, [refreshStatus])

  const handleCancel = useCallback(async () => {
    const result = await invokeCommand<void>('cancel_bulk_collection')
    if (!result.ok) {
      setError(result.error)
      return
    }

    setIsRunning(false)
    refreshStatus()
  }, [refreshStatus])

  const handleRetryFailed = useCallback(async () => {
    if (!runId) return

    setError(null)
    const result = await invokeCommand<BulkCollectionStarted>('retry_failed_targets', {
      request: { runId },
    })
    if (!result.ok) {
      setError(result.error)
      return
    }

    setRunId(result.data.runId)
    setIsRunning(true)
    setProgress({
      runId: result.data.runId,
      totalTargets: result.data.totalTargets,
      completedCount: 0,
      successCount: 0,
      failedCount: 0,
      currentTarget: null,
    })
    refreshStatus(result.data.runId)
  }, [runId, refreshStatus])

  const totalTargets = progress?.totalTargets ?? status?.totalTargets ?? 0
  const completedCount = progress?.completedCount ?? status?.completedCount ?? 0
  const successCount = progress?.successCount ?? status?.successCount ?? 0
  const failedCount = progress?.failedCount ?? status?.failedCount ?? 0

  const progressPercent = useMemo(() => {
    if (totalTargets === 0) return 0
    return Math.min(100, Math.round((completedCount / totalTargets) * 100))
  }, [completedCount, totalTargets])

  const statusLabel = useMemo(() => {
    if (!status) return null
    switch (status.status) {
      case 'running':
        return '実行中'
      case 'completed':
        return '完了'
      case 'cancelled':
        return 'キャンセル'
      default:
        return null
    }
  }, [status])

  const hasFailedTargets = (status?.failedCount ?? 0) > 0

  return (
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <span>一括収集</span>
          {statusLabel && <Badge variant="outline">{statusLabel}</Badge>}
        </CardTitle>
      </CardHeader>
      <CardContent className="space-y-4">
        {error && <ErrorAlert message={error} onDismiss={() => setError(null)} />}

        <div className="flex flex-wrap gap-2">
          <Button onClick={handleStart} disabled={isRunning}>
            すべての続きを収集
          </Button>
          <Button onClick={handleCancel} variant="outline" disabled={!isRunning}>
            キャンセル
          </Button>
          <Button
            onClick={handleRetryFailed}
            variant="secondary"
            disabled={isRunning || !hasFailedTargets}
          >
            失敗分を再試行
          </Button>
          <Button
            onClick={() => refreshStatus()}
            variant="ghost"
            disabled={!runId || isLoadingStatus}
          >
            状況を更新
          </Button>
        </div>

        <div className="space-y-2">
          <Progress value={progressPercent} />
          <div className="text-sm text-muted-foreground">
            {completedCount} / {totalTargets} 完了（成功: {successCount}, 失敗: {failedCount}）
          </div>
          {progress?.currentTarget && (
            <div className="text-xs text-muted-foreground">
              処理中: {progress.currentTarget.projectId} / {progress.currentTarget.branchName}
            </div>
          )}
        </div>

        {status?.results && status.results.length > 0 && (
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>プロジェクト</TableHead>
                <TableHead>ブランチ</TableHead>
                <TableHead>状態</TableHead>
                <TableHead>新規コミット</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {status.results.map((result) => (
                <TableRow key={`${result.projectId}-${result.branchName}`}>
                  <TableCell>{result.projectId}</TableCell>
                  <TableCell>{result.branchName}</TableCell>
                  <TableCell>
                    <Badge variant={result.status === 'failed' ? 'destructive' : 'outline'}>
                      {result.status === 'success'
                        ? '成功'
                        : result.status === 'failed'
                          ? '失敗'
                          : '保留'}
                    </Badge>
                  </TableCell>
                  <TableCell>{result.newCommitsCount ?? '-'}</TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        )}
      </CardContent>
    </Card>
  )
}

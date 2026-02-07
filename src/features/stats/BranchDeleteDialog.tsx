/**
 * ブランチ削除ダイアログコンポーネント
 *
 * ゴミ箱アイコンのトリガーボタンと、取り消し不可の確認ダイアログを提供する。
 * US2（影響サマリ）で拡張予定。
 */

import { useState, useCallback } from 'react'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog'
import { Button } from '@/components/ui/button'
import { Trash2 } from 'lucide-react'
import { useBranchDelete } from '@/features/stats/useBranchDelete'
import type { DeleteBranchImpactResponse } from '@/lib/contracts/tauriCommands'

/** ダイアログのプロパティ */
export interface BranchDeleteDialogProps {
  /** 対象プロジェクト ID */
  projectId: number
  /** 対象ブランチ名 */
  branchName: string
  /** 影響サマリ（US2 で使用、省略時は基本ダイアログ） */
  impact?: DeleteBranchImpactResponse | null
  /** 影響サマリ取得中フラグ */
  isLoadingImpact?: boolean
  /** 削除完了後のコールバック */
  onDeleted?: () => void
  /** ダイアログを開くときのコールバック（影響サマリ取得に使用） */
  onOpen?: () => void
}

/**
 * ブランチ削除ダイアログ
 */
export function BranchDeleteDialog({
  projectId,
  branchName,
  impact,
  isLoadingImpact,
  onDeleted,
  onOpen,
}: BranchDeleteDialogProps) {
  const [open, setOpen] = useState(false)
  const { execute, isDeleting, error, clearError } = useBranchDelete()

  /** ダイアログを開く */
  const handleOpenChange = useCallback(
    (nextOpen: boolean) => {
      setOpen(nextOpen)
      if (nextOpen) {
        clearError()
        onOpen?.()
      }
    },
    [clearError, onOpen],
  )

  /** 削除実行 */
  const handleConfirm = useCallback(async () => {
    try {
      await execute({ projectId, branchName })
      setOpen(false)
      onDeleted?.()
    } catch {
      // エラーは useBranchDelete 内で管理
    }
  }, [execute, projectId, branchName, onDeleted])

  // ブロック状態の判定（US2 で使用）
  const isBlocked = impact?.status === 'blocked'
  const hasNoCommits = impact?.status === 'no_commits'
  const canDelete = !isBlocked && !hasNoCommits && !isDeleting && !isLoadingImpact

  return (
    <Dialog open={open} onOpenChange={handleOpenChange}>
      <DialogTrigger asChild>
        <Button variant="ghost" size="icon" title="ブランチのコミットを削除">
          <Trash2 className="h-4 w-4 text-destructive" />
        </Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>ブランチのコミットを削除</DialogTitle>
          <DialogDescription>{branchName} の収集済みコミットをすべて削除します。</DialogDescription>
        </DialogHeader>

        <div className="space-y-4 py-4">
          {/* 影響サマリ読み込み中 */}
          {isLoadingImpact && <p className="text-sm text-muted-foreground">影響を確認中...</p>}

          {/* ブロック状態：収集中（US2） */}
          {isBlocked && impact?.blockReason && (
            <div className="rounded-md border border-destructive/50 bg-destructive/10 p-3">
              <p className="text-sm font-medium text-destructive">{impact.blockReason}</p>
            </div>
          )}

          {/* コミットなし（US2） */}
          {hasNoCommits && (
            <div className="rounded-md border p-3">
              <p className="text-sm text-muted-foreground">
                このブランチには削除対象のコミットがありません。
              </p>
            </div>
          )}

          {/* 影響サマリ表示（US2） */}
          {impact && impact.status === 'ok' && (
            <div className="rounded-md border p-3 space-y-2">
              <p className="text-sm">
                <span className="font-medium">削除対象:</span> {impact.commitCount} 件のコミット
              </p>
              {impact.affectedViews.length > 0 && (
                <p className="text-sm">
                  <span className="font-medium">影響ビュー:</span> {impact.affectedViews.join(', ')}
                </p>
              )}
            </div>
          )}

          {/* 取り消し不可の警告 */}
          {!isBlocked && !hasNoCommits && !isLoadingImpact && (
            <div className="rounded-md border border-orange-500/50 bg-orange-500/10 p-3">
              <p className="text-sm font-medium text-orange-700 dark:text-orange-400">
                ⚠ この操作は取り消しできません。削除されたコミットは復元できません。
              </p>
            </div>
          )}

          {/* エラー表示 */}
          {error && (
            <div className="rounded-md border border-destructive/50 bg-destructive/10 p-3">
              <p className="text-sm text-destructive">{error}</p>
            </div>
          )}
        </div>

        <DialogFooter>
          <Button variant="outline" onClick={() => setOpen(false)} disabled={isDeleting}>
            キャンセル
          </Button>
          <Button variant="destructive" onClick={handleConfirm} disabled={!canDelete}>
            {isDeleting ? '削除中...' : '削除する'}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}

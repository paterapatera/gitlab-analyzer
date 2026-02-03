/**
 * stats 欠損通知
 *
 * 欠損 stats の件数をユーザーに通知する。
 */

import type { MonthlyStatsResponse } from '@/lib/contracts/tauriCommands'
import { Alert, AlertTitle, AlertDescription } from '@/components/ui/alert'
import { AlertTriangle } from 'lucide-react'

/** 通知のプロパティ */
export interface MissingStatsNoticeProps {
  /** 集計データ */
  data: MonthlyStatsResponse
  /** 追加のCSSクラス */
  className?: string
}

/**
 * stats 欠損通知
 */
export function MissingStatsNotice({ data, className }: MissingStatsNoticeProps) {
  // 全体の欠損件数を計算
  const totalMissing = data.series.reduce(
    (sum, series) => sum + series.missingCounts.reduce((s, c) => s + c, 0),
    0,
  )

  if (totalMissing === 0) {
    return null
  }

  return (
    <Alert className={className}>
      <AlertTriangle className="h-4 w-4" />
      <AlertTitle>{totalMissing} 件のコミットで行数情報（stats）が取得できませんでした</AlertTitle>
      <AlertDescription>
        これらのコミットは 0 行として集計されています。GitLab API の制限により、 一部のコミットでは
        stats が取得できない場合があります。
      </AlertDescription>
    </Alert>
  )
}

/**
 * コミット収集結果表示
 *
 * 収集結果（挿入件数/スキップ件数/欠損件数）を表示する。
 */

import type { CollectCommitsResult } from '@/lib/contracts/tauriCommands'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'

/** 収集結果のプロパティ */
export interface CollectResultProps {
  /** 収集結果 */
  result: CollectCommitsResult
  /** 追加のCSSクラス */
  className?: string
}

/**
 * コミット収集結果表示
 */
export function CollectResult({ result, className }: CollectResultProps) {
  const total = result.insertedCount + result.skippedDuplicateCount

  return (
    <Card className={className}>
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <span>収集完了</span>
          <Badge variant="default">{total} 件処理</Badge>
        </CardTitle>
      </CardHeader>
      <CardContent>
        <dl className="grid grid-cols-2 gap-2 text-sm">
          <dt className="text-muted-foreground">新規挿入:</dt>
          <dd className="font-medium text-primary">{result.insertedCount} 件</dd>

          <dt className="text-muted-foreground">重複スキップ:</dt>
          <dd className="font-medium text-muted-foreground">{result.skippedDuplicateCount} 件</dd>

          <dt className="text-muted-foreground">stats 欠損:</dt>
          <dd className="font-medium">
            {result.missingStatsCount > 0 ? (
              <Badge variant="outline" className="text-amber-600 border-amber-300">
                {result.missingStatsCount} 件 (0として集計)
              </Badge>
            ) : (
              <span>{result.missingStatsCount} 件</span>
            )}
          </dd>
        </dl>
      </CardContent>
    </Card>
  )
}

/**
 * 月次データ表
 *
 * 月×ユーザーの行数データを表形式で表示する。
 */

import type { MonthlyStatsResponse } from '@/lib/contracts/tauriCommands'
import { cn } from '@/lib/utils'
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table'

/** 表のプロパティ */
export interface MonthlyTableProps {
  /** 集計データ */
  data: MonthlyStatsResponse
  /** 追加のCSSクラス */
  className?: string
}

/**
 * 月次データ表
 */
export function MonthlyTable({ data, className }: MonthlyTableProps) {
  if (data.series.length === 0) {
    return (
      <div className={cn('py-8 text-center text-muted-foreground', className)}>
        表示するデータがありません
      </div>
    )
  }

  return (
    <div className={className}>
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead>ユーザー</TableHead>
            {data.months.map((month) => (
              <TableHead key={month} className="text-right">
                {month}月
              </TableHead>
            ))}
            <TableHead className="text-right">合計</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {data.series.map((series) => {
            const total = series.totals.reduce((sum, v) => sum + v, 0)
            return (
              <TableRow key={series.userKey}>
                <TableCell className="font-medium">
                  {/* NOTE: displayName を使用（email を表示しない） */}
                  {series.displayName}
                </TableCell>
                {series.totals.map((value, index) => (
                  <TableCell key={index} className="text-right">
                    {value.toLocaleString()}
                  </TableCell>
                ))}
                <TableCell className="text-right font-semibold">{total.toLocaleString()}</TableCell>
              </TableRow>
            )
          })}
        </TableBody>
      </Table>
    </div>
  )
}

/**
 * 月次データ表
 *
 * 月×ユーザーの行数データを表形式で表示する。
 */

import type { MonthlyStatsResponse } from '@/lib/contracts/tauriCommands'
import { cn } from '@/lib/utils'
import { buildMonthlyTableHeaders } from '@/features/stats/buildMonthlyTableTsv'
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

  const headers = buildMonthlyTableHeaders(data)

  return (
    <div className={className}>
      <Table>
        <TableHeader>
          <TableRow>
            {headers.map((label, index) => (
              <TableHead
                key={`${label}-${index}`}
                className={index === 0 ? undefined : 'text-right'}
              >
                {label}
              </TableHead>
            ))}
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

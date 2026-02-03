/**
 * 月次縦棒グラフ
 *
 * Recharts を使用して月×ユーザーの集合縦棒グラフを表示する。
 */

import {
  BarChart,
  Bar,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
  ResponsiveContainer,
} from 'recharts'
import type { MonthlyStatsResponse } from '@/lib/contracts/tauriCommands'
import { cn } from '@/lib/utils'

/** グラフのプロパティ */
export interface MonthlyBarChartProps {
  /** 集計データ */
  data: MonthlyStatsResponse
  /** 追加のCSSクラス */
  className?: string
}

/** ユーザーごとの色 */
const COLORS = [
  '#3b82f6', // blue
  '#10b981', // green
  '#f59e0b', // amber
  '#ef4444', // red
  '#8b5cf6', // violet
  '#ec4899', // pink
  '#06b6d4', // cyan
  '#f97316', // orange
]

/**
 * 月次縦棒グラフ
 */
export function MonthlyBarChart({ data, className }: MonthlyBarChartProps) {
  // グラフ用のデータ形式に変換
  const chartData = data.months.map((month, index) => {
    const row: Record<string, number | string> = { month: `${month}月` }
    data.series.forEach((series) => {
      // NOTE: displayName を使用（email を表示しない）
      row[series.displayName] = series.totals[index]
    })
    return row
  })

  if (data.series.length === 0) {
    return (
      <div className={cn('flex h-64 items-center justify-center text-gray-500', className)}>
        表示するデータがありません
      </div>
    )
  }

  return (
    <div className={cn('h-80', className)}>
      <ResponsiveContainer width="100%" height="100%">
        <BarChart data={chartData}>
          <CartesianGrid strokeDasharray="3 3" />
          <XAxis dataKey="month" />
          <YAxis />
          <Tooltip />
          <Legend />
          {data.series.map((series, index) => (
            <Bar
              key={series.userKey}
              dataKey={series.displayName}
              fill={COLORS[index % COLORS.length]}
            />
          ))}
        </BarChart>
      </ResponsiveContainer>
    </div>
  )
}

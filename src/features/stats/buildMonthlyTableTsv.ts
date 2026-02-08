/**
 * 月次詳細テーブルのTSVを生成する。
 *
 * NOTE: authorEmail は使用せず、表示名のみを出力する。
 */

import type { MonthlyStatsResponse } from '@/lib/contracts/tauriCommands'

/**
 * 月次詳細テーブル用のヘッダー配列を構築する。
 */
export function buildMonthlyTableHeaders(data: MonthlyStatsResponse): string[] {
  return ['ユーザー', ...data.months.map((month) => `${month}月`), '合計']
}

/**
 * 月次詳細テーブル用TSVを構築する。
 *
 * @param data - 月次集計データ
 * @returns TSV文字列
 */
export function buildMonthlyTableTsv(data: MonthlyStatsResponse): string {
  const headers = buildMonthlyTableHeaders(data)
  const rows = data.series.map((series) => {
    const total = series.totals.reduce((sum, value) => sum + value, 0)
    return [
      series.displayName,
      ...series.totals.map((value) => value.toLocaleString()),
      total.toLocaleString(),
    ]
  })

  return [headers, ...rows].map((row) => row.join('\t')).join('\n')
}

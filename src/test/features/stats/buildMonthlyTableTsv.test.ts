import { describe, it, expect } from 'vitest'
import { buildMonthlyTableTsv } from '@/features/stats/buildMonthlyTableTsv'
import type { MonthlyStatsResponse } from '@/lib/contracts/tauriCommands'

const sampleData = (): MonthlyStatsResponse =>
  ({
    months: [1, 2, 3],
    series: [
      {
        userKey: 'alice@example.com',
        displayName: 'Alice A',
        totals: [1200, 0, 350],
        missingCounts: [0, 1, 0],
        authorEmail: 'alice@example.com',
      } as MonthlyStatsResponse['series'][number],
      {
        userKey: 'bob@example.com',
        displayName: 'Bob B',
        totals: [10, 20, 30],
        missingCounts: [0, 0, 0],
        authorEmail: 'bob@example.com',
      } as MonthlyStatsResponse['series'][number],
    ],
  }) as MonthlyStatsResponse

describe('buildMonthlyTableTsv', () => {
  it('headers, rows, totals, and displayName output match expected order', () => {
    const data = sampleData()

    const tsv = buildMonthlyTableTsv(data)
    const lines = tsv.split('\n')

    const expectedHeader = ['ユーザー', '1月', '2月', '3月', '合計'].join('\t')
    expect(lines[0]).toBe(expectedHeader)

    const aliceTotal = data.series[0].totals.reduce((sum, value) => sum + value, 0)
    const bobTotal = data.series[1].totals.reduce((sum, value) => sum + value, 0)

    const expectedAliceRow = [
      'Alice A',
      ...data.series[0].totals.map((value) => value.toLocaleString()),
      aliceTotal.toLocaleString(),
    ].join('\t')

    const expectedBobRow = [
      'Bob B',
      ...data.series[1].totals.map((value) => value.toLocaleString()),
      bobTotal.toLocaleString(),
    ].join('\t')

    expect(lines[1]).toBe(expectedAliceRow)
    expect(lines[2]).toBe(expectedBobRow)
  })

  it('does not include authorEmail even when present', () => {
    const data = sampleData()

    const tsv = buildMonthlyTableTsv(data)

    expect(tsv).not.toContain('alice@example.com')
    expect(tsv).not.toContain('bob@example.com')
  })
})

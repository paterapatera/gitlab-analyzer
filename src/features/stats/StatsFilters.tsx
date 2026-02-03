/**
 * 集計フィルタ UI
 *
 * 年/ユーザー/プロジェクト/ブランチのフィルタを提供する。
 */

import { cn } from '@/lib/utils'
import { Label } from '@/components/ui/label'
import { NativeSelect, NativeSelectOption } from '@/components/ui/native-select'

/** フィルタのプロパティ */
export interface StatsFiltersProps {
  /** 選択中の年 */
  year: number
  /** 年変更時のコールバック */
  onYearChange: (year: number) => void
  /** 利用可能な年の配列 */
  availableYears?: number[]
  /** 追加のCSSクラス */
  className?: string
}

/**
 * 集計フィルタ UI
 */
export function StatsFilters({ year, onYearChange, availableYears, className }: StatsFiltersProps) {
  // デフォルトは現在年を含む過去5年
  const currentYear = new Date().getFullYear()
  const years = availableYears || Array.from({ length: 5 }, (_, i) => currentYear - i)

  return (
    <div className={cn('flex items-center gap-4', className)}>
      <div className="flex items-center gap-2">
        <Label htmlFor="year">年:</Label>
        <NativeSelect
          id="year"
          value={year}
          onChange={(e) => onYearChange(Number(e.target.value))}
          size="sm"
        >
          {years.map((y) => (
            <NativeSelectOption key={y} value={y}>
              {y}年
            </NativeSelectOption>
          ))}
        </NativeSelect>
      </div>
    </div>
  )
}

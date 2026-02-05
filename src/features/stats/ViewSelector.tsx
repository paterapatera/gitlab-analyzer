/**
 * ビュー選択コンポーネント
 *
 * プロジェクト別/横断ビューの切り替えを行う。
 */

import { Label } from '@/components/ui/label'
import { NativeSelect, NativeSelectOption } from '@/components/ui/native-select'
import type { StatsView } from '@/features/stats/useStatsData'

/** ビュー選択のプロパティ */
export interface ViewSelectorProps {
  statsView: StatsView
  onStatsViewChange: (view: StatsView) => void
}

/** ビュー選択コンポーネント */
export function ViewSelector({ statsView, onStatsViewChange }: ViewSelectorProps) {
  return (
    <div className="flex items-center gap-2">
      <Label>ビュー:</Label>
      <NativeSelect
        value={statsView}
        onChange={(e) => onStatsViewChange(e.target.value as StatsView)}
        size="sm"
      >
        <NativeSelectOption value="project">プロジェクト別</NativeSelectOption>
        <NativeSelectOption value="cross">横断</NativeSelectOption>
      </NativeSelect>
    </div>
  )
}

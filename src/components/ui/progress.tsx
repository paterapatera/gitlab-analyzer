/**
 * 進捗バーコンポーネント
 *
 * 進捗率を視覚的に表示する。
 */

import * as React from 'react'
import { cn } from '@/lib/utils'

/** 進捗バーのプロパティ */
export interface ProgressProps extends React.ComponentProps<'div'> {
  /** 進捗率（0-100） */
  value: number
}

/**
 * 進捗バー
 */
export function Progress({ value, className, ...props }: ProgressProps) {
  const clampedValue = Math.max(0, Math.min(100, value))

  return (
    <div
      role="progressbar"
      aria-valuenow={clampedValue}
      aria-valuemin={0}
      aria-valuemax={100}
      className={cn('relative h-2 w-full overflow-hidden rounded-full bg-muted', className)}
      {...props}
    >
      <div
        className="h-full w-full flex-1 bg-primary transition-all"
        style={{ transform: `translateX(-${100 - clampedValue}%)` }}
      />
    </div>
  )
}

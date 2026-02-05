/**
 * ユーザーフィルタリングUI
 *
 * トグル一覧でユーザーを選択/解除する。
 * ユーザー名はアルファベット順（昇順）でソートされる。
 * 50名以上の場合はスクロール可能なコンテナに配置される。
 */

import { memo } from 'react'
import { Button } from '@/components/ui/button'
import { ScrollArea } from '@/components/ui/scroll-area'
import { ToggleGroup, ToggleGroupItem } from '@/components/ui/toggle-group'
import { cn } from '@/lib/utils'

/** ユーザー情報 */
export interface UserInfo {
  /** ユーザーKey（内部識別用、authorEmail優先） */
  userKey: string
  /** 表示名（authorName） */
  displayName: string
}

/** UserFilterコンポーネントのプロパティ */
export interface UserFilterProps {
  /** 表示可能な全ユーザー */
  availableUsers: UserInfo[]
  /** 選択されたユーザーKey配列 */
  selectedUsers: string[]
  /** 選択状態変更時のコールバック */
  onSelectionChange: (userKeys: string[]) => void
  /** 全選択時のコールバック */
  onSelectAll: () => void
  /** 全解除時のコールバック */
  onDeselectAll: () => void
  /** ローディング中フラグ */
  isLoading?: boolean
  /** 追加のCSSクラス */
  className?: string
}

/** スクロールが必要なユーザー数の閾値 */
const SCROLL_THRESHOLD = 10

/**
 * ユーザーフィルタリングUI
 *
 * NOTE: トグルはアルファベット順（昇順）で表示される
 * NOTE: 10名以上の場合はスクロール可能なコンテナに配置される
 */
export const UserFilter = memo(function UserFilter({
  availableUsers,
  selectedUsers,
  onSelectionChange,
  onSelectAll,
  onDeselectAll,
  isLoading = false,
  className,
}: UserFilterProps) {
  // ユーザー名でアルファベット順ソート
  const sortedUsers = [...availableUsers].sort((a, b) => a.displayName.localeCompare(b.displayName))

  // 閾値以上の場合はスクロール可能にする
  const needsScroll = sortedUsers.length >= SCROLL_THRESHOLD

  // ユーザーがいない場合
  if (sortedUsers.length === 0) {
    return (
      <div className={cn('text-sm text-muted-foreground', className)}>
        表示するユーザーがいません
      </div>
    )
  }

  const toggleList = (
    <ToggleGroup
      type="multiple"
      variant="outline"
      size="sm"
      spacing={4}
      value={selectedUsers}
      onValueChange={onSelectionChange}
      disabled={isLoading}
      className="flex w-full flex-wrap justify-start"
    >
      {sortedUsers.map((user) => (
        <ToggleGroupItem key={user.userKey} value={user.userKey} className="justify-start">
          {user.displayName}
        </ToggleGroupItem>
      ))}
    </ToggleGroup>
  )

  return (
    <div className={cn('space-y-3', className)}>
      <div className="flex items-center justify-between">
        <h3 className="text-sm font-medium">ユーザー選択</h3>
        <div className="flex gap-2">
          <Button variant="outline" size="sm" onClick={onSelectAll} disabled={isLoading}>
            全選択
          </Button>
          <Button variant="outline" size="sm" onClick={onDeselectAll} disabled={isLoading}>
            全解除
          </Button>
        </div>
      </div>

      {selectedUsers.length === 0 && (
        <p className="text-sm text-muted-foreground">ユーザーを選択してください</p>
      )}

      {needsScroll ? <ScrollArea className="h-[300px] pr-4">{toggleList}</ScrollArea> : toggleList}

      <p className="text-xs text-muted-foreground">
        {selectedUsers.length} / {sortedUsers.length} 名選択中
      </p>
    </div>
  )
})

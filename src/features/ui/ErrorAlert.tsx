/**
 * エラーアラートコンポーネント
 *
 * エラーメッセージをユーザーフレンドリーに表示する。
 * 次に取るべき行動が分かるようにガイダンスを含む。
 */

import { Alert, AlertTitle, AlertDescription } from '@/components/ui/alert'
import { Button } from '@/components/ui/button'
import { XCircle, X } from 'lucide-react'

/** エラーアラートのプロパティ */
export interface ErrorAlertProps {
  /** エラーメッセージ */
  message: string
  /** 追加のCSSクラス */
  className?: string
  /** 閉じるボタンのコールバック */
  onDismiss?: () => void
}

/**
 * エラーアラートコンポーネント
 *
 * @example
 * ```tsx
 * <ErrorAlert
 *   message="認証に失敗しました\n\n💡 トークンを確認してください"
 *   onDismiss={() => clearError()}
 * />
 * ```
 */
export function ErrorAlert({ message, className, onDismiss }: ErrorAlertProps) {
  // メッセージを改行で分割してレンダリング
  const lines = message.split('\n')
  const title = lines[0]
  const description = lines.slice(1).filter((line) => line.trim().length > 0)

  return (
    <Alert variant="destructive" className={className}>
      <XCircle className="h-4 w-4" />
      <AlertTitle className="flex items-center justify-between">
        <span>{title}</span>
        {onDismiss && (
          <Button
            variant="ghost"
            size="icon-xs"
            onClick={onDismiss}
            aria-label="閉じる"
            className="ml-2 -mr-2"
          >
            <X className="h-4 w-4" />
          </Button>
        )}
      </AlertTitle>
      {description.length > 0 && (
        <AlertDescription>
          {description.map((line, index) => (
            <p key={index}>{line}</p>
          ))}
        </AlertDescription>
      )}
    </Alert>
  )
}

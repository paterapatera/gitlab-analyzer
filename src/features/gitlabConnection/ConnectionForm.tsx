/**
 * GitLab 接続設定フォーム
 *
 * ベース URL とアクセストークンを入力して保存する。
 */

import { useState, useCallback, useEffect } from 'react'
import { invokeCommand } from '@/lib/tauri'
import type { GitLabConnection, GitLabConnectionInput } from '@/lib/contracts/tauriCommands'
import { ErrorAlert } from '@/features/ui/ErrorAlert'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '@/components/ui/card'
import { Skeleton } from '@/components/ui/skeleton'

/** 接続フォームのプロパティ */
export interface ConnectionFormProps {
  /** 保存成功時のコールバック */
  onSaved?: () => void
  /** 追加のCSSクラス */
  className?: string
}

/**
 * GitLab 接続設定フォーム
 */
export function ConnectionForm({ onSaved, className }: ConnectionFormProps) {
  const [baseUrl, setBaseUrl] = useState('')
  const [accessToken, setAccessToken] = useState('')
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [isInitialized, setIsInitialized] = useState(false)

  // 既存の接続設定を読み込む
  useEffect(() => {
    const loadConnection = async () => {
      const result = await invokeCommand<GitLabConnection | null>('get_gitlab_connection')
      if (result.ok && result.data) {
        setBaseUrl(result.data.baseUrl)
        // NOTE: トークンはセキュリティ上返却されない
      }
      setIsInitialized(true)
    }
    loadConnection()
  }, [])

  const handleSubmit = useCallback(
    async (e: React.FormEvent) => {
      e.preventDefault()
      setError(null)
      setIsLoading(true)

      try {
        const input: GitLabConnectionInput = {
          baseUrl: baseUrl.trim(),
          accessToken: accessToken.trim(),
        }

        const result = await invokeCommand<void>('set_gitlab_connection', { input })

        if (!result.ok) {
          setError(result.error)
          return
        }

        // 成功時はトークン入力をクリア
        setAccessToken('')
        onSaved?.()
      } finally {
        setIsLoading(false)
      }
    },
    [baseUrl, accessToken, onSaved],
  )

  // フォームのバリデーション
  const isValid = baseUrl.trim().length > 0 && accessToken.trim().length > 0
  const isHttpUrl = baseUrl.startsWith('http://') || baseUrl.startsWith('https://')

  if (!isInitialized) {
    return (
      <Card className={className}>
        <CardHeader>
          <Skeleton className="h-6 w-48" />
        </CardHeader>
        <CardContent className="space-y-4">
          <Skeleton className="h-9 w-full" />
          <Skeleton className="h-9 w-full" />
          <Skeleton className="h-9 w-full" />
        </CardContent>
      </Card>
    )
  }

  return (
    <Card className={className}>
      <CardHeader>
        <CardTitle>GitLab 接続設定</CardTitle>
        <CardDescription>GitLab のベース URL とアクセストークンを設定します</CardDescription>
      </CardHeader>
      <CardContent>
        <form onSubmit={handleSubmit} className="space-y-4">
          {error && <ErrorAlert message={error} onDismiss={() => setError(null)} />}

          <div className="space-y-2">
            <Label htmlFor="baseUrl">GitLab ベース URL</Label>
            <Input
              id="baseUrl"
              type="url"
              value={baseUrl}
              onChange={(e) => setBaseUrl(e.target.value)}
              placeholder="https://gitlab.example.com"
              aria-invalid={!isHttpUrl && baseUrl.length > 0}
              required
              disabled={isLoading}
            />
            {!isHttpUrl && baseUrl.length > 0 && (
              <p className="text-sm text-destructive">
                http:// または https:// で始まる URL を入力してください
              </p>
            )}
          </div>

          <div className="space-y-2">
            <Label htmlFor="accessToken">アクセストークン</Label>
            <Input
              id="accessToken"
              type="password"
              value={accessToken}
              onChange={(e) => setAccessToken(e.target.value)}
              placeholder="glpat-xxxxxxxxxxxx"
              required
              disabled={isLoading}
              autoComplete="off"
            />
            <p className="text-sm text-muted-foreground">
              GitLab の Settings → Access Tokens から取得できます（api または read_api
              スコープが必要）
            </p>
          </div>

          <Button type="submit" disabled={!isValid || !isHttpUrl || isLoading} className="w-full">
            {isLoading ? '保存中...' : '保存'}
          </Button>
        </form>
      </CardContent>
    </Card>
  )
}

/**
 * 非同期状態管理フック
 *
 * loading/empty/error/success の状態を統一的に管理する。
 */

import { useState, useCallback } from 'react'

/** 非同期操作の状態 */
export type AsyncState<T> =
  | { status: 'idle' }
  | { status: 'loading' }
  | { status: 'success'; data: T }
  | { status: 'error'; error: string }

/** 非同期状態管理フックの戻り値 */
export interface UseAsyncStateReturn<T, Args extends unknown[]> {
  /** 現在の状態 */
  state: AsyncState<T>
  /** 操作を実行 */
  execute: (...args: Args) => Promise<void>
  /** 状態をリセット */
  reset: () => void
  /** ローディング中かどうか */
  isLoading: boolean
  /** エラーメッセージ（エラー時のみ） */
  error: string | null
  /** データ（成功時のみ） */
  data: T | null
}

/**
 * 非同期操作の状態を管理するフック
 *
 * @param asyncFn - 実行する非同期関数
 * @returns 状態と操作関数
 *
 * @example
 * ```tsx
 * const { state, execute, isLoading, error, data } = useAsyncState(
 *   async (id: number) => await fetchData(id)
 * )
 *
 * // ボタンクリックで実行
 * <button onClick={() => execute(123)} disabled={isLoading}>
 *   {isLoading ? '読み込み中...' : '取得'}
 * </button>
 * ```
 */
export function useAsyncState<T, Args extends unknown[]>(
  asyncFn: (...args: Args) => Promise<T>,
): UseAsyncStateReturn<T, Args> {
  const [state, setState] = useState<AsyncState<T>>({ status: 'idle' })

  const execute = useCallback(
    async (...args: Args) => {
      setState({ status: 'loading' })
      try {
        const data = await asyncFn(...args)
        setState({ status: 'success', data })
      } catch (err) {
        const message = err instanceof Error ? err.message : String(err)
        setState({ status: 'error', error: message })
      }
    },
    [asyncFn],
  )

  const reset = useCallback(() => {
    setState({ status: 'idle' })
  }, [])

  return {
    state,
    execute,
    reset,
    isLoading: state.status === 'loading',
    error: state.status === 'error' ? state.error : null,
    data: state.status === 'success' ? state.data : null,
  }
}

/**
 * Tauri invoke の薄いラッパー
 *
 * フロントエンドから Tauri command を呼び出すためのユーティリティ。
 * 型安全な invoke 呼び出しと共通のエラーハンドリングを提供する。
 */

import { invoke as tauriInvoke } from '@tauri-apps/api/core'

/**
 * Tauri command の呼び出し結果
 * エラー時は `error` プロパティにメッセージが入る
 */
export type InvokeResult<T> = { ok: true; data: T } | { ok: false; error: string }

/**
 * Tauri command を呼び出し、結果を Result 型で返す
 *
 * @param cmd - コマンド名（Rust 側の `#[tauri::command]` 関数名）
 * @param args - コマンドに渡す引数（省略可能）
 * @returns 成功時は `{ ok: true, data }`, 失敗時は `{ ok: false, error }`
 *
 * @example
 * ```ts
 * const result = await invokeCommand<GitLabConnection>('get_gitlab_connection')
 * if (result.ok) {
 *   console.log(result.data.baseUrl)
 * } else {
 *   console.error(result.error)
 * }
 * ```
 */
export async function invokeCommand<T>(
  cmd: string,
  args?: Record<string, unknown>,
): Promise<InvokeResult<T>> {
  try {
    const data = await tauriInvoke<T>(cmd, args)
    return { ok: true, data }
  } catch (err) {
    // Tauri のエラーは文字列または Error オブジェクト
    const message =
      err instanceof Error ? err.message : typeof err === 'string' ? err : 'Unknown error'
    return { ok: false, error: message }
  }
}

/**
 * Tauri command を呼び出し、失敗時は例外をスローする
 *
 * @param cmd - コマンド名
 * @param args - コマンドに渡す引数（省略可能）
 * @returns コマンドの戻り値
 * @throws エラー時は Error をスロー
 */
export async function invokeCommandOrThrow<T>(
  cmd: string,
  args?: Record<string, unknown>,
): Promise<T> {
  const result = await invokeCommand<T>(cmd, args)
  if (!result.ok) {
    throw new Error(result.error)
  }
  return result.data
}

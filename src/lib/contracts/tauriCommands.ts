/**
 * Tauri Commands の TypeScript 型定義
 *
 * OpenAPI 契約 (`specs/001-gitlab-commit-lines/contracts/tauri-commands.openapi.yaml`) に対応する
 * フロントエンド側の型定義。Rust バックエンドとの通信に使用する。
 */

// =============================================================================
// GitLab 接続設定
// =============================================================================

/**
 * GitLab 接続情報（取得時のレスポンス）
 * NOTE: セキュリティ要件により accessToken は含まない（FR-016）
 */
export interface GitLabConnection {
  /** GitLab のベース URL（例: https://gitlab.example.com） */
  baseUrl: string
  /** 最終更新日時（ISO8601 UTC） */
  updatedAtUtc: string
}

/**
 * GitLab 接続情報の入力（登録/更新時のリクエスト）
 */
export interface GitLabConnectionInput {
  /** GitLab のベース URL */
  baseUrl: string
  /** アクセストークン */
  accessToken: string
}

// =============================================================================
// プロジェクト
// =============================================================================

/**
 * GitLab プロジェクト
 */
export interface Project {
  /** GitLab 内部 ID */
  projectId: number
  /** プロジェクト名 */
  name: string
  /** 名前空間付きパス（例: group/project） */
  pathWithNamespace: string
  /** Web UI の URL */
  webUrl: string
}

// =============================================================================
// ブランチ
// =============================================================================

/**
 * GitLab ブランチ
 */
export interface Branch {
  /** プロジェクト ID */
  projectId: number
  /** ブランチ名 */
  name: string
  /** デフォルトブランチかどうか */
  isDefault?: boolean
}

// =============================================================================
// コミット収集
// =============================================================================

/**
 * コミット収集リクエスト
 */
export interface CollectCommitsRequest {
  /** 対象プロジェクト ID */
  projectId: number
  /** 対象ブランチ名 */
  branchName: string
  /** 収集開始日時（ISO8601 UTC、省略可能） */
  sinceUtc?: string | null
  /** 収集終了日時（ISO8601 UTC、省略可能） */
  untilUtc?: string | null
}

/**
 * コミット収集結果
 */
export interface CollectCommitsResult {
  /** 新規挿入件数 */
  insertedCount: number
  /** 重複スキップ件数 */
  skippedDuplicateCount: number
  /** stats 欠損コミット件数 */
  missingStatsCount: number
}

// =============================================================================
// 一括コミット収集
// =============================================================================

/** 一括収集開始レスポンス */
export interface BulkCollectionStarted {
  runId: string
  totalTargets: number
}

/** 一括収集の対象結果 */
export interface BulkCollectionTargetResult {
  projectId: number
  branchName: string
  status: 'pending' | 'success' | 'failed'
  newCommitsCount?: number | null
  errorMessage?: string | null
  processedAt?: string | null
}

/** 一括収集の状態 */
export interface BulkCollectionStatus {
  runId: string
  status: 'running' | 'completed' | 'cancelled'
  totalTargets: number
  completedCount: number
  successCount: number
  failedCount: number
  startedAt: string
  completedAt?: string | null
  results?: BulkCollectionTargetResult[]
}

/** 一括収集進捗イベント */
export interface BulkCollectionProgress {
  runId: string
  totalTargets: number
  completedCount: number
  successCount: number
  failedCount: number
  currentTarget?: {
    projectId: number
    branchName: string
  } | null
}

/** 失敗対象の再試行リクエスト */
export interface RetryFailedRequest {
  runId: string
}

// =============================================================================
// 月次集計
// =============================================================================

/**
 * プロジェクトビュー集計リクエスト
 */
export interface ProjectViewStatsRequest {
  /** 対象プロジェクト ID */
  projectId: number
  /** 対象ブランチ名 */
  branchName: string
  /** 対象年 */
  year: number
  /** フィルタするユーザーキー（省略時は全ユーザー） */
  userKeys?: string[]
}

/**
 * 横断ビュー集計リクエスト
 */
export interface CrossViewStatsRequest {
  /** 対象年 */
  year: number
  /** フィルタするユーザーキー（省略時は全ユーザー） */
  userKeys?: string[]
}

/**
 * 月次集計レスポンス
 */
export interface MonthlyStatsResponse {
  /** 対象月の配列（1-12） */
  months: number[]
  /** ユーザー別のデータ系列 */
  series: UserMonthlySeries[]
}

/**
 * ユーザー別月次データ系列
 */
export interface UserMonthlySeries {
  /** ユーザーキー（内部識別用、authorEmail 優先） */
  userKey: string
  /** 表示名（authorName） */
  displayName: string
  /** 月別合計行数（months 配列に対応） */
  totals: number[]
  /** 月別欠損コミット件数（months 配列に対応） */
  missingCounts: number[]
}

// =============================================================================
// ユーザーフィルタ選択状態
// =============================================================================

/**
 * ユーザーフィルタ選択状態のビュー種別
 */
export type UserFilterViewType = 'project-view' | 'cross-view'

/**
 * ユーザーフィルタ選択状態のコンテキストキー
 * - プロジェクト別: "<project>/<branch>/<year>"
 * - 横断: "<year>"
 */
export type UserFilterContextKey = string

/**
 * 選択されたユーザーID配列（authorEmail優先、フォールバックauthorName）
 */
export type SelectedUsers = string[]

// =============================================================================
// Tauri Command Functions
// =============================================================================

import { invokeCommandOrThrow } from '@/lib/tauri'

/**
 * ユーザーフィルタ選択状態を取得
 *
 * @param viewType - ビュー種別（"project-view" or "cross-view"）
 * @param contextKey - コンテキストキー
 * @returns 選択されたユーザーID配列（存在しない場合は空配列）
 *
 * NOTE: 空配列は「未保存（全選択として扱う）」を意味する
 */
export async function getUserFilterState(
  viewType: UserFilterViewType,
  contextKey: UserFilterContextKey,
): Promise<SelectedUsers> {
  return invokeCommandOrThrow<SelectedUsers>('user_filter_get', {
    viewType,
    contextKey,
  })
}

/**
 * ユーザーフィルタ選択状態を保存
 *
 * @param viewType - ビュー種別（"project-view" or "cross-view"）
 * @param contextKey - コンテキストキー
 * @param selectedUsers - 選択されたユーザーID配列
 *
 * NOTE: 同一のview_type+context_keyが存在する場合は上書き
 */
export async function setUserFilterState(
  viewType: UserFilterViewType,
  contextKey: UserFilterContextKey,
  selectedUsers: SelectedUsers,
): Promise<void> {
  return invokeCommandOrThrow<void>('user_filter_set', {
    viewType,
    contextKey,
    selectedUsers,
  })
}

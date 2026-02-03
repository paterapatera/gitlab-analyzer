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

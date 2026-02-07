/**
 * プロジェクト一覧を検索文字列で絞り込む。
 */

import type { Project } from '@/lib/contracts/tauriCommands'

/** フィルタ結果 */
export interface ProjectFilterResult {
  items: Project[]
  isEmpty: boolean
  isTruncated: boolean
}

/** フィルタオプション */
export interface ProjectFilterOptions {
  maxResults?: number
  minQueryLength?: number
}

/**
 * プロジェクト一覧を部分一致（大小区別なし）で検索する。
 * NOTE: 空入力は全件表示、最大件数を超える場合は表示を切り詰める。
 */
export function filterProjects(
  projects: Project[],
  query: string,
  options: ProjectFilterOptions = {},
): ProjectFilterResult {
  const maxResults = options.maxResults ?? 100
  const minQueryLength = options.minQueryLength ?? 1
  const normalizedQuery = query.trim().toLowerCase()

  const filtered =
    normalizedQuery.length >= minQueryLength
      ? projects.filter((project) => {
          const name = project.name.toLowerCase()
          const path = project.pathWithNamespace.toLowerCase()
          return name.includes(normalizedQuery) || path.includes(normalizedQuery)
        })
      : projects

  const isTruncated = filtered.length > maxResults
  const items = isTruncated ? filtered.slice(0, maxResults) : filtered

  return {
    items,
    isEmpty: filtered.length === 0,
    isTruncated,
  }
}

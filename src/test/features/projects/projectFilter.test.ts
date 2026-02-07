import { describe, it, expect } from 'vitest'
import { filterProjects } from '@/features/projects/projectFilter'
import type { Project } from '@/lib/contracts/tauriCommands'

const projects: Project[] = [
  {
    projectId: 1,
    name: 'gitlab-analyzer',
    pathWithNamespace: 'tools/gitlab-analyzer',
    webUrl: 'https://example.com/tools/gitlab-analyzer',
  },
  {
    projectId: 2,
    name: 'acme-service',
    pathWithNamespace: 'acme/services/acme-service',
    webUrl: 'https://example.com/acme/services/acme-service',
  },
  {
    projectId: 3,
    name: 'misc',
    pathWithNamespace: 'group/misc',
    webUrl: 'https://example.com/group/misc',
  },
]

describe('filterProjects', () => {
  it('空入力は全件を返す', () => {
    const result = filterProjects(projects, '')

    expect(result.items).toHaveLength(3)
    expect(result.isEmpty).toBe(false)
  })

  it('名前とパスの部分一致で絞り込む', () => {
    const nameMatch = filterProjects(projects, 'Ana')
    expect(nameMatch.items).toHaveLength(1)
    expect(nameMatch.items[0].projectId).toBe(1)

    const pathMatch = filterProjects(projects, 'services')
    expect(pathMatch.items).toHaveLength(1)
    expect(pathMatch.items[0].projectId).toBe(2)
  })

  it('一致しない場合は空結果になる', () => {
    const result = filterProjects(projects, 'nomatch')

    expect(result.items).toHaveLength(0)
    expect(result.isEmpty).toBe(true)
  })

  it('最大件数を超える場合は切り詰める', () => {
    const largeList: Project[] = Array.from({ length: 5 }, (_, index) => ({
      projectId: index + 1,
      name: `project-${index + 1}`,
      pathWithNamespace: `group/project-${index + 1}`,
      webUrl: `https://example.com/group/project-${index + 1}`,
    }))

    const result = filterProjects(largeList, 'project', { maxResults: 2 })

    expect(result.items).toHaveLength(2)
    expect(result.isTruncated).toBe(true)
  })
})

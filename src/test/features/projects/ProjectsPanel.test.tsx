import { describe, it, expect, vi } from 'vitest'
import { render, screen, waitFor } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { ProjectsPanel } from '@/features/projects/ProjectsPanel'
import type { Project } from '@/lib/contracts/tauriCommands'

vi.mock('@/lib/tauri', () => ({
  invokeCommand: vi.fn(),
}))

import { invokeCommand } from '@/lib/tauri'

const mockInvokeCommand = vi.mocked(invokeCommand)

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
]

describe('ProjectsPanel', () => {
  it('プロジェクトが空の場合に案内を表示する', async () => {
    mockInvokeCommand.mockResolvedValue({ ok: true, data: [] })

    render(<ProjectsPanel onProjectSelect={vi.fn()} />)

    expect(
      await screen.findByText(
        'プロジェクトがありません。「プロジェクト同期」をクリックして取得してください。',
      ),
    ).toBeInTheDocument()
  })

  it('読み込み中はローディングメッセージが表示される', async () => {
    let resolvePromise: (value: unknown) => void
    const pending = new Promise((resolve) => {
      resolvePromise = resolve
    })

    mockInvokeCommand.mockReturnValue(pending as Promise<any>)

    render(<ProjectsPanel onProjectSelect={vi.fn()} />)

    const user = userEvent.setup()
    const input = screen.getByPlaceholderText('プロジェクトを検索')
    await user.click(input)

    expect(screen.getByText('読み込み中...')).toBeInTheDocument()

    resolvePromise!({ ok: true, data: [] })

    await waitFor(() => {
      expect(screen.queryByText('読み込み中...')).not.toBeInTheDocument()
    })
  })

  it('選択時にコールバックが呼ばれる', async () => {
    mockInvokeCommand.mockResolvedValue({ ok: true, data: projects })

    const onProjectSelect = vi.fn()
    render(<ProjectsPanel onProjectSelect={onProjectSelect} />)

    const user = userEvent.setup()
    const input = await screen.findByPlaceholderText('プロジェクトを検索')
    await user.click(input)
    await user.type(input, 'acme')

    await user.click(screen.getByText('acme-service'))

    expect(onProjectSelect).toHaveBeenCalledWith(projects[1])
  })
})

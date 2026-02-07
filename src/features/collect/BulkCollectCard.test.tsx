/**
 * 一括収集カードのテスト
 */

import { describe, it, expect, vi, beforeEach } from 'vitest'
import { render, screen, waitFor } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { BulkCollectCard } from './BulkCollectCard'

vi.mock('@/lib/tauri', () => ({
  invokeCommand: vi.fn(),
}))

const mockListen = vi.fn()
vi.mock('@tauri-apps/api/event', () => ({
  listen: (...args: unknown[]) => mockListen(...args),
}))

import { invokeCommand } from '@/lib/tauri'

const mockInvokeCommand = vi.mocked(invokeCommand)

describe('BulkCollectCard', () => {
  let progressHandler: ((event: { payload: any }) => void) | null = null

  beforeEach(() => {
    vi.clearAllMocks()
    progressHandler = null

    mockListen.mockImplementation((_eventName: string, handler: (event: any) => void) => {
      progressHandler = handler
      return Promise.resolve(() => {})
    })
  })

  it('進捗イベントで進捗表示が更新される', async () => {
    mockInvokeCommand.mockImplementation(async (cmd: string) => {
      if (cmd === 'collect_commits_bulk') {
        return { ok: true, data: { runId: 'run-1', totalTargets: 3 } }
      }
      if (cmd === 'get_bulk_collection_status') {
        return {
          ok: true,
          data: {
            runId: 'run-1',
            status: 'running',
            totalTargets: 3,
            completedCount: 0,
            successCount: 0,
            failedCount: 0,
            startedAt: '2026-02-07T00:00:00Z',
            completedAt: null,
            results: [],
          },
        }
      }
      return { ok: true, data: null }
    })

    render(<BulkCollectCard />)

    const user = userEvent.setup()
    await user.click(screen.getByRole('button', { name: 'すべての続きを収集' }))

    await waitFor(() => {
      expect(progressHandler).not.toBeNull()
    })

    progressHandler?.({
      payload: {
        runId: 'run-1',
        totalTargets: 3,
        completedCount: 1,
        successCount: 1,
        failedCount: 0,
        currentTarget: { projectId: 1, branchName: 'main' },
      },
    })

    await waitFor(() => {
      expect(screen.getByText('1 / 3 完了（成功: 1, 失敗: 0）')).toBeInTheDocument()
    })
  })

  it('失敗対象の再試行が実行される', async () => {
    mockInvokeCommand.mockImplementation(async (cmd: string) => {
      if (cmd === 'collect_commits_bulk') {
        return { ok: true, data: { runId: 'run-1', totalTargets: 1 } }
      }
      if (cmd === 'get_bulk_collection_status') {
        return {
          ok: true,
          data: {
            runId: 'run-1',
            status: 'completed',
            totalTargets: 1,
            completedCount: 1,
            successCount: 0,
            failedCount: 1,
            startedAt: '2026-02-07T00:00:00Z',
            completedAt: '2026-02-07T00:10:00Z',
            results: [
              {
                projectId: 1,
                branchName: 'main',
                status: 'failed',
                newCommitsCount: null,
                errorMessage: 'error',
                processedAt: '2026-02-07T00:05:00Z',
              },
            ],
          },
        }
      }
      if (cmd === 'retry_failed_targets') {
        return { ok: true, data: { runId: 'run-2', totalTargets: 1 } }
      }
      return { ok: true, data: null }
    })

    render(<BulkCollectCard />)

    const user = userEvent.setup()
    await user.click(screen.getByRole('button', { name: 'すべての続きを収集' }))

    await waitFor(() => {
      expect(screen.getByRole('button', { name: '失敗分を再試行' })).not.toBeDisabled()
    })

    await user.click(screen.getByRole('button', { name: '失敗分を再試行' }))

    expect(mockInvokeCommand).toHaveBeenCalledWith('retry_failed_targets', {
      request: { runId: 'run-1' },
    })
  })
})

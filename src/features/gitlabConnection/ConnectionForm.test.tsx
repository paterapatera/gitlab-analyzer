/**
 * 接続フォームのテスト
 */

import { describe, it, expect, vi, beforeEach } from 'vitest'
import { render, screen, waitFor } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { ConnectionForm } from './ConnectionForm'

// Tauri の invoke をモック
vi.mock('@/lib/tauri', () => ({
  invokeCommand: vi.fn(),
}))

import { invokeCommand } from '@/lib/tauri'

const mockInvokeCommand = vi.mocked(invokeCommand)

describe('ConnectionForm', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    // デフォルトで接続設定なし
    mockInvokeCommand.mockResolvedValue({ ok: true, data: null })
  })

  it('フォームが表示される', async () => {
    render(<ConnectionForm />)

    await waitFor(() => {
      expect(screen.getByLabelText(/ベース URL/i)).toBeInTheDocument()
    })

    expect(screen.getByLabelText(/アクセストークン/i)).toBeInTheDocument()
    expect(screen.getByRole('button', { name: /保存/i })).toBeInTheDocument()
  })

  it('無効な URL でエラーメッセージが表示される', async () => {
    render(<ConnectionForm />)

    await waitFor(() => {
      expect(screen.getByLabelText(/ベース URL/i)).toBeInTheDocument()
    })

    const user = userEvent.setup()
    await user.type(screen.getByLabelText(/ベース URL/i), 'invalid-url')

    expect(screen.getByText(/http:\/\/ または https:\/\//i)).toBeInTheDocument()
  })

  it('有効な入力で保存ボタンが有効になる', async () => {
    render(<ConnectionForm />)

    await waitFor(() => {
      expect(screen.getByLabelText(/ベース URL/i)).toBeInTheDocument()
    })

    const user = userEvent.setup()
    await user.type(screen.getByLabelText(/ベース URL/i), 'https://gitlab.example.com')
    await user.type(screen.getByLabelText(/アクセストークン/i), 'glpat-test')

    expect(screen.getByRole('button', { name: /保存/i })).not.toBeDisabled()
  })

  it('空のトークンでは保存ボタンが無効', async () => {
    render(<ConnectionForm />)

    await waitFor(() => {
      expect(screen.getByLabelText(/ベース URL/i)).toBeInTheDocument()
    })

    const user = userEvent.setup()
    await user.type(screen.getByLabelText(/ベース URL/i), 'https://gitlab.example.com')

    expect(screen.getByRole('button', { name: /保存/i })).toBeDisabled()
  })

  it('既存の接続設定が読み込まれる', async () => {
    mockInvokeCommand.mockResolvedValue({
      ok: true,
      data: { baseUrl: 'https://existing.gitlab.com', updatedAtUtc: '2026-01-01T00:00:00Z' },
    })

    render(<ConnectionForm />)

    await waitFor(() => {
      expect(screen.getByLabelText(/ベース URL/i)).toHaveValue('https://existing.gitlab.com')
    })
  })
})

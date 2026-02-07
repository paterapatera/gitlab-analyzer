/**
 * プロジェクト検索オートコンプリート
 */

import { useMemo, useState, useEffect, useCallback } from 'react'
import type { KeyboardEvent } from 'react'
import { Check, X } from 'lucide-react'
import type { Project } from '@/lib/contracts/tauriCommands'
import { cn } from '@/lib/utils'
import { Button } from '@/components/ui/button'
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandItem,
  CommandList,
  CommandLoading,
  CommandInput,
} from '@/components/ui/command'
import { Popover, PopoverAnchor, PopoverContent } from '@/components/ui/popover'
import { filterProjects } from '@/features/projects/projectFilter'

/** オートコンプリートのプロパティ */
export interface ProjectAutocompleteProps {
  projects: Project[]
  selectedProjectId?: number
  isLoading?: boolean
  placeholder?: string
  className?: string
  onSelect: (project: Project) => void
}

/**
 * プロジェクト検索オートコンプリート
 */
export function ProjectAutocomplete({
  projects,
  selectedProjectId,
  isLoading = false,
  placeholder = 'プロジェクトを検索',
  className,
  onSelect,
}: ProjectAutocompleteProps) {
  const [open, setOpen] = useState(false)
  const [query, setQuery] = useState('')
  const [debouncedQuery, setDebouncedQuery] = useState('')

  useEffect(() => {
    const handle = setTimeout(() => {
      setDebouncedQuery(query)
    }, 150)

    return () => clearTimeout(handle)
  }, [query])

  const filtered = useMemo(
    () => filterProjects(projects, debouncedQuery),
    [projects, debouncedQuery],
  )

  const emptyMessage = useMemo(() => {
    if (projects.length === 0 && !isLoading) {
      return 'プロジェクトがありません。'
    }

    return '該当するプロジェクトがありません'
  }, [projects.length, isLoading])

  const handleSelect = useCallback(
    (project: Project) => {
      onSelect(project)
      setQuery(project.name)
      setOpen(false)
    },
    [onSelect],
  )

  const handleClear = useCallback(() => {
    setQuery('')
    setOpen(true)
  }, [])

  const handleValueChange = useCallback((value: string) => {
    setQuery(value)
    setOpen(true)
  }, [])

  const handleKeyDown = useCallback(
    (event: KeyboardEvent<HTMLInputElement>) => {
      if (event.key === 'Escape') {
        event.preventDefault()
        setOpen(false)
        return
      }

      if ((event.key === 'ArrowDown' || event.key === 'ArrowUp') && !open) {
        setOpen(true)
      }
    },
    [open],
  )

  return (
    <Command
      shouldFilter={false}
      className={cn('w-full rounded-md border bg-background', className)}
    >
      <Popover open={open} onOpenChange={setOpen}>
        <PopoverAnchor asChild>
          <div className="relative" data-autocomplete-anchor>
            <CommandInput
              value={query}
              onValueChange={handleValueChange}
              placeholder={placeholder}
              className="pr-10"
              onFocus={() => setOpen(true)}
              onKeyDown={handleKeyDown}
            />
            {query.length > 0 && (
              <Button
                type="button"
                variant="ghost"
                size="icon-xs"
                className="absolute right-2 top-1/2 -translate-y-1/2"
                onClick={handleClear}
                aria-label="入力をクリア"
              >
                <X className="h-4 w-4" />
              </Button>
            )}
          </div>
        </PopoverAnchor>
        <PopoverContent
          className="w-[--radix-popover-trigger-width] p-0"
          align="start"
          onOpenAutoFocus={(event) => event.preventDefault()}
          onInteractOutside={(event) => {
            const target = event.target as HTMLElement | null
            if (target?.closest('[data-autocomplete-anchor]')) {
              event.preventDefault()
            }
          }}
        >
          <CommandList>
            {isLoading && <CommandLoading>読み込み中...</CommandLoading>}

            {!isLoading && filtered.isEmpty && <CommandEmpty>{emptyMessage}</CommandEmpty>}

            {!isLoading && !filtered.isEmpty && (
              <CommandGroup>
                {filtered.items.map((project) => (
                  <CommandItem
                    key={project.projectId}
                    value={project.name}
                    onSelect={() => handleSelect(project)}
                  >
                    <Check
                      className={cn(
                        'h-4 w-4 text-primary',
                        selectedProjectId === project.projectId ? 'opacity-100' : 'opacity-0',
                      )}
                    />
                    <div>
                      <div className="font-medium">{project.name}</div>
                      <div className="text-xs text-muted-foreground">
                        {project.pathWithNamespace}
                      </div>
                    </div>
                  </CommandItem>
                ))}
              </CommandGroup>
            )}
          </CommandList>

          {!isLoading && filtered.isTruncated && (
            <div className="border-t px-3 py-2 text-xs text-muted-foreground">
              表示件数が多いため、さらに絞り込んでください。
            </div>
          )}
        </PopoverContent>
      </Popover>
    </Command>
  )
}

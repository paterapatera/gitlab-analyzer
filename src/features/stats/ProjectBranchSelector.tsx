/**
 * プロジェクト・ブランチ選択コンポーネント
 *
 * プロジェクトとブランチを選択するためのドロップダウン。
 */

import type { Project } from '@/lib/contracts/tauriCommands'
import { Label } from '@/components/ui/label'
import { NativeSelect, NativeSelectOption } from '@/components/ui/native-select'
import { ProjectAutocomplete } from '@/features/projects/ProjectAutocomplete'

/** プロジェクト・ブランチ選択のプロパティ */
export interface ProjectBranchSelectorProps {
  projects: Project[]
  selectedProject: Project | null
  onProjectSelect: (project: Project) => void
  branches: Array<{ name: string }>
  selectedBranch: string
  onBranchChange: (e: React.ChangeEvent<HTMLSelectElement>) => void
}

/** プロジェクト・ブランチ選択コンポーネント */
export function ProjectBranchSelector({
  projects,
  selectedProject,
  onProjectSelect,
  branches,
  selectedBranch,
  onBranchChange,
}: ProjectBranchSelectorProps) {
  return (
    <>
      <div className="flex items-center gap-2">
        <Label>プロジェクト:</Label>
        <div className="min-w-[240px]">
          <ProjectAutocomplete
            projects={projects}
            selectedProjectId={selectedProject?.projectId}
            onSelect={onProjectSelect}
            placeholder="プロジェクトを検索"
          />
        </div>
      </div>

      {selectedProject && (
        <div className="flex items-center gap-2">
          <Label>ブランチ:</Label>
          <NativeSelect
            value={selectedBranch}
            onChange={onBranchChange}
            size="sm"
            className="min-w-[150px]"
          >
            {branches.map((branch) => (
              <NativeSelectOption key={branch.name} value={branch.name}>
                {branch.name}
              </NativeSelectOption>
            ))}
          </NativeSelect>
        </div>
      )}
    </>
  )
}

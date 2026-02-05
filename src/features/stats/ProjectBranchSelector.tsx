/**
 * プロジェクト・ブランチ選択コンポーネント
 *
 * プロジェクトとブランチを選択するためのドロップダウン。
 */

import type { Project } from '@/lib/contracts/tauriCommands'
import { Label } from '@/components/ui/label'
import { NativeSelect, NativeSelectOption } from '@/components/ui/native-select'

/** プロジェクト・ブランチ選択のプロパティ */
export interface ProjectBranchSelectorProps {
  projects: Project[]
  selectedProject: Project | null
  onProjectChange: (e: React.ChangeEvent<HTMLSelectElement>) => void
  branches: Array<{ name: string }>
  selectedBranch: string
  onBranchChange: (e: React.ChangeEvent<HTMLSelectElement>) => void
}

/** プロジェクト・ブランチ選択コンポーネント */
export function ProjectBranchSelector({
  projects,
  selectedProject,
  onProjectChange,
  branches,
  selectedBranch,
  onBranchChange,
}: ProjectBranchSelectorProps) {
  return (
    <>
      <div className="flex items-center gap-2">
        <Label>プロジェクト:</Label>
        <NativeSelect
          value={selectedProject?.projectId.toString() || ''}
          onChange={onProjectChange}
          size="sm"
          className="min-w-[200px]"
        >
          <NativeSelectOption value="">選択してください</NativeSelectOption>
          {projects.map((project) => (
            <NativeSelectOption key={project.projectId} value={project.projectId.toString()}>
              {project.name}
            </NativeSelectOption>
          ))}
        </NativeSelect>
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

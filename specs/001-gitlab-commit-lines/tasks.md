---
description: 'Task list for feature implementation'
---

# Tasks: GitLab 月次コミット行数分析

**Input**: specs/001-gitlab-commit-lines/plan.md, specs/001-gitlab-commit-lines/spec.md, specs/001-gitlab-commit-lines/research.md, specs/001-gitlab-commit-lines/data-model.md, specs/001-gitlab-commit-lines/contracts/tauri-commands.openapi.yaml

**Tech Stack**: TypeScript 5.6 / React 18 / Vite 6 / Vitest, Rust 2021 / Tauri 2 / cargo test, Bun

**Testing Policy**: plan.md の Constitution（原則 II）に従い、追加/変更ロジックには Vitest / cargo test を付ける。

## Format: `- [ ] T### [P?] [US?] Description with file path`

- **[P]**: 並列実行可能（異なるファイルで競合しない、かつ未完了タスクに依存しない）
- **[US#]**: ユーザーストーリーに紐づくタスクのみ付与（Setup/Foundational/Polish には付けない）
- **File path**: すべてのタスクに、対象ファイルパスを必ず含める

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: 開発・実装の土台を整え、以降のタスクが迷わず実行できる状態にする。

- [x] T001 仕様ドキュメントの参照先を揃える README.md
- [x] T002 Rust 側のモジュール配置を作成する src-tauri/src/commands/mod.rs
- [x] T003 [P] TypeScript 側の feature ディレクトリ指針を追加する src/features/README.md
- [x] T004 [P] Tauri invoke の薄いラッパーを作成する src/lib/tauri.ts
- [x] T005 [P] OpenAPI 契約に対応する TS 型定義を追加する src/lib/contracts/tauriCommands.ts
- [x] T006 手動検証手順の章を quickstart に追記する specs/001-gitlab-commit-lines/quickstart.md

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: 全ユーザーストーリーで共通利用する「エラー/ログ/永続化/HTTP クライアント」の基盤を実装する。

**Checkpoint**: この Phase 完了後に US1/US2/US3 を実装開始できる。

- [x] T007 Rust の依存を追加する（HTTP/日時/エラー） src-tauri/Cargo.toml
- [x] T008 [P] アプリ内エラー型を定義する src-tauri/src/error.rs
- [x] T009 [P] セキュアログ方針（トークン/メール非出力）をユーティリティ化する src-tauri/src/logging.rs
- [x] T010 アプリデータディレクトリ解決ヘルパーを追加する src-tauri/src/paths.rs
- [x] T011 [P] 永続化スキーマ（schemaVersion=1）を定義する src-tauri/src/storage/schema.rs
- [x] T012 [P] JSON ストア（read/write/atomic write）を実装する src-tauri/src/storage/json_store.rs
- [x] T013 [P] ストレージ抽象（Repository 風）を定義する src-tauri/src/storage/repository.rs
- [x] T014 JSON 保存フォーマットのルート型を実装する src-tauri/src/storage/model.rs
- [x] T015 [P] GitLab REST API クライアントの基盤（ベースURL/認証ヘッダ）を実装する src-tauri/src/gitlab/client.rs
- [x] T016 [P] GitLab API レスポンス型（Project/Branch/Commit）を定義する src-tauri/src/gitlab/types.rs
- [x] T017 [P] ストレージ基盤テスト（JSON ストアの read/write）を追加する src-tauri/src/storage/json_store.rs
- [x] T018 [P] ログの安全性テスト（トークン/メールが出ない）を追加する src-tauri/src/logging.rs
- [x] T069 [P] フロントのテスト環境（jsdom）を整備する vitest.config.ts
- [x] T070 [P] フロントのテスト依存（Testing Library / jest-dom / jsdom）を追加する package.json
- [x] T071 [P] テストセットアップ（jest-dom 読み込み）を追加する src/test/setup.ts

---

## Phase 3: User Story 1 - GitLab 接続設定とプロジェクト同期 (Priority: P1) 🎯 MVP

**Goal**: GitLab のベースURL/アクセストークンを登録し、アクセス可能なプロジェクト一覧を同期して UI に表示できる。

**Independent Test**: 有効な URL/トークンを入力→保存→プロジェクト同期で一覧が表示される。無効なトークンの場合、ユーザーが取るべき行動が分かるエラーを表示する。

### Tests for User Story 1

- [x] T019 [P] [US1] GitLabConnection のバリデーションをテストする src-tauri/src/domain/gitlab_connection.rs
- [x] T020 [P] [US1] connection の保存/読み込みをテストする src-tauri/src/storage/connection_repository.rs
- [x] T021 [P] [US1] フロントの接続フォームのバリデーションをテストする src/features/gitlabConnection/ConnectionForm.test.tsx（depends on T069, T070, T071）

### Implementation for User Story 1

- [x] T022 [P] [US1] 接続設定エンティティを実装する src-tauri/src/domain/gitlab_connection.rs
- [x] T023 [P] [US1] 接続設定リポジトリを実装する src-tauri/src/storage/connection_repository.rs
- [x] T024 [P] [US1] Project エンティティを実装する src-tauri/src/domain/project.rs
- [x] T025 [P] [US1] Project リポジトリを実装する src-tauri/src/storage/project_repository.rs
- [x] T026 [P] [US1] コマンド: 接続設定取得（トークン非返却）を実装する src-tauri/src/commands/gitlab_connection_get.rs
- [x] T027 [P] [US1] コマンド: 接続設定更新（トークン保存）を実装する src-tauri/src/commands/gitlab_connection_set.rs
- [x] T028 [P] [US1] コマンド: プロジェクト同期を実装する src-tauri/src/commands/projects_sync.rs
- [x] T029 [US1] invoke handler へコマンド登録を行う src-tauri/src/lib.rs
- [x] T030 [P] [US1] 画面: 接続設定フォームを実装する src/features/gitlabConnection/ConnectionForm.tsx
- [x] T031 [P] [US1] 画面: プロジェクト同期/一覧表示を実装する src/features/projects/ProjectsPanel.tsx
- [x] T032 [US1] 画面遷移/レイアウトの骨格を作る src/App.tsx
- [x] T033 [US1] トークンが UI とコンソールに出ないことを手動確認手順に追記する specs/001-gitlab-commit-lines/quickstart.md

**Checkpoint**: US1 のみで「接続設定 → プロジェクト同期 → 一覧表示」が成立する。

---

## Phase 4: User Story 2 - コミット収集と保存 (Priority: P2)

**Goal**: プロジェクト/ブランチ/期間でコミットを収集し、重複を作らずローカル JSON に保存できる（stats 欠損は 0 扱い + 欠損件数カウント）。

**Independent Test**: 任意のプロジェクト/ブランチを選択し、期間指定あり/なしで収集→保存件数が増える。再収集しても重複が増えない。障害時も途中まで保存が保持される。

### Tests for User Story 2

- [x] T034 [P] [US2] コミットの一意制約（projectId+branch+sha）をテストする src-tauri/src/storage/commit_repository.rs
- [x] T035 [P] [US2] stats 欠損が 0 扱いになることをテストする src-tauri/src/domain/commit.rs

### Implementation for User Story 2

- [x] T036 [P] [US2] Branch エンティティを実装する src-tauri/src/domain/branch.rs
- [x] T037 [P] [US2] Commit エンティティ（statsMissing を含む）を実装する src-tauri/src/domain/commit.rs
- [x] T038 [P] [US2] Commit リポジトリ（upsert/重複スキップ）を実装する src-tauri/src/storage/commit_repository.rs
- [x] T039 [P] [US2] GitLab API: ブランチ一覧取得を実装する src-tauri/src/gitlab/branches.rs
- [x] T040 [P] [US2] GitLab API: コミット一覧取得（ページング、with_stats、since/until）を実装する src-tauri/src/gitlab/commits.rs
- [x] T041 [US2] コマンド: ブランチ一覧取得を実装する src-tauri/src/commands/branches_list.rs
- [x] T042 [US2] コマンド: コミット収集（保存/結果集計）を実装する src-tauri/src/commands/commits_collect.rs
- [x] T043 [US2] コマンドのエラーメッセージ整形（再入力/権限確認など）を実装する src-tauri/src/error.rs
- [x] T044 [P] [US2] 画面: 収集条件フォーム（プロジェクト/ブランチ/期間）を実装する src/features/collect/CollectForm.tsx
- [x] T045 [P] [US2] 画面: 収集結果（inserted/skipped/missingStats）表示を実装する src/features/collect/CollectResult.tsx
- [x] T046 [US2] App 統合（US1 のプロジェクト選択と連携）を行う src/App.tsx

**Checkpoint**: US2 で「ブランチ選択 → 収集 → 保存 → 再収集で重複なし」が確認できる。

---

## Phase 5: User Story 3 - 月次コミット行数の集計を閲覧 (Priority: P3)

**Goal**: 保存済みコミットから、年/月/ユーザー単位で（追加+削除）を集計し、プロジェクト別ビュー/横断ビューでグラフと表に表示できる（欠損件数も表示）。

**Independent Test**: 小さな保存済みデータセットで、フィルタ変更に応じてグラフ/表が期待通りの数値になる。メールは画面に出ない。

### Tests for User Story 3

- [x] T047 [P] [US3] 月判定が UTC 基準であることをテストする src-tauri/src/stats/monthly_project_view.rs
- [x] T048 [P] [US3] userKey（authorEmail 優先、なければ authorName）が期待通りになることをテストする src-tauri/src/stats/types.rs
- [x] T049 [P] [US3] 集計結果の missingCount が期待通りになることをテストする src-tauri/src/stats/monthly_cross_view.rs

### Implementation for User Story 3

- [x] T050 [P] [US3] 月次集計 DTO（MonthlyStatsResponse）を実装する src-tauri/src/stats/types.rs
- [x] T051 [P] [US3] 月次集計ロジック（project-view）を実装する src-tauri/src/stats/monthly_project_view.rs
- [x] T052 [P] [US3] 月次集計ロジック（cross-view）を実装する src-tauri/src/stats/monthly_cross_view.rs
- [x] T053 [US3] コマンド: 月次集計（project-view）を実装する src-tauri/src/commands/stats_monthly_project_view.rs
- [x] T054 [US3] コマンド: 月次集計（cross-view）を実装する src-tauri/src/commands/stats_monthly_cross_view.rs
- [x] T055 [P] [US3] 画面: フィルタ UI（年/ユーザー/プロジェクト/ブランチ）を実装する src/features/stats/StatsFilters.tsx
- [x] T056 [P] [US3] 画面: Recharts 集合縦棒グラフを実装する src/features/stats/MonthlyBarChart.tsx
- [x] T057 [P] [US3] 画面: 月×ユーザーの表を実装する src/features/stats/MonthlyTable.tsx
- [x] T058 [P] [US3] 画面: 欠損件数表示（ユーザー別/月別）を実装する src/features/stats/MissingStatsNotice.tsx
- [x] T059 [US3] App 統合（Project view / Cross view の切替）を行う src/App.tsx
- [x] T060 [US3] 2 秒以内更新の手動検証手順を追記する specs/001-gitlab-commit-lines/quickstart.md

**Checkpoint**: US3 で「フィルタ変更→2秒以内にグラフ/表更新」が成立する。

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: 複数ストーリーに跨る品質（UX/安全性/保守性）を仕上げる。

- [x] T061 [P] UI 状態（loading/empty/error/success）を共通化する src/features/ui/useAsyncState.ts
- [x] T062 [P] エラーメッセージのガイド（次の行動が分かる）を整備する src/features/ui/ErrorAlert.tsx
- [x] T063 セキュリティ監査チェックリストを追加する specs/001-gitlab-commit-lines/checklists/security.md
- [x] T064 [P] 新規/変更シンボル（クラス/関数/プロパティ/定数）へ日本語説明コメントを追加する（`export` は優先対象） src/lib/tauri.ts
- [x] T065 [P] 新規/変更シンボル（クラス/関数/フィールド/定数）へ日本語説明コメントを追加する（`pub` は優先対象） src-tauri/src/lib.rs
- [x] T066 収集・集計の入力バリデーション（開始>終了など）を追加する src/features/collect/CollectForm.tsx
- [x] T067 全体のスモークテスト手順を quickstart にまとめる specs/001-gitlab-commit-lines/quickstart.md
- [x] T068 CI でテストを実行する workflow を追加する .github/workflows/test.yml

---

## Dependencies & Execution Order

### User Story Dependencies

- US1 (P1) は単独で MVP。US2/US3 の前提（接続設定・プロジェクト同期）。
- US2 (P2) は US1 に依存（プロジェクト/ブランチ選択と認証が必要）。
- US3 (P3) は US2 に依存（保存済みコミットが必要）。

### Dependency Graph

- Phase 1 → Phase 2 → US1 → US2 → US3 → Phase 6

### Phase Order

1. Phase 1: Setup
2. Phase 2: Foundational（完了が全ストーリーの開始条件）
3. Phase 3: US1（MVP）
4. Phase 4: US2
5. Phase 5: US3
6. Phase 6: Polish

---

## Parallel Execution Examples

### US1 Parallel Example

- Backend（並列）: T022, T023, T024, T025, T026, T027, T028 を担当する src-tauri/src/
- Frontend（並列）: T030, T031 を担当する src/features/

### US2 Parallel Example

- Backend（並列）: T036, T037, T039, T040 を担当する src-tauri/src/
- Frontend（並列）: T044, T045 を担当する src/features/

### US3 Parallel Example

- Backend（並列）: T051, T052 を担当する src-tauri/src/
- Frontend（並列）: T056, T057, T058 を担当する src/features/

---

## Implementation Strategy

### MVP First (US1 Only)

1. Phase 1 → Phase 2 → Phase 3（US1）まで実装する specs/001-gitlab-commit-lines/tasks.md
2. quickstart の手順で「接続設定 → プロジェクト同期」が成立することを確認する specs/001-gitlab-commit-lines/quickstart.md

### Incremental Delivery

- US1 の後に US2（収集）を追加し、最後に US3（集計閲覧）を追加する specs/001-gitlab-commit-lines/tasks.md
- 各ストーリー完了時に quickstart の手順で手動検証できる状態を保つ specs/001-gitlab-commit-lines/quickstart.md

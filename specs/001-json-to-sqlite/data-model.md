# Data Model: SQLiteストレージ

## Entities

### connections

- Purpose: GitLab 接続設定の永続化（accessToken は SQLite に保存）
- Fields:
  - id: integer (PK, 固定値 1 を想定)
  - base_url: text (not null)
  - author_email: text (nullable)
  - access_token: text (not null)
  - updated_at_utc: text (ISO-8601)

### projects

- Purpose: GitLab プロジェクト情報の永続化
- Fields:
  - project_id: integer (PK)
  - name: text (not null)
  - path_with_namespace: text (not null)
  - web_url: text (not null)
  - last_sync_time_utc: text (nullable)

### commits

- Purpose: コミット情報の永続化
- Fields:
  - project_id: integer (FK -> projects.project_id)
  - branch_name: text (not null)
  - sha: text (not null)
  - author_name: text (not null)
  - author_email: text (not null)
  - committed_date_utc: text (ISO-8601, not null)
  - additions: integer (not null)
  - deletions: integer (not null)
- Constraints:
  - Unique: (project_id, branch_name, sha)

### schema_migrations

- Purpose: スキーマバージョニング
- Fields:
  - version: integer (PK)
  - applied_at_utc: text (ISO-8601)

### user_filters

- Purpose: ユーザーフィルタの選択状態を永続化
- Fields:
  - view_type: text (not null, enum: project-view | cross-view)
  - context_key: text (not null)
  - selected_users_json: text (not null)
  - updated_at_utc: text (ISO-8601)
- Constraints:
  - Unique: (view_type, context_key)

## Relationships

- projects 1 --- \* commits

## Indexes

- commits(project_id)
- commits(branch_name)
- commits(committed_date_utc)
- commits(author_email)
- commits(project_id, branch_name, committed_date_utc)
- user_filters(view_type, context_key)

## Validation Rules

- base_url は URL 形式で保存する（既存バリデーションに準拠）。
- committed_date_utc は UTC で統一し、月次集計の精度を担保する。
- additions/deletions は 0 以上。

## State Transitions

- schema_migrations はバージョン番号の単調増加のみ許可。

-- SQLite スキーマ初期化（バージョン 1）
-- 
-- テーブル構成：
-- - connections: GitLab 接続設定（1レコード）
-- - projects: GitLab プロジェクト情報
-- - commits: コミット情報（複合ユニーク制約）
-- - user_filters: ユーザーフィルタの選択状態

-- connections テーブル
CREATE TABLE IF NOT EXISTS connections (
    id INTEGER PRIMARY KEY DEFAULT 1,
    base_url TEXT NOT NULL,
    author_email TEXT,
    access_token TEXT NOT NULL,
    updated_at_utc TEXT NOT NULL
);

-- projects テーブル
CREATE TABLE IF NOT EXISTS projects (
    project_id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    path_with_namespace TEXT NOT NULL,
    web_url TEXT NOT NULL,
    last_sync_time_utc TEXT
);

-- commits テーブル（複合ユニーク制約：project_id, branch_name, sha）
CREATE TABLE IF NOT EXISTS commits (
    project_id INTEGER NOT NULL,
    branch_name TEXT NOT NULL,
    sha TEXT NOT NULL,
    author_name TEXT NOT NULL,
    author_email TEXT NOT NULL,
    committed_date_utc TEXT NOT NULL,
    additions INTEGER NOT NULL,
    deletions INTEGER NOT NULL,
    PRIMARY KEY (project_id, branch_name, sha),
    FOREIGN KEY (project_id) REFERENCES projects(project_id) ON DELETE CASCADE
);

-- user_filters テーブル
CREATE TABLE IF NOT EXISTS user_filters (
    view_type TEXT NOT NULL,
    context_key TEXT NOT NULL,
    selected_users_json TEXT NOT NULL,
    updated_at_utc TEXT NOT NULL,
    PRIMARY KEY (view_type, context_key)
);

-- schema_migrations テーブルは migrations.rs で管理される

-- commits テーブルのインデックス
CREATE INDEX IF NOT EXISTS idx_commits_project_id 
    ON commits(project_id);

CREATE INDEX IF NOT EXISTS idx_commits_branch_name 
    ON commits(branch_name);

CREATE INDEX IF NOT EXISTS idx_commits_committed_date_utc 
    ON commits(committed_date_utc);

CREATE INDEX IF NOT EXISTS idx_commits_author_email 
    ON commits(author_email);

-- 複合インデックス（期間フィルタ＋プロジェクトフィルタの最適化）
CREATE INDEX IF NOT EXISTS idx_commits_project_date 
    ON commits(project_id, committed_date_utc);

-- user_filters テーブルのインデックス
CREATE INDEX IF NOT EXISTS idx_user_filters_view_context 
    ON user_filters(view_type, context_key);

-- テーブル統計情報の更新
ANALYZE;

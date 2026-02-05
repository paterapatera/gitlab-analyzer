# Feature Specification: SQLiteストレージへの移行

**Feature Branch**: `001-json-to-sqlite`  
**Created**: 2026-02-06  
**Status**: Draft  
**Input**: User description: "システムとして、データをJSONに保存するのではなく、SQLiteに保存するようにしたい。なぜなら、膨大な量のコミットを扱うため。"

## User Scenarios & Testing _(mandatory)_

### User Story 1 - SQLiteストレージレイヤーの実装 (Priority: P1)

システムは、既存のJSONファイルベースストレージの代わりにSQLiteデータベースを使用してデータを保存・取得でき、すべての既存機能（接続設定、プロジェクト管理、コミット収集）が正常に動作する。

**Why this priority**: 新しいストレージシステムの基盤がなければ、既存機能の継続利用や今後の拡張が不可能なため。

**Independent Test**: 開発環境で新規インストールを実行し、GitLab接続設定→プロジェクト同期→コミット収集→統計表示のフローが完全に動作することを確認できる。JSONファイルは作成されず、SQLiteデータベースファイルのみが作成される。

**Acceptance Scenarios**:

1. **Given** アプリケーションが初回起動される、**When** ユーザーがGitLab接続を設定する、**Then** 接続情報がSQLiteデータベースに保存され、アプリ再起動後も利用できる。
2. **Given** SQLiteストレージが有効、**When** プロジェクト同期を実行する、**Then** プロジェクト一覧がSQLiteデータベースに保存され、一覧表示で参照できる。
3. **Given** SQLiteストレージが有効、**When** コミット収集を実行する、**Then** コミットデータがSQLiteデータベースに保存され、重複チェックが正しく機能する。
4. **Given** 大量のコミットデータ（10万件以上）が保存されている、**When** 月次統計を表示する、**Then** クエリが3秒以内に完了し、正確な集計結果が表示される。

---

### User Story 2 - ストレージパフォーマンスの最適化とモニタリング (Priority: P2)

システムは、適切なインデックスとクエリ最適化により、大量のコミットデータ（100万件以上）に対しても、検索・集計操作が実用的な速度（5秒以内）で完了する。

**Why this priority**: 基本機能が動作すれば最低限の価値は提供できるため、パフォーマンス最適化は後回しにできる。ただし、「膨大な量のコミットを扱う」という要件を満たすには必須。

**Independent Test**: 100万件のテストコミットデータを生成し、各種クエリ（期間フィルタ、ユーザーフィルタ、月次集計）の実行時間を計測して、すべて5秒以内に完了することを確認できる。

**Acceptance Scenarios**:

1. **Given** 100万件のコミットデータが保存されている、**When** プロジェクト/ブランチでフィルタリングする、**Then** 2秒以内に結果が表示される。
2. **Given** 100万件のコミットデータが保存されている、**When** 月次集計クエリを実行する、**Then** 3秒以内に集計結果が返される。
3. **Given** システムが稼働している、**When** データベースサイズが一定の閾値（例：500MB）を超える、**Then** ユーザーに通知が表示され、データメンテナンスのオプション（古いデータの削除など）が提示される。
4. **Given** 大量データ操作中、**When** ユーザーがキャンセルを要求する、**Then** トランザクションがロールバックされ、データの整合性が保たれる。

---

### Edge Cases

- ディスク容量が不足している場合（保存前にサイズをチェックし、十分な空き容量がない場合は警告を表示）。
- 並行書き込み（複数タブ/インスタンス）の場合（SQLiteのロックメカニズムを活用し、適切なエラーハンドリングを実装）。
- トランザクション途中でアプリケーションがクラッシュした場合（SQLiteの自動ロールバック機能により、データ整合性が保たれる）。

## Requirements _(mandatory)_

### Functional Requirements

- **FR-001**: システムはSQLiteデータベースを使用してGitLab接続設定、プロジェクト情報、コミットデータを永続化しなければならない。
- **FR-002**: システムはコミットの重複を防ぐため、適切なユニーク制約（プロジェクトID、ブランチ名、コミットSHA）を実装しなければならない。
- **FR-003**: システムはトランザクションを使用して、複数の関連する操作（例：コミット一括保存）のアトミック性を保証しなければならない。
- **FR-004**: システムは頻繁に使用されるクエリパス（プロジェクト/ブランチフィルタ、日付範囲、ユーザーフィルタ）に対して適切なインデックスを作成しなければならない。
- **FR-005**: システムはSQLiteデータベースファイルの場所をプラットフォーム標準のデータディレクトリに配置しなければならない（既存のget_data_file_path関数と同様）。
- **FR-006**: システムは既存のすべてのTauriコマンド（gitlab_connection_set、projects_list、commits_collect など）でSQLiteストレージを使用しなければならない。
- **FR-007**: システムはSQLiteデータベースのスキーマバージョンを管理し、将来のスキーマ変更に対応できる仕組みを提供しなければならない。

### Key Entities _(include if feature involves data)_

- **GitLabConnection**: GitLab接続設定（base_url、access_token、author_email）。SQLiteテーブル：`connections`（1レコードのみ想定）。
- **Project**: GitLabプロジェクト情報（project_id、name、web_url、last_sync_time）。SQLiteテーブル：`projects`。ユニーク制約：project_id。
- **Commit**: コミット情報（sha、project_id、branch_name、author_name、author_email、committed_date、additions、deletions）。SQLiteテーブル：`commits`。複合ユニーク制約：(project_id, branch_name, sha)。インデックス：committed_date、author_email、project_id、branch_name。

## Success Criteria _(mandatory)_

### Measurable Outcomes

- **SC-001**: 新規インストールで、すべてのデータがJSONファイルではなくSQLiteデータベースに保存される（JSONファイル作成数：0件）。
- **SC-002**: 10万件のコミットデータに対する月次集計クエリが3秒以内に完了する（JSON版と比較して50%以上の性能向上）。
- **SC-003**: 100万件のコミットデータに対する検索・集計操作が5秒以内に完了する。
- **SC-004**: データベースファイルサイズがJSON形式と比較して30%以上削減される（同じデータ量で比較）。
- **SC-005**: すべての既存機能テストが新しいSQLiteストレージで成功する（回帰テスト合格率：100%）。

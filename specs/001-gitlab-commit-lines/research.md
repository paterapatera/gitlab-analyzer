# Research: GitLab 月次コミット行数分析

## 決定事項（技術選定）

### GitLab API との通信方式

- Decision: Rust 側（Tauri command）から GitLab REST API v4 を呼び出す
- Rationale: アクセストークンをフロントエンドに渡さずに済み、ログ/UI 露出リスクを最小化できる。収集・永続化・集計ロジックを Rust に集約できる。
- Alternatives considered:
  - フロントエンド（React）から直接 GitLab API を呼ぶ（CORS/トークン露出リスクが上がる）
  - GitLab GraphQL API を使う（学習コスト/必要データ取得の複雑化の可能性）

### コミット行数（追加/削除）の取得

- Decision: GitLab API の commit stats を利用し、取得不能な場合は 0 として集計しつつ欠損件数を記録する
- Rationale: 要件（欠損は 0 扱い + 欠損件数を UI 表示）に合致。データ収集の停止を避け、集計の一貫性を保つ。
- Alternatives considered:
  - stats 取得失敗をエラーとして扱い収集を中断（UX 悪化、要件不一致）
  - 差分 API から自前集計（API 呼び出し増、実装/パフォーマンスコスト増）

### ユーザー同一人物判定（集計キー）

- Decision: `authorEmail` を優先し、無い場合は `authorName` を利用する（キーは UI/ログに出さない）
- Rationale: 仕様の Clarifications/FR-021 に合致。
- Alternatives considered:
  - GitLab の userId をキーにする（コミット API から常に取得できない可能性）
  - 表示名ベースのみ（同名衝突が増える）

### 月次集計の月判定

- Decision: `committedDate` を UTC として月判定する
- Rationale: FR-024 / Clarifications に合致。
- Alternatives considered:
  - ローカルタイムゾーンで月判定（端末依存で再現性が落ちる）

### 永続ストレージ方式

- Decision: まずは「端末ローカルのアプリデータディレクトリに JSON ファイル保存」を採用する
- Rationale: 依存追加なしで実装可能（現状 Rust 側依存が `serde/serde_json` のみ）。個人利用前提の要件にも合致。将来 SQLite へ移行可能。
- Alternatives considered:
  - SQLite（検索/集計効率は高いが依存追加とマイグレーションが必要）
  - OS Keychain/Stronghold による暗号化保存（Out of Scope）

### UI グラフ

- Decision: Recharts の集合縦棒グラフで「x=月(1-12), series=ユーザー, y=追加+削除」を表示
- Rationale: 仕様の Clarifications/FR-014 に合致。既に `recharts` が導入済み。
- Alternatives considered:
  - 独自 SVG（工数増、UX 一貫性低下）

## 実装上のベストプラクティス

### GitLab API 呼び出し

- Decision: `per_page=100` 等のページネーションを前提に実装し、進捗表示・中断/再開しやすい処理単位に分割する
- Rationale: 大量データ時の UX/安定性を確保。
- Alternatives considered:
  - 1回で全件取得（タイムアウト/メモリ負荷/再実行しづらい）

### 使用する GitLab REST API v4 エンドポイント（確定）

- Decision: 以下を最小セットとして実装する
  - プロジェクト一覧: `GET /projects?membership=true&per_page=100&page=N`
  - ブランチ一覧: `GET /projects/:id/repository/branches`（必要なら `search` で絞り込み）
  - コミット一覧（期間・ブランチ指定）: `GET /projects/:id/repository/commits?ref_name=<branch>&since=<ISO8601>&until=<ISO8601>&with_stats=true&per_page=100&page=N`
- Rationale: 公式 Commits/Branches/Projects API が `since`/`until`（ISO8601）および `with_stats` をサポートしており、追加/削除（stats）を 1 回の一覧取得で回収できる。
- Alternatives considered:
  - `GET /projects/:id/repository/commits/:sha` をコミットごとに呼ぶ（API 呼び出しが増え、収集が遅くなる）

### 日付パラメータの形式（確定）

- Decision: `since` / `until` は ISO 8601（`YYYY-MM-DDTHH:MM:SSZ`）の UTC で送る
- Rationale: GitLab API の仕様に一致し、FR-024（UTC 集計）と整合する。
- Alternatives considered:
  - ローカルタイムゾーン文字列（環境依存で再現性が落ちる）

### ローカル保存スキーマのバージョニング（確定）

- Decision: 永続ストレージのルートに `schemaVersion: 1` を持たせ、将来の互換性変更に備える
- Rationale: JSON から SQLite 等へ移行する際のマイグレーション判断を容易にする。
- Alternatives considered:
  - バージョン無し（将来の変更時に破壊的になりやすい）

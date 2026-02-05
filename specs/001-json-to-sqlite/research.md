# Research: SQLiteストレージへの移行

## Decision 1: SQLite アクセスライブラリ

- Decision: `rusqlite` を採用し、同期 API + トランザクションで一括書き込みを行う。
- Rationale: Tauri コマンドは同期実行が主体であり、軽量な依存で SQLite への読み書きができる。バッチ挿入と WAL による性能改善が見込める。
- Alternatives considered: `sqlx`（コンパイル時チェックだが依存が重い）、`diesel`（学習コストと設定が大きい）。

## Decision 2: マイグレーション/スキーマ管理

- Decision: `schema_migrations` テーブル + SQL ファイル/文字列による段階的マイグレーションを導入する。
- Rationale: FR-007 のスキーマバージョニング要件を満たしつつ、将来の差分追加を簡潔に管理できる。
- Alternatives considered: `refinery` 等のマイグレーション専用クレート（導入コストが増えるため今回は見送り）。

## Decision 3: パフォーマンス設定

- Decision: WAL モード、有効なインデックス、バッチ挿入（トランザクション + prepared statements）を標準とする。
- Rationale: 10万〜100万件のコミットを 3〜5 秒で集計するために I/O とロック競合を抑える必要がある。
- Alternatives considered: 事前集計テーブル（MVP ではコストが高い）、外部 DB への移行（運用範囲外）。

## Decision 4: 機密情報の保存方法

- Decision: accessToken は SQLite に保存する（UI/ログには出力しない）。
- Rationale: 運用の単純化を優先し、セキュアストレージ依存を排除する。
- Alternatives considered: OS セキュアストレージ（環境依存が大きい）、SQLite 内の暗号化（鍵管理が必要）。

# Research: Delete Collected Branch Commits

## Decision 1: 物理削除（完全削除・復元不可）

- Decision: ブランチ単位の削除は物理削除で実施する。
- Rationale: 誤収集の即時是正を優先し、集計と保存の整合性を単純化できる。
- Alternatives considered: 論理削除（`deleted_at` で非表示）、短期バックアップ復元。

## Decision 2: 収集中の同一ブランチ削除はブロック

- Decision: 収集中の同一ブランチは削除不可として明確にブロックする。
- Rationale: 同時実行の競合や整合性破綻を避け、ユーザー意図の混乱を防ぐ。
- Alternatives considered: 収集中でも削除可、削除予約（収集完了後に自動実行）。

## Decision 3: 取り消し不可を明記した確認ダイアログ

- Decision: 削除実行前に「取り消し不可」を明示した確認ダイアログを表示する。
- Rationale: 不可逆操作の誤実行を抑止し、操作の理解を高める。
- Alternatives considered: 確認なし即時削除、入力必須の二段階確認。

## Decision 4: 権限制御なし

- Decision: 全ユーザーが削除を実行可能とする。
- Rationale: ローカル単一ユーザー利用前提のため権限制御は過剰。
- Alternatives considered: 管理者のみ、プロジェクトオーナーのみ。

## Decision 5: エクスポート機能は対象外

- Decision: エクスポート機能は存在しない前提で削除影響を扱う。
- Rationale: 現行プロダクトにエクスポート機能がないため。
- Alternatives considered: JSON/CSV/TSV エクスポートを追加。

## Decision 6: 影響計算と削除は Tauri コマンドで実施

- Decision: 影響件数の計算と削除処理は `src-tauri` 側のコマンドで実装する。
- Rationale: 収集データの永続化は SQLite にあり、フロントエンドは表示/操作に集中させるため。
- Alternatives considered: フロントエンドでの影響計算、HTTP サーバ経由の API 実装。

## Decision 7: 削除ボタンはゴミ箱アイコン

- Decision: 削除アクションはゴミ箱アイコンで表示する。
- Rationale: UI の意図を直感的に伝える。
- Alternatives considered: テキストのみ、別アイコン。

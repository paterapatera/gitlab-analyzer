# Research: 詳細データのコピー

## Decision 1: クリップボード形式

- Decision: TSV（タブ区切り、ヘッダー行あり）でコピー
- Rationale: スプレッドシート貼り付け時の列崩れが起きにくく、ヘッダーで内容を確認できる
- Alternatives considered: CSV（カンマ区切り）

## Decision 2: フィルタ/並び替え時の対象

- Decision: 画面に表示されている行・順序のみをコピー
- Rationale: 表示とコピーの一致でユーザーの検証が容易になる
- Alternatives considered: フィルタ/並び替え前の全行をコピー

## Decision 3: 0行時のコピー挙動

- Decision: 空のヘッダー行のみをコピー
- Rationale: 貼り付け先で列構成だけを維持でき、空コピーで誤解を避けられる
- Alternatives considered: コピーを実行しない

## Decision 4: コピー完了メッセージ

- Decision: 詳細データパネル上部に2秒表示
- Rationale: 操作の直後に視認でき、作業フローを妨げにくい
- Alternatives considered: 画面右下トースト表示、表示時間の長短調整

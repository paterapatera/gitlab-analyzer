# Data Model: 詳細データのコピー

## Entities

### Detail Data Panel

- **Description**: 集計表示の詳細データ表を表示するパネル
- **Key Attributes**:
  - `title`: パネル表示名（例: "詳細データ"）
  - `viewMode`: 表示モード（project / cross）
  - `tableHeaders`: 表のヘッダー配列
  - `tableRows`: 表示中の行データ配列

### Detail Data Row

- **Description**: 詳細データ表の1行（ユーザー単位の月次集計）
- **Key Attributes**:
  - `displayName`: 表示名（authorName）
  - `monthTotals`: 月別合計値の配列（monthsに対応）
  - `rowTotal`: 合計列の値

### Copy Payload

- **Description**: クリップボードに書き込むTSVデータ
- **Key Attributes**:
  - `headers`: string[]（ユーザー + 月別列 + 合計列）
  - `rows`: string[][]（表示順の行データ）
  - `format`: "tsv"
  - `includesHeader`: true

## Relationships

- Detail Data Panel 1..1 -> Detail Data Row 0..N
- Detail Data Panel 1..1 -> Copy Payload 0..1（コピー操作時に生成）

## Validation Rules

- `headers` の順序は表の表示順と一致する
- `rows` は現在のフィルタ/並び替えが適用された表示順を保持する
- `displayName` のみを使用し、authorEmailは含めない

## State Transitions

- Copy Idle -> Copy Requested -> Copy Succeeded (message 2s) -> Copy Idle
- Copy Idle -> Copy Requested -> Copy Succeeded (header-only when no rows) -> Copy Idle

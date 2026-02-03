//! リポジトリ抽象
//!
//! ストレージ操作の抽象インターフェース。
//! 将来的に SQLite 等へ移行する際の境界を提供する。

use crate::error::AppResult;

/// リポジトリトレイト（読み取り）
pub trait ReadRepository<T> {
    /// 全件取得
    fn find_all(&self) -> AppResult<Vec<T>>;
}

/// リポジトリトレイト（書き込み）
pub trait WriteRepository<T, ID> {
    /// 保存（挿入または更新）
    fn save(&mut self, entity: T) -> AppResult<()>;
    
    /// 削除
    fn delete(&mut self, id: &ID) -> AppResult<()>;
}

/// 一括操作リポジトリ
pub trait BulkRepository<T> {
    /// 一括挿入（重複はスキップ）
    fn bulk_upsert(&mut self, entities: Vec<T>) -> AppResult<BulkUpsertResult>;
}

/// 一括挿入結果
#[derive(Debug, Clone, Default)]
pub struct BulkUpsertResult {
    /// 新規挿入件数
    pub inserted: usize,
    /// 重複スキップ件数
    pub skipped: usize,
}

impl BulkUpsertResult {
    /// 合計処理件数
    pub fn total(&self) -> usize {
        self.inserted + self.skipped
    }
}

//! ストレージスキーマ定義
//!
//! 永続化データのスキーマバージョンを管理する。

use serde::{Deserialize, Serialize};

/// 現在のスキーマバージョン
pub const CURRENT_SCHEMA_VERSION: u32 = 6;

/// スキーマバージョン情報
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaVersion {
    /// スキーマバージョン番号
    pub version: u32,
}

impl Default for SchemaVersion {
    fn default() -> Self {
        Self {
            version: CURRENT_SCHEMA_VERSION,
        }
    }
}

impl SchemaVersion {
    /// 新規作成
    pub fn new() -> Self {
        Self::default()
    }

    /// 現在のバージョンと互換性があるか確認
    pub fn is_compatible(&self) -> bool {
        self.version == CURRENT_SCHEMA_VERSION
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_version() {
        let schema = SchemaVersion::default();
        assert_eq!(schema.version, CURRENT_SCHEMA_VERSION);
    }

    #[test]
    fn test_is_compatible() {
        let schema = SchemaVersion::new();
        assert!(schema.is_compatible());
    }
}

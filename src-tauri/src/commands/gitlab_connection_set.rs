//! コマンド: 接続設定更新
//!
//! GitLab 接続情報を登録/更新する。

use crate::domain::GitLabConnection;
use crate::error::AppResult;
use crate::logging::mask_sensitive;
use crate::storage::ConnectionRepository;
use serde::Deserialize;
use tracing::info;

/// 接続設定入力
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetConnectionInput {
    /// GitLab ベース URL
    pub base_url: String,
    /// アクセストークン
    pub access_token: String,
}

/// 接続設定を登録/更新
#[tauri::command]
pub fn set_gitlab_connection(input: SetConnectionInput) -> Result<(), String> {
    set_gitlab_connection_inner(input).map_err(|e| e.to_string())
}

fn set_gitlab_connection_inner(input: SetConnectionInput) -> AppResult<()> {
    info!(
        "接続設定を保存: base_url={}, token={}",
        input.base_url,
        mask_sensitive(&input.access_token)
    );

    let connection = GitLabConnection::new(&input.base_url, &input.access_token)?;
    ConnectionRepository::save(connection)?;

    info!("接続設定の保存完了");
    Ok(())
}

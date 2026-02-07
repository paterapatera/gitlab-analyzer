//! GitLab モジュール
//!
//! GitLab REST API との通信を担当する。

pub mod branches;
pub mod client;
pub mod commits;
pub mod types;

pub use client::*;
pub use types::*;
// NOTE: branches と commits は client の impl 拡張なので、client 経由で使用する

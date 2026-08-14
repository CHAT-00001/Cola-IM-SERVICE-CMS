// service/src/user/black/mod.rs
// 服务 - 可乐用户 - 黑名单 - 模块
// 2026/8/3 Created.

////////

pub mod add;
pub mod del;
pub mod get;
pub mod list;
pub mod manage;
pub mod state;

use anyhow::{Result, anyhow};
use repository::user::pg::black::get::UserBlackGetRepo;

////////

/// # [LIST SERVICE] - 列表黑名单服务
pub struct BlacklistService;

impl BlacklistService {
    ////////

    /// # 1. [SERVICE] - 获取用户的黑名单IDs
    /// * `uid` 用户ID
    /// * `offset` 分页偏移
    /// * `limit` 分页数量
    pub async fn get_black_ids(uid: i64, offset: i64, limit: i64) -> Result<Vec<i64>> {
        let ids = UserBlackGetRepo::find_black_ids_by_uid(uid, limit, offset)
            .await
            .map_err(|e| anyhow!("[BLACK SERVICE]: 查询黑名单IDs失败: {}", e))?;

        tracing::info!(
            "[BLACK SERVICE]: 黑名单查询成功, uid={}, count={}",
            uid,
            ids.len()
        );
        Ok(ids)
    }

    ////////
}

//////// END

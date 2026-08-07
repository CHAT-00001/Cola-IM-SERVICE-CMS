// repository/src/user/service/black/mod.rs
// 仓储中心 - USER - Service - 黑名单
// 2026/8/3 Created.

////////

mod manage;

use crate::user::pg::black_repo::UserBlackRepo;
use anyhow::{Result, anyhow};

////////

/// # [SERVICE] - 黑名单服务
pub struct BlacklistService;

impl BlacklistService {

    ////////

    /// # 1. [SERVICE] - 获取用户的黑名单IDs
    /// * `uid` 用户ID
    /// * `offset` 分页偏移
    /// * `limit` 分页数量
    pub async fn get_black_ids(
        uid: i64,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<i64>> {
        let ids = UserBlackRepo::find_black_ids_by_uid(uid, limit, offset)
            .await
            .map_err(|e| anyhow!("[BLACK SERVICE]: 查询黑名单IDs失败: {}", e))?;

        tracing::info!("[BLACK SERVICE]: 黑名单查询成功, uid={}, count={}", uid, ids.len());
        Ok(ids)
    }

    ////////

    /// # 2. [SERVICE] - 保存/取消拉黑记录
    /// * `uid` 当前用户
    /// * `user_id` 目标用户
    /// * `status` 1=拉黑, 0=取消
    pub async fn save_black_record(
        uid: i64,
        user_id: i64,
        remark: String,
        status: i16,
    ) -> Result<u64> {
        let rows = UserBlackRepo::save_black_record(uid, user_id, remark, status)
            .await
            .map_err(|e| anyhow!("[BLACK SERVICE]: 保存黑名单记录失败: {}", e))?;

        tracing::info!("[BLACK SERVICE]: 拉黑操作成功, uid={}, target={}, status={}", uid, user_id, status);
        Ok(rows)
    }

    ////////

    /// # 3. [SERVICE] - 获取被拉黑列表(谁拉黑了我)
    pub async fn get_blacker_ids(
        user_id: i64,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<i64>> {
        let ids = UserBlackRepo::find_blacker_ids_by_user_id(user_id, limit, offset)
            .await
            .map_err(|e| anyhow!("[BLACK SERVICE]: 查询被拉黑列表失败: {}", e))?;

        tracing::info!("[BLACK SERVICE]: 被拉黑查询成功, user_id={}, count={}", user_id, ids.len());
        Ok(ids)
    }
}

//////// END
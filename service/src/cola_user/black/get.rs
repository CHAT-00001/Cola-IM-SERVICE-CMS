// service/src/user/blackk/get.rs
// 👤 服务 - USER - 黑名单 - 获取
// 2026/8/3 14:28 Created.

////////

use anyhow::{Result, anyhow};
use repository::user::pg::black::get::UserBlackGetRepo;
use repository::pg_pool;

////////

/// # [GET SERVICE] -
/// * `desc`: `USER - 获取黑名单服务`
pub struct BlackGetService;

impl BlackGetService {
    //

    ////////

    /// # 1. [SERVICE] - 获取用户的黑名单IDs
    /// * `uid` 用户ID
    /// * `offset` 分页偏移
    /// * `limit` 分页数量
    pub async fn get_black_ids(
        uid: i64,    // 目标用户ID
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> Result<Vec<i64>> {
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

    ////////

    /// # 3. [SERVICE] - 获取被拉黑列表(谁拉黑了我)
    pub async fn get_blacker_ids(user_id: i64, offset: i64, limit: i64) -> Result<Vec<i64>> {
        let ids = UserBlackGetRepo::find_blacker_ids_by_user_id(user_id, limit, offset)
            .await
            .map_err(|e| anyhow!("[BLACK SERVICE]: 查询被拉黑列表失败: {}", e))?;

        tracing::info!(
            "[BLACK SERVICE]: 被拉黑查询成功, user_id={}, count={}",
            user_id,
            ids.len()
        );
        Ok(ids)
    }
}

//////// END

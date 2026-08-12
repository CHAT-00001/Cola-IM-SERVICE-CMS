// cola_user/src/case/follow/get.rs
// core - USER - case - 关注 - 获取
// 2026/8/3 12:18 Created.

////////

use anyhow::{Result, anyhow};
use cola_data::cola_user::info::user::UserInfo;
use port::app::ctx::AppContext;
use tracing::info;

////////

/// # [GET CASE]
/// * `desc`: 获取关注的用户列表 case
pub struct UserFollowGetCase;

impl UserFollowGetCase {
    //

    ////////

    /// # 1. [CASE] - 获取我的黑名单
    /// * `desc` 返回用户资料
    pub async fn case_get_follow_list(
        uid: i64,         // 操作者ID
        limit: i64,       // 数量
        offset: i64,      // 页码
        ctx: &AppContext, // 全局上下文
    ) -> Result<Vec<UserInfo>, anyhow::Error> {

        let user_id = uid;
        // 1. 先查关注IDs
        let ids = ctx
            .user
            .follow
            .get
            .get_he_follow_ids(user_id, limit, offset)
            .await
            .map_err(|e| anyhow!("[🤐 CASE]: ❌️ 获取关注的IDs失败: {}", e))?;

        // 2. 再拿ids查找用户资料
        let user_infos = ctx
            .user
            .user
            .get
            .batch_get_infos(ids)
            .await
            .map_err(|e| anyhow!("[🤐 CASE]: ❌️ 获取关注的用户列表失败: {}", e))?;

        info!(
            "[🗣️ CASE]: ✅️ 获取关注的用户列表成功, uid={}, count={}",
            uid,
            user_infos.len()
        );
        Ok(user_infos)
    }
}

//////// END

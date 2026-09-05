// user/src/case/black/list.rs -- USER - case - 黑名单 - 列表用例
// 2026/8/5 22:56 Created.

////////

use anyhow::{Result, anyhow};
use cola_data::cola_user::info::user::UserInfo;
use port::app::ctx::AppContext;
use tracing::info;

////////

/// # [LIST CASE] - 用户黑名单列表用例
pub struct UserBlackListCase;

impl UserBlackListCase {
    //

    ////////

    /// # 1. [CASE] - 用户主动的拉黑记录列表
    /// * `desc`: `获取用户的拉黑记录列表`
    pub async fn case_get_my_black_list(
        uid: i64,    // 操作者UID
        id: i64,     // 目标用户ID
        limit: i64,  // 数量
        offset: i64, // 页码
        ctx: &AppContext,
    ) -> Result<Vec<UserInfo>, anyhow::Error> {
        // ..
        // .. 目标用户ID 为操作者ID
        let user_id = uid;
        // 1. 先查黑名单的ids
        let _ids = ctx
            .user
            .black
            .get
            .get_black_ids(user_id, limit, offset)
            .await
            .map_err(|e| anyhow!("[CASE]: ❌️ 获取黑名单IDs失败: {}", e))?;

        // 2. 再拿ids查找用户资料
        let user_infos = ctx
            .user
            .profile
            .get
            .batch_get_infos(_ids)
            .await
            .map_err(|e| anyhow!("[CASE]: ❌️ 批量获取用户资料失败: {}", e))?;

        info!(
            "[CASE]: 黑名单查询成功, uid={}, count={}",
            uid,
            user_infos.len()
        );
        Ok(user_infos)
    }
}

//////// END

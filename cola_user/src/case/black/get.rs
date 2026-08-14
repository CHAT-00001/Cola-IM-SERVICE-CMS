// user/src/case/user/get.rs
// core - USER - case - user - 获取 用例
// 2026/8/3 12:18 Created.

////////

use anyhow::{Result, anyhow};
use cola_data::cola_user::info::user::UserInfo;
use port::app::ctx::AppContext;
use tracing::info;

////////

pub struct UserBlackGetCase;

impl UserBlackGetCase {
    //

    ////////

    /// # 1. [CASE] - 获取我的黑名单
    /// * `desc` 返回用户资料
    pub async fn case_get_black_list(
        uid: i64,
        offset: i64,
        limit: i64,
        ctx: &AppContext,
    ) -> Result<Vec<UserInfo>, anyhow::Error> {
        let user_id = uid;
        // 1. 先查黑名单的ids
        let user_ids = ctx
            .user
            .black
            .get
            .get_black_ids(user_id, limit, offset)
            .await
            .map_err(|e| anyhow!("[🤐 CASE]: ❌️ 获取黑名单IDs失败: {}", e))?;

        // 2. 再拿ids查找用户资料
        let user_infos = ctx
            .user
            .user
            .get
            .batch_get_infos(user_ids)
            .await
            .map_err(|e| anyhow!("[🤐 CASE]: ❌️ 批量获取用户资料失败: {}", e))?;

        info!(
            "[CASE]: 黑名单查询成功, uid={}, count={}",
            uid,
            user_infos.len()
        );
        Ok(user_infos)
    }
}

//////// END

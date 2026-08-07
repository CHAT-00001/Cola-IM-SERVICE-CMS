// cola_user/src/case/cola_user/get.rs
// core - USER - case - cola_user - 获取 用例
// 2026/8/3 12:18 Created.

////////

use anyhow::{Result, anyhow};
use cola_data::app::ctx::AppContext;
use cola_data::cola_user::info::user::UserInfo;
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

        // 1. 先查黑名单的ids
        let user_ids = ctx
            .user
            .black
            .get
            .get_my_black_ids(uid, uid, limit, offset)
            .await
            .map_err(|e| anyhow!("[CASE]: ❌️ 获取黑名单IDs失败: {}", e))?;

        // 2. 再拿ids查找用户资料
        let user_infos = ctx
            .user
            .info
            .batch_get_info(user_ids)
            .await
            .map_err(|e| anyhow!("[CASE]: ❌️ 批量获取用户资料失败: {}", e))?;

        info!("[CASE]: 黑名单查询成功, uid={}, count={}", uid, user_infos.len());
        Ok(user_infos)
    }
}

//////// END

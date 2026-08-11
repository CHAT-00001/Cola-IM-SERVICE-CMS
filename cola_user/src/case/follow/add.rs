// cola_user/src/case/follow/add.rs
// core - USER - case - follow - 关注/取关 用例
// 2026/6/10 07:00

////////

use anyhow::{Result, anyhow};
use port::ctx::AppContext;
use tracing::{info, warn};

////////

/// # [ADD CASE] - 关注/取关 用例
pub struct UserFollowAddCase;

impl UserFollowAddCase {
    //

    ////////

    /// # 1. [CASE] - 关注用户
    /// * `uid` 操作者ID
    /// * `target_id` 目标用户ID
    pub async fn case_add_follow(
        uid: i64,
        target_id: i64,
        ctx: &AppContext,
    ) -> Result<(), anyhow::Error> {
        ctx.user
            .follow
            .add
            .add_follow(uid, target_id)
            .await
            .map_err(|e| anyhow!("[CASE]: ❌️ 添加关注失败: {}", e))?;

        info!(
            "[CASE]: ✅️ 添加关注成功: uid={}, target_id={}",
            uid, target_id
        );
        Ok(())
    }

    ////////

    /// # 2. [CASE] - 取消关注
    /// * `uid` 操作者ID
    /// * `target_id` 目标用户ID
    pub async fn case_remove_follow(
        uid: i64,
        target_id: i64,
        ctx: &AppContext,
    ) -> Result<(), anyhow::Error> {
        ctx.user
            .follow
            .add
            .del_follow(uid, target_id)
            .await
            .map_err(|e| anyhow!("[CASE]: ❌️ 取消关注失败: {}", e))?;

        info!(
            "[CASE]: ✅️ 取消关注成功: uid={}, target_id={}",
            uid, target_id
        );
        Ok(())
    }

    ////////

    /// # 3. [CASE] - 检查关注状态
    /// * `uid` 操作者ID
    /// * `target_id` 目标用户ID
    pub async fn case_check_follow(
        uid: i64,
        target_id: i64,
        ctx: &AppContext,
    ) -> Result<bool, anyhow::Error> {
        //
        let user_id = target_id;
        let is_followed = ctx
            .user
            .follow
            .check
            .is_followed(uid, user_id)
            .await
            .map_err(|e| anyhow!("[CASE]: ❌️ 检查是否关注失败: {}", e))?;

        info!(
            "[🗣️ CASE]: ✅️ 是否关注查询成功: uid={}, target_id={}, is_followed={}",
            uid, target_id, is_followed
        );
        Ok(is_followed)
    }
}

//////// END

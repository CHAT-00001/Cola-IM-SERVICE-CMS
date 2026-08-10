// cola_user/case/black/check.rs
// 用户 - case - black - 检查
// 2026/8/5 21:29 Created.

////////

use anyhow::{Result, anyhow};
use port::ctx::AppContext;
use tracing::info;

////////

/// # [CHECK CASE] - 检查
/// * `desc`: `用户黑名单发布用例`
pub struct UserBlackCheckCase;

impl UserBlackCheckCase {
    //

    ////////

    /// # 1. [CASE] - 是否在黑名单
    /// * `desc`: `检查是否在黑名单`
    pub async fn case_check_black(
        uid: i64,         // 操作者ID
        id: i64,          // 目标用户ID
        ctx: &AppContext, // 全局上下文
    ) -> Result<bool, anyhow::Error> {
        // ..
        // ..
        // 🚧 Call Port..
        let is_black = ctx
            .user
            .black
            .check
            .is_blacked(uid, id)
            .await
            .map_err(|e| anyhow!("[🤐 CASE]: ❌️ 检查是否在黑名单失败: {}", e))?;

        info!(
            "[🗣️ CASE] - ✅️ 检查是否在黑名单成功: uid={}, target_id={},",
            uid, id
        );
        Ok(is_black)
    }
}

//////// END

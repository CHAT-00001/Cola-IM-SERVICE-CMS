// user/case/category/state.rs
// 用户 - case - 分类 - 状态
// 2026/8/4 01:01 Created.

////////

use anyhow::{Context, Result};
use cola_data::app::ctx::AppContext;
use tracing::info;

////////

/// # [STATE CASE] - 状态 用例
pub struct UserCategoryStateCase;

// 构造实现
impl UserCategoryStateCase {
    //

    ////////

    /// # 1. [CASE] - 检查
    /// * `desc` 检查分类状态是否正常
    pub async fn case_check_state(
        uid: i64,        // 操作者ID
        user_id: i64,    // 目标用户ID
        ctx: AppContext, // 全局上下文
    ) -> Result<bool, anyhow::Error> {

        // 1. 🗣️ CALL USER PORT.
        let is_black = ctx
            .user
            .category
            .check_state(uid, user_id)
            .await
            .map_err(|e| anyhow::anyhow!("[CASE]: ❌️ 检查黑名单状态失败: {}", e))?;

        info!("[CASE]: ✅️ 检查黑名单状态成功: uid={},", uid);

        Ok(is_black)
    }
}

//////// END

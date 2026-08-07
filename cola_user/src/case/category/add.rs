// user/case/category/add.rs
// 用户 - case - 分类 - 发布
// 2026/8/4 02:17 Created.

////////

use anyhow::{Result, anyhow};
use cola_data::app::ctx::AppContext;
use tracing::info;

////////

/// # [ADD CASE] - 用户 黑名单 添加/移除 用例
pub struct UserCategoryAddCase;

impl UserCategoryAddCase {
    //

    ////////

    /// # 1. [CASE] - 添加分类
    /// * `uid` 操作者
    /// * `id` 目标用户ID
    /// * `remark` 拉黑原因
    pub async fn case_add_category(
        uid: i64,         // 操作者ID
        id: i64,          // 目标用户ID
        ctx: &AppContext, // 全局上下文
    ) -> Result<(), anyhow::Error> {
        // 🚧 Call Port..
        ctx.user
            .category
            .add_following(uid, id)
            .await
            .map_err(|e| anyhow!("[🤐 CASE]: ❌️ 添加黑名单失败: {}", e))?;

        info!(
            "[🗣️ CASE] - ✅️ 添加黑名单成功: uid={}, target_id={},",
            uid, id
        );
        Ok(())
    }

    ////////

    /// # 2. [CASE] - 移除黑名单
    /// * `uid` 操作者
    /// * `id` 目标用户ID
    pub async fn case_del_category(
        uid: i64,         // 操作者ID
        id: i64,          // 目标ID
        ctx: &AppContext, // 全局上下文
    ) -> Result<(), anyhow::Error> {
        // 🚧 Call Port..
        ctx.user
            .category
            .single_del(uid, id)
            .await
            .map_err(|e| anyhow!("[🤐 CASE]: ❌️ 移除黑名单失败: {}", e))?;

        info!(
            "[🗣️ CASE] - ✅️ 移除黑名单成功: uid={}, target_id={}",
            uid, id
        );
        Ok(())
    }
}

//////// END

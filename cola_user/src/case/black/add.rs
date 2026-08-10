// cola_user/src/case/black/add.rs
// 用户 - case - black - 发布用例
// 2026/8/2 22:51 Created.

////////

use anyhow::{Result, anyhow};
use port::ctx::AppContext;
use tracing::info;

////////

/// # [ADD CASE] - 发布
/// * `desc`: `用户黑名单发布用例`
pub struct UserBlackAddCase;

impl UserBlackAddCase {
    //

    ////////

    /// # 1. [CASE] - 添加黑名单
    /// * `uid` 操作者
    /// * `id` 目标用户ID
    /// * `remark` 拉黑原因
    pub async fn case_add_black(
        uid: i64,         // 操作者ID
        id: i64,          // 目标用户ID
        ctx: &AppContext, // 全局上下文
    ) -> Result<(), anyhow::Error> {
        // 🚧 Call Port..
        ctx.user
            .black
            .add
            .add_black(uid, id)
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
    pub async fn case_del_black(
        uid: i64,         // 操作者ID
        id: i64,          // 目标ID
        ctx: &AppContext, // 全局上下文
    ) -> Result<(), anyhow::Error> {
        // 🚧 Call Port..
        ctx.user
            .black
            .del
            .single_soft_del(uid, id)
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

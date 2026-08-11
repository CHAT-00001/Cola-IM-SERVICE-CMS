// cola_user/case/category/add.rs
// 用户 - case - 分类 - 发布
// 2026/8/4 02:17 Created.

////////

use anyhow::{Result, anyhow};
use cola_data::cola_user::command::category::UserCategoryCommand;
use port::ctx::AppContext;
use tracing::info;

////////

/// # [ADD CASE] - 用户 分类 添加/删除 用例
pub struct UserCategoryAddCase;

impl UserCategoryAddCase {
    //

    ////////

    /// # 1. [CASE] - 添加分类
    /// * `uid` 操作者
    /// * `id` 目标分类ID
    pub async fn case_add_category(
        uid: i64,         // 操作者ID
        id: i64,          // 目标分类ID
        ctx: &AppContext, // 全局上下文
    ) -> Result<(), anyhow::Error> {
        // 🚧 Call Port..
        let mut cmd = UserCategoryCommand::new();
        cmd.name = Some(id.to_string());
        ctx.user
            .category
            .add
            .add_new_one(uid, cmd)
            .await
            .map_err(|e| anyhow!("[🤐 CASE]: ❌️ 添加分类失败: {}", e))?;

        info!(
            "[🗣️ CASE] - ✅️ 添加分类成功: uid={}, target_id={},",
            uid, id
        );
        Ok(())
    }

    ////////

    /// # 2. [CASE] - 删除分类
    /// * `uid` 操作者
    /// * `id` 目标分类ID
    pub async fn case_del_category(
        uid: i64,         // 操作者ID
        id: i64,          // 目标ID
        ctx: &AppContext, // 全局上下文
    ) -> Result<(), anyhow::Error> {
        // 🚧 Call Port..
        ctx.user
            .category
            .delete
            .batch_delete(vec![id])
            .await
            .map_err(|e| anyhow!("[🤐 CASE]: ❌️ 批量删除分类失败: {}", e))?;

        info!("[🗣️ CASE] - ✅️ 删除分类成功: uid={}, target_id={}", uid, id);
        Ok(())
    }
}

//////// END

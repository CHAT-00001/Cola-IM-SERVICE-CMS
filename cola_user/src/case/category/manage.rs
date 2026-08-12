// cola_user/case/category/manage.rs
// 用户 - case - 分类 - 管理
// 2026/8/2 22:48 Created.

////////

use anyhow::Result;
use port::app::ctx::AppContext;
use tracing::info;

////////

/// # [MANAGE CASE]
pub struct UserCategoryManageCase;

impl UserCategoryManageCase {
    //

    ////////

    /// # 1. [CASE] - 单个移除黑名单
    pub async fn case_single_del(
        uid: i64, // 操作者 ID
        id: i64,  // 目标 ID
        ctx: AppContext,
    ) -> Result<u16, anyhow::Error> {
        // 1. 调用底层，用 qty 接收删除影响的条数
        let qty = ctx
            .user
            .black
            .del
            .single_soft_del(uid, id)
            .await
            .map_err(|e| anyhow::anyhow!("[🗣️ CASE]: ❌️ 移除黑名单失败: {}", e))?;

        // 2. 可以根据业务需要在这里做判断（可选）
        if qty == 0 {
            info!(
                "[🗣️ CASE] - ⚠️ 移除黑名单未命中目标: uid={}, target_id={}",
                uid, id
            );
        } else {
            info!(
                "[🗣️ CASE] - ✅️ 移除黑名单成功: uid={}, target_id={}",
                uid, id
            );
        }

        // 3. 将统计数量返回给 API Handler 组装响应
        Ok(qty)
    }

    ////////

    /// # 2. [CASE] - 批量遍历移除黑名单
    pub async fn case_batch_del(
        uid: i64,      // 操作者ID
        ids: Vec<i64>, // 目标IDs
        ctx: AppContext,
    ) -> Result<u16, anyhow::Error> {
        // 1. 调用底层，用 qty 接收批量删除影响的条数
        let qty = ctx
            .user
            .black
            .del
            .batch_soft_del(uid, ids)
            .await
            .map_err(|e| anyhow::anyhow!("[🗣️ CASE]: ❌️ 批量移除黑名单失败: {}", e))?;

        info!(
            "[🗣️ CASE] - ✅️ 批量移除黑名单成功: uid={}, 共成功移除 {} 条.",
            uid, qty
        );

        // 2. 将统计数量返回给 API Handler 组装响应
        Ok(qty)
    }
}

//////// END

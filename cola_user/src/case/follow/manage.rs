// cola_user/src/case/follow/manage.rs
// core - USER - case - follow - 管理 用例
// 2026/8/3 12:16 Created.

////////

use anyhow::Result;
use tracing::info;
use port::ctx::AppContext;

////////

/// # [MANAGE CASE]
pub struct UserFollowManageCase;

impl UserFollowManageCase {
    //

    ////////

    /// # 1. [CASE] - 单个移除关注
    pub async fn case_single_del(
        uid: i64, // 操作者ID
        id: i64,  // 目标ID
        ctx: AppContext,
    ) -> Result<u16, anyhow::Error> {
        // 1. 调用底层，用 qty 接收删除影响的条数
        let qty = ctx
            .user
            .follow
            .del
            .single_soft_del(uid, id)
            .await
            .map_err(|e| anyhow::anyhow!("[🗣️ CASE]: ❌️ 移除关注失败: {}", e))?;

        // 2. 可以根据业务需要在这里做判断（可选）
        if qty == 0 {
            info!("[🗣️ CASE] - ⚠️ 移除关注未命中目标: uid={}, target_id={}", uid, id);
        } else {
            info!("[🗣️ CASE] - ✅️ 移除关注成功: uid={}, target_id={}", uid, id);
        }

        // 3. 将统计数量返回给 API Handler 组装响应
        Ok(qty)
    }

    ////////

    /// # 2. [CASE] - 批量遍历移除关注
    pub async fn case_batch_del(
        uid: i64,      // 操作者ID
        ids: Vec<i64>, // 目标IDs
        ctx: AppContext,
    ) -> Result<u16, anyhow::Error> {
        // 1. 调用底层，用 qty 接收批量删除影响的条数
        let qty = ctx
            .user
            .follow
            .del
            .batch_soft_del(uid, ids)
            .await
            .map_err(|e| anyhow::anyhow!("[🗣️ CASE]: ❌️ 批量移除关注失败: {}", e))?;

        info!("[🗣️ CASE] - ✅️ 批量移除关注成功: uid={}, 共成功移除 {} 条.", uid, qty);

        // 2. 将统计数量返回给 API Handler 组装响应
        Ok(qty)
    }
}

//////// END
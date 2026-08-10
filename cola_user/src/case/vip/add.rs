// cola_user/src/case/vip/add.rs
// 用户 - case - 贵宾 - 添加 用例
// 2026/8/2 22:52 Created.

////////

use anyhow::{Result, anyhow};
use tracing::info;
use port::ctx::AppContext;
////////

/// # [ADD CASE] - 添加
/// * `desc`: `用户贵宾添加用例`
pub struct UserVipAddCase;

impl UserVipAddCase {
    //

    ////////

    /// # 1. [CASE] - 添加
    /// * `desc`: `用户充值贵宾记录`
    pub async fn case_add_new(
        uid: i64,
        target_id: i64,
        ctx: &AppContext,
    ) -> Result<(), anyhow::Error> {
        // 核心数据持久化：通过 Port 调用 adapter → service → repository
        ctx.user
            .vip
            .add
            .add_black(uid, target_id)
            .await
            .map_err(|e| anyhow!("[🤐 CASE]: ❌️ 充值会员失败: {}", e))?;

        info!("[🗣️ CASE] - ✅️ 充值会员成功: uid={}, target_id={}", uid, target_id);

        Ok(())
    }

    ////////

    /// # 2. [CASE] - 取消VIP
    pub async fn case_cancel_vip(
        uid: i64,
        target_id: i64,
        ctx: &AppContext,
    ) -> Result<(), anyhow::Error> {
        ctx.user
            .vip
            .add
            .del_black(uid, target_id)
            .await
            .map_err(|e| anyhow!("[🤐 CASE]: ❌️ 取消VIP失败: {}", e))?;

        info!("[🗣️ CASE] - ✅️ 取消VIP成功: uid={}, target_id={}", uid, target_id);

        Ok(())
    }
}

//////// END

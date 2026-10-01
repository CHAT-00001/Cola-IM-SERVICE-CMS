// repo_adapter/src/aaaa/app/check.rs -- ADAPTER - BASIC - 应用 - 检查服务
// 2026/8/9 20:48 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::basic::app::check::AppCheckPort;

////////

/// # [CHECK ADAPTER] - 应用检查适配器
/// * `desc`: `COLA BASIC - Apps Check Adapter `
#[derive(Debug, Default, Clone)]
pub struct AppCheckAdapter;

#[async_trait]
impl AppCheckPort for AppCheckAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 健康
    async fn check_health(&self, uid: i64, comment_id: i64) -> Result<(bool)> {
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] - 状态
    async fn check_state(&self, uid: i64, comment_id: i64) -> Result<(bool)> {
        todo!()
    }

    ////////

    /// # 3. [ADAPTER] - 归属
    async fn is_owner(
        &self,
        uid: i64,
        user_id: i64,    // 用户 ID
        comment_id: i64, // 评论 ID
    ) -> Result<(bool)> {
        todo!()
    }
}

//////// END

// repo_adapter/src/fs/cdn/check.rs
// 🔌 适配器 - FS - CDN - 检查
// 2026/8/9 20:48 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::fs::cdn::check::CdnCheckPort;

////////

/// # [CHECK ADAPTER] - 检查
/// * `desc`: `FS - CDN检查适配器`
#[derive(Debug, Default, Clone)]
pub struct CdnCheckAdapter;

#[async_trait]
impl CdnCheckPort for CdnCheckAdapter {
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

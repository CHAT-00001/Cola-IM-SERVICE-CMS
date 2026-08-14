// repo_adapter/src/auth/identity/stat.rs
// 🔌 适配器 - AUTH - 身份识别 - 统计
// 2026/8/6 19:18 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::auth::identity::stat::IdentityStatPort;
use port::cola_video::comment::stat::VideoCommentStatPort;

////////

/// # [STAT ADAPTER] - 统计
/// * `desc`: `AUTH - 验证身份统计适配器`
#[derive(Debug, Default, Clone)]
pub struct IdentityStatAdapter;

#[async_trait]
impl IdentityStatPort for IdentityStatAdapter {
    async fn stat_count_by_user_id(&self, uid: i64, user_id: i64) -> Result<(u64)> {
        todo!()
    }

    async fn stat_count_by_video_id(&self, uid: i64, video_id: i64) -> Result<(u64)> {
        todo!()
    }
}

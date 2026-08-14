// repo_adapter/src/auth/session/stat.rs
// 🔌 适配器 - AUTH - SESSION - 统计
// 2026/8/6 19:18 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::auth::session::stat::SessionStatPort;
use port::cola_video::comment::stat::VideoCommentStatPort;

////////

/// # [STAT ADAPTER] - 发布
/// * `desc`: `AUTH - 验证会话统计适配器`
#[derive(Debug, Default, Clone)]
pub struct SessionStatAdapter;

#[async_trait]
impl SessionStatPort for SessionStatAdapter {
    async fn stat_count_by_user_id(&self, uid: i64, user_id: i64) -> Result<(u64)> {
        todo!()
    }

    async fn stat_count_by_video_id(&self, uid: i64, video_id: i64) -> Result<(u64)> {
        todo!()
    }
}

// repo_adapter/src/video/comment/check.rs
// 🔌 适配器 - ▶ 视频 -  评论 - 检查
// 2026/8/9 20:48 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_video::comment::check::VideoCommentCheckPort;

////////

/// # [CHECK ADAPTER] - 检查
/// * `desc`: `▶ 视频 - 评论记录检查适配器`
#[derive(Debug, Default, Clone)]
pub struct VideoCommentCheckAdapter;

#[async_trait]
impl VideoCommentCheckPort for VideoCommentCheckAdapter {
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

// repo_adapter/src/video/comment/like.rs
// 🔌 插头 - 可乐视频 - 评论 - 评论点赞
// 2026/8/6 19:18 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_video::comment::like::VideoCommentLikePort;

////////

/// # [ADD SERVICE] - 发布
/// * `desc`: `可乐视频 - 视频评论发布服务`
#[derive(Debug, Default, Clone)]
pub struct VideoCommentLikeAdapter;

#[async_trait]
impl VideoCommentLikePort for VideoCommentLikeAdapter {
    async fn upsert_like(&self, uid: i64, comment_id: i64, state: bool) -> Result<(bool)> {
        todo!()
    }

    async fn check_state(&self, uid: i64, comment_id: i64) -> Result<(bool)> {
        todo!()
    }
}

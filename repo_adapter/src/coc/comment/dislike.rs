// repo_adapter/src/video/comment/dislike.rs
// 🔌 插头 - 可乐视频 - 评论 - 不喜欢
// 2026/8/6 19:58 Created.

////////

use async_trait::async_trait;
use port::cola_video::comment::dislike::VideoCommentDisikePort;

/// # [DISLIKE SERVICE] - 评论不喜欢服务
pub struct VideoCommentDislikeAdapter;

#[async_trait]
impl VideoCommentDisikePort for VideoCommentDislikeAdapter {
    //

    ////////

    /// 1. # [SERVICE] 不喜欢/取消
    async fn upsert_dislike(
        &self,
        uid: i64,
        comment_id: i64,
        is_disliked: bool,
    ) -> anyhow::Result<(bool)> {
        todo!()
    }

    async fn check_state(&self, uid: i64, comment_id: i64) -> anyhow::Result<(bool)> {
        todo!()
    }
}

//////// END

// repo_adapter/src/video/comment/like.rs -- ADAPTER - VIDEO - 评论 - 点赞
// 2026/8/6 19:18 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_video::comment::like::VideoCommentLikePort;

////////

/// # [LIKE SERVICE] - 视频评论点赞适配器
/// * `desc`: `COLA VIDEO - 视频评论发布服务`
#[derive(Debug, Default, Clone)]
pub struct VideoCommentLikeAdapter;

#[async_trait]
impl VideoCommentLikePort for VideoCommentLikeAdapter {

    //

    ////////

    /// # 1. [SERVICE ADAPTER] - 点赞评论(支持正反操作)
    async fn upsert_like(&self, uid: i64, comment_id: i64, state: bool) -> Result<(bool)> {
        todo!()
    }

    ////////

    /// # 2. [SERVICE ADAPTER] - 检查状态
    async fn check_state(&self, uid: i64, comment_id: i64) -> Result<(bool)> {
        todo!()
    }
}

//////// END
// cola_live/src/case/comment/like.rs
// LIVE - 用例层 - 评论 - 互动
// 2026/8/12 04:51 Created.

////////

use anyhow::Result;
use port::app::ctx::AppContext;
use service::cola_video::comment::like::CommentLikeService;

////////

/// # [LIKE CASE] - 评论 喜欢/不喜欢
/// * `DESC`: `LIVE` - `评论互动用例编排`
pub struct CommentLikeCase;

////////
impl CommentLikeCase {
    //

    ////////

    /// # 1. [CASE] - 点赞/取消点赞
    /// * `描述` 支持正反操作
    pub async fn case_set_comment_like(uid: i64, comment_id: i64, is_liked: bool) -> Result<()> {
        CommentLikeService::update_comment_like_by_id(uid, comment_id, is_liked).await?;
        Ok(())
    }

    /////////

    /// # 7. [CASE] - 不喜欢/取消不喜欢
    /// * `描述` : 支持正反操作
    pub async fn case_set_comment_dislike(
        uid: i64,
        comment_id: i64,
        is_disliked: bool,
        ctx: &AppContext,
    ) -> Result<()> {
        CommentLikeService::update_comment_unlike_by_id(uid, comment_id, is_disliked).await?;
        Ok(())
    }
}

//////// END

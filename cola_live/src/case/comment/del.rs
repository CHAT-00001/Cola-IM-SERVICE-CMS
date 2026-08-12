// cola_live/src/case/comment/del.rs
// LIVE - 用例层 - 评论 - 删除
// 2026/8/12 04:51 Created.

////////

use anyhow::Result;
use port::app::ctx::AppContext;

////////

/// # [DELETE CASE] - 评论 删除
/// * `DESC`: 评论删除用例编排
pub struct CommentDelCase;

////////
impl CommentDelCase {
    //

    ////////

    /// # 1. [CASE] - 单条删除评论
    pub async fn case_single_delete(
        uid: i64,
        comment_id: i64, // 评论 ID
        ctx: &AppContext,
    ) -> Result<(String)> {
        ctx.video.comment.del.single_delete(comment_id).await?;
        Ok("单条删除评论成功~".to_string())
    }

    /////////

    /// # 2. [CASE] - 批量删除评论
    pub async fn case_batch_delete(comment_ids: Vec<i64>, ctx: &AppContext) -> Result<(String)> {
        ctx.video.comment.del.batch_delete(comment_ids).await?;
        Ok("批量删除评论成功~".to_string())
    }
}

//////// END

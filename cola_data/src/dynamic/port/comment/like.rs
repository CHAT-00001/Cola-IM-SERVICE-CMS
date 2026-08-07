// video/port/comment/like.rs
// 视频 - port - 评论 - 列表
// 2026/8/5 00:03 Created.

////////

use crate::video::command::comment::CommentCommand;
use crate::video::info::comment::VideoCommentInfo;

////////

/// # [LIST SERVICE] - 点赞
/// * `desc`: `视频评论点赞服务端口`
#[async_trait::async_trait]
pub trait LikePort: Send + Sync {
    //

    ////////

    /// # [PORT] - 更新/插入
    /// * `desc`: `用户更新/插入点赞记录`
    async fn upsert_like_record(
        &self,
        uid: i64,        // UID
        comment_id: i64, // 评论 ID
        is_liked: bool,  // 状态
    ) -> anyhow::Result<(VideoCommentInfo)>;

    ////////

    /// # [PORT] - 更新点赞记录
    /// * `desc`: `用户更新/插入点赞记录`
    async fn update_like_record(
        &self,
        uid: i64,        // UID
        comment_id: i64, // 评论ID
        status: i16,     // 状态码
    ) -> anyhow::Result<(VideoCommentInfo)>;

    ////////

    /// # [PORT] - 删除
    async fn del_comment_record(&self, comment_id: i64) -> anyhow::Result<()>;

    ////////

    /// # [PORT] - 批量删除
    async fn del_comments_record(&self, comment_ids: Vec<i64>) -> anyhow::Result<()>;
}

//////// END

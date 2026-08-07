// cola_video/prot/comment/get.rs
// 视频 - port - 评论 - 获取
// 2026/6/10 08:23 Created.
// 2026/8/5 00:03 Updated.

////////

use crate::cola_video::command::comment::CommentCommand;
use crate::cola_video::info::comment::VideoCommentInfo;

////////

/// # [SERVICE PORT] - 评论
#[async_trait::async_trait]
pub trait GetPort: Send + Sync {
    ////////

    /// # [PORT] - 保存
    async fn save_comment_record(
        &self,
        uid: i64,
        video_id: i64,
        is_liked: bool,
    ) -> anyhow::Result<(VideoCommentInfo)>;

    ////////

    /// # [PORT] - 编辑
    async fn edit_comment_record(
        &self,
        comment_id: i64,
        cmd: CommentCommand,
    ) -> anyhow::Result<(VideoCommentInfo)>;

    ////////

    /// # [PORT] - 删除
    async fn del_comment_record(&self, comment_id: i64) -> anyhow::Result<()>;

    ////////

    /// # [PORT] - 批量删除
    async fn del_comments_record(&self, comment_ids: Vec<i64>) -> anyhow::Result<()>;
}

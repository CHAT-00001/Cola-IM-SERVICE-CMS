// video/port/comment/add.rs
// 视频 - port - 评论 - 列表
// 2026/8/5 00:03 Created.

////////

use crate::video::command::comment::CommentCommand;
use crate::video::info::comment::VideoCommentInfo;

////////

/// # [LIST SERVICE] - 发布
/// * `desc`: `视频评论发布服务端口`
#[async_trait::async_trait]
pub trait AddPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 保存
    async fn save_comment_record(
        &self,
        uid: i64,
        video_id: i64,
        cmd: CommentCommand,
    ) -> anyhow::Result<(VideoCommentInfo)>;

    ////////

    /// # 2. [PORT] - 编辑
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

//////// END
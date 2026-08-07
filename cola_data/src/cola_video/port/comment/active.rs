
// /active.rs
// 
// 2026/8/5 00:03 Created.

////////


// /add  -- 评论服务端口
// 2026/6/10 23:35

////////


// /count  -- 收藏 服务端口
// 2026/6/10 08:23

////////

use crate::cola_video::command::comment::CommentCommand;
use crate::cola_video::info::comment::VideoCommentInfo;

////////

/// # [SERVICE PORT] - 评论
#[async_trait::async_trait]
pub trait CommentRepo: Send + Sync {

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
        cmd:CommentCommand,
    ) -> anyhow::Result<(VideoCommentInfo)>;

    ////////

    /// # [PORT] - 删除
    async fn del_comment_record(
        &self,
        comment_id: i64,
    ) -> anyhow::Result<()>;

    ////////

    /// # [PORT] - 批量删除
    async fn del_comments_record(
        &self,
        comment_ids: Vec<i64>,
    ) -> anyhow::Result<()>;
}
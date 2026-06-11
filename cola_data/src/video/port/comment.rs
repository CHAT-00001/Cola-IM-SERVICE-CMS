// /comment.rs  -- 评论服务端口
// 2026/6/10 23:35

////////


// /collect.rs  -- 收藏 服务端口
// 2026/6/10 08:23

////////

use crate::video::command::comment::CommentCommand;
use crate::video::info::comment::CommentInfo;

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
    ) -> anyhow::Result<(CommentInfo)>;

    ////////

    /// # [PORT] - 编辑
    async fn edit_comment_record(
        &self,
        comment_id: i64,
        cmd:CommentCommand,
    ) -> anyhow::Result<(CommentInfo)>;

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
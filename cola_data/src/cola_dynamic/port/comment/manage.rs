// cola_video/prot/comment/manage.rs
// 视频 - port - 评论 - 管理
// 2026/8/5 15:23 Created.

////////

use crate::cola_video::command::comment::CommentCommand;
use crate::cola_video::info::comment::VideoCommentInfo;

////////

/// # [MANAGE SERVICE PORT] - 管理
/// * `desc`: `评论管理服务端口`
#[async_trait::async_trait]
pub trait ManagePort: Send + Sync {
    //

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

//////// END
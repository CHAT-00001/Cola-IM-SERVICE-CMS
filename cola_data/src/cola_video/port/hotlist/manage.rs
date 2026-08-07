// cola_video/port/hotlist/manage.rs
// 视频 - port - 上热门 - 管理
// 2026/8/5 00:08 Created.

////////


// cola_video/prot/collect/manage.rs
// 视频 - port - 上热门 - 管理
// 2026/8/5 00:04 Created.

////////

use crate::cola_video::command::comment::CommentCommand;
use crate::cola_video::info::comment::VideoCommentInfo;

////////

/// # [MANAGE PORTS] - 管理
/// * `desc`: `视频上热门管理端口`
#[async_trait::async_trait]
pub trait VideoHotlistManagePort: Send + Sync {
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
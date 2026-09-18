// port/src/cola_dynamic/collect/manage.rs
// ⏩️ 端口 - 可乐动态 -收藏 - 管理
// 2026/8/5 00:04 Created.

////////

use cola_data::cola_video::command::comment::CommentCommand;
use cola_data::cola_video::info::comment::CommentInfo;

////////

/// # [MANAGE PORTS] - 管理
/// * `desc`: `动态收藏管理端口`
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
    ) -> anyhow::Result<(CommentInfo)>;

    ////////

    /// # [PORT] - 编辑
    async fn edit_comment_record(
        &self,
        comment_id: i64,
        cmd: CommentCommand,
    ) -> anyhow::Result<(CommentInfo)>;

    ////////

    /// # [PORT] - 删除
    async fn del_comment_record(&self, comment_id: i64) -> anyhow::Result<()>;

    ////////

    /// # [PORT] - 批量删除
    async fn del_comments_record(&self, comment_ids: Vec<i64>) -> anyhow::Result<()>;
}

//////// END

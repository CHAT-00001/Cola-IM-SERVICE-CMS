// port/src/cola_dynamic/identity/manage.rs
// ⏩️ 端口 - 可乐动态 - 评论 - 管理
// 2026/8/5 15:23 Created.

////////

use cola_data::cola_dynamic::info::comment::DynamicCommentInfo;
use cola_data::cola_video::command::comment::CommentCommand;
////////

/// # [MANAGE SERVICE PORT] - 管理
/// * `desc`: `评论管理服务端口`
#[async_trait::async_trait]
pub trait DynamicCommentManagePort: Send + Sync {
    //

    ////////

    /// # [PORT] - 保存
    async fn save_comment_record(
        &self,
        uid: i64,
        video_id: i64,
        is_liked: bool,
    ) -> anyhow::Result<(DynamicCommentInfo)>;

    ////////

    /// # [PORT] - 编辑
    async fn edit_comment_record(
        &self,
        comment_id: i64,
        cmd: CommentCommand,
    ) -> anyhow::Result<(DynamicCommentInfo)>;

    ////////

    /// # [PORT] - 删除
    async fn del_comment_record(&self, comment_id: i64) -> anyhow::Result<()>;

    ////////

    /// # [PORT] - 批量删除
    async fn del_comments_record(&self, comment_ids: Vec<i64>) -> anyhow::Result<()>;
}

//////// END
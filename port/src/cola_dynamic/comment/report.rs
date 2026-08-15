// cola_video/port/file/report.rs
// 视频 - port - 评论 - 列表
// 2026/8/5 00:03 Created.

////////

use cola_data::cola_video::command::comment::CommentCommand;

////////

/// # [LIST SERVICE] - 举报
/// * `desc`: `视频评论举报服务端口`
#[async_trait::async_trait]
pub trait DynamicCommentReportPort: Send + Sync {
    //

    ////////

    /// # [PORT] - 保存
    async fn save_comment_record(
        &self,
        uid: i64,
        video_id: i64,
        is_liked: bool,
    ) -> anyhow::Result<()>;

    ////////

    /// # [PORT] - 编辑
    async fn edit_comment_record(
        &self,
        comment_id: i64,
        cmd: CommentCommand,
    ) -> anyhow::Result<()>;

    ////////

    /// # [PORT] - 删除
    async fn del_comment_record(&self, comment_id: i64) -> anyhow::Result<()>;

    ////////

    /// # [PORT] - 批量删除
    async fn del_comments_record(&self, comment_ids: Vec<i64>) -> anyhow::Result<()>;
}

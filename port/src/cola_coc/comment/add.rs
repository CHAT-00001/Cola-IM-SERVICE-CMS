// port/src/cola_video/comment/add.rs
// ⏩️ 端口 - VIDEO - 评论 - 列表
// 2026/8/5 00:03 Created.

////////

use cola_data::cola_video::command::comment::CommentCommand;
use cola_data::cola_video::info::comment::VideoCommentInfo;

////////

/// # [ADD PORTS] - 发布
/// * `desc`: `VIDEO - 视频评论发布端口`
#[async_trait::async_trait]
pub trait VideoCommentAddPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 发送
    /// * `desc`: `▶ 可乐视频 - 根据视频ID发送评论`
    async fn send_comment(
        &self,
        uid: i64,            // UID
        video_id: i64,       // 视频 ID
        cmd: CommentCommand, // 命令
    ) -> anyhow::Result<(VideoCommentInfo)>;

    ////////

    /// # 2. [PORT] - 编辑
    /// * `desc`: `▶ 可乐视频 - 根据评论ID编辑评论`
    async fn edit_comment(
        &self,
        uid: i64,            // UID
        comment_id: i64,     // 评论 ID
        cmd: CommentCommand, // 命令
    ) -> anyhow::Result<(VideoCommentInfo)>;
}

//////// END

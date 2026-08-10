// port/src/cola_dynamic/comment/add.rs
// ⏩️ 端口 - ⏹ 可乐动态 - 评论 - 发布
// 2026/8/5 00:03 Created.

////////

use cola_data::cola_video::command::comment::CommentCommand;
use cola_data::cola_video::info::comment::VideoCommentInfo;

////////

/// # [ADD PORTS] - 发布
/// * `desc`: `⏹ 可乐动态 - 视频评论发布服务端口`
#[async_trait::async_trait]
pub trait DynamicCommentAddPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 发送评论
    /// * `desc`: `⏹ 可乐动态 - 发布评论`
    async fn send_comment(
        &self,
        uid: i64,
        video_id: i64,
        cmd: CommentCommand,
    ) -> anyhow::Result<(VideoCommentInfo)>;

    ////////

    /// # 2. [PORT] - 编辑评论
    /// * `desc`: `⏹ 可乐动态 - 编辑评论`
    async fn edit_comment(
        &self,
        comment_id: i64,
        cmd: CommentCommand,
    ) -> anyhow::Result<(VideoCommentInfo)>;
}

//////// END

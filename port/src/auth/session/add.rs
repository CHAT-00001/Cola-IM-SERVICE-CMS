// port/src/auth/session/add.rs
// ⏩️ 端口 - AUTH - 会话 - 发布
// 2026/8/5 00:03 Created.

////////

use cola_data::cola_video::command::comment::CommentCommand;
use cola_data::cola_video::info::comment::VideoCommentInfo;

////////

/// # [ADD PORT] - 发布
/// * `desc`: `AUTH- 会话发布端口`
#[async_trait::async_trait]
pub trait SessionAddPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 保存
    /// * `desc`: `AUTH - 根据用户ID保存会话`
    async fn save_session(
        &self,
        uid: i64,            // UID
        user_id: i64,       // 用户 ID
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

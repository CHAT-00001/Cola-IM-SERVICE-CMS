// port/src/auth/identit/get.rs
// ⏩️ 端口 - AUTH - 身份识别 - 获取
// 2026/6/10 08:23 Created.

////////

use cola_data::cola_video::command::comment::CommentCommand;
use cola_data::cola_video::info::comment::VideoCommentInfo;

////////

/// # [SERVICE PORT] - 评论
#[async_trait::async_trait]
pub trait IdentityGetPort: Send + Sync {
    ////////

    /// # [PORT] - 用户的
    async fn get_comment_by_user_id(
        &self,
        user_id: i64, // 用户 ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> anyhow::Result<(Vec<VideoCommentInfo>)>;

    ////////

    /// # [PORT] - 视频的
    async fn get_comment_by_video(
        &self,
        video_id: i64, // 视频 ID
        limit: i64,    // 数量
        offset: i64,   // 页码
    ) -> anyhow::Result<(Vec<VideoCommentInfo>)>;
}

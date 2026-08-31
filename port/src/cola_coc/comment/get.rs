// port/src/cola_video/comment/get.rs
// ⏩️ 端口 - VIDEO - 评论 - 获取
// 2026/6/10 08:23 Created.

////////

use cola_data::cola_video::command::comment::CommentCommand;
use cola_data::cola_video::info::comment::VideoCommentInfo;

////////

/// # [SERVICE PORT] - 评论
/// * `desc`: `VIDEO - 评论获取端口`
#[async_trait::async_trait]
pub trait VideoCommentGetPort: Send + Sync {
    //

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

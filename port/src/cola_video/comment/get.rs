// port/src/cola_video/comment/get.rs -- PORT - VIDEO - 评论 - 获取服务
// 2026/6/10 08:23 Created.

////////

use cola_data::cola_video::command::comment::CommentCommand;
use cola_data::cola_video::info::comment::CommentInfo;

////////

/// # [GET SERVICE PORT] - 视频评论获取服务端口
/// * `desc`: `VIDEO - Comment Get Service Ports.`
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
    ) -> anyhow::Result<(Vec<CommentInfo>)>;

    ////////

    /// # [PORT] - 视频的
    async fn get_comment_by_video_id(
        &self,
        video_id: i64, // 视频 ID
        limit: i64,    // 数量
        offset: i64,   // 页码
    ) -> anyhow::Result<(Vec<CommentInfo>)>;
}

//////// END

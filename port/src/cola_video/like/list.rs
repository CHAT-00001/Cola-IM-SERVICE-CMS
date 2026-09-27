// port/src/video/like/list.rs -- PORT - VIDEO - 点赞 - 列表
// 2026/8/5 00:04 Created.

////////

use cola_data::cola_video::info::comment::CommentInfo;

////////

/// # [LIST SERVICE] - 视频点赞列表服务端口
/// * `desc`: `COLA VIDEO - Content Like List Service Ports`
#[async_trait::async_trait]
pub trait VideoLikeListPort: Send + Sync {
    //

    ////////

    /// # [PORT] - 用户的
    /// * `desc`: `根据用户ID` - `获取用户的点赞记录信息`
    async fn get_like_infos_by_user_id(
        &self,
        uid: i64,     // UID
        user_id: i64, // 用户 ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> anyhow::Result<(CommentInfo)>;

    ////////

    /// # [PORT] - 视频的
    /// * `desc`: `根据视频ID` - `获取视频的点赞记录信息`
    async fn get_like_infos_by_video_id(
        &self,
        uid: i64,      // UID
        video_id: i64, // 视频 ID
        limit: i64,    // 数量
        offset: i64,   // 页码
    ) -> anyhow::Result<(CommentInfo)>;
}

//////// END

// port/src/videolike/list.rs
// ⏩️ 端口 - ▶ 视频 -  点赞 - 列表
// 2026/8/5 00:04 Created.

////////

use cola_data::cola_video::info::comment::VideoCommentInfo;

////////

/// # [LIST PORTS] - 列表
/// * `desc`: `获取视频点赞列表端口`
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
    ) -> anyhow::Result<(VideoCommentInfo)>;

    ////////

    /// # [PORT] - 视频的
    /// * `desc`: `根据视频ID` - `获取视频的点赞记录信息`
    async fn get_like_infos_by_video_id(
        &self,
        uid: i64,     // UID
        user_id: i64, // 用户 ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> anyhow::Result<(VideoCommentInfo)>;
}

//////// END

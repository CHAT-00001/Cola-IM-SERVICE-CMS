// port/src/video/dislike/list.rs
// ⏩️ 端口 - ▶ 视频 - 不喜欢 - 列表
// 2026/8/9 21:53 Created.

////////

use cola_data::cola_video::info::dislike::VideoDislikeInfo;

////////

/// # [LIST PORTS] - 列表
/// * `desc`: `获取视频不喜欢列表端口`
#[async_trait::async_trait]
pub trait VideoDislikeListPort: Send + Sync {
    //

    ////////

    /// # [PORT] - 用户的
    /// * `desc`: `根据用户ID` - `获取用户的不喜欢记录信息`
    async fn get_dislike_infos_by_user_id(
        &self,
        uid: i64,     // UID
        user_id: i64, // 用户 ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> anyhow::Result<(Vec<VideoDislikeInfo>)>;

    ////////

    /// # [PORT] - 视频的
    /// * `desc`: `根据视频ID` - `获取视频的不喜欢记录信息`
    async fn get_dislike_infos_by_video_id(
        &self,
        uid: i64,     // UID
        user_id: i64, // 用户 ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> anyhow::Result<(Vec<VideoDislikeInfo>)>;
}

//////// END

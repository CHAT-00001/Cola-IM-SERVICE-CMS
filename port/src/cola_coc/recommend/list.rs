// cola_video/hotlist/list.rs
// ⏩️ 端口 - ▶ 可乐视频 -  推荐 - 列表
// 2026/8/5 19:09 Created.

////////

use cola_data::cola_video::info::recommend::VideoRecommendInfo;

////////

/// # [LIST PORTS] - 列表
/// * `desc`: `视频推荐列表端口`
#[async_trait::async_trait]
pub trait VideoRecommendListPort: Send + Sync {
    //

    ////////

    /// # [PORT] - 用户的
    /// * `desc`: `根据用户ID` - `获取用户的推荐记录信息`
    async fn get_recommend_infos_user_id(
        &self,
        uid: i64,     // UID
        user_id: i64, // 用户 ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> anyhow::Result<(Vec<VideoRecommendInfo>)>;

    ////////

    /// # [PORT] - 视频的
    /// * `desc`: `根据视频ID` - `获取视频的推荐记录信息`
    async fn get_recommend_infos_video_id(
        &self,
        uid: i64,      // UID
        video_id: i64, // 视频 ID
        limit: i64,    // 数量
        offset: i64,   // 页码
    ) -> anyhow::Result<(Vec<VideoRecommendInfo>)>;
}

//////// END

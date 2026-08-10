// repo_adapter/src/cola_video/like/list.rs
// 🔌 插头 - 可乐视频 - 点赞 - 列表
// 2026/8/6 18:57 Created.

////////

use cola_data::cola_video::info::like::VideoLikeInfo;

/// # [LIST PORTS] - 列表
/// * `desc`: `▶ 视频 - 上热门列表端口`
#[async_trait::async_trait]
pub trait VideoHotlistListPort: Send + Sync {
    //

    ////////

    /// # [PORT] - 用户的
    /// * `desc`: `根据用户ID` - `获取用户的上热门记录`
    async fn get_hotlist_infos_by_user_id(
        &self,
        uid: i64,     // UID
        user_id: i64, // 用户 ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> anyhow::Result<(Vec<VideoLikeInfo>)>;

    ////////

    /// # [PORT] - 视频的
    /// * `desc`: `根据视频ID` - `获取视频的上热门记录`
    async fn get_hotlist_infos_by_video_id(
        &self,
        uid: i64,      // UID
        video_id: i64, // 视频 ID
        limit: i64,    // 数量
        offset: i64,   // 页码
    ) -> anyhow::Result<(Vec<VideoLikeInfo>)>;
}

//////// END

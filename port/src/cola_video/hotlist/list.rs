// port/src/cola_video/hotlist/list.rs
// ⏩️ 端口 - ▶ 视频 - 上热门 - 列表
// 2026/8/5 00:07 Created.

////////

use cola_data::cola_video::info::hotlist::VideoHotlistInfo;

////////

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
    ) -> anyhow::Result<(Vec<VideoHotlistInfo>)>;

    ////////

    /// # [PORT] - 视频的
    /// * `desc`: `根据视频ID` - `获取视频的上热门记录`
    async fn get_hotlist_infos_by_video_id(
        &self,
        uid: i64,      // UID
        video_id: i64, // 视频 ID
        limit: i64,    // 数量
        offset: i64,   // 页码
    ) -> anyhow::Result<(Vec<VideoHotlistInfo>)>;
}

//////// END

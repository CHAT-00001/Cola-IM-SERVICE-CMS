// port/src/cola_video/video/feed.rs
// ⏩️ 端口 - 可乐视频 -  视频 - 流
// 2026/6/10 08:25

////////

use cola_data::cola_video::info::video::VideoInfo;

////////

/// # [FEED PORTS] -  Feed
#[async_trait::async_trait]
pub trait FeedPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 新的
    async fn new_list(
        &self,
        uid: i64,
        video_id: i64,
        is_liked: bool,
    ) -> anyhow::Result<(Vec<VideoInfo>)>;

    ////////

    /// # 2. [PORT] - 热门
    async fn hot_list(
        &self,
        uid: i64,
        video_id: i64,
        is_unliked: bool,
    ) -> anyhow::Result<(Vec<VideoInfo>)>;

    ////////

    /// # 8. [PORT] - 附近
    async fn get_nearby_list(
        &self,
        lat: f64,   // 纬度
        lng: f64,   // 经度
        range: f64, // 范围
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<(Vec<VideoInfo>)>;
}

//////// END

// video/port/video/get.rs
// 视频 - port - 视频 - 获取
// 2026/8/5 00:00 Created.

////////

use crate::video::info::video::VideoInfo;

////////

/// # [GET PORTS] -  获取
/// * `desc`: `获取视频`
#[async_trait::async_trait]
pub trait VideoGetPort: Send + Sync {
    ////////

    /// # 1. [PORT] - 我的
    async fn get_my_list(
        &self,
        uid: i64,                // UID
        keyword: Option<String>, // 关键词
        limit: i64,              // 数量
        offset: i64,             // 页码
        is_liked: bool,
    ) -> anyhow::Result<(Vec<VideoInfo>)>;

    ////////

    /// # 2. [PORT] - TA的
    async fn get_he_list(
        &self,
        uid: i64,                // UID
        keyword: Option<String>, // 关键词
        limit: i64,              // 数量
        offset: i64,             // 页码
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

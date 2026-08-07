// /feed.rs  -- 数据流 服务端口
// 2026/7/7

//////

use crate::cola_gis::info::poi::PoiInfo;

/// # [SERVICE] - Feed
#[async_trait::async_trait]
pub trait FeedRepo: Send + Sync {

    ////////

    /// # 1. [PORT] - 新的
    async fn new_list(
        &self,
        uid: i64,
        poi_id: i64,
        is_liked: bool,
    ) -> anyhow::Result<Vec<PoiInfo>>;

    ////////

    /// # 2. [PORT] - 热门
    async fn hot_list(
        &self,
        uid: i64,
        poi_id: i64,
        is_unliked: bool,
    ) -> anyhow::Result<Vec<PoiInfo>>;

    ////////

    /// # 8. [PORT] - 附近
    async fn get_nearby_list(
        &self,
        lat: f64,   // 纬度
        lng: f64,   // 经度
        range: f64, // 范围
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<Vec<PoiInfo>>;
}
// repo_adapter/src/gis/feed.rs
// 2026-07-07

//////

use async_trait::async_trait;
use cola_data::gis::port::feed::FeedRepo;
use cola_data::gis::info::poi::PoiInfo;
use repo::gis::service::home::PoiHomeService;

//////

/// # [FEED PORT] - 流 适配器
pub struct FeedPortAdapter;

//////

#[async_trait]
impl FeedRepo for FeedPortAdapter {

    /// # 1. [PORT] - 最新
    async fn new_list(
        &self,
        _uid: i64,
        _poi_id: i64,
        _is_liked: bool,
    ) -> anyhow::Result<Vec<PoiInfo>> {
        PoiHomeService::find_new_gis_list(50, 0).await
    }

    /// # 2. [PORT] - 热门
    async fn hot_list(
        &self,
        _uid: i64,
        _poi_id: i64,
        _is_unliked: bool,
    ) -> anyhow::Result<Vec<PoiInfo>> {
        PoiHomeService::find_hot_gis_list(50, 0).await
    }

    /// # 3. [PORT] - 附近
    async fn get_nearby_list(
        &self,
        lat: f64,
        lng: f64,
        _range: f64,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<Vec<PoiInfo>> {
        PoiHomeService::find_city_gis_list(lat, lng, limit, offset).await
    }
}

////// END
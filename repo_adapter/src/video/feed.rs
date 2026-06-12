// repo_adapter/src/video/feed.rs
// 2026-06-12

////////

use async_trait::async_trait;
use cola_data::video::port::feed::FeedRepo;
use cola_data::video::info::video::VideoInfo;
use repo::video::service::home::VideoHomeService;

////////

/// # [FEED PORT] - 流 适配器
pub struct FeedPortAdapter;

////////

#[async_trait]
impl FeedRepo for FeedPortAdapter {

    /// # 1. [PORT] - 最新
    async fn new_list(
        &self,
        _uid: i64,
        _video_id: i64,
        _is_liked: bool,
    ) -> anyhow::Result<Vec<VideoInfo>> {
        VideoHomeService::find_new_video_list(50, 0).await
    }

    /// # 2. [PORT] - 热门
    async fn hot_list(
        &self,
        _uid: i64,
        _video_id: i64,
        _is_unliked: bool,
    ) -> anyhow::Result<Vec<VideoInfo>> {
        VideoHomeService::find_hot_video_list(50, 0).await
    }

    /// # 3. [PORT] -  附近
    async fn get_nearby_list(
        &self,
        lat: f64,
        lng: f64,
        range: f64,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<Vec<VideoInfo>> {
        VideoHomeService::find_city_video_list(lat, lng, limit, offset).await
    }
}

//////// END
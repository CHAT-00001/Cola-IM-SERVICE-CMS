// repo_adapter/src/cola_video/cola_video/list.rs
// 🔌 插头 - 可乐视频 - 视频 - 列表
// 2026/8/7 05:31 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::info::video::VideoInfo;
use cola_data::cola_video::port::video::list::VideoListPort;

////////

/// # [ADD ADAPTER] - 发布
/// * `desc`: `🔌 视频发布插头`
pub struct VideoListAdapter;

#[async_trait]
impl VideoListPort for VideoListAdapter {
    async fn get_new_list(&self, uid: i64, limit: i64, offset: i64) -> Result<(Vec<VideoInfo>)> {
        todo!()
    }

    async fn get_hot_list(&self, uid: i64, limit: i64, offset: i64) -> Result<(Vec<VideoInfo>)> {
        todo!()
    }

    async fn get_recommend_list(
        &self,
        uid: i64,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<VideoInfo>)> {
        todo!()
    }

    async fn get_category_list(
        &self,
        uid: i64,
        category_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<VideoInfo>)> {
        todo!()
    }

    async fn get_channel_list(
        &self,
        uid: i64,
        channel_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<VideoInfo>)> {
        todo!()
    }

    async fn get_city_list(
        &self,
        uid: i64,
        city_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<VideoInfo>)> {
        todo!()
    }

    async fn get_nearby_list(
        &self,
        lat: f64,
        lng: f64,
        range: f64,
        offset: i64,
        limit: i64,
    ) -> Result<(Vec<VideoInfo>)> {
        todo!()
    }

    async fn get_search_list(
        &self,
        uid: i64,
        keyword: Option<String>,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<VideoInfo>)> {
        todo!()
    }
}

//////// END

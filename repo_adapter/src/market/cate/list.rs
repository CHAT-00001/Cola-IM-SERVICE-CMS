// repo_adapter/src/cola_video/cola_video/list.rs
// 🔌 插头 - 可乐视频 - 视频 - 列表
// 2026/8/7 05:31 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::info::video::VideoInfo;
use port::cola_video::video::list::VideoListPort;
use port::market::cate::list::CateListPort;
////////

/// # [ADD ADAPTER] - 发布
/// * `desc`: `🔌 视频发布插头`
pub struct CateListAdapter;

#[async_trait]
impl CateListPort for CateListAdapter {
    async fn get_new_infos(&self, uid: i64, limit: i64, offset: i64) -> Result<(Vec<VideoInfo>)> {
        todo!()
    }

    async fn get_hot_infos(&self, uid: i64, limit: i64, offset: i64) -> Result<(Vec<VideoInfo>)> {
        todo!()
    }

    async fn get_recommend_infos(
        &self,
        uid: i64,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<VideoInfo>)> {
        todo!()
    }

    async fn get_category_infos(
        &self,
        uid: i64,
        category_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<VideoInfo>)> {
        todo!()
    }

    async fn get_channel_infos(
        &self,
        uid: i64,
        channel_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<VideoInfo>)> {
        todo!()
    }

    async fn get_city_infos(
        &self,
        uid: i64,
        city_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<VideoInfo>)> {
        todo!()
    }

    async fn get_nearby_infos(
        &self,
        lat: f64,
        lng: f64,
        range: f64,
        offset: i64,
        limit: i64,
    ) -> Result<(Vec<VideoInfo>)> {
        todo!()
    }

    async fn get_search_infos(
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

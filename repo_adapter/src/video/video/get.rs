// repo_adapter/src/video/video/get.rs
// 🔌 插头 - 可乐视频 - 视频 - 获取IDs
// 2026/8/6 19:19 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::video::info::video::VideoInfo;
use cola_data::video::port::video::get::VideoGetPort;

////////

/// # [ADD ADAPTER] - 发布
/// * `desc`: `🔌 视频发布插头`
pub struct VideoGetAdapter;

// 构造实现
#[async_trait]
impl VideoGetPort for VideoGetAdapter {
    async fn get_my_list(
        &self,
        uid: i64,
        keyword: Option<String>,
        limit: i64,
        offset: i64,
        is_liked: bool,
    ) -> Result<(Vec<VideoInfo>)> {
        todo!()
    }

    async fn get_he_list(
        &self,
        uid: i64,
        keyword: Option<String>,
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
}

//////// END

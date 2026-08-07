// repo_adapter/src/video/video/stat.rs
// 🔌 插头服务 - 可乐视频 - 视频 - 统计服务
// 2026/8/6 19:20 Created.

////////

use async_trait::async_trait;
use anyhow::Result;
use cola_data::video::info::video::VideoInfo;
use cola_data::video::port::video::stat::VideoStatPortt;

////////

/// # [STAT SERVICE] - 统计
/// * `desc`: `🔌 视频统计服务`
pub struct VideoStatAdapter;

// 构造实现
#[async_trait]
impl VideoStatPortt for VideoStatAdapter {
    async fn get_my_list(&self, uid: i64, keyword: Option<String>, limit: i64, offset: i64, is_liked: bool) -> Result<(Vec<VideoInfo>)> {
        todo!()
    }

    async fn get_he_list(&self, uid: i64, keyword: Option<String>, limit: i64, offset: i64) -> Result<(Vec<VideoInfo>)> {
        todo!()
    }

    async fn get_nearby_list(&self, lat: f64, lng: f64, range: f64, offset: i64, limit: i64) -> Result<(Vec<VideoInfo>)> {
        todo!()
    }
}

//////// END
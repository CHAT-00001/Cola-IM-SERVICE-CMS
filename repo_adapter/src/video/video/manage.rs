// repo_adapter/src/cola_video/cola_video/manage.rs
// 🔌 插头 - 可乐视频 - 视频 - 管理
// 2026/8/6 19:19 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::info::video::VideoInfo;
use cola_data::cola_video::port::video::manage::VideoManagePort;

////////

/// # [MANAGE ADAPTER] - 管理
/// * `desc`: `🔌 视频管理服务`
pub struct VideoManageAdapter;

#[async_trait]
impl VideoManagePort for VideoManageAdapter {
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

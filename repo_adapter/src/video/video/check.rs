// repo_adapter/src/video/video/check.rs
// 🔌 插头 - 可乐视频 - 视频 - 检查
// 2026/8/6 19:19 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::gis::command::poi::PoiCommand;
use cola_data::gis::port::add::AddPort;
use cola_data::video::info::video::VideoInfo;
use cola_data::video::port::video::check::VideoCheckPort;
////////

/// # [CHECK ADAPTER] - 检查
/// * `desc`: `🔌 视频检查服务`
pub struct VideoCheckAdapter;

#[async_trait]
impl VideoCheckPort for VideoCheckAdapter {
    async fn check_health(
        &self,
        uid: i64,
        keyword: Option<String>,
        limit: i64,
        offset: i64,
        is_liked: bool,
    ) -> Result<(Vec<VideoInfo>)> {
        todo!()
    }

    async fn check_state(
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

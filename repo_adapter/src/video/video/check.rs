// repo_adapter/src/cola_video/cola_video/check.rs -- ADAPTER - VIDEO - CONTENT - 检查适配器
// 2026/8/6 19:19 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_video::video::check::VideoCheckPort;

////////

/// # [CHECK ADAPTER] - 视频内容检查适配器
/// * `desc`: `VIDEO - Content Check Adapter.`
pub struct VideoCheckAdapter;

#[async_trait]
impl VideoCheckPort for VideoCheckAdapter {
    async fn check_health(&self, video_id: i64) -> Result<(bool)> {
        todo!()
    }

    async fn check_state(&self, video_id: i64) -> Result<(bool)> {
        todo!()
    }

    async fn is_owner(&self, uid: i64, video_id: i64) -> Result<(bool)> {
        todo!()
    }
}

//////// END

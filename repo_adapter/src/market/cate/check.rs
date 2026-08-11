// repo_adapter/src/cola_video/cola_video/check.rs
// 🔌 插头 - 可乐视频 - 视频 - 检查
// 2026/8/6 19:19 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_video::video::check::VideoCheckPort;
use port::market::cate::check::CateCheckPort;
////////

/// # [CHECK ADAPTER] - 检查
/// * `desc`: `🔌 视频检查服务`
pub struct CateCheckAdapter;

#[async_trait]
impl CateCheckPort for CateCheckAdapter {
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

// repo_adapter/src/video/share/check.rs
// 🔌 插头 - ▶ 视频 - 分享 - 检查
// 2026/8/6 18:57 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_video::share::check::VideoShareCheckPort;

////////

/// # [ADAPTER] - share check
#[derive(Debug, Default, Clone)]
pub struct VideoShareCheckAdapter;

#[async_trait]
impl VideoShareCheckPort for VideoShareCheckAdapter {
    async fn health(&self, uid: i64, video_id: i64) -> Result<()> {
        todo!()
    }

    async fn state(&self, uid: i64, video_id: i64) -> Result<()> {
        todo!()
    }
}

//////// END

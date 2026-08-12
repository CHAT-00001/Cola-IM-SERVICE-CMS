// repo_adapter/src/cola_video/hotlist/check.rs
// 🔌 插头 - 可乐视频 - 上热门 - 检查
// 2026/8/6 19:04 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_video::hotlist::check::VideoHotlistCheckPort;

////////

/// # [CHECK ADAPTER] - hotlist check
#[derive(Debug, Default, Clone)]
pub struct VideoHotlistCheckAdapter;

#[async_trait]
impl VideoHotlistCheckPort for VideoHotlistCheckAdapter {
    async fn health(&self, uid: i64, collect_id: i64) -> Result<()> {
        todo!()
    }

    async fn state(&self, uid: i64, collect_id: i64) -> Result<()> {
        todo!()
    }
}

//////// END

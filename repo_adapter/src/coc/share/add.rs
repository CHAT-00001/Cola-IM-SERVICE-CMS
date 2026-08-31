// repo_adapter/src/cola_video/share/add.rs
// 🔌 插头 - 可乐视频 - 分享 - 发布
// 2026/8/6 18:57 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::command::share::ShareCommand;
use port::cola_video::share::add::VideoShareAddPort;

////////

/// # [ADD ADAPTER] - share add
/// * `DESC`: `▶ 视频 - 视频分享发布适配器`
#[derive(Debug, Default, Clone)]
pub struct VideoShareAddAdapter;

#[async_trait]
impl VideoShareAddPort for VideoShareAddAdapter {
    async fn save_share_record(&self, uid: i64, video_id: i64, cmd: ShareCommand) -> Result<()> {
        todo!()
    }

    async fn delete_share_record(&self, uid: i64, video_id: i64) -> Result<()> {
        todo!()
    }
}

//////// END

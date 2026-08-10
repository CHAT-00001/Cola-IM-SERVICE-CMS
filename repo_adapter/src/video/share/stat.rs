// repo_adapter/src/cola_video/share/stat.rs
// 🔌 适配器 - ▶ 视频 - 分享记录 - 统计
// 2026/8/6 20:14 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_video::share::stat::VideoShareStatPort;

////////

/// # [STAT ADAPTER] - share stat
/// * `DESC`: `▶ 视频 - 视频分享记录统计数量适配器`
#[derive(Debug, Default, Clone)]
pub struct VideoShareStatAdapter;

#[async_trait]
impl VideoShareStatPort for VideoShareStatAdapter {
    async fn stat_count_by_user_id(&self, uid: i64, user_id: i64) -> Result<(u64)> {
        todo!()
    }

    async fn stat_count_by_video_id(&self, uid: i64, video_id: i64) -> Result<(u64)> {
        todo!()
    }
}

//////// END

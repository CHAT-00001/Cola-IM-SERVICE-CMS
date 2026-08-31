// repo_adapter/src/cola_video/danmaku/stat.rs
// 🔌 适配器 - VIDEO - 弹幕 - 统计
// 2026/8/6 19:28 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_video::danmaku::stat::VideoDanmakuStatPort;

////////

/// # [STAT ADAPTER] - danmaku 统计
#[derive(Debug, Default, Clone)]
pub struct VideoDanmakuStatAdapter;

#[async_trait]
impl VideoDanmakuStatPort for VideoDanmakuStatAdapter {
    async fn stat_count_by_user_id(&self, uid: i64, user_id: i64) -> Result<(u64)> {
        todo!()
    }

    async fn stat_count_by_video_id(&self, uid: i64, video_id: i64) -> Result<(u64)> {
        todo!()
    }
}

//////// END

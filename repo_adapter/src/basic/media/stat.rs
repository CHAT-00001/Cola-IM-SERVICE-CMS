// repo_adapter/src/fs/media/stat.rs
// 🔌 适配器 - FS - 媒体文件 - 统计
// 2026/8/14 15:00 Updated.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::fs::media::stat::MediaStatPort;

////////

/// # [STAT ADAPTER] - 统计
/// * `desc`: `FS - 媒体文件统计适配器`
#[derive(Debug, Default, Clone)]
pub struct MediaStatAdapter;

#[async_trait]
impl MediaStatPort for MediaStatAdapter {
    ////////

    /// # 1. [ADAPTER] - 用户媒体统计
    async fn stat_user_media_count(&self, _uid: i64) -> Result<u64> {
        // TODO: 实现用户媒体统计逻辑
        todo!("stat_user_media_count")
    }

    ////////

    /// # 2. [ADAPTER] - 应用媒体统计
    async fn stat_app_media_count(&self, _app_id: String) -> Result<u64> {
        // TODO: 实现应用媒体统计逻辑
        todo!("stat_app_media_count")
    }
}

// repo_adapter/src/fs/bucket/stat.rs
// 🔌 适配器 - FS - 存储桶 -统计
// 2026/8/6 19:18 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::fs::bucket::stat::BucketStatPort;

////////

/// # [STAT ADAPTER] - 统计
/// * `desc`: `FS - 存储桶统计适配器`
#[derive(Debug, Default, Clone)]
pub struct BucketStatAdapter;

#[async_trait]
impl BucketStatPort for BucketStatAdapter {
    async fn stat_count_by_user_id(&self, uid: i64, user_id: i64) -> Result<(u64)> {
        todo!()
    }

    async fn stat_count_by_video_id(&self, uid: i64, video_id: i64) -> Result<(u64)> {
        todo!()
    }
}

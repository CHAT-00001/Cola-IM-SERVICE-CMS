// repo_adapter/src/aaaa/app/stat.rs -- ADAPTER - BASIC - 应用 - 统计服务
// 2026/8/6 19:18 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::basic::app::stat::AppStatPort;

////////

/// # [STAT ADAPTER] - 统计
/// * `desc`: `COLA BASIC - 存储桶统计适配器`
#[derive(Debug, Default, Clone)]
pub struct AppStatAdapter;

#[async_trait]
impl AppStatPort for AppStatAdapter {
    async fn stat_count_by_user_id(&self, uid: i64, user_id: i64) -> Result<(u64)> {
        todo!()
    }

    async fn stat_count_by_video_id(&self, uid: i64, video_id: i64) -> Result<(u64)> {
        todo!()
    }
}

//////// END

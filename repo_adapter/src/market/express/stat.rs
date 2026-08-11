// repo_adapter/src/cola_video/cola_video/stat.rs
// 🔌 插头服务 - 可乐视频 - 视频 - 统计服务
// 2026/8/6 19:20 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_video::video::stat::VideoStatPort;

////////

/// # [STAT SERVICE] - 统计
/// * `desc`: `🔌 视频统计服务`
pub struct VideoStatAdapter;

// 构造实现
#[async_trait]
impl VideoStatPort for VideoStatAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 统计用户的视频数量
    async fn stat_count_by_user_id(&self, uid: i64, user_id: i64) -> Result<(u64)> {
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] - 统计所有视频数量
    async fn stat_count(&self) -> Result<(u64)> {
        todo!()
    }
}

//////// END

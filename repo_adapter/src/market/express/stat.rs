// repo_adapter/src/cola_video/cola_video/stat.rs
// 🔌 插头服务 - 可乐视频 - 视频 - 统计服务
// 2026/8/6 19:20 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::market::express::stat::ExpressStatPort;
use port::cola_video::video::stat::VideoStatPort;

////////

/// # [STAT SERVICE] - 统计
/// * `desc`: `🔌 视频统计服务`
pub struct ExpressStatAdapter;

// 构造实现
#[async_trait]
impl ExpressStatPort for ExpressStatAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 统计用户的视频数量
    async fn stat_count(&self) -> Result<(u64)> {
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] - 统计所有视频数量
    async fn stat_count_by_status_code(&self, status_code: i16) -> Result<(u64)> {
        todo!()
    }
}

//////// END

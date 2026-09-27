// repo_adapter/src/video/view/stat.rs -- ADAPTER  - VIDEO - 浏览 - 统计
// 2026/8/6 19:18 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_video::view::stat::VideoViewStatPort;

////////

/// # [STAT ADAPTER] - 视频浏览统计服务
/// * `desc`: `VIDEO - View Stat Adapter.`
pub struct VideoViewStatAdapter;

// 构造实现
#[async_trait]
impl VideoViewStatPort for VideoViewStatAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 用户主动浏览数量
    async fn stat_count_by_user_id(&self, user_id: i64) -> Result<(u64)> {
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] - 视频被动浏览数量
    async fn stat_count_by_video_id(&self, video_id: i64) -> Result<(u64)> {
        todo!()
    }
}

//////// END

// repo_adapter/src/video/like/check.rs
// 🔌 适配器 - ▶ 视频 - 点赞 - 检查
// 2026/8/8 13:05 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_video::like::check::VideoLikeCheckPort;

////////

/// # [CHECK ADAPTER] - like check
/// * `desc`: `▶ 视频 - 点赞检查Adapter implementation`
#[derive(Debug, Default, Clone)]
pub struct VideoLikeCheckAdapter;

#[async_trait]
impl VideoLikeCheckPort for VideoLikeCheckAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 健康
    async fn health(&self, uid: i64, collect_id: i64) -> Result<()> {
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] - 健康
    async fn state(&self, uid: i64, collect_id: i64) -> Result<()> {
        todo!()
    }
}

//////// END

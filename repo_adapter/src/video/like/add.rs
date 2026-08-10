// repo_adapter/src/cola_video/like/add.rs
// 🔌 插头 - 可乐视频 - 点赞 - 发布
// 2026/8/6 18:57 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_video::like::add::VideoLikeAddPort;

////////

/// # [ADAPTER] - like add
/// * `desc`: `▶ 视频 - 视频点赞发布 Adapter implementation`
#[derive(Debug, Default, Clone)]
pub struct VideoLikeAddAdapter;

// 构造实现
#[async_trait]
impl VideoLikeAddPort for VideoLikeAddAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 点赞发布
    async fn send_like(
        &self,
        uid: i64,       // UID
        video_id: i64,  // 视频 ID
        is_liked: bool, // 是否点赞
    ) -> Result<()> {
        todo!()
    }
}

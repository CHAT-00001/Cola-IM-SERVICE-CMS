// repo_adapter/src/cola_video/like/get.rs
// 🔌 适配器 - ▶ 视频 - 点赞记录 - 获取
// 2026/8/6 18:57 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_video::like::get::VideoLikeGetPort;

////////

/// # [ADAPTER] - like get
/// * desc: Adapter implementation
#[derive(Debug, Default, Clone)]
pub struct VideoLikeGetAdapter;

#[async_trait]
impl VideoLikeGetPort for VideoLikeGetAdapter {
    async fn get_my_like_ids(&self, uid: i64, limit: i64, offset: i64) -> Result<(Vec<i64>)> {
        todo!()
    }

    async fn get_he_like_ids(&self, uid: i64, limit: i64, offset: i64) -> Result<(u16)> {
        todo!()
    }
}

//////// END

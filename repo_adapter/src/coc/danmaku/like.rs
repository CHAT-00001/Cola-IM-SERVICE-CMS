// repo_adapter/src/cola_video/danmaku/like.rs
// 🔌 适配器 - VIDEO - 弹幕 - 弹幕点赞
// 2026/8/6 19:17 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_video::danmaku::like::VideoDanmakuLikePort;

////////

/// # [LIKE ADAPTER] - danmaku 点赞
/// * `desc`: `VIDEO - 视频弹幕点赞适配器`
#[derive(Debug, Default, Clone)]
pub struct VideoDanmakuLikeAdapter;

#[async_trait]
impl VideoDanmakuLikePort for VideoDanmakuLikeAdapter {
    async fn upsert_like(&self, uid: i64, comment_id: i64, is_liked: bool) -> Result<(bool)> {
        todo!()
    }

    async fn check_state(&self, uid: i64, comment_id: i64) -> Result<()> {
        todo!()
    }
}

//////// END

// repo_adapter/src/cola_video/danmaku/dislike.rs
// 🔌 插头 - VIDEO - 弹幕 - 不喜欢弹幕
// 2026/8/6 19:28 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_video::danmaku::dislike::VideoDanmakuDislikePort;

////////

/// # [DISLIKE ADAPTER] - danmaku 不喜欢
#[derive(Debug, Default, Clone)]
pub struct VideoDanmakuDislikeAdapter;

#[async_trait]
impl VideoDanmakuDislikePort for VideoDanmakuDislikeAdapter {
    async fn upsert_like_record(&self, uid: i64, comment_id: i64, is_liked: bool) -> Result<()> {
        todo!()
    }

    async fn check_like_state(&self, uid: i64, comment_id: i64) -> Result<()> {
        todo!()
    }
}

//////// END

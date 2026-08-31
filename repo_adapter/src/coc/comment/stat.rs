// repo_adapter/src/video/comment/stat.rs
// 🔌 插头 - 可乐视频 - 评论 - 统计
// 2026/8/6 19:18 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_video::comment::stat::VideoCommentStatPort;

////////

/// # [STAT ADAPTER] - 发布
/// * `desc`: `VIDEO - 视频评论统计适配器`
#[derive(Debug, Default, Clone)]
pub struct VideoCommentStatAdapter;

#[async_trait]
impl VideoCommentStatPort for VideoCommentStatAdapter {
    async fn stat_count_by_user_id(&self, uid: i64, user_id: i64) -> Result<(u64)> {
        todo!()
    }

    async fn stat_count_by_video_id(&self, uid: i64, video_id: i64) -> Result<(u64)> {
        todo!()
    }
}

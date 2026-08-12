// video/comment/del.rs
// 🔌 适配器 - VIDEO - 评论 - 逻辑删除
// 2026/8/6 19:12 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_video::comment::del::VideoCommentDelPort;

////////

/// # [ADD SERVICE] - 发布
/// * `desc`: `VIDEO` - `评论删除适配器`
#[derive(Debug, Default, Clone)]
pub struct VideoCommentDelAdapter;

#[async_trait]
impl VideoCommentDelPort for VideoCommentDelAdapter {
    async fn single_delete(&self, id: i64) -> Result<(u16)> {
        todo!()
    }

    async fn batch_delete(&self, ids: Vec<i64>) -> Result<(u16)> {
        todo!()
    }
}

//////// END
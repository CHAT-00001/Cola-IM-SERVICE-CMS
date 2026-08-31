// repo_adapter/src/video/hotlist/del.rs
// 🔌 适配器 - ▶ 视频 - 推荐 - 删除
// 2026/8/10 02:22 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_video::recommend::del::VideoRecommendDelPort;

////////

/// # [ADAPTER] - hotlist del
/// * `DESC`: `▶ 视频 - 视频推荐删除`
#[derive(Debug, Default, Clone)]
pub struct VideoRecommendDelAdapter;

#[async_trait]
impl VideoRecommendDelPort for VideoRecommendDelAdapter {
    async fn single_soft_del_record(&self, uid: i64, video_id: i64, id: i64) -> Result<(u16)> {
        todo!()
    }

    async fn batch_soft_del_record(&self, uid: i64, video_id: i64, ids: Vec<i64>) -> Result<(u16)> {
        todo!()
    }
}

//////// END

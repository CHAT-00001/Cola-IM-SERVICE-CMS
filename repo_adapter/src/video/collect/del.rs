// repo_adapter/src/video/collect/del.rs
// 🔌 适配器 - ▶ 视频 - 收藏 - 删除
// 2026/8/9 20:37 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_video::collect::del::VideoCollectDelPort;

////////

/// # [DELETE ADAPTER] - 删除
/// * `desc`: `▶ 视频 - 收藏记录软删除适配器`
#[derive(Debug, Default, Clone)]
pub struct VideoCollectDelAdapter;

#[async_trait]
impl VideoCollectDelPort for VideoCollectDelAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 单个
    /// * `desc`: `根据单个ID` - `单个删除记录`
    async fn single_soft_del_record(&self, uid: i64, video_id: i64, id: i64) -> Result<(u64)> {
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] - 批量
    /// * `desc`: `根据批量IDs` - `批量删除记录`
    async fn batch_soft_del_record(&self, uid: i64, video_id: i64, ids: Vec<i64>) -> Result<(u64)> {
        todo!()
    }
}

//////// END

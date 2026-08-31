// repo_adapter/src/cola_video/hotlist/del.rs
// 🔌 插头 - 可乐视频 - 上热门 - 删除
// 2026/8/6 19:04 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_video::hotlist::del::VideoHotlistDelPort;

////////

/// # [ADAPTER] - hotlist del
#[derive(Debug, Default, Clone)]
pub struct VideoHotlistDelAdapter;

#[async_trait]
impl VideoHotlistDelPort for VideoHotlistDelAdapter {
    async fn single_soft_del_record(&self, uid: i64, video_id: i64, id: i64) -> Result<(u16)> {
        todo!()
    }

    async fn batch_soft_del_record(&self, uid: i64, video_id: i64, ids: Vec<i64>) -> Result<(u16)> {
        todo!()
    }
}

//////// END

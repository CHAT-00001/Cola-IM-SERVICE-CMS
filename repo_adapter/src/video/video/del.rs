// repo_adapter/src/cola_video/cola_video/del.rs
// 🔌 插头 - 可乐视频 - 视频 - 删除服务
// 2026/8/6 19:19 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::port::video::del::VideoDelPort;

////////

/// # [DEL ADAPTER] - 删除
/// * `desc`: `🔌 视频删除服务`
pub struct VideoDelAdapter;

#[async_trait]
impl VideoDelPort for VideoDelAdapter {
    async fn single_soft_del(&self, uid: i64, id: i64) -> Result<(u16)> {
        todo!()
    }

    async fn batch_soft_del(&self, uid: i64, ids: Vec<i64>) -> Result<(u16)> {
        todo!()
    }
}

//////// END

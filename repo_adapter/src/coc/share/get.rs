// repo_adapter/src/cola_video/share/get.rs
// 🔌 插头 - 可乐视频 - 分享 - 获取
// 2026/8/6 18:57 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::command::share::ShareCommand;
use port::cola_video::share::get::VideoShareGetPort;

////////

/// # [GET ADAPTER] - share get
/// * `DESC`: `▶ 视频 - 视频分享记录获取适配器`
#[derive(Debug, Default, Clone)]
pub struct VideShareGetAdapter;

#[async_trait]
impl VideoShareGetPort for VideShareGetAdapter {
    async fn single_soft_del_record(
        &self,
        uid: i64,
        video_id: i64,
        id: i64,
        cmd: ShareCommand,
    ) -> Result<(u16)> {
        todo!()
    }

    async fn batch_soft_del_record(
        &self,
        uid: i64,
        video_id: i64,
        ids: Vec<i64>,
        cmd: ShareCommand,
    ) -> Result<(u16)> {
        todo!()
    }
}

//////// END

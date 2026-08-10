// repo_adapter/src/video/share/del.rs
// 🔌 插头 - ▶ 视频 - 分享 - 删除
// 2026/8/6 18:57 Created.

////////
use anyhow::Result;
use async_trait::async_trait;
use port::cola_video::share::del::VideoShareDelPort;

////////

/// # [ADAPTER] - share del
/// * `DESC`: `▶ 视频 - 视频分享记录删除适配器`
#[derive(Debug, Default, Clone)]
pub struct VideoShareDelAdapter;

#[async_trait]
impl VideoShareDelPort for VideoShareDelAdapter {
    async fn single_soft_del_record(&self, uid: i64, video_id: i64, id: i64) -> Result<(u16)> {
        todo!()
    }

    async fn batch_soft_del_record(&self, uid: i64, video_id: i64, ids: Vec<i64>) -> Result<(u16)> {
        todo!()
    }
    // TODO: 瀹炵幇鍏蜂綋鐨勪笟鍔￠€昏緫
}

//////// END

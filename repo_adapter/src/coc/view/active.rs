// repo_adapter/src/cola_video/view/active.rs
// 🔌 插头 - 可乐视频 - 浏览 - 活跃
// 2026/8/6 19:01 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_video::view::active::VideoViewActivePort;

////////

/// # [ALIVE SERVICE] - 存活
/// * `desc`: `用户浏览存活服务`
pub struct VideoViewActiveAdapter;

// 构造实现
#[async_trait]
impl VideoViewActivePort for VideoViewActiveAdapter {
    //

    ////////

    /// # 1. [SERVICE] - 保存浏览记录
    /// * `desc`: `单个软删除`
    async fn save_view(&self, uid: i64, video_id: i64) -> Result<()> {
        todo!()
    }

    async fn update_done_count(&self, uid: i64, video_id: i64, is_done: bool) -> Result<()> {
        todo!()
    }
}

//////// END

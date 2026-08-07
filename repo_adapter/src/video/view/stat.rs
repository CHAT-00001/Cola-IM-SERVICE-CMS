// repo_adapter/src/cola_video/view/stat.rs
// 🔌 插头 - 可乐视频 - 浏览 - 统计
// 2026/8/6 19:18 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::port::view::stat::VideoViewStatPort;

////////

/// # [STAT SERVICE] - 统计
/// * `desc`: `视频浏览统计服务`
pub struct ViewStatService;

// 构造实现
#[async_trait]
impl VideoViewStatPort for ViewStatService {
    //

    ////////

    /// # 1. [SERVICE] - 单个
    /// * `desc`: `单个软删除`
    async fn save_view(&self, uid: i64, video_id: i64) -> Result<()> {
        todo!()
    }

    async fn update_done_count(&self, uid: i64, video_id: i64, is_done: bool) -> Result<()> {
        todo!()
    }
}

//////// END

// repo_adapter/src/video/share/add.rs
// 🔌 插头 - 可乐视频 - 分享 - 发布
// 2026/8/6 18:57 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::video::port::view::add::VideoViewAddPort;

////////

/// # [ADD SERVICE] - 发布
/// * `desc`: `视频分享发布服务`
pub struct AddService;

// 构造实现
#[async_trait]
impl VideoViewAddPort for AddService {
    //

    ////////

    /// # 1. [SERVICE] - 发布
    /// * `desc`: `保存视频浏览记录`
    async fn save_view(&self, uid: i64, video_id: i64) -> Result<()> {
        todo!()
    }

    async fn update_done_count(&self, uid: i64, video_id: i64, is_done: bool) -> Result<()> {
        todo!()
    }
}

//////// END

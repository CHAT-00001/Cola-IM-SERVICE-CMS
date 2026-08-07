// repo_adapter/src/video/view/active.rs
// 🔌 插头 - 可乐视频 - 浏览 - 活跃
// 2026/8/6 19:01 Created.

////////


// repo_adapter/src/user/ban/del.rs
// 🔌 适配器 - 可乐用户 - 浏览 - 删除服务
// 2026/8/7 05:56 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::video::port::view::active::{VideoViewActivePort};
////////

/// # [ALIVE SERVICE] - 存活
/// * `desc`: `用户浏览存活服务`
pub struct AliveService;

// 构造实现
#[async_trait]
impl VideoViewActivePort for AliveService {
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

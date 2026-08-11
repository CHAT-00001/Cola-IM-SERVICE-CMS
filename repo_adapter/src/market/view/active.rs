// repo_adapter/src/market/view/active.rs
// 🔌 插头 - MARKET - 浏览 - 活跃
// 2026/8/6 19:01 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::market::view::active::GoodsViewActivePort;

////////

/// # [ALIVE SERVICE] - 存活
/// * `desc`: `用户浏览存活服务`
pub struct AliveService;

// 构造实现
#[async_trait]
impl GoodsViewActivePort for AliveService {
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

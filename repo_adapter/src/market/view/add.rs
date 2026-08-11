// repo_adapter/src/market/view/add.rs
// 🔌 插头 - MARKET - Goods - 发布
// 2026/8/6 19:00 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::market::view::add::GoodsViewAddPort;

////////

/// # [ADD ADAPTER] - 浏览 发布
/// * `desc`: `VIDEO - 视频浏览发布适配器`
pub struct GoodsViewAddAdapter;

// 构造实现
#[async_trait]
impl GoodsViewAddPort for GoodsViewAddAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 保存浏览记录
    /// * `desc`: `发布视频`
    async fn save_view(&self, uid: i64, video_id: i64) -> Result<()> {
        todo!()
    }

    /// # 1. [ADAPTER] - 更新浏览记录
    /// * `desc`: `发布视频`
    async fn update_done_count(&self, uid: i64, video_id: i64, is_done: bool) -> Result<()> {
        todo!()
    }
}

//////// END

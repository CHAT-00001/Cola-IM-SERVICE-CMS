// repo_adapter/src/market/collec/check.rs
// 🔌 适配器 - MARKET - 商品收藏 - Check Port
// 2026/8/9 20:36 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_video::collect::check::VideoCollectCheckPort;
use port::market::collect::check::GoodsCollectCheckPort;
////////

/// # [CHECK ADAPTER] - 检查
/// * `desc`: `商品收藏状态检查`
#[derive(Debug, Default, Clone)]
pub struct GoodsCollectCheckAdapter;

// 构造实现
#[async_trait]
impl GoodsCollectCheckPort for GoodsCollectCheckAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 收藏
    /// * `desc`: `用户收藏视频`
    async fn health(
        &self,
        uid: i64,        // 操作者 ID
        collect_id: i64, // 收藏 ID
    ) -> Result<()> {
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] - 收藏
    /// * `desc`: `用户收藏视频`
    async fn state(
        &self,
        uid: i64,        // 操作者 ID
        collect_id: i64, // 收藏 ID
    ) -> Result<()> {
        todo!()
    }
}

//////// END

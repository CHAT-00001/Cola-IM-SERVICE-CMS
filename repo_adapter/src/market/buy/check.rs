// repo_adapter/src/market/goods/buy/checkt.rs
// 🔌 适配器 - MARKET - 商品购买 - 检查
// 2026/8/8 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::market::buy::check::GoodsBuyCheckPort;

////////

/// # [CHECK ADAPTER] - 检查
/// * `desc`: `MARKET - 检查商品购买记录状态`
#[derive(Debug, Default, Clone)]
pub struct GoodsBuyCheckAdapter;

#[async_trait]
impl GoodsBuyCheckPort for GoodsBuyCheckAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 检查购买状态
    /// * `desc`: 检查指定购买记录的状态
    async fn is_bought(
        &self,
        user_id: i64,  // 用户 ID
        video_id: i64, // 视频 ID
    ) -> Result<bool, anyhow::Error> {
        Ok(false)
    }
}

//////// END

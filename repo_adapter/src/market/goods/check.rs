// repo_adapter/src/market/goods/check.rs
// 🔌 适配器 - MARKET - GOODS - 状态检查
// 2026/8/6 解耦: 检查商品是否存在/是否可上架等校验

////////

use anyhow::Result;
use async_trait::async_trait;
use port::market::goods::check::GoodsCheckPort;

////////
/// # [CHECK ADAPTER] - 商品 检查
/// `desc`: `MARKET - 商品检查适配器`
pub struct GoodsCheckAdapter;

#[async_trait]
impl GoodsCheckPort for GoodsCheckAdapter {
    async fn check_health(&self, goods_id: i64) -> Result<(bool)> {
        todo!()
    }

    async fn check_state(&self, goods_id: i64) -> Result<(bool)> {
        todo!()
    }

    async fn is_owner(&self, uid: i64, goods_id: i64) -> Result<(bool)> {
        todo!()
    }
}

//////// END
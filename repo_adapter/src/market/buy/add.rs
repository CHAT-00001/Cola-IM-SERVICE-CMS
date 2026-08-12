// repo_adapter/src/market/buy/add.rs
// 🔌 适配器 - MARKET - 购买 - ADD 服务
// 2026/8/8 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::market::buy::add::GoodsBuyAddPort;

////////

/// # [ADAPTER] - 商品购买添加
/// * `desc`: 实现商品购买记录的保存和管理
#[derive(Debug, Default, Clone)]
pub struct GoodsBuyAddAdapter;

#[async_trait]
impl GoodsBuyAddPort for GoodsBuyAddAdapter {
    ////////

    /// # [ADAPTER] - 添加订单
    async fn add_order(&self, _uid: i64, _goods_id: i64) -> Result<()> {
        todo!()
    }

    ////////

    /// # [ADAPTER] - 删除订单
    async fn del_order(&self, _uid: i64, _goods_id: i64) -> Result<()> {
        todo!()
    }
}

//////// END

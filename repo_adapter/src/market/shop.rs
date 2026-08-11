// repo_adapter/src/market/shop.rs
// 🔌 适配器 - MARKET - SHOP
// 2026/6/18

////////

use async_trait::async_trait;
use cola_data::cola_market::vo::shop_apply::ShopApplyVo;
use cola_data::cola_market::vo::shop_apply_history::ShopApplyHistoryVo;
use port::cola_market::shop::manage::ShopManagePort;

////////

/// # [SHOP ADAPTER] - 商店
/// * `desc`: `MARKET - SHOP 商店适配器`
pub struct ShopManageAdapter;

#[async_trait]
impl ShopManagePort for ShopManageAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 获取申请列表
    async fn get_apply(
        &self,
        _uid: i64,    // UID
        _limit: i64,  // 数量
        _offset: i64, // 页码
    ) -> anyhow::Result<ShopApplyVo> {
        Err(anyhow::anyhow!("not implemented"))
    }

    ////////

    /// # 2. [ADAPTER] - 获取申请历史
    async fn get_apply_history(
        &self,
        _uid: i64,     // UID
        _shop_id: i64, // 商店 ID
        _limit: i64,   // 数量
        _offset: i64,  // 页码
    ) -> anyhow::Result<ShopApplyHistoryVo> {
        Err(anyhow::anyhow!("not implemented"))
    }

    ////////

    /// # 3. [ADAPTER] - 审核申请
    async fn review_apply(&self, _uid: i64, _shop_id: i64, _reason: String) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("not implemented"))
    }

    ////////

    /// # 4. [ADAPTER] - 终止审核
    async fn abort_apply(&self, _uid: i64, _shop_id: i64, _reason: String) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("not implemented"))
    }

    ////////

    /// # 5. [ADAPTER] - 修改状态
    async fn change_status(&self, _uid: i64, _shop_id: i64, _status: i16) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("not implemented"))
    }

    ////////

    /// # 6. [ADAPTER] - 批量删除
    async fn batch_delete(&self, _uid: i64, _shop_ids: Vec<i64>) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("not implemented"))
    }
}

//////// END

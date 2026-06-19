// repo_adapter/src/market/shop.rs  -- 适配器 - 店铺管理
// 2026/6/18

//////

use async_trait::async_trait;
use cola_data::market::port::shop_manage::ShopManagePort;
use cola_data::market::vo::shop_apply::ShopApplyVo;
use cola_data::market::vo::shop_apply_history::ShopApplyHistoryVo;

//////

/// # [ADAPTER] - 店铺管理 端口适配器
pub struct ShopManageAdapter;

#[async_trait]
impl ShopManagePort for ShopManageAdapter {

    async fn get_apply(&self, _uid: i64, _offset: i64, _limit: i64) -> anyhow::Result<ShopApplyVo> {
        Err(anyhow::anyhow!("not implemented"))
    }

    async fn get_apply_history(&self, _uid: i64, _shop_id: i64, _offset: i64, _limit: i64) -> anyhow::Result<ShopApplyHistoryVo> {
        Err(anyhow::anyhow!("not implemented"))
    }

    async fn review_apply(&self, _uid: i64, _shop_id: i64, _reason: String) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("not implemented"))
    }

    async fn abort_apply(&self, _uid: i64, _shop_id: i64, _reason: String) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("not implemented"))
    }

    async fn change_status(&self, _uid: i64, _shop_id: i64, _status: i16) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("not implemented"))
    }

    async fn batch_delete(&self, _uid: i64, _shop_ids: Vec<i64>) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("not implemented"))
    }
}

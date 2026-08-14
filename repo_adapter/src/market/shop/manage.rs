// repo_adapter/src/market/shop/manage.rs
// 🔌 插头 - MARKET - SHOP - 管理
// 2026/8/6 19:19 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::market::vo::shop_apply::ShopApplyVo;
use cola_data::market::vo::shop_apply_history::ShopApplyHistoryVo;
use port::market::shop::manage::ShopManagePort;

////////

/// # [MANAGE ADAPTER] - 管理
/// * `desc`: `MARKET - 商店关联适配器`
pub struct ShopManageAdapter;

#[async_trait]
impl ShopManagePort for ShopManageAdapter {
    //

    ////////
    /// # [ADAPTER] - 管理员列表
    async fn get_apply(&self, uid: i64, offset: i64, limit: i64) -> Result<(ShopApplyVo)> {
        todo!()
    }

    async fn get_apply_history(
        &self,
        uid: i64,
        shop_id: i64,
        offset: i64,
        limit: i64,
    ) -> Result<(ShopApplyHistoryVo)> {
        todo!()
    }

    async fn review_apply(&self, uid: i64, shop_id: i64, reason: String) -> Result<()> {
        todo!()
    }

    async fn abort_apply(&self, uid: i64, shop_id: i64, reason: String) -> Result<()> {
        todo!()
    }

    async fn change_status(&self, uid: i64, shop_id: i64, status: i16) -> Result<()> {
        todo!()
    }

    async fn batch_delete(&self, uid: i64, shop_ids: Vec<i64>) -> Result<()> {
        todo!()
    }
}

//////// END

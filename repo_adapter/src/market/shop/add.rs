// repo_adapter/src/market/shop/add.rs
// 🔌 适配器 - MARKET - 商店 - 发布
// 2026-06-12 10:52 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::market::command::shop::add::CreatedShopApplyCommand;
use port::market::shop::add::ShopAddPort;

////////

/// # [ADD ADAPTER] - 发布
/// * `desc`: `MARKET -  商品发布适配器`
pub struct ShopAddAdapter;

#[async_trait]
impl ShopAddPort for ShopAddAdapter {
    async fn save_shop_appy_and_send_event(
        &self,
        uid: i64,
        cmd: CreatedShopApplyCommand,
    ) -> Result<()> {
        todo!()
    }

    async fn update_shop_apply_and_send_event(
        &self,
        uid: i64,
        shop_id: i64,
        cmd: CreatedShopApplyCommand,
    ) -> Result<()> {
        todo!()
    }

    async fn review_shop_apply_and_send_event(
        &self,
        uid: i64,
        shop_id: i64,
        cmd: CreatedShopApplyCommand,
    ) -> Result<()> {
        todo!()
    }

    async fn reject_shop_apply_and_send_event(
        &self,
        uid: i64,
        shop_id: i64,
        cmd: CreatedShopApplyCommand,
    ) -> Result<()> {
        todo!()
    }

    async fn admin_del_one_shop_and_send_event(&self, uid: i64, shop_id: i64) -> Result<()> {
        todo!()
    }

    async fn admin_batch_del_many_shops_and_send_event(
        &self,
        uid: i64,
        shop_ids: Vec<i64>,
    ) -> Result<()> {
        todo!()
    }
    //
}

//////// END

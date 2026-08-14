// repo_adapter/src/market/shop/appy.rs
// 🔌 适配器 - MARKET - 店铺 - 申请
// 2026/8/11 05:43 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::market::command::shop::add::CreatedShopApplyCommand;
use port::market::shop::appy::ShopAppyPort;

////////

/// # [APPY ADAPTER] - 店铺申请
/// * `desc`: `MARKET - 商店申请适配器`
pub struct ShopAppyAdapter;

#[async_trait]
impl ShopAppyPort for ShopAppyAdapter {
    ////////

    /// # 1. [ADAPTER] - 保存商店申请 + 发送事件
    async fn save_shop_appy_and_send_event(
        &self,
        _uid: i64,
        _cmd: CreatedShopApplyCommand,
    ) -> Result<()> {
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] - 编辑商店申请 + 发送事件
    async fn update_shop_apply_and_send_event(
        &self,
        _uid: i64,
        _shop_id: i64,
        _cmd: CreatedShopApplyCommand,
    ) -> Result<()> {
        todo!()
    }

    ////////

    /// # 3. [ADAPTER] - 审核商店申请 + 发送事件
    async fn review_shop_apply_and_send_event(
        &self,
        _uid: i64,
        _shop_id: i64,
        _cmd: CreatedShopApplyCommand,
    ) -> Result<()> {
        todo!()
    }

    ////////

    /// # 4. [ADAPTER] - 驳回商店申请 + 发送事件
    async fn reject_shop_apply_and_send_event(
        &self,
        _uid: i64,
        _shop_id: i64,
        _cmd: CreatedShopApplyCommand,
    ) -> Result<()> {
        todo!()
    }

    ////////

    /// # 5. [ADAPTER] - 管理员删除一个商店
    async fn admin_del_one_shop_and_send_event(&self, _uid: i64, _shop_id: i64) -> Result<()> {
        todo!()
    }

    ////////

    /// # 6. [ADAPTER] - 管理员批量删除多个商店
    async fn admin_batch_del_many_shops_and_send_event(
        &self,
        _uid: i64,
        _shop_ids: Vec<i64>,
    ) -> Result<()> {
        todo!()
    }
}

//////// END

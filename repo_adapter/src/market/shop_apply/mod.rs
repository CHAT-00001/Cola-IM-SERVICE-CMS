// repo_adapter/src/market/shop_apply/mod.rs
// 🔌 适配器 - MARKET - SHOP
// 2026/6/18 14:20 Created.

////////

use async_trait::async_trait;
use cola_data::market::command::shop::add::CreatedShopApplyCommand;
use port::cola_market::shop::add::ShopAppyPort;

////////

pub mod add;
pub mod check;
pub mod del;
pub mod get;
pub mod list;
pub mod manage;

////////

/// # [SHOP_APPLY ADAPTER] - 商店申请
/// * `desc`: `MARKET - SHOP_APPLY 适配器`
pub struct ShopAppyAdapter;

#[async_trait]
impl ShopAppyPort for ShopAppyAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 保存商店申请
    async fn save_shop_appy_and_send_event(
        &self,
        uid: i64,                     // 用户ID
        cmd: CreatedShopApplyCommand, // 申请命令
    ) -> anyhow::Result<()> {
        add::save_shop_apply(uid, cmd).await
    }

    ////////

    /// # 2. [ADAPTER] - 编辑商店申请
    async fn update_shop_apply_and_send_event(
        &self,
        uid: i64,                     // 用户ID
        shop_id: i64,                 // 店铺ID
        cmd: CreatedShopApplyCommand, // 申请命令
    ) -> anyhow::Result<()> {
        add::update_shop_apply(uid, shop_id, cmd).await
    }

    ////////

    /// # 3. [ADAPTER] - 审核商店申请
    async fn review_shop_apply_and_send_event(
        &self,
        uid: i64,                     // 管理员ID
        shop_id: i64,                 // 店铺ID
        cmd: CreatedShopApplyCommand, // 申请命令
    ) -> anyhow::Result<()> {
        manage::review_shop_apply(uid, shop_id, cmd).await
    }

    ////////

    /// # 4. [ADAPTER] - 驳回商店申请
    async fn reject_shop_apply_and_send_event(
        &self,
        uid: i64,                     // 管理员ID
        shop_id: i64,                 // 店铺ID
        cmd: CreatedShopApplyCommand, // 申请命令
    ) -> anyhow::Result<()> {
        manage::reject_shop_apply(uid, shop_id, cmd).await
    }

    ////////

    /// # 5. [ADAPTER] - 管理员删除单个商店
    async fn admin_del_one_shop_and_send_event(
        &self,
        uid: i64,     // 管理员ID
        shop_id: i64, // 店铺ID
    ) -> anyhow::Result<()> {
        del::soft_delete_single(uid, shop_id).await
    }

    ////////

    /// # 6. [ADAPTER] - 管理员批量删除商店
    async fn admin_batch_del_many_shops_and_send_event(
        &self,
        uid: i64,           // 管理员ID
        shop_ids: Vec<i64>, // 店铺ID列表
    ) -> anyhow::Result<()> {
        del::soft_delete_batch(uid, shop_ids).await
    }
}

//////// END

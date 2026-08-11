// /appy.rs
// 
// 2026/8/11 05:43 Created.

////////


// port/src/cola_market/shop_appy.rs
// ⏩️ 端口 - 可乐市场 - 商店 - 申请
// 2026/6/18 12:21 Created.

////////

use cola_data::cola_market::command::shop::add::CreatedShopApplyCommand;

////////

/// # [SERVICE PORT] - 商店申请 服务端口
#[async_trait::async_trait]
pub trait ShopAppyPort: Send + Sync {
    ////////

    /// # 1. [PORT] - 保存商店申请 + 发送事件
    async fn save_shop_appy_and_send_event(
        &self,
        uid: i64,
        cmd: CreatedShopApplyCommand,
    ) -> anyhow::Result<()>;

    ////////

    /// # 2. [PORT] - 编辑商店申请 + 发送事件
    async fn update_shop_apply_and_send_event(
        &self,
        uid: i64,
        shop_id: i64,
        cmd: CreatedShopApplyCommand,
    ) -> anyhow::Result<()>;

    ////////

    /// # 3. [PORT] - 审核商店申请 + 发送事件
    async fn review_shop_apply_and_send_event(
        &self,
        uid: i64,
        shop_id: i64,
        cmd: CreatedShopApplyCommand,
    ) -> anyhow::Result<()>;

    ////////

    /// # 4. [PORT] - 驳回商店申请 + 发送事件
    async fn reject_shop_apply_and_send_event(
        &self,
        uid: i64,
        shop_id: i64,
        cmd: CreatedShopApplyCommand,
    ) -> anyhow::Result<()>;

    ////////

    /// # 5. [PORT] - 管理员删除一个商店
    async fn admin_del_one_shop_and_send_event(&self, uid: i64, shop_id: i64)
                                               -> anyhow::Result<()>;

    ////////

    /// # 6. [PORT] - 管理员批量删除多个商店
    async fn admin_batch_del_many_shops_and_send_event(
        &self,
        uid: i64,
        shop_ids: Vec<i64>,
    ) -> anyhow::Result<()>;
}

//////// END

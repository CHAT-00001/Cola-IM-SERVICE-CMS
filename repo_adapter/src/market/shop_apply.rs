// repo_adapter/src/market/shop_apply.rs  -- 适配器 - 店铺申请
// 2026/6/18

//////

use async_trait::async_trait;
use cola_data::market::port::shop_appy::ShopAppyPort;
use cola_data::market::command::shop_apply::ShopApplyCommand;

//////

/// # [ADAPTER] - 店铺申请 端口适配器
pub struct ShopAppyAdapter;

#[async_trait]
impl ShopAppyPort for ShopAppyAdapter {

    async fn save_shop_appy_and_send_event(&self, _uid: i64, _cmd: ShopApplyCommand) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("not implemented"))
    }

    async fn update_shop_apply_and_send_event(&self, _uid: i64, _shop_id: i64, _cmd: ShopApplyCommand) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("not implemented"))
    }

    async fn review_shop_apply_and_send_event(&self, _uid: i64, _shop_id: i64, _cmd: ShopApplyCommand) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("not implemented"))
    }

    async fn reject_shop_apply_and_send_event(&self, _uid: i64, _shop_id: i64, _cmd: ShopApplyCommand) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("not implemented"))
    }

    async fn admin_del_one_shop_and_send_event(&self, _uid: i64, _shop_id: i64) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("not implemented"))
    }

    async fn admin_batch_del_many_shops_and_send_event(&self, _uid: i64, _shop_ids: Vec<i64>) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("not implemented"))
    }
}

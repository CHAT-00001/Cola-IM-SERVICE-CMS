// repo_adapter/src/market/address/del.rs
// 🔌 插头 - MARKET - ADDRESS - 删除
// 2026/8/6 19:19 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::market::address::delete::AddressDeletePort;

////////

/// # [DELETE ADAPTER] - 删除
/// * `desc`: `MARKET - 地址删除适配器`
pub struct AddressDelAdapter;

#[async_trait]
impl AddressDeletePort for AddressDelAdapter {
    async fn single_delete(&self, view_id: i64) -> Result<(u16)> {
        todo!()
    }

    async fn batch_delete(&self, view_ids: Vec<i64>) -> Result<(u16)> {
        todo!()
    }

    async fn delete_address_by_user_id(&self, user_id: i64) -> Result<(u64)> {
        todo!()
    }
}

//////// END

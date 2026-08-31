// repo_adapter/src/market/express/list.rs
// 🔌 适配器 - MARKET - 快递公司 - 列表
// 2026/8/7 05:31 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::market::info::express::express::ExpressInfo;
use port::market::express::list::ExpressListPort;

////////

/// # [LIST ADAPTER] - 列表
/// * `desc`: `MARKER - 快递公司列表适配器`
pub struct ExpressListAdapter;

#[async_trait]
impl ExpressListPort for ExpressListAdapter {
    async fn get_view_infos_by_user_id(
        &self,
        user_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<ExpressInfo>)> {
        todo!()
    }

    async fn get_new_infos(&self, limit: i64, offset: i64) -> Result<(Vec<ExpressInfo>)> {
        todo!()
    }

    async fn batch_get_infos_by_ids(&self, ids: Vec<i64>) -> Result<(Vec<ExpressInfo>)> {
        todo!()
    }
}

//////// END

// repo_adapter/src/market/express/alive.rs
// 🔌 适配器 - MARKET - EXPRESS - 活跃
// 2026/8/6 19:20 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::market::express::active::ExpressActivePort;

////////

/// # [ACTIVE ADAPTER] - 活跃
/// * `desc`: `快递公司活跃适配器` - 设置默认快递
pub struct ExpressAliveAdapter;

#[async_trait]
impl ExpressActivePort for ExpressAliveAdapter {
    /// # [ADAPTER] - 设置默认快递
    async fn set_default_express(
        &self,
        _uid: i64,        // 操作员 ID
        _express_id: i64, // 快递 ID
    ) -> Result<()> {
        todo!()
    }
}

//////// END

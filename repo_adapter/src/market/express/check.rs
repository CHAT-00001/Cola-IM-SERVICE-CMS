// repo_adapter/src/market/express/check.rs
// 🔌 适配器 - MARKET - EXPRESS - 检查
// 2026/8/6 19:19 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::market::express::check::ExpressCheckPort;

////////

/// # [CHECK ADAPTER] - 检查
/// * `desc`: `快递公司检查适配器`
pub struct ExpressCheckAdapter;

#[async_trait]
impl ExpressCheckPort for ExpressCheckAdapter {
    /// # 1. [ADAPTER] - 健康
    async fn check_health(&self, express_id: i64) -> Result<(bool)> {
        todo!()
    }

    /// # 2. [ADAPTER] - 状态
    async fn check_status(&self, uid: i64, express_id: i64) -> Result<(i16)> {
        todo!()
    }
}

//////// END

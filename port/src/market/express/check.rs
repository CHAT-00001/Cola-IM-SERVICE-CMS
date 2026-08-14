// port/src/market/express/check.rs
// ⏩️ 端口 - MARKET - 快递公司 - 检查
// 2026/8/12 01:00 Created.

////////

use cola_data::market::command::goods::GoodsCommand;
use cola_data::market::info::express::express::ExpressInfo;

////////

/// # [CHECK PORTS] - 检查
/// * `desc`: `MARKET - EXPRESS 检查端口`
#[async_trait::async_trait]
pub trait ExpressCheckPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 健康
    async fn check_health(
        &self,
        express_id: i64, // 快递公司的 ID
    ) -> anyhow::Result<(ExpressInfo)>;

    ////////

    /// # 2. [PORT] - 状态
    /// * `desc`: `返回状态码`
    async fn check_status(
        &self,
        uid: i64,        // 操作员 ID
        express_id: i64, // 快递公司 ID
    ) -> anyhow::Result<(i16)>;
}

//////// END

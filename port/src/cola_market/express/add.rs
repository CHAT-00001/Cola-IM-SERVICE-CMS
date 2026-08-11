// port/src/market/express.rs
// ⏩️ 端口 - MARKET - 快递公司 - mod
// 2026/6/18 14:00 Created.

////////

use cola_data::cola_market::command::goods::GoodsCommand;
use cola_data::cola_market::info::express::express::ExpressInfo;

////////

/// # [EXPRESS PORTS] - 商品 服务端口
/// * `desc`: `MARKET - EXPRESS 端口`
#[async_trait::async_trait]
pub trait ExpressAddPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 保存
    async fn save_express(
        &self,
        uid: i64,          // 操作员 ID
        cmd: GoodsCommand, // 命令
    ) -> anyhow::Result<(ExpressInfo)>;

    ////////

    /// # 2. [PORT] - 编辑
    async fn update_express(
        &self,
        uid: i64,          // 操作员 ID
        express_id: i64,   // 快递 ID
        cmd: GoodsCommand, // 命令
    ) -> anyhow::Result<(ExpressInfo)>;

    ////////

    /// # 3. [PORT] - 修改状态
    /// * `desc` 上架/下架
    async fn change_status(
        &self,
        express_id: i64,  // 快递 ID
        status_code: i16, // 状态码
    ) -> anyhow::Result<()>;

    ////////
}

//////// END

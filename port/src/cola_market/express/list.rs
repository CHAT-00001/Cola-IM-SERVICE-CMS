// port/src/cola_market/express/list.rs
// ⏩️ 端口 -MARKET - 快递公司 - 列表端口
// 2026/8/7 06:39 Created.

////////

use cola_data::cola_market::info::express::express::ExpressInfo;

////////

/// # [LIST PORTS]
/// * `desc`: `MARKET - 快递公司列表端口`
#[async_trait::async_trait]
pub trait ExpressListPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 用户的主动浏览记录
    async fn get_view_infos_by_user_id(
        &self,
        user_id: i64, // 用户ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> anyhow::Result<(Vec<ExpressInfo>)>;

    ////////

    /// # 2. [PORT] - 最新
    async fn get_new_infos(
        &self,
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> anyhow::Result<(Vec<ExpressInfo>)>;

    ////////

    /// # 2. [PORT] - 批量获取快递信息
    async fn batch_get_infos_by_ids(
        &self,
        ids: Vec<i64>, // IDs
    ) -> anyhow::Result<(Vec<ExpressInfo>)>;
}

//////// END

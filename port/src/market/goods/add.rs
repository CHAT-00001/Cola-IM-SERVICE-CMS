// port/src/market/goods/add.rs
// ⏩️ 端口 - MARKET - 商品 - 发布
// 2026/8/4 23:58 Created.

////////

use cola_data::market::command::goods::GoodsCommand;

////////

/// # [ADD PORTS] - 发布
/// * `desc`: `▶ VIDEO - 视频发布端口`
#[async_trait::async_trait]
pub trait GoodsAddPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 添加商品
    async fn add_goods(
        &self,
        uid: i64,          // 操作员 ID
        cmd: GoodsCommand, // 命令
    ) -> anyhow::Result<()>;

    ////////

    /// # 2. [PORT] - 编辑商品
    async fn update_goods(
        &self,
        uid: i64,
        goods_id: i64,     // 商品 ID
        cmd: GoodsCommand, // 命令
    ) -> anyhow::Result<()>;

    ////////

    /// # 3. [PORT] - 修改权限
    /// * `desc` 可见性等
    async fn change_permission(
        &self,
        uid: i64,
        goods_id: i64,    // 商品 ID
        status_code: i16, // 状态码
    ) -> anyhow::Result<()>;

    ////////

    /// # 4. [PORT] - 修改状态
    /// * `desc` 上架/下架
    async fn change_status(
        &self,
        uid: i64,
        goods_id: i64,    // 商品 ID
        status_code: i16, // 状态码
    ) -> anyhow::Result<()>;
}

//////// END

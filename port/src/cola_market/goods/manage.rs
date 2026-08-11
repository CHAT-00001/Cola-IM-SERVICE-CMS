// port/src/cola_market/goods/manage.rs
// ⏩️ 端口 - MARKET - 商品 - 管理
// 2026/8/5 00:39 Created.

////////

use cola_data::cola_market::info::goods::goods::GoodsInfo;
use rust_decimal::Decimal;

////////

/// # [MANAGE PORTS] - 管理
/// * `desc`: `▶ VIDEO - 商品管理端口`
#[async_trait::async_trait]
pub trait GoodsManagePort: Send + Sync {
    //

    ////////

    /// # [PORT] - 管理员列表
    /// * `desc`: `管理员查看所有商品`
    /// * `condition`: `⚠️ ADMIN / REVIEWER` - `无视权限/状态`
    async fn admin_list(
        &self,
        uid: i64,                    // 操作者 ID
        user_id: Option<i64>,        // 用户 ID
        goods_id: Option<i64>,       // 商品 ID
        one_class: Option<i64>,      // 一级分类
        two_class: Option<i64>,      // 二级分类
        three_class: Option<i64>,    // 三级分类
        channel_id: Option<i64>,     // 频道 ID
        city_id: Option<i64>,        // 城市 ID (同城时)
        keyword: Option<String>,     // 关键词
        start_time: Option<i64>,     // 开始时间
        end_time: Option<i64>,       // 结束时间
        low_price: Option<Decimal>,  // 最低价格
        high_price: Option<Decimal>, // 最高价格
        lat: Option<f64>,            // 经度
        lng: Option<f64>,            // 经度
        range: Option<i32>,          // 半径距离
        status_code: i16,            // 状态码
        limit: i64,                  // 数量
        offset: i64,                 // 页码
    ) -> anyhow::Result<(Vec<GoodsInfo>), u64>;
}

//////// END

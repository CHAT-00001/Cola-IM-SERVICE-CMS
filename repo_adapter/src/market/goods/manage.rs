// repo_adapter/src/cola_market/goods/manage.rs
// 🔌 适配器 - MARKET - GOODS - 管理
// 2026/8/11 08:45 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_market::info::goods::goods::GoodsInfo;
use port::cola_market::goods::manage::GoodsManagePort;
use rust_decimal::Decimal;

////////

/// # [MANAGE ADAPTER] - 管理
/// `desc`: `MARKET - 管理员商品列表适配器`
pub struct GoodsManageAdapter;

#[async_trait]
impl GoodsManagePort for GoodsManageAdapter {
    //

    ////////

    /// # [ADAPTER] - 管理员列表
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
    ) -> Result<(Vec<GoodsInfo>), u64> {
        todo!()
    }
}

//////// END

// repository/src/market/pg/goods/manage.rs
// 仓储 - MARKET - pg - 商品 - 管理获取
// 2026/8/11 08:46 Created.

////////

use crate::pg_pool;
use cola_data::market::entity::goods::goods::{GoodsEntity, GOODS_COLUMNS};
use rust_decimal::Decimal;
use sqlx::{Postgres, QueryBuilder};

////////

/// # [MANAGE REPOSITORY] - 管理员列表
/// * `desc`: `MARKET - 商品管理仓储`
pub struct GoodsManageRepo;

impl GoodsManageRepo {
    //

    ////////

    /// # [REPOSITORY] - 管理员列表
    /// * `desc`: `极其丰富的查询筛选（纯 SQL 经纬度计算，不依赖 PostGIS）`
    #[allow(clippy::too_many_arguments)]
    pub async fn admin_list(
        uid: Option<i64>,
        goods_id: Option<i64>,
        one_classid: Option<i64>,
        two_classid: Option<i64>,
        three_classid: Option<i64>,
        city_id: Option<i64>,
        status: Option<i16>,
        low_price: Option<Decimal>,
        high_price: Option<Decimal>,
        lat: Option<f64>,
        lng: Option<f64>,
        range: Option<i32>, // 半径距离（单位：米）
        keyword: Option<String>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<GoodsEntity>, sqlx::Error> {
        let pool = pg_pool();

        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(format!(
            "SELECT {} FROM cola_market.goods WHERE 1=1 ",
            GOODS_COLUMNS
        ));

        // 基础字段过滤
        if let Some(uid) = uid { query_builder.push(" AND uid = ").push_bind(uid); }
        if let Some(id) = goods_id { query_builder.push(" AND id = ").push_bind(id); }
        if let Some(c1) = one_classid { query_builder.push(" AND one_classid = ").push_bind(c1); }
        if let Some(c2) = two_classid { query_builder.push(" AND two_classid = ").push_bind(c2); }
        if let Some(c3) = three_classid { query_builder.push(" AND three_classid = ").push_bind(c3); }
        if let Some(city) = city_id { query_builder.push(" AND city_id = ").push_bind(city); }
        if let Some(s) = status { query_builder.push(" AND status = ").push_bind(s); }

        // 价格区间筛选
        if let Some(lp) = low_price { query_builder.push(" AND price >= ").push_bind(lp); }
        if let Some(hp) = high_price { query_builder.push(" AND price <= ").push_bind(hp); }

        // 时间范围筛选
        if let Some(st) = start_time { query_builder.push(" AND add_time >= ").push_bind(st); }
        if let Some(et) = end_time { query_builder.push(" AND add_time <= ").push_bind(et); }

        // 关键词模糊搜索
        if let Some(kw) = keyword {
            query_builder.push(" AND (name ILIKE ");
            query_builder.push_bind(format!("%{}%", kw));
            query_builder.push(" OR name_en ILIKE ");
            query_builder.push_bind(format!("%{}%", kw));
            query_builder.push(")");
        }

        // 纯 SQL 经纬度范围查询 (使用 Haversine 公式，不依赖 PostGIS 扩展)
        // 使用 least/greatest 防止浮点精度误差导致 acos 溢出（DOMAIN ERROR）
        if let (Some(target_lat), Some(target_lng), Some(target_range)) = (lat, lng, range) {
            query_builder.push(" AND (6371000 * acos(least(1.0, greatest(-1.0, cos(radians(");
            query_builder.push_bind(target_lat);
            query_builder.push(")) * cos(radians(lat)) * cos(radians(lng) - radians(");
            query_builder.push_bind(target_lng);
            query_builder.push(")) + sin(radians(");
            query_builder.push_bind(target_lat);
            query_builder.push(")) * sin(radians(lat)))))) <= ");
            query_builder.push_bind(target_range);
        }

        // 分页与排序
        query_builder.push(" ORDER BY add_time DESC LIMIT ");
        query_builder.push_bind(limit);
        query_builder.push(" OFFSET ");
        query_builder.push_bind(offset);

        query_builder
            .build_query_as::<GoodsEntity>()
            .fetch_all(&pool)
            .await
    }
}

//////// END
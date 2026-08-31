// repository/src/market/pg/goods/list.rs
// 仓储 - MARKET - pg - 商品 - 获取
// 2026/8/11 07:25 Created.

////////

use crate::pg_pool;
use cola_data::market::entity::goods::goods::{GOODS_COLUMNS, GoodsEntity};

////////

/// # [LIST REPOSITORY] - 列表
/// * `desc`: `查询商品记录列表`
pub struct GoodsListRepo;

impl GoodsListRepo {
    /// # 1. [REPOSITORY] - 最新商品
    pub async fn find_new(limit: i64, offset: i64) -> Result<Vec<GoodsEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_market.goods ORDER BY add_time DESC LIMIT $1 OFFSET $2",
            GOODS_COLUMNS
        );
        sqlx::query_as::<_, GoodsEntity>(&query)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 2. [REPOSITORY] - 热门商品
    pub async fn find_hot(limit: i64, offset: i64) -> Result<Vec<GoodsEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_market.goods ORDER BY sale_nums DESC, hits DESC LIMIT $1 OFFSET $2",
            GOODS_COLUMNS
        );
        sqlx::query_as::<_, GoodsEntity>(&query)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 3. [REPOSITORY] - 同城商品
    pub async fn find_city(
        city: &str, // 城市名称
        limit: i64,
        offset: i64,
    ) -> Result<Vec<GoodsEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_market.goods WHERE city = $1 ORDER BY add_time DESC LIMIT $2 OFFSET $3",
            GOODS_COLUMNS
        );
        sqlx::query_as::<_, GoodsEntity>(&query)
            .bind(city)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 4. [REPOSITORY] - 附近商品
    pub async fn find_nearby(
        lat: f64,   // 纬度
        lng: f64,   // 经度
        range: i32, // 范围(单位:m)
        limit: i64,
        offset: i64,
    ) -> Result<Vec<GoodsEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_market.goods ORDER BY (lat - $1)*(lat - $1) + (lng - $2)*(lng - $2) ASC LIMIT $3 OFFSET $4",
            GOODS_COLUMNS
        );
        sqlx::query_as::<_, GoodsEntity>(&query)
            .bind(lat)
            .bind(lng)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 5. [REPOSITORY] - 用户的商品列表
    pub async fn find_by_user_id(
        user_id: i64, // 用户 ID
        limit: i64,   // 数量
        offset: i64,  // 偏移量
    ) -> Result<Vec<GoodsEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_market.goods WHERE uid = $1 ORDER BY add_time DESC LIMIT $2 OFFSET $3",
            GOODS_COLUMNS,
        );
        sqlx::query_as::<_, GoodsEntity>(&query)
            .bind(user_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 6. [REPOSITORY] - 商店的商品列表
    pub async fn find_by_shop_id(
        market_id: i64, // 市场/商店 ID
        limit: i64,
        offset: i64,
    ) -> Result<Vec<GoodsEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_market.goods WHERE market_id = $1 ORDER BY add_time DESC LIMIT $2 OFFSET $3",
            GOODS_COLUMNS,
        );
        sqlx::query_as::<_, GoodsEntity>(&query)
            .bind(market_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 7. [REPOSITORY] - 单个查询
    pub async fn find_by_id(id: i64) -> Result<Option<GoodsEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_market.goods WHERE id = $1 LIMIT 1",
            GOODS_COLUMNS,
        );
        sqlx::query_as::<_, GoodsEntity>(&query)
            .bind(id)
            .fetch_optional(&pool)
            .await
    }

    ////////

    /// # 8. [REPOSITORY] - 批量查询
    pub async fn find_by_ids(ids: &[i64]) -> Result<Vec<GoodsEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_market.goods WHERE id = ANY($1)",
            GOODS_COLUMNS,
        );
        sqlx::query_as::<_, GoodsEntity>(&query)
            .bind(ids)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 9. [REPOSITORY] - 推荐列表
    pub async fn find_recommend(limit: i64, offset: i64) -> Result<Vec<GoodsEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_market.goods WHERE isrecom = 1 ORDER BY sale_nums DESC LIMIT $1 OFFSET $2",
            GOODS_COLUMNS,
        );
        sqlx::query_as::<_, GoodsEntity>(&query)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 10. [REPOSITORY] - 分类综合查询
    pub async fn find_by_category(
        one_classid: Option<i64>,   // 一级分类
        two_classid: Option<i64>,   // 二级分类
        three_classid: Option<i64>, // 三级分类
        keyword: Option<String>,    // 搜索关键词(匹配 name / name_en)
        max_price: Option<bool>,    // 按价格排序 (true: 降序/最贵)
        max_sales: Option<bool>,    // 按销量排序 (true: 降序/最高)
        limit: i64,
        offset: i64,
    ) -> Result<Vec<GoodsEntity>, sqlx::Error> {
        let pool = pg_pool();
        let mut conditions = vec!["1=1".to_string()];

        if let Some(v) = one_classid {
            conditions.push(format!("one_classid = {}", v));
        }
        if let Some(v) = two_classid {
            conditions.push(format!("two_classid = {}", v));
        }
        if let Some(v) = three_classid {
            conditions.push(format!("three_classid = {}", v));
        }

        // 处理 keyword
        let mut has_keyword = false;
        let keyword_pattern = if let Some(ref kw) = keyword {
            if !kw.trim().is_empty() {
                has_keyword = true;
                format!("%{}%", kw.trim())
            } else {
                "".to_string()
            }
        } else {
            "".to_string()
        };

        if has_keyword {
            conditions.push("(name ILIKE $1 OR name_en ILIKE $1)".to_string());
        }

        let where_clause = conditions.join(" AND ");

        // 动态排序
        let order_by = if let Some(true) = max_price {
            "ORDER BY present_price DESC"
        } else if let Some(true) = max_sales {
            "ORDER BY sale_nums DESC"
        } else {
            "ORDER BY add_time DESC"
        };

        // 根据是否有 keyword 动态调整占位符索引
        let query = if has_keyword {
            format!(
                "SELECT {} FROM cola_market.goods WHERE {} {} LIMIT $2 OFFSET $3",
                GOODS_COLUMNS, where_clause, order_by
            )
        } else {
            format!(
                "SELECT {} FROM cola_market.goods WHERE {} {} LIMIT $1 OFFSET $2",
                GOODS_COLUMNS, where_clause, order_by
            )
        };

        let mut q = sqlx::query_as::<_, GoodsEntity>(&query);

        if has_keyword {
            q = q.bind(keyword_pattern);
        }

        q.bind(limit).bind(offset).fetch_all(&pool).await
    }

    ////////

    /// # 11. [REPOSITORY] - 搜索商品
    pub async fn search_list(
        keyword: &str,
        city_id: Option<i64>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<GoodsEntity>, sqlx::Error> {
        let pool = pg_pool();
        let keyword_like = format!("%{}%", keyword);
        let mut conditions = vec!["(name ILIKE $1 OR name_en ILIKE $1)".to_string()];
        if let Some(cat) = city_id {
            conditions.push(format!("one_classid = {}", cat));
        }
        let where_clause = conditions.join(" AND ");
        let query = format!(
            "SELECT {} FROM cola_market.goods WHERE {} ORDER BY add_time DESC LIMIT $2 OFFSET $3",
            GOODS_COLUMNS, where_clause
        );
        sqlx::query_as::<_, GoodsEntity>(&query)
            .bind(&keyword_like)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }
}

//////// END

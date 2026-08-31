// repository/src/market/pg/goods/feed.rs
// 仓储 - MARKET - pg - 商品 - 获取
// 2026/8/11 07:37 Created.

////////

use crate::pg_pool;
use cola_data::market::entity::goods::goods::{GOODS_COLUMNS, GoodsEntity};

////////

/// # [FEED REPOSITORY] - 流
/// * `desc`: `feed`
pub struct GoodsFeedRepo;

impl GoodsFeedRepo {
    //

    ////////

    /// # 5. [REPOSITORY] - 上架/下架
    pub async fn toggle_status(id: i64) -> Result<(), sqlx::Error> {
        let pool = pg_pool();
        sqlx::query(
            "UPDATE cola_market.goods SET issale = CASE WHEN issale = 1 THEN 0 ELSE 1 END WHERE id = $1",
        )
            .bind(id)
            .execute(&pool)
            .await?;
        Ok(())
    }

    ////////

    /// 6. 删除商品
    pub async fn delete(id: i64) -> Result<(), sqlx::Error> {
        let pool = pg_pool();
        sqlx::query("DELETE FROM cola_market.goods WHERE id = $1")
            .bind(id)
            .execute(&pool)
            .await?;
        Ok(())
    }

    ////////

    /// 7. 推荐列表
    pub async fn find_recommend(offset: i64, limit: i64) -> Result<Vec<GoodsEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_market.goods WHERE isrecom = 1 ORDER BY sale_nums DESC LIMIT $1 OFFSET $2",
            GOODS_COLUMNS
        );
        sqlx::query_as::<_, GoodsEntity>(&query)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// 8. 分类查询
    pub async fn find_by_category(
        one_classid: Option<i16>,
        two_classid: Option<i16>,
        three_classid: Option<i16>,
        offset: i64,
        limit: i64,
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
        let where_clause = conditions.join(" AND ");
        let query = format!(
            "SELECT {} FROM cola_market.goods WHERE {} ORDER BY add_time DESC LIMIT $1 OFFSET $2",
            GOODS_COLUMNS, where_clause
        );
        sqlx::query_as::<_, GoodsEntity>(&query)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// 9. 搜索商品
    pub async fn search(
        keyword: &str,
        category_id: Option<i16>,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<GoodsEntity>, sqlx::Error> {
        let pool = pg_pool();
        let keyword_like = format!("%{}%", keyword);
        let mut conditions = vec!["(name ILIKE $1 OR name_en ILIKE $1)".to_string()];
        if let Some(cat) = category_id {
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

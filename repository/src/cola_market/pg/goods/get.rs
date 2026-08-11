// repository/src/cola_market/pg/goods/get.rs
// 仓储 - MARKET - pg - 商品 - 获取
// 2026/8/11 07:23 Created.

////////

use crate::pg_pool;
use cola_data::cola_market::entity::goods::goods::GoodsEntity;

////////

/// # [GET REPOSITORY] - 商品 发布 仓储
/// * `desc`: `获取商品记录`
pub struct GoodsGetRepo;

impl GoodsGetRepo {
    const COLUMNS: &'static str = r#"
        id, uid, market_id, name, name_en, no,
        one_classid, two_classid, three_classid,
        video_url, video_thumb, video_length, thumbs, content, pictures,
        specs, postage, hits, isrecom, sale_nums, refuse_reason, issale, type,
        original_price, present_price, goods_desc, href, live_isshow,
        low_price, admin_id, commission, share_income,
        lat, lng, city, address, label_id,
        collects, shares, add_time, upd_time, create_at, update_at
    "#;

    ////////

    /// # 1. [REPOSITORY] - 添加商品
    pub async fn find_by_uid(
        uid: i64,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<GoodsEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM shop_goods WHERE uid = $1 ORDER BY add_time DESC LIMIT $2 OFFSET $3",
            Self::COLUMNS
        );
        sqlx::query_as::<_, GoodsEntity>(&query)
            .bind(uid)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// 4. 按ID查询商品
    pub async fn find_by_id(id: i64) -> Result<Option<GoodsEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM shop_goods WHERE id = $1 LIMIT 1",
            Self::COLUMNS
        );
        sqlx::query_as::<_, GoodsEntity>(&query)
            .bind(id)
            .fetch_optional(&pool)
            .await
    }

    ////////

    /// 5. 上架/下架
    pub async fn toggle_status(id: i64) -> Result<(), sqlx::Error> {
        let pool = pg_pool();
        sqlx::query(
            "UPDATE shop_goods SET issale = CASE WHEN issale = 1 THEN 0 ELSE 1 END WHERE id = $1",
        )
            .bind(id)
            .execute(&pool)
            .await?;
        Ok(())
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
            "SELECT {} FROM shop_goods WHERE {} ORDER BY add_time DESC LIMIT $1 OFFSET $2",
            Self::COLUMNS,
            where_clause
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
            "SELECT {} FROM shop_goods WHERE {} ORDER BY add_time DESC LIMIT $2 OFFSET $3",
            Self::COLUMNS,
            where_clause
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

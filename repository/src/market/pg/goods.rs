// repository/src/market/pg/goods.rs  -- 仓储层 - MARKET - pg - 商品
// 2026/6/18 14:29

////////

use cola_data::market::entity::goods::GoodsEntity;
use crate::pg_pool;

////////

/// # [GOODS REPO] - 商品 仓储
pub struct GoodsRepo;

impl GoodsRepo {

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

    /// 1. 插入商品
    pub async fn insert(entity: &GoodsEntity) -> Result<GoodsEntity, sqlx::Error> {
        let pool = pg_pool();
        sqlx::query_as::<_, GoodsEntity>(
            "INSERT INTO shop_goods (uid, market_id, name, name_en, no, one_classid, two_classid, three_classid, video_url, video_thumb, video_length, thumbs, content, pictures, specs, postage, hits, isrecom, sale_nums, refuse_reason, issale, type, original_price, present_price, goods_desc, href, live_isshow, low_price, admin_id, commission, share_income, lat, lng, city, address, label_id, collects, shares, add_time, upd_time) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,$18,$19,$20,$21,$22,$23,$24,$25,$26,$27,$28,$29,$30,$31,$32,$33,$34,$35,$36,$37,$38,$39,$40) RETURNING *"
        )
        .bind(&entity.uid).bind(&entity.market_id).bind(&entity.name).bind(&entity.name_en).bind(&entity.no)
        .bind(&entity.one_classid).bind(&entity.two_classid).bind(&entity.three_classid)
        .bind(&entity.video_url).bind(&entity.video_thumb).bind(&entity.video_length)
        .bind(&entity.thumbs).bind(&entity.content).bind(&entity.pictures).bind(&entity.specs)
        .bind(&entity.postage).bind(&entity.hits).bind(&entity.isrecom).bind(&entity.sale_nums)
        .bind(&entity.refuse_reason).bind(&entity.issale).bind(&entity.r#type)
        .bind(&entity.original_price).bind(&entity.present_price).bind(&entity.goods_desc).bind(&entity.href)
        .bind(&entity.live_isshow).bind(&entity.low_price).bind(&entity.admin_id).bind(&entity.commission)
        .bind(&entity.share_income).bind(&entity.lat).bind(&entity.lng).bind(&entity.city).bind(&entity.address)
        .bind(&entity.label_id).bind(&entity.collects).bind(&entity.shares).bind(&entity.add_time).bind(&entity.upd_time)
        .fetch_one(&pool)
        .await
    }

    ////////

    /// 2. 更新商品
    pub async fn update(id: i64, entity: &GoodsEntity) -> Result<GoodsEntity, sqlx::Error> {
        let pool = pg_pool();
        sqlx::query_as::<_, GoodsEntity>(
            "UPDATE shop_goods SET name=$1, name_en=$2, no=$3, one_classid=$4, two_classid=$5, three_classid=$6, thumbs=$7, content=$8, pictures=$9, specs=$10, present_price=$11, original_price=$12, upd_time=$13 WHERE id=$14 RETURNING *"
        )
        .bind(&entity.name).bind(&entity.name_en).bind(&entity.no)
        .bind(&entity.one_classid).bind(&entity.two_classid).bind(&entity.three_classid)
        .bind(&entity.thumbs).bind(&entity.content).bind(&entity.pictures).bind(&entity.specs)
        .bind(&entity.present_price).bind(&entity.original_price).bind(&entity.upd_time)
        .bind(id)
        .fetch_one(&pool)
        .await
    }

    ////////

    /// 3. 按用户ID查询商品列表
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
            .bind(uid).bind(limit).bind(offset)
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
        sqlx::query("UPDATE shop_goods SET issale = CASE WHEN issale = 1 THEN 0 ELSE 1 END WHERE id = $1")
            .bind(id)
            .execute(&pool)
            .await?;
        Ok(())
    }

    ////////

    /// 6. 删除商品
    pub async fn delete(id: i64) -> Result<(), sqlx::Error> {
        let pool = pg_pool();
        sqlx::query("DELETE FROM shop_goods WHERE id = $1")
            .bind(id)
            .execute(&pool)
            .await?;
        Ok(())
    }

    ////////

    /// 7. 推荐列表
    pub async fn find_recommend(
        offset: i64,
        limit: i64,
    ) -> Result<Vec<GoodsEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM shop_goods WHERE isrecom = 1 ORDER BY sale_nums DESC LIMIT $1 OFFSET $2",
            Self::COLUMNS
        );
        sqlx::query_as::<_, GoodsEntity>(&query)
            .bind(limit).bind(offset)
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
        if let Some(v) = one_classid { conditions.push(format!("one_classid = {}", v)); }
        if let Some(v) = two_classid { conditions.push(format!("two_classid = {}", v)); }
        if let Some(v) = three_classid { conditions.push(format!("three_classid = {}", v)); }
        let where_clause = conditions.join(" AND ");
        let query = format!(
            "SELECT {} FROM shop_goods WHERE {} ORDER BY add_time DESC LIMIT $1 OFFSET $2",
            Self::COLUMNS, where_clause
        );
        sqlx::query_as::<_, GoodsEntity>(&query)
            .bind(limit).bind(offset)
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
            Self::COLUMNS, where_clause
        );
        sqlx::query_as::<_, GoodsEntity>(&query)
            .bind(&keyword_like).bind(limit).bind(offset)
            .fetch_all(&pool)
            .await
    }
}

//////// END
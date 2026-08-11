// repository/src/cola_market/pg/goods/add.rs
// 仓储 - MARKET - pg - 商品 - 发布
// 2026/8/11 07:21 Created.

////////

use cola_data::cola_market::command::goods::GoodsCommand;
use crate::pg_pool;
use cola_data::cola_market::entity::goods::goods::GoodsEntity;

////////

/// # [ADD REPOSITORY] - 发布
/// * `desc`: `用户发布新商品`
pub struct GoodsAddRepo;

impl GoodsAddRepo {
    const COLUMNS: &'static str = r#"
        id, uid, city_id, name, name_en, no,
        one_classid, two_classid, three_classid,
        video_url, video_thumb, video_length, thumbs, content, pictures,
        specs, postage, hits, isrecom, sale_nums, refuse_reason, issale, type,
        original_price, present_price, goods_desc, href, live_isshow,
        low_price, admin_id, commission, share_income,
        lat, lng, city, address, label_id,
        collects, shares, add_time, upd_time, created_at, updated_at
    "#;

    ////////

    /// # 1. [REPOSITORY] - 添加商品
    pub async fn save_goods(command: GoodsCommand, admin_id: i64) -> Result<GoodsEntity, sqlx::Error> {
        let pool = pg_pool();

        // 核心：将前端传进来的简易 Command 转换为包含 40 多个字段的完整 Entity
        let entity = command.to_entity(admin_id);

        sqlx::query_as::<_, GoodsEntity>(
            r#"
            INSERT INTO cola_market.goods (
                uid, city_id, name, name_en, no, one_classid, two_classid, three_classid,
                video_url, video_thumb, video_length, thumbs, content, pictures, specs,
                postage, hits, isrecom, sale_nums, refuse_reason, issale, type,
                original_price, present_price, goods_desc, href, live_isshow,
                low_price, admin_id, commission, share_income, lat, lng, city,
                address, label_id, collects, shares, add_time, upd_time
            ) VALUES (
                $1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,$18,$19,$20,
                $21,$22,$23,$24,$25,$26,$27,$28,$29,$30,$31,$32,$33,$34,$35,$36,$37,$38,$39,$40
            ) RETURNING *
            "#
        )
            .bind(entity.uid).bind(entity.city_id).bind(entity.name).bind(entity.name_en).bind(entity.no)
            .bind(entity.one_classid).bind(entity.two_classid).bind(entity.three_classid)
            .bind(entity.video_url).bind(entity.video_thumb).bind(entity.video_length)
            .bind(entity.thumbs).bind(entity.content).bind(entity.pictures).bind(entity.specs)
            .bind(entity.postage).bind(entity.hits).bind(entity.isrecom).bind(entity.sale_nums)
            .bind(entity.refuse_reason).bind(entity.issale).bind(entity.r#type)
            .bind(entity.original_price).bind(entity.present_price).bind(entity.goods_desc).bind(entity.href)
            .bind(entity.live_isshow).bind(entity.low_price).bind(entity.admin_id).bind(entity.commission)
            .bind(entity.share_income).bind(entity.lat).bind(entity.lng).bind(entity.city).bind(entity.address)
            .bind(entity.label_id).bind(entity.collects).bind(entity.shares).bind(entity.add_time).bind(entity.upd_time)
            .fetch_one(&pool)
            .await
    }

    ////////

    /// # 2. [REPOSITORY] - 更新商品
    pub async fn update_goods(id: i64, entity: &GoodsEntity) -> Result<GoodsEntity, sqlx::Error> {
        let pool = pg_pool();
        sqlx::query_as::<_, GoodsEntity>(
            r#"
            UPDATE cola_market.goods
            SET name=$1, name_en=$2, no=$3, one_classid=$4, two_classid=$5, three_classid=$6,
                thumbs=$7, content=$8, pictures=$9, specs=$10, present_price=$11, original_price=$12, upd_time=$13
            WHERE id=$14
            RETURNING *
            "#
        )
            .bind(&entity.name).bind(&entity.name_en).bind(&entity.no)
            .bind(entity.one_classid).bind(entity.two_classid).bind(entity.three_classid)
            .bind(&entity.thumbs).bind(&entity.content).bind(&entity.pictures).bind(&entity.specs)
            .bind(&entity.present_price).bind(&entity.original_price).bind(entity.upd_time)
            .bind(id)
            .fetch_one(&pool)
            .await
    }

    ////////

    /// # 3. [REPOSITORY] - 上架/下架
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
}

//////// END
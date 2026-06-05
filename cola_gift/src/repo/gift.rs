// src/gift/info/repo.rs
// 2026-02-07 11:29:00

use sqlx::PgPool;
use crate::gift::entity::gift::GiftEntity;

/// 定义统一的查询字段 (确保字段顺序与数据库及 Entity 映射一致)
const GIFT_COLUMNS: &str = r#"
    id, mark, "type", sid, name, name_en, thumb, price, need_coin,
    gift_icon, list_order, swf_type, swf, swf_time, is_plat_gift,
    sticker_id, sort, is_del, add_time, update_time, created_at, updated_at
"#;


/// ## REPO - 新增礼物
pub async fn repo_add_gift(
    pool: &PgPool,
    gift: &GiftEntity,
) -> Result<GiftEntity, sqlx::Error> {
    let now_ts = chrono::Utc::now().timestamp();

    let sql = format!(
        r#"
        INSERT INTO gift (
            mark, type, sid, name, name_en, thumb, price, need_coin,
            gift_icon, list_order, swf_type, swf, swf_time, is_plat_gift,
            sticker_id, sort, is_del, add_time, update_time, created_at, updated_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, 0, $17, $17, NOW(), NOW())
        RETURNING {}
        "#,
        GIFT_COLUMNS
    );

    sqlx::query_as::<_, GiftEntity>(sql.as_str())
        .bind(gift.mark)
        .bind(gift.r#type)
        .bind(gift.sid)
        .bind(&gift.name)
        .bind(&gift.name_en)
        .bind(&gift.thumb)
        .bind(gift.price)
        .bind(gift.need_coin)
        .bind(&gift.gift_icon)
        .bind(gift.list_order)
        .bind(gift.swf_type)
        .bind(&gift.swf)
        .bind(gift.swf_time)
        .bind(gift.is_plat_gift)
        .bind(gift.sticker_id)
        .bind(gift.sort)
        .bind(now_ts)
        .fetch_one(pool)
        .await
}

/// ## REPO - 软删除礼物
pub async fn repo_soft_del_gifts(
    pool: &PgPool,
    ids: &[i64],
) -> Result<u64, sqlx::Error> {
    let sql = "UPDATE gift SET is_del = 1, update_time = $1, updated_at = NOW() WHERE id = ANY($2)";

    let result = sqlx::query(sql)
        .bind(chrono::Utc::now().timestamp())
        .bind(ids)
        .execute(pool)
        .await?;

    Ok(result.rows_affected())
}

/// ## REPO - 更新礼物详情
pub async fn repo_update_gift(
    pool: &PgPool,
    id: i64,
    gift: &GiftEntity,
) -> Result<u64, sqlx::Error> {
    let sql = r#"
        UPDATE gift SET
            mark = $1, name = $2, price = $3, need_coin = $4,
            sort = $5, update_time = $6, updated_at = NOW()
        WHERE id = $7 AND is_del = 0
    "#;

    let result = sqlx::query(sql)
        .bind(gift.mark)
        .bind(&gift.name)
        .bind(gift.price)
        .bind(gift.need_coin)
        .bind(gift.sort)
        .bind(chrono::Utc::now().timestamp())
        .bind(id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected())
}


/// ## REPO - 获取礼物列表
pub async fn repo_get_active_gifts(
    pool: &PgPool,
    limit: i64,
    offset: i64,
) -> Result<Vec<GiftEntity>, sqlx::Error> {
    let sql = format!(
        "SELECT {} FROM gift WHERE is_del = 0 ORDER BY sort ASC, id DESC LIMIT $1 OFFSET $2",
        GIFT_COLUMNS
    );

    sqlx::query_as::<_, GiftEntity>(sql.as_str())
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

/// ## REPO - 按分类 ID 查询
pub async fn repo_find_by_sid(
    pool: &PgPool,
    sid: i32,
) -> Result<Vec<GiftEntity>, sqlx::Error> {
    let sql = format!(
        "SELECT {} FROM gift WHERE sid = $1 AND is_del = 0 ORDER BY sort ASC",
        GIFT_COLUMNS
    );

    sqlx::query_as::<_, GiftEntity>(sql.as_str())
        .bind(sid)
        .fetch_all(pool)
        .await
}

/// ## REPO - 根据id查找对象
pub async fn repo_find_by_id(
    pool: &PgPool,
    id: i64,
) -> Result<GiftEntity, sqlx::Error> {
    let sql = format!("SELECT {} FROM gift WHERE id = $1 AND is_del = 0 LIMIT 1", GIFT_COLUMNS);

    sqlx::query_as::<_, GiftEntity>(sql.as_str())
        .bind(id)
        .fetch_one(pool)
        .await
}


/// REPO - 获取礼物列表（active）
pub async fn repo_get_active_gifts2(
    pool: &PgPool,
    size: i64,
    page: i64,
) -> Result<Vec<GiftEntity>, sqlx::Error> {

    let offset = (page - 1) * size;

    let sql = format!(
        r#"
        SELECT {}
        FROM gift
        WHERE is_del = 0
        ORDER BY sort ASC, id DESC
        LIMIT $1 OFFSET $2
        "#,
        GIFT_COLUMNS
    );

    sqlx::query_as::<_, GiftEntity>(&sql.as_str())
        .bind(page)
        .bind(offset)
        .fetch_all(pool)
        .await
}
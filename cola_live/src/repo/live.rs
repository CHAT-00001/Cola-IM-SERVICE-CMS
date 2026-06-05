// cola_video/src/live/repo/live.rs  -- 直播 仓储中心
// 2026-02-07 16:22:02

use sqlx::PgPool;
use uuid::Uuid;
use crate::model::entity::live::LiveEntity;

/// 构建live表基础字段（匹配最终版 Entity）
const LIVE_COLUMNS: &str = r#"
    uid, room_id, show_id, title, thumb, pull, stream, channel_id,
    is_video, is_mic, is_hot, is_recommend, is_live, is_shop, is_off, status,
    good_num, anyway, hot_votes, gift_total_coin, gift_user_total, banker_coin,
    pk_uid, pk_stream, video_url, province, city, address, lng, lat,
    live_type, type_val, device_info, game_action, voice_type,
    sw_player_status, sw_player_id, sw_pull_url,
    start_time, off_time, recommend_time
"#;

// --- 核心业务操作 ---

/// # REPO - 查询HOME直播列表
pub async fn repo_find_home_live_list(
    pool: &PgPool,
    uid: i64,
    title: String,
) -> Result<LiveEntity, sqlx::Error> {
    // 生成无分隔符 UUIDv4 作为房间识别码
    let room_id = Uuid::new_v4().simple().to_string();

    // 生成流名示例：uid_时间戳
    let stream = format!("{}_{}", uid, chrono::Utc::now().timestamp());

    let sql = format!(
        r#"
        INSERT INTO live (uid, room_id, title, stream, status, is_live, start_time)
        VALUES ($1, $2, $3, $4, 1, 1, EXTRACT(EPOCH FROM NOW())::BIGINT)
        ON CONFLICT (uid)
        DO UPDATE SET
            room_id = EXCLUDED.room_id,
            title = EXCLUDED.title,
            stream = EXCLUDED.stream,
            status = 1,
            is_live = 1,
            start_time = EXCLUDED.start_time,
            is_off = 0
        RETURNING {}, NULL AS distance
        "#,
        LIVE_COLUMNS
    );

    sqlx::query_as::<_, LiveEntity>(sql.as_str())
        .bind(uid)
        .bind(room_id)
        .bind(title)
        .bind(stream)
        .fetch_one(pool)
        .await
}

/// # REPO - 查询直播列表by ids（关注/收藏/看过）需要用
pub async fn repo_find_live_list_by_ids(
    pool: &PgPool,
    uid: i64,
) -> Result<u64, sqlx::Error> {
    let sql = r#"
        UPDATE live
        SET status = 0, is_live = 0, is_off = 1, off_time = EXTRACT(EPOCH FROM NOW())::BIGINT
        WHERE uid = $1 AND status = 1
    "#;

    let result = sqlx::query(sql)
        .bind(uid)
        .execute(pool)
        .await?;

    Ok(result.rows_affected())
}

// --- 查询操作 ---

/// # REPO - 新的
pub async fn repo_find_live_list_by_time(
    pool: &PgPool,
    limit: i64,
    offset: i64,
) -> Result<Vec<LiveEntity>, sqlx::Error> {
    let sql = format!(
        "SELECT {}, NULL AS distance FROM live WHERE status = 1 ORDER BY start_time DESC LIMIT $1 OFFSET $2",
        LIVE_COLUMNS
    );

    sqlx::query_as::<_, LiveEntity>(sql.as_str())
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

/// # REPO - 附近
pub async fn repo_find_live_list_by_geo(
    pool: &PgPool,
    lat: f64,
    lng: f64,
    limit: i64,
    offset: i64,
) -> Result<Vec<LiveEntity>, sqlx::Error> {
    // 使用 Postgres 的 <-> 算子计算经纬度点距离 (单位通过计算转换)
    let sql = format!(
        r#"
        SELECT {},
        (point(lng, lat) <-> point($1, $2)) * 111.325 AS distance
        FROM live
        WHERE status = 1
        ORDER BY distance ASC
        LIMIT $3 OFFSET $4
        "#,
        LIVE_COLUMNS
    );

    sqlx::query_as::<_, LiveEntity>(sql.as_str())
        .bind(lng)
        .bind(lat)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

/// # REPO - 更新热度/票数
pub async fn repo_change_live_votes_by_id(
    pool: &PgPool,
    uid: i64,
    votes: i64,
) -> Result<u64, sqlx::Error> {
    let sql = "UPDATE live SET hot_votes = hot_votes + $1 WHERE uid = $2";

    let result = sqlx::query(sql)
        .bind(votes)
        .bind(uid)
        .execute(pool)
        .await?;

    Ok(result.rows_affected())
}
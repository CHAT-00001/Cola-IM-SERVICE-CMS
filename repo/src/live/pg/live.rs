// pg/live.rs  - PG - live
// 2026/6/12 23:29

////////

use sqlx::PgPool;
use uuid::Uuid;
use cola_data::live::entity::live::LiveEntity;

////////

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

/// # REPO - 开启直播 (创建或更新直播状态)
/// 逻辑：生成无分隔符 UUID，使用 UPSERT 保证 uid 唯一
pub async fn repo_start_live(
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

/// # REPO - 停止直播
pub async fn repo_stop_live(
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

/// # REPO - 获取最新直播列表
pub async fn repo_find_new(
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

/// # REPO - 获取附近直播 (PostGIS 排序)
pub async fn repo_find_nearby(
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
pub async fn repo_add_votes(
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
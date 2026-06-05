// repo/src/pg/video/music  -- 存储 - PG - 短视频 - 信息
// 2026/5/19 15:07 by wx: cestbon10080

use sqlx::{self, PgPool};
use serde::Serialize;

// 数据表字段
const COLUMNS: &str = r#"id, user_id, title, thumb, href, add_time"#;

// 定义首页视频返回值结构体
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct VideoHomeItem {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub thumb: String,
    pub href: String,
    pub add_time: i64, // 假设使用时间戳，若是 NaiveDateTime 可改为 chrono::NaiveDateTime
    #[sqlx(default)]
    pub distance: Option<f64>, // 仅在“附近”查询时生效
}

////////

/// # [PG] - 通过 ID 查找单条视频数据
pub async fn pg_find_one_by_id(
    pool: &PgPool,
    id: i64, // 替换原有的 limit 和 offset，改为接收 id
) -> Result<Option<VideoHomeItem>, sqlx::Error> {
    // 1. 修改 SQL 语句，使用占位符 $1 来匹配 id
    let query = format!(
        "SELECT {} FROM video WHERE id = $1 LIMIT 1",
        COLUMNS
    );

    // 2. 使用 fetch_optional。如果找到则返回 Ok(Some(...))，没找到返回 Ok(None)
    //    如果数据库连接失败或语法错误，则直接返回 Err(sqlx::Error) 抛给调用层
    sqlx::query_as::<_, VideoHomeItem>(&query)
        .bind(id)
        .fetch_optional(pool)
        .await
}

////////

/// # [PG] - 通过一组 ID 批量查找视频数据
pub async fn pg_find_many_by_ids(
    pool: &PgPool,
    ids: &[i64], // 接收一个 id 的切片（Slice）
) -> Result<Vec<VideoHomeItem>, sqlx::Error> {
    // 如果传入的 id 列表为空，直接返回空数组，避免去数据库走弯路
    if ids.is_empty() {
        return Ok(vec![]);
    }

    // 使用 = ANY($1) 语法，性能比传统的 IN 更好，且完美契合 sqlx 的数组绑定
    let query = format!(
        "SELECT {} FROM video WHERE id = ANY($1)",
        COLUMNS
    );

    // 将 ids 传入，fetch_all 会返回所有匹配到的数据行
    sqlx::query_as::<_, VideoHomeItem>(&query)
        .bind(ids)
        .fetch_all(pool)
        .await
}

//////// END
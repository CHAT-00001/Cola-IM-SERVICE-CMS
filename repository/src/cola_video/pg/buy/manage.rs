// repository/src/cola_video/pg/buy/manage.rs
// 仓储 - VIDEO - pg - buy - manage 管理
// 2026/8/2 15:41 Created.

////////

use crate::pg_pool;
use cola_data::cola_video::entity::buy::{VIDEO_BUY_COLUMNS, VideoBuyEntity};
use sqlx::{self};

////////

/// # [MANAGE REPOSITORY] - 管理
/// * `desc`: `▶ 可乐视频 - 🛢 购买管理仓储`
pub struct VideoBuyManageRepo;

// 构造函数
impl VideoBuyManageRepo {
    //

    ////////

    /// # 1. [REPOSITORY] - 综合列表
    /// * `desc`: `管理员查看视频购买记录综合列表（支持多条件组合筛选）`
    pub async fn find_all_record_at_admin(
        _uid: i64,               // 操作者 ID（预留审计）
        user_id: Option<i64>,    // 用户 ID
        _keyword: Option<String>,// 关键词（预留）
        start_time: Option<i64>, // 开始时间
        end_time: Option<i64>,   // 结束时间
        video_id: Option<i64>,   // 视频 ID
        status: Option<i16>,     // 状态码
        limit: i64,              // 数量
        offset: i64,             // 偏移量
    ) -> Result<Vec<VideoBuyEntity>, sqlx::Error> {
        let pool = pg_pool();

        let query = format!(
            r#"
            SELECT {} FROM "cola_video"."buy"
            WHERE ($1::BIGINT IS NULL OR user_id = $1)
              AND ($2::BIGINT IS NULL OR video_id = $2)
              AND ($3::SMALLINT IS NULL OR status = $3)
              AND ($4::BIGINT IS NULL OR create_time >= $4)
              AND ($5::BIGINT IS NULL OR create_time <= $5)
              AND is_deleted = false
            ORDER BY id DESC
            LIMIT $6 OFFSET $7
            "#,
            VIDEO_BUY_COLUMNS
        );

        sqlx::query_as::<_, VideoBuyEntity>(&query)
            .bind(user_id)
            .bind(video_id)
            .bind(status)
            .bind(start_time)
            .bind(end_time)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 2. [REPOSITORY] - 单个操作
    /// * `desc`: `管理员更新单个购买记录状态`
    pub async fn reset_record_status_by_id(
        _uid: i64,   // 操作者 ID
        buy_id: i64, // 购买记录 ID
        status: i16, // 新状态码
    ) -> Result<u64, sqlx::Error> {
        let pool = pg_pool();

        let query = r#"
            UPDATE "cola_video"."buy"
            SET status = $1, updated_at = NOW()
            WHERE id = $2 AND is_deleted = false
        "#;

        let result = sqlx::query(query)
            .bind(status)
            .bind(buy_id)
            .execute(&pool)
            .await?;

        Ok(result.rows_affected())
    }

    ////////

    /// # 3. [REPOSITORY] - 批量操作
    /// * `desc`: `管理员批量更新购买记录状态`
    pub async fn reset_record_status_by_ids(
        _uid: i64,       // 操作者 ID
        buy_ids: &[i64], // 购买记录 IDs
        status: i16,     // 新状态码
    ) -> Result<u64, sqlx::Error> {
        if buy_ids.is_empty() {
            return Ok(0);
        }

        let pool = pg_pool();

        let query = r#"
            UPDATE "cola_video"."buy"
            SET status = $1, updated_at = NOW()
            WHERE id = ANY($2) AND is_deleted = false
        "#;

        let result = sqlx::query(query)
            .bind(status)
            .bind(buy_ids)
            .execute(&pool)
            .await?;

        Ok(result.rows_affected())
    }
}

//////// END
// repository/src/video/pg/danmaku/del.rs -- VIDEO - PG - 弹幕 - 删除仓储
// 2026/8/9 00:52 Created.

////////

use crate::pg_pool;
use cola_data::cola_video::entity::danmaku::VIDEO_DANMAKU_COLUMNS;
use sqlx::{self, Postgres};

////////

/// # [DELETE REPOSITORY] - 视频弹幕删除仓储
/// * `desc`: `COLA VIDEO - DELETE DANMAKU REPOSITORY`
pub struct VideoDanmakuDelRepo;

// 构造实现
impl VideoDanmakuDelRepo {
    //

    ////////

    /// # 1. [REPOSITORY] - 视频的
    /// * `desc`: `COLA VIDEO - 根据视频ID - 删除弹幕记录`
    pub async fn soft_delete_danmakus_by_video_id(
        video_id: i64, // 视频 ID
    ) -> Result<u64, sqlx::Error> {
        let pool = pg_pool();
        let now = chrono::Utc::now();
        let datetime = now.naive_utc();

        let query = format!(
            r#"
            UPDATE cola_video.danmaku
            SET is_deleted = true,
                deleted_at = $1,
                updated_at = $1
            WHERE video_id = $2 AND is_deleted = false
            RETURNING {}
            "#,
            VIDEO_DANMAKU_COLUMNS
        );

        let result = sqlx::query(&query)
            .bind(datetime)
            .bind(video_id)
            .execute(&pool)
            .await?;

        Ok(result.rows_affected())
    }

    ////////

    /// # 2. [REPOSITORY] - 用户的
    /// * `desc`: `COLA VIDEO - 根据用户ID - 删除用户的弹幕`
    pub async fn soft_delete_danmakus_by_user_id(
        user_id: i64, // 用户 ID
    ) -> Result<u64, sqlx::Error> {
        let pool = pg_pool();
        let now = chrono::Utc::now();
        let datetime = now.naive_utc();

        let query = format!(
            r#"
            UPDATE cola_video.danmaku
            SET is_deleted = true,
                deleted_at = $1,
                updated_at = $1
            WHERE uid = $2 AND is_deleted = false
            RETURNING {}
            "#,
            VIDEO_DANMAKU_COLUMNS
        );

        let result = sqlx::query(&query)
            .bind(datetime)
            .bind(user_id)
            .execute(&pool)
            .await?;

        Ok(result.rows_affected())
    }

    ////////

    /// # 3. [REPOSITORY] - 单条逻辑删除
    /// * `desc`: `单条软删除弹幕记录`
    pub async fn single_delete_danmakus_by_id(danmaku_id: i64) -> Result<u64, sqlx::Error> {
        let pool = pg_pool();
        let now = chrono::Utc::now();
        let datetime = now.naive_utc();

        let query = format!(
            r#"
            UPDATE cola_video.danmaku
            SET is_deleted = true,
                deleted_at = $1,
                updated_at = $1
            WHERE id = $2 AND is_deleted = false
            RETURNING {}
            "#,
            VIDEO_DANMAKU_COLUMNS
        );

        let result = sqlx::query(&query)
            .bind(datetime)
            .bind(danmaku_id)
            .execute(&pool)
            .await?;

        Ok(result.rows_affected())
    }

    ////////

    /// # 4. [REPOSITORY] - 批量逻辑删除
    /// * `desc`: `批量软删除弹幕记录`
    pub async fn batch_delete_danmaku_by_ids(
        danmaku_ids: &[i64], // 弹幕 IDs
    ) -> Result<u64, sqlx::Error> {
        if danmaku_ids.is_empty() {
            return Ok(0);
        }

        let pool = pg_pool();
        let now = chrono::Utc::now();
        let datetime = now.naive_utc();

        let query = format!(
            r#"
            UPDATE cola_video.danmaku
            SET is_deleted = true,
                deleted_at = $1,
                updated_at = $1
            WHERE id = ANY($2) AND is_deleted = false
            RETURNING {}
            "#,
            VIDEO_DANMAKU_COLUMNS
        );

        let result = sqlx::query(&query)
            .bind(datetime)
            .bind(danmaku_ids)
            .execute(&pool)
            .await?;

        Ok(result.rows_affected())
    }
}

//////// END

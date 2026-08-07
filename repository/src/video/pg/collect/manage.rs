// repository/src/video/pg/collect/manage.rs
// 仓储 - VIDEO - pg - collect - manage 管理
// 2026/8/2 15:41 Created.

////////

use crate::pg_pool;
use sqlx::{self, Postgres};

////////

/// # [MANAGE REPOSITORY] - 视频 收藏 管理 仓储
pub struct CollectManageRepo;

// 构造函数
impl CollectManageRepo {
    ////////

    /// # 1. [REPOSITORY] - 用户软删除单条收藏记录
    /// * `uid`: 用户 ID
    /// * `video_id`: 视频 ID
    /// * 将 `is_deleted` 置为 `true`，并记录 `deleted_at` 为当前时间
    pub async fn soft_delete_collect_by_video_id(
        uid: i64,
        video_id: i64,
    ) -> Result<u64, sqlx::Error> {
        let pool = pg_pool();
        let now = chrono::Utc::now();
        let datetime = now.naive_utc();

        let query = "
            UPDATE cola_video.collect
            SET is_deleted = true,
                deleted_at = $1,
                updated_at = $1
            WHERE user_id = $2 AND video_id = $3 AND is_deleted = false
        ";

        let result = sqlx::query(query)
            .bind(datetime)
            .bind(uid)
            .bind(video_id)
            .execute(&pool)
            .await?;

        Ok(result.rows_affected())
    }

    ////////

    /// # 2. [REPOSITORY] - 用户批量软删除收藏记录
    /// * `uid`: 用户 ID
    /// * `video_ids`: 视频 ID 列表
    /// * 将指定的多个视频收藏记录批量标记为软删除
    pub async fn batch_soft_delete_collect_by_video_ids(
        uid: i64,
        video_ids: &[i64],
    ) -> Result<u64, sqlx::Error> {
        if video_ids.is_empty() {
            return Ok(0);
        }

        let pool = pg_pool();
        let now = chrono::Utc::now();
        let datetime = now.naive_utc();

        let query = "
            UPDATE cola_video.collect
            SET is_deleted = true,
                deleted_at = $1,
                updated_at = $1
            WHERE user_id = $2 AND video_id = ANY($3) AND is_deleted = false
        ";

        let result = sqlx::query(query)
            .bind(datetime)
            .bind(uid)
            .bind(video_ids)
            .execute(&pool)
            .await?;

        Ok(result.rows_affected())
    }
}

//////// END
// repository/src/video/pg/buy/del.rs -- 仓储 - VIDEO - PG - 购买 - 删除仓储
// 2026/8/7 23:40 Created.

////////

use crate::pg_pool;
use sqlx::{self, Postgres};

////////

/// # [DELETE REPOSITORY] - 视频购买删除仓储
/// * `desc`: `VIDEO - Buy Delete Repository.`
pub struct VideoBuyDeleteRepo;

// 构造函数
impl VideoBuyDeleteRepo {
    //

    ////////

    /// # 1. [REPOSITORY] - 单条软删除
    /// `desc`: `根据ID - 软删除一条收藏记录`
    /// * `uid`: 用户 ID
    pub async fn single_soft_delete_record_by_id(
        uid: i64,    // 操作者 ID
        buy_id: i64, // 购买 ID
    ) -> Result<u64, sqlx::Error> {
        let pool = pg_pool();
        let now = chrono::Utc::now();
        let datetime = now.naive_utc();

        let query = "
            UPDATE cola_video.buy
            SET is_deleted = true,
                deleted_at = $1,
                updated_at = $1
            WHERE user_id = $2 AND video_id = $3 AND is_deleted = false
        ";

        let result = sqlx::query(query)
            .bind(datetime)
            .bind(uid)
            .bind(buy_id)
            .execute(&pool)
            .await?;

        Ok(result.rows_affected())
    }

    ////////

    /// # 2. [REPOSITORY] - 批量软删除
    /// * `desc`: `根据IDs - 批量软删除购买记录`
    pub async fn batch_soft_delete_record_by_ids(
        uid: i64,
        buy_ids: &[i64],
    ) -> Result<u64, sqlx::Error> {
        if buy_ids.is_empty() {
            return Ok(0);
        }

        let pool = pg_pool();
        let now = chrono::Utc::now();
        let datetime = now.naive_utc();

        let query = "
            UPDATE cola_video.buy
            SET is_deleted = true,
                deleted_at = $1,
                updated_at = $1
            WHERE user_id = $2 AND video_id = ANY($3) AND is_deleted = false
        ";

        let result = sqlx::query(query)
            .bind(datetime)
            .bind(uid)
            .bind(buy_ids)
            .execute(&pool)
            .await?;

        Ok(result.rows_affected())
    }
}

//////// END

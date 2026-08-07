// repository/src/video/pg/recommend/clean.rs
// 仓储 - VIDEO - pg - recommend - clean 清除
// 2026/8/2 13:41 Created.

////////

use crate::pg_pool;

////////

/// # [CLEAN REPOSITORY] - 视频 推荐/历史 清除 仓储
pub struct VideoRecommendCleanRepository;

impl VideoRecommendCleanRepository {
    //

    ////////

    /// # 1. [REPOSITORY] - 定时任务：物理删除失效的记录
    /// * 删除 `is_deleted = 1` 且 `deleted_at` 距今超过 180 天的物理记录
    /// * 使用 PostgreSQL 原生 `INTERVAL` 计算天数
    pub async fn pg_purge_expired_deleted_records() -> Result<u64, sqlx::Error> {
        let pool = pg_pool();

        let query = "
            DELETE FROM cola_video.recommend_record
            WHERE is_deleted = 1
              AND deleted_at < (NOW() - INTERVAL '180 days')
        ";

        let result = sqlx::query(query)
            .execute(&pool)
            .await?;

        Ok(result.rows_affected())
    }
}

//////// END
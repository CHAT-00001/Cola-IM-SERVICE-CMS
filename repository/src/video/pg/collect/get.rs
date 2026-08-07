// repository/src/video/pg/collect/get.rs
// 仓储 - VIDEO - pg - collect - get 获取
// 2026/8/2 15:41 Created.

////////

use crate::pg_pool;
use sqlx::{self, Postgres};
use cola_data::video::entity::collect::{VideoCollectEntity, VIDEO_COLLECT_COLUMNS};

////////

/// # [GET REPOSITORY] - 视频 收藏 获取 仓储
pub struct CollectGetRepo;

// 构造函数
impl CollectGetRepo {
    ////////

    /// # 1. [REPOSITORY] - 获取用户收藏记录的视频 IDs (带分页)
    /// * `user_id`: 用户ID
    /// * `limit`: 返回数量限制
    /// * `offset`: 分页偏移量
    pub async fn find_collect_ids_by_user_id(
        user_id: i64,
        keyword: Option<String>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<i64>, sqlx::Error> {
        let pool = pg_pool();

        let query = "
            SELECT video_id
            FROM cola_video.collect
            WHERE uid = $1 AND status = 1 AND is_deleted = false
            ORDER BY add_time DESC
            LIMIT $2 OFFSET $3
        ";

        sqlx::query_scalar::<_, i64>(query)
            .bind(user_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 2. [REPOSITORY] - 根据视频 ID 获取该视频下的所有收藏记录列表 (带分页)
    /// * `video_id`: 视频ID
    /// * `limit`: 返回数量限制
    /// * `offset`: 分页偏移量
    /// * 映射为 `VideoCollectEntity`
    pub async fn find_collect_records_by_video_id(
        video_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoCollectEntity>, sqlx::Error> {
        let pool = pg_pool();

        let query = format!(
            "SELECT {}
             FROM cola_video.collect
             WHERE video_id = $1 AND status = 1 AND is_deleted = false
             ORDER BY add_time DESC
             LIMIT $2 OFFSET $3",
            VIDEO_COLLECT_COLUMNS
        );

        sqlx::query_as::<_, VideoCollectEntity>(&query)
            .bind(video_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }
}

//////// END
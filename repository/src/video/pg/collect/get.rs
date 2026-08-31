// repository/src/video/pg/collect/get.rs
// 2026/8/2 15:41 Created.

////////

use crate::pg_pool;
use cola_data::cola_video::entity::collect::{VIDEO_COLLECT_COLUMNS, VideoCollectEntity};
use sqlx;

////////

/// # [GET REPOSITORY] - 获取
/// * `desc`: `▶ 可乐视频 - 🛢 收藏获取仓储`
pub struct CollectGetRepo;

// 构造实现
impl CollectGetRepo {
    //

    ////////

    /// # 1. [REPOSITORY] - 用户收藏的视频 IDs
    /// * `desc`: `根据用户ID获取她收藏视频IDs仓储`
    pub async fn find_video_ids_by_user_id(
        user_id: i64, // 用户 ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> Result<Vec<i64>, sqlx::Error> {
        let pool = pg_pool();

        let query = "
            SELECT video_id
            FROM \"cola_video\".\"collect\"
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

    /// # 2. [REPOSITORY] - 视频的
    /// *  `desc`: `根据该视频ID - 查找他的收藏记录列表`
    pub async fn find_collect_records_by_video_id(
        video_id: i64, // 视频 ID
        limit: i64,    // 数量
        offset: i64,   // 页码
    ) -> Result<Vec<VideoCollectEntity>, sqlx::Error> {
        let pool = pg_pool();

        let query = format!(
            "SELECT {}
             FROM \"cola_video\".\"collect\"
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

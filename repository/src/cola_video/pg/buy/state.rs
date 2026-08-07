// repository/src/cola_video/pg/collect/ban
// 仓储 - VIDEO - pg - collect - state 状态
// 2026/8/2 15:40 Created.

////////

use crate::pg_pool;
use sqlx::{self, Postgres};

////////

/// # [STATE REPOSITORY] - 视频 收藏 状态 仓储
pub struct CollectStateRepo;

// 构造函数
impl CollectStateRepo {
    //

    ////////

    /// # 1. [REPOSITORY] - 检查用户是否已收藏指定视频 (双向命中：uid + video_id)
    /// * `uid`: 用户ID
    /// * `video_id`: 视频ID
    /// * 返回: bool (当 status = 1 且 (is_deleted = false 或 is_deleted IS NULL) 时返回 true，否则返回 false)
    pub async fn check_is_collected(
        uid: i64,
        video_id: i64,
    ) -> Result<bool, sqlx::Error> {
        let pool = pg_pool();

        let query = "
            SELECT EXISTS (
                SELECT 1
                FROM cola_video.collect
                WHERE user_id = $1
                  AND video_id = $2
                  AND status = 1
                  AND (is_deleted = false OR is_deleted IS NULL)
            )
        ";

        let is_collected: bool = sqlx::query_scalar(query)
            .bind(uid)
            .bind(video_id)
            .fetch_one(&pool)
            .await?;

        Ok(is_collected)
    }
}

//////// END
// repository/src/video/pg/buy/stat.rs -- 仓储 - VIDEO - PG - 购买 - 统计仓储
// 2026/8/2 15:40 Created.

////////

use crate::pg_pool;
use sqlx::{self, Postgres};

////////

/// # [STAT REPOSITORY] - 视频购买统计仓储
/// * `DESC`: `VIDEO - Buy Stat Repository.`
pub struct VideoBuyStatRepo;

// 构造函数
impl VideoBuyStatRepo {
    //

    ////////

    /// # 1. [REPOSITORY] - 检查用户是否已收藏指定视频 (双向命中：uid + video_id)
    /// * `uid`: 用户ID
    /// * `video_id`: 视频ID
    /// * 返回: bool (当 status = 1 且 (is_deleted = false 或 is_deleted IS NULL) 时返回 true，否则返回 false)
    pub async fn check_is_collected(uid: i64, video_id: i64) -> Result<bool, sqlx::Error> {
        let pool = pg_pool();

        let query = "
            SELECT EXISTS (
                SELECT 1
                FROM cola_video.buy
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

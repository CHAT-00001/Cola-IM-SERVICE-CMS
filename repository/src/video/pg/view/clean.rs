// repository/src/video/pg/view/clean.rs
// 仓储 - VIDEO - pg - view - clean 清楚
// 2026/8/2 13:21 Created.

////////

use crate::pg_pool;
use cola_data::video::entity::view::{VIDEO_VIEW_COLUMNS, VideoViewEntity};
use sqlx::{self, Postgres};

////////

/// # [ADD REPOSITORY] - 清理 仓储
pub struct VideoViewCleanRepo;

impl VideoViewCleanRepo {
    //

    ////////

    /// # 6. [REPOSITORY] - 定时任务 - 物理删除过期的历史记录
    /// * 删除早于指定时间戳（如过期阈值）的记录
    pub async fn pg_clean_expired_history(expire_before_time: i64) -> Result<u64, sqlx::Error> {
        let pool = pg_pool();
        let query = "
            DELETE FROM cola_video.view_history
            WHERE addtime < $1
        ";

        let result = sqlx::query(query)
            .bind(expire_before_time)
            .execute(&pool)
            .await?;

        Ok(result.rows_affected())
    }
}

//////// END

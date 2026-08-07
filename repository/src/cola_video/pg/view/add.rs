// repository/src/cola_video/pg/view/active
// 仓储 - VIDEO - pg - view - add 添加
// 2026/8/2 13:18 Created.

////////

use crate::pg_pool;
use cola_data::cola_video::entity::view::{VideoViewEntity, VIDEO_VIEW_COLUMNS};
use sqlx::{self, Postgres};

////////

/// # [ADD REPOSITORY] - 浏览历史 添加
pub struct VideoViewAddRepo;

impl VideoViewAddRepo {
    ////////

    /// # 1. [REPOSITORY] - 插入或更新浏览记录 (uid 和 video_id 双命中)
    /// * 如果存在则更新浏览时间及状态，如果不存在则插入新记录
    pub async fn pg_save_or_update(uid: i64, video_id: i64) -> Result<(), sqlx::Error> {
        let pool = pg_pool();
        let now = chrono::Utc::now().timestamp();

        let query = r#"
            INSERT INTO cola_video.view_history (uid, video_id, addtime, is_deleted, status)
            VALUES ($1, $2, $3, 0, 1)
            ON CONFLICT (uid, video_id)
            DO UPDATE SET
                addtime = EXCLUDED.addtime,
                is_deleted = 0,
                status = 1
        "#;

        sqlx::query(query)
            .bind(uid)
            .bind(video_id)
            .bind(now)
            .execute(&pool)
            .await?;

        Ok(())
    }
}

//////// END
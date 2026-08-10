// repository/src/cola_video/pg/buy/get.rs
// 仓储 - VIDEO - pg - buy - get 获取
// 2026/8/2 15:41 Created.

////////

use crate::pg_pool;
use sqlx::{self, Postgres};
use cola_data::cola_video::entity::buy::{VideoBuyEntity, VIDEO_BUY_COLUMNS};

////////

/// # [GET REPOSITORY] - 视频 购买 获取 仓储
pub struct VideoBuyGetRepo;

// 构造函数
impl VideoBuyGetRepo {
    //

    ////////

    /// # 1. [REPOSITORY] - 用户的
    /// * `desc`: `根据用户ID` - `获取购买的视频IDs`
    pub async fn find_video_ids_by_user_id(
        user_id: i64, // 用户 ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> Result<Vec<i64>, sqlx::Error> {
        let pool = pg_pool();

        let query = "
            SELECT video_id
            FROM cola_video.buy
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

    /// # 2. [REPOSITORY] - 根据视频 ID 获取该视频下的所有购买记录列表 (带分页)
    /// * `video_id`: 视频ID
    /// * `limit`: 返回数量限制
    /// * `offset`: 分页偏移量
    /// * 映射为 `VideoBuyEntity`
    pub async fn find_buy_records_by_video_id(
        video_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoBuyEntity>, sqlx::Error> {
        let pool = pg_pool();

        let query = format!(
            "SELECT {}
             FROM cola_video.buy
             WHERE video_id = $1 AND status = 1 AND is_deleted = false
             ORDER BY add_time DESC
             LIMIT $2 OFFSET $3",
            VIDEO_BUY_COLUMNS
        );

        sqlx::query_as::<_, VideoBuyEntity>(&query)
            .bind(video_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }
}

//////// END

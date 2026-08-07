// repository/src/video/pg/recommend/active
// 仓储 - VIDEO - pg - recommend - add 添加
// 2026/8/2 13:39 Created.

////////

use crate::pg_pool;
use cola_data::video::command::recommend::RecommendCommand;

////////

/// # [ADD REPOSITORY] - 视频 推荐记录 添加 仓储
pub struct VideoRecommendAddRepository;

// 构造实现
impl VideoRecommendAddRepository {
    //

    ////////

    /// # 1. [REPOSITORY] - 保存推荐记录 (支持 upsert，双命中 uid 和 video_id)
    /// * 状态处理：status 设为传入值（如 1 激活），
    /// * `is_deleted` 归零，`deleted_at` 置空，更新操作时间
    pub async fn pg_save_recommend_record(
        uid: i64,
        video_id: i64,
        status: i16,
        cmd: &RecommendCommand,
    ) -> Result<u64, sqlx::Error> {
        let pool = pg_pool();

        let now = chrono::Utc::now();
        let timestamp = now.timestamp();
        let datetime = now.naive_utc();

        let query = "
            INSERT INTO cola_video.recommend_record
            (user_id, video_id, remark, status, is_deleted, deleted_at, add_time, created_at, updated_at)
            VALUES ($1, $2, $3, $4, 0, NULL, $5, $6, $6)
            ON CONFLICT (user_id, video_id)
            DO UPDATE SET
                remark = EXCLUDED.remark,
                status = EXCLUDED.status,
                is_deleted = 0,
                deleted_at = NULL,
                updated_at = EXCLUDED.updated_at
        ";

        let result = sqlx::query(query)
            .bind(uid)         // $1
            .bind(video_id)    // $2
            .bind(&cmd.remark) // $3
            .bind(status)      // $4
            .bind(timestamp)   // $5
            .bind(datetime)    // $6
            .execute(&pool)
            .await?;

        Ok(result.rows_affected())
    }
}

//////// END
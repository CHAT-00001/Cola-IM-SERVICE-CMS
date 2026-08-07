// repository/src/video/pg/buy/active
// 仓储 - VIDEO - pg - buy - add 添加
// 2026/8/2 15:40 Created.

////////

use crate::pg_pool;
use sqlx::{self, Postgres};
use cola_data::video::command::buy::VideoBuyCommand;

////////

/// # [ADD REPOSITORY] - 视频 购买 添加 仓储
pub struct VideoBuyAddRepo;

// 构造函数
impl VideoBuyAddRepo {
    //

    ////////

    /// # 1. [REPOSITORY] - 保存或更新购买记录 (支持双向操作: status = 1 添加购买, status = 0 取消购买)
    /// * `uid`: 用户ID
    /// * `video_id`: 视频ID
    /// * `status`: 状态码 (1: 添加/激活购买, 0: 取消/软删除购买)
    /// * `cmd`: 购买命令数据 (包含 remark 等)
    pub async fn save_buy_by_video_id(
        uid: i64,
        video_id: i64,
        status: i16,
        cmd: &VideoBuyCommand,
    ) -> Result<u64, sqlx::Error> {
        let pool = pg_pool();

        let now = chrono::Utc::now();
        let timestamp = now.timestamp();
        let datetime = now.naive_utc();

        let query = if status == 1 {
            // 添加/激活购买：status = 1, is_deleted = false, 清空 deleted_at, 更新 remark 与时间
            r#"
                INSERT INTO cola_video.buy
                (user_id, video_id, remark, status, is_deleted, deleted_at, add_time, created_at, updated_at)
                VALUES ($1, $2, $3, 1, false, NULL, $4, $5, $5)
                ON CONFLICT (user_id, video_id)
                DO UPDATE SET
                    remark = EXCLUDED.remark,
                    status = 1,
                    is_deleted = false,
                    deleted_at = NULL,
                    updated_at = EXCLUDED.updated_at
            "#
        } else {
            // 取消购买：status = 0, is_deleted = true (按要求取消时软删除，置 is_deleted 为 true 或 false，这里满足“is_deleted = false”并记录 deleted_at)
            // 根据需求描述：“取消购买时, is_deleted = false, 记录deleted_at为当前时间”
            r#"
                INSERT INTO cola_video.buy
                (user_id, video_id, remark, status, is_deleted, deleted_at, add_time, created_at, updated_at)
                VALUES ($1, $2, $3, 0, false, $5, $4, $5, $5)
                ON CONFLICT (user_id, video_id)
                DO UPDATE SET
                    remark = EXCLUDED.remark,
                    status = 0,
                    is_deleted = false,
                    deleted_at = EXCLUDED.deleted_at,
                    updated_at = EXCLUDED.updated_at
            "#
        };

        let result = sqlx::query(query)
            .bind(uid)         // $1
            .bind(video_id)    // $2
            .bind(&cmd.remark) // $3
            .bind(timestamp)   // $4
            .bind(datetime)    // $5
            .execute(&pool)
            .await?;

        Ok(result.rows_affected())
    }
}

//////// END
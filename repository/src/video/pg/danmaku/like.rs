// repository/src/video/pg/danmaku/like.rs -- VIDEO - PG - 弹幕 - 点赞仓储
// 2026/9/7 16:39 Created.

////////

use crate::pg_pool;
use cola_data::cola_video::command::danmaku::DanmakuCommand;
use cola_data::cola_video::entity::danmaku::{DanmakuEntity, VIDEO_DANMAKU_COLUMNS};
use sqlx::{self, Postgres, QueryBuilder};

////////

/// # [LIKE REPO] - 视频弹幕点赞仓储
pub struct DanmakuLikeRepo;

impl DanmakuLikeRepo {
    //

    ////////

    /// # 1. [REPOSITORY] - 保存弹幕点赞
    /// * `DESC`: `根据弹幕ID + 用户ID - 保存弹幕点赞`
    pub async fn save_danmaku_like(
        user_id: i64,    // 操作者 ID
        danmaku_id: i64, // 弹幕 ID
        status: i16,     // 状态码
    ) -> Result<bool, sqlx::Error> {
        let pool = pg_pool();

        // 如果存在记录（无论是正常还是被逻辑删除过），则更新状态、重置逻辑删除标记并清空 deleted_at；如果不存在则插入新记录
        let query = r#"
            INSERT INTO cola_video.danmaku_like (user_id, danmaku_id, status, is_deleted, deleted_at, created_at, updated_at)
            VALUES ($1, $2, $3, false, NULL, NOW(), NOW())
            ON CONFLICT (user_id, danmaku_id)
            DO UPDATE SET
                status = EXCLUDED.status,
                is_deleted = false,
                deleted_at = NULL,
                updated_at = NOW()
        "#;

        let result = sqlx::query(query)
            .bind(user_id)
            .bind(danmaku_id)
            .bind(status)
            .execute(&pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    ////////

    /// # 2. [REPOSITORY] - 取消点赞
    /// * ``: `根据弹幕ID + 用户ID - 取消弹幕点赞
    pub async fn delete_danmaku_like(
        user_id: i64,    // 用户 ID
        danmaku_id: i64, // 弹幕 ID
    ) -> Result<bool, sqlx::Error> {
        let pool = pg_pool();

        // 1. 构建逻辑删除的 SQL（不执行物理 DELETE）
        let sql = r#"
            UPDATE cola_video.danmaku_like
            SET is_deleted = true,
                deleted_at = NOW(),
                updated_at = NOW()
            WHERE user_id = $1
              AND danmaku_id = $2
              AND is_deleted = false
        "#;

        // 2. 执行更新
        let result = sqlx::query(sql)
            .bind(user_id)
            .bind(danmaku_id)
            .execute(&pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    ////////

    /// # 3. [REPOSITORY] - 检查是否点赞
    /// * `desc`: `user_id + video_id - 检查是否存在点赞关系`
    pub async fn check_is_liked(
        user_id: i64,    // 用户 ID
        danmaku_id: i64, // 弹幕 ID
    ) -> Result<bool, sqlx::Error> {
        let pool = pg_pool();

        let query = r#"
            SELECT EXISTS (
                SELECT 1
                FROM cola_video.danmaku_like
                WHERE user_id = $1
                  AND danmaku_id = $2
                  AND status = 1
                  AND is_deleted = false
            )
        "#;

        let exists: bool = sqlx::query_scalar(query)
            .bind(user_id)
            .bind(danmaku_id)
            .fetch_one(&pool)
            .await?;

        Ok(exists)
    }
}

//////// END

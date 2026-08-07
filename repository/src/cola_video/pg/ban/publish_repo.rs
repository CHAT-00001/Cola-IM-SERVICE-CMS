// repository/src/new/pg/banned/publish_repo.rs
// 仓储 - new - pg - ban - 发布功能
// 2026/8/1 14:32

////////

use crate::pg_pool;
use chrono::{DateTime, Utc};
use cola_data::cola_video::entity::banned::publish::{
    VIDEO_BANNED_PUBLISH_COLUMNS, VideoBannedPublishEntity,
};

////////

/// # [BANNED REPOSITORY] - 短视频 - 发布功能 - 封禁
pub struct VideoBannedPublishRepo;

impl VideoBannedPublishRepo {
    //

    ////////

    /// # 1. [REPOSITORY] - 根据用户ID 保存或更新封禁记录
    pub async fn save_banned_by_user_id(
        operator_uid: i64,               // 操作者 ID
        uid: i64,                        // 目标用户ID
        begin_at: Option<DateTime<Utc>>, // 开始时间
        end_at: Option<DateTime<Utc>>,   // 结束时间
        reason: Option<String>,          // 封禁原因
    ) -> Result<(), sqlx::Error> {
        let pool = pg_pool();

        let query = r#"
            INSERT INTO cola_video.banned_publish (
                uid, operator_uid, begin_at, end_at, reason, status, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, 1, NOW(), NOW())
            ON CONFLICT (uid)
            DO UPDATE SET
                operator_uid = EXCLUDED.operator_uid,
                begin_at = EXCLUDED.begin_at,
                end_at = EXCLUDED.end_at,
                reason = EXCLUDED.reason,
                status = EXCLUDED.status,
                updated_at = NOW()
        "#;

        sqlx::query(query)
            .bind(uid)
            .bind(operator_uid.to_string())
            .bind(begin_at)
            .bind(end_at)
            .bind(reason)
            .execute(&pool)
            .await?;

        Ok(())
    }

    ////////

    /// # 2. [REPOSITORY] - 根据用户ID 查找用户的短视频封禁记录
    pub async fn find_banned_by_user_id(
        uid: i64,
    ) -> Result<Option<VideoBannedPublishEntity>, sqlx::Error> {
        let pool = pg_pool();

        let query = format!(
            "SELECT {} FROM cola_video.banned_publish WHERE uid = $1 AND (is_deleted = false OR is_deleted IS NULL) LIMIT 1",
            VIDEO_BANNED_PUBLISH_COLUMNS
        );

        sqlx::query_as::<_, VideoBannedPublishEntity>(&query)
            .bind(uid)
            .fetch_optional(&pool)
            .await
    }

    ////////

    /// # 3. [REPOSITORY] - 根据用户ID 更新用户封禁记录
    pub async fn update_banned_by_user_id(
        operator_uid: i64,               // 操作者 ID
        uid: i64,                        // 目标用户ID
        begin_at: Option<DateTime<Utc>>, // 封禁开始时间
        end_at: Option<DateTime<Utc>>,   // 封禁结束时间
        reason: Option<String>,          // 封禁原因
    ) -> Result<(), sqlx::Error> {
        let pool = pg_pool();

        let query = r#"
            UPDATE cola_video.banned_publish
            SET
                operator_uid = $2,
                begin_at = $3,
                end_at = $4,
                reason = $5,
                updated_at = NOW()
            WHERE uid = $1
        "#;

        sqlx::query(query)
            .bind(uid)
            .bind(operator_uid.to_string())
            .bind(begin_at)
            .bind(end_at)
            .bind(reason)
            .execute(&pool)
            .await?;

        Ok(())
    }

    ////////

    /// # 4. [REPOSITORY] - 软删除用户封禁记录
    pub async fn soft_delete_banned_by_user_id(uid: i64) -> Result<(), sqlx::Error> {
        let pool = pg_pool();

        let query = r#"
            UPDATE cola_video.banned_publish
            SET
                is_deleted = true,
                deleted_at = NOW(),
                updated_at = NOW()
            WHERE uid = $1
        "#;

        sqlx::query(query).bind(uid).execute(&pool).await?;

        Ok(())
    }

    ////////

    /// # 5. [REPOSITORY] - 分页查找所有的封禁记录
    /// * `desc`: `管理员`
    /// * `offset`: 分页偏移量
    /// * `limit`: 每页数量
    pub async fn find_all_banned_record(
        offset: i64,
        limit: i64,
    ) -> Result<Vec<VideoBannedPublishEntity>, sqlx::Error> {
        let pool = pg_pool();

        let query = format!(
            r#"
            SELECT {}
            FROM cola_video.banned_publish
            WHERE is_deleted = false OR is_deleted IS NULL
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
        "#,
            VIDEO_BANNED_PUBLISH_COLUMNS
        );

        let records = sqlx::query_as::<_, VideoBannedPublishEntity>(&query)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await?;

        Ok(records)
    }

    ////////
}

//////// END

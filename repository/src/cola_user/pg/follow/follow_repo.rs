// repository/src/cola_user/pg/follow_repo.rs  -- 仓储中心 USER pg 关注
// 2026/6/29 03:54

////////

use crate::pg_pool;
use app_config::GLOBAL_DB;
use cola_data::user::entity::user::UserEntity;
use sqlx::{self, PgPool};

//////

/// # [SERVICE] - 用户 关注
pub struct UserFollowRepo;

// 构造函数
impl UserFollowRepo {
    ////////

    /// # 1. [REPOSITORY] - 关注列表
    /// * `desc`: 主动关注
    /// * `sort`: 更新时间降序
    pub async fn find_follow_ids_by_uid(
        uid: i64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<i64>, sqlx::Error> {
        let pool = pg_pool();

        // 使用 query_scalar，并指明映射类型为 i64
        let query = "SELECT follow_id FROM \"user_follow\" WHERE uid = $1 AND status = 1 ORDER BY id DESC LIMIT $2 OFFSET $3";

        sqlx::query_scalar::<_, i64>(query)
            .bind(uid)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 2. [REPOSITORY] - 被关注列表 (获取谁关注了我)
    /// * `desc`: 被动关注
    pub async fn find_follower_ids_by_user_id(
        user_id: i64, // 被关注者ID
        limit: i64,
        offset: i64,
    ) -> Result<Vec<i64>, sqlx::Error> {
        let pool = pg_pool();

        // 只查询 uid (关注者 ID)，且不进行 JOIN，直接查 user_follow 表
        let query = "SELECT uid FROM \"user_follow\" WHERE user_id = $1 AND status = 1 ORDER BY id DESC LIMIT $2 OFFSET $3";

        sqlx::query_scalar::<_, i64>(query)
            .bind(user_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 3. [REPOSITORY] - 保存关注记录
    pub async fn save_follow_record(
        uid: i64,       // 当前用户
        user_id: i64,   // 目标用户
        remark: String, // 备注
        status: i16,    // 状态 (1: 关注, 0: 取消)
    ) -> Result<u64, sqlx::Error> {
        // 返回受影响的行数或 ()
        let pool = pg_pool();

        // 使用 ON CONFLICT 处理“重复关注”的情况
        let query = r#"
        INSERT INTO "user_follow" (uid, user_id,remark, status, created_at)
        VALUES ($1, $2, $3, $4, NOW())
        ON CONFLICT (uid, user_id)
        DO UPDATE SET status = EXCLUDED.status, updated_at = NOW()
    "#;

        let result = sqlx::query(query)
            .bind(uid)
            .bind(user_id)
            .bind(remark)
            .bind(status)
            .execute(&pool)
            .await?;

        Ok(result.rows_affected())
    }

    ////////

    /// # 4. [REPOSITORY] - 批量注销/取消关注
    /// * `desc`: 用户注销时，将其发起的所有关注记录设为失效 (status = 0)
    pub async fn soft_delete_follows_by_uid(uid: i64) -> Result<u64, sqlx::Error> {
        let pool = pg_pool();

        // 逻辑：用户注销时，标记为已删除，并记录时间，以便后续脚本硬删除
        let query = r#"
        UPDATE "user_follow"
        SET is_deleted = 1, deleted_at = NOW(), status = 0
        WHERE uid = $1 AND is_deleted = 0
    "#;

        let result = sqlx::query(query).bind(uid).execute(&pool).await?;

        Ok(result.rows_affected())
    }

    ////////

    /// # 5. [REPOSITORY] - 软删除被关注记录
    /// * `desc`: 当被关注者注销时，将所有关注该用户的记录标记为失效
    pub async fn soft_delete_follows_by_user_id(user_id: i64) -> Result<u64, sqlx::Error> {
        let pool = pg_pool();

        // 逻辑：将所有关注了该用户的记录标记为已删除
        let query = r#"
        UPDATE "user_follow"
        SET is_deleted = 1, deleted_at = NOW(), status = 0
        WHERE user_id = $1 AND is_deleted = 0
    "#;

        let result = sqlx::query(query).bind(user_id).execute(&pool).await?;

        Ok(result.rows_affected())
    }

    ////////

    /// # 6. [REPOSITORY] - 星标关注
    pub async fn find_star_follow_ids(
        uid: i64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<i64>, sqlx::Error> {
        let pool = pg_pool();

        // 使用 query_scalar，并指明映射类型为 i64
        let query = "SELECT follow_id FROM \"user_follow\" WHERE uid = $1 AND status = 1 AND star = 1 ORDER BY id DESC LIMIT $2 OFFSET $3";

        sqlx::query_scalar::<_, i64>(query)
            .bind(uid)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////


}

//////// END

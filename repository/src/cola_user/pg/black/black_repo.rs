// repository/src/cola_user/pg/black_repo.rs  -- 仓储中心 USER pg 拉黑
// 2026/6/29 03:54

////////

use crate::pg_pool;
use app_config::GLOBAL_DB;
use cola_data::user::entity::user::UserEntity;
use sqlx::{self, PgPool};

//////

/// # [SERVICE] - 用户 黑名单
pub struct UserBlackRepo;

// 构造函数
impl UserBlackRepo {
    ////////

    /// # 1. [REPOSITORY] - 拉黑列表
    /// * `desc`: 主动拉黑
    /// * `sort`: 更新时间降序
    pub async fn find_black_ids_by_uid(
        uid: i64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<i64>, sqlx::Error> {
        let pool = pg_pool();

        // 使用 query_scalar，并指明映射类型为 i64
        let query = "SELECT black_id FROM \"user_black\" WHERE uid = $1 AND status = 1 ORDER BY id DESC LIMIT $2 OFFSET $3";

        sqlx::query_scalar::<_, i64>(query)
            .bind(uid)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 2. [REPOSITORY] - 被拉黑列表 (获取谁拉黑了我)
    /// * `desc`: 被动拉黑
    pub async fn find_blacker_ids_by_user_id(
        user_id: i64, // 被拉黑者ID
        limit: i64,
        offset: i64,
    ) -> Result<Vec<i64>, sqlx::Error> {
        let pool = pg_pool();

        // 只查询 uid (拉黑者 ID)，且不进行 JOIN，直接查 user_black 表
        let query = "SELECT uid FROM \"user_black\" WHERE user_id = $1 AND status = 1 ORDER BY id DESC LIMIT $2 OFFSET $3";

        sqlx::query_scalar::<_, i64>(query)
            .bind(user_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 3. [REPOSITORY] - 保存拉黑记录
    pub async fn save_black_record(
        uid: i64,       // 当前用户
        user_id: i64,   // 目标用户
        remark: String, // 备注
        status: i16,    // 状态 (1: 拉黑, 0: 取消)
    ) -> Result<u64, sqlx::Error> {
        // 返回受影响的行数或 ()
        let pool = pg_pool();

        // 使用 ON CONFLICT 处理“重复拉黑”的情况
        let query = r#"
        INSERT INTO "user_black" (uid, user_id,remark, status, created_at)
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

    /// # 4. [REPOSITORY] - 批量注销/取消拉黑
    /// * `desc`: 用户注销时，将其发起的所有拉黑记录设为失效 (status = 0)
    pub async fn soft_delete_blacks_by_uid(uid: i64) -> Result<u64, sqlx::Error> {
        let pool = pg_pool();

        // 逻辑：用户注销时，标记为已删除，并记录时间，以便后续脚本硬删除
        let query = r#"
        UPDATE "user_black"
        SET is_deleted = 1, deleted_at = NOW(), status = 0
        WHERE uid = $1 AND is_deleted = 0
    "#;

        let result = sqlx::query(query).bind(uid).execute(&pool).await?;

        Ok(result.rows_affected())
    }

    ////////

    /// # 5. [REPOSITORY] - 软删除被拉黑记录
    /// * `desc`: 当被拉黑者注销时，将所有拉黑该用户的记录标记为失效
    pub async fn soft_delete_blacks_by_user_id(user_id: i64) -> Result<u64, sqlx::Error> {
        let pool = pg_pool();

        // 逻辑：将所有拉黑了该用户的记录标记为已删除
        let query = r#"
        UPDATE "user_black"
        SET is_deleted = 1, deleted_at = NOW(), status = 0
        WHERE user_id = $1 AND is_deleted = 0
    "#;

        let result = sqlx::query(query).bind(user_id).execute(&pool).await?;

        Ok(result.rows_affected())
    }

    ////////

    /// # 6. [REPOSITORY] - 星标拉黑
    pub async fn find_star_black_ids(
        uid: i64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<i64>, sqlx::Error> {
        let pool = pg_pool();

        // 使用 query_scalar，并指明映射类型为 i64
        let query = "SELECT black_id FROM \"user_black\" WHERE uid = $1 AND status = 1 AND star = 1 ORDER BY id DESC LIMIT $2 OFFSET $3";

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

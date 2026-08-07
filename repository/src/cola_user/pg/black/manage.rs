// repository/src/cola_user/pg/black/manage.rs
// 仓储中心 - USER - pg - black - manage 管理
// 2026/8/3 01:23 Created.

////////

use crate::pg_pool;
use app_config::GLOBAL_DB;
use cola_data::cola_user::entity::user::UserEntity;
use sqlx::{self, PgPool};

////////

/// # [MANAGE REPOSITORY] - 管理
/// * `desc`: `仓储层用户黑名单管理仓储`
pub struct UserBlackManageRepo;

// 构造函数
impl UserBlackManageRepo {
    //

    ////////

    /// # 4. [REPOSITORY] - 批量注销/取消拉黑
    /// * `desc`: 用户注销时，将其发起的所有拉黑记录设为失效 (status = 0)
    pub async fn soft_delete_blacks_by_uid(uid: i64) -> Result<u64, sqlx::Error> {
        let pool = pg_pool();

        // 逻辑：用户注销时，标记为已删除，并记录时间，以便后续脚本硬删除
        let query = r#"
        UPDATE "cola_user.black"
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
        UPDATE "cola_user.black"
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
        let query = "SELECT black_id FROM \"cola_user.black\" WHERE uid = $1 AND status = 1 AND star = 1 ORDER BY id DESC LIMIT $2 OFFSET $3";

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

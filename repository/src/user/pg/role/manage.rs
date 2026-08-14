// repository/src/user/pg/role/mod.rs
// 仓储 - 可乐用户 - pg - 角色 - 模块
// 2026/8/3 12:58 Created.

////////

use crate::pg_pool;
use app_config::GLOBAL_DB;
use sqlx::{self, PgPool};

//////

/// # [MANAGE REPOSITORY] - 用户 角色 管理 仓储
pub struct UserRoleManageRepo;

// 构造函数
impl UserRoleManageRepo {
    //

    ////////

    /// # 1. [REPOSITORY] - 🚮 单个软删除
    pub async fn pg_single_soft_del_by_id(id: i64, // 角色ID
    ) -> Result<u64, sqlx::Error> {
        let pool = pg_pool();

        // 修复表名格式和查询条件：根据主键 id 进行单个软删除
        let query = r#"
        UPDATE "user"."role"
        SET is_deleted = 1, deleted_at = NOW(), status = 0
        WHERE id = $1 AND is_deleted = 0
    "#;

        let result = sqlx::query(query).bind(id).execute(&pool).await?;

        Ok(result.rows_affected())
    }

    ////////

    /// # 2. [REPOSITORY] - 🚮 批量软删除（遍历/批量删除）
    /// * `desc`: 根据一组角色 ID 批量软删除
    pub async fn pg_batch_soft_del_by_ids(
        ids: Vec<i64>, // 角色ID列表
    ) -> Result<u64, sqlx::Error> {
        let pool = pg_pool();

        if ids.is_empty() {
            return Ok(0);
        }

        // 修复表名格式，并使用 ANY($1) 进行批量匹配删除
        let query = r#"
        UPDATE "user"."role"
        SET is_deleted = 1, deleted_at = NOW(), status = 0
        WHERE id = ANY($1) AND is_deleted = 0
    "#;

        let result = sqlx::query(query).bind(ids).execute(&pool).await?;

        Ok(result.rows_affected())
    }
}

//////// END

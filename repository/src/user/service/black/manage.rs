// repository/src/user/service/black/manage.rs
// 仓储 - USER - service - black - 管理
// 2026/8/3 19:38 Created.

////////

use crate::pg_pool;
use sqlx::{self, PgPool};

//////

/// # [MANAGE REPOSITORY] - 用户 黑名单 管理 仓储
pub struct UserBlackManageRepo;

// 构造函数
impl UserBlackManageRepo {
    //

    ////////

    /// # 1. [REPOSITORY] - 🚮 单个软删除
    pub async fn pg_single_soft_del_by_id(
        id: i64,       // 黑名单ID
    ) -> Result<u64, sqlx::Error> {
        let pool = pg_pool();

        // 修复表名格式和查询条件：根据主键 id 进行单个软删除
        let query = r#"
        UPDATE "cola_user"."black"
        SET is_deleted = 1, deleted_at = NOW(), status = 0
        WHERE id = $1 AND is_deleted = 0
    "#;

        let result = sqlx::query(query).bind(id).execute(&pool).await?;

        Ok(result.rows_affected())
    }

    ////////

    /// # 2. [REPOSITORY] - 🚮 批量软删除（遍历/批量删除）
    /// * `desc`: 根据一组黑名单 ID 批量软删除
    pub async fn pg_batch_soft_del_by_ids(
        ids: &[i64],   // 黑名单ID列表
    ) -> Result<u64, sqlx::Error> {
        let pool = pg_pool();

        if ids.is_empty() {
            return Ok(0);
        }

        // 修复表名格式，并使用 ANY($1) 进行批量匹配删除
        let query = r#"
        UPDATE "cola_user"."black"
        SET is_deleted = 1, deleted_at = NOW(), status = 0
        WHERE id = ANY($1) AND is_deleted = 0
    "#;

        let result = sqlx::query(query).bind(ids).execute(&pool).await?;

        Ok(result.rows_affected())
    }
}

//////// END
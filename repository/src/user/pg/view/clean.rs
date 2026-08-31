// repository/src/user/pg/view/clean.rs
// 仓储 - USER - pg - 浏览 - 清除
// 2026/8/7 00:01 Created.

////////

use crate::pg_pool;
use sqlx::{self, PgPool};

//////

/// # [CLEAN REPOSITORY] - 清除
/// * `desc`: `用户浏览记录清除仓储`
pub struct UserViewCleanRepo;

// 构造函数
impl UserViewCleanRepo {
    //

    ////////

    /// # 1. [REPOSITORY] - ❌️ 定期清理过期超过180天的软删除记录（物理删除或状态归档）
    /// * `desc`: 命中规则 is_deleted = true（或状态为已删除），且 deleted_at 超过 180 天
    /// * `return`: u64 删除了多少条记录.
    pub async fn pg_clean_expired_deleted_records() -> Result<u64, sqlx::Error> {
        let pool = pg_pool();

        // 规范做法：限制单次删除条数（例如每次最多删 1000 条），防止大事务锁表
        // 实际定时任务中可以在上层逻辑写一个 loop 循环调用，直到返回影响行数为 0
        let query = r#"
            DELETE FROM "user.view"
            WHERE id IN (
                SELECT id FROM "user.view"
                WHERE is_deleted = 1
                  AND deleted_at IS NOT NULL
                  AND deleted_at < NOW() - INTERVAL '180 days'
                LIMIT 1000
            )
        "#;

        let result = sqlx::query(query).execute(&pool).await?;

        Ok(result.rows_affected())
    }
}

//////// END

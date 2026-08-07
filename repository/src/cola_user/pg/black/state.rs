// repository/src/cola_user/pg/black/state.rs
// 仓储中心 - USER - pg - black - 状态查询
// 2026/8/3 01:23 Created.

////////

use crate::pg_pool;
use sqlx::{self, PgPool};

////////

/// # [STATE REPOSITORY] - 状态
/// * `desc`: `仓储层用户黑名单状态仓储`
pub struct UserBlackStateRepo;

impl UserBlackStateRepo {
    //

    ////////

    /// # 1. [REPOSITORY] - 状态查询
    /// * `desc`: `检查用户是否处于黑名单中`
    /// * `logic`: `is_deleted = false 且 status = 1 时返回 true，其余情况返回 false`
    pub async fn find_black_state_by_uid_and_user_id(
        uid: i64,
        user_id: i64,
    ) -> Result<bool, sqlx::Error> {
        let pool = pg_pool();

        // 查询是否存在符合条件的记录：匹配 uid、user_id、未软删除(is_deleted = false) 且状态正常(status = 1)
        // 使用 EXISTS 可以让数据库在找到第一条符合条件的数据时直接返回，性能最优
        let query = r#"
            SELECT EXISTS (
                SELECT 1
                FROM "cola_user.black"
                WHERE uid = $1
                  AND user_id = $2
                  AND is_deleted = false
                  AND status = 1
            )
        "#;

        let exists: bool = sqlx::query_scalar(query)
            .bind(uid)
            .bind(user_id)
            .fetch_one(&pool)
            .await?;

        Ok(exists)
    }
}

//////// END

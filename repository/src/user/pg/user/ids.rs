// repository/src/user/pg/user/ids.rs
//
// 2026/8/6 22:58 Created.

////////

// repository/src/user/pg/user/list.rs
// 仓储层 - 可乐用户 - pg - 用户 - 列表仓储
// 2026/8/6 22:37 Created.

////////

use crate::pg_pool;
use cola_data::cola_user::entity::user::{USER_COLUMNS, UserEntity};
use sqlx::{self};

////////

/// # [IDS REPOSITORY] - 前台列表
/// * `desc`: `用户前台列表仓储`
pub struct UserIdsRepo;

impl UserIdsRepo {
    //

    ////////

    /// # 2. [REPOSITORY] - 查找一个用户 (精准匹配)
    pub async fn find_user_by_id(user_id: i64) -> Result<Option<UserEntity>, sqlx::Error> {
        let pool = pg_pool(); // 👈 抽出来
        // 💡 修正：通过 user_id (或 id) 精准定位单条记录，不再是查列表
        let query = format!(
            "SELECT {} FROM \"user.users\" WHERE id = $1 LIMIT 1",
            USER_COLUMNS
        );

        sqlx::query_as::<_, UserEntity>(&query)
            .bind(user_id)
            .fetch_optional(&pool)
            .await
    }

    ////////

    /// # 3. [REPOSITORY] - 批量查找用户
    pub async fn find_many_users_by_ids(user_ids: &[i64]) -> Result<Vec<UserEntity>, sqlx::Error> {
        let pool = pg_pool(); // 👈 抽出来
        // 💡 修正：使用 PostgreSQL 的 ANY 语法批量匹配数组中的所有 id
        let query = format!(
            "SELECT {} FROM \"user.users\" WHERE id = ANY($1) AND status = 1 ORDER BY create_time DESC",
            USER_COLUMNS
        );

        sqlx::query_as::<_, UserEntity>(&query)
            .bind(user_ids)
            .fetch_all(&pool)
            .await
    }

    ////////
}

//////// END

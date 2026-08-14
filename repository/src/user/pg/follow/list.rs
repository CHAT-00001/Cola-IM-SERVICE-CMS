// repository/src/user/pg/follow/list.rs
// 仓储 - USER - pg - follow - 列表查询
// 2026/8/6 Created.

////////

use crate::pg_pool;
use sqlx;

////////

/// # [LIST REPOSITORY] - 关注列表仓储
pub struct UserFollowListRepo;

impl UserFollowListRepo {

    ////////

    /// # 1. [REPOSITORY] - 查询我关注的用户IDs
    pub async fn pg_find_my_follow_ids(
        uid: i64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<i64>, sqlx::Error> {
        let pool = pg_pool();
        let query = "SELECT user_id FROM user_follow WHERE uid = $1 AND status = 1 ORDER BY id DESC LIMIT $2 OFFSET $3";
        sqlx::query_scalar::<_, i64>(query)
            .bind(uid)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 2. [REPOSITORY] - 查询TA关注的用户IDs
    pub async fn pg_find_he_follow_ids(
        target_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<i64>, sqlx::Error> {
        let pool = pg_pool();
        let query = "SELECT user_id FROM user_follow WHERE uid = $1 AND status = 1 ORDER BY id DESC LIMIT $2 OFFSET $3";
        sqlx::query_scalar::<_, i64>(query)
            .bind(target_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 3. [REPOSITORY] - 批量查询用户实体(根据IDs)
    pub async fn pg_find_users_by_ids(
        ids: &[i64],
    ) -> Result<Vec<cola_data::cola_user::entity::user::UserEntity>, sqlx::Error> {
        use cola_data::cola_user::entity::user::USER_COLUMNS;
        if ids.is_empty() {
            return Ok(vec![]);
        }
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM \"user\" WHERE id = ANY($1) AND status = 1",
            USER_COLUMNS
        );
        sqlx::query_as::<_, cola_data::cola_user::entity::user::UserEntity>(&query)
            .bind(ids)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 4. [REPOSITORY] - 获取关注总数
    pub async fn pg_count_follows(uid: i64) -> Result<i64, sqlx::Error> {
        let pool = pg_pool();
        let query = "SELECT COUNT(*) FROM user_follow WHERE uid = $1 AND status = 1";
        sqlx::query_scalar::<_, i64>(query)
            .bind(uid)
            .fetch_one(&pool)
            .await
    }
}

//////// END
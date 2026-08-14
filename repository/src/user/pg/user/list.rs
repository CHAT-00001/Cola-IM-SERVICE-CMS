// repository/src/user/pg/user/list.rs
// 仓储 - USER - pg - 用户 - 列表仓储
// 2026/8/6 22:37 Created.

////////

use crate::pg_pool;
use cola_data::cola_user::entity::user::{USER_COLUMNS, UserEntity};
use sqlx::{self};

////////

/// # [LIST REPOSITORY] - 前台列表
/// * `desc`: `用户前台列表仓储`
pub struct UserListRepo;

impl UserListRepo {
    //

    ////////

    /// # 1. [REPOSITORY] - 前台最新
    /// * `desc`: `查找最新的列表`
    pub async fn find_new_list(
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> Result<Vec<UserEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM \"cola_user\".\"user\" WHERE status = 1 ORDER BY create_time DESC LIMIT $1 OFFSET $2",
            USER_COLUMNS
        );

        sqlx::query_as::<_, UserEntity>(&query)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 2. [REPOSITORY] - 查找一个用户 (精准匹配)
    pub async fn find_user_by_id(user_id: i64) -> Result<Option<UserEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM \"cola_user\".\"user\" WHERE id = $1 LIMIT 1",
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
        if user_ids.is_empty() {
            return Ok(vec![]);
        }

        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM \"cola_user\".\"user\" WHERE id = ANY($1) AND status = 1 ORDER BY create_time DESC",
            USER_COLUMNS
        );

        sqlx::query_as::<_, UserEntity>(&query)
            .bind(user_ids)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 4. [REPOSITORY] - 附近的用户
    pub async fn find_nearby_list(
        lat: f64,    // 纬度
        lng: f64,    // 经度
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> Result<Vec<UserEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {}, SQRT(POW(lat - $1, 2) + POW(lng - $2, 2)) AS distance
             FROM \"cola_user\".\"user\"
             WHERE status = 1 AND lat IS NOT NULL AND lng IS NOT NULL
             ORDER BY distance ASC
             LIMIT $3 OFFSET $4",
            USER_COLUMNS
        );

        sqlx::query_as::<_, UserEntity>(&query)
            .bind(lat)
            .bind(lng)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 5. [REPOSITORY] - 分类（频道）下的用户
    pub async fn find_category_list(
        limit: i64,
        offset: i64,
    ) -> Result<Vec<UserEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM \"cola_user\".\"user\" WHERE status = 1 ORDER BY likes DESC, create_time DESC LIMIT $1 OFFSET $2",
            USER_COLUMNS
        );

        sqlx::query_as::<_, UserEntity>(&query)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 6. [REPOSITORY] - 精选用户
    pub async fn find_featured_list(
        limit: i64,
        offset: i64,
    ) -> Result<Vec<UserEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM \"cola_user\".\"user\" WHERE status = 1 AND is_recommend = 1 ORDER BY likes DESC, create_time DESC LIMIT $1 OFFSET $2",
            USER_COLUMNS
        );

        sqlx::query_as::<_, UserEntity>(&query)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 7. [REPOSITORY] - 搜索关键词 (模糊匹配昵称 + 距离排序)
    pub async fn search_keyword(
        keyword: &str,
        lat: f64,
        lng: f64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<UserEntity>, sqlx::Error> {
        let pool = pg_pool();
        let keyword_like = format!("%{}%", keyword);

        let query = format!(
            "SELECT {}, SQRT(POW(lat - $2, 2) + POW(lng - $3, 2)) AS distance
             FROM \"cola_user\".\"user\"
             WHERE status = 1 AND user_nickname ILIKE $1
             ORDER BY distance ASC
             LIMIT $4 OFFSET $5",
            USER_COLUMNS
        );

        sqlx::query_as::<_, UserEntity>(&query)
            .bind(keyword_like)
            .bind(lat)
            .bind(lng)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }
}

//////// END
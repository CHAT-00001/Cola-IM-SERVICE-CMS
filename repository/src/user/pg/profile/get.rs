// repository/src/user/pg/profile/get.rs
// 仓储 - USER - pg - profile - get 获取
// 2026/8/3 12:57 Created.

////////

use crate::pg_pool;
use app_config::GLOBAL_DB;
use cola_data::user::entity::profile::UserProfileEntity;
use sqlx::{self, PgPool};

//////

/// # [GET REPOSITORY] - 用户 资料 获取 仓储
/// * `desc`: `用户 资料 获取 仓储`
pub struct UserProfileGetRepo;

// 构造函数
impl UserProfileGetRepo {
    //

    ////////

    /// # 1. [REPOSITORY] - 最新
    /// * `desc`: 按 created_at 倒序，最新的在前面
    pub async fn pg_find_new_list(
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> Result<Vec<UserProfileEntity>, sqlx::Error> {
        let pool = pg_pool();
        let fetch_limit = limit + 1; // 多查一条用于判断是否有下一页

        let query = r#"
            SELECT p.*
            FROM "cola_user"."profile" p
            WHERE p.status = 1 AND p.is_deleted = 0
            ORDER BY p.created_at DESC, p.id DESC
            LIMIT $1 OFFSET $2
        "#;

        sqlx::query_as::<_, UserProfileEntity>(query)
            .bind(fetch_limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 2. [REPOSITORY] - 热门
    /// * `desc`: 按 likes 值和创建时间双倒序
    pub async fn pg_find_hot_list(
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> Result<Vec<UserProfileEntity>, sqlx::Error> {
        let pool = pg_pool();
        let fetch_limit = limit + 1; // 多查一条用于判断是否有下一页

        let query = r#"
            SELECT p.*
            FROM "cola_user"."profile" p
            WHERE p.status = 1 AND p.is_deleted = 0
            ORDER BY p.likes DESC, p.created_at DESC, p.id DESC
            LIMIT $1 OFFSET $2
        "#;

        sqlx::query_as::<_, UserProfileEntity>(query)
            .bind(fetch_limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 3. [REPOSITORY] - 推荐
    /// * `desc`: 用随机函数模拟推荐
    pub async fn pg_find_recommend_list(
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> Result<Vec<UserProfileEntity>, sqlx::Error> {
        let pool = pg_pool();
        let fetch_limit = limit + 1; // 多查一条用于判断是否有下一页

        // 使用 RANDOM() 进行随机排序模拟推荐
        let query = r#"
            SELECT p.*
            FROM "cola_user"."profile" p
            WHERE p.status = 1 AND p.is_deleted = 0
            ORDER BY RANDOM()
            LIMIT $1 OFFSET $2
        "#;

        sqlx::query_as::<_, UserProfileEntity>(query)
            .bind(fetch_limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 4. [REPOSITORY] - 同城
    /// * `desc`: 按 likes 和 created_at 双倒序
    pub async fn pg_find_city_list(
        city_id: i64, // 城市ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> Result<Vec<UserProfileEntity>, sqlx::Error> {
        let pool = pg_pool();
        let fetch_limit = limit + 1; // 多查一条用于判断是否有下一页

        let query = r#"
            SELECT p.*
            FROM "cola_user"."profile" p
            WHERE p.city_id = $1 AND p.status = 1 AND p.is_deleted = 0
            ORDER BY p.likes DESC, p.created_at DESC, p.id DESC
            LIMIT $2 OFFSET $3
        "#;

        sqlx::query_as::<_, UserProfileEntity>(query)
            .bind(city_id)
            .bind(fetch_limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 5. [REPOSITORY] - 搜索
    /// * `desc`: 按 likes 和 created_at 双倒序
    pub async fn pg_find_keyword_list(
        keyword: &String, // 关键词
        limit: i64,       // 数量
        offset: i64,      // 页码
    ) -> Result<Vec<UserProfileEntity>, sqlx::Error> {
        let pool = pg_pool();
        let fetch_limit = limit + 1; // 多查一条用于判断是否有下一页
        let search_pattern = format!("%{}%", keyword);

        let query = r#"
            SELECT p.*
            FROM "cola_user"."profile" p
            WHERE p.status = 1
              AND p.is_deleted = 0
              AND (p.nickname ILIKE $1 OR p.bio ILIKE $1)
            ORDER BY p.likes DESC, p.created_at DESC, p.id DESC
            LIMIT $2 OFFSET $3
        "#;

        sqlx::query_as::<_, UserProfileEntity>(query)
            .bind(search_pattern)
            .bind(fetch_limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }
}

//////// END
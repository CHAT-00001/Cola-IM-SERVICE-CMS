// repository/src/pg/user/pg/view/view_repo.rs
// 仓储 - 可乐用户 - pg - 浏览 - 原始
// 2026/6/29 03:54

////////

use sqlx::{self, PgPool};
use app_config::GLOBAL_DB;
use cola_data::cola_user::entity::user::{UserEntity, USER_COLUMNS};
use crate::pg_pool;

//////

// 局部辅助结构体：用来承接带有"动态计算距离"的数据库返回行
#[derive(Debug, sqlx::FromRow)]
pub struct VideoHomeRow {
    #[sqlx(flatten)] // 自动把标准字段映射进 UserEntity
    pub entity: UserEntity,
    #[sqlx(default)]
    pub distance: Option<f64>, // 承接动态计算的距离
}

/// # [SERVICE] - 用户 浏览 REPO
pub struct UserViewRepo;

// 构造函数
impl UserViewRepo {

    ////////

    /// # 1. [REPOSITORY] - 根据ID 查找一个用户 (精准匹配)
    pub async fn find_user_by_id(
        user_id: i64,
    ) -> Result<Option<UserEntity>, sqlx::Error> {
        let pool = pg_pool(); // 👈 抽出来
        let query = format!(
            "SELECT {} FROM \"user\" WHERE id = $1 LIMIT 1",
            USER_COLUMNS
        );

        sqlx::query_as::<_, UserEntity>(&query)
            .bind(user_id)
            .fetch_optional(&pool)
            .await
    }

    ////////

    /// # 2. [REPOSITORY] - 根据电话号码查找一个用户
    pub async fn find_user_by_phone(
        phone: &str,
    ) -> Result<Option<UserEntity>, sqlx::Error> {
        let pool = pg_pool(); // 👈 抽出来
        let query = format!(
            "SELECT {} FROM \"user\" WHERE phone = $1 LIMIT 1",
            USER_COLUMNS
        );

        sqlx::query_as::<_, UserEntity>(&query)
            .bind(phone)
            .fetch_optional(&pool)
            .await
    }

    ////////

    /// # 3. [REPOSITORY] - 根据邮箱查找一个用户
    pub async fn find_user_by_email(
        phone: &str,
    ) -> Result<Option<UserEntity>, sqlx::Error> {
        let pool = pg_pool(); // 👈 抽出来
        let query = format!(
            "SELECT {} FROM \"user\" WHERE email = $1 LIMIT 1",
            USER_COLUMNS
        );

        sqlx::query_as::<_, UserEntity>(&query)
            .bind(phone)
            .fetch_optional(&pool)
            .await
    }

    ////////

    /// # 4. [REPOSITORY] - 批量查找用户
    pub async fn find_batch_users_by_ids(
        user_ids: &[i64],
    ) -> Result<Vec<UserEntity>, sqlx::Error> {
        let pool = pg_pool(); // 👈 抽出来
        let query = format!(
            "SELECT {} FROM \"user\" WHERE id = ANY($1) AND status = 1 ORDER BY id DESC",
            USER_COLUMNS
        );

        sqlx::query_as::<_, UserEntity>(&query)
            .bind(user_ids)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 6. [REPOSITORY] - 附近的用户
    pub async fn find_nearby_user_list(
        lat: f64,
        lng: f64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoHomeRow>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {}, SQRT(POW(lat - $1, 2) + POW(lng - $2, 2)) AS distance
             FROM \"user\"
             WHERE status = 1
             ORDER BY distance ASC
             LIMIT $3 OFFSET $4",
            USER_COLUMNS
        );

        sqlx::query_as::<_, VideoHomeRow>(&query)
            .bind(lat)
            .bind(lng)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    /// # 7. [REPOSITORY] - 分类（频道）下的用户
    pub async fn find_category(
        limit: i64,
        offset: i64,
    ) -> Result<Vec<UserEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM \"user\" WHERE status = 1 ORDER BY likes DESC NULLS LAST, id DESC LIMIT $1 OFFSET $2",
            USER_COLUMNS
        );

        sqlx::query_as::<_, UserEntity>(&query)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    /// # 8. [REPOSITORY] - 精选用户
    pub async fn find_featured(
        limit: i64,
        offset: i64,
    ) -> Result<Vec<UserEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM \"user\" WHERE status = 1 ORDER BY likes DESC NULLS LAST, id DESC LIMIT $1 OFFSET $2",
            USER_COLUMNS
        );

        sqlx::query_as::<_, UserEntity>(&query)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    /// # 9. [REPOSITORY] - 搜索关键词 (模糊匹配昵称 + 距离排序)
    pub async fn search_keyword(
        keyword: &str,
        lat: f64,
        lng: f64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoHomeRow>, sqlx::Error> {
        let pool = pg_pool();
        let keyword_like = format!("%{}%", keyword);

        let query = format!(
            "SELECT {}, SQRT(POW(lat - $2, 2) + POW(lng - $3, 2)) AS distance
             FROM \"user\"
             WHERE status = 1 AND user_nickname LIKE $1
             ORDER BY distance ASC
             LIMIT $4 OFFSET $5",
            USER_COLUMNS
        );

        sqlx::query_as::<_, VideoHomeRow>(&query)
            .bind(keyword_like)
            .bind(lat)
            .bind(lng)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }
}

//////// END OF REPOSITORY IMPLEMENTATION ////////

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::postgres::PgPoolOptions;
    use std::env;
    use std::sync::Arc;

    /// 辅助单测：模拟程序启动时初始化全局数据库
    async fn setup_global_db_for_test() {
        let db_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:123456@127.0.0.1:5432/live_2026?options=-c%20lc_messages=en_US.UTF-8".to_string());

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await
            .expect("❌ 无法连接到测试数据库");

        // 假定你的 DbService 有一个从 pool 实例化的方法或者包装
        // 这里需要跟你的真实结构契合，通过 init_global_db 注入全局
        // 示例直接注入伪造的依赖（如果 OnceLock 已经满了解包 ok 即可）
        // let service = DbService::from_pool(pool);
        // api::init_global_db(service);
    }

    // 注意：在真实单测中，确保在调用 UserRepo 前已经执行了静态池初始化。
}

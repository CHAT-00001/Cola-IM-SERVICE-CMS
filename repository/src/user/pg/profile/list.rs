// repository/src/user/pg/user/list.rs
// 🛢️ 仓储 - 可乐用户中心 - pg - 用户 - 列表
// 2026/8/6 23:16 Created.

////////

use crate::pg_pool;
use cola_data::cola_user::entity::user::{USER_COLUMNS, UserEntity};
use sqlx::{self};

////////

/// # [LIST REPOSITORY] - 列表
/// * `desc`: `用户中心PG资料列表仓储`
pub struct PgProfileListRepo;

impl PgProfileListRepo {
    // 💡

    ////////

    /// # 1. [REPOSITORY] - 最新
    /// * `desc`: `查找最新的列表`
    pub async fn find_new_user_list(
        limit: i64,
        offset: i64,
    ) -> Result<Vec<UserEntity>, sqlx::Error> {
        let pool = pg_pool(); // 👈 抽出来
        let query = format!(
            "SELECT {} FROM \"user.profile\" WHERE status = 1 ORDER BY create_time DESC LIMIT $1 OFFSET $2",
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
        let pool = pg_pool(); // 👈 抽出来
        // 💡 修正：通过 user_id (或 id) 精准定位单条记录，不再是查列表
        let query = format!(
            "SELECT {} FROM \"user.profile\" WHERE id = $1 LIMIT 1",
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
            "SELECT {} FROM \"user.profile\" WHERE id = ANY($1) AND status = 1 ORDER BY create_time DESC",
            USER_COLUMNS
        );

        sqlx::query_as::<_, UserEntity>(&query)
            .bind(user_ids)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 4. [REPOSITORY] - 附近的用户
    pub async fn find_nearby_user_list(
        lat: f64,    // 纬度
        lng: f64,    // 经度
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> Result<Vec<UserEntity>, sqlx::Error> {
        let pool = pg_pool(); // 👈 抽出来
        // 💡 修正：表名换回 \"user.profile\"
        let query = format!(
            "SELECT {}, SQRT(POW(lat - $1, 2) + POW(lng - $2, 2)) AS distance
             FROM \"user.profile\"
             WHERE status = 1
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
    pub async fn find_category(limit: i64, offset: i64) -> Result<Vec<UserEntity>, sqlx::Error> {
        let pool = pg_pool(); // 👈 抽出来
        // 💡 修正：从 \"user.profile\" 表按热度排序（可按你实际的业务权重字段调整）
        let query = format!(
            "SELECT {} FROM \"user.profile\" WHERE status = 1 ORDER BY likes DESC, views DESC LIMIT $1 OFFSET $2",
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
    pub async fn find_featured(limit: i64, offset: i64) -> Result<Vec<UserEntity>, sqlx::Error> {
        let pool = pg_pool(); // 👈 抽出来
        let query = format!(
            "SELECT {} FROM \"user.profile\" WHERE status = 1 AND is_recommend = 1 ORDER BY likes DESC, create_time DESC LIMIT $1 OFFSET $2",
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
        let pool = pg_pool(); // 👈 抽出来
        let keyword_like = format!("%{}%", keyword);

        // 💡 修正：用 \"user.profile\" 表，并将 title 替换为符合 UserEntity 属性的 nickname 模糊查询
        let query = format!(
            "SELECT {}, SQRT(POW(lat - $2, 2) + POW(lng - $3, 2)) AS distance
             FROM \"user.profile\"
             WHERE status = 1 AND user_nickname LIKE $1
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

//////// END OF REPOSITORY IMPLEMENTATION ////////

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::postgres::PgPoolOptions;
    use std::env;

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

//////// END

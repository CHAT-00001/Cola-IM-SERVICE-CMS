// pg/state.rs  -- 仓储中心 - pg - 用户状态
// 2026/6/9 09:20

////////

use sqlx::{self, PgPool};
use app_config::GLOBAL_DB;
use cola_data::user::entity::user::UserEntity;
use crate::pg_pool;

////////

// 数据表原始字段（对应 Entity 的基础字段，1:1 完全一致）
const COLUMNS: &str = r#"
    id, uuid, show_id, user_id, nickname, avatar, sing, birthday,
    email, phone, href, href_w, original_url, tags, lat, lng, duration,
    width, height, fps, bit, views, likes, steps, collects, comments,
    done_play_qty, visibility, allow_comment, allow_danmaku, shares,
    is_public, status, music_id, goods_id, create_time, created_at, updated_at
"#;

// 局部辅助结构体：用来承接带有“动态计算距离”的数据库返回行
#[derive(Debug, sqlx::FromRow)]
pub struct VideoHomeRow {
    #[sqlx(flatten)] // 自动把标准字段映射进 UserEntity
    pub entity: UserEntity,
    #[sqlx(default)]
    pub distance: Option<f64>, // 承接动态计算的距离
}

/// # [SERVICE] - 用户状态服务
pub struct UserStateRepo;

// 构造函数
impl UserStateRepo {

    /// # 1. [REPOSITORY] - 查找最新的列表
    pub async fn find_new_user_list(
        limit: i64,
        offset: i64,
    ) -> Result<Vec<UserEntity>, sqlx::Error> {
        let pool = pg_pool(); // 👈 抽出来
        let query = format!(
            "SELECT {} FROM \"user\" WHERE status = 1 ORDER BY addtime DESC LIMIT $1 OFFSET $2",
            COLUMNS
        );

        sqlx::query_as::<_, UserEntity>(&query)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    /// # 2. [REPOSITORY] - 查找一个用户 (精准匹配)
    pub async fn find_user_by_id(
        user_id: i64,
    ) -> Result<Option<UserEntity>, sqlx::Error> {
        let pool = pg_pool(); // 👈 抽出来
        // 💡 修正：通过 user_id (或 id) 精准定位单条记录，不再是查列表
        let query = format!(
            "SELECT {} FROM \"user\" WHERE user_id = $1 LIMIT 1",
            COLUMNS
        );

        sqlx::query_as::<_, UserEntity>(&query)
            .bind(user_id)
            .fetch_optional(&pool)
            .await
    }

    /// # 2. [REPOSITORY] - 根据电话号码查找一个用户
    pub async fn find_user_by_phone(
        phone: &str,
    ) -> Result<Option<UserEntity>, sqlx::Error> {
        let pool = pg_pool(); // 👈 抽出来
        // 💡 修正：通过 user_id (或 id) 精准定位单条记录，不再是查列表
        let query = format!(
            "SELECT {} FROM \"user\" WHERE phone = $1 LIMIT 1",
            COLUMNS
        );

        sqlx::query_as::<_, UserEntity>(&query)
            .bind(phone)
            .fetch_optional(&pool)
            .await
    }

    /// # 1. [REPOSITORY] - 保存用户并返回实体
    pub async fn save_user(
        entity: UserEntity,
    ) -> Result<UserEntity, anyhow::Error> {
        let pool = pg_pool();

        // 💡 PostgreSQL 的 RETURNING * 可以在插入后直接返回完整记录
        // 假设 COLUMNS 是定义好的列字符串，例如 "id, nickname, avatar, ..."
        let query = format!(
            "INSERT INTO \"user\" (send_id, user_nickname, avatar, bg_img, signature, phone, birthday, status, perm_id, add_time)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
         RETURNING *",
        );

        let saved_user = sqlx::query_as::<_, UserEntity>(&query)
            .bind(entity.send_id)
            .bind(entity.user_nickname)
            .bind(entity.avatar)
            .bind(entity.bg_img)
            .bind(entity.signature)
            .bind(entity.phone)
            .bind(entity.birthday)
            .bind(entity.status)
            .bind(entity.perm_id)
            .bind(entity.create_time)
            .fetch_one(&pool)
            .await?;

        Ok(saved_user)
    }

    /// # 3. [REPOSITORY] - 批量查找用户
    pub async fn find_many_users_by_ids(
        user_ids: &[i64],
    ) -> Result<Vec<UserEntity>, sqlx::Error> {
        let pool = pg_pool(); // 👈 抽出来
        // 💡 修正：使用 PostgreSQL 的 ANY 语法批量匹配数组中的所有 id
        let query = format!(
            "SELECT {} FROM \"user\" WHERE user_id = ANY($1) AND status = 1 ORDER BY addtime DESC",
            COLUMNS
        );

        sqlx::query_as::<_, UserEntity>(&query)
            .bind(user_ids)
            .fetch_all(&pool)
            .await
    }

    /// # 4. [REPOSITORY] - 附近的用户
    pub async fn find_nearby_user_list(
        lat: f64,
        lng: f64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoHomeRow>, sqlx::Error> {
        let pool = pg_pool(); // 👈 抽出来
        // 💡 修正：表名换回 \"user\"
        let query = format!(
            "SELECT {}, SQRT(POW(lat - $1, 2) + POW(lng - $2, 2)) AS distance
             FROM \"user\"
             WHERE status = 1
             ORDER BY distance ASC
             LIMIT $3 OFFSET $4",
            COLUMNS
        );

        sqlx::query_as::<_, VideoHomeRow>(&query)
            .bind(lat)
            .bind(lng)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    /// # 5. [REPOSITORY] - 分类（频道）下的用户
    pub async fn find_category(
        limit: i64,
        offset: i64,
    ) -> Result<Vec<UserEntity>, sqlx::Error> {
        let pool = pg_pool(); // 👈 抽出来
        // 💡 修正：从 \"user\" 表按热度排序（可按你实际的业务权重字段调整）
        let query = format!(
            "SELECT {} FROM \"user\" WHERE status = 1 ORDER BY likes DESC, views DESC LIMIT $1 OFFSET $2",
            COLUMNS
        );

        sqlx::query_as::<_, UserEntity>(&query)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    /// # 6. [REPOSITORY] - 精选用户
    pub async fn find_featured(
        limit: i64,
        offset: i64,
    ) -> Result<Vec<UserEntity>, sqlx::Error> {
        let pool = pg_pool(); // 👈 抽出来
        let query = format!(
            "SELECT {} FROM \"user\" WHERE status = 1 AND is_public = 1 ORDER BY likes DESC, addtime DESC LIMIT $1 OFFSET $2",
            COLUMNS
        );

        sqlx::query_as::<_, UserEntity>(&query)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    /// # 7. [REPOSITORY] - 搜索关键词 (模糊匹配昵称 + 距离排序)
    pub async fn search_keyword(
        keyword: &str,
        lat: f64,
        lng: f64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoHomeRow>, sqlx::Error> {
        let pool = pg_pool(); // 👈 抽出来
        let keyword_like = format!("%{}%", keyword);

        // 💡 修正：用 \"user\" 表，并将 title 替换为符合 UserEntity 属性的 nickname 模糊查询
        let query = format!(
            "SELECT {}, SQRT(POW(lat - $2, 2) + POW(lng - $3, 2)) AS distance
             FROM \"user\"
             WHERE status = 1 AND nickname LIKE $1
             ORDER BY distance ASC
             LIMIT $4 OFFSET $5",
            COLUMNS
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
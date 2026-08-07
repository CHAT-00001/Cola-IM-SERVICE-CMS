// repository/src/user/pg/home  -- 仓储中心 用户 HOME repository
// 2026/6/25 22:21

////////

use sqlx::{self, PgPool};
use app_config::GLOBAL_DB;
use cola_data::user::entity::user::UserEntity;
use crate::pg_pool;

//////

// 数据表原始字段（对应 Entity 的基础字段，1:1 完全一致）
const COLUMNS: &str = r#"
    id, user_type, user_nickname, signature, avatar, bg_img,
    email, phone, sns_url, birthday, sex, perm_id, likes, fans, follows,
    level, author_level, lat, lng, login_ip, register_ip, status,
    create_time, created_at, updated_at
"#;

// 局部辅助结构体：用来承接带有"动态计算距离"的数据库返回行
#[derive(Debug, sqlx::FromRow)]
pub struct VideoHomeRow {
    #[sqlx(flatten)] // 自动把标准字段映射进 UserEntity
    pub entity: UserEntity,
    #[sqlx(default)]
    pub distance: Option<f64>, // 承接动态计算的距离
}

/// # [SERVICE] - 用户首页REPO
pub struct UserHomeRepo;

// 构造函数
impl UserHomeRepo {

    ////////

    /// # 1. [REPOSITORY] - 最新
    pub async fn find_new_user_list(
        limit: i64,
        offset: i64,
    ) -> Result<Vec<UserEntity>, sqlx::Error> {
        let pool = pg_pool(); // 👈 抽出来
        let query = format!(
            "SELECT {} FROM \"user\" WHERE status = 1 ORDER BY id DESC LIMIT $1 OFFSET $2",
            COLUMNS
        );

        sqlx::query_as::<_, UserEntity>(&query)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 2. [REPOSITORY] - 热门
    pub async fn find_hot_user_list(
        limit: i64,
        offset: i64,
    ) -> Result<Vec<UserEntity>, sqlx::Error> {
        let pool = pg_pool(); // 👈 抽出来
        let query = format!(
            "SELECT {} FROM \"user\" WHERE status = 1 ORDER BY id DESC LIMIT $1 OFFSET $2",
            COLUMNS
        );

        sqlx::query_as::<_, UserEntity>(&query)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 3. [REPOSITORY] - 推荐
    pub async fn find_recommend_users_list(
        limit: i64,
        offset: i64,
    ) -> Result<Vec<UserEntity>, sqlx::Error> {
        let pool = pg_pool(); // 👈 抽出来
        let query = format!(
            "SELECT {} FROM \"user\" WHERE is_recommend = $1 AND status = 1 ORDER BY id DESC LIMIT $1 OFFSET $2",
            COLUMNS
        );

        sqlx::query_as::<_, UserEntity>(&query)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 4. [REPOSITORY] - 附近
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

    ////////

    /// # 5. [REPOSITORY] - 分类（频道）下的用户
    pub async fn find_category(
        limit: i64,
        offset: i64,
    ) -> Result<Vec<UserEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM \"user\" WHERE status = 1 ORDER BY likes DESC NULLS LAST, id DESC LIMIT $1 OFFSET $2",
            COLUMNS
        );

        sqlx::query_as::<_, UserEntity>(&query)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 8. [REPOSITORY] - 精选用户
    pub async fn find_featured(
        limit: i64,
        offset: i64,
    ) -> Result<Vec<UserEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM \"user\" WHERE status = 1 ORDER BY likes DESC NULLS LAST, id DESC LIMIT $1 OFFSET $2",
            COLUMNS
        );

        sqlx::query_as::<_, UserEntity>(&query)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 9. [REPOSITORY] - 搜索关键词 (模糊匹配昵称 + 距离排序)
    pub async fn find_users_by_keyword(
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

//////// END
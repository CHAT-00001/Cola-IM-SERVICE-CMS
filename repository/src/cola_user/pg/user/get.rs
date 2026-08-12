// repository/src/cola_user/pg/cola_user/get.rs
// 仓储层 - 可乐用户 - pg - 用户 - 获取仓储
// 2026/8/3 19:01 Created.

////////

use crate::pg_pool;
use cola_data::cola_user::entity::user::{UserEntity, USER_COLUMNS};
use sqlx::{self};

//////

// 数据表原始字段（对应 Entity 的基础字段，1:1 完全一致）
// ⚠️ login_ip / register_ip 用 COALESCE 兜底：兼容旧数据 NULL 不报错
const COLUMNS: &str = r#"
    id, send_id, user_type, user_nickname, signature, avatar, bg_img,
    email, phone, sns_url, birthday, sex, perm_id, likes, fans, follows,
    level, author_level, lat, lng,
    COALESCE(login_ip, '未知IP') AS login_ip,
    COALESCE(register_ip, '未知IP') AS register_ip,
    status,
    create_time, created_at, updated_at
"#;

// INSERT/RETURNING 用别名确保 COALESCE 不会在写入时报错
const INSERT_RETURNING: &str = r#"
    id, send_id, user_type, user_nickname, avatar, bg_img,
    signature, email, phone, birthday, status, perm_id, create_time,
    login_ip, register_ip
"#;

//////

/// # [GET REPOSITORY] - 获取
/// * `desc`: `用户获取仓储`
pub struct UserGetRepo;

// 构造实现
impl UserGetRepo {
    //

    ////////

    /// # 1. [REPOSITORY] - 单个
    /// * `desc`: `根据用户ID单个查找用户 (精准匹配)`
    pub async fn single_find_user_by_id(
        user_id: i64, // 目标ID
    ) -> Result<Option<UserEntity>, sqlx::Error> {
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

    /// # 2. [REPOSITORY] - 批量
    ///  * `desc`: `根据用户IDs批量查找用户 (精准匹配)`
    pub async fn batch_find_users_by_ids(
        user_ids: &[i64], // 目标IDs
    ) -> Result<Vec<UserEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM \"cola_user\".\"user\" WHERE id = ANY($1) AND user_status = 1 ORDER BY id DESC",
            USER_COLUMNS
        );

        sqlx::query_as::<_, UserEntity>(&query)
            .bind(user_ids)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 3. [REPOSITORY] - 电话号码
    /// * `desc`: `根据电话号码查找一个用户`
    pub async fn find_user_by_phone(
        phone: &str, // 电话号码
    ) -> Result<Option<UserEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM \"cola_user\".\"user\" WHERE phone = $1 LIMIT 1",
            USER_COLUMNS
        );

        sqlx::query_as::<_, UserEntity>(&query)
            .bind(phone)
            .fetch_optional(&pool)
            .await
    }
}

//////// END

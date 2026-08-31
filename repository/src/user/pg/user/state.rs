// repository/src/user/pg/user/state.rs
// 仓储 - 可乐用户 - pg - 用户 - 状态
// 2026/6/9 09:20 Created.

////////

use crate::pg_pool;
use cola_data::cola_user::entity::user::{USER_COLUMNS, UserEntity};
use sqlx::{self};
use tracing::{error, info}; // 👈 引入日志宏

////////

// // 数据表原始字段（对应 Entity 的基础字段，1:1 完全一致）
// // ⚠️ login_ip / register_ip 用 COALESCE 兜底：兼容旧数据 NULL 不报错
// const COLUMNS: &str = r#"
//     id, _id, user_type, user_nickname, signature, avatar, bg_img,
//     email, phone, sns_url, birthday, sex, perm_id, likes, fans, follows,
//     level, author_level, lat, lng,
//     COALESCE(login_ip, '未知IP') AS login_ip,
//     COALESCE(register_ip, '未知IP') AS register_ip,
//     status,
//     create_time, created_at, updated_at
// "#;

// INSERT/RETURNING 用别名确保 COALESCE 不会在写入时报错
const INSERT_RETURNING: &str = r#"
    id, _id, user_type, user_nickname, avatar, bg_img,
    signature, email, phone, birthday, status, perm_id, create_time,
    login_ip, register_ip
"#;

// 安全的 PG 表名（使用双引号包裹并对内部双引号进行转义，防止 SQL 注入）
const TABLE_USER: &str = "\"cola_user\".\"user\"";

////////

/// # [STATE REPOSITORY] - 状态
/// * `desc`: `用户状态服务`
pub struct UserStateRepo;

// 构造函数
impl UserStateRepo {
    //

    ////////

    /// # 2. [REPOSITORY] - 查找一个用户 (精准匹配)
    pub async fn find_user_by_id(user_id: i64) -> Result<Option<UserEntity>, sqlx::Error> {
        let pool = pg_pool(); // 👈 抽出来
        let query = format!(
            "SELECT {} FROM {} WHERE id = $1 LIMIT 1",
            USER_COLUMNS, TABLE_USER
        );

        match sqlx::query_as::<_, UserEntity>(&query)
            .bind(user_id)
            .fetch_optional(&pool)
            .await
        {
            Ok(result) => {
                info!(user_id = user_id, "根据 ID 查找用户成功");
                Ok(result)
            }
            Err(e) => {
                error!(user_id = user_id, error = %e, "根据 ID 查找用户失败");
                Err(e)
            }
        }
    }

    ////////

    /// # 3. [REPOSITORY] - 根据电话号码查找一个用户
    pub async fn find_user_by_phone(
        phone: &str, // 电话号码
    ) -> Result<Option<UserEntity>, sqlx::Error> {
        let pool = pg_pool(); // 👈 抽出来
        let query = format!(
            "SELECT {} FROM {} WHERE phone = $1 LIMIT 1",
            USER_COLUMNS, TABLE_USER
        );

        match sqlx::query_as::<_, UserEntity>(&query)
            .bind(phone)
            .fetch_optional(&pool)
            .await
        {
            Ok(result) => {
                info!(phone = %phone, "[REPO] - ✅️根据电话号码查找用户成功");
                Ok(result)
            }
            Err(e) => {
                error!(phone = %phone, error = %e, "[REPO] - ❌️ 根据电话号码查找用户失败");
                Err(e)
            }
        }
    }

    ////////

    /// # 5. [REPOSITORY] - 批量查找用户
    pub async fn find_many_users_by_ids(user_ids: &[i64]) -> Result<Vec<UserEntity>, sqlx::Error> {
        let pool = pg_pool(); // 👈 抽出来
        let query = format!(
            "SELECT {} FROM {} WHERE id = ANY($1) AND status = 1 ORDER BY id DESC",
            USER_COLUMNS, TABLE_USER
        );

        match sqlx::query_as::<_, UserEntity>(&query)
            .bind(user_ids)
            .fetch_all(&pool)
            .await
        {
            Ok(result) => {
                info!(count = result.len(), "批量查找用户成功");
                Ok(result)
            }
            Err(e) => {
                error!(error = %e, "批量查找用户失败");
                Err(e)
            }
        }
    }
}

//////// END

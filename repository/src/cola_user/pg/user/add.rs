// repository/src/cola_user/pg/cola_user/add.rs
// 仓储 - 可乐用户 - pg - 用户 - 发布仓储
// 2026/6/9 09:20 Created.

////////

use crate::pg_pool;
use cola_data::cola_user::entity::user::UserEntity;
use sqlx::{self};

////////

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


////////

/// # [REPOSITORY] - 用户状态服务
pub struct UserAddRepo;

// 构造实现
impl UserAddRepo {
    //

    ////////

    /// # 1. [REPOSITORY] - 保存用户并返回实体
    pub async fn save_user(entity: UserEntity) -> Result<UserEntity, anyhow::Error> {
        let pool = pg_pool();

        // INSERT 包含 login_ip / register_ip，确保数据库 NOT NULL 不报错
        let query = format!(r#"
        INSERT INTO "cola_user"."cola_user" (
            send_id, user_type, user_nickname, avatar, bg_img,
            signature, email, phone,sns_url, birthday, status, perm_id, create_time,
            login_ip, register_ip
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
        RETURNING {}
    "#, INSERT_RETURNING);

        let saved_user = sqlx::query_as::<_, UserEntity>(&query)
            .bind(&entity.send_id)
            .bind(entity.user_type)
            .bind(&entity.user_nickname)
            .bind(&entity.avatar)
            .bind(&entity.bg_img)
            .bind(&entity.signature)
            .bind(&entity.email)
            .bind(&entity.phone)
            .bind(&entity.sns_url)
            .bind(entity.birthday)
            .bind(entity.status)
            .bind(entity.perm_id)
            .bind(entity.create_time)
            .bind(&entity.login_ip)      // ← 修复：写入登录 IP
            .bind(&entity.register_ip)    // ← 修复：写入注册 IP
            .fetch_one(&pool)
            .await?;

        Ok(saved_user)
    }
}

//////// END
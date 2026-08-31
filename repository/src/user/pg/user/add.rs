// repository/src/user/pg/user/add.rs
// 仓储 - 可乐用户 - pg - 用户 - 发布仓储
// 2026/6/9 09:20 Created.

////////

use crate::pg_pool;
use cola_data::cola_user::entity::user::{INSERT_RETURNING, UserEntity};
use sqlx::{self};
use tracing::{error, info};

////////

/// # [ADD REPOSITORY] - 用户发布
/// * `desc`: `新用户创建仓储`
pub struct UserAddRepo;

// 构造实现
impl UserAddRepo {
    //

    ////////

    /// # 1. [REPOSITORY] - 保存用户
    /// * `desc`: 保存用户并返回实体
    pub async fn save_user(entity: UserEntity) -> Result<UserEntity, anyhow::Error> {
        let pool = pg_pool();

        // INSERT 包含 login_ip / register_ip，确保数据库 NOT NULL 不报错
        let query = format!(
            r#"
        INSERT INTO "cola_user"."user" (
            _id, user_type, user_nickname, avatar,avatar_thumb, bg_img,
            signature, email, phone,sns_url, birthday, status, perm_id, create_time,
            login_ip, register_ip, created_at,last_login_time, score, coin, user_status
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19,
        $20, $21 )
        RETURNING {}
    "#,
            INSERT_RETURNING
        );

        let saved_user = match sqlx::query_as::<_, UserEntity>(&query)
            .bind(&entity._id) // UUID v4
            .bind(entity.user_type) // 类型
            .bind(&entity.user_nickname) // 昵称
            .bind(&entity.avatar) // 头像
            .bind(&entity.avatar_thumb) // 小头像
            .bind(&entity.bg_img) // 背景
            .bind(&entity.signature) // 个签
            .bind(&entity.email) // 邮箱
            .bind(&entity.phone) // 电话
            .bind(&entity.sns_url) // 社交网站
            .bind(entity.birthday) // 生日
            .bind(entity.status) // 状态码
            .bind(entity.perm_id) // 权限ID
            .bind(entity.create_time) // 创建时间(旧版)
            .bind(&entity.login_ip) // 登录的 IP
            .bind(&entity.register_ip) // 注册时的IP
            .bind(&entity.created_at) // 创建时间 (新版)
            .bind(&entity.last_login_time) // 最后登录的时间(时间戳)
            .bind(&entity.score) // 积分
            .bind(&entity.coin) // 钻石user_status
            .bind(&entity.user_status) // 状态码
            .fetch_one(&pool)
            .await
        {
            Ok(user) => user,
            Err(e) => {
                // 打印 sqlx 的错误日志
                error!(error = %e, "[🤐 REPO] - ❌️ 保存用户到数据库失败 (sqlx error)");
                return Err(e.into());
            }
        };

        Ok(saved_user)
    }
}

//////// END

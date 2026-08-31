// repository/src/auth/pg/identity/phone.rs
// ✅ REPOSITORY - AUTH 手机身份查询与绑定
// 2026/8/20 Created.

////////

use crate::pg_pool;
use anyhow::{Context, Result, anyhow};
use chrono::Utc;
use sqlx::Row;
use uuid::Uuid;

////////

/// # [REPOSITORY] - 手机身份仓储
pub struct PhoneIdentityRepo;

impl PhoneIdentityRepo {
    /// # 1. [REPOSITORY] - 查询手机号绑定用户
    /// * `desc`: `只查询正常且未删除的手机身份`
    pub async fn find_user_id_by_phone(phone: &str) -> Result<Option<i64>> {
        if phone.trim().is_empty() {
            return Err(anyhow!("手机号不能为空"));
        }

        let pool = pg_pool();
        let row = sqlx::query(
            r#"
            SELECT user_id
            FROM cola_auth.identity
            WHERE id_type = 1
              AND identifier = $1
              AND status = 1
              AND COALESCE(is_deleted, FALSE) = FALSE
            LIMIT 1
            "#,
        )
        .bind(phone)
        .fetch_optional(&pool)
        .await
        .context("查询手机号身份失败")?;

        Ok(row.map(|item| item.get::<i64, _>("user_id")))
    }

    ////////

    /// # 2. [REPOSITORY] - 绑定手机号
    /// * `desc`: `通过唯一约束避免同一手机号绑定多个用户`
    pub async fn bind_phone(user_id: i64, phone: &str) -> Result<i64> {
        if user_id <= 0 || phone.trim().is_empty() {
            return Err(anyhow!("用户ID或手机号无效"));
        }

        let pool = pg_pool();
        let now = Utc::now();
        let row = sqlx::query(
            r#"
            INSERT INTO cola_auth.identity
                (_id, id_type, user_id, identifier, secret, verified_at,
                 status, is_deleted, create_time, created_at, updated_at)
            VALUES ($1, 1, $2, $3, NULL, $4, 1, FALSE, $5, $4, $4)
            ON CONFLICT (id_type, identifier) DO UPDATE
                SET user_id = EXCLUDED.user_id,
                    verified_at = EXCLUDED.verified_at,
                    status = 1,
                    is_deleted = FALSE,
                    updated_at = EXCLUDED.updated_at,
                    deleted_at = NULL
            WHERE cola_auth.identity.user_id = EXCLUDED.user_id
            RETURNING user_id
            "#,
        )
        .bind(Uuid::new_v4().to_string())
        .bind(user_id)
        .bind(phone)
        .bind(now)
        .bind(now.timestamp())
        .fetch_one(&pool)
        .await
        .context("绑定手机号身份失败")?;

        Ok(row.get::<i64, _>("user_id"))
    }
}

//////// END

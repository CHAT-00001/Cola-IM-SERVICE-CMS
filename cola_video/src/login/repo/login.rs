// cola_video/src/login/repo/session  -- 登录状态 仓储
// 2026-03-11 10:45:00

use crate::login::entity::LoginSessionEntity;
use data::app_state::AppState;
use sqlx::{PgPool, Postgres, Transaction};


////////

/// # 统一的登录会话查询字段
/// 需确保与 LoginSessionEntity 结构体字段一一对应
const SESSION_COLUMNS: &str = r#"
    id,              -- 自增ID
    uuid,            -- Session唯一标识
    user_id,         -- 关联用户ID
    token_hash,      -- Token哈希
    client_id,       -- 客户端ID
    device_id,       -- 设备唯一标识 (指纹)
    device_name,     -- 设备名称
    last_ip,         -- 最后登录 IP
    platform,        -- 平台类型 (ios, android, web)
    expired_at,      -- 过期时间戳
    last_active_at,  -- 最后活跃时间
    status,          -- 状态: 1有效, 0注销, -1被挤掉
    created_at,      -- 首次登录时间
    updated_at       -- 记录更新时间
"#;

pub struct LoginRepo;

impl LoginRepo {
    /// # REPOSITORY - 插入新登录会话并“制裁”同平台旧设备
    /// 逻辑：在插入新 Session 前，将该用户在同平台下的其他活跃 Session 置为 -1 (被挤下线)
    pub async fn insert_session_with_kickout(
        app_state: &AppState,
        entity: &LoginSessionEntity,
    ) -> Result<i64, sqlx::Error> {
        let pool = &app_state.db.pg_pool;

        // 使用事务确保“挤人”和“登录”的原子性
        let mut tx = pool.begin().await?;

        // 1. 制裁逻辑：将同用户、同平台、状态为活跃的其他设备标记为“被挤下线”
        sqlx::query!(
            r#"
            UPDATE login
            SET status = -1, updated_at = EXTRACT(EPOCH FROM NOW())::BIGINT
            WHERE user_id = $1 AND platform = $2 AND status = 1 AND device_id != $3
            "#,
            entity.user_id,
            entity.platform,
            entity.device_id
        )
            .execute(&mut *tx)
            .await?;

        // 2. 插入新登录记录
        let row = sqlx::query!(
            r#"
            INSERT INTO login (
                uuid, user_id, token_hash, client_id, device_id, device_name,
                last_ip, platform, expired_at, last_active_at, status
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, 1)
            RETURNING id
            "#,
            entity.uuid, entity.user_id, entity.token_hash, entity.client_id,
            entity.device_id, entity.device_name, entity.last_ip,
            entity.platform, entity.expired_at, entity.last_active_at
        )
            .fetch_one(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(row.id)
    }

    /// # REPOSITORY - 注销登录 (主动退出)
    pub async fn logout_session(
        app_state: &AppState,
        uuid: &str,
    ) -> Result<u64, sqlx::Error> {
        let pool = &app_state.db.pg_pool;

        let result = sqlx::query!(
            "UPDATE login SET status = 0, updated_at = EXTRACT(EPOCH FROM NOW())::BIGINT WHERE uuid = $1",
            uuid
        )
            .execute(pool)
            .await?;

        Ok(result.rows_affected())
    }

    /// # REPOSITORY - 检查 Token 是否有效 (用于中间件鉴权)
    /// 必须满足：状态为 1 且未过期
    pub async fn find_active_session_by_token(
        app_state: &AppState,
        token_hash: &str,
    ) -> Result<Option<LoginSessionEntity>, sqlx::Error> {
        let pool = &app_state.db.pg_pool;
        let now = chrono::Utc::now().timestamp();

        let sql = format!(
            "SELECT {} FROM login WHERE token_hash = $1 AND status = 1 AND expired_at > $2 LIMIT 1",
            SESSION_COLUMNS
        );

        sqlx::query_as::<_, LoginSessionEntity>(&sql)
            .bind(token_hash)
            .bind(now)
            .fetch_optional(pool)
            .await
    }

    /// # REPOSITORY - 获取用户当前所有在线设备列表 (用于安全中心展示)
    pub async fn find_online_devices_by_uid(
        app_state: &AppState,
        user_id: i64,
    ) -> Result<Vec<LoginSessionEntity>, sqlx::Error> {
        let pool = &app_state.db.pg_pool;

        let sql = format!(
            "SELECT {} FROM login WHERE user_id = $1 AND status = 1 ORDER BY last_active_at DESC",
            SESSION_COLUMNS
        );

        sqlx::query_as::<_, LoginSessionEntity>(&sql)
            .bind(user_id)
            .fetch_all(pool)
            .await
    }

    /// # REPOSITORY - 强制清理过期 Session (系统维护)
    pub async fn clean_expired_sessions(app_state: &AppState) -> Result<u64, sqlx::Error> {
        let pool = &app_state.db.pg_pool;
        let now = chrono::Utc::now().timestamp();

        let result = sqlx::query!(
            "DELETE FROM login WHERE expired_at < $1",
            now
        )
            .execute(pool)
            .await?;

        Ok(result.rows_affected())
    }
}